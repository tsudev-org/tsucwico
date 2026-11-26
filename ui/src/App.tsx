/**
 * Application shell and the one place selection state lives.
 *
 * The flow the whole interface is built around:
 *
 *   scan → select → build a plan → read what was refused → confirm → run
 *
 * The plan step is not optional and not skippable. Every destructive action
 * passes through `buildPlan`, which is the same Rust code path the CLI uses,
 * so the front end cannot invent a shortcut around the safety gate.
 */

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import * as api from "./api";
import { CwicoError } from "./api";
import { formatDateTime, translator } from "./i18n";
import {
  LOCALE_CHOICES,
  THEME_CHOICES,
  initialLocaleChoice,
  initialTheme,
  resolveLocale,
  type LocaleChoice,
  type ThemeChoice,
} from "./defaults";
import type {
  AboutInfo,
  UpdateStatus,
  Locale,
  ProgressEvent,
  RemovalPlan,
  RunReport,
  ScanOptions,
  ScanReport,
  Selection,
  SoftwareItem,
  Tweak,
  TweakOutcome,
} from "./types";
import type { AccountView, RenewalNotice as RenewalNoticeData } from "./types";
import { DEFAULT_PLAN_OPTIONS, DEFAULT_SCAN_OPTIONS } from "./types";
import { Brand, MakerBrand } from "./components/Brand";
import { AboutView } from "./components/AboutView";
import { ConfirmCaution } from "./components/ConfirmCaution";
import { PlanDialog } from "./components/PlanDialog";
import { RenewalNotice } from "./components/RenewalNotice";
import { RunPanel, eventToLine, type LogLine } from "./components/RunPanel";
import { SoftwareTable } from "./components/SoftwareTable";
import { TweaksView } from "./components/TweaksView";
import { UpdateGate } from "./components/UpdateGate";
import { Button, Notice, Panel, ProgressBar } from "./components/primitives";
import { formatBytes } from "./i18n";

type Tab = "software" | "tweaks" | "activity" | "about";

const THEME_KEY = "cwico.theme";
/** Chỉ Sáng / Tối / Hệ thống; lần đầu theo hệ thống (PRODUCT_POLICY 1, xem defaults.ts). */
const systemDark = () => window.matchMedia("(prefers-color-scheme: dark)").matches;
const LOCALE_KEY = "cwico.locale";
/** Ngôn ngữ ưu tiên của Windows, như WebView2 báo. */
const systemLanguages = (): readonly string[] => navigator.languages ?? [navigator.language];

export default function App() {
  const [localeChoice, setLocaleChoice] = useState<LocaleChoice>(() =>
    initialLocaleChoice(localStorage.getItem(LOCALE_KEY)),
  );
  const [osLanguages, setOsLanguages] = useState<readonly string[]>(systemLanguages);
  const locale: Locale = resolveLocale(localeChoice, osLanguages);
  const [theme, setTheme] = useState<ThemeChoice>(() => initialTheme(localStorage.getItem(THEME_KEY)));
  const [osDark, setOsDark] = useState(systemDark);
  const dark = theme === "dark" || (theme === "system" && osDark);
  const [tab, setTab] = useState<Tab>("software");

  /** `null` while the check is still running - the app is not shown yet. */
  const [update, setUpdate] = useState<UpdateStatus | null>(null);

  const [about, setAbout] = useState<AboutInfo | null>(null);
  const [report, setReport] = useState<ScanReport | null>(null);
  const [scanning, setScanning] = useState(false);
  const [deepScan, setDeepScan] = useState(false);

  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [confirmed, setConfirmed] = useState<Set<string>>(new Set());
  const [deepClean, setDeepClean] = useState(true);

  /** The Caution/Unknown item awaiting its own acknowledgement. */
  const [pendingConfirm, setPendingConfirm] = useState<SoftwareItem | null>(null);

  const [plan, setPlan] = useState<RemovalPlan | null>(null);
  const [planOpen, setPlanOpen] = useState(false);

  const [running, setRunning] = useState(false);
  const [runOpen, setRunOpen] = useState(false);
  const [runReport, setRunReport] = useState<RunReport | null>(null);
  const [progress, setProgress] = useState<number | undefined>(undefined);

  const [lines, setLines] = useState<LogLine[]>([]);
  const [error, setError] = useState<string | null>(null);

  const [tweaks, setTweaks] = useState<Tweak[]>([]);
  const [tweakBusy, setTweakBusy] = useState(false);
  const [tweakOutcomes, setTweakOutcomes] = useState<TweakOutcome[]>([]);
  // Device count for the "several devices, no Pro" notice, or null.
  const [account, setAccount] = useState<AccountView>({
    signedIn: false,
    proActive: false,
    proUntil: 0,
    badge: "none",
    devices30d: 0,
  });
  const [renewal, setRenewal] = useState<RenewalNoticeData | null>(null);
  const busyRef = useRef(false);
  busyRef.current = running;

  const t = translator(locale);

  /* ------------------------------------------------------------- effects */

  useEffect(() => {
    document.documentElement.classList.toggle("dark", dark);
    // The ecosystem tokens key off data-theme; the `dark` class stays for Tailwind.
    document.documentElement.dataset.theme = dark ? "dark" : "light";
  }, [dark]);

  useEffect(() => {
    localStorage.setItem(THEME_KEY, theme);
  }, [theme]);

  // "Hệ thống": đổi theo Windows ngay, không cần mở lại app.
  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const onChange = () => setOsDark(mq.matches);
    mq.addEventListener("change", onChange);
    return () => mq.removeEventListener("change", onChange);
  }, []);

  useEffect(() => {
    const onChange = () => setOsLanguages(systemLanguages());
    window.addEventListener("languagechange", onChange);
    return () => window.removeEventListener("languagechange", onChange);
  }, []);

  useEffect(() => {
    localStorage.setItem(LOCALE_KEY, localeChoice);
  }, [localeChoice]);

  useEffect(() => {
    document.documentElement.lang = locale;
  }, [locale]);

  // Checked before the application is shown, because the gate must not appear
  // in the middle of someone's work. Bounded so a hanging request cannot leave
  // the user staring at a splash screen: a timeout is treated exactly like any
  // other failed check, and the app starts normally.
  useEffect(() => {
    const CHECK_TIMEOUT_MS = 10_000;
    let settled = false;

    const settle = (status: UpdateStatus) => {
      if (!settled) {
        settled = true;
        setUpdate(status);
      }
    };

    const timer = window.setTimeout(
      () =>
        settle({
          available: false,
          checked: false,
          checkError: `the update check did not answer within ${
            CHECK_TIMEOUT_MS / 1000
          }s`,
          currentRelease: "",
          currentVersion: "",
        }),
      CHECK_TIMEOUT_MS,
    );

    void api.checkForUpdate().then((status) => {
      window.clearTimeout(timer);
      settle(status);
    });

    return () => window.clearTimeout(timer);
  }, []);

  // Start-up: the cached account, then a refresh (signed out = no network).
  // The only notice that may open by itself is the renewal reminder / expiry
  // notice of PRODUCT_POLICY 7.6 - and only for someone who bought Pro; the
  // rule is in `cwico_core::pro`. Nothing else invites to sign in, buy Pro or
  // donate on its own (PRODUCT_POLICY 3.8, AUTH_AND_ACCOUNT 14.4).
  useEffect(() => {
    try {
      for (const key of api.LEGACY_LOCAL_KEYS) localStorage.removeItem(key);
    } catch {
      /* storage unavailable: nothing to clean */
    }
    void (async () => {
      const cached = await api.accountStatus().catch(() => null);
      if (cached) setAccount(cached);
      const fresh = await api.accountRefresh().catch(() => api.accountStatus().catch(() => null));
      if (fresh) setAccount(fresh);
      const due = await api.proRenewalNotice(null).catch(() => null);
      if (due) setRenewal(due);
    })();
  }, []);

  // Coming back after >= 30 minutes in the background is a new "open" for
  // 7.6; the 30-minute rule itself lives in Rust. Never mid-task: skipped
  // while a run is going or the user is typing.
  useEffect(() => {
    let hiddenAt: number | null = null;
    const onVisibility = () => {
      if (document.visibilityState === "hidden") {
        hiddenAt = Date.now();
        return;
      }
      if (hiddenAt === null) return;
      const since = hiddenAt;
      hiddenAt = null;
      const typing = document.activeElement?.matches("input, textarea, select") ?? false;
      if (busyRef.current || typing) return;
      void api
        .accountRefresh()
        .then(setAccount)
        .catch(() => undefined)
        .then(() => api.proRenewalNotice(since))
        .then((due) => due && setRenewal(due))
        .catch(() => undefined);
    };
    document.addEventListener("visibilitychange", onVisibility);
    return () => document.removeEventListener("visibilitychange", onVisibility);
  }, []);

  useEffect(() => {
    api.about().then(setAbout).catch(() => setAbout(null));
    api
      .tweakCatalog()
      .then((catalog) => setTweaks(catalog.tweaks))
      .catch(() => setTweaks([]));
  }, []);

  useEffect(() => {
    let counter = 0;
    let unlisten: (() => void) | undefined;

    void api
      .onProgress((event: ProgressEvent) => {
        const line = eventToLine(event, (counter += 1));
        if (line) setLines((previous) => [...previous.slice(-400), line]);

        if (event.type === "stepStarted") {
          setProgress(event.index / Math.max(event.total, 1));
        }
        if (event.type === "runFinished" || event.type === "scanFinished") {
          setProgress(undefined);
        }
      })
      .then((fn) => {
        unlisten = fn;
      });

    return () => unlisten?.();
  }, []);

  /* -------------------------------------------------------------- actions */

  const handleError = useCallback((cause: unknown) => {
    setError(
      cause instanceof CwicoError
        ? `${cause.message} (${cause.code})`
        : String(cause),
    );
  }, []);

  const runScan = useCallback(async () => {
    setScanning(true);
    setError(null);
    setLines([]);
    try {
      const options: ScanOptions = deepScan
        ? {
            ...DEFAULT_SCAN_OPTIONS,
            leftovers: true,
            includeSystemComponents: true,
            includeNonRemovable: true,
            measureDiskUsage: true,
          }
        : DEFAULT_SCAN_OPTIONS;
      const result = await api.scan(options);
      setReport(result);
      // Ids are stable across scans, but an item that vanished must not stay
      // selected or the next plan would fail on a ghost.
      const present = new Set(result.items.map((item) => item.id));
      setSelected((previous) => new Set([...previous].filter((id) => present.has(id))));
    } catch (cause) {
      handleError(cause);
    } finally {
      setScanning(false);
    }
  }, [deepScan, handleError]);

  const items = report?.items ?? [];

  const deselect = useCallback((id: string) => {
    setSelected((previous) => {
      const next = new Set(previous);
      next.delete(id);
      return next;
    });
    setConfirmed((previous) => {
      const next = new Set(previous);
      next.delete(id);
      return next;
    });
  }, []);

  const toggleItem = useCallback(
    (item: SoftwareItem) => {
      // Critical items are not selectable at all - the row renders a lock
      // instead of a checkbox, and this is the second line of that defence.
      if (item.safety === "critical") return;

      if (selected.has(item.id)) {
        deselect(item.id);
        return;
      }

      // Caution and Unknown items need a deliberate acknowledgement, not a
      // tick. The dialog is what sets `confirmed`, and `confirmed` is what
      // the engine requires before it will plan the removal.
      if (item.safety === "caution" || item.safety === "unknown") {
        setPendingConfirm(item);
        return;
      }

      setSelected((previous) => new Set(previous).add(item.id));
    },
    [selected, deselect],
  );

  const acceptPendingConfirm = useCallback(() => {
    if (!pendingConfirm) return;
    setSelected((previous) => new Set(previous).add(pendingConfirm.id));
    setConfirmed((previous) => new Set(previous).add(pendingConfirm.id));
    setPendingConfirm(null);
  }, [pendingConfirm]);

  const selectAllSafe = useCallback(() => {
    setSelected(
      new Set(items.filter((item) => item.safety === "safe").map((item) => item.id)),
    );
    setConfirmed(new Set());
  }, [items]);

  const clearSelection = useCallback(() => {
    setSelected(new Set());
    setConfirmed(new Set());
  }, []);

  const selections = useMemo<Selection[]>(
    () =>
      [...selected].map((id) => ({
        itemId: id,
        action: deepClean ? "uninstall_and_deep_clean" : "uninstall",
        confirmed: confirmed.has(id),
      })),
    [selected, confirmed, deepClean],
  );

  const reviewPlan = useCallback(async () => {
    setError(null);
    try {
      const built = await api.buildPlan(selections, {
        ...DEFAULT_PLAN_OPTIONS,
        dryRun: false,
      });
      setPlan(built);
      setPlanOpen(true);
    } catch (cause) {
      handleError(cause);
    }
  }, [selections, handleError]);

  const execute = useCallback(
    async (dryRun: boolean) => {
      if (!plan) return;
      setPlanOpen(false);
      setLines([]);
      setRunReport(null);
      setRunOpen(true);
      setRunning(true);
      try {
        const target: RemovalPlan = dryRun
          ? await api.buildPlan(selections, { ...DEFAULT_PLAN_OPTIONS, dryRun: true })
          : plan;
        const result = await api.executePlan(target);
        setRunReport(result);
        if (!dryRun) {
          clearSelection();
          await runScan();
        }
      } catch (cause) {
        handleError(cause);
      } finally {
        setRunning(false);
        setProgress(undefined);
      }
    },
    [plan, selections, clearSelection, runScan, handleError],
  );

  const applyTweaks = useCallback(
    async (ids: string[], enable: boolean) => {
      setTweakBusy(true);
      setError(null);
      try {
        setTweakOutcomes(await api.applyTweaks(ids, enable, false));
      } catch (cause) {
        handleError(cause);
      } finally {
        setTweakBusy(false);
      }
    },
    [handleError],
  );

  /* ---------------------------------------------------------------- view */

  const elevated = about?.platform.elevated ?? false;
  const selectedItems = items.filter((item) => selected.has(item.id));
  const selectedBytes = selectedItems.reduce(
    (sum, item) => sum + (item.sizeBytes ?? 0),
    0,
  );

  // Nothing is rendered until the update check has settled: showing the app
  // and then yanking it away a second later would be worse than a brief wait.
  if (update === null) {
    return (
      <div
        className="flex h-full flex-col items-center justify-center gap-5"
        style={{ background: "var(--surface-app)" }}
      >
        <Brand size="lg" />
        <p className="text-sm" style={{ color: "var(--text-muted)" }}>
          {t("update.checking")}
        </p>
        <div className="w-56">
          <ProgressBar />
        </div>
      </div>
    );
  }

  if (update.available) {
    return (
      <UpdateGate
        status={update}
        locale={locale}
        onLocaleToggle={() => setLocaleChoice(locale === "vi" ? "en" : "vi")}
      />
    );
  }

  return (
    <div className="flex h-full flex-col">
      {/* --------------------------------------------------------- header */}
      <header
        className="flex shrink-0 items-center gap-4 border-b px-5 py-3"
        style={{
          background: "var(--surface-panel)",
          borderColor: "var(--border-subtle)",
        }}
      >
        <Brand size="md" tagline={t("app.subtitle")} />

        <nav className="ml-4 flex items-center gap-1">
          {(
            [
              ["software", t("nav.software")],
              ["tweaks", t("nav.tweaks")],
              ["activity", t("nav.activity")],
              ["about", t("nav.about")],
            ] as [Tab, string][]
          ).map(([id, label]) => (
            <button
              key={id}
              type="button"
              onClick={() => setTab(id)}
              className="rounded-lg px-3 py-1.5 text-sm font-medium transition-colors"
              style={
                tab === id
                  ? {
                      background: "color-mix(in srgb, var(--primary) 12%, var(--bg-surface))",
                      color: "var(--text-link)",
                    }
                  : { color: "var(--text-secondary)" }
              }
            >
              {label}
            </button>
          ))}
        </nav>

        <span className="flex-1" />

        <select
          value={localeChoice}
          onChange={(e) => setLocaleChoice(e.target.value as LocaleChoice)}
          aria-label={t("locale.group")}
          title={t("locale.group")}
          className="rounded-lg border px-2 py-1.5 text-xs font-semibold"
          style={{
            borderColor: "var(--border-control)",
            color: "var(--text-secondary)",
            background: "var(--surface-panel)",
          }}
        >
          {LOCALE_CHOICES.map((c) => (
            <option key={c} value={c}>
              {c === "system" ? `${t("locale.system")} (${locale === "vi" ? "VI" : "EN"})` : t(`locale.${c}`)}
            </option>
          ))}
        </select>
        <div
          role="radiogroup"
          aria-label={t("theme.group")}
          className="flex items-center gap-0.5 rounded-full border p-0.5"
          style={{ borderColor: "var(--border-control)", background: "var(--surface-panel)" }}
        >
          {THEME_CHOICES.map((c, i) => (
            <button
              key={c}
              type="button"
              role="radio"
              aria-checked={theme === c}
              tabIndex={theme === c ? 0 : -1}
              title={t(`theme.${c}`)}
              aria-label={t(`theme.${c}`)}
              onClick={() => setTheme(c)}
              onKeyDown={(e) => {
                const step = e.key === "ArrowRight" || e.key === "ArrowDown" ? 1 : e.key === "ArrowLeft" || e.key === "ArrowUp" ? -1 : 0;
                if (!step) return;
                e.preventDefault();
                const next = THEME_CHOICES[(i + step + THEME_CHOICES.length) % THEME_CHOICES.length] as ThemeChoice;
                setTheme(next);
                (e.currentTarget.parentElement?.querySelectorAll("button")[THEME_CHOICES.indexOf(next)] as HTMLButtonElement | undefined)?.focus();
              }}
              className="flex h-7 w-7 items-center justify-center rounded-full"
              style={{
                background: theme === c ? "var(--surface-hover)" : "transparent",
                color: theme === c ? "var(--text-primary)" : "var(--text-muted)",
              }}
            >
              <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
                {c === "light" && (<><circle cx="12" cy="12" r="4.2" /><path d="M12 2.6v2.2M12 19.2v2.2M2.6 12h2.2M19.2 12h2.2M5.3 5.3l1.6 1.6M17.1 17.1l1.6 1.6M18.7 5.3l-1.6 1.6M6.9 17.1l-1.6 1.6" /></>)}
                {c === "dark" && <path d="M20.5 14.3A8.5 8.5 0 1 1 9.7 3.5a6.8 6.8 0 0 0 10.8 10.8Z" />}
                {c === "system" && (<><rect x="2.8" y="4.4" width="18.4" height="12.2" rx="1.6" /><path d="M8.6 20.2h6.8M12 16.6v3.6" /></>)}
              </svg>
            </button>
          ))}
        </div>
      </header>

      {/* ----------------------------------------------------------- body */}
      <main className="flex min-h-0 flex-1 flex-col gap-3 px-5 py-4">
        {error && (
          <Notice
            tone="danger"
            title={t("error.title")}
            action={
              <Button variant="ghost" onClick={() => setError(null)}>
                {t("action.close")}
              </Button>
            }
          >
            <span className="selectable">{error}</span>
          </Notice>
        )}

        {!update.checked && (
          <Notice tone="info" title={t("update.checkFailed")}>
            {t("update.checkFailedBody")}
            {update.checkError && (
              <span
                className="selectable mt-1 block font-mono text-[11px]"
                style={{ color: "var(--text-muted)" }}
              >
                {update.checkError}
              </span>
            )}
          </Notice>
        )}

        {about && !elevated && (
          <Notice
            tone="warning"
            title={t("elevation.required")}
            action={
              <Button variant="outline" onClick={() => void api.relaunchAsAdmin()}>
                {t("elevation.relaunch")}
              </Button>
            }
          >
            {t("elevation.body")}
          </Notice>
        )}

        {about && elevated && !about.platform.systemRestoreAvailable && (
          <Notice tone="warning" title={t("restore.unavailable")}>
            {t("restore.unavailableBody")}
          </Notice>
        )}

        {tab === "software" && (
          <>
            <div className="flex flex-wrap items-center gap-3">
              <Button variant="primary" onClick={() => void runScan()} disabled={scanning}>
                {scanning ? t("scan.running") : report ? t("scan.rerun") : t("scan.run")}
              </Button>
              <label
                className="flex items-center gap-2 text-[13px]"
                style={{ color: "var(--text-secondary)" }}
                title={t("scan.deepHint")}
              >
                <input
                  type="checkbox"
                  checked={deepScan}
                  onChange={(event) => setDeepScan(event.target.checked)}
                  className="h-4 w-4 accent-[var(--primary)]"
                />
                {t("scan.deep")}
              </label>
              <label
                className="flex items-center gap-2 text-[13px]"
                style={{ color: "var(--text-secondary)" }}
              >
                <input
                  type="checkbox"
                  checked={deepClean}
                  onChange={(event) => setDeepClean(event.target.checked)}
                  className="h-4 w-4 accent-[var(--color-dev-500)]"
                />
                {t("plan.deepClean")}
              </label>

              {report && (
                <span className="text-[13px]" style={{ color: "var(--text-muted)" }}>
                  {t("scan.lastScan")}: {formatDateTime(locale, report.startedAt)} ·{" "}
                  {t("scan.reclaimable")}: {formatBytes(locale, report.stats.reclaimableBytes)}
                </span>
              )}
            </div>

            {scanning && <ProgressBar />}

            {!report && !scanning ? (
              // First screen, `trinh-dien` layout (DESIGN_SYSTEM 1.2.2): words and the
              // action on the left, what protects the machine on the right; static
              // `luoi-quet` motif behind (1.2.4).
              <Panel className="motif-luoi-quet grid flex-1 content-center gap-8 overflow-hidden px-8 py-10 md:grid-cols-[minmax(0,5fr)_minmax(0,6fr)] md:items-center">
                <div className="flex flex-col items-start gap-4">
                  <Brand size="lg" />
                  <h2 className="text-2xl font-bold leading-tight tracking-tight">{t("app.subtitle")}</h2>
                  <p className="max-w-md text-sm" style={{ color: "var(--text-secondary)" }}>
                    {t("scan.empty")}
                  </p>
                  <Button variant="primary" onClick={() => void runScan()}>
                    {t("scan.run")}
                  </Button>
                </div>
                {about && (
                  <ul className="grid grid-cols-2 gap-3" aria-label={t("about.safetyDb")}>
                    {[
                      { label: t("safety.critical"), value: about.criticalRules, colour: "var(--danger)", wide: true },
                      { label: t("safety.caution"), value: about.cautionRules, colour: "var(--warning)", wide: false },
                      { label: t("safety.safe"), value: about.safeRules, colour: "var(--success)", wide: false },
                    ].map((cell) => (
                      <li
                        key={cell.label}
                        className={`rounded-lg border px-4 py-4 ${cell.wide ? "col-span-2" : ""}`}
                        style={{
                          borderColor: `color-mix(in srgb, ${cell.colour} 30%, transparent)`,
                          background: `color-mix(in srgb, ${cell.colour} 7%, var(--surface-panel))`,
                        }}
                      >
                        <p className="font-display text-3xl font-bold tabular-nums" style={{ color: cell.colour }}>
                          {cell.value}
                        </p>
                        <p className="text-[12px]" style={{ color: "var(--text-secondary)" }}>
                          {cell.label} · {t("about.rules")}
                        </p>
                      </li>
                    ))}
                  </ul>
                )}
              </Panel>
            ) : (
              report && (
                <SoftwareTable
                  items={items}
                  locale={locale}
                  selected={selected}
                  confirmed={confirmed}
                  onToggle={toggleItem}
                  onSelectAllSafe={selectAllSafe}
                  onClearSelection={clearSelection}
                />
              )
            )}
          </>
        )}

        {tab === "tweaks" && (
          <TweaksView
            tweaks={tweaks}
            locale={locale}
            busy={tweakBusy}
            outcomes={tweakOutcomes}
            onApply={(ids, enable) => void applyTweaks(ids, enable)}
          />
        )}

        {tab === "activity" && (
          <Panel className="min-h-0 flex-1 overflow-y-auto">
            <h2 className="mb-3 text-sm font-semibold">{t("activity.title")}</h2>
            {lines.length === 0 ? (
              <p className="text-sm" style={{ color: "var(--text-muted)" }}>
                {t("activity.empty")}
              </p>
            ) : (
              <div className="selectable font-mono text-[12px] leading-relaxed">
                {lines.map((line) => (
                  <div key={line.id}>{line.text}</div>
                ))}
              </div>
            )}
          </Panel>
        )}

        {tab === "about" && (
          <AboutView info={about} locale={locale} account={account} onAccountChange={setAccount} />
        )}
      </main>

      {/* ---------------------------------------------------- action bar */}
      {tab === "software" && selected.size > 0 && (
        <div
          className="flex shrink-0 items-center gap-4 border-t px-5 py-3"
          style={{
            background: "var(--surface-panel)",
            borderColor: "var(--border-subtle)",
          }}
        >
          <span className="text-sm">
            <strong className="tabular-nums">{selected.size}</strong> {t("select.count")}
            {selectedBytes > 0 && (
              <span style={{ color: "var(--text-muted)" }}>
                {" "}
                · {formatBytes(locale, selectedBytes)}
              </span>
            )}
          </span>
          <span className="flex-1" />
          <Button variant="ghost" onClick={clearSelection}>
            {t("select.none")}
          </Button>
          <Button variant="primary" onClick={() => void reviewPlan()}>
            {t("action.review")} →
          </Button>
        </div>
      )}

      {/* --------------------------------------------------------- footer */}
      <footer
        className="flex shrink-0 items-center justify-between gap-3 border-t px-5 py-2 text-[11px]"
        style={{
          background: "var(--surface-sunken)",
          borderColor: "var(--border-subtle)",
          color: "var(--text-muted)",
        }}
      >
        <MakerBrand size="sm" />
        <span className="selectable">
          {about?.appRelease ?? update.currentRelease}
          {about && ` · ${t("about.safetyDb")} v${about.safetyDbVersion}`}
        </span>
      </footer>

      <ConfirmCaution
        item={pendingConfirm}
        locale={locale}
        onConfirm={acceptPendingConfirm}
        onCancel={() => setPendingConfirm(null)}
      />

      <PlanDialog
        open={planOpen}
        plan={plan}
        locale={locale}
        busy={running}
        onClose={() => setPlanOpen(false)}
        onDryRun={() => void execute(true)}
        onExecute={() => void execute(false)}
      />

      <RunPanel
        open={runOpen}
        locale={locale}
        running={running}
        progress={progress}
        lines={lines}
        report={runReport}
        onClose={() => setRunOpen(false)}
      />

      <RenewalNotice
        notice={renewal}
        locale={locale}
        onRenew={() => {
          setRenewal(null);
          void api.openProPage();
        }}
        onClose={() => setRenewal(null)}
      />

    </div>
  );
}

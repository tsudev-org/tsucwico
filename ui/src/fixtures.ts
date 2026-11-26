/**
 * Browser fixtures.
 *
 * When the UI runs outside the Tauri window - `npm run dev` in a browser, or
 * a screenshot build in CI - there is no engine behind it. These fixtures
 * stand in, and they deliberately mirror the Rust `MockBackend`: one item of
 * every safety class and every source kind, so the confirmation gate and the
 * hard block on Critical items are exercised during ordinary UI work rather
 * than discovered on a real machine.
 *
 * None of this is compiled into the desktop build's behaviour: `api.ts` only
 * reaches for it when `__TAURI_INTERNALS__` is absent.
 */

import pkg from "../package.json";
import type {
  AboutInfo,
  AccountView,
  RenewalNotice,
  PlanOptions,
  PlannedItem,
  ProgressEvent,
  RemovalPlan,
  RejectedSelection,
  RunReport,
  ScanOptions,
  ScanReport,
  Selection,
  SoftwareItem,
  TweakCatalog,
  TweakOutcome,
  UpdateProgress,
  UpdateStatus,
} from "./types";
import { DEFAULT_PLAN_OPTIONS, DEFAULT_SCAN_OPTIONS } from "./types";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/**
 * `?preview=demo` is the state the product website records its screenshots and
 * walkthrough video in (`website/scripts/quay-demo.mjs`): elevated, System
 * Restore available, the real version number, no "browser preview" warning.
 * Same fixture items - only the banners that would never show on a correctly
 * started desktop session are switched off.
 */
const previewModes = (): string[] =>
  typeof window === "undefined"
    ? []
    : (new URLSearchParams(window.location.search).get("preview") ?? "").split(",");
const isDemo = (): boolean => previewModes().includes("demo");

const DAY = 86_400;
const nowSec = () => Math.floor(Date.now() / 1000);

export function signedOut(): AccountView {
  return { signedIn: false, proActive: false, proUntil: 0, badge: "none", devices30d: 0 };
}

/**
 * `?preview=signed-in` (free, donor), `?preview=pro` (Pro, 120 days left),
 * `?preview=pro-renew` (Pro, 5 days left), `?preview=pro-expired`.
 */
export async function account(): Promise<AccountView> {
  const m = previewModes();
  if (m.includes("pro")) return { signedIn: true, proActive: true, proUntil: nowSec() + 120 * DAY, badge: "both", devices30d: 2 };
  if (m.includes("pro-renew")) return { signedIn: true, proActive: true, proUntil: nowSec() + 5 * DAY, badge: "pro", devices30d: 1 };
  if (m.includes("pro-expired")) return { signedIn: true, proActive: false, proUntil: nowSec() - 2 * DAY, badge: "none", devices30d: 1 };
  if (m.includes("signed-in")) return { signedIn: true, proActive: false, proUntil: 0, badge: "donor", devices30d: 1 };
  return signedOut();
}

export async function proRenewalNotice(): Promise<RenewalNotice | null> {
  const m = previewModes();
  if (m.includes("pro-renew")) return { kind: "renew", daysLeft: 5, proUntil: nowSec() + 5 * DAY };
  if (m.includes("pro-expired")) return { kind: "expired", proUntil: nowSec() - 2 * DAY };
  return null;
}

/**
 * The CHANGELOG section of the version being built - real notes for the demo
 * update screen. Imported lazily so the desktop bundle never carries it.
 */
async function releaseNotes(version: string): Promise<string> {
  const { default: changelog } = await import("../../CHANGELOG.md?raw");
  const start = changelog.indexOf(`## v${version} `);
  if (start < 0) return "";
  const body = changelog.slice(changelog.indexOf("\n", start) + 1);
  const end = body.search(/^## /m);
  return (end < 0 ? body : body.slice(0, end)).trim();
}

let emit: ((event: ProgressEvent) => void) | null = null;

export function onProgress(handler: (event: ProgressEvent) => void) {
  emit = handler;
  return () => {
    emit = null;
  };
}

function item(partial: Partial<SoftwareItem> & Pick<SoftwareItem, "id" | "name" | "source" | "safety">): SoftwareItem {
  return {
    scope: "machine",
    arch: "x64",
    state: "installed",
    executables: [],
    canDisable: false,
    canUninstall: true,
    extra: {},
    ...partial,
  };
}

const ITEMS: SoftwareItem[] = [
  item({
    id: "reg:hkcu:OneDriveSetup.exe",
    name: "Microsoft OneDrive",
    source: "registry_uninstall",
    safety: "safe",
    version: "24.201.1006.0005",
    publisher: "Microsoft Corporation",
    scope: "user",
    sizeBytes: 1932735283,
    installDate: "2026-03-14",
    installLocation: "C:\\Users\\demo\\AppData\\Local\\Microsoft\\OneDrive",
    registryKey:
      "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\OneDriveSetup.exe",
    uninstallString: "C:\\Windows\\SysWOW64\\OneDriveSetup.exe /uninstall",
    quietUninstallString:
      "C:\\Windows\\SysWOW64\\OneDriveSetup.exe /uninstall /silent",
    executables: ["OneDrive.exe"],
    state: "running",
    safetyReason: {
      en: "Cloud file sync. Windows boots and runs normally without it; local files are untouched.",
      vi: "Đồng bộ tệp đám mây. Windows vẫn khởi động và chạy bình thường khi gỡ; tệp cục bộ không bị ảnh hưởng.",
    },
  }),
  item({
    id: "appx:Microsoft.XboxApp_8wekyb3d8bbwe",
    name: "Xbox Console Companion",
    source: "appx_package",
    safety: "safe",
    version: "48.94.13001.0",
    publisher: "Microsoft Corporation",
    scope: "allusers",
    sizeBytes: 184549376,
    packageFamilyName: "Microsoft.XboxApp_8wekyb3d8bbwe",
    safetyReason: {
      en: "Gaming social layer and screen-recording overlay. Removing it does not affect games that do not use Xbox Live.",
      vi: "Lớp mạng xã hội chơi game và overlay quay màn hình. Gỡ không ảnh hưởng các game không dùng Xbox Live.",
    },
  }),
  item({
    id: "appx:king.com.CandyCrushSaga_kgqvnymyfvs32",
    name: "Candy Crush Saga",
    source: "appx_package",
    safety: "safe",
    publisher: "king.com",
    scope: "allusers",
    sizeBytes: 96468992,
    packageFamilyName: "king.com.CandyCrushSaga_kgqvnymyfvs32",
    safetyReason: {
      en: "Promotional software delivered by Windows' Consumer Experience. Not part of Windows.",
      vi: "Phần mềm quảng bá do Consumer Experience của Windows tải về. Không thuộc Windows.",
    },
  }),
  item({
    id: "appx:Microsoft.YourPhone_8wekyb3d8bbwe",
    name: "Phone Link",
    source: "appx_package",
    safety: "safe",
    publisher: "Microsoft Corporation",
    sizeBytes: 212336640,
    packageFamilyName: "Microsoft.YourPhone_8wekyb3d8bbwe",
    safetyReason: {
      en: "Android/iPhone companion. Nothing else uses it.",
      vi: "Ứng dụng liên kết điện thoại Android/iPhone. Không có thành phần nào khác dùng đến.",
    },
  }),
  item({
    id: "appxprov:Microsoft.BingNews_8wekyb3d8bbwe",
    name: "Microsoft News",
    source: "appx_provisioned",
    safety: "safe",
    publisher: "Microsoft Corporation",
    scope: "allusers",
    packageFamilyName: "Microsoft.BingNews_8wekyb3d8bbwe",
    description: {
      en: "Staged on this Windows image: it will be installed automatically for every new user account until it is deprovisioned.",
      vi: "Được nạp sẵn trong bản Windows này: nó sẽ tự động cài cho mọi tài khoản người dùng mới cho đến khi bị gỡ khỏi image.",
    },
    safetyReason: {
      en: "Content apps that feed the taskbar widgets. Purely optional.",
      vi: "Ứng dụng nội dung cung cấp dữ liệu cho widget trên thanh tác vụ. Hoàn toàn tùy chọn.",
    },
  }),
  item({
    id: "svc:DiagTrack",
    name: "Connected User Experiences and Telemetry",
    source: "windows_service",
    safety: "safe",
    systemName: "DiagTrack",
    state: "running",
    canDisable: true,
    extra: { startType: "automatic" },
    safetyReason: {
      en: "Connected User Experiences and Telemetry. Not required to run Windows.",
      vi: "Dịch vụ trải nghiệm người dùng & telemetry. Không cần thiết để Windows chạy.",
    },
  }),
  item({
    id: "task:\\Microsoft\\Windows\\Customer Experience Improvement Program\\Consolidator",
    name: "Consolidator",
    source: "scheduled_task",
    safety: "safe",
    systemName:
      "\\Microsoft\\Windows\\Customer Experience Improvement Program\\Consolidator",
    state: "enabled",
    canDisable: true,
    safetyReason: {
      en: "Scheduled uploads of usage statistics to Microsoft. Disabling has no functional cost.",
      vi: "Tác vụ định kỳ gửi thống kê sử dụng về Microsoft. Tắt đi không mất tính năng nào.",
    },
  }),
  item({
    id: "startup:HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run:Spotify",
    name: "Spotify",
    source: "startup_entry",
    safety: "unknown",
    state: "enabled",
    canDisable: true,
    extra: {
      startupLocation:
        "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
      command: "C:\\Users\\demo\\AppData\\Roaming\\Spotify\\Spotify.exe --autostart",
    },
    safetyReason: {
      en: "Not in the safety database - third-party or uncommon software. Review before removing.",
      vi: "Chưa có trong cơ sở dữ liệu an toàn - phần mềm bên thứ ba hoặc ít gặp. Hãy xem kỹ trước khi gỡ.",
    },
  }),
  item({
    id: "reg:hklm64:Microsoft Edge",
    name: "Microsoft Edge",
    source: "registry_uninstall",
    safety: "caution",
    version: "129.0.2792.52",
    publisher: "Microsoft Corporation",
    sizeBytes: 627048448,
    executables: ["msedge.exe"],
    registryKey:
      "HKLM\\SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Microsoft Edge",
    uninstallString:
      "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\setup.exe --uninstall --system-level",
    safetyReason: {
      en: "Windows uses Edge to render help pages, PDF previews, sign-in prompts and some Settings panes. Removing it can break those flows and Windows Update may reinstall it.",
      vi: "Windows dùng Edge để hiển thị trang trợ giúp, xem trước PDF, hộp đăng nhập và một số trang Settings. Gỡ có thể làm hỏng các luồng này và Windows Update có thể tự cài lại.",
    },
  }),
  item({
    id: "appx:Microsoft.Windows.Photos_8wekyb3d8bbwe",
    name: "Photos",
    source: "appx_package",
    safety: "caution",
    publisher: "Microsoft Corporation",
    sizeBytes: 318767104,
    packageFamilyName: "Microsoft.Windows.Photos_8wekyb3d8bbwe",
    safetyReason: {
      en: "Default image viewer and the handler for most picture file types. Removing it leaves images with no default viewer until you set another one.",
      vi: "Trình xem ảnh mặc định và trình xử lý hầu hết định dạng ảnh. Gỡ xong ảnh sẽ không có trình xem mặc định cho đến khi bạn chỉ định app khác.",
    },
  }),
  item({
    id: "reg:hklm64:AcmeLedger",
    name: "Acme Ledger Desktop",
    source: "registry_uninstall",
    safety: "unknown",
    version: "7.2.1",
    publisher: "Acme Industrial Software Ltd",
    sizeBytes: 478150656,
    installDate: "2025-11-02",
    installLocation: "C:\\Program Files\\Acme\\Ledger",
    uninstallString: "C:\\Program Files\\Acme\\Ledger\\uninstall.exe",
    executables: ["AcmeLedger.exe"],
    safetyReason: {
      en: "Not in the safety database - third-party or uncommon software. Review before removing.",
      vi: "Chưa có trong cơ sở dữ liệu an toàn - phần mềm bên thứ ba hoặc ít gặp. Hãy xem kỹ trước khi gỡ.",
    },
  }),
  item({
    id: "svc:WinDefend",
    name: "Microsoft Defender Antivirus Service",
    source: "windows_service",
    safety: "critical",
    systemName: "WinDefend",
    state: "running",
    canDisable: true,
    safetyReason: {
      en: "The operating system's antivirus, firewall and security centre. Removing it leaves the machine unprotected and Windows Update will fight to restore it.",
      vi: "Chương trình diệt virus, tường lửa và trung tâm bảo mật của hệ điều hành. Gỡ bỏ khiến máy không được bảo vệ và Windows Update sẽ liên tục khôi phục lại.",
    },
  }),
  item({
    id: "svc:RpcSs",
    name: "Remote Procedure Call (RPC)",
    source: "windows_service",
    safety: "critical",
    systemName: "RpcSs",
    state: "running",
    canDisable: true,
    safetyReason: {
      en: "RPC, DCOM, WMI and session management. Disabling this prevents Windows from booting or from letting you log in.",
      vi: "RPC, DCOM, WMI và quản lý phiên. Tắt dịch vụ này sẽ khiến Windows không khởi động được hoặc không đăng nhập được.",
    },
  }),
  item({
    id: "reg:hklm64:{e46eca4f}",
    name: "Microsoft Visual C++ 2015-2022 Redistributable (x64) - 14.38.33130",
    source: "registry_uninstall",
    safety: "critical",
    version: "14.38.33130.0",
    publisher: "Microsoft Corporation",
    sizeBytes: 23068672,
    safetyReason: {
      en: "Shared libraries that hundreds of installed applications link against at load time. Removing one silently breaks every program that depends on it.",
      vi: "Thư viện dùng chung mà hàng trăm ứng dụng đã cài liên kết đến khi khởi chạy. Gỡ một cái sẽ âm thầm làm hỏng mọi chương trình phụ thuộc.",
    },
  }),
];

export function about(): AboutInfo {
  return {
    appVersion: isDemo() ? pkg.version : "26.8.1901",
    appRelease: isDemo() ? pkg.version : "26.8.1901",
    productUrl: "https://cwico.tsudev.com",
    platform: {
      platform: "browser",
      osDescription: "Browser preview (no Windows backend)",
      arch: "x86_64",
      elevated: isDemo(),
      systemRestoreAvailable: isDemo(),
    },
    safetyDbVersion: "1.0.2",
    safetyDbUpdated: "2026-08-19",
    safetyRules: 58,
    safeRules: 29,
    cautionRules: 11,
    criticalRules: 18,
    backupDir: "%LOCALAPPDATA%\\tsudev-cwico\\backups",
  };
}

export async function scan(options?: ScanOptions): Promise<ScanReport> {
  const passes = ["reg", "appx", "appxprov", "svc", "task", "startup"];
  emit?.({ type: "scanStarted", passes: passes.length });
  for (const [index, pass] of passes.entries()) {
    emit?.({ type: "scanPassStarted", pass, index: index + 1, total: passes.length });
    await sleep(180);
    emit?.({
      type: "scanPassFinished",
      pass,
      found: ITEMS.filter((i) => i.id.startsWith(pass)).length,
    });
  }
  emit?.({ type: "scanFinished", total: ITEMS.length, durationMs: 1100 });

  const bySafety: Record<string, number> = {};
  const byKind: Record<string, number> = {};
  for (const i of ITEMS) {
    bySafety[i.safety] = (bySafety[i.safety] ?? 0) + 1;
    byKind[i.source] = (byKind[i.source] ?? 0) + 1;
  }

  return {
    startedAt: new Date().toISOString(),
    options: options ?? DEFAULT_SCAN_OPTIONS,
    items: ITEMS,
    warnings: isDemo()
      ? []
      : [
          {
            pass: "elevation",
            message:
              "browser preview: no Windows backend is attached, so this inventory is fixture data",
          },
        ],
    stats: {
      total: ITEMS.length,
      byKind,
      bySafety,
      reclaimableBytes: ITEMS.reduce((sum, i) => sum + (i.sizeBytes ?? 0), 0),
      durationMs: 1100,
    },
    safetyDbVersion: "1.0.2",
    elevated: isDemo(),
  };
}

export async function buildPlan(
  selections: Selection[],
  options?: PlanOptions,
): Promise<RemovalPlan> {
  const items: PlannedItem[] = [];
  const rejected: RejectedSelection[] = [];

  for (const selection of selections) {
    const found = ITEMS.find((i) => i.id === selection.itemId);
    if (!found) {
      rejected.push({
        itemId: selection.itemId,
        name: selection.itemId,
        code: "unknown_item",
        detail: "not present in the current scan",
      });
      continue;
    }
    // The same two gates the Rust planner applies.
    if (found.safety === "critical") {
      rejected.push({
        itemId: found.id,
        name: found.name,
        code: "protected_component",
        detail: found.safetyReason?.en ?? "classified Critical",
      });
      continue;
    }
    if (
      (found.safety === "caution" || found.safety === "unknown") &&
      !selection.confirmed
    ) {
      rejected.push({
        itemId: found.id,
        name: found.name,
        code: "needs_confirmation",
        detail: "this item is Caution or Unknown and needs an explicit confirmation",
      });
      continue;
    }

    const steps: { kind: string }[] = [];
    if (found.executables.length) steps.push({ kind: "killProcesses" });
    if (found.source === "appx_package" || found.source === "appx_provisioned") {
      steps.push({ kind: "removeAppxPackage" }, { kind: "removeAppxProvisioned" });
    } else if (found.source === "windows_service") {
      steps.push({ kind: "stopServices" }, { kind: "setServiceStartup" });
    } else if (found.source === "scheduled_task") {
      steps.push({ kind: "setTaskEnabled" });
    } else if (found.source === "startup_entry") {
      steps.push({ kind: "removeStartupEntry" });
    } else {
      steps.push({ kind: "runOfficialUninstaller" });
    }
    // Mirrors `RemovalPlan::leaves_residue`: services, tasks and autostart
    // entries are reversible state changes, not installations, so there is
    // nothing to sweep - and their registry key *is* the thing itself.
    const leavesResidue =
      found.source === "registry_uninstall" ||
      found.source === "appx_package" ||
      found.source === "appx_provisioned" ||
      found.source === "leftover";
    if (selection.action === "uninstall_and_deep_clean" && leavesResidue) {
      steps.push({ kind: "deepCleanFiles" }, { kind: "deepCleanRegistry" });
    }

    items.push({
      itemId: found.id,
      name: found.name,
      source: found.source,
      safety: found.safety,
      action: selection.action,
      steps,
      registryKeysAtRisk: found.registryKey ? [found.registryKey] : [],
    });
  }

  const planOptions = options ?? DEFAULT_PLAN_OPTIONS;
  const preamble: { kind: string }[] = [];
  if (items.length && !planOptions.dryRun) {
    if (planOptions.createRestorePoint) preamble.push({ kind: "createRestorePoint" });
    if (planOptions.backupRegistry) preamble.push({ kind: "backupRegistry" });
  }

  return { options: planOptions, preamble, items, rejected };
}

export async function executePlan(plan: RemovalPlan): Promise<RunReport> {
  const total =
    plan.preamble.length +
    plan.items.reduce((sum, item) => sum + item.steps.length, 0);
  emit?.({ type: "runStarted", totalSteps: total, dryRun: plan.options.dryRun });

  let index = 0;
  for (const step of plan.preamble) {
    index += 1;
    emit?.({ type: "stepStarted", step: step.kind, index, total });
    await sleep(320);
    emit?.({
      type: "stepFinished",
      step: step.kind,
      status: "succeeded",
      detail: "done (browser preview)",
    });
  }

  let freed = 0;
  for (const item of plan.items) {
    for (const step of item.steps) {
      index += 1;
      emit?.({ type: "stepStarted", itemId: item.itemId, step: step.kind, index, total });
      await sleep(220);
      emit?.({
        type: "stepFinished",
        itemId: item.itemId,
        step: step.kind,
        status: plan.options.dryRun ? "simulated" : "succeeded",
        detail: "done (browser preview)",
      });
    }
    freed += ITEMS.find((i) => i.id === item.itemId)?.sizeBytes ?? 0;
    emit?.({
      type: "itemFinished",
      itemId: item.itemId,
      name: item.name,
      status: plan.options.dryRun ? "simulated" : "succeeded",
    });
  }

  emit?.({
    type: "runFinished",
    succeeded: plan.items.length,
    failed: 0,
    skipped: 0,
    durationMs: 2400,
  });

  return {
    startedAt: new Date().toISOString(),
    finishedAt: new Date().toISOString(),
    dryRun: plan.options.dryRun,
    restorePoint: plan.options.createRestorePoint
      ? {
          sequenceNumber: 42,
          description: "Tsu Cwico: before removing items",
          createdAt: new Date().toISOString(),
        }
      : undefined,
    registryBackups: [],
    items: plan.items.map((i) => ({
      itemId: i.itemId,
      name: i.name,
      status: plan.options.dryRun ? "simulated" : "succeeded",
      steps: [],
      bytesFreed: 0,
    })),
    succeeded: plan.items.length,
    failed: 0,
    skipped: 0,
    bytesFreed: freed,
    rebootRequired: false,
    warnings: [],
  };
}

export async function tweakCatalog(): Promise<TweakCatalog> {
  const response = await fetch("./tweaks.json").catch(() => null);
  if (response?.ok) return (await response.json()) as TweakCatalog;
  return { schemaVersion: 1, version: "fixture", tweaks: [] };
}

export async function applyTweaks(
  ids: string[],
  enable: boolean,
): Promise<TweakOutcome[]> {
  await sleep(400);
  return ids.map((id) => ({
    id,
    ok: true,
    detail: `simulated ${enable ? "apply" : "revert"} (browser preview)`,
  }));
}

// ---------------------------------------------------------------------------
// Updates
// ---------------------------------------------------------------------------

let updateEmit: ((p: UpdateProgress) => void) | null = null;

export function onUpdateProgress(handler: (p: UpdateProgress) => void) {
  updateEmit = handler;
  return () => {
    updateEmit = null;
  };
}

/**
 * The browser preview reports itself current by default.
 *
 * Append `?preview=update` to the URL to exercise the blocking gate, or
 * `?preview=offline` to see the "could not check" state. Both are how those
 * screens get designed and screenshotted without waiting for a real release.
 * `?preview=demo,update` shows the real upgrade path instead: the previous
 * release (26.11.1002) offered the version being built, with its CHANGELOG notes.
 */
export async function checkForUpdate(): Promise<UpdateStatus> {
  await sleep(500);
  const modes = previewModes();

  const current = isDemo() ? (modes.includes("update") ? "26.11.1002" : pkg.version) : "26.8.1901";
  const base = {
    currentRelease: current,
    currentVersion: current,
  };

  if (modes.includes("update") && isDemo()) {
    return {
      ...base,
      available: true,
      checked: true,
      newRelease: pkg.version,
      newVersion: pkg.version,
      publishedAt: "2026-11-19T13:16:00Z",
      notes: await releaseNotes(pkg.version),
    };
  }
  if (modes.includes("update")) {
    return {
      ...base,
      available: true,
      checked: true,
      newRelease: "26.8.2501",
      newVersion: "26.8.2501",
      publishedAt: "2026-08-25T09:14:00Z",
      // Deliberately the shape a real CHANGELOG section produces - blockquote,
      // fenced code, bullets, inline code - so the dialog is designed against
      // what it will actually receive rather than against clean prose.
      notes: [
        "> **Read this before updating.** This release changes what the tool",
        "> considers safe to remove.",
        "",
        "### Changed",
        "",
        "- Reclassifies **Microsoft Store** as `Critical` after reports that",
        "  removing it left machines unable to update any Store application.",
        "- Adds 14 OEM bloatware rules.",
        "",
        "### Fixed",
        "",
        "- A deep-clean path could match a folder outside the product's install",
        "  location. Check what a removal will do first:",
        "",
        "```",
        "cwico plan --safe-only",
        "```",
      ].join("\n"),
    };
  }
  if (modes.includes("offline")) {
    return {
      ...base,
      available: false,
      checked: false,
      checkError: "error sending request for url (https://github.com/...)",
    };
  }
  return { ...base, available: false, checked: true };
}

export async function installUpdate(): Promise<void> {
  const total = 2_936_012;
  let downloaded = 0;
  while (downloaded < total) {
    downloaded = Math.min(total, downloaded + 180_000);
    updateEmit?.({ downloaded, total, installing: false });
    await sleep(90);
  }
  updateEmit?.({ downloaded: total, total, installing: true });
  await sleep(1500);
  // The real one never returns: the process is replaced by the new version.
  await new Promise(() => {});
}

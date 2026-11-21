/**
 * Optional TsuDEV account + the Pro screen (ADR-0002 P2).
 *
 * Nothing here nags: no prompt to sign in anywhere else in the app, and the
 * subscription terms (PRODUCT_POLICY 7.5, verbatim) stand before the button.
 */

import { useState } from "react";
import type { AccountView, Locale } from "../types";
import type { TranslationKey } from "../i18n";
import { translator } from "../i18n";
import * as api from "../api";
import { Button, Notice, Panel } from "./primitives";
import { TsuBadge } from "./TsuBadge";
import { noticeDate } from "./RenewalNotice";

export function AccountPanel({
  account,
  locale,
  onChange,
}: {
  account: AccountView;
  locale: Locale;
  onChange: (next: AccountView) => void;
}) {
  const t = translator(locale);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const run = async (action: () => Promise<AccountView>) => {
    setBusy(true);
    setError(null);
    try {
      onChange(await action());
    } catch (e) {
      const code = (e as { code?: string })?.code ?? "signin_network";
      setError(code);
    } finally {
      setBusy(false);
    }
  };

  const errorText = (code: string) => {
    const key = `account.err.${code}` as TranslationKey;
    const text = t(key);
    return text ?? t("account.err.signin_network");
  };

  const proLapsed = account.signedIn && !account.proActive && account.proUntil > 0;

  return (
    <>
      <Panel>
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div className="min-w-0">
            <h3 className="flex items-center gap-2 text-sm font-semibold">
              {t("account.title")}
              {account.signedIn && <TsuBadge badge={account.badge} locale={locale} />}
            </h3>
            <p className="mt-1 text-[13px]" style={{ color: "var(--text-secondary)" }}>
              {!account.signedIn
                ? t("account.optional")
                : account.proActive
                  ? t("account.proUntil").replace("{date}", noticeDate(locale, account.proUntil))
                  : proLapsed
                    ? t("account.proExpired").replace("{date}", noticeDate(locale, account.proUntil))
                    : `${t("account.signedIn")} · ${t("account.free")}`}
            </p>
            {account.signedIn && account.devices30d > 1 && (
              <p className="mt-0.5 text-[12px]" style={{ color: "var(--text-muted)" }}>
                {t("account.devices").replace("{n}", String(account.devices30d))}
              </p>
            )}
          </div>
          {account.signedIn ? (
            <Button disabled={busy} onClick={() => void run(api.accountSignOut)}>
              {t("account.signOut")}
            </Button>
          ) : busy ? (
            <Button onClick={() => void api.accountSignInCancel()}>{t("account.cancel")}</Button>
          ) : (
            <Button variant="outline" onClick={() => void run(api.accountSignIn)}>
              {t("account.signIn")} ↗
            </Button>
          )}
        </div>
        {busy && !account.signedIn && (
          <p className="mt-3 text-[12px]" role="status" style={{ color: "var(--text-muted)" }}>
            {t("account.signingIn")} {t("account.signingInHint")}
          </p>
        )}
        {error && (
          <div className="mt-3">
            <Notice tone={error === "signin_cancelled" ? "info" : "warning"} title={errorText(error)} />
          </div>
        )}
      </Panel>

      <Panel>
        <h3 className="mb-1 flex items-center gap-2 text-sm font-semibold">
          {t("pro.title")} <TsuBadge badge="pro" locale={locale} />
        </h3>
        <p className="text-[13px]" style={{ color: "var(--text-secondary)" }}>
          {t("pro.what")}
        </p>
        <p className="mt-2 text-[12px] leading-relaxed" style={{ color: "var(--text-muted)" }}>
          {t("pro.terms").replace("{price}", t("pro.price"))}
        </p>
        <div className="mt-3">
          <Button variant="primary" onClick={() => void api.openProPage()}>
            {account.proActive || proLapsed ? t("pro.renew") : t("pro.buy")} ↗
          </Button>
        </div>
      </Panel>
    </>
  );
}

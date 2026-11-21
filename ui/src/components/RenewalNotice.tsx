/**
 * Renewal reminder and "Pro has expired" notice (PRODUCT_POLICY 7.6).
 *
 * Verbatim ecosystem wording; when to show it is decided in
 * `cwico_core::pro::renewal`. Closable, never blocks the app, shown only
 * when the app is opened - never in the middle of a task.
 */

import type { Locale, RenewalNotice as Notice } from "../types";
import { translator } from "../i18n";
import { Button, Modal } from "./primitives";

/** DD/MM/YYYY (vi) or the locale's own date (en), in local time (7.6). */
export function noticeDate(locale: Locale, unixSeconds: number): string {
  const d = new Date(unixSeconds * 1000);
  if (locale === "vi") {
    const p = (n: number) => String(n).padStart(2, "0");
    return `${p(d.getDate())}/${p(d.getMonth() + 1)}/${d.getFullYear()}`;
  }
  return new Intl.DateTimeFormat("en", { dateStyle: "long" }).format(d);
}

export function RenewalNotice({
  notice,
  locale,
  onRenew,
  onClose,
}: {
  notice: Notice | null;
  locale: Locale;
  onRenew: () => void;
  onClose: () => void;
}) {
  const t = translator(locale);
  if (!notice) return null;
  const renew = notice.kind === "renew";
  const body = renew
    ? t("renew.body")
        .replace("{n}", String(Math.max(1, notice.daysLeft)))
        .replace("{date}", noticeDate(locale, notice.proUntil))
    : notice.cloudDeleteAt === null
      ? t("expired.body")
      : `${t("expired.body")} ${t("expired.cloud").replace("{date}", noticeDate(locale, notice.cloudDeleteAt))}`;

  return (
    <Modal
      open
      title={renew ? t("renew.title") : t("expired.title")}
      onClose={onClose}
      width="max-w-lg"
      footer={
        <>
          <Button variant="ghost" onClick={onClose}>
            {renew ? t("renew.later") : t("expired.close")}
          </Button>
          <Button variant="primary" onClick={onRenew}>
            {renew ? t("renew.primary") : t("expired.primary")} ↗
          </Button>
        </>
      }
    >
      <p className="text-[13px] leading-relaxed">{body}</p>
    </Modal>
  );
}

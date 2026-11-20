/**
 * The "several devices, no Pro" notice (PRODUCT_POLICY 7.3).
 *
 * Text is the ecosystem's verbatim wording; whether to show it at all is
 * decided in `cwico_core::pro::notice_due`. Closable, never blocks the app,
 * no countdown - and the app only asks at start-up, before any work begins.
 */

import type { Locale } from "../types";
import { translator } from "../i18n";
import { Button, Modal } from "./primitives";

export function ProNotice({
  devices,
  locale,
  onPro,
  onBackup,
  onClose,
}: {
  devices: number | null;
  locale: Locale;
  onPro: () => void;
  onBackup: () => void;
  onClose: () => void;
}) {
  const t = translator(locale);
  if (devices === null) return null;

  return (
    <Modal
      open
      title={t("proNotice.title")}
      onClose={onClose}
      width="max-w-lg"
      footer={
        <>
          <Button variant="ghost" onClick={onClose}>
            {t("proNotice.later")}
          </Button>
          <Button variant="primary" onClick={onPro}>
            {t("proNotice.primary")} ↗
          </Button>
        </>
      }
    >
      <p className="text-[13px] leading-relaxed">
        {t("proNotice.body").replace("{n}", new Intl.NumberFormat(locale).format(devices))}
      </p>
      <button
        type="button"
        className="mt-3 text-[13px] underline underline-offset-2"
        style={{ color: "var(--link, var(--primary))" }}
        onClick={onBackup}
      >
        {t("proNotice.backup")}
      </button>
    </Modal>
  );
}

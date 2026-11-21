/**
 * Pro / supporter badge, redrawn from the ecosystem's SVG masters
 * (tsudev-standards `assets/badges/`, PRODUCT_POLICY 4 and 7.5): same colours,
 * pill shape and labels; real text so screen readers and zoom work. No
 * animation here, so reduced motion needs nothing extra.
 */

import type { Badge, Locale } from "../types";
import { translator } from "../i18n";

const pill = "inline-flex h-[18px] items-center gap-1 rounded-full px-2 text-[11px] font-bold leading-none";

export function TsuBadge({ badge, locale }: { badge: Badge; locale: Locale }) {
  const t = translator(locale);
  if (badge === "none") return null;
  const pro = badge === "pro" || badge === "both";
  const donor = badge === "donor" || badge === "both";
  return (
    <span className="inline-flex items-center gap-1.5">
      {pro && (
        <span
          role="img"
          aria-label={t("badge.pro")}
          className={pill}
          style={{ background: "linear-gradient(90deg, #30C8F0, #1E90FF)", color: "#0A0A0A" }}
        >
          PRO
        </span>
      )}
      {donor && (
        <span role="img" aria-label={t("badge.donor")} className={`${pill} tsu-badge-donor border`}>
          <svg aria-hidden="true" viewBox="0 0 16 16" width="10" height="10" fill="currentColor">
            <path d="M8 14s-5.5-3.4-5.5-7.2C2.5 4.6 4.1 3 6 3c.9 0 1.6.4 2 1 .4-.6 1.1-1 2-1 1.9 0 3.5 1.6 3.5 3.8C13.5 10.6 8 14 8 14z" />
          </svg>
          {t("badge.donorText")}
        </span>
      )}
    </span>
  );
}

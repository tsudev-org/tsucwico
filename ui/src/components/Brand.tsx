/**
 * Brand marks (tsudev-standards BRAND_ASSETS.md section 15, ticket LG1).
 *
 * - `Brand`: the PRODUCT - the Tsu Cwico logo and name, as in the header of
 *   cwico.tsudev.com. Used wherever the app identifies itself: header, About,
 *   empty state, update and error screens. Opens the product site.
 * - `MakerBrand`: the "made by" block - the current TsuDEV logo (as in the
 *   website footer, 15.5), opening tsudev.com.
 *
 * Images come from the brand pipeline (`website/scripts/brand-assets.mjs`
 * writes `ui/public/brand/`; CI runs it with `--check`). The name is text,
 * so it scales with the user's font size and stays selectable.
 */

import { openMakerSite, openProductSite } from "../api";

export type BrandSize = "sm" | "md" | "lg";

const SIZES: Record<BrandSize, { logo: string; word: string; gap: string }> = {
  sm: { logo: "h-6 w-6", word: "text-[15px]", gap: "gap-1.5" },
  md: { logo: "h-9 w-9", word: "text-xl", gap: "gap-2.5" },
  lg: { logo: "h-16 w-16", word: "text-4xl", gap: "gap-4" },
};

interface BrandProps {
  size?: BrandSize;
  /** Shown next to the name from `sm` screens up. */
  tagline?: string;
  className?: string;
}

function Mark({
  href,
  label,
  logo,
  name,
  onOpen,
  size = "md",
  tagline,
  className = "",
}: BrandProps & { href: string; label: string; logo: string; name: string; onOpen: () => Promise<void> }) {
  const scale = SIZES[size];
  const open = (event: React.MouseEvent | React.KeyboardEvent) => {
    event.preventDefault();
    void onOpen();
  };
  return (
    <a
      href={href}
      onClick={open}
      onKeyDown={(event) => {
        if (event.key === "Enter" || event.key === " ") open(event);
      }}
      title={href.replace("https://", "")}
      className={`group inline-flex items-center ${scale.gap} rounded-lg px-1 py-0.5 no-underline transition-opacity hover:opacity-85 ${className}`}
    >
      <img
        src={logo}
        alt=""
        aria-hidden="true"
        className={`${scale.logo} shrink-0 object-contain transition-transform duration-200 group-hover:scale-105`}
      />
      <span className={`${scale.word} font-semibold leading-none tracking-tight`} style={{ color: "var(--text-primary)" }}>
        {name}
      </span>
      {tagline && (
        <span className="ml-1 hidden text-[13px] font-normal sm:inline" style={{ color: "var(--text-muted)" }}>
          {tagline}
        </span>
      )}
      {/* Accessible name = visible text + this hint (WCAG 2.5.3 label in name; an aria-label dropped the tagline). */}
      <span className="sr-only"> - {label}</span>
    </a>
  );
}

export function Brand(props: BrandProps) {
  return (
    <Mark
      {...props}
      href="https://cwico.tsudev.com"
      label="mở cwico.tsudev.com / open cwico.tsudev.com"
      logo="./brand/logo-cwico-256.png"
      name="Tsu Cwico"
      onOpen={openProductSite}
    />
  );
}

export function MakerBrand(props: BrandProps) {
  return (
    <Mark
      {...props}
      href="https://tsudev.com"
      label="mở tsudev.com / open tsudev.com"
      logo="./brand/logo-tsudev-96.png"
      name="TsuDEV"
      onOpen={openMakerSite}
    />
  );
}

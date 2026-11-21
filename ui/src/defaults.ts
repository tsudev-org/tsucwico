// PRODUCT_POLICY mục 1 (tsudev-standards v3.8.0): lần chạy đầu - chưa có lựa chọn đã lưu - đi theo hệ thống.
// Máy đã cài vẫn giữ lựa chọn cũ vì App.tsx luôn lưu `cwico.theme` / `cwico.locale` ngay lần chạy trước.
import type { Locale } from "./types";

export type ThemeChoice = "light" | "dark" | "system";
export const THEME_CHOICES: ThemeChoice[] = ["light", "dark", "system"];

/**
 * Giá trị đã lưu hợp lệ thì dùng; `warm` (chế độ Ấm đã bỏ, khối token gỡ ở v5.0.0) ⇒ "light";
 * không có hoặc lạ ⇒ "system" (theo Windows, đổi ngay khi Windows đổi).
 */
export function initialTheme(saved: string | null): ThemeChoice {
  if (saved === "warm") return "light";
  return THEME_CHOICES.includes(saved as ThemeChoice) ? (saved as ThemeChoice) : "system";
}

/** "system" = theo ngôn ngữ của Windows (PRODUCT_POLICY 1.1: có lựa chọn "Theo hệ thống" để quay về mặc định). */
export type LocaleChoice = "system" | Locale;
export const LOCALE_CHOICES: LocaleChoice[] = ["system", "vi", "en"];

/** Giá trị đã lưu hợp lệ thì dùng; không có hoặc lạ ⇒ "system". */
export function initialLocaleChoice(saved: string | null): LocaleChoice {
  return LOCALE_CHOICES.includes(saved as LocaleChoice) ? (saved as LocaleChoice) : "system";
}

/** Ngôn ngữ ưu tiên đầu tiên của hệ thống mà app hỗ trợ (vi, en); ngôn ngữ chưa hỗ trợ ⇒ tiếng Anh. */
export function systemLocale(languages: readonly string[]): Locale {
  for (const tag of languages) {
    const base = tag.toLowerCase().split("-")[0];
    if (base === "vi" || base === "en") return base;
  }
  return "en";
}

/** Ngôn ngữ hiển thị cho một lựa chọn. */
export function resolveLocale(choice: LocaleChoice, languages: readonly string[]): Locale {
  return choice === "system" ? systemLocale(languages) : choice;
}

/** Lần khởi động: lựa chọn đã lưu (vi/en) thắng; không có hoặc "system" ⇒ theo hệ thống. */
export function initialLocale(saved: string | null, languages: readonly string[]): Locale {
  return resolveLocale(initialLocaleChoice(saved), languages);
}

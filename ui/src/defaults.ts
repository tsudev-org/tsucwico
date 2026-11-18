// PRODUCT_POLICY mục 1 (tsudev-standards v3.8.0): lần chạy đầu - chưa có lựa chọn đã lưu - đi theo hệ thống.
// Máy đã cài vẫn giữ lựa chọn cũ vì App.tsx luôn lưu `cwico.theme` / `cwico.locale` ngay lần chạy trước.
import type { Locale } from "./types";

export type ThemeChoice = "light" | "dark" | "system";
export const THEME_CHOICES: ThemeChoice[] = ["light", "dark", "system"];

/** Giá trị đã lưu hợp lệ thì dùng; không có hoặc lạ ⇒ "system" (theo Windows, đổi ngay khi Windows đổi). */
export function initialTheme(saved: string | null): ThemeChoice {
  return THEME_CHOICES.includes(saved as ThemeChoice) ? (saved as ThemeChoice) : "system";
}

/**
 * Giá trị đã lưu hợp lệ thì dùng; không có ⇒ ngôn ngữ ưu tiên đầu tiên của hệ thống mà app hỗ trợ
 * (vi, en); hệ thống dùng ngôn ngữ chưa hỗ trợ ⇒ tiếng Anh.
 */
export function initialLocale(saved: string | null, languages: readonly string[]): Locale {
  if (saved === "vi" || saved === "en") return saved;
  for (const tag of languages) {
    const base = tag.toLowerCase().split("-")[0];
    if (base === "vi" || base === "en") return base;
  }
  return "en";
}

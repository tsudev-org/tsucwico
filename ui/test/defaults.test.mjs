// PRODUCT_POLICY mục 1.4 (tsudev-standards v3.8.0): khởi động với ngôn ngữ hệ thống vi, en và một ngôn ngữ chưa hỗ
// trợ (ja) ⇒ đúng thứ tự vi/en → en; chưa chọn giao diện ⇒ Hệ thống. Chạy: npm --prefix ui test (Node 22, tự bỏ kiểu TS).
import { test } from "node:test";
import assert from "node:assert/strict";
import { initialLocale, initialTheme } from "../src/defaults.ts";

test("ngôn ngữ lần đầu theo hệ thống", () => {
  assert.equal(initialLocale(null, ["vi-VN", "en-US"]), "vi");
  assert.equal(initialLocale(null, ["en-GB"]), "en");
  assert.equal(initialLocale(null, ["ja-JP"]), "en");
  assert.equal(initialLocale(null, ["ja-JP", "vi"]), "vi");
  assert.equal(initialLocale(null, []), "en");
});

test("lựa chọn ngôn ngữ đã lưu thắng ngôn ngữ hệ thống", () => {
  assert.equal(initialLocale("vi", ["en-US"]), "vi");
  assert.equal(initialLocale("en", ["vi-VN"]), "en");
  assert.equal(initialLocale("fr", ["vi-VN"]), "vi");
});

test("giao diện lần đầu là Hệ thống; lựa chọn đã lưu giữ nguyên", () => {
  assert.equal(initialTheme(null), "system");
  assert.equal(initialTheme("warm"), "system");
  assert.equal(initialTheme("dark"), "dark");
  assert.equal(initialTheme("light"), "light");
  assert.equal(initialTheme("system"), "system");
});

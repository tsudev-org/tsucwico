// Chụp lại các ảnh màn hình KHÔNG cần máy Windows thật (docs/screenshots/), từ giao diện thật của app chạy trong trình
// duyệt với dữ liệu mẫu (ui/src/fixtures.ts, `?preview=demo`): kế hoạch gỡ, xác nhận Cẩn trọng, Giới thiệu, màn cập
// nhật (đường nâng cấp thật 26.11.1002 -> bản đang build, ghi chú lấy từ CHANGELOG).
// Ảnh quét và tinh chỉnh (scan-*-vi, tweaks-dark-vi) chụp trên Windows 11 thật - script này KHÔNG đụng tới.
//
// Cách chạy (từ website/):  (cd ../ui && npm run dev -- --port 5317)  rồi  node scripts/chup-anh.mjs http://localhost:5317/
import { chromium } from 'playwright';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const ra = join(dirname(fileURLToPath(import.meta.url)), '../../docs/screenshots');
const goc = process.argv[2] ?? 'http://localhost:5317/';
const browser = await chromium.launch();

async function trang({ locale, scheme, preview = 'demo', w = 1280, h = 860 }) {
  const ctx = await browser.newContext({ viewport: { width: w, height: h }, colorScheme: scheme, locale });
  const page = await ctx.newPage();
  const u = new URL(goc);
  u.searchParams.set('preview', preview);
  await page.goto(u.href);
  await page.waitForTimeout(900);
  return page;
}
const chup = async (page, ten) => {
  await page.mouse.move(0, 0);
  await page.waitForTimeout(400);
  await page.screenshot({ path: join(ra, `${ten}.png`) });
  await page.context().close();
  console.log('[chup-anh]', ten);
};

// Kế hoạch gỡ (tiếng Anh, Sáng)
{
  const p = await trang({ locale: 'en-US', scheme: 'light', h: 940 });
  await p.getByRole('button', { name: 'Scan system' }).first().click();
  await p.getByRole('button', { name: /Select all Safe items/ }).click();
  await p.getByRole('button', { name: /Review plan/ }).click();
  await p.getByRole('dialog').waitFor();
  await chup(p, 'plan-light-en');
}
// Xác nhận riêng mục Cẩn trọng (tiếng Việt, Tối)
{
  const p = await trang({ locale: 'vi-VN', scheme: 'dark' });
  await p.getByRole('button', { name: 'Quét hệ thống' }).first().click();
  await p.getByLabel('Lọc theo mức an toàn').selectOption({ label: 'Cẩn trọng' });
  await p.locator('input[type=checkbox][aria-label]:not(:checked):not(:disabled)').first().click();
  await p.getByRole('dialog').waitFor();
  await chup(p, 'caution-dark-vi');
}
// Giới thiệu (tiếng Anh, Sáng)
{
  const p = await trang({ locale: 'en-US', scheme: 'light' });
  await p.getByRole('button', { name: 'About' }).click();
  await chup(p, 'about-light-en');
}
// Màn cập nhật bắt buộc (tiếng Việt, Tối) và lúc đang cài (tiếng Anh, Tối)
{
  const p = await trang({ locale: 'vi-VN', scheme: 'dark', preview: 'demo,update', w: 1100 });
  await p.getByRole('button', { name: 'Cập nhật' }).waitFor();
  await chup(p, 'update-gate-vi');
}
{
  const p = await trang({ locale: 'en-US', scheme: 'dark', preview: 'demo,update', w: 1100 });
  await p.getByRole('button', { name: 'Update' }).click();
  await p.getByText('Installing…').first().waitFor();
  await chup(p, 'update-installing-en');
}
await browser.close();

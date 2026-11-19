// Quay video giới thiệu (PRODUCT_POLICY mục 5) từ giao diện THẬT của app, chạy trong trình duyệt với
// dữ liệu mẫu (ui/src/fixtures.ts, `?preview=demo`) - không dữ liệu của máy hay người dùng thật nào.
//
// Cách chạy (từ website/):
//   (cd ../ui && npm run dev -- --port 5317)                       # cửa sổ khác
//   FFMPEG=/đường/dẫn/ffmpeg node scripts/quay-demo.mjs http://localhost:5317/
//
// Ra: ../docs/video/gioi-thieu.{mp4,webm,vi.vtt} + gioi-thieu-poster.png. ffmpeg cần libx264 + libvpx-vp9
// (ffmpeg đi kèm Playwright không có; `pip install imageio-ffmpeg` có đủ). sync-content.mjs chép sang public/video/.
import { chromium } from 'playwright';
import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const goc = dirname(dirname(fileURLToPath(import.meta.url)));
const raDir = join(goc, '../docs/video');
const url = new URL(process.argv[2] ?? 'http://localhost:5317/');
url.searchParams.set('preview', 'demo');
const ffmpeg = process.env.FFMPEG ?? 'ffmpeg';
const W = 1280;
const H = 800;

const tam = mkdtempSync(join(tmpdir(), 'cwico-demo-'));
const browser = await chromium.launch();
const ctx = await browser.newContext({
  viewport: { width: W, height: H },
  colorScheme: 'dark',
  locale: 'vi-VN',
  recordVideo: { dir: tam, size: { width: W, height: H } },
});
// Con trỏ nhìn thấy được: video headless không vẽ chuột.
await ctx.addInitScript(() => {
  addEventListener('DOMContentLoaded', () => {
    const c = document.createElement('div');
    c.style.cssText =
      'position:fixed;left:0;top:0;width:22px;height:22px;margin:-11px 0 0 -11px;border-radius:50%;' +
      'background:rgba(56,189,248,.35);border:2px solid #38bdf8;pointer-events:none;z-index:2147483647;' +
      'transition:transform .12s ease;';
    document.body.appendChild(c);
    addEventListener('mousemove', (e) => { c.style.left = e.clientX + 'px'; c.style.top = e.clientY + 'px'; }, true);
    addEventListener('mousedown', () => { c.style.transform = 'scale(.7)'; }, true);
    addEventListener('mouseup', () => { c.style.transform = ''; }, true);
  });
});

const page = await ctx.newPage();
const t0 = Date.now();
const phuDe = [];
let dangMo = null;
const giay = () => (Date.now() - t0) / 1000;
const noi = (chu) => {
  if (dangMo) dangMo.den = giay();
  dangMo = { tu: giay(), den: null, chu };
  phuDe.push(dangMo);
};
const cho = (ms) => page.waitForTimeout(ms);
let chuot = { x: W / 2, y: H / 2 };
async function bam(loc, { cho: sau = 700 } = {}) {
  await loc.scrollIntoViewIfNeeded();
  const b = await loc.boundingBox();
  const dich = { x: b.x + b.width / 2, y: b.y + b.height / 2 };
  await page.mouse.move(dich.x, dich.y, { steps: 18 });
  chuot = dich;
  await cho(250);
  await loc.click();
  await cho(sau);
}

await page.goto(url.href);
await page.mouse.move(chuot.x, chuot.y);
await cho(1200);
noi('Tsu Cwico - gỡ phần mềm thừa trên Windows mà không làm hỏng máy.');
await cho(3000);

noi('Một lượt quét đọc cả sáu nơi Windows giấu phần mềm: chương trình, UWP, gói nạp sẵn, dịch vụ, tác vụ, khởi động.');
await bam(page.getByRole('button', { name: 'Quét hệ thống' }).first(), { cho: 3200 });

noi('Mỗi mục có mức an toàn và lý do ngay trên dòng - không phải rê chuột mới biết.');
const anhBia = giay() + 1;
await page.mouse.move(640, 520, { steps: 25 });
await cho(2600);
await page.mouse.wheel(0, 320);
await cho(1500);
await page.mouse.wheel(0, -320);
await cho(800);

noi('Lọc theo mức: mục Trọng yếu bị chặn cứng, không cách nào đưa vào kế hoạch gỡ.');
const locMuc = page.getByLabel('Lọc theo mức an toàn');
await page.mouse.move(1010, 232, { steps: 15 });
await locMuc.selectOption({ label: 'Trọng yếu' }).catch(() => locMuc.selectOption({ index: 3 }));
await cho(3200);
await locMuc.selectOption({ index: 0 });
await cho(600);

noi('Chọn nhanh mọi mục An toàn.');
await bam(page.getByRole('button', { name: /Chọn tất cả mục An toàn/ }), { cho: 1800 });

noi('Mục Cẩn trọng phải xác nhận riêng - đánh dấu ô chọn chưa đủ.');
await locMuc.selectOption({ label: 'Cẩn trọng' }).catch(() => locMuc.selectOption({ index: 2 }));
await cho(900);
const oCanTrong = page.locator('input[type=checkbox][aria-label]:not(:checked):not(:disabled)').first();
await bam(oCanTrong, { cho: 2600 });
await bam(page.getByRole('button', { name: 'Tôi hiểu, tiếp tục' }), { cho: 900 });
await locMuc.selectOption({ index: 0 });
await cho(600);

noi('Kế hoạch gỡ: tạo điểm khôi phục và sao lưu Registry chạy trước mọi thay đổi.');
await bam(page.getByRole('button', { name: /Xem kế hoạch/ }), { cho: 3400 });
const hop = page.getByRole('dialog');
await hop.hover();
await page.mouse.wheel(0, 400);
await cho(2200);

noi('Chạy thử trước nếu muốn - không thay đổi gì trên máy.');
await bam(hop.getByRole('button', { name: /Chạy thử/ }), { cho: 5200 });
const dong = page.getByRole('button', { name: 'Đóng' });
if (await dong.count()) await bam(dong.first(), { cho: 900 });

noi('Tinh chỉnh hệ thống: chọn riêng từng mục, mục nào cũng hoàn tác được.');
await bam(page.getByRole('button', { name: 'Tinh chỉnh' }), { cho: 1500 });
await bam(page.getByRole('button', { name: 'Chọn bộ khuyến nghị' }), { cho: 2600 });

noi('Giới thiệu: bao nhiêu quy tắc an toàn đang nạp, tệp hoàn tác nằm ở đâu.');
await bam(page.getByRole('button', { name: 'Giới thiệu' }), { cho: 3600 });

noi('Song ngữ Việt - Anh, giao diện Sáng / Tối / Theo hệ thống.');
await bam(page.locator('[aria-label="Sáng"]'), { cho: 2200 });
await bam(page.getByRole('button', { name: 'Phần mềm' }), { cho: 2400 });

noi('Miễn phí, mã nguồn mở MIT, không tài khoản. Tải tại cwico.tsudev.com');
await cho(3500);
dangMo.den = giay();

const video = page.video();
await ctx.close();
await browser.close();
const tho = await video.path();

// Phụ đề WebVTT (tiếng Việt), thời điểm lấy từ chính lượt quay.
const ts = (s) => {
  const ms = Math.round(s * 1000);
  const p = (n, d = 2) => String(n).padStart(d, '0');
  return `${p(Math.floor(ms / 3600000))}:${p(Math.floor(ms / 60000) % 60)}:${p(Math.floor(ms / 1000) % 60)}.${p(ms % 1000, 3)}`;
};
mkdirSync(raDir, { recursive: true });
writeFileSync(
  join(raDir, 'gioi-thieu.vi.vtt'),
  'WEBVTT\n\n' + phuDe.map((c, i) => `${i + 1}\n${ts(c.tu)} --> ${ts(c.den)}\n${c.chu}\n`).join('\n'),
);

const chay = (...a) => execFileSync(ffmpeg, ['-hide_banner', '-loglevel', 'error', '-y', ...a], { stdio: 'inherit' });
chay('-i', tho, '-an', '-c:v', 'libx264', '-preset', 'slow', '-crf', '26', '-pix_fmt', 'yuv420p', '-profile:v', 'high',
  '-movflags', '+faststart', '-r', '25', join(raDir, 'gioi-thieu.mp4'));
chay('-i', tho, '-an', '-c:v', 'libvpx-vp9', '-crf', '36', '-b:v', '0', '-deadline', 'good', '-cpu-used', '4', '-row-mt', '1', '-r', '25',
  join(raDir, 'gioi-thieu.webm'));
chay('-ss', anhBia.toFixed(2), '-i', tho, '-frames:v', '1', join(raDir, 'gioi-thieu-poster.png'));
rmSync(tam, { recursive: true, force: true });

console.log(`[quay-demo] ${giay().toFixed(1)} giây, ${phuDe.length} phụ đề ->`, readdirSync(raDir).join(', '));

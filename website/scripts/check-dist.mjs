// Kiểm tra bản build (chạy sau astro build, CI chặn khi lỗi):
// - security.txt còn hạn 30-366 ngày, có mailto security@tsudev.com, Policy /security/; trang /security/ có nội dung thật;
// - mọi trang có title đúng khuôn, og:site_name = TsuDEV, canonical, description 120-160 ký tự (ECOSYSTEM_IDENTITY mục 2.2);
// - không có script/CSS nội tuyến (CSP chặn); script ngoài chỉ được là /js/loc.js, /js/hien-dan.js và /js/theme.js; không có on*="...";
// - /js/theme.js (chống nháy màu) nạp đồng bộ trong <head>, trước mọi stylesheet; _headers giữ script-src 'self';
// - bộ chọn giao diện Sáng/Tối/Hệ thống là radiogroup ở chân trang, mặc định Hệ thống (PRODUCT_POLICY 1.2);
// - liên kết ra ngoài mở tab mới có rel="noopener noreferrer";
// - mọi liên kết nội bộ (kể cả #neo) trỏ tới trang và id có thật;
// - đủ bộ tài sản nhận diện bắt buộc (BRAND_ASSETS mục 12);
// - hình/video (PRODUCT_POLICY mục 5): mọi src/srcset/poster có tệp thật, <img> có width/height, <video> có phụ đề
//   tiếng Việt và không tự phát; _headers có media-src 'self'; trang chủ có video MP4 + WebM.
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const dist = join(dirname(fileURLToPath(import.meta.url)), '../dist');
const loi = [];
const trang = new Map(); // "/duong-dan/" -> Set(id)
const lienKet = [];      // [tep, href]
const taiNguyen = [];    // [tep, đường dẫn] của src/srcset/poster

function duyet(thuMuc) {
  for (const e of readdirSync(thuMuc, { withFileTypes: true })) {
    const p = join(thuMuc, e.name);
    if (e.isDirectory()) duyet(p);
    else if (e.isFile() && e.name.endsWith('.html')) kiemTra(p, readFileSync(p, 'utf8'));
  }
}

function kiemTra(tep, html) {
  const ten = tep.slice(dist.length).replace(/\\/g, '/');
  const duong = ten.replace(/index\.html$/, '');
  trang.set(duong, new Set([...html.matchAll(/\sid="([^"]+)"/g)].map((m) => m[1])));

  const title = html.match(/<title>([^<]*)<\/title>/)?.[1] ?? '';
  if (!/^(.+ - )?Tsu Cwico \| TsuDEV$/.test(title)) loi.push(`${ten}: title sai khuôn: ${title}`);
  if (!html.includes('<meta property="og:site_name" content="TsuDEV">')) loi.push(`${ten}: thiếu og:site_name TsuDEV`);
  if (!/<link rel="canonical" href="https:\/\/cwico\.tsudev\.com\//.test(html)) loi.push(`${ten}: thiếu canonical`);
  const mota = html.match(/<meta name="description" content="([^"]*)"/)?.[1] ?? '';
  const dai = [...mota.replace(/&[a-z#0-9]+;/g, 'x')].length;
  if (dai < 120 || dai > 160) loi.push(`${ten}: description ${dai} ký tự (cần 120-160)`);

  for (const s of html.matchAll(/<script\b([^>]*)>/gi)) {
    const attr = s[1];
    if (attr.includes('type="application/ld+json"')) continue;
    if (/\ssrc="\/js\/theme\.js"/.test(attr)) continue;
    if (!/\ssrc="\/js\/(loc|hien-dan)\.js"/.test(attr)) loi.push(`${ten}: script không được phép: <script${attr}>`);
  }
  const head = html.match(/<head>([\s\S]*?)<\/head>/)?.[1] ?? '';
  const theme = head.match(/<script\b([^>]*)\ssrc="\/js\/theme\.js"([^>]*)>/);
  if (!theme) loi.push(`${ten}: thiếu <script src="/js/theme.js"> trong <head> (chống nháy màu)`);
  else {
    if (/\s(defer|async)\b|type="module"/.test(theme[1] + theme[2])) loi.push(`${ten}: /js/theme.js phải nạp đồng bộ (không defer/async/module)`);
    const css = head.search(/<link[^>]*rel="stylesheet"/);
    if (css !== -1 && css < theme.index) loi.push(`${ten}: /js/theme.js phải đứng trước stylesheet`);
  }
  // Bộ chọn giao diện (phiếu 20261110-05): đúng ba lựa chọn, radiogroup, nằm ở chân trang; mặc định Hệ thống (PRODUCT_POLICY 1.2).
  const luaChon = [...html.matchAll(/data-chon-giao-dien="([a-z]+)"/g)].map((m) => m[1]).join(',');
  if (luaChon !== 'light,dark,system') loi.push(`${ten}: bộ chọn giao diện cần đúng light,dark,system: ${luaChon}`);
  const viTriChon = html.indexOf('id="giao-dien"');
  if (viTriChon < html.indexOf('<footer') || !/id="giao-dien" role="radiogroup" aria-label="Giao diện"/.test(html)) loi.push(`${ten}: bộ chọn giao diện phải là radiogroup trong <footer>`);
  if (!/<html lang="vi" data-product="cwico" data-theme="dark">/.test(html)) loi.push(`${ten}: mặc định phải là data-product="cwico" data-theme="dark"`);
  if (/<style[\s>]/.test(html) || / style="/.test(html)) loi.push(`${ten}: có CSS nội tuyến (CSP chặn)`);
  if (/\son[a-z]+="/i.test(html)) loi.push(`${ten}: có trình xử lý sự kiện nội tuyến`);
  for (const a of html.matchAll(/<a [^>]*target="_blank"[^>]*>/g)) {
    if (!a[0].includes('rel="noopener noreferrer"')) loi.push(`${ten}: thiếu rel noopener noreferrer: ${a[0]}`);
  }
  for (const a of html.matchAll(/<a [^>]*href="([^"]+)"/g)) lienKet.push([ten, duong, a[1]]);

  for (const m of html.matchAll(/\s(?:src|poster)="(\/[^"]+)"/g)) taiNguyen.push([ten, m[1]]);
  for (const m of html.matchAll(/\ssrcset="([^"]+)"/g)) {
    for (const u of m[1].split(',')) taiNguyen.push([ten, u.trim().split(/\s+/)[0]]);
  }
  for (const img of html.matchAll(/<img\b[^>]*>/g)) {
    if (!/\swidth="\d+"/.test(img[0]) || !/\sheight="\d+"/.test(img[0])) loi.push(`${ten}: <img> thiếu width/height: ${img[0].slice(0, 80)}`);
    if (!/\salt="/.test(img[0])) loi.push(`${ten}: <img> thiếu alt`);
  }
  for (const v of html.matchAll(/<video\b([^>]*)>([\s\S]*?)<\/video>/g)) {
    if (/\sautoplay\b/.test(v[1]) && !/\smuted\b/.test(v[1])) loi.push(`${ten}: video tự phát có tiếng`);
    if (!/\spreload="(none|metadata)"/.test(v[1])) loi.push(`${ten}: video cần preload="none|metadata"`);
    if (!/\sposter="/.test(v[1])) loi.push(`${ten}: video thiếu poster`);
    if (!/<track\b[^>]*kind="captions"[^>]*srclang="vi"/.test(v[2])) loi.push(`${ten}: video thiếu phụ đề tiếng Việt`);
  }
}

duyet(dist);

for (const [ten, duong, href] of lienKet) {
  if (/^(https?:|mailto:)/.test(href)) continue;
  const [p, neo] = href.split('#', 2);
  const dich = p === '' ? duong : p;
  if (!dich.startsWith('/')) { loi.push(`${ten}: liên kết tương đối: ${href}`); continue; }
  if (trang.has(dich)) {
    if (neo && !trang.get(dich).has(decodeURIComponent(neo))) loi.push(`${ten}: neo không tồn tại: ${href}`);
  } else if (!existsSync(join(dist, decodeURIComponent(dich)))) {
    loi.push(`${ten}: liên kết hỏng: ${href}`);
  }
}

for (const f of ['favicon.ico', 'favicon-96x96.png', 'apple-touch-icon.png', 'og-image.png',
  'android-chrome-192x192.png', 'android-chrome-512x512.png', 'android-chrome-maskable-192x192.png',
  'android-chrome-maskable-512x512.png', 'manifest.webmanifest', 'brand/logo-cwico-96.webp', '.well-known/security.txt']) {
  if (!existsSync(join(dist, f))) loi.push(`thiếu tài sản bắt buộc: ${f} (BRAND_ASSETS mục 12)`);
}

// CSP: script theme nạp từ tệp cùng gốc, nên script-src phải có 'self' và KHÔNG được nới bằng 'unsafe-inline'.
{
  const csp = readFileSync(join(dist, '_headers'), 'utf8').match(/Content-Security-Policy:([^\n]*)/)?.[1] ?? '';
  const scriptSrc = csp.match(/script-src([^;]*)/)?.[1] ?? '';
  if (!/'self'/.test(scriptSrc)) loi.push(`_headers: script-src thiếu 'self' - /js/theme.js sẽ bị chặn`);
  if (/unsafe-inline/.test(scriptSrc)) loi.push(`_headers: script-src không được có 'unsafe-inline'`);
}
if (!existsSync(join(dist, 'js/theme.js'))) loi.push('thiếu dist/js/theme.js');

for (const [ten, u] of taiNguyen) {
  if (!u.startsWith('/')) { loi.push(`${ten}: tài nguyên không cùng gốc hoặc tương đối: ${u}`); continue; }
  if (!existsSync(join(dist, decodeURIComponent(u)))) loi.push(`${ten}: tài nguyên không tồn tại: ${u}`);
}
for (const f of ['video/gioi-thieu.mp4', 'video/gioi-thieu.webm', 'video/gioi-thieu.vi.vtt']) {
  if (!existsSync(join(dist, f))) loi.push(`thiếu ${f} (PRODUCT_POLICY mục 5 - chạy scripts/quay-demo.mjs)`);
}
if (!/media-src 'self'/.test(readFileSync(join(dist, '_headers'), 'utf8'))) loi.push(`_headers: thiếu media-src 'self' - video sẽ bị chặn`);

// security.txt (RFC 9116, phiếu 20261125-SEC2 E4): có mailto security@tsudev.com, Policy trỏ /security/, Expires còn
// >= 30 ngày và không quá 1 năm tính từ lúc build - quá hạn thì dừng build để nhớ gia hạn.
{
  const st = readFileSync(join(dist, '.well-known/security.txt'), 'utf8');
  if (!/^Contact: mailto:security@tsudev\.com$/m.test(st)) loi.push('security.txt: thiếu Contact: mailto:security@tsudev.com');
  if (!/^Policy: https:\/\/cwico\.tsudev\.com\/security\/$/m.test(st)) loi.push('security.txt: Policy phải là https://cwico.tsudev.com/security/');
  if (!/^Canonical: https:\/\/cwico\.tsudev\.com\/\.well-known\/security\.txt$/m.test(st)) loi.push('security.txt: Canonical sai');
  const het = Date.parse(st.match(/^Expires: (\S+)$/m)?.[1] ?? '');
  const conNgay = (het - Date.now()) / 86_400_000;
  if (!(conNgay >= 30 && conNgay <= 366)) loi.push(`security.txt: Expires còn ${Math.round(conNgay)} ngày (cần 30-366)`);
}
// Trang /security/ phải có nội dung thật (phiếu SEC2 mục 2): mục Báo lỗ hổng + ít nhất một liên kết "Kiểm chứng".
{
  const f = join(dist, 'security/index.html');
  const h = existsSync(f) ? readFileSync(f, 'utf8') : '';
  if (!h.includes('id="bao-lo-hong"') || !h.includes('mailto:security@tsudev.com') || !h.includes('Kiểm chứng ↗')) loi.push('security/: thiếu nội dung Báo lỗ hổng hoặc bảng bằng chứng');
}
for (const l of loi) console.error('[check-dist] LỖI', l);
console.log(`[check-dist] ${trang.size} trang, ${lienKet.length} liên kết, ${loi.length} lỗi`);
process.exit(loi.length ? 1 : 0);

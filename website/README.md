# Website cwico.tsudev.com

Trang sản phẩm của cwico: [Astro](https://astro.build) dựng trang tĩnh, host trên **Cloudflare Workers Static Assets**
(Worker `tsucwico-website`, chỉ tài sản tĩnh, không có mã chạy phía máy chủ; tên miền `cwico.tsudev.com` gắn bằng
`custom_domain` trong `wrangler.jsonc`). Không dùng Pages: wrangler 4.15x chuyển lệnh tạo dự án Pages mới sang Workers
và tự cài adapter SSR - xem chú thích trong `wrangler.jsonc`.

* **Một nguồn chân lý.** Số quy tắc, danh mục quy tắc và tinh chỉnh đọc từ `data/safety-db.json`, `data/tweaks.json`
  lúc build; trang Cơ chế an toàn là mục "Cơ chế an toàn" của `docs/README.vi.md`; Nhật ký và Bảo mật là
  `CHANGELOG.md`, `SECURITY.md`; ảnh lấy từ `docs/screenshots/`, video từ `docs/video/`. Sửa nội dung ở đó, không sửa
  trong `website/`.
* Phiên bản, liên kết tải và SHA-256 lấy từ GitHub Releases lúc build (trường `digest` GitHub tự tính).
* Hình và video (PRODUCT_POLICY mục 5): `scripts/sync-content.mjs` chép ảnh + video vào `public/` lúc build và sinh
  bản AVIF/WebP của mỗi PNG; component `src/components/Anh.astro` vẽ `<picture>` (cặp Sáng/Tối theo `data-theme`).
  Video tự lưu trữ cùng gốc (CSP `media-src 'self'`), MP4 H.264 + WebM VP9, phụ đề `.vi.vtt`.
* JavaScript: `public/js/theme.js` (giao diện), `public/js/hien-dan.js` (hiện dần khi cuộn ở trang chủ, tắt khi giảm
  chuyển động) và `public/js/loc.js`: ô tìm và nhóm lọc ở trang Quy tắc, Tinh chỉnh (tìm không dấu, tô sáng đúng
  trên chữ có dấu, theo `SEARCH_AND_FILTER.md`). Tắt JS thì danh mục vẫn hiện đầy đủ.
* Tài sản thương hiệu (`public/favicon*`, `android-chrome-*`, `og-image.png`, `brand/`) sinh bằng
  `scripts/brand-assets.mjs` từ MỘT ảnh gốc `assets/brand/source/` (cấu hình `assets/brand/brand.json`); không sửa tay - đổi logo: thay ảnh gốc rồi `node website/scripts/brand-assets.mjs`, CI chạy `--check`.
* Font Be Vietnam Pro tự host (SIL OFL 1.1); token màu sao nguyên văn từ `.standards/tokens/tokens.css`.
* Header bảo mật (HSTS, CSP `default-src 'none'`, `X-Frame-Options: DENY`...) ở `public/_headers`; `security.txt` ở
  `public/.well-known/`. `scripts/check-dist.mjs` chạy sau mỗi lần build: title, `og:site_name`, canonical, độ dài mô
  tả, chặn script/CSS nội tuyến, kiểm mọi liên kết nội bộ và neo `#`, đủ tài sản nhận diện bắt buộc.

```bash
cd website
npm ci
npm run dev          # xem thử ở http://localhost:4321
npm run build        # dựng vào dist/ và kiểm tra
npm run check:a11y   # axe-core WCAG 2.2 AA mọi trang, sáng + tối (lần đầu: npx playwright install chromium)
npm run deploy       # build rồi wrangler deploy (cần `npx wrangler login` với tài khoản TsuDEV)
```

Sau mỗi lần phát hành bản mới hoặc sửa `data/*.json`, chạy lại `npm run deploy` để trang cập nhật.

Sinh lại tài sản thương hiệu (khi thương hiệu đổi):

```bash
node website/scripts/brand-assets.mjs           # sinh lại từ assets/brand/source/
node website/scripts/brand-assets.mjs --check   # như CI
```

Chụp lại ảnh và quay lại video giới thiệu (khi giao diện đổi) - app chạy trong trình duyệt với dữ liệu mẫu
(`ui/src/fixtures.ts`, `?preview=demo`), không dữ liệu máy thật:

```bash
(cd ui && npm run dev -- --port 5317)                         # cửa sổ khác
cd website
node scripts/chup-anh.mjs http://localhost:5317/              # plan/caution/about/update-* trong docs/screenshots/
FFMPEG=/đường/dẫn/ffmpeg node scripts/quay-demo.mjs http://localhost:5317/   # docs/video/ (cần libx264 + libvpx-vp9)
```

`scan-*-vi.png` và `tweaks-dark-vi.png` chụp trên Windows 11 thật - chụp lại bằng tay trên máy Windows.

Ghi chú vận hành: vùng `tsudev.com` bật Cloudflare Web Analytics tự chèn `beacon.min.js`; CSP của trang này chặn nó (lỗi
console vô hại, không gửi dữ liệu). Muốn tắt hẳn thì tắt tự chèn cho host này ở cấu hình vùng, không nới CSP.

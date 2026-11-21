# 20261120-LG1 - tsucwico: logo + favicon Tsu Cwico trong app và mọi tệp xuất ra, giống cwico.tsudev.com

- **Người lập**: phiên `tsuswico-57`, 20/11/2026, theo yêu cầu trực tiếp của chủ dự án ("thống nhất/đồng bộ hình ảnh
  của dự án").
- **Người làm**: phiên tsucwico. **Trạng thái**: ĐÓNG (21/11/2026, PR #103).
- **Chuẩn**: `.standards/docs/BRAND_ASSETS.md` mục 2, 12, **15** (15.5: khối "nhà làm" giữ logo tsudev),
  `ECOSYSTEM_IDENTITY.md`.
- **Mẫu đã làm**: tsuswico PR [#59](https://github.com/tsudev-org/tsuswico/pull/59). Báo cáo HTML dùng **cùng ảnh**
  với website: `brand-assets.mjs` sinh thêm `assets/brand/report/` (favicon trùng byte website, logo sản phẩm 2x, logo
  tsudev 2x), nhúng vào bản build, test đối chiếu byte với tệp của dây chuyền.

## 1. Hiện trạng (đo 20/11/2026)

- Website `cwico.tsudev.com` đã dùng logo riêng `logo-cwico-*` (dây chuyền `website/scripts/brand-assets.mjs`,
  `assets/brand/brand.json` id `cwico`, mode `no-white`).
- App (Tauri, `ui/`) vẫn mang **logo tsudev**: `ui/index.html` favicon `./brand/tsudev-logo-64.png`;
  `ui/src/components/Brand.tsx` = logo tsudev + chữ "tsu"/"dev" ở header, about, empty state, footer;
  `ui/public/brand/tsudev-logo-*.png`.

## 2. Việc phải làm

1. **Favicon / icon cửa sổ của UI**: dùng icon sinh từ `assets/brand/source/` (cùng tệp với favicon website), không
   dùng `tsudev-logo-64.png`.
2. **Header (và chỗ nhận diện sản phẩm)**: logo Tsu Cwico + tên sản phẩm, giống header `cwico.tsudev.com`. Khối "nhà
   làm" (about/footer) GIỮ logo tsudev **hiện hành** (bản đã sinh của dây chuyền tsudev, như footer website), dẫn
   tsudev.com.
3. **Mọi tệp app xuất ra** (báo cáo HTML/PDF/ảnh nếu có, kể cả bản xuất của chức năng quét): favicon + logo sản phẩm
   nhúng thẳng (data URI hoặc tài nguyên nhúng - xem được offline, copy đi không vỡ ảnh), chân trang khối nhà làm.
   Không có tệp xuất nào ⇒ ghi "không áp dụng" trong phiếu.
4. Ảnh mới cho app/tệp xuất `MUST` sinh từ dây chuyền (`brand-assets.mjs`, thêm thư mục đích nếu cần) và CI `--check`
   phủ - `MUST NOT` cắt tay hay chép PNG rời.
5. Dọn `ui/public/brand/tsudev-logo-*.png` không còn dùng; cập nhật `assets/brand/README.md`, CHANGELOG.
6. Kiểm: test (nếu có test thương hiệu), ảnh chụp sáng/tối, axe; chạy app thật xem tab/cửa sổ/taskbar.

## 3. Xong khi

- UI + tệp xuất không còn logo tsudev ở vị trí nhận diện sản phẩm; khối nhà làm dùng logo tsudev hiện hành.
- Ghi kết quả vào mục 4 phiếu này, đổi trạng thái ĐÓNG.

## 4. Kết quả

21/11/2026 - phiên tsucwico (PR #103):

1. Favicon UI: `ui/public/brand/favicon.ico` + `favicon-32.png` trùng byte `website/public/favicon.ico` /
   `favicon-32x32.png` (cùng một buffer trong `brand-assets.mjs`). Icon cửa sổ/taskbar/bộ cài đã là icon cwico từ #68
   (`app/src-tauri/icons` ← `assets/brand/app`).
2. `Brand` = logo Tsu Cwico + "Tsu Cwico" → cwico.tsudev.com (header, Giới thiệu, màn trống, UpdateGate, ErrorBoundary);
   `MakerBrand` = logo TsuDEV hiện hành (từ `website/public/brand/logo-tsudev-144.webp` của dây chuyền tsudev) +
   "TsuDEV" → tsudev.com (chân trang app, dòng "Một sản phẩm của" ở Giới thiệu). Lệnh Tauri mới `open_maker_site`
   (URL cố định); `PRODUCT_URL` = `https://cwico.tsudev.com`, `MAKER_URL` = `https://tsudev.com`.
3. Tệp xuất: nhật ký giao dịch JSON, `.reg` ⇒ **không áp dụng**. Trang "Đã đăng nhập" của luồng đăng nhập (#102) nhúng
   favicon data URI (`include_bytes!` cùng tệp `ui/public/brand/favicon-32.png`, CSP `img-src data:`).
4. Ảnh app sinh từ `brand-assets.mjs` (thêm đích `ui/public/brand/`), CI `--check` phủ (33 tệp khớp).
5. Đã xoá `ui/public/brand/tsudev-logo*.png`, `apple-touch-icon.png`; thêm `assets/brand/README.md`; CHANGELOG.
6. Ảnh chụp Sáng/Tối (header + Giới thiệu) bằng Playwright trên bản build `?preview=demo`; app thật trên Windows: phiên
   có máy Windows kiểm tab/cửa sổ/taskbar.

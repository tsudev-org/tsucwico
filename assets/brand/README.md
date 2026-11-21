# assets/brand - nhận diện Tsu Cwico

Dây chuyền duy nhất: `node website/scripts/brand-assets.mjs` (CI chạy `--check`; lệch nguồn ⇒ đỏ). Chuẩn:
tsudev-standards `BRAND_ASSETS.md` mục 15, phiếu `logs/handover/20261120-LG1_*`.

| Nguồn / cấu hình | Sinh ra |
| --- | --- |
| `source/logo-cwico.png` + `brand.json` | website `website/public/` (favicon, PWA, `og-image.png`, `brand/logo-cwico-*`); icon desktop `app/` (Tauri, bộ cài) |
| như trên | **app** `ui/public/brand/`: `favicon.ico` + `favicon-32.png` (trùng byte website), `logo-cwico-256.png` (header, Giới thiệu, màn trống, cập nhật, lỗi) |
| `website/public/brand/logo-tsudev-144.webp` (logo TsuDEV hiện hành, nhận từ dây chuyền tsudev) | **app** `ui/public/brand/logo-tsudev-96.png` - chỉ cho khối "nhà làm" (chân trang, "Một sản phẩm của", mục 15.5) |

`tsudev-logo*.png`, `tsudev-wordmark*.png` ở thư mục này chỉ dùng cho đầu README/SECURITY/CONTRIBUTING (ECOSYSTEM_IDENTITY
mục 2.1). KHÔNG cắt tay hay chép PNG rời vào `ui/` hay `website/`.

Tệp xuất của app: nhật ký giao dịch (JSON) và bản sao lưu `.reg` không có phần hình ⇒ không áp dụng logo. Trang
"Đã đăng nhập" mà app trả cho trình duyệt (đăng nhập TsuDEV) nhúng favicon dạng data URI.

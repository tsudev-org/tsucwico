# PHIẾU YÊU CẦU - Bản sắc riêng của Tsu Cwico (tsudev-standards v5.0.0)

- **Mã phiếu**: 20261121-STD50
- **Từ**: phiên tsudev-standards, 21/11/2026 - **Đến**: phiên kế tiếp của `tsucwico`
- **Trạng thái**: MỞ
- **Nguồn**: https://github.com/tsudev-org/tsudev-standards/releases/tag/v5.0.0 - `docs/DESIGN_SYSTEM.md` mục 1.2,
  CHANGELOG 5.0.0 "Hướng dẫn nâng cấp"
- **Bối cảnh**: chủ dự án muốn mỗi sản phẩm có đặc trưng nổi bật riêng (màu, giao diện, bố cục). Trước v5.0.0, 9/11
  sản phẩm dùng cùng một màu xanh. Từ v5.0.0: nền/chữ/trạng thái/tiếp cận là chung; màu nhấn, bố cục, bo góc, hoạ
  tiết, font tiêu đề là riêng, khai ở `.standards/tokens/products/cwico.json` - KHÔNG tự khai lại tại repo này.

## Bản sắc đã chốt cho `tsucwico`

| Trường | Giá trị |
| --- | --- |
| `data-product` | `cwico` |
| Màu chính Tối / Sáng | `#0EB5CE` / `#087485` (hue OKLCH 212; lý do: mũi tên xanh ngọc trên khiên navy, đẩy về phía navy để tách khỏi Tsu Contact) |
| Bố cục `layout` | `trinh-dien` - hero hai cột (chữ trái, ảnh chụp app lớn tràn phải) + khối tính năng dạng bento |
| Bo góc `shape` | `sac` - 2/3/4px, nút 3px |
| Hoạ tiết `motif` | `luoi-quet` (tĩnh, ở hero + `og-image`, phủ ≤ 15%) |
| Font tiêu đề | Be Vietnam Pro (tự host woff2 bộ `vietnamese`, tối đa 2 độ đậm) |

Muốn đổi bất kỳ giá trị nào: PR vào tsudev-standards (sửa `hue`/`layout`... + chạy `gen-product-palette.mjs`), cổng
kiểm chống trùng phải xanh. KHÔNG ghi đè tại repo này.

## Việc cần làm - chia hai PR, mỗi PR một phiên

**PR 1 - màu + hình** (`feat(ui): bản sắc cwico theo tsudev-standards v5.0.0`):
1. Sync `.standards/` lên v5.0.0 (bước `curl` sync-standards.sh trước, theo `docs/SYNC.md`).
2. Web: nạp `.standards/tokens/tokens.css` rồi `.standards/tokens/products.css`; `<html data-product="cwico">` (cạnh
   `data-theme`). Desktop/di động: gộp đè `products/cwico.json` lên `color.light`/`color.dark` lúc build.
3. Xoá bảng màu tự khai (`palette.css`, `extensions.*.json`, dải `--color-*` riêng) và mã màu cứng còn sót.
4. Giá trị đã lưu `warm` đọc thành `light` (khối `warm` đã gỡ khỏi token).
5. Nút dùng `var(--radius-control)`; h1-h3 và số liệu lớn dùng `var(--font-display)`.
6. Ảnh chụp trước/sau, Sáng + Tối, đính vào PR. Cổng kiểm + test xanh.

**PR 2 - bố cục** (`feat(ui): bố cục trinh-dien + hoạ tiết luoi-quet`):
1. Dựng phần THÂN trang chủ website / màn hình đầu app theo `trinh-dien`; header, khu tài khoản, chân trang, bộ chọn
   giao diện giữ theo `ECOSYSTEM_IDENTITY.md` mục 4.
2. Hoạ tiết `luoi-quet` tĩnh ở hero, vẽ bằng `accent`/`glow`; sinh lại `og-image.png` cùng hoạ tiết.
3. Ảnh chụp trước/sau, Sáng + Tối.

## Việc riêng của repo này

- **TS-8 (đóng ở v5.0.0):** gỡ dải cam `--color-dev-50..900` ở `ui/src/index.css` và chỗ dùng trong `ui/src/App.tsx`; khối "nhà làm" dùng logo TsuDEV dạng ảnh (`BRAND_ASSETS.md` mục 15.5), không tô chữ "dev" cam.
- Màu chính đổi từ xanh dùng chung sang xanh lơ ngả petrol của mũi tên trên khiên (hue 212).

## Kết quả xử lý

- 21/11/2026 phiên tsucwico - **PR 1 (#104)**: sync v5.0.0; app `ui/` nạp `tokens.css` + `products.css`,
  `<html data-product="cwico">`, xoá dải `--color-tsu-*` / `--color-dev-*` (TS-8) + khối ghi đè màu Sáng/Tối tự khai;
  `warm` ⇒ `light` (+ test); nút `--radius-control`, `rounded-sm/md/lg` của Tailwind đọc thẳng bộ `sac`, `xl` theo `lg`;
  h1-h3 + số liệu lớn `--font-display` (Be Vietnam Pro 600/700 tự host, latin + vietnamese). Website: bỏ
  `src/styles/tokens.css` (bản cũ có `warm`) + `palette.css`, nạp thẳng `.standards/tokens/*`, `data-product`,
  bo góc theo token, `check-dist` canh `data-product`. Khối nhà làm dùng logo TsuDEV dạng ảnh ở PR LG1 (#103).
  Ảnh chụp Sáng/Tối trước/sau đã kiểm bằng Playwright. **PR 2 (bố cục `trinh-dien` + `luoi-quet`)**: phiên kế tiếp. 

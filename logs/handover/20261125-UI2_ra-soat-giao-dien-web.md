# PHIẾU - UI2: rà quét sâu giao diện web + hiện đại hoá (tsucwico)

- **Mã phiếu**: 20261125-UI2 (phiếu con của `~/projects/logs/handover/20261125-UI2_ra-soat-giao-dien-web-toan-he.md`)
- **Từ**: chủ dự án (25/11/2026), phát bởi phiên `tsudev-3e` - **Đến**: agent của repo `tsucwico`
- **Trạng thái**: MỞ
- **Phạm vi**: A + B (thuộc hệ sinh thái TsuDEV)
- **Ủy quyền**: chủ dự án cho agent TỰ MỞ TRÌNH DUYỆT (Playwright/Chromium headless) để rà website của repo này.

## Việc cần làm

1. Đọc phiếu toàn hệ (đường dẫn ở trên) - mục 0 là ba nguyên nhân "trang nhúng khi Ctrl+F5" đã gặp ở tsudev.com; kiểm
   cả ba ở repo này TRƯỚC (fade-in ở màn đầu, font không preload/dự phòng lệch số đo, scroll-reveal ẩn trước khi đo).
2. Làm phần A (rà quét sâu bằng trình duyệt: quay khung hình tải lạnh, ảnh 3 khổ × Sáng/Tối, console/mạng, tiếng
   Việt, bàn phím, LCP/CLS/INP).
3. Làm phần B (hiện đại hoá) - thay đổi bảng màu/font CHUNG chờ bản phát hành tsudev-standards, không tự đổi hex.
4. Ghi từng lỗi + cách sửa vào mục Kết quả; sửa theo PR nhỏ; `akb new issue` cho lỗi tốn > 10 phút.

## Kết quả xử lý

- 25/11/2026 - tsudev-3e: mở phiếu.
- 26/11/2026 - tsucwico (SES-20261126-2316-6da2): **phần A đợt 1** (Playwright headless, bản build cục bộ qua
  `scripts/may-chu-tinh.mjs` = cùng CSP production). Mục 0: nguyên nhân 1 KHÔNG có (màn đầu không opacity 0), nguyên nhân 3
  KHÔNG có (`hien-dan.js` đo trước rồi mới ẩn, chỉ khối dưới màn đầu), **nguyên nhân 2 CÓ**: Be Vietnam Pro không preload,
  không font dự phòng đo số ⇒ quay khung hình tải lạnh (chặn cache, 150 ms/4 Mbps): 1366 px tiêu đề 5 → 4 dòng, 360 px
  đầu trang 2 → 1 hàng khi font tới; CLS trang chủ 0,087 (360) / 0,14 (1366), production 0,035. Sửa (PR UI2-font):
  preload 400 + 600 (`latin`, `vietnamese`), `@font-face 'Be Vietnam Pro Fallback'` (số đo của tsudev.com) chèn vào
  `--font-family` ⇒ CLS **0** ở 360/1366 × Sáng/Tối, khung vẽ đầu tiên đã đúng font. Quét 9 trang × 360/768/1366 ×
  Sáng/Tối (54 tổ hợp): 0 lỗi (cuộn ngang, tràn, ảnh vỡ, h1, tên truy cập, console, HTTP); soi ảnh 360 bằng mắt ổn.
  Còn: đo lại trên production sau deploy (token CF), Lighthouse/INP, phần B.
- 27/11/2026 - tsucwico-6da2: deploy `b64df5cd`; đo production (context mới, mạng 150 ms/4 Mbps) 8 lần: 1366 px CLS 0-0,035,
  360 px CLS 0 hoặc 0,257 (~50%) - lần vẽ đầu trước khi font preload tới, máy đo chỉ có DejaVu nên font dự phòng đo số
  không áp dụng. Đề xuất `font-display: optional` cho 400/600 - chờ chủ dự án chốt.

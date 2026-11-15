# PHIẾU YÊU CẦU - Hồ sơ trên các nền tảng xác minh (Best Practices, Scorecard, phát hành, kênh gói) cho `tsudev-cwico`

- **Mã phiếu**: 20261115-01 (phiếu con)
- **Từ**: phiên agent-files (repo `tsufiles`), chủ dự án yêu cầu 15/11/2026 - **Đến**: phiên kế tiếp của repo `tsudev-cwico`
- **Trạng thái**: MỞ
- **Phiếu mẹ (ĐỌC TRƯỚC, mục 0 nguyên tắc + mục 3 quy trình)**: `~/projects/logs/handover/20261115-01_yeu-cau-ho-so-xac-minh-toan-he.md`
- **Công cụ + mẫu đã đạt passing 100%**: `~/projects/logs/tools/bestpractices/` (`README.md`, `bp.cjs`, `mau-tsufiles.json`);
  mẫu hồ sơ dạng đọc: `~/projects/tsufiles/docs/best-practices.md`.

## 1. Hiện trạng đo 15/11/2026

MIT, 2 Release. Bản mới nhất **KHÔNG có tệp SHA256SUMS** (có 2 tệp chữ ký) - chặn Scoop (phiếu cwico 20261113-02) và tiêu chí `delivery_unsigned`.

## 2. Việc của repo này (theo thứ tự)

- [ ] 2.1. Sửa phát hành trước: thêm SHA256SUMS liệt kê mọi tệp (sinh trong workflow), ký, `attest-build-provenance`; KHÔNG đổi tên tệp cũ nếu có trình cập nhật đọc chúng.
- [ ] 2.2. Sau đó Best Practices passing + `scorecard.yml`; rồi Scoop (bucket `tsudev-org/scoop-bucket`).
- [ ] 2.3. Hồ sơ Best Practices (nếu áp dụng): soạn `docs/best-practices.json` + `.md` (mỗi `Met` có URL bằng chứng trên
  `main`), gộp PR, rồi `bp.cjs new` -> `fill` chạy thử -> xem ảnh -> `fill --submit` -> `inspect` ra passing 100% -> huy hiệu README.
- [ ] 2.4. Ghi kết quả có số đo dưới đây + đánh dấu dòng `tsudev-cwico` ở mục 4 phiếu mẹ + một dòng mục 7 kế hoạch `20261113-02`.

**Nhắc lại**: không ghi `Met` cho điều repo chưa làm. Chưa đạt thì sửa repo trước, hoặc ghi `Unmet`/`N/A` trung thực.
Tự chứng nhận sai bị phát hiện sẽ làm hỏng hồ sơ SignPath của cả hệ.

## Kết quả xử lý (phiên nhận điền)

-

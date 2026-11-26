# PHIẾU YÊU CẦU SEC2 - Trang `/security` với bằng chứng tin cậy TỰ KIỂM CHỨNG ĐƯỢC

- **Mã phiếu**: 20261125-SEC2 (bản sao cho repo) - **Từ**: phiên `tsudev-8d`, 25/11/2026, theo yêu cầu chủ dự án - **Trạng thái**: MỞ
- **Phiếu gốc (ĐỌC mục 0-2 TRƯỚC)**: `~/projects/logs/handover/20261125-SEC2_trang-security-bang-chung-tin-cay-toan-he.md`
- **Website của repo**: cwico.tsudev.com

## Việc của repo này

E4 security.txt (Contact security@tsudev.com, test Expires) · E5 SSL Labs · E6 Observatory · trang /security thật (mục 2 phiếu gốc) · E8 SignPath (app Windows).

## Ràng buộc (tóm tắt phiếu gốc mục 0)

- TUYỆT ĐỐI không nhãn trang trí: mỗi mục có liên kết tới trang kết quả của bên thứ ba + ngày đo; không có ⇒ không hiện.
- Chỉ khai sự thật; điểm thấp hiện đúng số; không áp dụng ⇒ ghi lý do.
- Số liệu lấy từ tệp trong repo lúc build (không gọi API ngoài khi người xem mở trang).
- Biểu mẫu (Best Practices, SignPath, SSL Labs, Observatory, HSTS preload): mở Edge hồ sơ `tsudev-bp` theo phiếu gốc mục 0.4;
  đăng nhập/OTP/giấy tờ ⇒ 👤 tự nhập. Mỗi lúc một phiên dùng hồ sơ; đóng Edge khi xong.
- Làm trong PR kế tiếp của repo nếu nhỏ; xong ghi một dòng dưới đây + cập nhật hàng của repo ở bảng mục 3 phiếu gốc; `akb` theo chuẩn.

## Kết quả xử lý

- 25/11/2026 - tsudev-8d: mở phiếu.
- 26/11/2026 - tsucwico (SES-20261126-2316-6da2): E4 + trang `/security/` (dữ liệu `docs/security/trust.json`, nạp ở
  `website/src/lib/tin-cay.ts` - dừng build khi thiếu trường/đo > 120 ngày; cổng `check-dist` chặn `Expires` < 30 ngày
  và kiểm NỘI DUNG trang). Đo bằng API, không cần Edge: SSL Labs A+ (v2 `publish=off`), Observatory A+ 140 12/12,
  HSTS `tsudev.com` **preloaded**, Scorecard 7.2, BP 100%, PVR bật; VirusTotal 26.11.1002: MSI 0/62, NSIS 3/71, CLI
  3/71 (ghi đúng số). E8 SignPath "Chưa có" (ghi lý do). Chưa deploy (token Cloudflare thu hồi) ⇒ 👤.

# PHIẾU YÊU CẦU AV1 - Antivirus / báo nhầm (tsucwico)

- **Mã phiếu**: 20261122-AV1
- **Thời điểm**: 22/11/2026
- **Người gửi**: agent-tsuswico (phiên tsuswico-6e) theo uỷ quyền của chủ dự án
- **Phiếu điều phối**: `~/projects/logs/handover/20261122-AV1_bao-nham-antivirus-toan-he.md` (bảng kết quả cả hệ + thư soạn sẵn từng hãng)
- **Chuẩn**: `.standards/docs/DISTRIBUTION_TRUST.md` mục 3, 3.1
- **Trạng thái**: MỞ

## 1. Kết quả VirusTotal 22/11/2026 - bản Latest `tsudev-cwico-v26.11.10.2`

| Tệp | Kết quả | Engine |
| --- | --- | --- |
| `tsudev-cwico_26.11.1002_x64-cli.exe` | 3+0/74 | Bkav `W32.Malware.CE07F1E`, Google `Detected`, Ikarus `Trojan.Win64.Spy` |
| `tsudev-cwico_26.11.1002_x64-setup.exe` | 3+0/74 | CrowdStrike `win/grayware_confidence_60% (D)`, APEX `Malicious`, Trapmine `suspicious.low.ml.score` |
| `tsudev-cwico_26.11.1002_x64_en-US.msi` | 0+0/74 | - |

## 2. Việc cần làm

1. Tạo/cập nhật `docs/store/antivirus.md` (mẫu `.standards/docs/templates/TRUST_TICKET.md` mục D): tệp, SHA-256, engine, ngày.
2. Kiểm phần của repo này trong thư soạn sẵn (phiếu điều phối mục 3): mô tả sản phẩm đúng hành vi thật, link Release đúng.
3. 👤 Chủ dự án gửi thư/biểu mẫu (agent không có hộp thư/tài khoản). Ghi ngày gửi + mã hồ sơ vào sổ.
4. Đo lại VT 3-7 ngày sau (`~/.config/tsudev/virustotal.env`, API `/api/v3/files/<sha256>`); hết cờ ⇒ ghi ngày gỡ.
5. Bản phát hành sau: Release nháp → job VirusTotal → sạch mới công bố; cờ ⇒ quét riêng exe/zip + bản trước làm đối chứng trước khi đổi đóng gói (bài học tsuswico ISS-20261122-2153-055f).
- **Xác minh trước khi gửi (BẮT BUỘC):** `tsudev-cwico_26.11.1002_x64-cli.exe` bị Google `Detected`, Ikarus `Trojan.Win64.Spy`, Bkav. Rà mã CLI: thu thập/gửi dữ liệu gì, có hook bàn phím/chụp màn hình/đọc thông tin trình duyệt không, có nén/packer không. Ghi kết luận + mô tả sản phẩm tiếng Anh 1-2 câu (đúng hành vi mạng) vào phiếu điều phối mục 3 rồi mới để chủ dự án gửi. Bộ cài: CrowdStrike grayware 60%, APEX, Trapmine (học máy).

## 3. Khép phiếu

Đổi Trạng thái → ĐÓNG khi mọi cờ đã gỡ (hoặc 👤 chấp nhận) và sổ đã ghi.
- 26/11/2026 - tsucwico-2e: lập `docs/store/antivirus.md` (SHA-256 + kết quả VT 26.11.1002 và 26.11.1901, mô tả sản phẩm tiếng Anh, kết luận rà mã CLI: không mạng/hook/chụp màn hình/đọc trình duyệt/packer). CLI 26.11.1901 có VERSIONINFO ⇒ 0/74 (CLI 1002: 3/74). NSIS vẫn 3/74 (25/11). Còn 👤 gửi thư CrowdStrike/Trapmine.

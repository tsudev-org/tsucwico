# Sổ antivirus / báo nhầm - Tsu Cwico

Theo `.standards/docs/DISTRIBUTION_TRUST.md` mục 3, 3.1 và mẫu `.standards/docs/templates/TRUST_TICKET.md` mục D.
Phiếu: `logs/handover/20261122-AV1_bao-nham-antivirus.md`, `logs/handover/20261119-VT1_*`.
Đo bằng VirusTotal API `/api/v3/files/<sha256>` (khoá ở `~/.config/tsudev/virustotal.env`, không in).

## 1. Kết quả đo (cập nhật 26/11/2026)

| Bản | Tệp | SHA-256 | VirusTotal | Engine gắn cờ | Lượt quét |
| --- | --- | --- | --- | --- | --- |
| 26.11.1002 (Latest) | `tsudev-cwico_26.11.1002_x64-cli.exe` | `53d9967a0736aa6e96e5d3d9031a2b0a24cade9168354d15ee3c07e084dea5de` | **3/74** | Bkav `W32.Malware.CE07F1E`, Google `Detected`, Ikarus `Trojan.Win64.Spy` | 22/11/2026 15:22 UTC |
| 26.11.1002 | `tsudev-cwico_26.11.1002_x64-setup.exe` (NSIS) | `ebd71b9415c7aa97bd851da76bc8e1081372ed7671a7f6b0cb2e284fcb93df28` | **3/74** | CrowdStrike `win/grayware_confidence_60% (D)`, APEX `Malicious`, Trapmine `suspicious.low.ml.score` | 22/11/2026 15:23 UTC |
| 26.11.1002 | `tsudev-cwico_26.11.1002_x64_en-US.msi` | `9ac86c74702ceabe11711c9b5304a8051db5345bd84e3f30ab8ad8cc5931f821` | 0/74 | - | 22/11/2026 15:23 UTC |
| 26.11.1901 (nháp) | `tsucwico_26.11.1901_windows-x64-cli.exe` | `b56bf4bcf9f94d69debdcfc8bb48bb79f8783030ec6beb03a477440c5f908bf6` | 0/74 | - | 19/11/2026 13:26 UTC |
| 26.11.1901 (nháp) | `tsucwico_26.11.1901_windows-x64-setup.exe` (NSIS) | `28d7b74c990bdf54d94e4921217f6a8181eb609a65b3dc021f7a43d47add2537` | **3/74** | CrowdStrike `win/grayware_confidence_60% (D)`, APEX `Malicious`, Trapmine `malicious.moderate.ml.score` | 25/11/2026 04:36 UTC |
| 26.11.1901 (nháp) | `tsucwico_26.11.1901_windows-x64.msi` | `cef57526eb122854ba01e34ff36f1fa3886ab47ed81dcd042323ff6a44fc4eaa` | 0/74 | - | 19/11/2026 13:33 UTC |

**Nhận xét:** CLI 26.11.1901 có tài nguyên thông tin phiên bản (CompanyName TsuDEV, FileVersion; #87) ⇒ 0/74, trong khi CLI
26.11.1002 (chưa có) bị 3/74 ⇒ bản 26.11.1901 tự gỡ cờ CLI khi Publish. Bộ cài NSIS bị cả ba engine **học máy** gắn cờ ở
cả hai bản (cùng khuôn NSIS của Tauri), MSI cùng nội dung luôn sạch ⇒ báo nhầm do heuristic của gói NSIS, không do mã.

## 2. Xác minh hành vi trước khi gửi khiếu nại (AV1 mục "Xác minh", 26/11/2026)

Rà `crates/cwico-cli`, `crates/cwico-win`, `crates/cwico-core`:

- **Mạng**: CLI không có thư viện HTTP/socket (không `reqwest`/`ureq`/`hyper`/`TcpStream`/WinHTTP/WinINet) - không gửi gì
  ra ngoài. (App GUI có kiểm tra cập nhật ở `github.com` qua updater Tauri; không gửi dữ liệu máy - `SECURITY.md`.)
- **Không** hook bàn phím/chuột (`SetWindowsHookEx`, `GetAsyncKeyState`), **không** chụp màn hình (`BitBlt`/`GetDC`),
  **không** đọc mật khẩu/cookie trình duyệt (`CryptUnprotectData`, `Login Data`, `Cookies`), **không** clipboard.
- **Không** packer/protector: bản release dựng bằng `rustc` thường, `opt-level = "z"`, LTO, `strip = true`
  (`Cargo.toml` `[profile.release]`). Không UPX.
- API Windows dùng (`crates/cwico-win/Cargo.toml`): Registry, Services, Task Scheduler, ToolHelp (liệt kê tiến trình),
  System Restore, Storage/FileSystem, Shell - đúng chức năng "rà quét phần mềm/dịch vụ/tác vụ/mục khởi động và gỡ mục người dùng
  chọn, tạo điểm khôi phục trước khi đổi". Tổ hợp này + tệp đã `strip` là lý do có thể khiến mô hình học máy gắn nhãn `Spy`/grayware.

**Mô tả sản phẩm (tiếng Anh, để gửi hãng):** Tsu Cwico is an open-source (MIT) Windows debloater and software removal
tool: it reads installed programs, UWP/AppX packages, services, scheduled tasks and startup entries, classifies them, and
removes only what the user selects, creating a System Restore point and a registry backup first. It sends no user or machine data anywhere; its only network request is the app's
update check against GitHub Releases (source: https://github.com/tsudev-org/tsucwico).

## 3. Hồ sơ gửi hãng (👤 gửi - agent không có hộp thư/tài khoản)

| Hãng / nền tảng | Tệp | Ngày gửi | Mã hồ sơ / link | Trạng thái | Lý do (nguyên văn) |
| --- | --- | --- | --- | --- | --- |
| SecureAge (APEX) | NSIS 26.11.1901 | 19/11/2026 (VT1) | | đã nộp | |
| CrowdStrike | NSIS 26.11.1901 | | | chờ 👤 (thư soạn sẵn ở phiếu điều phối AV1 mục 3) | |
| Trapmine | NSIS 26.11.1901 | | | chờ 👤 | |
| Bkav / Google / Ikarus | CLI 26.11.1002 | | | không cần nếu Publish 26.11.1901 (CLI mới 0/74) | |
| Microsoft WDSI (Software developer) | - | | | không bị Defender gắn cờ - chưa cần | |

## 4. Quy trình bản sau

Release nháp → job VirusTotal trong `release.yml` → sạch mới Publish. Có cờ ⇒ so với bản trước làm đối chứng trước khi đổi
cách đóng gói (bài học tsuswico ISS-20261122-2153-055f). Đo lại 3-7 ngày sau khi gửi; hết cờ ⇒ ghi ngày gỡ vào mục 1.

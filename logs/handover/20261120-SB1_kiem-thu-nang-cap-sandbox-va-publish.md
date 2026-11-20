# PHIẾU BÀN GIAO - Kiểm thử nâng cấp bằng Windows Sandbox + đề xuất Publish v26.11.1901

- **Mã phiếu**: 20261120-SB1
- **Từ**: agent-tsucwico (phiên tsucwico-e7) - **Đến**: phiên tsucwico kế tiếp + 👤 chủ dự án
- **Thời điểm**: 20/11/2026
- **Trạng thái**: MỞ - chờ 👤 khởi động lại máy (đang có nhiều phiên agent chạy, chưa restart được)
- **Liên quan**: REL-TSU-1 (`logs/STATE.md`), `docs/RELEASING.md` mục 5-7, ADR-0001 mục 3, phiếu VT1, phiếu tsudev CW1

## 1. Quyết định: Windows Sandbox thay máy ảo có snapshot

`docs/RELEASING.md` mục 5 đòi thử nâng cấp từ 26.11.1002 (MSI và NSIS) trên "máy ảo có snapshot" trước khi Publish
bản đầu tiên mang tên Tsu Cwico. Máy phát triển là Windows 11 Pro, 16 GB RAM, Hyper-V đang chạy (WSL2) ⇒ dùng
**Windows Sandbox**: mỗi lần mở là Windows sạch, đóng là mất ⇒ tương đương snapshot, không cần dựng/khôi phục máy ảo.
Sandbox CHƯA bật (20/11: không có `C:\Windows\System32\WindowsSandbox.exe`); bật cần quyền Admin + khởi động lại.

Giới hạn (ghi trung thực): updater của bản đã cài chỉ đọc `releases/latest`, Release nháp không được tính ⇒ trước
Publish KHÔNG thử được đường "app tự thấy bản mới". Phần đó đã kiểm riêng trên nháp (phiên tsucwico-ea): `latest.json`
cùng dạng URL với 26.11.1002 (đã cập nhật thật được), `tools/verify_update_signature.py` đạt. Sandbox thử phần dễ hỏng
nhất: **cài đè bằng đúng lệnh updater gọi** (lấy từ mã `tauri-plugin-updater` 2.14.1, `installMode: passive`):
MSI `msiexec /i <msi> /passive /promptrestart`, NSIS `<setup.exe> /P /UPDATE`.

## 2. Bộ kiểm thử (đã có trong repo + đã dựng sẵn trên máy)

- `tools/sandbox/chuan-bi.sh [thẻ-cũ] [thẻ-mới]` (mặc định `tsudev-cwico-v26.11.10.2` → `v26.11.1901`): tải 4 bộ
  cài, đối chiếu SHA-256 với `SHA256SUMS.txt` của từng Release, chép script, sinh `kiem-thu-msi.wsb` +
  `kiem-thu-nsis.wsb` vào `%USERPROFILE%\tsucwico-sandbox\` (máy này: `C:\Users\devng\tsucwico-sandbox\`,
  WSL: `/mnt/c/Users/devng/tsucwico-sandbox/`). Đã chạy 20/11: 4/4 OK.
- `tools/sandbox/kiem-thu.ps1 -Duong msi|nsis` (chạy TRONG Sandbox qua `LogonCommand`): cài 26.11.1002 → mở thử →
  đặt tệp `dau-kiem-thu.txt` vào `%APPDATA%\com.tsudev.cwico`, `%LOCALAPPDATA%\com.tsudev.cwico`,
  `%LOCALAPPDATA%\tsudev-cwico\backups` → nâng cấp bằng lệnh updater → chấm ĐẠT/TRƯỢT: exit code 2 lần cài;
  **đúng 1** mục `cwico` trong Add/Remove Programs, tên `Tsu Cwico`, không còn `tsudev-cwico`, `DisplayVersion`
  26.11.1901; không còn exe trong `Program Files\tsudev-cwico`; exe mới ở `Program Files\Tsu Cwico`, FileVersion
  đúng; 3 tệp dấu còn nguyên; bản cũ và bản mới mở được ≥ 12 giây.
- Kết quả: `ketqua\<msi|nsis>\KET-QUA.txt` (dòng đầu `TONG KET [..]: DAT tat ca N muc` hoặc `TRUOT k / N`),
  `nhat-ky.txt` (transcript), `arp-truoc.txt`, `arp-sau.txt`, `cai-ban-cu.log` + `nang-cap.log` (MSI, `/l*v`).
- Script đã qua bộ phân tích cú pháp PowerShell (0 lỗi) nhưng **CHƯA chạy thật lần nào** ⇒ lần đầu có thể phải sửa
  (ví dụ: `LogonCommand` không đủ quyền, WebView2 thiếu trong Sandbox ⇒ mục "mở được" TRƯỢT dù cài đúng).
- Hướng dẫn cho 👤 (bản dễ đọc): `C:\Users\devng\tsucwico-sandbox\HUONG-DAN.md`.

## 3. Việc cần làm

### 3.1. 👤 Sau khi khởi động lại được máy

1. PowerShell **Run as administrator**:
   `Enable-WindowsOptionalFeature -Online -FeatureName Containers-DisposableClientVM -All` → Restart.
2. Đóng bớt ứng dụng (RAM thường 92-97%, Sandbox cần ~3 GB).
3. Bấm đúp `kiem-thu-msi.wsb`, không thao tác trong Sandbox, chờ Notepad hiện `KET-QUA.txt`, đóng Sandbox; lặp lại
   với `kiem-thu-nsis.wsb`. Nhắn phiên tsucwico: "Sandbox xong".

### 3.2. Agent - phiên kế tiếp (khởi động)

1. `ls /mnt/c/Windows/System32/WindowsSandbox.exe` - chưa có ⇒ Sandbox chưa bật, nhắc 👤 mục 3.1, bỏ qua tới mục 3.4.
2. Có `ketqua/msi/KET-QUA.txt` và `ketqua/nsis/KET-QUA.txt` ⇒ đọc cả hai + `arp-sau.txt` + `nhat-ky.txt`.
   Chưa có ⇒ chờ 👤. Bộ kit mất/cũ ⇒ chạy lại `tools/sandbox/chuan-bi.sh`.
3. Có mục TRƯỢT:
   - Lỗi của script/môi trường (không phải của bộ cài) ⇒ sửa `tools/sandbox/kiem-thu.ps1`, chạy lại `chuan-bi.sh`,
     nhờ 👤 chạy lại.
   - Lỗi thật của bộ cài (2 mục ARP, mất dữ liệu, cài sai thư mục) ⇒ GIỮ nháp, mở việc sửa, dựng bản mới (thẻ mới,
     VirusTotal quét lại từ đầu), xoá nháp `v26.11.1901` khi bản thay thế sẵn sàng.
4. Ghi kết quả vào mục 5 của phiếu này + REL-TSU-1 trong `logs/STATE.md`.

### 3.3. Agent - sau khi cả hai đường ĐẠT: đề xuất Publish (👤 quyết)

**Khuyến nghị: Publish `v26.11.1901` theo NGOẠI LỆ, không chờ VirusTotal về 0.** Gửi 👤 đúng đề xuất này, chờ câu
đồng ý rõ ràng ("Đồng ý Publish v26.11.1901 theo ngoại lệ") - KHÔNG tự Publish.

Lý do:
- Cả 3 cờ trên NSIS (CrowdStrike `win/grayware_confidence_60% (D)`, Trapmine `malicious.moderate.ml.score`, APEX
  `Malicious`) là phán đoán học máy trên stub NSIS chưa ký; MSI + CLI cùng bản dựng 0 cờ; Defender không gắn cờ.
  Đọc lại 19/11 và 20/11: không đổi.
- Cờ học máy thường không tự hết nếu hãng không phản hồi; chờ thêm chỉ giữ người dùng ở bản tên cũ.
- Bằng chứng bản dựng sạch đầy đủ: attestation 4/4, Sigstore, SHA256SUMS ký, chữ ký updater đúng khoá.

Rủi ro còn lại (nói rõ với 👤): máy cài bằng NSIS **và** chạy CrowdStrike/Trapmine/APEX có thể bị chặn bản cập nhật
bắt buộc - nhóm rất nhỏ (công cụ gỡ phần mềm cá nhân). Publish là đẩy tới MỌI máy, không có phát hành từng phần.

Chuẩn DISTRIBUTION_TRUST mục 3 yêu cầu giữ nháp khi còn cờ ⇒ Publish là ngoại lệ do 👤 quyết, BẮT BUỘC ghi vào
phiếu VT1 mục 4 (ngày, người quyết, lý do, số cờ lúc Publish) như tiền lệ lotus-share.

Trước khi gửi đề xuất: đọc lại VirusTotal (`K=$(tr -d '[:space:]' < ~/.config/tsudev/virustotal.env)`,
`GET /api/v3/files/28d7b74c990bdf54d94e4921217f6a8181eb609a65b3dc021f7a43d47add2537`). Về 0 ⇒ Publish bình
thường, không cần ngoại lệ.

### 3.4. Agent - sau khi 👤 đồng ý Publish (theo `docs/RELEASING.md` mục 6-7 + REL-TSU-1)

1. `gh release edit v26.11.1901 -R tsudev-org/tsucwico --draft=false --latest`.
2. Kiểm: `curl -sL https://github.com/tsudev-org/tsucwico/releases/latest/download/latest.json` ⇒ `version`
   `26.11.1901`; endpoint cũ `https://github.com/tsudev-org/tsudev-cwico/releases/latest/download/latest.json` (bản đã
   cài gọi) cũng ⇒ 26.11.1901.
3. Deploy website (trang tải về đọc `releases/latest` lúc build), đo `/tai-ve/` hiện 26.11.1901.
4. `tools/apply_winget_manifest.py` (manifest trong artifact `installers` của run phát hành) + nộp winget (đợi
   winget-pkgs#476024 xử lý xong). Scoop tự lên nhờ scoop-bucket#24 - kiểm sau vài giờ.
5. Ghi ngoại lệ vào VT1 mục 4; đóng REL-TSU-1 trong STATE; nếu 👤 có máy thật đang chạy 26.11.1002, nhờ mở app xác
   nhận màn cập nhật bắt buộc → cài → khởi động lại thành 26.11.1901 (đường updater thật, mục 1).

### 3.5. Việc liên quan đang chờ (không chặn phiếu này)

- **Neon (phiếu tsudev CW1 phần b)**: ✅ 👤 chèn client `tsucwico` 20/11 9:36 (giờ VN), "Statement executed
  successfully" (lần 9:10 lỗi cú pháp, không ghi gì); đã báo tsudev-f6. Agent không có quyền đọc Neon ⇒ xác minh gián tiếp sau khi tsudev
  deploy nhánh `feat/oidc-client-native-loopback`: `/oidc/auth?client_id=tsucwico&redirect_uri=http://127.0.0.1:<cổng>/callback&...`
  ⇒ 303 (không phải 400).
- Email báo nhầm CrowdStrike/Trapmine: phiếu VT1 mục 3.1 (👤).

## 4. Tiêu chí hoàn thành

`KET-QUA.txt` của cả hai đường `DAT tat ca`; 👤 đã quyết Publish (hoặc giữ nháp) và quyết định được ghi; nếu Publish:
mục 3.4 bước 1-5 xong.

## 5. Kết quả xử lý (phiên nhận điền)

- (chưa)

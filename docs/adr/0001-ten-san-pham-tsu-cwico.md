# ADR-0001: Tên sản phẩm Tsu Cwico, định danh `tsucwico`

- **Trạng thái**: Đã chấp nhận (16/11/2026, chủ dự án)
- **Tiền lệ**: `tsudev-org/tsufiles` ADR-0003 (TsuFiles), `tsudev-org/tsucontact` ADR-0001 (Tsu Contact)

## 1. Bối cảnh

Chủ dự án (16/11/2026) chốt tên sản phẩm mới **Tsu Cwico** cho thống nhất với hệ sinh thái tsudev, đồng thời đổi tên
repo và thư mục `tsudev-cwico` thành `tsucwico`, và đưa mọi thông tin ủng hộ về trang trung tâm của hệ sinh thái.

Khác Tsu Contact, app này **có trình cập nhật bắt buộc** (Tauri updater) trong các bản đã cài - 26.8.19 và 26.11.1002.
Mọi quyết định dưới đây xoay quanh một ràng buộc: hai bản đó phải tự nâng lên bản mang tên mới, tại chỗ, không để lại
bản cài thứ hai.

## 2. Quyết định

| Hạng mục | Trước | Sau |
| --- | --- | --- |
| Tên hiển thị (app, cửa sổ, bộ cài, Add/Remove Programs, website, tài liệu) | `tsudev-cwico`, `cwico` | **Tsu Cwico** |
| Định danh máy (gói, Worker, package npm) | `tsudev-cwico` | `tsucwico` |
| Repo GitHub | `tsudev-org/tsudev-cwico` | `tsudev-org/tsucwico` (đổi 16/11/2026) |
| Nhãn phát hành | `tsudev-cwico-v26.11.10.2` | `tsucwico-v26.11.16` (công cụ vẫn đọc nhãn cũ: `LEGACY_PRODUCT`) |
| Tệp phát hành | `tsudev-cwico_<v>_x64-setup.exe`, `…_x64_en-US.msi`, `…_x64-cli.exe` | `tsucwico_<v>_x64-setup.exe`, `tsucwico_<v>_x64.msi`, `tsucwico_<v>_x64-cli.exe` + `SHA256SUMS.txt(.sig)` |
| Tauri `productName` | `tsudev-cwico` | `Tsu Cwico` |
| MSI `UpgradeCode` | sinh từ `productName` | **ghim** `96371ec9-5fcd-5edb-a65d-8641d9975827` (đọc từ MSI 26.11.1002) |
| Bộ cài NSIS | khóa theo `productName` | hook `installer/hooks.nsh` gỡ im lặng bản `tsudev-cwico` (cờ `/UPDATE`, giữ dữ liệu) trước khi cài |
| Endpoint cập nhật | `github.com/tsudev-org/tsudev-cwico/...latest.json` | `github.com/tsudev-org/tsucwico/...latest.json` |
| Worker Cloudflare | `tsudev-cwico-website` | `tsucwico-website` (giữ tên miền `cwico.tsudev.com`) |
| Gói Scoop (chưa phát hành) | dự kiến `tsudev-cwico` | `tsucwico` (lệch `PACKAGING.md` mục 2 có chủ đích, như `tsucontact`) |
| Ủng hộ | chưa có | nút "Ủng hộ tsudev" trong Giới thiệu + chân trang website + `.github/FUNDING.yml`, đều mở `https://tsudev.com/ung-ho` (quyết định `20261112-02`) |

**Giữ có chủ đích** (bản đã cài hoặc dữ liệu người dùng phụ thuộc):

- `identifier` Tauri `com.tsudev.cwico` - đổi thì updater và thư mục dữ liệu WebView coi là app khác.
- Khóa công khai updater - bản đã cài chỉ nhận gói ký bằng khóa này.
- Mã winget `tsudev.cwico` - mã đã công bố không đổi (`PACKAGING.md` mục 2); chỉ `PackageName` đổi thành `Tsu Cwico`.
- Lệnh `cwico` / tệp `cwico.exe`, tên crate `cwico-*` - không mang tên cũ `tsudev-cwico`, đổi chỉ làm gãy script.
- Thư mục sao lưu `%LOCALAPPDATA%\tsudev-cwico\backups` - chứa `.reg` và nhật ký giao dịch để khôi phục; di chuyển
  dữ liệu khôi phục không đem lại gì mà thêm rủi ro.
- Thư mục khóa ký `~/.tsudev-cwico/` trên máy chủ dự án (`docs/SIGNING.md`).
- Tên miền `cwico.tsudev.com`, logo `logo-cwico`, giấy phép MIT và dòng bản quyền.
- Lịch sử: `CHANGELOG.md` bản cũ, `logs/`, `docs/sessions/`, manifest winget đã nộp (`26.8.1901`, `26.11.1002`).

## 3. Hệ quả

- Bản 26.8.19 / 26.11.1002 đã cài gọi endpoint cũ, GitHub chuyển hướng 301 sang repo mới (kể cả URL API tải tài sản
  trong `latest.json`), tải gói `tsucwico_*` ký bằng cùng khóa:
  - cài bằng MSI: MSI mới cùng `UpgradeCode` ⇒ nâng cấp lớn bình thường, gỡ bản cũ, cài vào `Program Files\Tsu Cwico`;
  - cài bằng NSIS: hook gỡ bản `tsudev-cwico` (giữ dữ liệu), rồi cài `Tsu Cwico`.
- **Cấm vĩnh viễn** tạo lại repo/fork/đổi tên thành `tsudev-cwico` trong `tsudev-org` (và `tsudev-tsudev`): chuyển
  hướng GitHub mất ngay, mọi bản đã cài sẽ im lặng không bao giờ cập nhật nữa (hoặc cập nhật từ repo sai). Đã ghi vào
  `~/.claude/CLAUDE.md` của máy phát triển.
- Trường hợp chưa xử lý: máy cài bản cũ bằng MSI rồi tự chạy bộ cài NSIS mới bằng tay (hoặc ngược lại) sẽ có hai mục
  trong Add/Remove Programs - giống hệt hành vi trước khi đổi tên khi trộn hai loại bộ cài.
- Kiểm thử thật phần nâng cấp trên Windows (máy ảo có snapshot) là bước bắt buộc trước khi công bố bản đầu tiên mang
  tên mới - xem `docs/RELEASING.md`.

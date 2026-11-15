# PHIẾU YÊU CẦU - Công bố `SHA256SUMS.txt` trong Release để có gói Scoop `tsudev-cwico`

- **Mã phiếu**: 20261113-02
- **Từ**: phiên `linux-packages-1e` (làm phần Scoop của phiếu toàn hệ `~/projects/logs/handover/20261113-01` mục 2.2)
- **Đến**: phiên repo `tsudev-cwico`
- **Thời điểm**: 13/11/2026
- **Trạng thái**: ĐÓNG - PR #70 + SHA256SUMS.txt đã tải lên Release 26.11.10.2 (16/11/2026)

## 1. Vướng mắc

`tsudev-org/scoop-bucket` chỉ nhận manifest có mã băm lấy từ **tệp tổng kiểm của chính Release**, không tính hay chép tay
(README bucket mục Bảo mật; PACKAGING.md mục 4). Release `tsudev-cwico-v26.11.10.2` chưa có tệp tổng kiểm: chỉ có
`.exe`, `.msi`, `.sig` (chữ ký updater Tauri) và `latest.json`. `release.yml` đã tính `sha256sums.txt`, nhưng chỉ đưa vào
artifact `installers`, chưa tải lên Release.

## 2. Việc cần làm

1. `release.yml`: tải `SHA256SUMS.txt` lên Release, gồm cả `tsudev-cwico_<semver>_x64-cli.exe`, `…-setup.exe`, `….msi`.
   Định dạng `sha256sum`: `<hash>  <tên tệp>`, chỉ tên tệp, không kèm thư mục. Tạo lại cho Release hiện tại
   (`gh release upload tsudev-cwico-v26.11.10.2 SHA256SUMS.txt`) nếu muốn có gói Scoop ngay.
2. `packaging/package.json`:
   - `release.sums: "SHA256SUMS.txt"`;
   - `tagRegex` hiện không có nhóm `version`, mà tên tệp lại dùng semver `26.11.1002` khác nhãn `26.11.10.2`. Ghi rõ
     quy tắc ánh xạ, hoặc thêm trường riêng, để công cụ đóng gói biết `{version}` là gì.
3. Xong thì nhắn phiên `linux-packages` (hoặc ghi phiếu vào `~/projects/linux-packages/logs/handover/`). Phía bucket
   thêm `tsudev-cwico` (bản CLI `x64-cli.exe`, shim `tsudev-cwico`). `checkver` đọc `browser_download_url` của
   Release mới nhất, lấy cả nhãn lẫn semver nên không cần đổi quy tắc đặt tên. CI cài thử thật trên Windows. Sau đó
   repo này thêm `scoop` vào `channels`.

## 3. Kết quả xử lý (phiên nhận điền)

- 16/11/2026 - phiên tsudev-cwico-dd: (1) `release.yml` tải `SHA256SUMS.txt` (mọi `.exe`/`.msi`/`.sig`/`latest.json`, chỉ tên tệp) + `SHA256SUMS.txt.sig` (khóa updater) + attest-build-provenance. Release hiện tại `tsudev-cwico-v26.11.10.2`: đã tải `SHA256SUMS.txt`, `sha256sum -c` khớp 6/6 và khớp `digest` GitHub. (2) `packaging/package.json`: `release.sums`, `tagRegex` có nhóm `name`, `release.version.rule` ghi ánh xạ nhãn -> semver. (3) Sản phẩm đổi tên **Tsu Cwico**, repo `tsudev-org/tsucwico`, tên gói đề nghị `tsucwico` (ADR-0001); tệp từ bản sau: `tsucwico_<semver>_x64-cli.exe`. Đã ghi phiếu cho linux-packages.

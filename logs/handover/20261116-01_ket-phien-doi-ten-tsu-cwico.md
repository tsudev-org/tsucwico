# PHIẾU BÀN GIAO - Kết phiên đổi tên Tsu Cwico (16/11/2026)

- **Mã phiếu**: 20261116-01
- **Từ**: phiên `tsudev-cwico-dd` - **Đến**: phiên kế tiếp của repo `tsudev-org/tsucwico`
- **Trạng thái**: MỞ (việc còn lại ở hàng đợi `logs/STATE.md`)

## Đã làm (PR #70, đã merge + deploy)

- Sản phẩm **Tsu Cwico**, định danh `tsucwico`; repo đổi tên `tsudev-org/tsucwico` (bảo mật giữ nguyên, URL cũ 301,
  endpoint updater cũ trả 200). Quyết định + danh sách giữ có chủ đích: `docs/adr/0001-ten-san-pham-tsu-cwico.md`.
- `~/.claude/CLAUDE.md`: thêm cấm tạo lại `tsudev-cwico`.
- Website `cwico.tsudev.com` chạy trên Worker mới **`tsucwico-website`** (Version `cbc34afb`). Worker cũ
  `tsudev-cwico-website` không còn route - chủ dự án xóa trên dashboard khi muốn (`npx wrangler delete --name tsudev-cwico-website`).
- Ủng hộ -> `https://tsudev.com/ung-ho`; khối "Cài bằng lệnh"; `SHA256SUMS.txt` cho Release 26.11.10.2; Scorecard.

## Việc tiếp theo (theo thứ tự)

1. **REL-TSU-1** (STATE): phát hành bản đầu tiên tên mới. Chưa chạy thật: `releaseAssetNamePattern`, bước
   SHA256SUMS/attest. Nếu `[arch]` ra khác `x64`, sửa `packaging/package.json`, `tools/winget-manifest.ps1`,
   `tools/apply_winget_manifest.py` theo tên thật. BẮT BUỘC nâng cấp thử 26.11.1002 -> bản mới trên máy ảo Windows
   (MSI và NSIS) trước khi chủ dự án công bố bản nháp.
2. Thư mục cục bộ: đổi `~/projects/tsudev-cwico` -> `~/projects/tsucwico` khi không còn phiên nào mở trong thư mục
   (`mv ~/projects/tsudev-cwico ~/projects/tsucwico`); `git remote` đã trỏ repo mới.
3. SCOOP-1, BP-1 (STATE).

## Phiếu đã gửi

- `~/projects/linux-packages/logs/handover/20261116-01_yeu-cau-goi-scoop-tsucwico.md` (gói Scoop `tsucwico`).
- `~/projects/tsudev/logs/handover/20261116-90_yeu-cau-doi-ten-tsu-cwico.md` (danh bạ tsudev.com) + đã nhắn phiên `tsudev-1e`.

## Kết quả phiếu (cập nhật cuối phiên)

- `tsudev` 20261116-90: XONG phần danh bạ - tsudev#275 đã gộp + deploy (v2026.11.16.3), `/apps` hiện "Tsu Cwico",
  link Release trỏ `tsudev-org/tsucwico`; giữ mã `cwico`, winget `tsudev.cwico`, tên miền, logo. **Còn chờ chủ dự
  án duyệt** tsudev#276 (`TRADEMARKS` + khóa `metadata.json` `tsudev-cwico` -> `tsucwico`, vùng ⛔). Gói Scoop
  `tsucwico` tự vào danh mục ở lần `npm run pkg:catalog` kế tiếp sau khi bucket có gói.
- `linux-packages` 20261116-01 (gói Scoop): chưa có phản hồi - theo dõi ở SCOOP-1.

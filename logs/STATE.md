# STATE.md - Trạng thái project

> **Phiên kế tiếp:** việc đầu tiên là REL-TSU-1 theo phiếu **`logs/handover/20261120-SB1_*`** (thử nâng cấp bằng
> Windows Sandbox thay máy ảo - chờ 👤 bật Sandbox + restart; đọc `ketqua/` rồi gửi đề xuất Publish ngoại lệ, 👤 quyết).
> VirusTotal NSIS vẫn 3/71 (phiếu VT1 mục 4). **PRO1 P2 HẾT bị chặn** (đo 20/11: `/oidc/auth?client_id=tsucwico` loopback 127.0.0.1 / [::1] cổng bất kỳ ⇒ 303,
> `localhost` ⇒ 400; `/pro` ⇒ 200; tsudev#339) ⇒ việc thứ hai: bắt đầu P2 theo ADR-0002 mục 3. **PRO1 P1 đã gộp**
> (#94). P2 chờ tsudev: phiếu **tsudev `20261120-CW1`** (client native loopback `tsucwico` - cần sửa mã `oidc.ts` + 👤
> INSERT Neon; (a) đã làm ở nhánh tsudev `feat/oidc-client-native-loopback`, chờ 👤 duyệt push/PR - oidc-provider 9.14
> tự coi client có mọi redirect `http://127.0.0.1/...`/`http://[::1]/...` không cổng là native, cổng bất kỳ; SQL (b) dùng
> `ARRAY['http://127.0.0.1/callback','http://[::1]/callback']`) và B3 `/pro` (đang làm ở nhánh tsudev `feat/pro-b3-thanh-toan-vietqr`); tsudev#336 ĐÃ gộp 19/11. Đo lại:
> `/oidc/auth?client_id=tsucwico` (20/11: 400) và `/pro` (20/11: 404). P3 chờ PRO-D3. Còn **P2 MSIX** - chờ 👤 Partner Center.

> Agent đọc file này ở đầu mỗi phiên và cập nhật ở cuối mỗi phiên.
> Quy trình: `.standards/docs/AGENT_PROTOCOL.md`.

## Hàng đợi task (làm từ trên xuống)

- [ ] **REL-TSU-1** Bản đầu tiên mang tên Tsu Cwico + khuôn thẻ v3: **Release nháp [`v26.11.1901`](https://github.com/tsudev-org/tsucwico/releases/tag/v26.11.1901)** (19/11/2026, phiên tsucwico-ea, run 44531891240). Đã kiểm trên nháp: 10 tệp đúng khuôn (`tsucwico_26.11.1901_windows-x64-setup.exe`, `_windows-x64.msi`, `_windows-x64-cli.exe`, `_sbom.spdx.json`, `.sig`, `latest.json`, `SHA256SUMS.txt` + `.sig` + `.sigstore.json`); `sha256sum -c` 7/7; `tools/verify_update_signature.py` - bản đã cài chấp nhận 3 gói; `gh attestation verify` 4/4 (ký bởi `release.yml@refs/tags/v26.11.1901`); Sigstore `OK`; CLI có CompanyName TsuDEV / FileVersion 26.11.1901; `latest.json` cùng dạng URL với 26.11.10.2 (đã cập nhật thật được). **VirusTotal**: MSI + CLI 0/74; bộ cài NSIS 3/74 engine học máy (CrowdStrike, APEX, Trapmine) ⇒ báo nhầm theo phiếu `logs/handover/20261119-VT1_*` (SecureAge đã nộp; 👤 gửi 2 email soạn sẵn). **Còn 👤:** nâng cấp thật trên máy ảo Windows có snapshot từ 26.11.1002 (MSI và NSIS) - mỗi đường chỉ một mục "Tsu Cwico" trong Add/Remove Programs (ADR-0001 mục 3, `docs/RELEASING.md` mục 5) ⇒ **Publish** (updater bắt buộc: Publish là đẩy tới mọi máy). **20/11: thử nâng cấp chuyển sang Windows Sandbox, bộ kiểm `tools/sandbox/` - phiếu `20261120-SB1`.** Sau Publish: deploy website (trang tải về đọc `releases/latest` lúc build), `tools/apply_winget_manifest.py` (manifest trong artifact `installers`) + nộp winget (đợi #476024), Scoop tự lên nhờ scoop-bucket#24.
- [ ] **PRO1** Lưu cục bộ mặc định; Pro = đồng bộ cài đặt (TsuDEV Sync, crate `tsusync`); nhắc nhiều thiết bị mỗi 30
  ngày. Phiếu `logs/handover/20261119-PRO1_luu-cuc-bo-pro-dong-bo.md`, kế hoạch phase ở **ADR-0002 mục 3**.
  **P1 ✅** 20/11 (#94; kiểm kê 0 byte rời máy, chuẩn v3.9.1, `cwico_core::pro`, hộp thoại `ProNotice`). **P2 sẵn sàng** (20/11: client `tsucwico` native
  loopback trên production - phiếu tsudev CW1 đóng, tsudev#339; `/pro` 200; tsudev#336 gộp 19/11). **P3** chờ
  PRO-D3. Hợp đồng: PRODUCT_POLICY 7.4 bản v3.9.1 (KHÔNG theo mục 9.2 phiếu gốc - đã lỗi thời).
- [ ] **P2-MSIX** Đích MSIX (`Version` = `YY.M.DDNN.0`) + nộp Microsoft Store + nút Store trên website. Chờ 👤 tạo tài
  khoản Partner Center cá nhân (phiếu P2 mục 4.1) - không đoán Identity Name/Publisher.
- [x] **T8** ✅ 19/11/2026 (phiên tsucwico-c4): #92 gộp `20de2ab`, deploy Worker version `2fb094b0`; đo production: 1 video
  (WebM phát được, 12 cue phụ đề), 9 `<picture>`, `media-src 'self'`, tệp video/AVIF/WebP 200 đúng content-type. Bộ ảnh
  cửa hàng (`assets/store/`) làm cùng P2-MSIX.
- [x] **SCOOP-1** ✅ 16/11/2026 (phiên tsudev-27): gói `tsucwico` có ở scoop-bucket (`dba0759`, cài thử đạt), `channels` có `scoop`; website deploy lại (Worker `2aa95bdf`) - `/tai-ve/` hiện `scoop install tsudev/tsucwico`. Mô tả gói chép nhầm của Swico ⇒ sửa ở scoop-bucket#14.
- [x] **BP-1** ✅ 17/11/2026 (phiên tsudev-27): hồ sơ [16838](https://www.bestpractices.dev/projects/16838) passing 100% (`docs/best-practices.json`, giấy phép MIT; `dynamic_analysis` Unmet trung thực). Kèm #75 (ghim SHA 20 action, quyền token, SECURITY.md khớp mã: có updater + chữ ký minisign), CodeQL default setup, #76 (cảnh báo CodeQL high ở check-dist). Còn: P1.5 good first issue (phiếu 20261113-03).
- [x] **LOCAL-DIR** ✅ 16/11/2026 (phiên tsudev-27): thư mục cục bộ đổi `~/projects/tsudev-cwico` -> `~/projects/tsucwico`.
- [x] **TSUDEV-276** ✅ tsudev#276 đã MERGE (chủ dự án duyệt 16/11/2026).
- [ ] **WINGET-2** Manifest 26.11.1002 đã nộp: [microsoft/winget-pkgs#476024](https://github.com/microsoft/winget-pkgs/pull/476024) (10/11/2026, fork `tsudev-tsudev/winget-pkgs`). Theo dõi kiểm định tự động và CLA; gộp xong thì `winget install tsudev.cwico` ra bản mới. 10/11 phiên 06: 01-07, 09, 10 + CLA xanh; bước 08 (cài thử) NEUTRAL, nhãn **`Validation-Executable-Error`** (bộ kiểm không chạy được exe sau cài). Đã đối chiếu: SHA-256, ProductCode `{09A40823-...}`, ProductVersion khớp MSI; `cwico.exe` vào INSTALLDIR; app không đổi gì ở phần khởi động so với 26.8.19 (bản đó qua bước 08). Nhãn chỉ quản trị viên winget gỡ được - chủ dự án bình luận giải thích trên PR (GUI Tauri cần WebView2, không cần tham số), rồi chờ.
- [ ] **QA-WIN-1** Chạy thử phần GHI trên Windows thật, có quyền Admin: `cwico plan --safe-only` (không đổi gì), rồi gỡ một mục An toàn trên máy ảo có snapshot - kiểm điểm khôi phục, `.reg`, nhật ký giao dịch. Chưa phiên bản nào làm việc này. Phiên 06 (10/11/2026) đã làm phần CHỈ ĐỌC trên Windows 11 25H2 của chủ dự án, không Admin, bằng CLI tải từ bản phát hành 26.11.1002 (SHA-256 khớp): `info`, `plan --safe-only` -> 30 mục / 63 bước trong 2,8 giây, không bị từ chối mục nào. Phát hiện `plan` giấu bước điểm khôi phục + sao lưu registry - sửa ở PR #55. Còn lại: phần GHI trên máy ảo có Admin.
- [x] **QU-STD-1** XONG 10/11/2026 (phiếu `20261110-05`): chế độ Warm BỎ theo quyết định chủ dự án (chỉ Sáng/Tối/Hệ thống); nút chính, thanh tiến trình, ô chọn, hàng đang chọn dùng token `primary`. Màu chữ "tsu/dev" của wordmark vẫn theo QU-STD-BRAND/TS-8.
- [ ] **QU-STD-BRAND (phần còn lại)** Màu chữ "dev" ở `ui/src/components/Brand.tsx` (cam, 4,18:1 - hai lỗi axe duy nhất còn lại) lệch website (xanh) - chờ TS-8 ở `tsudev-standards`, chưa tự đổi. Ảnh đại diện tổ chức GitHub là việc cấp tổ chức `tsudev-org`.
- [ ] **DEP-1 (phần còn lại)** Mọi PR dependabot đã gộp/đóng (#22 thay bằng #59, đóng 10/11/2026). Chỉ còn cảnh báo bảo mật #1 `glib < 0.20` (GHSA-wrw7-89jp-8q8g): phụ thuộc gtk3 của tauri trên Linux, chưa có bản vá, không vào bản Windows - chủ dự án đóng dạng "tolerable risk" hoặc chờ tauri lên gtk4.
- [x] **CF-ZONE-1** ✅ JS Detections tắt ở vùng tsudev.com (16/11/2026, tsudev#280); đo 16/11 23:10: HTML `cwico.tsudev.com` không còn `challenge-platform` / `__CF$cv`.

## Đang thực hiện

| Task | Agent | Bắt đầu |
| --- | --- | --- |

## Đã hoàn thành (mới nhất trên cùng)

- 20/11/2026 - Phiên tsucwico-e7: #94 (PRO1 P1) đỏ do README ghi 143 test (thực tế 154) ⇒ sửa, CI xanh, gộp squash.
  Release vẫn nháp ⇒ bỏ qua deploy/winget; VirusTotal NSIS đọc lại 00:39 UTC vẫn 3/71 (lượt 19/11 14:04), gửi
  `analyse` lại. PRO1 P2 chưa đủ điều kiện (client `tsucwico` ⇒ 400, `/pro` ⇒ 404) ⇒ phiếu tsudev `20261120-CW1`
  (phát hiện: `clientMetadata()` chỉ tạo client web khớp tuyệt đối, loopback cổng ngẫu nhiên cần sửa mã) + nhắn tsudev-f6.

- 20/11/2026 - Phiên tsucwico-e6: **PRO1 P1** - chuẩn v3.9.1; ADR-0002 (kiểm kê: bản miễn phí 0 byte dữ liệu người
  dùng rời máy); `cwico_core::pro` (claims JWS 7.4, `notice_due` 11 test gồm 5 ca bắt buộc, mã cài đặt 128 bit);
  `ProNotice` vi/en nguyên văn + lệnh Tauri `pro_notice`/`open_pro_page`; khối "Dữ liệu chỉ ở máy này" ở Giới thiệu;
  Ủng hộ có `?app=tsucwico`. Release vẫn nháp, winget#476024 vẫn chờ quản trị viên.

- 19/11/2026 - Phiên tsucwico-c4: **T8** website có hình + video (#92): hero cặp ảnh Sáng/Tối, 5 tính năng so le, video
  86 giây MP4 + WebM + phụ đề vi tự lưu trữ, AVIF/WebP lúc build, khối Nền tảng, hiện dần khi cuộn; 5 ảnh preview cũ (tên
  tsudev-cwico) chụp lại bằng `website/scripts/chup-anh.mjs`, video bằng `quay-demo.mjs` (`?preview=demo` trong fixtures);
  deploy + đo production. VirusTotal NSIS đọc lại 2 lần: vẫn 3/71 (phiếu VT1 mục 4). Release vẫn nháp ⇒ bỏ qua bước
  deploy-sau-Publish / winget / Scoop. Nhận phiếu PRO1.

- 19/11/2026 - Phiên tsucwico-ea: **P1 phiếu P2 hàng 5** - khuôn VERSIONING v3 (#87: thẻ `v26.11.DDNN`, tiêu đề `Tsu Cwico <v>`, tệp `tsucwico_<v>_windows-x64-*`, test `updaterOrder` bằng crate `semver` của updater), SBOM + Sigstore + attestation + VirusTotal trong `release.yml`, CLI có thông tin phiên bản; scoop-bucket#24 (regex 2 khuôn, giấy phép MIT); **T3** lựa chọn ngôn ngữ "Theo hệ thống" (#88, thử thật trên Chromium); dependabot #79-#81; chuẩn v3.8.3 (#90); bản phát hành #89 + thẻ `v26.11.1901` (nháp). Phiếu V1 mục 6 ghi kết quả bước 2.
- 16/11/2026 - Phiên tsudev-cwico-dd: **đổi tên sản phẩm Tsu Cwico, repo `tsudev-org/tsucwico`** (ADR-0001; bảo mật repo giữ nguyên, URL cũ 301, endpoint updater cũ 200); ghim MSI UpgradeCode + hook NSIS cho bản đã cài; SHA256SUMS + chữ ký + attestation trong `release.yml`, `SHA256SUMS.txt` tải lên Release 26.11.10.2; Ủng hộ -> `tsudev.com/ung-ho` (app, website, FUNDING.yml); khối "Cài bằng lệnh" sinh lúc build; sửa id trùng `/nhat-ky/`; Scorecard. PR #70. Đóng phiếu 20261111-01, 20261112-01, 20261113-02; 20261113-07, 20261114-02 xong sau deploy.
- 10/11/2026 - Phiếu `20261110-05` (giao diện đen + tím kiểu Supabase): website (bộ chọn Sáng/Tối/Hệ thống ở chân trang thay menu 4 lựa chọn ở header của #58) + app Tauri (3 lựa chọn, mặc định Tối, bảng §2). Chi tiết: mục Kết quả của phiếu.
- 10/11/2026 - Bộ quy ước nâng lên v3.3.0 (#63).
- 10/11/2026 - Phiên 06 (tiếp): nút đổi giao diện Sáng/Ấm/Tối/Theo hệ thống cho website (#58, phiếu 20261110-03 từ phiên tsudev-c8); nâng nhóm tauri thay #22 (#59); chủ dự án gộp các PR dependabot còn lại.
- 10/11/2026 - Phiên 06: `cwico plan` hiện bước điểm khôi phục + sao lưu registry (#55); phần chỉ đọc của QA-WIN-1 trên máy thật. Phiếu `logs/handover/20261110-02_ket-phien-06.md`.
- 10/11/2026 - **Phát hành `tsudev-cwico-v26.11.10.2`** (chủ dự án công bố). Kiểm đầu cuối trên máy thật: bản 26.8.19 đã cài hiện màn cập nhật bắt buộc, bấm Cập nhật -> tải, kiểm chữ ký, cài, khởi động lại thành 26.11.1002. Website triển khai lại; manifest winget 26.11.1002 (#51).
- 10/11/2026 - Phiên 05 (tiếp): CLI vào tài sản phát hành (REL-2, #45); sửa quy trình phát hành gọi sai `-Version` (#45) và đọc CHANGELOG bằng cp1252 (#48); phân trang danh mục website (WEB-1, #46); CSDL an toàn 1.0.2 thêm tên cho 17 quy tắc (#46); ảnh chụp app thật vào README (#47); bản nháp `tsudev-cwico-v26.11.10.2` (#49).
- 10/11/2026 - Phiên 05: website `https://cwico.tsudev.com` (PR #37, Workers Static Assets); chạy thật trên Windows 11 lần đầu, sửa UI trắng cửa sổ + hợp đồng IPC (PR #38); phân trang mọi danh sách (QU-STD-TABLE, PR #40); ảnh bản cài + cổng CI tài sản (PR #41); token chuẩn + border-control (QU-STD-3, phần lớn QU-STD-1, PR #42). Chi tiết: `logs/handover/20261110-01_ket-phien-05.md`.
- 09/11/2026 - Phiếu 20261109-01: README mở đầu theo `ECOSYSTEM_IDENTITY.md` mục 2.1, ghi hạng C; mọi URL repo trỏ `tsudev-org/tsudev-cwico` (giữ nguyên manifest winget 26.8.1901 đã phát hành và nhật ký phiên cũ).
- 24/08/2026 - Đưa bộ quy ước tsudev v2.1.0 vào repo, bật cổng kiểm CI.

## Quyết định quan trọng

> Quyết định kiến trúc lớn thì viết ADR riêng theo `docs/templates/ADR.md` và chỉ
> ghi một dòng tham chiếu ở đây.

- 16/11/2026 - Tên sản phẩm **Tsu Cwico**, định danh `tsucwico`; CẤM tạo lại repo `tsudev-cwico`: `docs/adr/0001-ten-san-pham-tsu-cwico.md`.

- 10/11/2026 - Website `cwico.tsudev.com` chạy trên **Cloudflare Workers Static Assets** (Worker `tsudev-cwico-website`, không có mã phía máy chủ), không phải Pages: wrangler 4.158 chuyển lệnh tạo dự án Pages mới sang Workers và tự cài adapter SSR. Lý do trong `website/wrangler.jsonc`.
- 10/11/2026 - Hợp đồng JSON engine -> UI: trường tập hợp (`executables`, `extra`, `registryBackups`, `warnings`) luôn được tuần tự hóa; `crates/cwico-core/tests/ipc_contract.rs` canh.
- 09/11/2026 - cwico là **hạng C - ngoại tuyến** (`AUTH_AND_ACCOUNT.md` mục 1), không phải hạng B như hàng đợi cũ ghi: không có tính năng nào cần máy chủ hay tài khoản; kiểm tra cập nhật lúc khởi động là yêu cầu công khai, không gửi dữ liệu máy, mất mạng vẫn chạy. Bỏ QU-STD-AUTH; đăng nhập (nếu có sau này) chỉ là tùy chọn theo mục 14.
- 24/08/2026 - Bộ quy ước đồng bộ từ `tsudev-standards`, bản sao chỉ-đọc ở `.standards/`. Không sửa ngược.

## Sự cố bảo mật

> Ghi theo `.standards/docs/SECURITY_BASELINE.md` mục 9. Để trống nếu chưa có.

- (chưa có)

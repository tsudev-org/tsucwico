# STATE.md - Trạng thái project

> **Phiên kế tiếp:** (1) **REL-TSU-1** vẫn chờ 👤 (Sandbox chưa có `ketqua/`, VT 26.11.1901 NSIS 3/51). (2) Phiếu MỞ:
> **STD90** (đồng bộ chuẩn v9.0.0, phiếu `20261127-STD90`), **INT1** (Opus), **UI2** (còn Lighthouse/INP, phần B), **QR1**, SEC2-d (E8 SignPath - 👤). (3) 👤: co VHDX (phiếu DD1
> mục 3). Deploy: OAuth wrangler do 👤 đăng nhập 27/11; `~/.config/tsudev/cloudflare.env` vẫn là token đã thu hồi.

> Agent đọc file này ở đầu mỗi phiên và cập nhật ở cuối mỗi phiên.
> Quy trình: `.standards/docs/AGENT_PROTOCOL.md`.

## Hàng đợi task (làm từ trên xuống)

- [ ] **INT1** Toàn vẹn ứng dụng (phiếu `20261125-INT1`, chuẩn v8.3.0 `APP_INTEGRITY.md`): cổng phiên bản đọc
  `https://tsudev.com/.well-known/tsudev/versions.json` + `.sig` Ed25519, modal khoá toàn màn (Cập nhật + Xuất dữ liệu),
  gửi `X-Tsu-App`, xử lý `426`. Cần Opus (mật mã). Chờ tsudev phát hành bản kê (đợt 1).
- [ ] **SEC2** Trang `/security` + `security.txt`: **mã ✅** 26/11 (#119 gộp `1c4c558`, `docs/security/trust.json`); còn deploy +
  E8 SignPath (nộp đơn - 👤) - phiếu `20261125-SEC2`. **Deploy ✅** 27/11 Worker `b64df5cd`: `/security/` có
  nội dung thật, security.txt mailto + Policy.
- [ ] **UI2** Rà website bằng trình duyệt (A) + hiện đại hoá (B) - phiếu `20261125-UI2`. **A đợt 1 ✅** 26/11 (#120: preload
  font + dự phòng đo số, CLS 0,087-0,14 → 0; 54 tổ hợp 0 lỗi). Còn: đo production sau deploy, Lighthouse/INP, phần B.
- [ ] **QR1** QR4 + UI1 (tương phản biểu tượng ≥ 3:1, test chặn tái phát) - phiếu `20261124-QR1`.
- [ ] **AV1** 👤 gửi thư CrowdStrike/Trapmine (sổ `docs/store/antivirus.md`); đo lại VT 3-7 ngày sau.
- [ ] **REL-TSU-1** Bản đầu tiên mang tên Tsu Cwico + khuôn thẻ v3: **Release nháp [`v26.11.1901`](https://github.com/tsudev-org/tsucwico/releases/tag/v26.11.1901)** (19/11/2026, phiên tsucwico-ea, run 44531891240). Đã kiểm trên nháp: 10 tệp đúng khuôn (`tsucwico_26.11.1901_windows-x64-setup.exe`, `_windows-x64.msi`, `_windows-x64-cli.exe`, `_sbom.spdx.json`, `.sig`, `latest.json`, `SHA256SUMS.txt` + `.sig` + `.sigstore.json`); `sha256sum -c` 7/7; `tools/verify_update_signature.py` - bản đã cài chấp nhận 3 gói; `gh attestation verify` 4/4 (ký bởi `release.yml@refs/tags/v26.11.1901`); Sigstore `OK`; CLI có CompanyName TsuDEV / FileVersion 26.11.1901; `latest.json` cùng dạng URL với 26.11.10.2 (đã cập nhật thật được). **VirusTotal**: MSI + CLI 0/74; bộ cài NSIS 3/74 engine học máy (CrowdStrike, APEX, Trapmine) ⇒ báo nhầm theo phiếu `logs/handover/20261119-VT1_*` (SecureAge đã nộp; 👤 gửi 2 email soạn sẵn). **Còn 👤:** nâng cấp thật trên máy ảo Windows có snapshot từ 26.11.1002 (MSI và NSIS) - mỗi đường chỉ một mục "Tsu Cwico" trong Add/Remove Programs (ADR-0001 mục 3, `docs/RELEASING.md` mục 5) ⇒ **Publish** (updater bắt buộc: Publish là đẩy tới mọi máy). **20/11: thử nâng cấp chuyển sang Windows Sandbox, bộ kiểm `tools/sandbox/` - phiếu `20261120-SB1`.** Sau Publish: deploy website (trang tải về đọc `releases/latest` lúc build), `tools/apply_winget_manifest.py` (manifest trong artifact `installers`) + nộp winget (đợi #476024), Scoop tự lên nhờ scoop-bucket#24.
- [ ] **STD50** Bản sắc v5.0.0: **PR 1 ✅** #104 (sync v5.0.0, `products.css`, màu hue 212, bo góc `sac`, Be Vietnam Pro,
  xoá dải tsu/dev - TS-8), **PR 2 ✅** #105 (`trinh-dien` + `luoi-quet`, og-image). Còn: ảnh chụp trên Windows thật
  (`scan-*-vi`, `tweaks-dark-vi`) + video demo quay lại ⇒ đóng phiếu `20261121-STD50`.
- [x] **LG1** ✅ 21/11/2026 #103: logo Tsu Cwico trong app (header, Giới thiệu, màn trống, cập nhật, lỗi, favicon trùng byte
  website), khối nhà làm logo TsuDEV; trang "Đã đăng nhập" nhúng favicon. Phiếu `20261120-LG1` ĐÓNG.
- [x] **PRO2** Pro kỳ 1 năm: **2.1 ✅** #102 (nhắc gia hạn 7 ngày + báo hết kỳ, 6 test 7.6); nội dung 7.6 mới #117. 2.2
  (đồng bộ qua TsuDEV Sync) BỎ theo SYN1. Phiếu `20261120-PRO2`.
- [ ] **PRO1** Lưu cục bộ mặc định; Pro = đồng bộ cài đặt (TsuDEV Sync, crate `tsusync`); nhắc nhiều thiết bị mỗi 30
  ngày. Phiếu `logs/handover/20261119-PRO1_luu-cuc-bo-pro-dong-bo.md`, kế hoạch phase ở **ADR-0002 mục 3**.
  **P1 ✅** 20/11 (#94; kiểm kê 0 byte rời máy, chuẩn v3.9.1, `cwico_core::pro`, hộp thoại `ProNotice`). **P2 ✅ mã** 21/11 (#102: đăng nhập loopback PKCE, refresh token ở
  Credential Manager, kiểm JWS ES256, màn Pro, huy hiệu, THREAT_MODEL) - còn bước 7 trên Windows thật (phiếu PRO1 mục 7).
  **P3 BỎ 26/11/2026** (SYN1: không đồng bộ lên internet; ADR-0002 sửa đổi) - đồng bộ giữa máy nếu làm thì theo
  `LOCAL_SYNC.md`/`tsulink`, ADR mới. Hợp đồng JWS: PRODUCT_POLICY 7.4 (chuẩn v8.3.0).
- [ ] **P2-MSIX** Đích MSIX (`Version` = `YY.M.DDNN.0`) + nộp Microsoft Store + nút Store trên website. Chờ 👤 tạo tài
  khoản Partner Center cá nhân (phiếu P2 mục 4.1) - không đoán Identity Name/Publisher.
- [x] **T8** ✅ 19/11/2026 (phiên tsucwico-c4): #92 gộp `20de2ab`, deploy Worker version `2fb094b0`; đo production: 1 video
  (WebM phát được, 12 cue phụ đề), 9 `<picture>`, `media-src 'self'`, tệp video/AVIF/WebP 200 đúng content-type. Bộ ảnh
  cửa hàng (`assets/store/`) làm cùng P2-MSIX.
- [x] **SCOOP-1** ✅ 16/11/2026 (phiên tsudev-27): gói `tsucwico` có ở scoop-bucket (`dba0759`, cài thử đạt), `channels` có `scoop`; website deploy lại (Worker `2aa95bdf`) - `/tai-ve/` hiện `scoop install tsudev/tsucwico`. Mô tả gói chép nhầm của Swico ⇒ sửa ở scoop-bucket#14.
- [x] **BP-1** ✅ 17/11/2026 (phiên tsudev-27): hồ sơ [16838](https://www.bestpractices.dev/projects/16838) passing 100% (`docs/best-practices.json`, giấy phép MIT; `dynamic_analysis` Unmet trung thực). Kèm #75 (ghim SHA 20 action, quyền token, SECURITY.md khớp mã: có updater + chữ ký minisign), CodeQL default setup, #76 (cảnh báo CodeQL high ở check-dist). Còn: P1.5 good first issue (phiếu 20261113-03).
- [x] **LOCAL-DIR** ✅ 16/11/2026 (phiên tsudev-27): thư mục cục bộ đổi `~/projects/tsudev-cwico` -> `~/projects/tsucwico`.
- [x] **TSUDEV-276** ✅ tsudev#276 đã MERGE (chủ dự án duyệt 16/11/2026).
- [x] **WINGET-2** ✅ [microsoft/winget-pkgs#476024](https://github.com/microsoft/winget-pkgs/pull/476024) (manifest 26.11.1002) đã MERGE 21/11/2026 06:04 UTC (`Moderator-Approved`, `Publish-Pipeline-Succeeded`). Manifest 26.11.1901 nộp sau khi Publish (REL-TSU-1).
- [ ] **QA-WIN-1** Chạy thử phần GHI trên Windows thật, có quyền Admin: `cwico plan --safe-only` (không đổi gì), rồi gỡ một mục An toàn trên máy ảo có snapshot - kiểm điểm khôi phục, `.reg`, nhật ký giao dịch. Chưa phiên bản nào làm việc này. Phiên 06 (10/11/2026) đã làm phần CHỈ ĐỌC trên Windows 11 25H2 của chủ dự án, không Admin, bằng CLI tải từ bản phát hành 26.11.1002 (SHA-256 khớp): `info`, `plan --safe-only` -> 30 mục / 63 bước trong 2,8 giây, không bị từ chối mục nào. Phát hiện `plan` giấu bước điểm khôi phục + sao lưu registry - sửa ở PR #55. Còn lại: phần GHI trên máy ảo có Admin.
- [x] **QU-STD-1** XONG 10/11/2026 (phiếu `20261110-05`): chế độ Warm BỎ theo quyết định chủ dự án (chỉ Sáng/Tối/Hệ thống); nút chính, thanh tiến trình, ô chọn, hàng đang chọn dùng token `primary`. Màu chữ "tsu/dev" của wordmark vẫn theo QU-STD-BRAND/TS-8.
- [x] **QU-STD-BRAND** ✅ 22/11/2026 (phiên tsucwico-86): wordmark "tsu/dev" cam đã bỏ ở #103/#104 (TS-8). Đo lại axe-core trên app (`?preview=demo|demo,update|empty`, Sáng/Tối, vi/en): còn 2 lỗi khác - tên truy cập logo thiếu dòng phụ đề (2.5.3) và ghi chú cập nhật cuộn được nhưng không lấy được tiêu điểm (2.1.1) ⇒ sửa ở PR a11y, đo lại 0/12. Ảnh đại diện tổ chức GitHub là việc cấp tổ chức `tsudev-org`.
- [x] **DEP-1** ✅ cảnh báo #1 `glib < 0.20` đã đóng dạng `tolerable_risk` (đọc lại API 22/11/2026: 0 cảnh báo mở; 2-15 `fixed`).
- [x] **CF-ZONE-1** ✅ JS Detections tắt ở vùng tsudev.com (16/11/2026, tsudev#280); đo 16/11 23:10: HTML `cwico.tsudev.com` không còn `challenge-platform` / `__CF$cv`.

## Đang thực hiện

| Task | Agent | Bắt đầu |
| --- | --- | --- |

## Đã hoàn thành (mới nhất trên cùng)

- 27/11/2026 - Phiên tsucwico-6da2 (tiếp): chủ dự án ủy quyền làm phương án tối ưu ⇒ #123 `font-display: optional`
  (gộp `1e15801`), deploy Worker `6ed55d62`; production 16/16 lần CLS 0 (360/1366 x Sáng/Tối). Đánh đổi đo được: lần đầu
  mạng chậm (400 ms) hiện font hệ thống, 150 ms/360 px có chỗ còn font hệ thống; lần tải lại đủ Be Vietnam Pro.

- 27/11/2026 - Phiên tsucwico-6da2 (tiếp): 👤 `wrangler login` ⇒ deploy Worker version `b64df5cd` (main `9da97b1`). Đo
  production: theme-color Sáng `#eef1f5` (nền `rgb(238,241,245)`), Tối `#0a0a0a`; `/security/` 10 liên kết Kiểm chứng;
  security.txt `mailto:security@tsudev.com`; 4 preload font. CLS: 1366 0-0,035, 360 0 hoặc 0,257 (đua font - xem Phiên kế tiếp).

- 26/11/2026 - Phiên tsucwico-6da2: deploy website BỊ CHẶN (token CF thu hồi, OAuth hết hạn). **SEC2**: trang
  `/security/` + `trust.json` + security.txt `mailto:security@tsudev.com`, cổng Expires/nội dung trong `check-dist`; đo:
  SSL Labs A+ 4/4, Observatory A+ 140, HSTS `tsudev.com` preloaded, Scorecard 7.2, BP 100%, VT 26.11.1002 MSI 0/62, NSIS
  3/71, CLI 3/71 (#119 gộp). **UI2 A** #120: chỉ dính nguyên nhân 2 (font) ⇒ preload 400+600 + `Be Vietnam Pro Fallback`,
  CLS 0. REL-TSU-1 không đổi (VT 26.11.1901 NSIS 3/51). AKB: LES-20261126-2352-5f65, LES-20261126-2352-b6f6.

- 26/11/2026 - Phiên tsucwico-2e: **dọn đĩa toàn máy** (ổ C 9,2 → 22 GB trống, phiếu `20261126-DD1`, phối hợp tsudisk-4c);
  **SEC3**: gộp Dependabot #109 #110 #112 #113 #114, #111 thay bằng #115 (p256 0.14, MSRV khai đúng 1.95, cookie 0.18.3
  RUSTSEC-2026-0594), gitleaks toàn lịch sử 0, cargo/npm audit 0; **LG4/SYN1** #116 gỡ thông báo tự hiện 7.3, xoá khoá cục
  bộ cũ, test ca 17, chuẩn v8.3.0 (+ sửa theme-color website); **PRO3/SYN1** #117 màn Pro + 7.5/7.6 mới, bỏ Q3, ADR-0002
  sửa đổi (P3 `tsusync` BỎ); **AV1** sổ `docs/store/antivirus.md`; **SEC1** ĐÓNG; `.wrangler/` vào `.gitignore`. Chưa
  deploy website (token CF thu hồi). Xoá nhánh remote đã gộp bị chặn quyền ⇒ 👤.

- 22/11/2026 - Phiên tsucwico-86: vẫn chặn ở 👤 - `ketqua/` Sandbox trống, VirusTotal NSIS 3/74 (lượt 21/11 16:04 UTC),
  `tsudev.com/oidc/*` vẫn 503, Release nháp. Sửa 2 lỗi axe trong app (PR a11y), đóng QU-STD-BRAND + DEP-1 (mục cũ).

- 22/11/2026 - Phiên tsucwico (tiếp sau khi bị tắt): winget-pkgs#476024 đã gộp ⇒ đóng WINGET-2. Sandbox đã bật nhưng
  tự chạy `kiem-thu-msi.wsb` từ WSL không ra kết quả (phiếu SB1 mục 5) ⇒ chờ 👤. VirusTotal NSIS vẫn 3/74.
  `tsudev.com/oidc/auth` 503 `temporarily_unavailable`. Release vẫn nháp.

- 21/11/2026 - Phiên tsucwico-ea (tiếp sau khi bị tắt): gộp #102 (PRO1 P2 + PRO2 2.1), #103 (LG1), #104 (STD50 PR 1,
  chuẩn v5.0.0), #105 (STD50 PR 2); #106 chụp lại 5 ảnh `docs/screenshots` không cần Windows; deploy website (Worker version `babc50d9`, đo production: `data-product="cwico"`, hero `luoi-quet`, og-image 200). VirusTotal NSIS đọc lại
  16:04 UTC: vẫn 3/74 (lượt 20/11 00:39, CrowdStrike/APEX/Trapmine), gửi `analyse` lại (200). Release vẫn nháp.

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

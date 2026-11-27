# ADR-0003: Cổng phiên bản - bản kê `versions.json` ký Ed25519 cạnh `latest.json`, `X-Tsu-App`, Xuất dữ liệu

- **Trạng thái**: Chấp nhận - 27/11/2026.
- **Quyết định chủ dự án (27/11/2026)**: kiểm bản kê cho **mọi người dùng**, **cả hai** nguồn lúc mở app: `latest.json`
  trên GitHub (như cũ) và bản kê tsudev.com (mới). Người chưa đăng nhập cũng gọi tsudev.com (một GET, không dữ liệu người
  dùng) - cam kết "0 byte tới tsudev.com khi chưa đăng nhập" (SECURITY.md cam kết 1, THREAT_MODEL mục 3) được sửa theo.
- **Liên quan**: `.standards/docs/APP_INTEGRITY.md` v1.0.0 mục 1-2, 5.2, 6, 8; `LOCAL_SYNC.md` mục 9; ADR-0002 mục 1
  (kiểm kê dữ liệu rời máy); phiếu `logs/handover/20261125-INT1_*.md`; phía máy chủ: repo tsudev `docs/release-signing.md`
  (bản kê, `.sig`, JWKS, `POST /api/versions/raise`, cổng `X-Tsu-App`); tiền lệ: tsudisk ADR-0012.

## Bối cảnh

Tsu Cwico đã có cổng cập nhật chặn: lúc mở app, `tauri-plugin-updater` đọc `latest.json` của GitHub Releases; có bản mới
xác nhận được ⇒ `UpdateGate` thay toàn bộ app, chỉ còn nút cài (gói ký minisign, khoá ghim trong `tauri.conf.json`).
Lỗi mạng ⇒ chạy bình thường (fail open, lý do ở đầu `app/src-tauri/src/update.rs`).

APP_INTEGRITY thêm: bản kê chung có chữ ký, chốt chặn theo kênh, nguyên văn modal, nút **Xuất dữ liệu** (I6), kiểm lại
mỗi 6 giờ và khi quay lại sau ≥ 30 phút, header `X-Tsu-App` + xử lý `426`, gia cố bản phát hành (Cwico là OPEN, MIT:
không làm rối mã). tsudev.com đã chạy: `versions.json` + `.sig` (JWS tách nội dung, `{"alg":"EdDSA","kid":"tsu-versions-2026"}`)
+ `versions-jwks.json`; hôm nay bản kê chưa có mục `tsucwico`.

## Quyết định

1. **Hai nguồn, mọi người dùng, song song lúc mở app.** `latest.json` (GitHub) và bản kê (tsudev.com) chạy đồng thời
   trong thời hạn khởi động sẵn có (10 giây); riêng bản kê thời hạn 5 giây. Nguồn nào báo có bản mới ⇒ khoá. Cả hai lỗi
   ⇒ chạy bình thường như hôm nay. GET bản kê chỉ gửi `User-Agent` + `X-Tsu-App` (I5), không cookie, không định danh máy.
2. **Kiểm chữ ký bằng `ring`** (`ring::signature::ED25519`): `ring` 0.17 đã có trong `Cargo.lock` (rustls của updater và
   đăng nhập), nên không thêm crate mật mã mới; chỉ dùng hàm kiểm. Phương án loại: `ed25519-dalek` (thêm cây phụ thuộc
   thứ hai cho cùng việc), tự viết (cấm tự viết mật mã), dùng lại `minisign-verify` (khác định dạng).
3. **Khoá công khai ghim lúc dựng**: bảng hằng `kid -> 32 byte` trong `cwico_core::integrity` (`tsu-versions-2026` lấy từ
   JWKS production, test đối chiếu tệp JWKS mẫu). Không tải JWKS lúc chạy; không biến môi trường, tham số hay tệp cấu
   hình nào đổi được khoá hoặc URL (đó là cửa bỏ qua cổng). Xoay khoá: phát hành bản ghim cả khoá mới trước.
4. **Thứ tự kiểm** (sai bước nào bỏ bản kê ở bước đó): `.sig` đúng 3 phần, phần giữa rỗng ⇒ header `alg == "EdDSA"`,
   `kid` có trong bảng, chữ ký 64 byte ⇒ Ed25519 trên `b64url(header) + "." + b64url(byte tải về)` ⇒ chỉ khi đó mới
   parse JSON ⇒ `v == 1`, `expires_at - issued_at <= 7 ngày`, `now < expires_at`, `issued_at >=` bản đệm (chống phát
   lại). Trần kích thước: bản kê 256 KiB, `.sig` 1 KiB. HTTPS tới host cố định `tsudev.com`, không theo chuyển hướng.
5. **Đệm** byte bản kê + `.sig` đã đạt ở `%LOCALAPPDATA%\tsudev-cwico\versions\` (ghi tệp tạm rồi đổi tên). Bản đệm
   được kiểm lại đầy đủ mỗi lần đọc. Lỗi mạng ⇒ dùng bản đệm còn hạn; không có ⇒ không chặn.
6. **Kênh và so phiên bản.** Mã sản phẩm `tsucwico` (`cwico_core::version::PRODUCT`). Kênh: `scoop` khi tệp chạy nằm
   dưới `...\scoop\apps\tsucwico\`, còn lại `github` (winget cài đúng gói MSI của GitHub Releases và updater trong app
   cập nhật từ GitHub, nên `github` là nguồn cập nhật thật). Chỉ so `products.tsucwico.channels.<kênh của mình>.latest`
   bằng `Release::parse_name` (VERSIONING mục 2; bản thử nhỏ hơn bản chính cùng số). Sản phẩm/kênh vắng ⇒ không khoá.
   Bản tự dựng có phiên bản không đúng khuôn ⇒ không kiểm (OPEN, Kerckhoffs: bảo vệ thật là `426` ở máy chủ, I1).
7. **`X-Tsu-App: tsucwico/<YY.M.DDNN>/<kênh>`** là header mặc định của client HTTP tới tsudev.com (đăng nhập, quyền lợi,
   bản kê). `426 update_required` ⇒ ghi dấu `versions/server-lock.json` (gắn phiên bản đang chạy, hết hạn 24 giờ) và khoá
   như bản kê. `403 untrusted_client` ⇒ modal "Phiên bản không chính thức" (APP_INTEGRITY 2.3). Desktop không có chứng
   thực app: Cwico `MUST NOT` tự nhận "đã chứng thực".
8. **Thời điểm kiểm**: lúc mở app (trước khi hiện app, như hôm nay), mỗi 6 giờ, và khi cửa sổ được kích hoạt lại sau ≥ 30
   phút. Đang chạy dở một kế hoạch gỡ (RunPanel) ⇒ chờ chạy xong mới hiện modal - không bao giờ cắt ngang thao tác ghi.
9. **Modal khoá** (`UpdateGate`): nguyên văn APP_INTEGRITY 2.2 vi/en, đúng hai nút **Cập nhật ngay** + **Xuất dữ liệu**
   (bỏ nút đổi ngôn ngữ trong cổng; ngôn ngữ theo Cài đặt/hệ thống). `role="dialog"`, `aria-modal`, bẫy focus, chặn
   Esc. **Cập nhật ngay**: updater có gói ⇒ cài gói đã ký như hôm nay; không có (bản kê/`426` đi trước `latest.json`, hoặc
   kênh Scoop) ⇒ Scoop: sao chép `scoop update tsucwico` và hiện lệnh; GitHub: mở `url` của bản kê đã kiểm, chỉ khi host
   là `github.com/tsudev-org/tsucwico/` hoặc `tsudev.com` - lệnh Rust không nhận URL từ web view (giữ cam kết 5).
10. **Xuất dữ liệu** (I6, LOCAL_SYNC 9): lệnh Rust `export_data` mở hộp thoại lưu tệp gốc (phía Rust, không cấp quyền
    filesystem cho web view - cam kết 4), ghi `.tsubak` (zip) gồm thư mục sao lưu (`.reg`, kịch bản khôi phục, nhật ký
    giao dịch), `settings.json` (giao diện, ngôn ngữ, số dòng mỗi trang) và `manifest.json` (tệp + SHA-256). Không xuất
    refresh token, `account.json`, JWS - đó là bí mật/thẻ của thiết bị. Không mã hoá: dữ liệu D1 (LOCAL_SYNC 9 chỉ bắt
    buộc mã hoá từ D2). Nút có ở modal khoá, modal không chính thức và trang Giới thiệu/Cài đặt.
11. **CLI** (`cwico`) không gọi mạng (dùng trong kịch bản quản trị, máy không mạng). CLI đọc bản kê đệm + dấu `426` mà
    app GUI đã ghi (kiểm chữ ký lại); biết chắc có bản mới ⇒ từ chối lệnh GHI (gỡ/áp dụng), lệnh chỉ đọc vẫn chạy, in
    nguyên văn 2.2 + cách cập nhật, mã thoát riêng. Không có bản đệm ⇒ không chặn.
12. **Gia cố bản phát hành (OPEN, 5.2)**: `[profile.release]` `lto = "fat"`, `strip = "symbols"`, `debug = false`,
    `codegen-units = 1`, `panic = "abort"` (giữ `opt-level = "z"`); `release.yml` thêm `--remap-path-prefix` bỏ đường dẫn
    máy dựng. Không làm rối mã, không packer (5.3).
13. **Nâng `latest`** (chốt chặn theo kênh): workflow `kenh-phat-hanh.yml` chạy khi Release được **Publish**, mỗi 6 giờ và
    `workflow_dispatch`; Environment `versions`, secret `TSU_VERSIONS_TOKEN`, quyền `contents: read`. Kênh `github`: POST
    `url` = `.../releases/tag/v<bản>`. Kênh `scoop`: chỉ khi `bucket/tsucwico.json` ở `tsudev-org/scoop-bucket` có
    `version` bằng bản mới. `409 version_not_newer` = xong; `409 download_unverified` thử lại 3 lần cách 2 phút; lỗi ⇒ mở
    Issue, không chạm Release. Release vẫn là **nháp tới khi chủ dự án Publish** - không có đường nào nâng `latest` trước.

## Hệ quả

- (+) Đạt APP_INTEGRITY mục 8 cho desktop OPEN; người dùng Scoop không bị khoá trước khi bucket có bản.
- (+) Lộ `TSU_VERSIONS_TOKEN` chỉ nâng được `latest` lên bản tải được thật (máy chủ GET `url`).
- (-) Thêm một đích mạng (`tsudev.com`) cho người chưa đăng nhập; phải sửa SECURITY.md cam kết 1, THREAT_MODEL mục 3,
  ADR-0002 mục 1, trang riêng tư trên website.
- (-) Rủi ro lớn nhất vẫn là nâng `latest` khi chưa có bản: ba lớp giảm - máy chủ GET thật `url`, workflow chỉ chạy sau
  Publish/khi bucket có bản, modal luôn có **Xuất dữ liệu**.

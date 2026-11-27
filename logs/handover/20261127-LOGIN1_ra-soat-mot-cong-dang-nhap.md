# PHIẾU RÀ SOÁT MỘT CỔNG ĐĂNG NHẬP - tsucwico

> Mẫu theo `AUTH_AND_ACCOUNT.md` mục 2.1. KHÔNG dán giá trị bí mật, token hay dữ liệu tài khoản vào phiếu.

- **Trạng thái**: ĐÓNG (27/11/2026, #127) - **Bộ quy ước**: v9.0.0 - **Phiên**: tsucwico (STD90), 27/11/2026
- **Bề mặt**: desktop (Tauri) + website tĩnh + CLI - **Có tài khoản TsuDEV**: có (app desktop, không bắt buộc)

## 1. Kết quả quét

`.standards/scripts/check-central-login.sh --strict`: 0 nghi vấn. Đã đưa vào CI (`.github/workflows/standards.yml`).

## 2. Rà tay (đọc mã; CHƯA chạy sản phẩm trên Windows - cùng bước 7 phiếu PRO1)

- [x] Website: không trang/hộp thoại đăng nhập, đăng ký, quên mật khẩu; không ô mật khẩu/OTP
- [x] Không nút, SDK, OAuth client riêng của Google/Apple/GitHub; không WebView hứng đăng nhập
- [x] App/CLI: chỉ nút mở trình duyệt (PKCE + `state` + `iss`) **và** nút mở trang tài khoản (V2 đã sửa)
- [x] `client_id` + `redirect_uri` đăng ký tại tsudev.com (tsudev#339); URL lấy từ discovery (V1 đã sửa)
- [x] Token ở kho bí mật hệ điều hành, có đăng xuất; nút xoá tài khoản (V3 đã sửa)
- [x] Máy chủ sản phẩm không nhận, không lưu mật khẩu (sản phẩm không có máy chủ riêng)
- [x] `/privacy` và hồ sơ cửa hàng khớp (27/11: chỉ thêm GET discovery sau khi bấm đăng nhập, không gửi dữ liệu mới; khối "Dữ liệu chỉ ở máy này" vẫn đúng; hồ sơ cửa hàng chưa nộp) dữ liệu gửi đi: đối chiếu lại khi sửa V1-V7 (hồ sơ cửa hàng chưa nộp)

## 3. Vi phạm và cách sửa

| Vi phạm (tệp:dòng) | Cách sửa | Di trú dữ liệu | PR | Trạng thái |
| --- | --- | --- | --- | --- |
| V1 `crates/cwico-core/src/pro/signin.rs:16-18`, `jws.rs:24`: ghi cứng authorize/token/revocation/JWKS (mục 2, 2.1) | Đọc `https://tsudev.com/oidc/.well-known/openid-configuration` sau cú bấm; kiểm `issuer` khớp `ISSUER`, mọi endpoint `https://tsudev.com`; không gọi mạng trước khi đăng nhập | không | | ĐÃ SỬA #127 |
| V2 `ui/src/components/AccountPanel.tsx`: không có nút "Mở trang tài khoản" | Lệnh Tauri URL cố định `https://tsudev.com/settings/profile` (đo 200) | không | | ĐÃ SỬA #127 |
| V3 như trên: không có "Xoá tài khoản" | Nút hiện khi đã đăng nhập, bước xác nhận, mở `https://tsudev.com/settings/security` (trang xoá của cổng) | không | | ĐÃ SỬA #127 |
| V4 `ui/src/i18n` `account.signIn`: "Đăng nhập TsuDEV" | Đổi thành "Đăng nhập trên trình duyệt" / "Sign in in your browser" | không | | ĐÃ SỬA #127 |
| V5 thiếu "Vì sao nên đăng nhập?" (mục 14.4.2.1) | Liên kết `https://tsudev.com/privacy#dang-nhap` khi chưa đăng nhập | không | | ĐÃ SỬA #127 |
| V6 `app/src-tauri/src/account.rs` `access_token`: phiên bị từ chối thì về "chưa đăng nhập" không nói gì (14.4.2.5) | Lưu cờ hết phiên, hiện "Phiên đã hết hạn - Đăng nhập lại", không hộp thoại | khoá mới trong `account.json` | | ĐÃ SỬA #127 |
| V7 app không đọc `status` tài khoản (14.4.3) | Đọc claim `status`/`status_reason`; `suspended`/`restricted` hiện huy hiệu + lý do + đường khiếu nại trong mục Tài khoản; xác minh nguồn claim theo hợp đồng tsudev trước | không | | ĐÃ SỬA #127 |

Gia cố kèm V1 (khuyến nghị, không phải vi phạm đã xác định): kiểm tham số `iss` ở callback (RFC 9207, discovery báo hỗ trợ).

## 4. Ngoại lệ đã khai trong `.central-login-allow`

Không có (không cần tệp).

Đóng phiếu khi mục 3 hết dòng mở và `check-central-login.sh --strict` đạt trong CI. Bản phát hành kế tiếp không còn ô
nhập tài khoản (hiện đã đạt). Sửa mã đăng nhập: dùng Opus, PR riêng.

## 5. Kết quả sửa (27/11/2026, phiên tsucwico)

- Nhánh `fix/login1-mot-cong-dang-nhap` (tách từ `chore/std90-sync-v9.0.0`). Gia cố RFC 9207 (`iss`) và không theo
  chuyển hướng HTTP làm kèm.
- Kiểm: `cargo fmt`, `cargo clippy --all-targets -D warnings`, `cargo test` (workspace mặc định) xanh; `cwico-app`
  biên dịch trên Linux (cảnh báo `Emitter` không dùng ở `update.rs` có từ trước, chỉ trên Linux); UI build + test;
  axe-core 20 tổ hợp (5 trạng thái x Sáng/Tối x vi/en) 0 lỗi, hộp xác nhận xoá 0 lỗi sau khi thêm `aria-labelledby`
  cho `Modal`; `check-central-login.sh --strict` 0 nghi vấn.
- CI #127 xanh 12/12 (gồm Windows build và cổng `--strict`); README số test 168 → 177. STD90 gộp #126.
- CHƯA kiểm: đăng nhập đầu cuối
  trên Windows thật (phiếu PRO1 bước 7, 👤).
- Ghi nhận cho chủ dự án: AUTH_AND_ACCOUNT mục 2 nêu discovery ở `https://tsudev.com/.well-known/openid-configuration`
  nhưng địa chỉ đó trả 404; discovery thật nằm dưới issuer `https://tsudev.com/oidc/.well-known/...` (đúng OIDC
  Discovery 4). Câu chữ chuẩn nên sửa ở tsudev-standards - không sửa hộ.

ĐÓNG 27/11/2026: #127 gộp, CI xanh. Việc còn lại thuộc phiếu PRO1 bước 7 (👤).

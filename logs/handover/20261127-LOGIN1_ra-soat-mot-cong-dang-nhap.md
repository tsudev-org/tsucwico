# PHIẾU RÀ SOÁT MỘT CỔNG ĐĂNG NHẬP - tsucwico

> Mẫu theo `AUTH_AND_ACCOUNT.md` mục 2.1. KHÔNG dán giá trị bí mật, token hay dữ liệu tài khoản vào phiếu.

- **Trạng thái**: MỞ - **Bộ quy ước**: v9.0.0 - **Phiên**: tsucwico (STD90), 27/11/2026
- **Bề mặt**: desktop (Tauri) + website tĩnh + CLI - **Có tài khoản TsuDEV**: có (app desktop, không bắt buộc)

## 1. Kết quả quét

`.standards/scripts/check-central-login.sh --strict`: 0 nghi vấn. Đã đưa vào CI (`.github/workflows/standards.yml`).

## 2. Rà tay (đọc mã; CHƯA chạy sản phẩm trên Windows - cùng bước 7 phiếu PRO1)

- [x] Website: không trang/hộp thoại đăng nhập, đăng ký, quên mật khẩu; không ô mật khẩu/OTP
- [x] Không nút, SDK, OAuth client riêng của Google/Apple/GitHub; không WebView hứng đăng nhập
- [ ] App/CLI: chỉ nút mở trình duyệt (PKCE + `state` ✓) **và** nút mở trang tài khoản (thiếu - V2)
- [ ] `client_id` + `redirect_uri` đăng ký tại tsudev.com ✓ (tsudev#339); URL lấy từ discovery (chưa - V1)
- [ ] Token ở kho bí mật hệ điều hành ✓, có đăng xuất ✓; nút xoá tài khoản (thiếu - V3)
- [x] Máy chủ sản phẩm không nhận, không lưu mật khẩu (sản phẩm không có máy chủ riêng)
- [ ] `/privacy` và hồ sơ cửa hàng khớp dữ liệu gửi đi: đối chiếu lại khi sửa V1-V7 (hồ sơ cửa hàng chưa nộp)

## 3. Vi phạm và cách sửa

| Vi phạm (tệp:dòng) | Cách sửa | Di trú dữ liệu | PR | Trạng thái |
| --- | --- | --- | --- | --- |
| V1 `crates/cwico-core/src/pro/signin.rs:16-18`, `jws.rs:24`: ghi cứng authorize/token/revocation/JWKS (mục 2, 2.1) | Đọc `https://tsudev.com/oidc/.well-known/openid-configuration` sau cú bấm; kiểm `issuer` khớp `ISSUER`, mọi endpoint `https://tsudev.com`; không gọi mạng trước khi đăng nhập | không | | MỞ |
| V2 `ui/src/components/AccountPanel.tsx`: không có nút "Mở trang tài khoản" | Lệnh Tauri URL cố định `https://tsudev.com/settings/profile` (đo 200) | không | | MỞ |
| V3 như trên: không có "Xoá tài khoản" | Nút hiện khi đã đăng nhập, bước xác nhận, mở `https://tsudev.com/settings/security` (trang xoá của cổng) | không | | MỞ |
| V4 `ui/src/i18n` `account.signIn`: "Đăng nhập TsuDEV" | Đổi thành "Đăng nhập trên trình duyệt" / "Sign in in your browser" | không | | MỞ |
| V5 thiếu "Vì sao nên đăng nhập?" (mục 14.4.2.1) | Liên kết `https://tsudev.com/privacy#dang-nhap` khi chưa đăng nhập | không | | MỞ |
| V6 `app/src-tauri/src/account.rs` `access_token`: phiên bị từ chối thì về "chưa đăng nhập" không nói gì (14.4.2.5) | Lưu cờ hết phiên, hiện "Phiên đã hết hạn - Đăng nhập lại", không hộp thoại | khoá mới trong `account.json` | | MỞ |
| V7 app không đọc `status` tài khoản (14.4.3) | Đọc claim `status`/`status_reason`; `suspended`/`restricted` hiện huy hiệu + lý do + đường khiếu nại trong mục Tài khoản; xác minh nguồn claim theo hợp đồng tsudev trước | không | | MỞ |

Gia cố kèm V1 (khuyến nghị, không phải vi phạm đã xác định): kiểm tham số `iss` ở callback (RFC 9207, discovery báo hỗ trợ).

## 4. Ngoại lệ đã khai trong `.central-login-allow`

Không có (không cần tệp).

Đóng phiếu khi mục 3 hết dòng mở và `check-central-login.sh --strict` đạt trong CI. Bản phát hành kế tiếp không còn ô
nhập tài khoản (hiện đã đạt). Sửa mã đăng nhập: dùng Opus, PR riêng.

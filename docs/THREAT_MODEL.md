# Mô hình mối đe doạ - Tsu Cwico

> Phạm vi: đăng nhập TsuDEV tùy chọn + quyền lợi TsuDEV Pro (ADR-0002 phase P2, PRODUCT_POLICY 7.4-7.6).
> Phần gỡ phần mềm / dọn dẹp có lớp bảo vệ riêng ở `docs/SAFETY.md`. Cập nhật lần cuối: 27/11/2026 (LOGIN1: discovery, `iss` RFC 9207, trạng thái tài khoản).

## 1. Tài sản và ranh giới tin cậy

| Tài sản | Ở đâu | Ai đọc được |
| --- | --- | --- |
| Refresh token tsudev.com (xoay vòng, 30 ngày) | Windows Credential Manager, mục `com.tsudev.cwico` / `tsudev-refresh-token` (DPAPI, theo người dùng Windows) | tiến trình chạy dưới cùng người dùng Windows |
| Access token (15 phút) | bộ nhớ tiến trình Rust | không ra đĩa, không vào web view |
| JWS quyền lợi + JWKS đã dùng để kiểm + mã cài đặt + mốc thông báo | `%LOCALAPPDATA%\com.tsudev.cwico\account.json` | không bí mật; JWS được kiểm lại chữ ký mỗi lần đọc |
| Dữ liệu máy (danh sách phần mềm, kết quả quét, nhật ký) | chỉ trên máy | không bao giờ gửi đi (ADR-0002 mục 1) |

Ranh giới: web view (UI) ⇄ lõi Rust qua IPC Tauri. Web view KHÔNG nhận token nào; lệnh `account_*` chỉ trả
`AccountView` (đăng nhập?, Pro?, `pro_until`, huy hiệu, số thiết bị). CSP của web view vẫn `connect-src 'self' ipc:` -
mọi lưu lượng tới tsudev.com đi từ Rust.

## 2. Mối đe doạ và biện pháp

| # | Mối đe doạ | Biện pháp | Kiểm chứng |
| --- | --- | --- | --- |
| T1 | **Giả mạo quyền lợi** - sửa `account.json` để tự bật Pro, hoặc JWS `alg: none` / HS256 ký bằng khoá công khai | JWS kiểm ES256 bằng JWKS tsudev.com, chọn khoá theo `kid`, chỉ nhận `alg` = ES256, `typ` = `tsu-ent+jwt`, khoá EC P-256; kiểm `iss`, `aud` = `tsucwico`, `exp`, `exp - iat` ≤ 7 ngày. Kiểm lại MỖI lần đọc cache. Sửa JWKS trong cache cũng vô ích với máy chủ đồng bộ (P3): máy chủ tự kiểm bằng JWKS của nó | `pro::jws::tests` (thân bị sửa, khoá khác, `kid` lạ, `alg` none/HS256, `typ` sai, P-384) |
| T2 | **Token bị đánh cắp** - mã độc cùng người dùng đọc Credential Manager | Ngoài phạm vi bảo vệ của app (cùng quyền người dùng = thua sẵn); hạn chế thiệt hại: refresh token xoay vòng (dùng lại token cũ ⇒ tsudev.com thu hồi cả grant), access token 15 phút chỉ trong bộ nhớ, scope tối thiểu `openid tsudev.entitlements` - chỉ đọc được trạng thái Pro, không đọc dữ liệu máy. Đăng xuất gọi `revocation` rồi xoá cục bộ. DPoP (gắn khoá thiết bị) bật khi tsusync hỗ trợ (tsusync ADR-0002 mục 3.2) | đọc mã `account.rs` |
| T3 | **Cổng loopback bị chiếm** - tiến trình khác nghe trước / bắt `code` | Bind `127.0.0.1:0` (cổng ngẫu nhiên do hệ điều hành cấp, không ra mạng); `state` 128 bit chống CSRF; PKCE S256 - `code` bắt được vô dụng nếu không có `code_verifier` (chỉ trong bộ nhớ); `nonce` kiểm trong `id_token`; `iss` ở callback phải bằng issuer (RFC 9207 - discovery báo hỗ trợ nên thiếu cũng từ chối), chặn mã từ máy chủ ủy quyền khác; nhận đúng MỘT request `/callback` rồi đóng cổng; đường khác ⇒ 404; hết 5 phút ⇒ huỷ | `pro::signin::tests` (RFC 7636 phụ lục B, `state` sai/thiếu, `iss` sai/thiếu, đường lạ, POST) |
| T4 | **Chuyển hướng giả mạo / client khác** | `redirect_uri` loopback đăng ký trên máy chủ (tsudev#339: `localhost` / đường khác ⇒ 400); `id_token` kiểm `iss`, `aud` = `tsucwico` (mảng chỉ một phần tử), `nonce`, `exp` | `pro::jws::tests::id_token_*` |
| T5 | **Máy chủ / mạng giả** (MITM) | HTTPS bắt buộc (`https_only`), rustls + kho chứng chỉ hệ thống, không theo chuyển hướng (307 không mang `code`/token đi nơi khác). Điểm cuối OIDC đọc từ discovery `https://tsudev.com/oidc/.well-known/openid-configuration` (AUTH_AND_ACCOUNT 2.1), chỉ sau cú bấm Đăng nhập hoặc khi đã đăng nhập; tài liệu bị từ chối cả khối nếu `issuer` khác hoặc bất kỳ điểm cuối nào không phải `https://tsudev.com/...` (không cổng lạ, không userinfo) hoặc thiếu PKCE S256. Trang tài khoản/xoá/khiếu nại là enum cố định trong lõi - web view không truyền URL nào | `pro::discovery::tests`, đọc mã |
| T6 | **Lộ dữ liệu khi chưa đăng nhập** | Không có refresh token ⇒ `account_refresh` trả ngay, 0 request; mã cài đặt chỉ sinh khi gọi quyền lợi lần đầu sau đăng nhập | đo proxy (mục 3) |
| T7 | **Liệt kê thiết bị** | `X-Tsu-Installation` là 128 bit ngẫu nhiên (AUTH_AND_ACCOUNT 14.3), không suy từ phần cứng; app chỉ nhận về `devices_30d` (một con số) | `pro::tests::installation_id_*` |
| T8 | **Đồng hồ máy bị chỉnh** | `iat` ở tương lai > 60 giây ⇒ từ chối; mốc thông báo ở tương lai ⇒ coi như chưa hiện; hết kỳ báo theo `pro_until` của JWS mới nhất | `pro::tests`, `pro::renewal::tests` |
| T10 | **Hiển thị sai trạng thái tài khoản** | `status`/`status_reason` chỉ đọc từ JWS quyền lợi đã kiểm chữ ký; app không tự suy luận vi phạm, không tự chặn tính năng (chỉ hiện huy hiệu + lý do + Khiếu nại); giá trị lạ ⇒ không hiện | `pro::tests::claims_carry_*`, `only_states_*` |
| T9 | **Dùng lại biên nhận / vượt hạn mức** | Không áp dụng ở app: biên nhận chỉ xác minh ở tsudev.com (PRODUCT_POLICY 7.4); hạn mức do TsuDEV Sync (P3) | - |

## 3. Đo "chưa đăng nhập ⇒ 0 byte tới tsudev.com"

Trên Windows: chạy app qua proxy ghi lại (Fiddler / mitmproxy, `HTTPS_PROXY`), không bấm Đăng nhập, mở đủ 4 tab, quét
một lần. Kỳ vọng: chỉ một request - kiểm tra cập nhật tới `github.com/tsudev-org/tsucwico/releases/latest/...`; KHÔNG
request nào tới `tsudev.com`. Kết quả ghi vào phiếu `logs/handover/20261119-PRO1_*` mục 7.

## 4. Rà soát bảo mật P2 (21/11/2026)

- [x] Không token nào đi qua IPC hay vào `localStorage`.
- [x] Refresh token chỉ trong Credential Manager; xoay vòng được lưu NGAY khi nhận, trước mọi bước có thể lỗi;
  làm mới được tuần tự hoá (khoá `tokens`) để hai lần làm mới song song không dùng lại cùng một token.
- [x] `invalid_grant` khi làm mới ⇒ xoá token cục bộ, app chạy như bản miễn phí, không báo lỗi chặn.
- [x] Lệnh mở trình duyệt không nhận URL từ web view (`open_pro_page`, đăng nhập dựng URL trong Rust).
- [ ] Đo proxy mục 3 trên Windows thật (👤 hoặc phiên có máy Windows).
- [ ] Đăng nhập đầu cuối với tài khoản thật: token ⇒ `/api/me/entitlements` ⇒ JWS `aud` = `tsucwico`.

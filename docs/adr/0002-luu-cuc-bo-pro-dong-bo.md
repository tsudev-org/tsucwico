# ADR-0002: Lưu cục bộ mặc định, TsuDEV Pro = đồng bộ cài đặt giữa các máy

- **Trạng thái**: Đã chấp nhận (20/11/2026, phiên tsucwico-e6, theo quyết định chung của chủ dự án 19/11/2026 -
  PRODUCT_POLICY v1.1.0 mục 7, bộ quy ước v3.9.1)
- **Phiếu**: `logs/handover/20261119-PRO1_luu-cuc-bo-pro-dong-bo.md` (phiếu gốc
  `~/projects/logs/handover/20261119-PRO1_*`)
- **Liên quan**: hạng C - ngoại tuyến (STATE "Quyết định quan trọng" 09/11/2026), ADR-0001

## 1. Kiểm kê: dữ liệu nào đang rời máy (20/11/2026, bản 26.11.1901)

| Đường đi | Gửi gì | Dữ liệu người dùng? |
| --- | --- | --- |
| Kiểm tra cập nhật lúc khởi động (`tauri-plugin-updater`, chỉ bản Windows) | `GET .../releases/latest/download/latest.json` trên GitHub, rồi tải gói cài nếu có bản mới. Không tham số, không định danh máy | Không |
| Nút Truy cập tsudev.com / Ủng hộ / thư mục sao lưu | Mở trình duyệt mặc định (hoặc Explorer); app không tự gửi gì | Không |
| Cài đặt giao diện (Sáng/Tối/Hệ thống, ngôn ngữ, số dòng mỗi trang) | `localStorage` của WebView2 trên máy | Ở lại máy |
| Sao lưu registry `.reg`, nhật ký giao dịch, điểm khôi phục | Thư mục sao lưu cục bộ (`about.backupDir`) | Ở lại máy |
| Đo lường, báo lỗi, quảng cáo | Không có (CSP `connect-src 'self' ipc:`) | - |

Kết luận: bản miễn phí đã đạt N2 - **0 byte dữ liệu người dùng rời máy**, không máy chủ nào giữ dữ liệu của người dùng
miễn phí, không có đường đồng bộ miễn phí nào phải tắt. Đo bằng proxy làm ở phase P2 (cùng lúc có đường mạng mới).

## 2. Quyết định

1. **Miễn phí giữ nguyên toàn bộ tính năng hiện có**: quét, gỡ, tinh chỉnh, sao lưu/khôi phục ra tệp, CLI. Không tính
   năng nào đang miễn phí bị chuyển sang Pro.
2. **Pro = đồng bộ cài đặt của Tsu Cwico giữa các máy cùng tài khoản** (giao diện, ngôn ngữ, danh sách mục tự loại trừ
   và bộ tinh chỉnh đã chọn khi có), qua dịch vụ dùng chung TsuDEV Sync (`tsudev-org/tsusync`, crate Rust `tsusync`),
   mã hoá đầu cuối, hạn mức mặc định 100 MiB + 1 GiB (dư nhiều). Không thêm tính năng Pro nào khác khi chủ dự án chưa
   duyệt. Dữ liệu đồng bộ là cấu hình (D1), vẫn mã hoá đầu cuối theo hợp đồng chung.
3. **Đăng nhập là tùy chọn** (AUTH_AND_ACCOUNT mục 14): tài khoản TsuDEV qua OIDC Authorization Code + PKCE với
   `redirect_uri` loopback `http://127.0.0.1:<cổng ngẫu nhiên>/callback` (RFC 8252), `client_id` = `tsucwico`, scope
   `openid tsudev.entitlements`. Token lưu bằng DPAPI (Windows Credential Manager). App vẫn là hạng C: không đăng nhập
   ⇒ mọi thứ như hôm nay, không có lời nhắc đăng nhập nào.
4. **Quyền lợi**: `GET https://tsudev.com/api/me/entitlements` (header `X-Tsu-Installation`) ⇒ JWS ES256 kiểm bằng
   JWKS `https://tsudev.com/oidc/jwks` (chọn theo `kid`, chỉ nhận `alg` = `ES256`, `typ` = `tsu-ent+jwt`), rồi
   `cwico_core::pro::EntitlementStatus::from_claims` kiểm `iss`, `aud`, `exp`, trần 7 ngày. Đệm cục bộ; quá `exp` ⇒
   miễn phí, không báo lỗi. Gate Pro thật nằm ở máy chủ TsuDEV Sync (`402 pro_required`); gate trong app chỉ ẩn/hiện.
5. **Mã cài đặt** 128 bit (`cwico_core::pro::new_installation_id`) sinh và lưu ở thư mục dữ liệu app lần đầu chạy
   bản có đăng nhập; chỉ gửi khi đã đăng nhập.
6. ~~**Thông báo nhiều thiết bị**~~ - **GỠ 26/11/2026** (PRODUCT_POLICY v2.0.0 bỏ mục 7.3; v3.0.0 mục 3.8 + AUTH 14.4
   cấm mọi lời mời tự hiện - phiếu `20261125-SYN1`, `20261126-LG4`): bỏ `notice_due`, lệnh `pro_notice`,
   `ProNotice.tsx`; khoá `tsu.pro_notice.last_shown` + `tsu.signin_reminder.*` xoá lúc khởi động
   (`cwico_core::pro::LEGACY_LOCAL_KEYS`). `devices_30d` trong JWS vẫn đọc, không hiện thành lời mời. Bề mặt tự hiện duy
   nhất còn lại là nhắc gia hạn/hết kỳ 7.6 (chỉ người đã mua Pro); test `free_user_sees_no_notice_over_five_opens_and_thirty_days`.
7. **Kênh thanh toán**: bản tải trực tiếp/winget/Scoop ⇒ `https://tsudev.com/pro?app=tsucwico`, Ủng hộ ⇒
   `https://tsudev.com/ung-ho?app=tsucwico` (PRODUCT_POLICY 3.3, 7.4). Bản Microsoft Store (P2-MSIX) đọc lại 4.3 trước
   khi nộp. **Không đặt nút mua trong app khi `tsudev.com/pro` chưa có** (20/11/2026 trả 404 - phase B3 của tsudev).
8. **Ân hạn**: không cần (chưa có người dùng thật, chưa có đồng bộ miễn phí). Pro hết hạn ⇒ theo PRODUCT_POLICY 7.2;
   cài đặt trên máy không bao giờ bị xoá.

## 3. Các phase (mỗi phase vừa một phiên)

| Phase | Việc | Chờ |
| --- | --- | --- |
| **P1** ✅ | Kiểm kê, ADR này, chuẩn v3.9.1, `cwico_core::pro` (claims, trạng thái, nhắc 30 ngày, mã cài đặt), hộp thoại + i18n vi/en, khối "Dữ liệu chỉ ở máy này" ở Giới thiệu, `DONATE_URL` có `?app=tsucwico` | - |
| **P2** ✅ 21/11 (#102; còn đo đầu cuối trên Windows) | Đăng nhập loopback PKCE + DPAPI, kiểm JWS ES256 (RustCrypto `p256`), lưu mã cài đặt, nút Đăng nhập/Đăng xuất + huy hiệu vẽ từ `assets/badges/`, màn Pro (điều khoản PRODUCT_POLICY 7.5 trước nút), phiếu rà soát bảo mật + `docs/THREAT_MODEL.md` (giả mạo quyền lợi, token bị đánh cắp, cổng loopback bị chiếm), đo proxy 0 byte | tsudev đăng ký client `tsucwico` (redirect loopback) + PRO-B gộp (tsudev#336) + B3 (`/pro`) |
| **P3** | Đồng bộ cài đặt qua crate `tsusync`; 402 ⇒ màn Pro, không thử lại; "Chưa đồng bộ" khi 429/5xx; đo < 100 ms | PRO-D3 (crate Rust `tsusync`) |

## 4. Hệ quả

- Không thêm phụ thuộc mật mã ở P1 (chỉ `getrandom` đã có sẵn trong cây phụ thuộc); `p256` vào ở P2 khi có chỗ dùng.
- (26/11/2026) Hộp thoại nhiều thiết bị đã gỡ - xem mục 2.6.

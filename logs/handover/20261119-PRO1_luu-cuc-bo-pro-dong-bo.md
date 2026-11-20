# PHIẾU YÊU CẦU - PRO1: lưu cục bộ mặc định, Pro mới có đồng bộ/sao lưu đám mây (tsucwico)

- **Mã phiếu**: 20261119-PRO1 (phiếu con)
- **Từ**: agent-qulythuchi (phiên `qulythuchi-b2`), 19/11/2026, theo yêu cầu trực tiếp của chủ dự án
- **Đến**: phiên kế tiếp của repo `tsucwico`
- **Trạng thái**: MỞ - chưa viết mã trước khi tsudev-standards v3.9.0 (PRO-A) phát hành; phần nào cần máy chủ quyền lợi
  thì chờ PRO-B
- **Phiếu gốc (ĐỌC TRƯỚC)**: `~/projects/logs/handover/20261119-PRO1_ke-hoach-luu-cuc-bo-pro-dong-bo-toan-he.md` - mục 1 (yêu cầu nguyên văn tóm tắt), 2 (nguyên tắc N1-N8), 3 (thông báo nhiều
  thiết bị, chu kỳ 30 ngày), 5 (khung việc mỗi repo), 7 (nghiệm thu), 8 (câu hỏi 👤). Phiếu này THAY hàng T7 của
  `~/projects/logs/handover/20261119-01_ke-hoach-toan-he-v3.8.0.md` cho repo này.

## 1. Việc riêng của repo này

Desktop Rust, D1. Như tsufiles: Pro = đồng bộ cài đặt/cấu hình qua PRO-D (thư viện máy khách Rust của PRO-D), đăng nhập loopback PKCE, thanh toán qua tsudev.com, huy hiệu.

## 2. Khung việc (tóm tắt phiếu gốc mục 5)

1. Kiểm kê: dữ liệu nào đang rời thiết bị, nằm ở đâu ⇒ ghi mục 4 dưới đây.
2. ADR riêng: phần nào là Pro, cách ân hạn cho người dùng cũ, kênh thanh toán theo PRODUCT_POLICY 3.3/4.3.
3. Mã: đọc quyền lợi (JWS tsudev, đệm ≤ 7 ngày), gate Pro ở MÁY CHỦ, thông báo nhiều thiết bị (5 test bắt buộc), màn
   Pro/Ủng hộ, huy hiệu PRO-C, vi + en, test.
4. Đo phản hồi thao tác chính (< 100 ms) và chịu tải (nếu có máy chủ), ghi vào PR.
5. Phiếu rà soát bảo mật + THREAT_MODEL. Không tự tạo khoá/tài khoản thanh toán thật ⇒ việc 👤.

## 3. Việc cần chủ dự án (👤)

Xem phiếu gốc mục 8 (giá Pro, ân hạn, nơi đặt dịch vụ đồng bộ dùng chung, ngoại lệ nhắc 30 ngày).

## 4. Kết quả xử lý (phiên nhận điền)

- 20/11/2026 - tsucwico-e6: **P1** (PR `feat/pro1-buoc-1`). Kiểm kê: bản miễn phí đã 0 byte dữ liệu người dùng rời máy
  (chỉ kiểm tra cập nhật GitHub, không định danh; cài đặt ở `localStorage`; sao lưu ở thư mục cục bộ) - không có đường
  đồng bộ miễn phí nào phải tắt. ADR-0002 (Pro = đồng bộ cài đặt qua `tsusync`; đăng nhập tùy chọn loopback PKCE;
  phase P1-P3). Chuẩn v3.9.1. `cwico_core::pro`: claims 7.4 (`iss` `https://tsudev.com/oidc`, `aud` `tsucwico`, trần 7
  ngày), `notice_due` (5 ca bắt buộc + 6 ca phụ), mã cài đặt 128 bit. Hộp thoại `ProNotice` nguyên văn 7.3 vi/en, chỉ
  hỏi lúc khởi động, lệnh `pro_notice` luôn `None` khi chưa có đăng nhập. Không đặt nút mua: `tsudev.com/pro` còn 404.
  Còn: **P2** (chờ client `tsucwico` ở tsudev + tsudev#336 + B3), **P3** (chờ PRO-D3).
- 20/11/2026 - tsucwico-e7: #94 gộp (squash). Điều kiện P2: tsudev#336 ✅ gộp 19/11; client `tsucwico` ❌ (`/oidc/auth`
  ⇒ 400) - `clientMetadata()` của tsudev chỉ dựng client web, redirect khớp tuyệt đối ⇒ loopback cổng ngẫu nhiên (RFC 8252)
  cần sửa mã trước khi INSERT; viết phiếu tsudev `logs/handover/20261120-CW1_yeu-cau-client-oidc-tsucwico.md` + nhắn
  phiên tsudev-f6. `/pro` ❌ 404 (B3 đang làm). P2 chưa bắt đầu.
- 20/11/2026 - tsucwico-e7 (sau): đủ điều kiện P2. tsudev#339 (client native: mọi redirect `http://127.0.0.1/...` /
  `http://[::1]/...` không cổng ⇒ cổng bất kỳ) + 👤 INSERT Neon (`redirectUris` 127.0.0.1 và [::1] `/callback`, scope
  `tsudev.entitlements`). Đo: 127.0.0.1:53682 và [::1]:49152 ⇒ 303; localhost ⇒ 400; `/pro` ⇒ 200. Tiêu chí 3 của
  CW1 (token ⇒ `/api/me/entitlements`, `aud` = `tsucwico`) đo trong P2 khi đăng nhập thật.

## 6. P2 - điểm bắt đầu cho phiên kế tiếp (ghi 20/11/2026, tsucwico-e7)

Phạm vi P2 = ADR-0002 mục 3 (một phiên). Điều kiện tiên quyết đã đạt và đã đo trên production 20/11:

| Thứ | Giá trị đo được |
| --- | --- |
| Discovery | `https://tsudev.com/oidc/.well-known/openid-configuration` - issuer `https://tsudev.com/oidc` |
| Endpoint | `auth` `/oidc/auth`, `token` `/oidc/token`, `jwks` `/oidc/jwks`, `revocation` `/oidc/token/revocation`, `end_session` `/oidc/session/end` |
| PKCE | chỉ `S256`; client công khai `tsucwico` (không secret); `id_token` ES256 |
| Redirect | `http://127.0.0.1:<cổng bất kỳ>/callback` hoặc `http://[::1]:<cổng>/callback` ⇒ 303; `localhost` / path khác ⇒ 400 (tsudev#339, CW1) |
| Scope | `openid tsudev.entitlements` (+ `offline_access` - provider luôn cho, xoay vòng refresh token; dùng lại token đã xoay ⇒ thu hồi cả grant) |
| Quyền lợi | `GET https://tsudev.com/api/me/entitlements`, `Authorization: Bearer <access token>`, `X-Tsu-Installation: <22 ký tự base64url>` (khớp `cwico_core::pro::new_installation_id`; sai dạng ⇒ server lặng lẽ bỏ qua), `X-Tsu-Platform: windows` |
| Trả về | `200 {"jws": "<compact>"}`, `Cache-Control: no-store`; header JWS `alg` ES256, `typ` `tsu-ent+jwt`, `kid` (JWKS 20/11 có 1 khoá P-256 `c77f010f-…`); `aud` = client của token = `tsucwico` |
| Lỗi | 401 `invalid_token` (thiếu/hết hạn - không token ⇒ đã đo 401), 403 `insufficient_scope`, 503 `entitlements_unavailable`, 429 (60 lượt/phút/IP) |
| Trang | `https://tsudev.com/pro` 200 (có `?app=tsucwico`) - đặt nút "Nâng cấp Pro" trỏ tới đây |

Nguồn hợp đồng máy chủ: tsudev `services/auth-service/src/entitlementRoutes.ts`, `entitlements.ts` (`signJws`,
`parseInstallationId`), `oidc.ts`. Đọc lại nếu thấy lệch.

Thứ tự gợi ý trong phiên P2:
1. `cwico_core::pro`: kiểm JWS (RustCrypto `p256` ECDSA P-256 + SHA-256, chữ ký dạng r||s 64 byte), chọn khoá JWKS
   theo `kid`, từ chối `alg` ≠ ES256 / `typ` ≠ `tsu-ent+jwt`, rồi `EntitlementStatus::from_claims` (đã có). Test bằng
   khoá sinh trong test + một JWS thật lấy được ở bước 4 làm fixture (không chứa bí mật).
2. Đăng nhập loopback trong backend Tauri: bind `127.0.0.1:0`, `state` + `nonce` + PKCE S256, mở trình duyệt hệ thống,
   nhận đúng một request `/callback` rồi đóng cổng, hết giờ chờ ⇒ huỷ; đổi code lấy token; kiểm `id_token`.
3. Lưu refresh token bằng DPAPI / Windows Credential Manager (không bao giờ vào `localStorage` hay tệp thường); đăng
   xuất = gọi revocation + xoá cục bộ.
4. Gọi `/api/me/entitlements`, đệm JWS cục bộ, quá `exp` ⇒ miễn phí, không báo lỗi; nối `pro_notice` (hiện luôn `None`)
   với `devices_30d` thật.
5. UI: nút Đăng nhập/Đăng xuất (tùy chọn - không có lời nhắc đăng nhập nào), huy hiệu từ `assets/badges/`, màn Pro với
   điều khoản PRODUCT_POLICY 7.5 trước nút, i18n vi/en.
6. `docs/THREAT_MODEL.md` + phiếu rà soát bảo mật (giả mạo quyền lợi, token bị đánh cắp, cổng loopback bị chiếm);
   đo bằng proxy: chưa đăng nhập ⇒ 0 byte tới tsudev.com.
7. Đo đầu cuối với tài khoản thật của 👤 (trên Windows thật - đăng nhập cần trình duyệt): token ⇒
   `/api/me/entitlements` ⇒ JWS `aud` = `tsucwico` (tiêu chí 3 còn lại của phiếu tsudev CW1).

## 5. Đã chốt (19/11/2026, bổ sung sau khi mở phiếu)

Chủ dự án giao agent chốt mọi câu hỏi mở và xác nhận **chưa có người dùng thật** ⇒ phiếu gốc mục 8 ĐÃ CHỐT, mục 9
là **HỢP ĐỒNG CHUNG** (mọi repo dùng đúng định danh, không tự đặt khác): gói chung **TsuDEV Pro 49.000 ₫/tháng,
490.000 ₫/năm**; Ủng hộ 20/50/100/200 nghìn; Play `tsudev_pro` (`thang`/`nam`) + `ung_ho_20k`…`ung_ho_200k`; JWS
quyền lợi (ES256, `exp` ≤ 7 ngày, `badge`, `devices_30d`); HTTP 402 `pro_required`; mã cài đặt `X-Tsu-Installation`;
dịch vụ đồng bộ dùng chung ở repo mới `tsudev-org/tsusync`; huy hiệu `TsuBadge`; nội dung thông báo vi/en nguyên văn.
Pro hết hạn: chặn đẩy, kéo/xuất 90 ngày, báo trước 30 và 7 ngày, dữ liệu trên thiết bị không bao giờ bị xoá.

Làm được NGAY (không phụ thuộc PRO-B/PRO-D): kiểm kê, ADR, màn Pro/Ủng hộ, thông báo nhiều thiết bị (nguồn quyền lợi giả
trong test), tắt mọi đường gửi dữ liệu người dùng lên mạng ở bản miễn phí. Thứ tự toàn hệ: phiếu gốc mục 9.5.



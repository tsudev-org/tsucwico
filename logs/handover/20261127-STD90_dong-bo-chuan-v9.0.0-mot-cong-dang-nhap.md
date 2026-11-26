# PHIẾU STD90 - Đồng bộ tsudev-standards v9.0.0 (một cổng đăng nhập)

- **Mã phiếu**: 20261127-STD90 - **Từ**: tin nhắn phiên `mot-cong-dang-nhap-cp3`, ghi bởi tsucwico-6da2 - **Trạng thái**: MỞ
- Đã kiểm 27/11/2026: Release `v9.0.0` của `tsudev-org/tsudev-standards` có thật (phát hành 26/11 17:54 UTC); repo này đang `v8.3.0`.

## Việc

- [ ] Chạy sync-standards.sh **hai lượt** lên v9.0.0 (đọc CHANGELOG 9.0.0 trước).
- [ ] Chạy `.standards/scripts/check-central-login.sh`.
- [ ] Lập phiếu `RA_SOAT_DANG_NHAP` theo CHANGELOG 9.0.0. Yêu cầu chính (AUTH_AND_ACCOUNT 2.1): app không ô nhập tài
  khoản, không nút/SDK nhà cung cấp, chỉ mở trình duyệt. Đăng nhập chạm xác thực ⇒ dùng Opus khi phải sửa mã.

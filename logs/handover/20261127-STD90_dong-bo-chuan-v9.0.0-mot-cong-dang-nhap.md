# PHIẾU STD90 - Đồng bộ tsudev-standards v9.0.0 (một cổng đăng nhập)

- **Mã phiếu**: 20261127-STD90 - **Từ**: tin nhắn phiên `mot-cong-dang-nhap-cp3`, ghi bởi tsucwico-6da2 - **Trạng thái**: ĐÓNG (27/11/2026)
- Đã kiểm 27/11/2026: Release `v9.0.0` của `tsudev-org/tsudev-standards` có thật (phát hành 26/11 17:54 UTC); repo này đang `v8.3.0`.

## Việc

- [x] Chạy sync-standards.sh **hai lượt** lên v9.0.0 (đọc CHANGELOG 9.0.0 trước).
- [x] Chạy `.standards/scripts/check-central-login.sh`.
- [x] Lập phiếu `RA_SOAT_DANG_NHAP` theo CHANGELOG 9.0.0. Yêu cầu chính (AUTH_AND_ACCOUNT 2.1): app không ô nhập tài
  khoản, không nút/SDK nhà cung cấp, chỉ mở trình duyệt. Đăng nhập chạm xác thực ⇒ dùng Opus khi phải sửa mã.

## Kết quả 27/11/2026

Đồng bộ v9.0.0 (script cục bộ cũng cập nhật), quét 0 nghi vấn, cổng `--strict` vào CI. Rà tay ra 7 điểm V1-V7: phiếu
`20261127-LOGIN1_ra-soat-mot-cong-dang-nhap.md` (MỞ).

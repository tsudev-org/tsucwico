# PLAN.md - Kế hoạch theo Phase

> Nguồn sự thật cho việc đang làm (AGENT_PROTOCOL 4.2). Một ngữ cảnh làm một Phase; xong Phase thì đánh `[x]`, cập nhật
> `logs/STATE.md`, commit, rồi dừng.

## INT1 - Toàn vẹn ứng dụng: cổng phiên bản (ADR-0003)

Phiếu `logs/handover/20261125-INT1_*.md`, chuẩn `.standards/docs/APP_INTEGRITY.md` v1.0.0. Thiết kế: `docs/adr/0003-*.md`.

## Phase INT1-0 - ADR + kế hoạch

- [x] ADR-0003 (quyết định chủ dự án 27/11/2026: mọi người dùng, cả `latest.json` lẫn bản kê)
- [x] `docs/PLAN.md` chia Phase

Nghiệm thu: ADR + PLAN gộp vào `main`, `check-standards.sh` đạt.

## Phase INT1-1 - Lõi `cwico_core::integrity` (thuần Rust, test được trên Linux)

- [ ] Phụ thuộc `ring` (đã có trong `Cargo.lock`) cho `cwico-core`; bảng khoá ghim `tsu-versions-2026` + test đối chiếu
  JWKS mẫu `data/versions/jwks.json`
- [ ] Kiểm `.sig` + bản kê theo ADR-0003 mục 4 (thứ tự, trần kích thước), parse sau khi chữ ký đạt
- [ ] Quyết định khoá: kênh của mình, so `Release::parse_name`, sản phẩm/kênh vắng, phiên bản tự dựng
- [ ] Chọn kênh từ đường dẫn tệp chạy (`scoop`/`github`), header `X-Tsu-App`
- [ ] Dấu `426` (`server-lock.json`: phiên bản, hết hạn 24 giờ) - đọc/ghi thuần, không mạng
- [ ] Test mục 6 APP_INTEGRITY: chữ ký sai, `kid` lạ, `alg` khác, hết hạn, cửa sổ > 7 ngày, `issued_at` cũ hơn đệm,
  có bản mới ⇒ khoá, kênh khác mới hơn ⇒ không, sản phẩm vắng ⇒ không, bản thử < bản chính, quá kích thước; khoá thử
  sinh từ seed cố định CHỈ trong test; vector vàng = bản kê production thật tải ngày 27/11/2026
- [ ] Gia cố `[profile.release]` (ADR-0003 mục 12)

Nghiệm thu: `cargo test -p cwico-core`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check` đạt; PR gộp.

## Phase INT1-2 - App (Rust): tải, đệm, `X-Tsu-App`, `426`/`403`, Xuất dữ liệu

- [ ] `app/src-tauri/src/integrity.rs`: client riêng (HTTPS, không chuyển hướng, 5 giây, trần kích thước), đệm ghi
  nguyên tử, chạy song song với `update::check`; `UpdateStatus` thêm nguồn khoá (`github`/`manifest`/`server`), `latest`,
  `url`, kênh
- [ ] `account.rs`: `X-Tsu-App` mặc định; `426` ⇒ dấu khoá; `403 untrusted_client` ⇒ trạng thái không chính thức
- [ ] Lệnh `open_update` (không nhận URL; chỉ `url` đã kiểm, host trong danh sách cho phép) + lệnh Scoop
- [ ] Lệnh `export_data`: hộp thoại lưu phía Rust, `.tsubak` (zip) + `manifest.json` SHA-256; test nội dung gói (không
  có token/`account.json`)

Nghiệm thu: build `cwico-app` (mock trên Linux), test lõi + test gói xuất đạt; PR gộp.

## Phase INT1-3 - Giao diện: modal khoá, modal không chính thức, kiểm định kỳ

- [ ] `UpdateGate` nguyên văn 2.2 vi/en, đúng hai nút, `role="dialog"` + `aria-modal` + bẫy focus + chặn Esc; bỏ nút
  ngôn ngữ trong cổng
- [ ] `UnofficialNotice` nguyên văn 2.3 (Tải bản chính thức + Xuất dữ liệu)
- [ ] Kiểm lại mỗi 6 giờ + khi cửa sổ kích hoạt lại sau ≥ 30 phút; chờ RunPanel xong mới hiện
- [ ] Nút Xuất dữ liệu ở Giới thiệu/Cài đặt; `?preview=lock|unofficial` cho ảnh chụp
- [ ] Test UI (nguyên văn, hai nút) + axe-core Sáng/Tối vi/en 0 lỗi

Nghiệm thu: `npm run build`, `npm test`, axe 0 lỗi; PR gộp.

## Phase INT1-4 - CLI + tài liệu

- [ ] CLI đọc bản đệm + dấu `426` (ADR-0003 mục 11): chặn lệnh ghi, mã thoát riêng, test
- [ ] SECURITY.md cam kết 1, `docs/THREAT_MODEL.md` (mục 3 + mối đe doạ mới: giả bản kê, phát lại, khoá oan), ADR-0002
  mục 1, trang riêng tư website, README/CHANGELOG

Nghiệm thu: test CLI đạt, `check-standards.sh` + `check-central-login.sh --strict` đạt; PR gộp.

## Phase INT1-5 - Phát hành: nâng `latest` theo kênh + gia cố dựng

- [ ] `release.yml`: `--remap-path-prefix`
- [ ] `.github/workflows/kenh-phat-hanh.yml` (ADR-0003 mục 13), ghim SHA action, quyền tối thiểu
- [ ] 👤 tạo Environment `versions` + secret `TSU_VERSIONS_TOKEN` (cách làm ngay dưới; làm sớm được, không phụ thuộc mã)

### Cách tạo Environment `versions` + secret `TSU_VERSIONS_TOKEN`

Nguồn giá trị: biến `VERSIONS_RAISE_TOKEN` trong `~/.config/tsudev/keys/tsu-versions-2026.env` (KEY=VALUE, 600). Bẫy:
tên biến nguồn KHÁC tên secret. Không in giá trị ra màn hình, không dán vào chat/tệp git.

**Cách A - dòng lệnh (gõ trong Claude Code với tiền tố `!`, hoặc terminal WSL):**

```bash
R=tsudev-org/tsucwico
# 1. Environment chỉ cho nhánh/thẻ khai báo
gh api -X PUT repos/$R/environments/versions \
  -F 'deployment_branch_policy[protected_branches]=false' \
  -F 'deployment_branch_policy[custom_branch_policies]=true'
# 2. Cho phép nhánh main (workflow_dispatch/schedule) + thẻ v* (phát hành)
gh api -X POST repos/$R/environments/versions/deployment-branch-policies -f name=main -f type=branch
gh api -X POST repos/$R/environments/versions/deployment-branch-policies -f name='v*' -f type=tag
# 3. Secret: nạp biến trong subshell, truyền qua biến, không in
( set -a; . ~/.config/tsudev/keys/tsu-versions-2026.env; set +a
  gh secret set TSU_VERSIONS_TOKEN --env versions --repo $R --body "$VERSIONS_RAISE_TOKEN" )
# 4. Kiểm (chỉ tên + chính sách, không giá trị)
gh api repos/$R/environments/versions/secrets --jq '.secrets[].name'
gh api repos/$R/environments/versions/deployment-branch-policies --jq '.branch_policies[] | "\(.type) \(.name)"'
```

Kỳ vọng bước 4: `TSU_VERSIONS_TOKEN`; `branch main`, `tag v*`.

**Cách B - giao diện web:** github.com/tsudev-org/tsucwico ⇒ **Settings** ⇒ **Environments** ⇒ **New environment**,
tên `versions` ⇒ **Configure environment** ⇒ mục **Deployment branches and tags** chọn **Selected branches and tags** ⇒
**Add deployment branch or tag rule**: Ref type **Branch**, pattern `main`; thêm lần nữa: Ref type **Tag**, pattern `v*`
⇒ mục **Environment secrets** ⇒ **Add environment secret**: Name `TSU_VERSIONS_TOKEN`, Value = giá trị
`VERSIONS_RAISE_TOKEN` (mở tệp bằng trình soạn thảo trên máy, chép phần sau dấu `=`, không kèm dấu nháy/xuống dòng) ⇒
**Add secret**. Không bật "Required reviewers" (workflow chạy theo lịch sẽ treo chờ duyệt). Không đặt secret này ở mức
repo hay Environment khác (tiền lệ tsudisk #90: chỉ ở `versions`).
- [ ] Cập nhật bảng mục 3 phiếu INT1 (hàng tsucwico), DoD APP_INTEGRITY mục 8

Nghiệm thu: workflow chạy `workflow_dispatch` khô (không token ⇒ dừng sạch); thử thật trên Windows (👤): nâng `latest` ⇒
bản cũ hiện modal khoá; Xuất dữ liệu mở được trên máy khác.

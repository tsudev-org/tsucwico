# PHIẾU DD1 - Dọn ổ đĩa toàn máy (WSL Ubuntu + rác Windows) 26/11/2026

- **Mã phiếu**: 20261126-DD1 - **Phiên**: tsucwico-2e (agent-tsucwico), theo yêu cầu chủ dự án 26/11/2026
- **Phối hợp**: phiên `tsudisk-4c` chỉ dọn trong `~/projects/tsudisk` (phiếu `~/projects/tsudisk/logs/handover/20261126-DD1_phoi-hop-don-dia.md`);
  phiên này dọn phần toàn máy, KHÔNG đụng `~/projects/tsudisk`, `~/.nuget`, `~/.dotnet`.
- **Trạng thái**: MỞ - còn bước 👤 mục 3 (co VHDX mới trả dung lượng cho ổ C)

## 1. Số đo

| | Trước | Sau |
| --- | --- | --- |
| Ổ C (Windows) trống | 9,2 GB (98%) | **22 GB** (94%) |
| Ubuntu `df /` đã dùng | 154 GB | 124 GB |
| `ext4.vhdx` (`C:\Users\devng\AppData\Local\wsl\{c3304ba3-...}`) | 186 GB | 186 GB - chỉ co sau mục 3 (dự kiến về ~125 GB, trả thêm ~60 GB cho C) |

## 2. Đã dọn (chỉ thứ tạo lại được hoặc rác của phần mềm đã gỡ)

| Nơi | Dung lượng | Cách / lý do an toàn |
| --- | --- | --- |
| `target/` Rust của `tsulink`, `tsuscan`, `ubnd-app/src-tauri`, `sound-db` | 30,4 GB | `cargo clean` - không phiên nào đang mở các repo này (ListAgents). `cargo sweep --time 7` báo 0 B vì mọi tệp vừa được dùng ⇒ với repo không ai làm, `cargo clean` mới có tác dụng. Giữ `tsucwico/target` (phiên này đang dùng). |
| `~/.npm` (cache) | ~2,6 GB | `npm cache clean --force` |
| `~/.cache/pip` | 0,2 GB | `pip cache purge` |
| `~/.cache/{tmp-cong,lotus-tmp,ubnd-gate-tmp}`, `/tmp` của người dùng | nhỏ | chỉ mục cấp đầu cũ hơn 3 ngày (các thư mục này còn đang được cổng kiểm của dự án khác dùng) |
| Windows `%TEMP%` | 14 GB → 3,4 GB | mục cấp đầu cũ hơn 2 ngày (113 mục; tệp Windows đang khoá tự từ chối); gồm 3 tệp `swap.vhdx` WSL bỏ lại sau các lần khởi động trước (~9 GB), `wsl.3.0.5.0.x64.msi`, cache `WinGet`. **Giữ** `413F030B-.../swap.vhdx` (WSL đang chạy dùng). |
| `C:\Users\devng\.cache\codex-runtimes` | 1,4 GB | Codex không còn cài ở Windows lẫn WSL |
| `C:\Users\devng\.cache\puppeteer` | 0,7 GB | cache trình duyệt (tháng 6), tải lại được |
| `AppData\Local\Programs\Opera GX` | 3,6 MB | rác của phần mềm đã gỡ (chỉ còn prefs + thư mục phiên bản rỗng) |

**Không dọn (cố ý):** toolchain Rust `1.97.1`/`1.99.1` (13 repo ghim trong `rust-toolchain.toml`); `~/.gradle` (Gradle tự dọn,
2 bản wrapper đều đang dùng); `node_modules` các repo (cần lockfile cài lại, không ai yêu cầu); `~/.claude` (lịch sử phiên);
`C:\Windows\Prefetch` (Windows tự quản, xoá chỉ làm khởi động chậm hơn, không lợi dung lượng); journal/apt (cần `sudo`, agent
không có mật khẩu - xem mục 3).

## 3. Việc của chủ dự án 👤 (co VHDX - bắt buộc để ổ C nhận lại dung lượng)

Dọn trong Ubuntu KHÔNG làm nhỏ `ext4.vhdx`: đĩa ảo động chỉ phình, không tự co (AKB `LES-20261126-2217-2a2b`, phiên tsudisk-4c).
Làm **cuối cùng**, sau khi mọi phiên Claude đã đóng (bước 2 tắt WSL):

1. Trong Ubuntu: `sudo apt-get clean && sudo journalctl --vacuum-size=200M && sudo fstrim -av`
2. PowerShell (Windows): `wsl --shutdown` rồi `wsl --manage Ubuntu --set-sparse true`
3. Mở lại Ubuntu, chạy lại `sudo fstrim -av` ⇒ VHDX thưa trả khối trống cho ổ C (lần sau tự trả, không cần làm lại).
   Kiểm: `Get-Item C:\Users\devng\AppData\Local\wsl\*\ext4.vhdx | % Length` giảm về ~125-130 GB.
4. Tuỳ chọn (PowerShell **Admin**): `Dism.exe /Online /Cleanup-Image /StartComponentCleanup` (kho thành phần Windows
   Update) và `cleanmgr /d C` tích "Windows Update Cleanup" (`SoftwareDistribution\Download` hiện 1,2 GB).

## 4. Phát hiện bảo mật trong lúc rà (chỉ xem tên tệp + quyền, không đọc giá trị)

- `/mnt/e/tsudev-sao-luu/ubnd-app-khoa/` - khoá ký updater + mật khẩu **dạng rõ** trên NTFS. Đã có phiếu
  `~/projects/ubnd-app/logs/handover/20261126-XK5_ma-hoa-sao-luu-khoa-ky-o-e.md` (MỞ, việc 👤: `age -p` rồi xoá bản rõ).
  Không tự làm vì là khoá ký phát hành (CLAUDE.md mục 7).
- Các bản sao lưu khác ở E: đã là `.age`. `~/.config/tsudev` quyền 700. Desktop/Downloads/Documents Windows: không có tệp
  tên kiểu bí mật.
- `.env.production` ở `ubnd*`/`tsudev` (644, được git theo dõi): chỉ chứa URL công khai (`VITE_SITE_ORIGIN`,
  `NEXT_PUBLIC_MAIN_URL`...) - không phải bí mật, không cần đổi.

## 5. Dọn định kỳ (an toàn, mọi phiên dùng được)

```bash
cargo sweep -r --time 14 ~/projects          # artefact Rust > 14 ngày không dùng
cargo sweep -r --installed ~/projects        # artefact của toolchain đã gỡ
npm cache clean --force; pip cache purge
# repo không ai làm lâu: (cd <repo> && cargo clean)
# Windows %TEMP% (từ WSL): find /mnt/c/Users/devng/AppData/Local/Temp -mindepth 1 -maxdepth 1 -mtime +2 -exec rm -rf {} +
#   (tệp đang khoá tự bị bỏ qua; KHÔNG xoá swap.vhdx của lần khởi động WSL hiện tại)
```

## 6. Kết quả xử lý

- 26/11/2026 - tsucwico-2e: dọn mục 2, ổ C 9,2 → 22 GB trống. Chờ 👤 mục 3.

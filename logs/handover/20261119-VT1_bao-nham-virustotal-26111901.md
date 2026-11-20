# PHIẾU - Báo nhầm VirusTotal cho Tsu Cwico 26.11.1901

- **Mã phiếu**: 20261119-VT1
- **Từ**: phiên tsucwico-ea, 19/11/2026
- **Trạng thái**: MỞ - SecureAge đã nộp; CrowdStrike + Trapmine chờ 👤 gửi email; Release `v26.11.1901` đang NHÁP;
  đọc lại 19/11 15:27 UTC vẫn 3/71 (mục 4)

## 1. Kết quả quét (job `virustotal` của run 44531891240)

| Tệp | SHA-256 | Kết quả |
| --- | --- | --- |
| `tsucwico_26.11.1901_windows-x64-setup.exe` (NSIS) | `28d7b74c990bdf54d94e4921217f6a8181eb609a65b3dc021f7a43d47add2537` | **3/74**: CrowdStrike 1.0 `win/grayware_confidence_60% (D)`, APEX 6.843 `Malicious`, Trapmine 4.0.17.0 `malicious.moderate.ml.score` |
| `tsucwico_26.11.1901_windows-x64.msi` | `cef57526eb122854ba01e34ff36f1fa3886ab47ed81dcd042323ff6a44fc4eaa` | 0/74 |
| `tsucwico_26.11.1901_windows-x64-cli.exe` | `b56bf4bcf9f94d69debdcfc8bb48bb79f8783030ec6beb03a477440c5f908bf6` | 0/74 |

Cả ba đều là mô hình học máy/heuristic độ tin thấp; Microsoft Defender và mọi engine theo chữ ký sạch. Thẻ VirusTotal:
`overlay`, `detect-debug-environment`, `obfuscated` - đặc trưng của stub NSIS chưa ký, cùng kiểu lotus-share 26.11.1801.
Bản NSIS 26.11.1002 chưa từng lên VirusTotal nên không so được.

## 2. Đã làm

- 19/11 - Yêu cầu VirusTotal phân tích lại (`POST /files/<sha>/analyse`) - đang xếp hàng; đọc lại ở mục 3.2.
- 19/11 - **SecureAge (APEX)**: form `uav.secureage.com/falsepositive`, đính kèm tệp, bật "Email me when verified", email
  liên hệ như tiền lệ lotus-share (chủ dự án đã chọn cho báo nhầm) - trang trả **"Report Submitted"**. Agent gửi bằng Playwright.

## 3. Việc còn lại

1. 👤 Gửi 2 email soạn sẵn (Google chặn đăng nhập trên trình duyệt tự động - mở link bằng trình duyệt thường, bấm Gửi;
   đính kèm tệp nếu hãng hỏi):
   - CrowdStrike `VTscanner@crowdstrike.com`: [soạn sẵn](https://mail.google.com/mail/?view=cm&fs=1&to=VTscanner%40crowdstrike.com&su=False%20positive%3A%20tsucwico_26.11.1901_windows-x64-setup.exe%20%28SHA-256%2028d7b74c...%29&body=Hello%2C%0A%0AWe%20believe%20this%20is%20a%20false%20positive%20on%20an%20open-source%20installer.%0A%0AProduct%3A%20Tsu%20Cwico%2026.11.1901%20-%20open-source%20%28MIT%29%20Windows%20software%20removal%20tool%20by%20TsuDEV%2C%20https%3A%2F%2Fcwico.tsudev.com%0AFile%3A%20tsucwico_26.11.1901_windows-x64-setup.exe%20%28NSIS%20installer%2C%20not%20yet%20Authenticode-signed%29%0ASHA-256%3A%2028d7b74c990bdf54d94e4921217f6a8181eb609a65b3dc021f7a43d47add2537%0ADetection%3A%20CrowdStrike%20Falcon%201.0%20-%20win%2Fgrayware_confidence_60%25%20%28D%29%0AVirusTotal%3A%20https%3A%2F%2Fwww.virustotal.com%2Fgui%2Ffile%2F28d7b74c990bdf54d94e4921217f6a8181eb609a65b3dc021f7a43d47add2537%0A%0ASource%3A%20https%3A%2F%2Fgithub.com%2Ftsudev-org%2Ftsucwico%20-%20built%20by%20GitHub%20Actions%20from%20tag%20v26.11.1901.%20Build%20provenance%20can%20be%20verified%20with%20%22gh%20attestation%20verify%20%3Cfile%3E%20-R%20tsudev-org%2Ftsucwico%22%3B%20SHA256SUMS.txt%20is%20signed%20with%20Sigstore.%20The%20MSI%20and%20the%20CLI%20from%20the%20same%20build%20are%200%2F74%20on%20VirusTotal.%0A%0ADownload%3A%20https%3A%2F%2Fgithub.com%2Ftsudev-org%2Ftsucwico%2Freleases%20%28release%20v26.11.1901%29%0A%0APlease%20review%20and%20whitelist.%20Thank%20you.%0ATsuDEV)
   - Trapmine `fp@trapmine.com`: [soạn sẵn](https://mail.google.com/mail/?view=cm&fs=1&to=fp%40trapmine.com&su=False%20positive%3A%20tsucwico_26.11.1901_windows-x64-setup.exe%20%28SHA-256%2028d7b74c...%29&body=Hello%2C%0A%0AWe%20believe%20this%20is%20a%20false%20positive%20on%20an%20open-source%20installer.%0A%0AProduct%3A%20Tsu%20Cwico%2026.11.1901%20-%20open-source%20%28MIT%29%20Windows%20software%20removal%20tool%20by%20TsuDEV%2C%20https%3A%2F%2Fcwico.tsudev.com%0AFile%3A%20tsucwico_26.11.1901_windows-x64-setup.exe%20%28NSIS%20installer%2C%20not%20yet%20Authenticode-signed%29%0ASHA-256%3A%2028d7b74c990bdf54d94e4921217f6a8181eb609a65b3dc021f7a43d47add2537%0ADetection%3A%20Trapmine%204.0.17.0%20-%20malicious.moderate.ml.score%0AVirusTotal%3A%20https%3A%2F%2Fwww.virustotal.com%2Fgui%2Ffile%2F28d7b74c990bdf54d94e4921217f6a8181eb609a65b3dc021f7a43d47add2537%0A%0ASource%3A%20https%3A%2F%2Fgithub.com%2Ftsudev-org%2Ftsucwico%20-%20built%20by%20GitHub%20Actions%20from%20tag%20v26.11.1901.%20Build%20provenance%20can%20be%20verified%20with%20%22gh%20attestation%20verify%20%3Cfile%3E%20-R%20tsudev-org%2Ftsucwico%22%3B%20SHA256SUMS.txt%20is%20signed%20with%20Sigstore.%20The%20MSI%20and%20the%20CLI%20from%20the%20same%20build%20are%200%2F74%20on%20VirusTotal.%0A%0ADownload%3A%20https%3A%2F%2Fgithub.com%2Ftsudev-org%2Ftsucwico%2Freleases%20%28release%20v26.11.1901%29%0A%0APlease%20review%20and%20whitelist.%20Thank%20you.%0ATsuDEV)
2. Agent (phiên sau, 24-72 giờ): đọc lại VirusTotal (`GET /api/v3/files/<sha>`, khóa ở `~/.config/tsudev/virustotal.env`), ghi
   số engine vào mục 4.
3. 👤 Quyết Publish: giữ nháp tới khi 0 cờ (chuẩn DISTRIBUTION_TRUST mục 3), HOẶC Publish ngoại lệ có ghi như lotus-share
   (chỉ engine ML; MSI - đường updater dùng - và CLI sạch). Dù chọn đường nào vẫn phải qua bước thử nâng cấp trên máy ảo
   (`logs/STATE.md` REL-TSU-1).

## 4. Kết quả xử lý

- 19/11/2026 - phiên tsucwico-ea: mục 1-2.
- 19/11/2026 - phiên tsucwico-c4: đọc `GET /files/<sha>` lúc 14:04 UTC (lượt quét 13:46) - NSIS **3/71** (CrowdStrike
  `win/grayware_confidence_60% (D)`, Trapmine `malicious.moderate.ml.score`, APEX `Malicious` - y như mục 1); MSI 0/59,
  CLI 0/64. Gửi lại `POST /analyse` (200) lúc 14:04; đọc lại 15:27 UTC: lượt 14:04:41 vẫn **3/71**, cùng 3 engine. Chưa
  hãng nào gỡ (mới vài giờ sau báo; SecureAge thường 1-3 ngày). Chưa rõ 👤 đã gửi 2 email mục 3.1 chưa. Phiên sau đọc
  lại sau 20/11. Ghi chú: `~/.config/tsudev/virustotal.env` chứa khóa trần (không dạng `VT_API_KEY=`) - đọc bằng
  `K=$(tr -d '[:space:]' < ~/.config/tsudev/virustotal.env)`, KHÔNG `sed`/`cat` để xem dạng tệp.
- 20/11/2026 - phiên tsucwico-e7: đọc 00:39 UTC - lượt mới nhất vẫn 19/11 14:04:41, NSIS **3/71** cùng 3 engine
  (CrowdStrike, Trapmine, APEX). Gửi `POST /analyse` (200) lúc 00:40 UTC; đọc lại 00:51 lượt mới chưa chạy xong.
  Release vẫn nháp. Phiên sau đọc lại.

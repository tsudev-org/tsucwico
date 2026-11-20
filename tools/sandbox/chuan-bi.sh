#!/usr/bin/env bash
# Dựng bộ kiểm thử nâng cấp trong Windows Sandbox (phiếu logs/handover/20261120-SB1_*).
# Chạy từ WSL ở gốc repo:  tools/sandbox/chuan-bi.sh [thẻ-cũ] [thẻ-mới]
# Tạo %USERPROFILE%\tsucwico-sandbox\ gồm pkg\ (4 bộ cài, SHA-256 đã đối chiếu SHA256SUMS.txt của Release),
# kiem-thu.ps1, kiem-thu-msi.wsb, kiem-thu-nsis.wsb. Bấm đúp tệp .wsb trên Windows để chạy.
set -euo pipefail
CU=${1:-tsudev-cwico-v26.11.10.2}
MOI=${2:-v26.11.1901}
REPO=tsudev-org/tsucwico
WINPROFILE=$(cmd.exe /c 'echo %USERPROFILE%' 2>/dev/null | tr -d '\r')
KIT_WIN="$WINPROFILE\\tsucwico-sandbox"
KIT=$(wslpath "$KIT_WIN")
GOC=$(cd "$(dirname "$0")" && pwd)

mkdir -p "$KIT/pkg" "$KIT/ketqua"
rm -f "$KIT"/pkg/*
cd "$KIT/pkg"
gh release download "$CU" -R "$REPO" -p '*x64-setup.exe' -p '*.msi' -p SHA256SUMS.txt
mv SHA256SUMS.txt cu.sums
gh release download "$MOI" -R "$REPO" -p '*windows-x64-setup.exe' -p '*windows-x64.msi' -p SHA256SUMS.txt
mv SHA256SUMS.txt moi.sums
sha256sum -c --ignore-missing cu.sums
sha256sum -c --ignore-missing moi.sums
rm -f cu.sums moi.sums
ls "$KIT/pkg" | grep -c -E '\.(exe|msi)$' | grep -qx 4 || { echo "thiếu bộ cài trong pkg/"; exit 1; }

cp "$GOC/kiem-thu.ps1" "$KIT/kiem-thu.ps1"
for d in msi nsis; do
  cat > "$KIT/kiem-thu-$d.wsb" <<WSB
<Configuration>
  <MappedFolders>
    <MappedFolder>
      <HostFolder>$KIT_WIN</HostFolder>
      <SandboxFolder>C:\\kit</SandboxFolder>
      <ReadOnly>false</ReadOnly>
    </MappedFolder>
  </MappedFolders>
  <MemoryInMB>3072</MemoryInMB>
  <LogonCommand>
    <Command>powershell.exe -NoProfile -ExecutionPolicy Bypass -File C:\\kit\\kiem-thu.ps1 -Duong $d</Command>
  </LogonCommand>
</Configuration>
WSB
done
echo "Xong: $KIT_WIN"

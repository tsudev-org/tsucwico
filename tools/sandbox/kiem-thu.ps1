# Kiem thu nang cap ban tsudev-cwico (26.11.1002) -> ban Tsu Cwico moi trong Windows Sandbox.
# Mo phong dung lenh updater Tauri 2 (installMode passive) chay sau khi tai + kiem chu ky:
#   MSI : msiexec /i <msi> /passive /promptrestart
#   NSIS: <setup.exe> /P /UPDATE
# Tieu chi: docs/RELEASING.md muc 5 + ADR-0001 muc 3 (repo tsucwico).
param([Parameter(Mandatory)][ValidateSet('msi', 'nsis')][string]$Duong)

$ErrorActionPreference = 'Continue'
$kit = 'C:\kit'; $pkg = "$kit\pkg"; $out = "$kit\ketqua\$Duong"
Remove-Item -Recurse -Force $out -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force $out | Out-Null
Start-Transcript "$out\nhat-ky.txt" -Force | Out-Null

$kq = [System.Collections.Generic.List[string]]::new()
function Ghi([bool]$ok, [string]$ten) { $kq.Add(("{0}  {1}" -f ($(if ($ok) { 'DAT  ' } else { 'TRUOT' })), $ten)) }

function Arp {
    $keys = 'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*',
            'HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*',
            'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*'
    @(Get-ItemProperty $keys -ErrorAction SilentlyContinue |
        Where-Object { $_.DisplayName -match 'cwico' } |
        Select-Object DisplayName, DisplayVersion, InstallLocation, PSChildName)
}

function Chay([string]$file, [string]$argv) {
    Write-Host ">> $file $argv"
    $p = Start-Process -FilePath $file -ArgumentList $argv -Wait -PassThru
    Write-Host "   exit $($p.ExitCode)"
    $p.ExitCode
}

function TimExe([string]$thuMuc) {
    if (-not (Test-Path $thuMuc)) { return $null }
    Get-ChildItem $thuMuc -Filter *.exe -File |
        Where-Object { $_.Name -notmatch '^(uninstall|unins)' } | Select-Object -First 1
}

function ThuMo([string]$exe, [string]$nhan) {
    $p = Start-Process -FilePath $exe -PassThru
    Start-Sleep -Seconds 12
    $song = -not $p.HasExited
    Ghi $song "$nhan mo duoc va con chay sau 12 giay ($exe)"
    if ($song) { Stop-Process -Id $p.Id -Force; Start-Sleep -Seconds 2 }
}

$pf = $env:ProgramFiles
# Ban cu: ten tsudev-cwico_*; ban moi: tsucwico_* (chuan-bi.sh tai dung 4 tep).
function Tep([string]$mau) { (Get-ChildItem $pkg -Filter $mau | Select-Object -First 1).FullName }
$cuMsi = Tep 'tsudev-cwico_*.msi'
$cuNsis = Tep 'tsudev-cwico_*-setup.exe'
$moiMsi = Tep 'tsucwico_*.msi'
$moiNsis = Tep 'tsucwico_*-setup.exe'
$moiVer = ([regex]'tsucwico_([\d.]+)_').Match((Split-Path $moiMsi -Leaf)).Groups[1].Value

# 1. Cai ban cu 26.11.1002
if ($Duong -eq 'msi') { $e = Chay 'msiexec.exe' "/i `"$cuMsi`" /qn /norestart /l*v `"$out\cai-ban-cu.log`"" }
else { $e = Chay $cuNsis '/S' }
Ghi ($e -eq 0) "Cai 26.11.1002 bang $Duong (exit $e)"

$arpTruoc = Arp
$arpTruoc | Format-List | Out-File "$out\arp-truoc.txt" -Encoding utf8
$exeCu = TimExe "$pf\tsudev-cwico"
Ghi ($null -ne $exeCu) "Ban cu nam o $pf\tsudev-cwico"
if ($exeCu) { ThuMo $exeCu.FullName 'Ban cu 26.11.1002' }

# 2. Dat dau du lieu nguoi dung (phai con nguyen sau nang cap)
$dau = @("$env:APPDATA\com.tsudev.cwico\dau-kiem-thu.txt",
         "$env:LOCALAPPDATA\com.tsudev.cwico\dau-kiem-thu.txt",
         "$env:LOCALAPPDATA\tsudev-cwico\backups\dau-kiem-thu.txt")
foreach ($f in $dau) { New-Item -ItemType File -Force $f -Value 'tsucwico-e7' | Out-Null }

# 3. Nang cap dung nhu updater
if ($Duong -eq 'msi') { $e = Chay 'msiexec.exe' "/i `"$moiMsi`" /passive /promptrestart /l*v `"$out\nang-cap.log`"" }
else { $e = Chay $moiNsis '/P /UPDATE' }
Ghi ($e -eq 0) "Nang cap len $moiVer bang lenh cua updater (exit $e)"
Start-Sleep -Seconds 3

# 4. Kiem ket qua
$arpSau = Arp
$arpSau | Format-List | Out-File "$out\arp-sau.txt" -Encoding utf8
Ghi ($arpSau.Count -eq 1) "Add/Remove Programs chi con 1 muc cwico (dang co $($arpSau.Count))"
Ghi (@($arpSau | Where-Object DisplayName -eq 'Tsu Cwico').Count -eq 1) 'Muc do ten "Tsu Cwico"'
Ghi (@($arpSau | Where-Object DisplayName -match 'tsudev-cwico').Count -eq 0) 'Khong con muc "tsudev-cwico"'
Ghi (@($arpSau | Where-Object DisplayVersion -like "$moiVer*").Count -eq 1) "Phien ban $moiVer ($(($arpSau.DisplayVersion) -join ', '))"
Ghi ($null -eq (TimExe "$pf\tsudev-cwico")) "Khong con exe trong $pf\tsudev-cwico"
$exeMoi = TimExe "$pf\Tsu Cwico"
Ghi ($null -ne $exeMoi) "Ban moi nam o $pf\Tsu Cwico"
foreach ($f in $dau) { Ghi (Test-Path $f) "Du lieu giu nguyen: $f" }
if ($exeMoi) {
    $v = $exeMoi.VersionInfo
    Ghi ($v.FileVersion -like "$moiVer*" -or $v.ProductVersion -like "$moiVer*") "exe moi: FileVersion $($v.FileVersion) / ProductVersion $($v.ProductVersion)"
    ThuMo $exeMoi.FullName "Ban moi $moiVer"
}

$truot = @($kq | Where-Object { $_ -like 'TRUOT*' }).Count
$tong = if ($truot -eq 0) { "TONG KET [$Duong]: DAT tat ca $($kq.Count) muc" } else { "TONG KET [$Duong]: TRUOT $truot / $($kq.Count) muc" }
(@($tong, '') + $kq) | Out-File "$out\KET-QUA.txt" -Encoding utf8
Stop-Transcript | Out-Null
Start-Process notepad.exe "$out\KET-QUA.txt"

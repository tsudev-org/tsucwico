# Changelog

All notable changes to this project are documented here, following
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## Versioning

From 19 November 2026 releases follow tsudev-standards `VERSIONING.md` 2.2:
version `YY.M.DDNN`, tag `v` + version, title `Tsu Cwico {version}`.

| Release | Tag | semver |
|---|---|---|
| first on 19 Nov 2026 | `v26.11.1901` | `26.11.1901` |
| second that day | `v26.11.1902` | `26.11.1902` |
| 3 Dec 2026 | `v26.12.0301` | `26.12.301` |

Older releases were tagged `tsudev-cwico-v26.8.19` (= `26.8.1901`) and
`tsudev-cwico-v26.11.10.2` (= `26.11.1002`); those tags never change.
`tools/version.py` is the only thing that should ever compute a version;
`tools/test_version.py` checks the mapping still sorts in release order.

## Unreleased

### Added

- TsuDEV Pro engine pieces (PRO1 P2, PRO2): the entitlement token from
  tsudev.com is checked offline against its published ES256 key (anything
  but ES256 / `tsu-ent+jwt`, an unknown key or a tampered body is refused),
  and the renewal rules of PRODUCT_POLICY 7.6 - a reminder at every app open
  in the last 7 days of the 1-year term, an "expired" notice exactly once.

- About shows "Your data stays on this PC": no account, nothing sent to the
  internet or the cloud; the only connection is the update check (ADR-0002).
- Groundwork for TsuDEV Pro (settings sync across PCs, later, with optional
  sign-in): entitlement rules and the "several devices, no Pro" notice from
  tsudev-standards PRODUCT_POLICY 7.3, in Vietnamese and English. It cannot
  appear yet - there is no sign-in.

### Changed

- The Support button opens `tsudev.com/ung-ho?app=tsucwico`.
- tsudev-standards v3.9.1.

## v26.11.1901 - 2026-11-19

> **Tên mới: Tsu Cwico.** Phần mềm đổi tên từ tsudev-cwico thành Tsu Cwico,
> có logo riêng, giao diện xanh TsuDEV, và lần chạy đầu theo ngôn ngữ + chế
> độ sáng/tối của Windows. Cập nhật thay thế bản cũ tại chỗ: Add/Remove
> Programs chỉ còn một mục "Tsu Cwico", dữ liệu và nhật ký giữ nguyên. Việc
> phần mềm được hay không được gỡ không thay đổi.
>
> **New name: Tsu Cwico.** tsudev-cwico is now Tsu Cwico, with its own logo,
> the TsuDEV blue look, and a first run that follows Windows' language and
> light/dark mode. The update replaces the old build in place: one "Tsu Cwico"
> entry in Add/Remove Programs, your data and logs kept. Nothing changed about
> what the tool will or will not remove.

From this release tags follow tsudev-standards `VERSIONING.md` 2.2:
`v26.11.1901`, title `Tsu Cwico 26.11.1901`, files
`tsucwico_26.11.1901_windows-x64-setup.exe` / `_windows-x64.msi` /
`_windows-x64-cli.exe` (before: `tsudev-cwico-v26.11.10.2`,
`tsudev-cwico_26.11.1002_x64-setup.exe`). Installed 26.8.1901 and 26.11.1002
builds see this release as newer - tested with the `semver` crate their
updater uses.

### Added

- Language picker with **Match system** (Theo hệ thống) next to Tiếng Việt
  and English. A first run follows Windows' language (vi, en; anything else
  falls back to English) and keeps following it; picking a language stores
  it. The theme picker is Light / Dark / **System**, and System is the
  default for a first run.
- Own product logo (app icon, installer, website) and the display name
  **TsuDEV** throughout.
- Releases carry `SHA256SUMS.txt` signed keyless with Sigstore
  (`SHA256SUMS.txt.sigstore.json`) and with the updater key, an SPDX SBOM,
  build provenance for every file (`gh attestation verify`), and a VirusTotal
  scan of the draft. The command-line `cwico.exe` carries Windows version
  information (CompanyName TsuDEV).
- Scoop: `scoop install tsudev/tsucwico` (command-line build).

### Changed

- Renamed Tsu Cwico (repository `tsudev-org/tsucwico`, docs/adr/0001). The
  MSI keeps the old UpgradeCode and the NSIS setup removes a tsudev-cwico
  install silently first, so either installer upgrades in place. The winget
  identifier `tsudev.cwico`, the command `cwico` and the updater key are
  unchanged.
- TsuDEV look: black and the logo's blue replace the earlier violet;
  primary button, progress bar, checkboxes and the selected row use the
  `primary` token. The About screen and the update dialog show the version as
  `26.11.1901`.
- "Support the project" points to `tsudev.com/ung-ho`.

### Fixed

- `cwico plan` now lists the run preamble - the System Restore point and the
  registry backup taken before anything is removed. The plan was built as a
  dry run, which drops the preamble, so the person reviewing it never saw the
  two safety steps `remove --apply` performs first.

### Security

- rustls-webpki 0.103.17 and webpki-root-certs 1.0.12 (RUSTSEC-2026-0382,
  RUSTSEC-2026-0383), used by the update check's HTTPS client.

## tsudev-cwico-v26.11.10.2 - 2026-11-10

> **Bản sửa lỗi quan trọng.** Ở bản 26.8.19, bấm *Quét hệ thống* trên máy
> thật làm cả cửa sổ trắng trơn. Bản này sửa lỗi đó, thêm phân trang cho danh
> sách hàng trăm mục, và không thay đổi việc phần mềm được hay không được gỡ.
>
> **A critical fix.** In 26.8.19, pressing *Scan* on a real machine blanked
> the whole window. This release fixes it, pages long lists, and changes
> nothing about what the tool will or will not remove.

The first build of the day, `tsudev-cwico-v26.11.10`, stopped in the release
pipeline before producing anything (see the last Fixed item below) and was
never published. Its tag remains; this is the same change set.

### Added

- A standalone command-line build, `tsudev-cwico_<version>_x64-cli.exe`, is
  attached to each release - no Rust toolchain needed. See
  [cwico.tsudev.com/dong-lenh](https://cwico.tsudev.com/dong-lenh/).
- Product site [cwico.tsudev.com](https://cwico.tsudev.com): downloads with
  SHA-256, and a searchable catalogue of every safety rule and tweak.

- The installers carry the tsudev mark: NSIS header and sidebar, MSI banner
  and welcome dialog, generated by `tools/gen_installer_images.py` from the
  official mark. The standards gate now fails a build that is missing an
  icon or installer image, or whose `tauri.conf.json` points at one that does
  not exist.
- Every record list - the scan results, the tweak catalogue, the items in the
  removal plan - pages at 10 / 20 / 50 / 100 / 200 rows (default 10, picker
  bottom-left, remembered per list). A real scan returns hundreds of items.
- Search in the software list ignores Vietnamese diacritics: `dich vu` finds
  `Dịch vụ`.

### Changed

- Safety database 1.0.2: the 17 rules that had no display name (Skype,
  Microsoft Edge, Photos, Cortana, Copilot and others) now carry one in both
  languages, so the app groups them under a readable name. No rule changed
  class or what it matches.
- Colours come from the tsudev design tokens. Muted text was 3.6:1 on white,
  below WCAG AA; interactive edges now use the 3:1 `border-control`; safety
  badges and status colours use the status tokens. An automated WCAG 2.2 AA
  check of the scan screen went from 783 contrast failures to 2 (light) and
  951 to 0 (dark).

### Fixed

- **The window went blank after the first scan of a real machine.** Empty
  `extra` and `executables` were left out of the JSON the engine sends the UI,
  which types them as required; one item without them unmounted the whole
  interface. The same gap in `RunReport.warnings` would have blanked the run
  panel after every clean removal. Both are now always sent, and
  `tests/ipc_contract.rs` pins the contract. Found by running 26.8.1901 on
  Windows 11 Pro 25H2 - the first live run of this tool.
- A render error now shows a bilingual error screen with the details and a
  Reload button instead of an empty window.
- Dates read `HH:mm DD/MM/YYYY` / `DD/MM/YYYY` in both languages, as the
  tsudev design system requires, instead of `9 thg 11, 2026` and ISO dates;
  the About panel's Release and Architecture labels are translated.
- Window title, installer copyright and long description no longer contain
  escaped em-dashes; copyright reads `Copyright (c) 2026 tsudev`.
- The release build passed `-Version` to the winget manifest generator, which
  takes `-Tag`; every release run would have failed at that step.
- The release build read `CHANGELOG.md` as cp1252 on the Windows runner and
  died on Vietnamese text; the job now runs Python in UTF-8 mode.

## tsudev-cwico-v26.8.19 - 2026-08-19

> **Read this before installing.** This build compiles, passes 136 tests and
> packages cleanly on a Windows CI runner, but it has never been run on a live
> Windows machine. Every Win32 and WinRT call is exercised only by unit tests
> of its pure logic - no code path has yet touched a real registry, service
> control manager or task scheduler.
>
> Start with the commands that change nothing:
>
> ```
> cwico info                    # what it detects about your machine
> cwico scan                    # the inventory, classified
> cwico plan --safe-only        # what removal would do - a dry run
> ```
>
> The desktop app's **Dry run** button does the same thing. Only `--apply`,
> or the Remove button after the plan dialog, changes anything - and even then
> a System Restore Point is taken first, or the run is cancelled.
>
> The installer is unsigned, so Windows SmartScreen will warn on first run.

First release. The project was previously a single 640-line PowerShell script
(`legacy/Optimize_Win11_For_Dev.ps1`) that applied twelve fixed groups of
changes with no inspection, no selection and no way back.

### Added - scanning

* Registry uninstall keys across `HKLM`, `HKCU` and **both** the 64-bit and
  `WOW6432Node` views, reading `DisplayName`, `DisplayVersion`, `Publisher`,
  `UninstallString`, `QuietUninstallString`, `InstallLocation`,
  `EstimatedSize` and `InstallDate`.
* UWP/MSIX packages via the WinRT `PackageManager`, including framework and
  system-signature flags.
* Provisioned packages - the ones that reinstall themselves for every new user
  account, which is why removed bloatware "comes back".
* Windows services via `EnumServicesStatusExW` / `QueryServiceConfigW`.
* Scheduled tasks via the `ITaskService` COM interfaces.
* Autostart entries: `Run` and `RunOnce` in both hives and views, plus the
  per-user and all-users Startup folders, with detection of entries pointing
  at programs that no longer exist.

### Added - safety

* **Safety database** (`data/safety-db.json`): 58 rules classifying software
  as Safe (29), Caution (11) or Critical (18), each with a bilingual
  explanation. Unmatched items are `Unknown`, never `Safe`.
* **Hard block on Critical items.** `RemovalPlan::build` refuses to plan them;
  no flag, confirmation or CLI switch overrides it, and the engine re-asserts
  the invariant before executing.
* **Per-item confirmation** for Caution and Unknown items. Bulk selection
  cannot supply it.
* **Deletion guard** (`cwico_core::guard`): validates every filesystem path
  and registry key immediately before deletion. Rejects drive roots, system
  directories, parents of protected directories, user profile roots and
  personal folders, shared containers, every registry hive root, the service
  control database, and any path with an unexpanded `%VARIABLE%`, a `..`
  traversal or a wildcard.
* **Process deny-list**: shared hosts (`svchost.exe`), boot-critical
  processes and Defender are never terminated.

### Added - rollback

* System Restore Point via `SRSetRestorePointW` before the first destructive
  step. If it cannot be created the run is cancelled rather than proceeding
  unprotected, and Windows' 24-hour throttle is detected rather than mistaken
  for success.
* `.reg` export of every registry key a run will touch, in both views, plus a
  generated `restore-registry.cmd` that re-imports them without this tool.
* JSON transaction log per run.

### Added - removal

Four-step flow: terminate the software's processes, run the vendor's own
uninstaller silently, remove and deprovision the package, then sweep residue.
A failed step stops that item without stopping the run, and deep clean never
follows a failed uninstall.

### Added - interfaces

* Desktop application (Tauri v2, React 19, TypeScript, Tailwind 4) in
  Vietnamese and English, with light and dark themes.
* Headless CLI (`cwico`) with JSON output for auditing and scripting.
  Everything is a dry run until `--apply`.
* System tweaks catalogue (`data/tweaks.json`): 36 individually selectable
  changes ported from the legacy script, each with a safety class and - where
  possible - a revert path.

### Packaging

* MSI and NSIS installers via the Tauri bundler.
* MSIX manifest and Microsoft Store submission notes.
* `winget` manifests in English and Vietnamese.
* GitHub Actions for CI (engine tests on Linux, Windows type-checking from
  Linux, front-end build, Windows build) and releases.

### Notes

Behaviour deliberately **not** carried over from the legacy script:
`$ErrorActionPreference = "SilentlyContinue"` swallowing every failure;
unconditional removal of Edge, Media Player and Cortana; silently disabling
Windows Search, SysMain and the Program Compatibility Assistant; deleting the
`TEMP` directory rather than its contents; and restarting Explorer without
asking. See [legacy/README.md](legacy/README.md).

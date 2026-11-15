; Tsu Cwico - NSIS installer hooks (docs/adr/0001-ten-san-pham-tsu-cwico.md).
;
; Up to 26.11.1002 the product was called "tsudev-cwico". Tauri keys its NSIS
; installer on productName - uninstall entry, install folder, Start menu
; shortcut - so the first "Tsu Cwico" setup does not see the old install and
; would put a second copy beside it. That includes the mandatory in-app
; update, which runs this setup with /P /UPDATE.
;
; Before installing, remove a leftover "tsudev-cwico" NSIS install by running
; its own uninstaller silently with /UPDATE: that flag keeps the user's data
; and autostart entry (the uninstaller only deletes app data when it is NOT
; updating), exactly as a normal version-to-version update does. Backups in
; %LOCALAPPDATA%\tsudev-cwico\backups are never touched by the uninstaller.
;
; An MSI install of the old name is handled by MSI itself: tauri.conf.json pins
; the old UpgradeCode, so the new MSI performs a normal major upgrade.

!macro NSIS_HOOK_PREINSTALL
  Push $0
  Push $1
  Push $2
  ReadRegStr $0 SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\tsudev-cwico" "UninstallString"
  ReadRegStr $1 SHCTX "Software\tsudev\tsudev-cwico" ""
  ${If} $0 != ""
  ${AndIf} $1 != ""
    DetailPrint "Removing the previous install (tsudev-cwico) from $1"
    ; `_?=` runs the uninstaller in place and must come last, unquoted.
    ExecWait '$0 /S /UPDATE _?=$1' $2
    ${If} $2 = 0
      Delete "$1\uninstall.exe"
      RMDir "$1"
      ; The uninstaller leaves these behind in update mode; nothing reads them
      ; once the old product is gone.
      DeleteRegKey SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\tsudev-cwico"
      DeleteRegKey SHCTX "Software\tsudev\tsudev-cwico"
    ${Else}
      DetailPrint "The previous uninstaller returned $2; leaving it in place"
    ${EndIf}
  ${EndIf}
  Pop $2
  Pop $1
  Pop $0
!macroend

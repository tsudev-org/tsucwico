//! Embeds the Windows version resource into cwico.exe.
//!
//! The desktop app gets the same from tauri-build (`bundle.publisher`,
//! `bundle.copyright` in tauri.conf.json); the CLI is a plain binary and had
//! none, so its Properties > Details tab was empty - one of the signals
//! antivirus heuristics weigh (DISTRIBUTION_TRUST.md 3).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    #[cfg(windows)]
    {
        // Only when targeting Windows (cross builds from Windows to other
        // targets have nothing to embed into).
        if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
            let mut res = tauri_winres::WindowsResource::new();
            res.set_icon("../../app/src-tauri/icons/icon.ico")
                .set("CompanyName", "TsuDEV")
                .set("ProductName", "Tsu Cwico")
                .set("FileDescription", "Tsu Cwico - command-line interface")
                .set("OriginalFilename", "cwico.exe")
                .set("InternalName", "cwico")
                .set("LegalCopyright", "Copyright (c) 2026 TsuDEV");
            res.compile()
                .expect("could not embed the Windows version resource");
        }
    }
}

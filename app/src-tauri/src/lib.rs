//! The Tauri shell: a thin IPC layer over `cwico-core`.
//!
//! Deliberately thin. No safety decision is made here - the engine refuses
//! `Critical` items and the guard vets every delete target, so a bug in the
//! UI cannot talk the backend into doing something dangerous. What this layer
//! owns is: which backend to use, where backups go, keeping the last scan so
//! the front end can build a plan against it, and streaming progress.

use cwico_core::backend::{Event, EventSink, PlatformBackend, PlatformInfo};
use cwico_core::{
    Engine, PlanOptions, RemovalPlan, RunReport, SafetyDatabase, ScanOptions, ScanReport,
    Selection, TweakCatalog,
};
pub mod account;
pub mod update;

use serde::Serialize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, State};

/// The channel progress events are emitted on.
pub const PROGRESS_EVENT: &str = "cwico://progress";

/// A serialisable error for the front end.
///
/// Carries a stable `code` so the UI can translate the message rather than
/// showing an English string to a Vietnamese user.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

impl From<cwico_core::Error> for ApiError {
    fn from(error: cwico_core::Error) -> Self {
        Self {
            code: error.code().to_string(),
            message: error.to_string(),
        }
    }
}

impl ApiError {
    fn other(message: impl Into<String>) -> Self {
        Self {
            code: "other".into(),
            message: message.into(),
        }
    }
}

type ApiResult<T> = Result<T, ApiError>;

/// Forwards engine progress to the web view.
struct WindowSink {
    app: AppHandle,
}

impl EventSink for WindowSink {
    fn emit(&self, event: Event) {
        // A failed emit means the window is closing. Losing a progress event
        // then is fine; the run itself keeps going and is reported at the end.
        if let Err(e) = self.app.emit(PROGRESS_EVENT, &event) {
            tracing::debug!(error = %e, "could not deliver a progress event");
        }
    }
}

pub struct AppState {
    backend: Arc<dyn PlatformBackend>,
    db: Arc<SafetyDatabase>,
    /// The last scan, so `build_plan` can resolve item ids without the front
    /// end having to send the whole inventory back.
    last_scan: Arc<Mutex<Option<ScanReport>>>,
    backup_dir: PathBuf,
}

impl AppState {
    fn new(backend: Arc<dyn PlatformBackend>, backup_dir: PathBuf) -> Self {
        Self {
            backend,
            db: Arc::new(SafetyDatabase::builtin()),
            last_scan: Arc::new(Mutex::new(None)),
            backup_dir,
        }
    }
}

/// Where `.reg` backups and transaction logs are written.
fn default_backup_dir() -> PathBuf {
    std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir())
        .join("tsudev-cwico")
        .join("backups")
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Summary shown in the "About" panel: how much protection is actually loaded.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutInfo {
    /// The raw semver, for support reports.
    pub app_version: String,
    /// The version users see (VERSIONING.md 2.5): `26.11.1901`.
    pub app_release: String,
    pub product_url: String,
    pub platform: PlatformInfo,
    pub safety_db_version: String,
    pub safety_db_updated: String,
    pub safety_rules: usize,
    pub safe_rules: usize,
    pub caution_rules: usize,
    pub critical_rules: usize,
    pub backup_dir: String,
}

#[tauri::command]
fn about(state: State<'_, AppState>) -> AboutInfo {
    let (safe, caution, critical) = state.db.class_counts();
    AboutInfo {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        app_release: cwico_core::current_release_name(),
        product_url: cwico_core::PRODUCT_URL.to_string(),
        platform: state.backend.platform_info(),
        safety_db_version: state.db.version().to_string(),
        safety_db_updated: state.db.updated().to_string(),
        safety_rules: state.db.rules().len(),
        safe_rules: safe,
        caution_rules: caution,
        critical_rules: critical,
        backup_dir: state.backup_dir.display().to_string(),
    }
}

#[tauri::command]
async fn scan(
    app: AppHandle,
    state: State<'_, AppState>,
    options: Option<ScanOptions>,
) -> ApiResult<ScanReport> {
    let backend = Arc::clone(&state.backend);
    let db = Arc::clone(&state.db);
    let last_scan = Arc::clone(&state.last_scan);
    let options = options.unwrap_or_default();

    // A full scan takes seconds and blocks; keep it off the UI thread.
    let report = tauri::async_runtime::spawn_blocking(move || {
        let sink = WindowSink { app };
        let engine = Engine::new(backend.as_ref(), &db);
        let report = engine.scan(&options, &sink)?;
        *last_scan.lock().expect("scan mutex") = Some(report.clone());
        Ok::<_, cwico_core::Error>(report)
    })
    .await
    .map_err(|e| ApiError::other(format!("the scan task did not finish: {e}")))??;

    Ok(report)
}

/// Build a plan from the current selection, without executing it.
///
/// The front end calls this on every selection change so the confirmation
/// panel can show exactly which items were refused and why.
#[tauri::command]
fn build_plan(
    state: State<'_, AppState>,
    selections: Vec<Selection>,
    options: Option<PlanOptions>,
) -> ApiResult<RemovalPlan> {
    let guard = state.last_scan.lock().expect("scan mutex");
    let report = guard
        .as_ref()
        .ok_or_else(|| ApiError::other("no scan has been run yet"))?;

    let mut options = options.unwrap_or_default();
    if options.backup_dir.is_none() {
        options.backup_dir = Some(state.backup_dir.clone());
    }
    Ok(RemovalPlan::build(report, &selections, options))
}

#[tauri::command]
async fn execute_plan(
    app: AppHandle,
    state: State<'_, AppState>,
    plan: RemovalPlan,
) -> ApiResult<RunReport> {
    let backend = Arc::clone(&state.backend);
    let db = Arc::clone(&state.db);

    let report = tauri::async_runtime::spawn_blocking(move || {
        let sink = WindowSink { app };
        let engine = Engine::new(backend.as_ref(), &db);
        engine.execute(&plan, &sink)
    })
    .await
    .map_err(|e| ApiError::other(format!("the removal task did not finish: {e}")))??;

    Ok(report)
}

#[tauri::command]
fn tweak_catalog() -> TweakCatalog {
    TweakCatalog::builtin()
}

/// Apply or revert tweaks by id.
#[tauri::command]
async fn apply_tweaks(
    _app: AppHandle,
    state: State<'_, AppState>,
    ids: Vec<String>,
    enable: bool,
    dry_run: bool,
) -> ApiResult<Vec<TweakOutcome>> {
    let elevated = state.backend.is_elevated();
    if !dry_run && !elevated {
        return Err(ApiError {
            code: "needs_elevation".into(),
            message: "changing system settings requires running as Administrator".into(),
        });
    }

    tauri::async_runtime::spawn_blocking(move || run_tweaks(ids, enable, dry_run))
        .await
        .map_err(|e| ApiError::other(format!("the tweak task did not finish: {e}")))?
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakOutcome {
    pub id: String,
    pub ok: bool,
    pub detail: String,
}

#[cfg(windows)]
fn run_tweaks(ids: Vec<String>, enable: bool, dry_run: bool) -> ApiResult<Vec<TweakOutcome>> {
    let catalog = TweakCatalog::builtin();
    let mut out = Vec::with_capacity(ids.len());

    for id in ids {
        let Some(tweak) = catalog.get(&id) else {
            out.push(TweakOutcome {
                id,
                ok: false,
                detail: "no such tweak".into(),
            });
            continue;
        };
        match cwico_win::tweak_apply::apply(tweak, enable, dry_run) {
            Ok(results) => out.push(TweakOutcome {
                id,
                ok: true,
                detail: results
                    .iter()
                    .map(|r| r.detail.as_str())
                    .collect::<Vec<_>>()
                    .join("; "),
            }),
            Err(e) => out.push(TweakOutcome {
                id,
                ok: false,
                detail: e.to_string(),
            }),
        }
    }
    Ok(out)
}

#[cfg(not(windows))]
fn run_tweaks(ids: Vec<String>, enable: bool, _dry_run: bool) -> ApiResult<Vec<TweakOutcome>> {
    // The fixture build reports what it would do so the UI can be exercised.
    Ok(ids
        .into_iter()
        .map(|id| TweakOutcome {
            id: id.clone(),
            ok: true,
            detail: format!(
                "simulated {} of `{id}` (this build has no Windows backend)",
                if enable { "apply" } else { "revert" }
            ),
        })
        .collect())
}

// ---------------------------------------------------------------------------
// Updates
// ---------------------------------------------------------------------------

/// Ask whether a newer release exists.
///
/// Never fails: a check that could not run reports `checked: false` and the
/// app carries on. See `update.rs` for why the gate fails open.
#[tauri::command]
async fn check_for_update(app: AppHandle) -> update::UpdateStatus {
    update::check(&app).await
}

/// Download and install the update, then restart into it.
#[tauri::command]
async fn install_update(app: AppHandle) -> ApiResult<()> {
    update::install(&app).await.map_err(|message| ApiError {
        code: "update_failed".into(),
        message,
    })
}

/// Open the product site in the user's browser. Used by the product logo.
///
/// Restricted to the product's own site: a command that opens any URL is a
/// phishing primitive if the web view is ever compromised.
#[tauri::command]
fn open_product_site(app: AppHandle) -> ApiResult<()> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_url(cwico_core::PRODUCT_URL, None::<&str>)
        .map_err(|e| ApiError::other(format!("could not open the browser: {e}")))
}

/// Open tsudev.com (the "made by" block and About's website button).
/// Fixed URL for the same reason as `open_product_site`.
#[tauri::command]
fn open_maker_site(app: AppHandle) -> ApiResult<()> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_url(cwico_core::MAKER_URL, None::<&str>)
        .map_err(|e| ApiError::other(format!("could not open the browser: {e}")))
}

/// Open the tsudev donation page in the user's browser (About panel).
///
/// Fixed URL for the same reason as `open_product_site`.
#[tauri::command]
fn open_donate_page(app: AppHandle) -> ApiResult<()> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_url(cwico_core::DONATE_URL, None::<&str>)
        .map_err(|e| ApiError::other(format!("could not open the browser: {e}")))
}

/// Whether to show the "several devices, no Pro" notice (PRODUCT_POLICY 7.3).
/// The UI passes its stored `tsu.pro_notice.last_shown`; the rule lives in
/// `cwico_core::pro`, the status comes from the cached, re-verified JWS.
/// Signed out ⇒ always `None`, and never touches the network.
#[tauri::command]
async fn pro_notice(
    account: State<'_, account::Account>,
    last_shown: Option<String>,
) -> Result<Option<u32>, ()> {
    use cwico_core::pro::{notice_due, parse_last_shown};
    let status = account.status().await;
    Ok(notice_due(
        &status,
        parse_last_shown(last_shown.as_deref()),
        time::OffsetDateTime::now_utc(),
    ))
}

/// Renewal reminder / expiry notice of PRODUCT_POLICY 7.6. `last_active_ms`
/// is when the window last went to the background (`None` on a cold start).
#[tauri::command]
async fn pro_renewal_notice(
    account: State<'_, account::Account>,
    last_active_ms: Option<i64>,
) -> Result<Option<cwico_core::pro::renewal::RenewalNotice>, ()> {
    Ok(account.renewal_notice(last_active_ms).await)
}

impl From<account::AccountError> for ApiError {
    fn from(e: account::AccountError) -> Self {
        Self {
            code: e.code().into(),
            message: e.message(),
        }
    }
}

/// The account as cached - offline, instant.
#[tauri::command]
async fn account_status(account: State<'_, account::Account>) -> Result<account::AccountView, ()> {
    Ok(account.view().await)
}

/// Re-reads the entitlement if signed in and the cache is stale (or
/// `force`). Signed out ⇒ no network at all.
#[tauri::command]
async fn account_refresh(
    account: State<'_, account::Account>,
    force: Option<bool>,
) -> ApiResult<account::AccountView> {
    Ok(account.refresh(force.unwrap_or(false)).await?)
}

/// Opens the system browser on tsudev.com and waits for it to come back.
#[tauri::command]
async fn account_sign_in(
    app: AppHandle,
    account: State<'_, account::Account>,
) -> ApiResult<account::AccountView> {
    use tauri_plugin_opener::OpenerExt;
    let opener = app.opener();
    Ok(account
        .sign_in(|url| {
            opener
                .open_url(url, None::<&str>)
                .map_err(|e| format!("could not open the browser: {e}"))
        })
        .await?)
}

#[tauri::command]
fn account_sign_in_cancel(account: State<'_, account::Account>) {
    account.cancel_sign_in();
}

#[tauri::command]
async fn account_sign_out(account: State<'_, account::Account>) -> ApiResult<account::AccountView> {
    Ok(account.sign_out().await?)
}

/// Opens the TsuDEV Pro page. Takes no URL for the same reason as
/// `open_product_site`.
#[tauri::command]
fn open_pro_page(app: AppHandle) -> ApiResult<()> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_url(cwico_core::pro::PRO_URL, None::<&str>)
        .map_err(|e| ApiError::other(format!("could not open the browser: {e}")))
}

/// Open the folder holding `.reg` backups and transaction logs.
#[tauri::command]
fn open_backup_dir(app: AppHandle, state: State<'_, AppState>) -> ApiResult<()> {
    use tauri_plugin_opener::OpenerExt;
    std::fs::create_dir_all(&state.backup_dir)
        .map_err(|e| ApiError::other(format!("could not create the backup folder: {e}")))?;
    app.opener()
        .open_path(state.backup_dir.to_string_lossy(), None::<&str>)
        .map_err(|e| ApiError::other(format!("could not open the folder: {e}")))
}

/// Relaunch the application with an elevation prompt.
#[tauri::command]
fn relaunch_as_admin(app: AppHandle) -> ApiResult<()> {
    #[cfg(windows)]
    {
        let exe = std::env::current_exe()
            .map_err(|e| ApiError::other(format!("could not locate the executable: {e}")))?;
        // `runas` is the shell verb that raises the UAC prompt. Going through
        // the opener plugin keeps this a single, auditable path.
        use tauri_plugin_opener::OpenerExt;
        app.opener()
            .open_path(exe.to_string_lossy(), Some("runas"))
            .map_err(|e| ApiError::other(format!("the elevation prompt failed: {e}")))?;
        app.exit(0);
        Ok(())
    }
    #[cfg(not(windows))]
    {
        let _ = app;
        Err(ApiError::other(
            "elevation is a Windows concept; this build has no Windows backend",
        ))
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[cfg(all(windows, not(feature = "mock")))]
fn make_backend() -> Arc<dyn PlatformBackend> {
    Arc::new(cwico_win::WindowsBackend::new())
}

/// The fixture backend: any non-Windows host, or `--features mock` on Windows.
#[cfg(any(not(windows), feature = "mock"))]
fn make_backend() -> Arc<dyn PlatformBackend> {
    Arc::new(cwico_core::mock::MockBackend::new())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_env("CWICO_LOG")
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

    #[cfg(windows)]
    let builder = builder.plugin(tauri_plugin_updater::Builder::new().build());

    builder
        .setup(|app| {
            let state = AppState::new(make_backend(), default_backup_dir());
            tracing::info!(
                version = env!("CARGO_PKG_VERSION"),
                platform = %state.backend.platform_info().platform,
                elevated = state.backend.is_elevated(),
                "Tsu Cwico starting"
            );
            app.manage(state);
            let data = app
                .path()
                .app_local_data_dir()
                .unwrap_or_else(|_| std::env::temp_dir().join("tsucwico"));
            app.manage(account::Account::load(data));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            about,
            scan,
            build_plan,
            execute_plan,
            tweak_catalog,
            apply_tweaks,
            check_for_update,
            install_update,
            open_product_site,
            open_maker_site,
            open_donate_page,
            open_pro_page,
            pro_notice,
            pro_renewal_notice,
            account_status,
            account_refresh,
            account_sign_in,
            account_sign_in_cancel,
            account_sign_out,
            open_backup_dir,
            relaunch_as_admin,
        ])
        .run(tauri::generate_context!())
        .expect("failed to start the Tsu Cwico window");
}

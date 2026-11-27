//! Optional "Sign in with TsuDEV" and the TsuDEV Pro entitlement (ADR-0002,
//! phase P2; PRODUCT_POLICY 7.4 and 7.6).
//!
//! Signing in is never required and never prompted: every feature works
//! without it. Until the user presses "Sign in", nothing in this module
//! touches the network - [`Account::refresh`] returns at once when no
//! refresh token is stored (the "0 bytes before sign-in" guarantee, N2).
//!
//! Where things live:
//! - the refresh token: Windows Credential Manager (DPAPI-protected, per
//!   user), never a plain file nor the web view's storage;
//! - the access token: memory only (15 minutes);
//! - the entitlement JWS, the JWKS it was checked with, the installation id
//!   and the notice marks: `account.json` in the app's local data folder -
//!   none of it is secret, and the JWS is re-verified on every read.

use cwico_core::pro::discovery::{Provider, DISCOVERY_URL};
use cwico_core::pro::jws::{self, Jwks};
use cwico_core::pro::renewal::{self, RenewalNotice};
use cwico_core::pro::session::{session, Session};
use cwico_core::pro::signin::{Callback, SignInRequest, ENTITLEMENTS_URL};
use cwico_core::pro::{self, AccountState, Badge, EntitlementClaims, EntitlementStatus, CLIENT_ID};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::async_runtime::Mutex;
use time::OffsetDateTime;

/// Credential Manager target: `com.tsudev.cwico` / `tsudev-refresh-token`.
const KEYRING_SERVICE: &str = "com.tsudev.cwico";
const KEYRING_USER: &str = "tsudev-refresh-token";

/// How long the loopback port waits for the browser to come back.
const SIGNIN_TIMEOUT: Duration = Duration::from_secs(5 * 60);

/// A cached entitlement younger than this is not re-fetched at start-up.
const REFRESH_AFTER: time::Duration = time::Duration::hours(12);

#[derive(Debug, Default, Serialize, Deserialize)]
struct Store {
    installation_id: Option<String>,
    jws: Option<String>,
    jwks: Option<String>,
    /// This device has held a token with Pro active (7.6 expiry condition).
    #[serde(default)]
    seen_pro: bool,
    /// `tsu.pro_expired.notified_until` of PRODUCT_POLICY 7.6.
    #[serde(default, rename = "tsu.pro_expired.notified_until")]
    expired_notified_until: Option<i64>,
    /// The gateway refused the refresh token: the Account section says the
    /// session expired until the next sign-in or sign-out (14.4.2.5).
    #[serde(default)]
    session_expired: bool,
}

/// What the UI shows about the account. Carries no token.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountView {
    pub signed_in: bool,
    pub pro_active: bool,
    /// Unix seconds; 0 when the account never had Pro.
    pub pro_until: i64,
    pub badge: Badge,
    pub devices_30d: u32,
    /// Signed out because the session ran out, not because the user chose to.
    pub session_expired: bool,
    /// Restriction or suspension decided by the server (14.4.3).
    pub notice: Option<AccountNotice>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountNotice {
    pub state: AccountState,
    pub reason: Option<String>,
}

impl AccountView {
    fn signed_out() -> Self {
        Self::from_status(EntitlementStatus::free(), 0)
    }

    fn from_status(s: EntitlementStatus, pro_until: i64) -> Self {
        Self {
            signed_in: s.signed_in,
            pro_active: s.pro_active,
            pro_until,
            badge: s.badge,
            devices_30d: s.devices_30d,
            session_expired: false,
            notice: None,
        }
    }

    fn from_claims(c: &EntitlementClaims, s: EntitlementStatus) -> Self {
        Self {
            notice: c
                .status
                .as_deref()
                .and_then(AccountState::from_claim)
                .map(|state| AccountNotice {
                    state,
                    reason: c.status_reason.clone(),
                }),
            ..Self::from_status(s, c.pro_until)
        }
    }
}

/// Errors the UI translates by `code`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountError {
    /// The user closed the browser tab, pressed Cancel, or took too long.
    Cancelled,
    /// The provider said no (`access_denied`, ...).
    Denied,
    /// Network or server trouble; the app keeps working as free.
    Network(String),
    /// A reply failed a check (state, id_token, signature) - possibly an
    /// attack, never retried silently.
    Rejected(String),
    /// Credential Manager or the data folder refused.
    Storage(String),
}

impl AccountError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Cancelled => "signin_cancelled",
            Self::Denied => "signin_denied",
            Self::Network(_) => "signin_network",
            Self::Rejected(_) => "signin_rejected",
            Self::Storage(_) => "signin_storage",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::Cancelled => "sign-in cancelled".into(),
            Self::Denied => "sign-in refused".into(),
            Self::Network(m) | Self::Rejected(m) | Self::Storage(m) => m.clone(),
        }
    }
}

type Result<T> = std::result::Result<T, AccountError>;

pub struct Account {
    path: PathBuf,
    store: Mutex<Store>,
    /// Serialises token refreshes: the provider rotates refresh tokens and
    /// revokes the whole grant when an old one is replayed, so two refreshes
    /// racing with the same token would sign the user out.
    tokens: Mutex<Option<(String, Instant)>>,
    cancel: Arc<AtomicBool>,
    /// Discovery document, fetched at most once per run and only after the
    /// user pressed "Sign in" or when already signed in (N2).
    provider: Mutex<Option<Provider>>,
    http: reqwest::Client,
}

impl Account {
    pub fn load(dir: PathBuf) -> Self {
        let path = dir.join("account.json");
        let store = std::fs::read(&path)
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default();
        // reqwest is built without a default crypto provider (same choice
        // as tauri-plugin-updater: ring, no C toolchain needed).
        let _ = rustls::crypto::ring::default_provider().install_default();
        let http = reqwest::Client::builder()
            .user_agent(concat!("TsuCwico/", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_secs(20))
            .https_only(true)
            // A 307 would resend the code or the refresh token elsewhere.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("static HTTP client configuration");
        Self {
            path,
            store: Mutex::new(store),
            tokens: Mutex::new(None),
            cancel: Arc::new(AtomicBool::new(false)),
            provider: Mutex::new(None),
            http,
        }
    }

    /// The account as the cache says, offline. Never fails.
    pub async fn view(&self) -> AccountView {
        let store = self.store.lock().await;
        let now = OffsetDateTime::now_utc();
        let claims = valid_claims(&store, now);
        match session(
            has_refresh_token(),
            store.jws.is_some(),
            claims.is_some(),
            store.session_expired,
        ) {
            Session::SignedIn => match claims {
                Some((c, s)) => AccountView::from_claims(&c, s),
                None => AccountView {
                    signed_in: true,
                    ..AccountView::signed_out()
                },
            },
            Session::Expired => AccountView {
                session_expired: true,
                ..AccountView::signed_out()
            },
            Session::SignedOut => AccountView::signed_out(),
        }
    }

    /// Re-fetches the entitlement when signed in and the cache is old (or
    /// `force`). Signed out ⇒ returns at once, no network.
    pub async fn refresh(&self, force: bool) -> Result<AccountView> {
        if !has_refresh_token() {
            return Ok(self.view().await);
        }
        if !force {
            let store = self.store.lock().await;
            let fresh = valid_claims(&store, OffsetDateTime::now_utc()).is_some_and(|(c, _)| {
                OffsetDateTime::now_utc().unix_timestamp() - c.iat < REFRESH_AFTER.whole_seconds()
            });
            if fresh {
                drop(store);
                return Ok(self.view().await);
            }
        }
        let access = self.access_token().await?;
        self.fetch_entitlement(&access).await?;
        Ok(self.view().await)
    }

    /// Runs the whole browser sign-in. Resolves when the user is back.
    pub async fn sign_in(
        &self,
        open_browser: impl FnOnce(&str) -> std::result::Result<(), String>,
    ) -> Result<AccountView> {
        self.cancel.store(false, Ordering::SeqCst);
        // 127.0.0.1 only: nothing off this machine can reach the port, and
        // the state + PKCE verifier make a code caught by another local
        // process useless (docs/THREAT_MODEL.md, T3).
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .map_err(|e| AccountError::Storage(format!("cannot open the loopback port: {e}")))?;
        let port = listener
            .local_addr()
            .map_err(|e| AccountError::Storage(e.to_string()))?
            .port();
        let req = SignInRequest::new(port).map_err(|e| AccountError::Storage(e.to_string()))?;
        // The first packet to tsudev.com: after the click.
        let provider = self.provider().await?;
        if self.cancel.load(Ordering::SeqCst) {
            return Err(AccountError::Cancelled);
        }
        open_browser(&req.authorize_url(&provider)).map_err(AccountError::Network)?;

        let cancel = self.cancel.clone();
        let waiting = req.clone();
        let gateway = provider.clone();
        let code = tauri::async_runtime::spawn_blocking(move || {
            wait_for_callback(listener, &waiting, &gateway, &cancel)
        })
        .await
        .map_err(|e| AccountError::Network(e.to_string()))??;

        let tokens: TokenResponse = self
            .post_form(
                provider.token_endpoint.as_str(),
                &[
                    ("grant_type", "authorization_code"),
                    ("code", &code),
                    ("redirect_uri", &req.redirect_uri),
                    ("client_id", CLIENT_ID),
                    ("code_verifier", &req.code_verifier),
                ],
            )
            .await?;
        let jwks = self.fetch_jwks(&provider).await?;
        let id_token = tokens
            .id_token
            .as_deref()
            .ok_or_else(|| AccountError::Rejected("no id_token".into()))?;
        jws::verify_id_token(id_token, &jwks.1, &req.nonce, OffsetDateTime::now_utc())
            .map_err(|e| AccountError::Rejected(format!("id_token: {e:?}")))?;
        let refresh = tokens
            .refresh_token
            .ok_or_else(|| AccountError::Rejected("no refresh_token".into()))?;
        save_refresh_token(&refresh)?;
        *self.tokens.lock().await = Some((tokens.access_token.clone(), expiry(tokens.expires_in)));
        {
            let mut store = self.store.lock().await;
            store.jwks = Some(jwks.0);
            // Whatever an earlier session left behind is not this one's.
            store.jws = None;
            store.session_expired = false;
            self.save(&store)?;
        }
        self.fetch_entitlement(&tokens.access_token).await?;
        Ok(self.view().await)
    }

    pub fn cancel_sign_in(&self) {
        self.cancel.store(true, Ordering::SeqCst);
    }

    /// Revokes the refresh token (best effort) and forgets everything but
    /// the installation id. Data on this PC is untouched.
    pub async fn sign_out(&self) -> Result<AccountView> {
        let mut tokens = self.tokens.lock().await;
        if let Some(token) = load_refresh_token() {
            if let Err(e) = self.revoke(&token).await {
                tracing::warn!(error = ?e, "token revocation failed; forgetting it locally anyway");
            }
        }
        delete_refresh_token();
        *tokens = None;
        drop(tokens);
        let mut store = self.store.lock().await;
        let id = store.installation_id.take();
        *store = Store {
            installation_id: id,
            ..Store::default()
        };
        self.save(&store)?;
        Ok(AccountView::signed_out())
    }

    /// The renewal reminder / expiry notice due now (7.6). `last_active_ms`
    /// is when the window last went to the background (`None` on a cold
    /// start). Marks an expiry notice as told before returning it.
    pub async fn renewal_notice(&self, last_active_ms: Option<i64>) -> Option<RenewalNotice> {
        let now = OffsetDateTime::now_utc();
        let last_active = last_active_ms.and_then(|ms| {
            OffsetDateTime::from_unix_timestamp_nanos(i128::from(ms) * 1_000_000).ok()
        });
        let mut store = self.store.lock().await;
        let claims = valid_claims(&store, now).map(|(c, _)| c);
        let notice = renewal::renewal_notice(
            claims.as_ref(),
            store.seen_pro,
            store.expired_notified_until,
            renewal::is_new_open(last_active, now),
            now,
        );
        if let Some(RenewalNotice::Expired { pro_until, .. }) = notice {
            store.expired_notified_until = Some(pro_until);
            if let Err(e) = self.save(&store) {
                tracing::warn!(error = ?e, "could not record the expiry notice");
            }
        }
        notice
    }

    // -- internals --------------------------------------------------------

    async fn access_token(&self) -> Result<String> {
        let mut tokens = self.tokens.lock().await;
        if let Some((t, until)) = tokens.as_ref() {
            if Instant::now() + Duration::from_secs(30) < *until {
                return Ok(t.clone());
            }
        }
        let refresh = load_refresh_token().ok_or(AccountError::Cancelled)?;
        let provider = self.provider().await?;
        let reply = self
            .http
            .post(provider.token_endpoint.clone())
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh.as_str()),
                ("client_id", CLIENT_ID),
            ])
            .send()
            .await
            .map_err(|e| AccountError::Network(e.to_string()))?;
        if reply.status() == reqwest::StatusCode::BAD_REQUEST
            || reply.status() == reqwest::StatusCode::UNAUTHORIZED
        {
            // invalid_grant: expired (30 days unused), revoked, or replayed.
            // The user is signed out; the app simply carries on as free.
            tracing::info!("refresh token no longer accepted; signing out locally");
            delete_refresh_token();
            *tokens = None;
            let mut store = self.store.lock().await;
            // The cached entitlement would otherwise keep saying "signed in"
            // for up to 7 days.
            store.jws = None;
            store.session_expired = true;
            self.save(&store)?;
            return Err(AccountError::Denied);
        }
        let t: TokenResponse = json_or_error(reply).await?;
        // Store the rotated token before anything else can fail.
        if let Some(next) = &t.refresh_token {
            save_refresh_token(next)?;
        }
        *tokens = Some((t.access_token.clone(), expiry(t.expires_in)));
        Ok(t.access_token)
    }

    async fn provider(&self) -> Result<Provider> {
        let mut cached = self.provider.lock().await;
        if let Some(p) = cached.as_ref() {
            return Ok(p.clone());
        }
        let body = self.get_text(DISCOVERY_URL).await?;
        let p = Provider::from_json(&body)
            .map_err(|e| AccountError::Rejected(format!("discovery: {e:?}")))?;
        *cached = Some(p.clone());
        Ok(p)
    }

    async fn revoke(&self, token: &str) -> Result<()> {
        let provider = self.provider().await?;
        self.http
            .post(provider.revocation_endpoint)
            .form(&[
                ("token", token),
                ("token_type_hint", "refresh_token"),
                ("client_id", CLIENT_ID),
            ])
            .send()
            .await
            .map_err(|e| AccountError::Network(e.to_string()))?;
        Ok(())
    }

    /// A 2xx body; a redirect is not followed and counts as a failure.
    async fn get_text(&self, url: &str) -> Result<String> {
        let reply = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| AccountError::Network(e.to_string()))?;
        if !reply.status().is_success() {
            return Err(AccountError::Network(format!("HTTP {}", reply.status())));
        }
        reply
            .text()
            .await
            .map_err(|e| AccountError::Network(e.to_string()))
    }

    async fn fetch_jwks(&self, provider: &Provider) -> Result<(String, Jwks)> {
        let body = self.get_text(provider.jwks_uri.as_str()).await?;
        let jwks =
            Jwks::from_json(&body).map_err(|e| AccountError::Rejected(format!("jwks: {e:?}")))?;
        Ok((body, jwks))
    }

    async fn fetch_entitlement(&self, access: &str) -> Result<()> {
        let installation = {
            let mut store = self.store.lock().await;
            match store.installation_id.clone() {
                Some(id) if pro::is_installation_id(&id) => id,
                _ => {
                    let id = pro::new_installation_id()
                        .map_err(|e| AccountError::Storage(e.to_string()))?;
                    store.installation_id = Some(id.clone());
                    self.save(&store)?;
                    id
                }
            }
        };
        let reply = self
            .http
            .get(ENTITLEMENTS_URL)
            .bearer_auth(access)
            .header("X-Tsu-Installation", installation)
            .header("X-Tsu-Platform", "windows")
            .send()
            .await
            .map_err(|e| AccountError::Network(e.to_string()))?;
        #[derive(Deserialize)]
        struct Body {
            jws: String,
        }
        let body: Body = json_or_error(reply).await?;

        let now = OffsetDateTime::now_utc();
        let cached = self.store.lock().await.jwks.clone();
        let mut checked = cached
            .as_deref()
            .and_then(|j| Jwks::from_json(j).ok())
            .map(|k| jws::verify_entitlement(&body.jws, &k, now));
        // No cached JWKS, or it no longer verifies (the server rotated its
        // key): fetch the JWKS once more and decide with that.
        if !matches!(checked, Some(Ok(_))) {
            let provider = self.provider().await?;
            let (raw, fresh) = self.fetch_jwks(&provider).await?;
            checked = Some(jws::verify_entitlement(&body.jws, &fresh, now));
            self.store.lock().await.jwks = Some(raw);
        }
        let (_, status) = checked
            .unwrap_or(Err(jws::JwsRejected::UnknownKey))
            .map_err(|e| AccountError::Rejected(format!("entitlement: {e:?}")))?;
        let mut store = self.store.lock().await;
        store.jws = Some(body.jws);
        store.seen_pro |= status.pro_active;
        self.save(&store)
    }

    async fn post_form<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        form: &[(&str, &str)],
    ) -> Result<T> {
        let reply = self
            .http
            .post(url)
            .form(form)
            .send()
            .await
            .map_err(|e| AccountError::Network(e.to_string()))?;
        json_or_error(reply).await
    }

    fn save(&self, store: &Store) -> Result<()> {
        if let Some(dir) = self.path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| AccountError::Storage(e.to_string()))?;
        }
        let tmp = self.path.with_extension("json.tmp");
        std::fs::write(&tmp, serde_json::to_vec_pretty(store).unwrap_or_default())
            .and_then(|()| std::fs::rename(&tmp, &self.path))
            .map_err(|e| AccountError::Storage(e.to_string()))
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    id_token: Option<String>,
    #[serde(default)]
    expires_in: Option<u64>,
}

fn expiry(expires_in: Option<u64>) -> Instant {
    Instant::now() + Duration::from_secs(expires_in.unwrap_or(600))
}

async fn json_or_error<T: for<'de> Deserialize<'de>>(reply: reqwest::Response) -> Result<T> {
    let status = reply.status();
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        return Err(AccountError::Denied);
    }
    if !status.is_success() {
        return Err(AccountError::Network(format!("HTTP {status}")));
    }
    reply
        .json()
        .await
        .map_err(|e| AccountError::Rejected(e.to_string()))
}

/// The cached JWS, re-verified with the cached JWKS and checked at `now`.
fn valid_claims(
    store: &Store,
    now: OffsetDateTime,
) -> Option<(EntitlementClaims, EntitlementStatus)> {
    let jwks = Jwks::from_json(store.jwks.as_deref()?).ok()?;
    jws::verify_entitlement(store.jws.as_deref()?, &jwks, now).ok()
}

/// Accepts connections on the loopback port until our `/callback` arrives,
/// the user cancels, or the timeout passes.
fn wait_for_callback(
    listener: TcpListener,
    req: &SignInRequest,
    provider: &Provider,
    cancel: &AtomicBool,
) -> Result<String> {
    listener
        .set_nonblocking(true)
        .map_err(|e| AccountError::Storage(e.to_string()))?;
    let deadline = Instant::now() + SIGNIN_TIMEOUT;
    while Instant::now() < deadline && !cancel.load(Ordering::SeqCst) {
        let stream = match listener.accept() {
            Ok((stream, _)) => stream,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(e) => return Err(AccountError::Network(e.to_string())),
        };
        let Some((line, mut stream)) = read_request_line(stream) else {
            continue;
        };
        match req.callback(&line, provider) {
            Callback::NotOurs => {
                let _ = stream.write_all(
                    b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                );
            }
            Callback::Code(code) => {
                respond(&mut stream, true);
                return Ok(code);
            }
            Callback::Denied(error) => {
                respond(&mut stream, false);
                return Err(if error == "access_denied" {
                    AccountError::Cancelled
                } else {
                    AccountError::Denied
                });
            }
            Callback::WrongState => {
                respond(&mut stream, false);
                return Err(AccountError::Rejected("state mismatch".into()));
            }
            Callback::WrongIssuer => {
                respond(&mut stream, false);
                return Err(AccountError::Rejected("issuer mismatch".into()));
            }
        }
    }
    Err(AccountError::Cancelled)
}

fn read_request_line(mut stream: TcpStream) -> Option<(String, TcpStream)> {
    stream.set_nonblocking(false).ok()?;
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok()?;
    let mut buf = Vec::with_capacity(1024);
    let mut chunk = [0u8; 1024];
    while !buf.windows(2).any(|w| w == b"\r\n") && buf.len() < 8192 {
        let n = stream.read(&mut chunk).ok()?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    let line = String::from_utf8_lossy(&buf).lines().next()?.to_string();
    Some((line, stream))
}

fn respond(stream: &mut TcpStream, ok: bool) {
    let (title, body) = if ok {
        (
            "Đã đăng nhập - Tsu Cwico",
            "Đã đăng nhập TsuDEV. Bạn có thể đóng thẻ này và quay lại Tsu Cwico.<br>Signed in to TsuDEV. You can close this tab and return to Tsu Cwico.",
        )
    } else {
        (
            "Chưa đăng nhập - Tsu Cwico",
            "Chưa đăng nhập được. Quay lại Tsu Cwico để thử lại.<br>Sign-in did not complete. Return to Tsu Cwico to try again.",
        )
    };
    // Product favicon, embedded so the tab shows Tsu Cwico with nothing else
    // fetched (ticket LG1; same bytes as cwico.tsudev.com's favicon).
    use base64::Engine as _;
    let icon = base64::engine::general_purpose::STANDARD
        .encode(include_bytes!("../../../ui/public/brand/favicon-32.png"));
    let html = format!(
        "<!doctype html><html lang=\"vi\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width\"><link rel=\"icon\" type=\"image/png\" href=\"data:image/png;base64,{icon}\"><title>{title}</title><body style=\"font-family:system-ui;max-width:32rem;margin:4rem auto;padding:0 1rem;line-height:1.6\"><p>{body}</p></body></html>"
    );
    let head = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-store\r\nContent-Security-Policy: default-src 'none'; style-src 'unsafe-inline'; img-src data:\r\nConnection: close\r\n\r\n",
        html.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(html.as_bytes());
}

// -- Credential Manager ------------------------------------------------------
//
// Windows: a generic credential of the signed-in Windows user, encrypted by
// DPAPI. Other hosts only run the fixture build, which keeps the token in
// memory for the life of the process.

#[cfg(windows)]
fn entry() -> Option<keyring::Entry> {
    keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER).ok()
}

#[cfg(windows)]
fn load_refresh_token() -> Option<String> {
    entry()?.get_password().ok()
}

#[cfg(windows)]
fn save_refresh_token(token: &str) -> Result<()> {
    entry()
        .ok_or_else(|| AccountError::Storage("credential store unavailable".into()))?
        .set_password(token)
        .map_err(|e| AccountError::Storage(e.to_string()))
}

#[cfg(windows)]
fn delete_refresh_token() {
    if let Some(e) = entry() {
        let _ = e.delete_credential();
    }
}

#[cfg(not(windows))]
static MEMORY_TOKEN: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

#[cfg(not(windows))]
fn load_refresh_token() -> Option<String> {
    let _ = (KEYRING_SERVICE, KEYRING_USER);
    MEMORY_TOKEN.lock().ok()?.clone()
}

#[cfg(not(windows))]
fn save_refresh_token(token: &str) -> Result<()> {
    *MEMORY_TOKEN
        .lock()
        .map_err(|e| AccountError::Storage(e.to_string()))? = Some(token.to_owned());
    Ok(())
}

#[cfg(not(windows))]
fn delete_refresh_token() {
    if let Ok(mut t) = MEMORY_TOKEN.lock() {
        *t = None;
    }
}

fn has_refresh_token() -> bool {
    load_refresh_token().is_some()
}

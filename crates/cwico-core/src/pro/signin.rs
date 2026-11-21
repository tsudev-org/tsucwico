//! The pure half of "Sign in with TsuDEV": OAuth 2.0 for native apps
//! (RFC 8252) with a loopback redirect and PKCE S256 (RFC 7636).
//!
//! The app binds `127.0.0.1:<random port>`, opens the system browser at
//! [`SignInRequest::authorize_url`], and accepts exactly one request to
//! `/callback`. This module builds the request and judges the callback; the
//! socket, the browser and the HTTP calls live in the app.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use sha2::{Digest, Sha256};
use url::Url;

use super::CLIENT_ID;

pub const AUTHORIZE_URL: &str = "https://tsudev.com/oidc/auth";
pub const TOKEN_URL: &str = "https://tsudev.com/oidc/token";
pub const REVOCATION_URL: &str = "https://tsudev.com/oidc/token/revocation";
pub const ENTITLEMENTS_URL: &str = "https://tsudev.com/api/me/entitlements";
/// `offline_access` is granted anyway by the provider; asking for it keeps
/// the request honest about the refresh token the app will hold.
pub const SCOPE: &str = "openid tsudev.entitlements offline_access";
/// Path the provider redirects to; anything else on the port is refused.
pub const CALLBACK_PATH: &str = "/callback";

/// One sign-in attempt. Every field is single-use.
#[derive(Debug, Clone)]
pub struct SignInRequest {
    pub redirect_uri: String,
    pub state: String,
    pub nonce: String,
    pub code_verifier: String,
}

impl SignInRequest {
    /// Fresh secrets from the OS CSPRNG for a loopback listener on `port`.
    pub fn new(port: u16) -> Result<Self, getrandom::Error> {
        Ok(Self {
            redirect_uri: format!("http://127.0.0.1:{port}{CALLBACK_PATH}"),
            state: random_b64(16)?,
            nonce: random_b64(16)?,
            // 32 bytes -> 43 characters, the RFC 7636 minimum length.
            code_verifier: random_b64(32)?,
        })
    }

    pub fn code_challenge(&self) -> String {
        pkce_challenge(&self.code_verifier)
    }

    pub fn authorize_url(&self) -> String {
        let mut url = Url::parse(AUTHORIZE_URL).expect("constant URL");
        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", CLIENT_ID)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("scope", SCOPE)
            .append_pair("state", &self.state)
            .append_pair("nonce", &self.nonce)
            .append_pair("code_challenge", &self.code_challenge())
            .append_pair("code_challenge_method", "S256")
            // Refresh tokens are only issued on an explicit consent prompt.
            .append_pair("prompt", "consent");
        url.into()
    }

    /// Judges the request line of one loopback connection, e.g.
    /// `GET /callback?code=..&state=.. HTTP/1.1`.
    pub fn callback(&self, request_line: &str) -> Callback {
        let mut words = request_line.split_whitespace();
        let (Some("GET"), Some(target)) = (words.next(), words.next()) else {
            return Callback::NotOurs;
        };
        let Ok(url) = Url::parse(&format!("http://127.0.0.1{target}")) else {
            return Callback::NotOurs;
        };
        if url.path() != CALLBACK_PATH {
            return Callback::NotOurs;
        }
        let get = |k: &str| {
            url.query_pairs()
                .find(|(key, _)| key == k)
                .map(|(_, v)| v.into_owned())
        };
        // A forged redirect (CSRF) carries someone else's code: the state
        // proves the browser came back from *our* request.
        if get("state").as_deref() != Some(self.state.as_str()) {
            return Callback::WrongState;
        }
        if let Some(error) = get("error") {
            return Callback::Denied(error);
        }
        match get("code") {
            Some(code) if !code.is_empty() => Callback::Code(code),
            _ => Callback::WrongState,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Callback {
    /// Exchange this code (with the verifier) at the token endpoint.
    Code(String),
    /// The user cancelled, or the provider refused (`access_denied`...).
    Denied(String),
    /// Right path but the state does not match - abort the attempt.
    WrongState,
    /// Another path (a browser asking for `/favicon.ico`): answer 404 and
    /// keep waiting.
    NotOurs,
}

/// `BASE64URL(SHA256(verifier))` (RFC 7636 4.2).
pub fn pkce_challenge(verifier: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()))
}

fn random_b64(n: usize) -> Result<String, getrandom::Error> {
    let mut bytes = vec![0u8; n];
    getrandom::fill(&mut bytes)?;
    Ok(URL_SAFE_NO_PAD.encode(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pkce_matches_rfc7636_appendix_b() {
        assert_eq!(
            pkce_challenge("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"),
            "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
        );
    }

    #[test]
    fn the_authorize_url_carries_the_contract() {
        let r = SignInRequest::new(51_234).unwrap();
        assert_eq!(r.code_verifier.len(), 43);
        let url = Url::parse(&r.authorize_url()).unwrap();
        let q: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
        assert_eq!(url.origin().ascii_serialization(), "https://tsudev.com");
        assert_eq!(q["client_id"], "tsucwico");
        assert_eq!(q["redirect_uri"], "http://127.0.0.1:51234/callback");
        assert_eq!(q["code_challenge_method"], "S256");
        assert_eq!(q["code_challenge"], pkce_challenge(&r.code_verifier));
        assert_eq!(q["scope"], SCOPE);
        assert_eq!(q["state"], r.state);
        assert_eq!(q["nonce"], r.nonce);
        assert_ne!(r.state, SignInRequest::new(1).unwrap().state);
    }

    #[test]
    fn callback_takes_the_code_only_with_our_state() {
        let r = SignInRequest::new(1).unwrap();
        let line = |q: &str| format!("GET /callback?{q} HTTP/1.1");
        assert_eq!(
            r.callback(&line(&format!("code=abc&state={}", r.state))),
            Callback::Code("abc".into())
        );
        assert_eq!(
            r.callback(&line("code=abc&state=forged")),
            Callback::WrongState
        );
        assert_eq!(r.callback(&line("code=abc")), Callback::WrongState);
        assert_eq!(
            r.callback(&line(&format!("error=access_denied&state={}", r.state))),
            Callback::Denied("access_denied".into())
        );
        assert_eq!(r.callback("GET /favicon.ico HTTP/1.1"), Callback::NotOurs);
        assert_eq!(
            r.callback(&format!("POST /callback?code=a&state={} HTTP/1.1", r.state)),
            Callback::NotOurs
        );
        assert_eq!(r.callback(""), Callback::NotOurs);
    }
}

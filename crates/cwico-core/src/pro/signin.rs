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

use super::discovery::Provider;
use super::{CLIENT_ID, ISSUER};

/// The product API on tsudev.com - not an OIDC endpoint, so discovery does
/// not list it.
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

    /// The gateway page the system browser opens: the provider's
    /// authorisation endpoint, from discovery, plus this attempt's request.
    pub fn authorize_url(&self, provider: &Provider) -> String {
        let mut url = provider.authorization_endpoint.clone();
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
    pub fn callback(&self, request_line: &str, provider: &Provider) -> Callback {
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
        // RFC 9207: the answer names who sent it, on success and on error,
        // so a code from another authorisation server is never exchanged.
        match get("iss") {
            Some(iss) if iss != ISSUER => return Callback::WrongIssuer,
            None if provider.iss_in_callback => return Callback::WrongIssuer,
            _ => {}
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
    /// `iss` is missing or names another server - abort the attempt.
    WrongIssuer,
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
    use crate::pro::discovery::tests::PRODUCTION;

    fn provider() -> Provider {
        Provider::from_json(PRODUCTION).unwrap()
    }

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
        let url = Url::parse(&r.authorize_url(&provider())).unwrap();
        let q: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
        assert_eq!(url.origin().ascii_serialization(), "https://tsudev.com");
        assert_eq!(url.path(), "/oidc/auth");
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
        let p = provider();
        let line = |q: &str| format!("GET /callback?{q}&iss={ISSUER} HTTP/1.1");
        assert_eq!(
            r.callback(&line(&format!("code=abc&state={}", r.state)), &p),
            Callback::Code("abc".into())
        );
        assert_eq!(
            r.callback(&line("code=abc&state=forged"), &p),
            Callback::WrongState
        );
        assert_eq!(r.callback(&line("code=abc"), &p), Callback::WrongState);
        assert_eq!(
            r.callback(&line(&format!("error=access_denied&state={}", r.state)), &p),
            Callback::Denied("access_denied".into())
        );
        assert_eq!(
            r.callback("GET /favicon.ico HTTP/1.1", &p),
            Callback::NotOurs
        );
        assert_eq!(
            r.callback(
                &format!("POST /callback?code=a&state={} HTTP/1.1", r.state),
                &p
            ),
            Callback::NotOurs
        );
        assert_eq!(r.callback("", &p), Callback::NotOurs);
    }

    #[test]
    fn callback_refuses_a_missing_or_foreign_issuer() {
        let r = SignInRequest::new(1).unwrap();
        let mut p = provider();
        let line = |tail: &str| format!("GET /callback?state={}&{tail} HTTP/1.1", r.state);
        for tail in [
            "code=abc",
            "code=abc&iss=https%3A%2F%2Fevil.example",
            "error=access_denied",
        ] {
            assert_eq!(r.callback(&line(tail), &p), Callback::WrongIssuer, "{tail}");
        }
        assert_eq!(
            r.callback(&line("code=abc&iss=https%3A%2F%2Ftsudev.com%2Foidc"), &p),
            Callback::Code("abc".into())
        );
        // A provider that does not advertise `iss`: absence is fine, a wrong
        // one still is not.
        p.iss_in_callback = false;
        assert_eq!(
            r.callback(&line("code=abc"), &p),
            Callback::Code("abc".into())
        );
        assert_eq!(
            r.callback(&line("code=abc&iss=x"), &p),
            Callback::WrongIssuer
        );
    }
}

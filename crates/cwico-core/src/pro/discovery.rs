//! OpenID Connect discovery of the one TsuDEV sign-in gateway
//! (AUTH_AND_ACCOUNT 2 and 2.1): the app learns the provider's endpoints
//! from the issuer instead of writing them down, so the gateway can move or
//! be fixed once for every product.
//!
//! Pure: the caller fetches the document (only after the user pressed
//! "Sign in", or when already signed in); this module decides whether to
//! believe it. A document that points anywhere but `https://tsudev.com` is
//! refused whole - it is where the code and the refresh token would be sent.

use serde::Deserialize;
use url::Url;

use super::ISSUER;

/// `<issuer>/.well-known/openid-configuration` (OIDC Discovery 1.0, 4).
pub const DISCOVERY_URL: &str = "https://tsudev.com/oidc/.well-known/openid-configuration";

/// The only host an endpoint may name.
const GATEWAY_HOST: &str = "tsudev.com";

/// The endpoints the app uses, already checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provider {
    pub authorization_endpoint: Url,
    pub token_endpoint: Url,
    pub revocation_endpoint: Url,
    pub jwks_uri: Url,
    /// RFC 9207: the redirect back must then carry `iss`.
    pub iss_in_callback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoveryRejected {
    /// Not JSON, or a required field is missing.
    Malformed,
    WrongIssuer,
    /// An endpoint that is not plain `https://tsudev.com/...`.
    ForeignEndpoint(&'static str),
    /// The provider does not offer PKCE `S256`.
    NoPkce,
}

#[derive(Deserialize)]
struct Document {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
    revocation_endpoint: String,
    jwks_uri: String,
    #[serde(default)]
    code_challenge_methods_supported: Vec<String>,
    #[serde(default)]
    authorization_response_iss_parameter_supported: bool,
}

impl Provider {
    pub fn from_json(json: &str) -> Result<Self, DiscoveryRejected> {
        let d: Document = serde_json::from_str(json).map_err(|_| DiscoveryRejected::Malformed)?;
        // Compared byte for byte, as OIDC Discovery 4.3 requires.
        if d.issuer != ISSUER {
            return Err(DiscoveryRejected::WrongIssuer);
        }
        if !d
            .code_challenge_methods_supported
            .iter()
            .any(|m| m == "S256")
        {
            return Err(DiscoveryRejected::NoPkce);
        }
        Ok(Self {
            authorization_endpoint: endpoint("authorization_endpoint", &d.authorization_endpoint)?,
            token_endpoint: endpoint("token_endpoint", &d.token_endpoint)?,
            revocation_endpoint: endpoint("revocation_endpoint", &d.revocation_endpoint)?,
            jwks_uri: endpoint("jwks_uri", &d.jwks_uri)?,
            iss_in_callback: d.authorization_response_iss_parameter_supported,
        })
    }
}

fn endpoint(name: &'static str, value: &str) -> Result<Url, DiscoveryRejected> {
    let refused = DiscoveryRejected::ForeignEndpoint(name);
    let url = Url::parse(value).map_err(|_| refused.clone())?;
    // `port()` is `None` for the scheme's default port.
    let ours = url.scheme() == "https"
        && url.host_str() == Some(GATEWAY_HOST)
        && url.port().is_none()
        && url.username().is_empty()
        && url.password().is_none()
        && url.fragment().is_none();
    if ours {
        Ok(url)
    } else {
        Err(refused)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// The live document of tsudev.com as fetched on 27/11/2026 (public
    /// data), reduced to the fields that matter plus one that is ignored.
    pub(crate) const PRODUCTION: &str = r#"{
        "issuer": "https://tsudev.com/oidc",
        "authorization_endpoint": "https://tsudev.com/oidc/auth",
        "token_endpoint": "https://tsudev.com/oidc/token",
        "revocation_endpoint": "https://tsudev.com/oidc/token/revocation",
        "jwks_uri": "https://tsudev.com/oidc/jwks",
        "userinfo_endpoint": "https://tsudev.com/oidc/me",
        "code_challenge_methods_supported": ["S256"],
        "authorization_response_iss_parameter_supported": true
    }"#;

    #[test]
    fn the_production_document_is_accepted() {
        let p = Provider::from_json(PRODUCTION).unwrap();
        assert_eq!(p.token_endpoint.as_str(), "https://tsudev.com/oidc/token");
        assert!(p.iss_in_callback);
        assert_eq!(
            DISCOVERY_URL,
            format!("{ISSUER}/.well-known/openid-configuration")
        );
    }

    #[test]
    fn another_issuer_is_refused() {
        for bad in [
            "https://tsudev.com",
            "https://tsudev.com/oidc/",
            "https://evil.example/oidc",
        ] {
            let doc = PRODUCTION.replace(
                r#""issuer": "https://tsudev.com/oidc""#,
                &format!(r#""issuer": "{bad}""#),
            );
            assert_eq!(
                Provider::from_json(&doc),
                Err(DiscoveryRejected::WrongIssuer),
                "{bad}"
            );
        }
    }

    #[test]
    fn an_endpoint_off_the_gateway_is_refused() {
        for bad in [
            "http://tsudev.com/oidc/token",
            "https://evil.example/oidc/token",
            "https://tsudev.com.evil.example/oidc/token",
            "https://id.tsudev.com.evil.example/token",
            "https://tsudev.com:8443/oidc/token",
            "https://user@tsudev.com/oidc/token",
            "https://tsudev.com@evil.example/oidc/token",
            "https://tsudev.com/oidc/token#x",
            "/oidc/token",
        ] {
            let doc = PRODUCTION.replace(
                r#""token_endpoint": "https://tsudev.com/oidc/token""#,
                &format!(r#""token_endpoint": "{bad}""#),
            );
            assert_eq!(
                Provider::from_json(&doc),
                Err(DiscoveryRejected::ForeignEndpoint("token_endpoint")),
                "{bad}"
            );
        }
    }

    #[test]
    fn without_s256_or_a_required_field_it_is_refused() {
        assert_eq!(
            Provider::from_json(&PRODUCTION.replace("S256", "plain")),
            Err(DiscoveryRejected::NoPkce)
        );
        assert_eq!(
            Provider::from_json(&PRODUCTION.replace("jwks_uri", "x")),
            Err(DiscoveryRejected::Malformed)
        );
        assert_eq!(
            Provider::from_json("<html>"),
            Err(DiscoveryRejected::Malformed)
        );
    }
}

//! Signature check of the entitlement JWS (PRODUCT_POLICY 7.4).
//!
//! The token is a compact JWS signed by tsudev.com with its OIDC key:
//! header `alg` = `ES256`, `typ` = `tsu-ent+jwt`, `kid` naming a key of
//! `https://tsudev.com/oidc/jwks`. Anything else is refused before the
//! claims are even parsed - an `alg` the server never uses is the classic
//! way to forge a token (`none`, or HS256 keyed with the public key).
//!
//! Pure: the caller fetches the JWKS and the token; this module only decides.

use super::{ClaimsRejected, EntitlementClaims, EntitlementStatus, CLIENT_ID, ISSUER};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use p256::ecdsa::signature::Verifier;
use p256::ecdsa::{Signature, VerifyingKey};
use serde::Deserialize;
use time::OffsetDateTime;

/// The only signature algorithm accepted.
pub const ALG: &str = "ES256";
/// Media type of an entitlement token (`typ` header).
pub const TYP: &str = "tsu-ent+jwt";
/// Where the signing keys are published (from the OIDC discovery document).
pub const JWKS_URL: &str = "https://tsudev.com/oidc/jwks";

/// One public key of the issuer's JWKS. Only P-256 EC keys are usable;
/// others (an RSA key published for `id_token` of other clients, say) are
/// kept so a `kid` that names them is refused rather than "not found".
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Jwk {
    pub kty: String,
    #[serde(default)]
    pub kid: Option<String>,
    #[serde(default)]
    pub crv: Option<String>,
    #[serde(default)]
    pub x: Option<String>,
    #[serde(default)]
    pub y: Option<String>,
    #[serde(default)]
    pub alg: Option<String>,
    #[serde(default, rename = "use")]
    pub use_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}

impl Jwks {
    pub fn from_json(json: &str) -> Result<Self, JwsRejected> {
        serde_json::from_str(json).map_err(|_| JwsRejected::Malformed)
    }

    fn find(&self, kid: &str) -> Option<&Jwk> {
        self.keys.iter().find(|k| k.kid.as_deref() == Some(kid))
    }
}

/// Why a token was refused. The app then simply behaves as free.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JwsRejected {
    /// Not three base64url parts, or a part is not the JSON it should be.
    Malformed,
    WrongAlgorithm,
    WrongType,
    /// No usable key with that `kid` - refresh the JWKS once and retry.
    UnknownKey,
    BadSignature,
    Claims(ClaimsRejected),
}

#[derive(Deserialize)]
struct Header {
    alg: String,
    #[serde(default)]
    typ: Option<String>,
    #[serde(default)]
    kid: Option<String>,
}

/// Verifies `compact` against `jwks` and checks its claims at `now`.
///
/// Order matters: header, then signature, and only then are the claims
/// trusted enough to be interpreted.
pub fn verify_entitlement(
    compact: &str,
    jwks: &Jwks,
    now: OffsetDateTime,
) -> Result<(EntitlementClaims, EntitlementStatus), JwsRejected> {
    let claims = verify_signature(compact, jwks)?;
    let status = EntitlementStatus::from_claims(&claims, now).map_err(JwsRejected::Claims)?;
    Ok((claims, status))
}

/// Header + signature check only; the claims come back unchecked. Used to
/// read a cached token that may have expired (which still tells the app the
/// device once saw Pro), never to grant anything.
pub fn verify_signature(compact: &str, jwks: &Jwks) -> Result<EntitlementClaims, JwsRejected> {
    verify_es256(compact, jwks, Some(TYP))
}

/// ES256 check against `jwks`, then the payload parsed as `T`. `typ` is
/// required to equal `want_typ` when given (entitlement tokens); an
/// `id_token` is checked with `None`, since OIDC sets no `typ` for it.
pub fn verify_es256<T: for<'de> Deserialize<'de>>(
    compact: &str,
    jwks: &Jwks,
    want_typ: Option<&str>,
) -> Result<T, JwsRejected> {
    let compact = compact.trim();
    let mut parts = compact.split('.');
    let (Some(h64), Some(p64), Some(s64), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(JwsRejected::Malformed);
    };
    let header: Header = decode_json(h64)?;
    if header.alg != ALG {
        return Err(JwsRejected::WrongAlgorithm);
    }
    if want_typ.is_some_and(|t| header.typ.as_deref() != Some(t)) {
        return Err(JwsRejected::WrongType);
    }
    let key = header
        .kid
        .as_deref()
        .and_then(|kid| jwks.find(kid))
        .ok_or(JwsRejected::UnknownKey)
        .and_then(verifying_key)?;
    let sig = URL_SAFE_NO_PAD
        .decode(s64)
        .map_err(|_| JwsRejected::Malformed)?;
    // JWS carries the raw r||s form (RFC 7518 3.4), never DER.
    let sig = Signature::from_slice(&sig).map_err(|_| JwsRejected::BadSignature)?;
    let signing_input = &compact[..h64.len() + 1 + p64.len()];
    key.verify(signing_input.as_bytes(), &sig)
        .map_err(|_| JwsRejected::BadSignature)?;
    decode_json(p64)
}

/// Why an `id_token` was refused (OIDC Core 3.1.3.7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdTokenRejected {
    Jws(JwsRejected),
    WrongIssuer,
    WrongAudience,
    /// The `nonce` sent with the authorisation request did not come back.
    WrongNonce,
    Expired,
}

#[derive(Deserialize)]
struct IdTokenClaims {
    iss: String,
    sub: String,
    aud: Audience,
    exp: i64,
    #[serde(default)]
    nonce: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Audience {
    One(String),
    Many(Vec<String>),
}

/// Verifies the `id_token` of a sign-in and returns its `sub`.
pub fn verify_id_token(
    compact: &str,
    jwks: &Jwks,
    nonce: &str,
    now: OffsetDateTime,
) -> Result<String, IdTokenRejected> {
    let c: IdTokenClaims = verify_es256(compact, jwks, None).map_err(IdTokenRejected::Jws)?;
    if c.iss != ISSUER {
        return Err(IdTokenRejected::WrongIssuer);
    }
    let for_us = match &c.aud {
        Audience::One(a) => a == CLIENT_ID,
        Audience::Many(v) => v.len() == 1 && v[0] == CLIENT_ID,
    };
    if !for_us {
        return Err(IdTokenRejected::WrongAudience);
    }
    if c.nonce.as_deref() != Some(nonce) {
        return Err(IdTokenRejected::WrongNonce);
    }
    if c.exp + 60 <= now.unix_timestamp() {
        return Err(IdTokenRejected::Expired);
    }
    Ok(c.sub)
}

fn verifying_key(jwk: &Jwk) -> Result<VerifyingKey, JwsRejected> {
    if jwk.kty != "EC"
        || jwk.crv.as_deref() != Some("P-256")
        || jwk.alg.as_deref().is_some_and(|a| a != ALG)
        || jwk.use_.as_deref().is_some_and(|u| u != "sig")
    {
        return Err(JwsRejected::UnknownKey);
    }
    let coord = |c: &Option<String>| -> Result<Vec<u8>, JwsRejected> {
        let bytes = URL_SAFE_NO_PAD
            .decode(c.as_deref().unwrap_or_default())
            .map_err(|_| JwsRejected::UnknownKey)?;
        if bytes.len() == 32 {
            Ok(bytes)
        } else {
            Err(JwsRejected::UnknownKey)
        }
    };
    let mut sec1 = Vec::with_capacity(65);
    sec1.push(0x04);
    sec1.extend(coord(&jwk.x)?);
    sec1.extend(coord(&jwk.y)?);
    VerifyingKey::from_sec1_bytes(&sec1).map_err(|_| JwsRejected::UnknownKey)
}

fn decode_json<T: for<'de> Deserialize<'de>>(part: &str) -> Result<T, JwsRejected> {
    let bytes = URL_SAFE_NO_PAD
        .decode(part)
        .map_err(|_| JwsRejected::Malformed)?;
    serde_json::from_slice(&bytes).map_err(|_| JwsRejected::Malformed)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::pro::Badge;
    use p256::ecdsa::signature::Signer;
    use p256::ecdsa::SigningKey;
    use time::Duration;

    pub(crate) fn key(seed: u8) -> SigningKey {
        SigningKey::from_slice(&[seed; 32]).unwrap()
    }

    pub(crate) fn jwks_for(key: &SigningKey, kid: &str) -> Jwks {
        let point = key.verifying_key().to_sec1_point(false);
        Jwks {
            keys: vec![Jwk {
                kty: "EC".into(),
                kid: Some(kid.into()),
                crv: Some("P-256".into()),
                x: Some(URL_SAFE_NO_PAD.encode(point.x().unwrap())),
                y: Some(URL_SAFE_NO_PAD.encode(point.y().unwrap())),
                alg: Some("ES256".into()),
                use_: Some("sig".into()),
            }],
        }
    }

    pub(crate) fn sign(key: &SigningKey, header: &str, payload: &str) -> String {
        let input = format!(
            "{}.{}",
            URL_SAFE_NO_PAD.encode(header),
            URL_SAFE_NO_PAD.encode(payload)
        );
        let sig: Signature = key.sign(input.as_bytes());
        format!("{input}.{}", URL_SAFE_NO_PAD.encode(sig.to_bytes()))
    }

    pub(crate) const HEADER: &str = r#"{"alg":"ES256","typ":"tsu-ent+jwt","kid":"k1"}"#;

    pub(crate) fn payload(iat: i64, pro_until: i64) -> String {
        serde_json::json!({
            "iss": ISSUER, "sub": "u1", "aud": CLIENT_ID,
            "iat": iat, "exp": iat + 7 * 86_400,
            "pro_until": pro_until, "pro_source": "web", "donor": true,
            "badge": "both", "devices_30d": 2, "cnf": { "jkt": "x" }
        })
        .to_string()
    }

    fn now() -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(1_800_000_000).unwrap()
    }

    #[test]
    fn accepts_a_token_signed_by_the_jwks_key() {
        let k = key(7);
        let jws = sign(&k, HEADER, &payload(now().unix_timestamp(), 0));
        let (claims, status) = verify_entitlement(&jws, &jwks_for(&k, "k1"), now()).unwrap();
        assert_eq!(claims.badge, Badge::Both);
        assert!(status.signed_in && !status.pro_active);
        assert_eq!(status.devices_30d, 2);
    }

    #[test]
    fn refuses_another_key_a_tampered_payload_and_an_unknown_kid() {
        let k = key(7);
        let jws = sign(&k, HEADER, &payload(now().unix_timestamp(), 0));
        assert_eq!(
            verify_entitlement(&jws, &jwks_for(&key(8), "k1"), now()),
            Err(JwsRejected::BadSignature)
        );
        assert_eq!(
            verify_entitlement(&jws, &jwks_for(&k, "k2"), now()),
            Err(JwsRejected::UnknownKey)
        );
        // Swap in a payload granting Pro, keep the original signature.
        let forged = payload(now().unix_timestamp(), now().unix_timestamp() + 86_400);
        let mut parts: Vec<&str> = jws.split('.').collect();
        let p = URL_SAFE_NO_PAD.encode(forged);
        parts[1] = &p;
        assert_eq!(
            verify_entitlement(&parts.join("."), &jwks_for(&k, "k1"), now()),
            Err(JwsRejected::BadSignature)
        );
    }

    #[test]
    fn refuses_alg_none_hs256_and_a_wrong_typ() {
        let k = key(7);
        let jwks = jwks_for(&k, "k1");
        let p = payload(now().unix_timestamp(), 0);
        for (header, why) in [
            (
                r#"{"alg":"none","typ":"tsu-ent+jwt","kid":"k1"}"#,
                JwsRejected::WrongAlgorithm,
            ),
            (
                r#"{"alg":"HS256","typ":"tsu-ent+jwt","kid":"k1"}"#,
                JwsRejected::WrongAlgorithm,
            ),
            (
                r#"{"alg":"ES256","typ":"JWT","kid":"k1"}"#,
                JwsRejected::WrongType,
            ),
            (r#"{"alg":"ES256","kid":"k1"}"#, JwsRejected::WrongType),
        ] {
            assert_eq!(
                verify_entitlement(&sign(&k, header, &p), &jwks, now()),
                Err(why)
            );
        }
        assert_eq!(
            verify_entitlement("a.b", &jwks, now()),
            Err(JwsRejected::Malformed)
        );
        assert_eq!(
            verify_entitlement("a.b.c.d", &jwks, now()),
            Err(JwsRejected::Malformed)
        );
    }

    #[test]
    fn signature_ok_but_expired_or_foreign_audience_is_refused() {
        let k = key(7);
        let jwks = jwks_for(&k, "k1");
        let old = (now() - Duration::days(8)).unix_timestamp();
        let jws = sign(&k, HEADER, &payload(old, 0));
        assert_eq!(
            verify_entitlement(&jws, &jwks, now()),
            Err(JwsRejected::Claims(ClaimsRejected::Expired))
        );
        // ...yet the signature alone still verifies, for the "seen Pro" mark.
        assert!(verify_signature(&jws, &jwks).is_ok());
        let foreign = payload(now().unix_timestamp(), 0).replace(CLIENT_ID, "tsuswico");
        assert_eq!(
            verify_entitlement(&sign(&k, HEADER, &foreign), &jwks, now()),
            Err(JwsRejected::Claims(ClaimsRejected::WrongAudience))
        );
    }

    #[test]
    fn refuses_a_non_p256_key_under_the_right_kid() {
        let k = key(7);
        let mut jwks = jwks_for(&k, "k1");
        jwks.keys[0].crv = Some("P-384".into());
        let jws = sign(&k, HEADER, &payload(now().unix_timestamp(), 0));
        assert_eq!(
            verify_entitlement(&jws, &jwks, now()),
            Err(JwsRejected::UnknownKey)
        );
    }

    #[test]
    fn id_token_checks_issuer_audience_nonce_and_expiry() {
        let k = key(9);
        let jwks = jwks_for(&k, "k1");
        let h = r#"{"alg":"ES256","kid":"k1"}"#;
        let tok = |iss: &str, aud: serde_json::Value, nonce: &str, exp: i64| {
            let p = serde_json::json!({"iss": iss, "sub": "s1", "aud": aud, "exp": exp, "iat": 0, "nonce": nonce});
            sign(&k, h, &p.to_string())
        };
        // Fresh random nonces, as the app makes them (SignInRequest::new).
        let n1 = crate::pro::new_installation_id().unwrap();
        let n2 = crate::pro::new_installation_id().unwrap();
        let (n1, n2) = (n1.as_str(), n2.as_str());
        let exp = now().unix_timestamp() + 600;
        let ok = tok(ISSUER, CLIENT_ID.into(), n1, exp);
        assert_eq!(verify_id_token(&ok, &jwks, n1, now()), Ok("s1".into()));
        assert_eq!(
            verify_id_token(&ok, &jwks, n2, now()),
            Err(IdTokenRejected::WrongNonce)
        );
        let list = tok(ISSUER, serde_json::json!([CLIENT_ID]), n1, exp);
        assert!(verify_id_token(&list, &jwks, n1, now()).is_ok());
        let other = tok(ISSUER, serde_json::json!([CLIENT_ID, "x"]), n1, exp);
        assert_eq!(
            verify_id_token(&other, &jwks, n1, now()),
            Err(IdTokenRejected::WrongAudience)
        );
        let iss = tok("https://evil.example", CLIENT_ID.into(), n1, exp);
        assert_eq!(
            verify_id_token(&iss, &jwks, n1, now()),
            Err(IdTokenRejected::WrongIssuer)
        );
        let old = tok(ISSUER, CLIENT_ID.into(), n1, now().unix_timestamp() - 120);
        assert_eq!(
            verify_id_token(&old, &jwks, n1, now()),
            Err(IdTokenRejected::Expired)
        );
        let none = sign(&k, r#"{"alg":"none","kid":"k1"}"#, "{}");
        assert_eq!(
            verify_id_token(&none, &jwks, n1, now()),
            Err(IdTokenRejected::Jws(JwsRejected::WrongAlgorithm))
        );
    }

    /// The live tsudev.com JWKS as fetched on 21/11/2026 (public data): the
    /// key must load, or every real token would be refused as UnknownKey.
    #[test]
    fn the_production_jwks_key_loads() {
        let j = Jwks::from_json(
            r#"{"keys":[{"kty":"EC","use":"sig","kid":"c77f010f-3d39-4690-807c-33d1bd6dbf2d","alg":"ES256","crv":"P-256","x":"_W6m2gUZ0RTJgQneHtKJmsnKbwHUjQOcX7ZjP9P4m0g","y":"gOeqTamCmrW2iY8uXIme3j0Sz5tq4hMBrlqdM6kJ1Qg"}]}"#,
        )
        .unwrap();
        assert!(verifying_key(j.find("c77f010f-3d39-4690-807c-33d1bd6dbf2d").unwrap()).is_ok());
    }

    #[test]
    fn parses_a_jwks_document() {
        let j = Jwks::from_json(
            r#"{"keys":[{"kty":"EC","crv":"P-256","kid":"c77f","x":"AA","y":"AA","alg":"ES256","use":"sig"},{"kty":"RSA","kid":"r","n":"x","e":"AQAB"}]}"#,
        )
        .unwrap();
        assert_eq!(j.keys.len(), 2);
        assert_eq!(j.find("r").unwrap().kty, "RSA");
    }
}

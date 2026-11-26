//! TsuDEV Pro: entitlement claims and the "several devices, no Pro" notice.
//!
//! Pure logic only - nothing here touches the network or the disk, so every
//! rule of PRODUCT_POLICY.md section 7 is unit-tested on any host
//! (ADR-0002). [`jws`] checks the ES256 signature of the entitlement token,
//! [`renewal`] decides the renewal reminder and expiry notice of 7.6. The
//! network and the credential store live in the app (sign-in is optional:
//! without an account [`EntitlementStatus::free`] is the only state).
//!
//! What the free build guarantees (N2): no user data leaves the machine. The
//! only outbound request is the update check, which sends nothing about the
//! machine or the user (docs/adr/0002 section 1).

use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

pub mod jws;
pub mod renewal;
pub mod signin;

/// OIDC `client_id` of this product at tsudev.com - also the `app` parameter
/// of the Pro and donation pages (PRODUCT_POLICY 7.4).
pub const CLIENT_ID: &str = "tsucwico";

/// Issuer every entitlement JWS must carry (PRODUCT_POLICY 7.4).
pub const ISSUER: &str = "https://tsudev.com/oidc";

/// Pro plans page for builds outside an app store (PRODUCT_POLICY 7.4) -
/// also the target of the "Renew" button (7.6).
pub const PRO_URL: &str = "https://tsudev.com/pro?app=tsucwico";

/// Longest lifetime an entitlement token may claim (`exp` <= `iat` + 7 days).
pub const MAX_TOKEN_LIFETIME: Duration = Duration::days(7);

/// Local storage keys left by notices that no longer exist: the "several
/// devices, no Pro" notice (PRODUCT_POLICY 7.3, removed in v2.0.0) and the
/// sign-in reminder of tsudev_ui LG3 (AUTH_AND_ACCOUNT 14.4, v4.0.0). The UI
/// deletes them at start-up; nothing reads them any more.
pub const LEGACY_LOCAL_KEYS: [&str; 3] = [
    "tsu.pro_notice.last_shown",
    "tsu.signin_reminder.next_at",
    "tsu.signin_reminder.dismiss_count",
];

/// Claims of the entitlement JWS, exactly as PRODUCT_POLICY 7.4 defines them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitlementClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,
    /// Unix seconds; 0 means the account never had Pro.
    pub pro_until: i64,
    #[serde(default)]
    pub pro_source: Option<String>,
    pub donor: bool,
    /// `pro` | `donor` | `both` | `none`, already filtered by `badge_display`.
    pub badge: Badge,
    pub devices_30d: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Badge {
    Pro,
    Donor,
    Both,
    None,
}

/// Why a set of claims was not accepted. The app then behaves as free; none
/// of these is shown to the user as an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimsRejected {
    WrongIssuer,
    WrongAudience,
    Expired,
    /// `exp` further than 7 days past `iat`, or `iat` in the future.
    BadLifetime,
}

/// What the app may do right now, derived from the cached claims (if any).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementStatus {
    /// Signed in with a token that is still valid.
    pub signed_in: bool,
    pub pro_active: bool,
    pub badge: Badge,
    /// Installations of this product active in the last 30 days.
    pub devices_30d: u32,
}

impl EntitlementStatus {
    /// No account, or no valid token: everything stays on this machine.
    pub const fn free() -> Self {
        Self {
            signed_in: false,
            pro_active: false,
            badge: Badge::None,
            devices_30d: 0,
        }
    }

    /// Checks `iss`, `aud`, `exp` and the 7-day lifetime cap. The caller has
    /// already verified the ES256 signature against the tsudev.com JWKS.
    pub fn from_claims(
        claims: &EntitlementClaims,
        now: OffsetDateTime,
    ) -> Result<Self, ClaimsRejected> {
        if claims.iss != ISSUER {
            return Err(ClaimsRejected::WrongIssuer);
        }
        if claims.aud != CLIENT_ID {
            return Err(ClaimsRejected::WrongAudience);
        }
        let now = now.unix_timestamp();
        if claims.iat > now + 60 || claims.exp - claims.iat > MAX_TOKEN_LIFETIME.whole_seconds() {
            return Err(ClaimsRejected::BadLifetime);
        }
        if claims.exp <= now {
            return Err(ClaimsRejected::Expired);
        }
        Ok(Self {
            signed_in: true,
            pro_active: claims.pro_until > now,
            badge: claims.badge,
            devices_30d: claims.devices_30d,
        })
    }

    /// Same as [`from_claims`](Self::from_claims), but any rejection - an
    /// expired cache while offline included - simply means "free".
    pub fn from_cached(claims: Option<&EntitlementClaims>, now: OffsetDateTime) -> Self {
        claims
            .and_then(|c| Self::from_claims(c, now).ok())
            .unwrap_or_else(Self::free)
    }
}

/// A fresh installation id: 128 random bits from the OS CSPRNG, base64url
/// without padding - 22 characters (AUTH_AND_ACCOUNT 14.3). Never derived
/// from hardware, and only ever sent once the user has signed in.
pub fn new_installation_id() -> Result<String, getrandom::Error> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes)?;
    Ok(base64url(&bytes))
}

/// True for a well-formed installation id (22 base64url characters).
pub fn is_installation_id(s: &str) -> bool {
    s.len() == 22
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        // 128 bits fill 21 characters and 2 bits of the last one; the other
        // 4 bits must be zero for the value to round-trip.
        && B64URL.iter().position(|&c| c == s.as_bytes()[21]).is_some_and(|i| i & 0x0f == 0)
}

const B64URL: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn base64url(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |acc, (i, &b)| acc | u32::from(b) << (16 - 8 * i));
        let chars = chunk.len() + 1;
        for i in 0..chars {
            out.push(B64URL[(n >> (18 - 6 * i) & 0x3f) as usize] as char);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(n: i64) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(1_800_000_000).unwrap() + Duration::days(n)
    }

    fn claims(now: OffsetDateTime, devices: u32, pro_until: i64) -> EntitlementClaims {
        EntitlementClaims {
            iss: ISSUER.into(),
            sub: "u1".into(),
            aud: CLIENT_ID.into(),
            iat: now.unix_timestamp(),
            exp: (now + Duration::days(7)).unix_timestamp(),
            pro_until,
            pro_source: None,
            donor: false,
            badge: Badge::None,
            devices_30d: devices,
        }
    }

    #[test]
    fn offline_with_expired_cache_is_free_and_does_not_fail() {
        let cached = claims(day(0), 2, 0);
        let s = EntitlementStatus::from_cached(Some(&cached), day(8));
        assert_eq!(s, EntitlementStatus::free());
    }

    // Claim checks.

    #[test]
    fn no_account_is_free() {
        assert_eq!(
            EntitlementStatus::from_cached(None, day(0)),
            EntitlementStatus::free()
        );
    }

    #[test]
    fn rejects_foreign_issuer_audience_and_long_lifetime() {
        let mut c = claims(day(0), 2, 0);
        c.iss = "https://tsudev.com".into();
        assert_eq!(
            EntitlementStatus::from_claims(&c, day(0)),
            Err(ClaimsRejected::WrongIssuer)
        );
        let mut c = claims(day(0), 2, 0);
        c.aud = "qulythuchi".into();
        assert_eq!(
            EntitlementStatus::from_claims(&c, day(0)),
            Err(ClaimsRejected::WrongAudience)
        );
        let mut c = claims(day(0), 2, 0);
        c.exp = (day(0) + Duration::days(8)).unix_timestamp();
        assert_eq!(
            EntitlementStatus::from_claims(&c, day(0)),
            Err(ClaimsRejected::BadLifetime)
        );
    }

    #[test]
    fn pro_lapses_when_pro_until_passes() {
        let c = claims(day(0), 1, day(1).unix_timestamp());
        assert!(
            EntitlementStatus::from_claims(&c, day(0))
                .unwrap()
                .pro_active
        );
        assert!(
            !EntitlementStatus::from_claims(&c, day(2))
                .unwrap()
                .pro_active
        );
    }

    #[test]
    fn installation_id_is_22_chars_and_random() {
        let a = new_installation_id().unwrap();
        let b = new_installation_id().unwrap();
        assert!(is_installation_id(&a), "{a}");
        assert_ne!(a, b);
        assert!(!is_installation_id("short"));
        assert!(!is_installation_id("AAAAAAAAAAAAAAAAAAAAAB")); // stray low bits
    }

    #[test]
    fn base64url_matches_rfc4648_vectors() {
        assert_eq!(base64url(b"f"), "Zg");
        assert_eq!(base64url(b"fo"), "Zm8");
        assert_eq!(base64url(b"foo"), "Zm9v");
        assert_eq!(base64url(&[0xfb, 0xff]), "-_8");
    }
}

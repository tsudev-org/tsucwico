//! Renewal reminder and "Pro has expired" notice (PRODUCT_POLICY 7.6).
//!
//! Pro is a one-year term that never renews by itself. In the last 7 days
//! the app reminds the user once per app open; once the term has passed it
//! says so exactly once per `pro_until`. Both rules read `pro_until` from a
//! signature-checked, unexpired entitlement JWS only: with no such token the
//! app is free and shows nothing (7.6, first paragraph).

use super::EntitlementClaims;
use serde::Serialize;
use time::{Duration, OffsetDateTime};

/// Reminder window before `pro_until`.
pub const RENEWAL_WINDOW: Duration = Duration::days(7);

/// Coming back to the app after at least this long in the background counts
/// as a new "open" (7.6).
pub const REOPEN_AFTER: Duration = Duration::minutes(30);

/// How long the cloud copy stays downloadable after Pro lapses (7.2).
pub const CLOUD_GRACE: Duration = Duration::days(90);

/// Local key holding the `pro_until` (Unix seconds) the expiry notice was
/// last shown for - the name is part of the shared contract (7.6).
pub const EXPIRED_NOTIFIED_KEY: &str = "tsu.pro_expired.notified_until";

/// Whether this product syncs anything to the cloud yet. Settings sync
/// arrives with phase P3 (ADR-0002); until then the expiry notice drops the
/// sentence about the cloud copy, as 7.6 requires of products without one.
pub const HAS_CLOUD_SYNC: bool = false;

/// What to show at this app open, if anything.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RenewalNotice {
    /// `days_left` is rounded up and at least 1.
    #[serde(rename_all = "camelCase")]
    Renew { days_left: u32, pro_until: i64 },
    /// `cloud_delete_at` is `pro_until` + 90 days, or `None` when the product
    /// has no cloud copy to speak of.
    #[serde(rename_all = "camelCase")]
    Expired {
        pro_until: i64,
        cloud_delete_at: Option<i64>,
    },
}

/// Whether becoming active now is a new app open: a cold start
/// (`last_active` = `None`), or a return after >= 30 minutes away.
pub fn is_new_open(last_active: Option<OffsetDateTime>, now: OffsetDateTime) -> bool {
    last_active.is_none_or(|at| now - at >= REOPEN_AFTER || at > now)
}

/// The notice due at an app open.
///
/// - `claims`: the current entitlement, already verified *and* unexpired -
///   pass `None` when there is no valid token (offline with a stale cache
///   included); the result is then always `None`.
/// - `seen_pro`: this device has held a token with Pro active before (the
///   "JWS trước đó của thiết bị từng có Pro" condition of 7.6).
/// - `notified_until`: the stored [`EXPIRED_NOTIFIED_KEY`].
/// - `is_open`: [`is_new_open`] for this activation.
pub fn renewal_notice(
    claims: Option<&EntitlementClaims>,
    seen_pro: bool,
    notified_until: Option<i64>,
    is_open: bool,
    now: OffsetDateTime,
) -> Option<RenewalNotice> {
    let pro_until = claims?.pro_until;
    if pro_until <= 0 {
        return None;
    }
    let left = pro_until - now.unix_timestamp();
    if left > 0 {
        if !is_open || left > RENEWAL_WINDOW.whole_seconds() {
            return None;
        }
        let days = (left + 86_399) / 86_400;
        return Some(RenewalNotice::Renew {
            days_left: u32::try_from(days.max(1)).unwrap_or(u32::MAX),
            pro_until,
        });
    }
    // Expired: once per pro_until, and only if this device ever saw Pro -
    // signing in to an account whose Pro lapsed years ago is not news.
    if !seen_pro || notified_until == Some(pro_until) {
        return None;
    }
    Some(RenewalNotice::Expired {
        pro_until,
        cloud_delete_at: HAS_CLOUD_SYNC.then(|| pro_until + CLOUD_GRACE.whole_seconds()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pro::{Badge, EntitlementStatus, CLIENT_ID, ISSUER};

    const DAY: i64 = 86_400;

    fn t(secs: i64) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(1_800_000_000 + secs).unwrap()
    }

    /// A token issued at `now`, valid 7 days, with Pro until `pro_until`.
    fn jws(now: OffsetDateTime, pro_until: OffsetDateTime) -> EntitlementClaims {
        EntitlementClaims {
            iss: ISSUER.into(),
            sub: "u1".into(),
            aud: CLIENT_ID.into(),
            iat: now.unix_timestamp(),
            exp: now.unix_timestamp() + 7 * DAY,
            pro_until: pro_until.unix_timestamp(),
            pro_source: Some("web".into()),
            donor: false,
            badge: Badge::Pro,
            devices_30d: 1,
        }
    }

    /// Mirrors what the app does: only a verified, unexpired token counts.
    fn valid(c: &EntitlementClaims, now: OffsetDateTime) -> Option<&EntitlementClaims> {
        EntitlementStatus::from_claims(c, now).ok().map(|_| c)
    }

    // The six mandatory cases of PRODUCT_POLICY 7.6.

    #[test]
    fn eight_days_left_no_reminder() {
        let c = jws(t(0), t(8 * DAY));
        assert_eq!(
            renewal_notice(valid(&c, t(0)), true, None, true, t(0)),
            None
        );
    }

    #[test]
    fn seven_days_left_reminds() {
        let c = jws(t(0), t(7 * DAY));
        assert_eq!(
            renewal_notice(valid(&c, t(0)), true, None, true, t(0)),
            Some(RenewalNotice::Renew {
                days_left: 7,
                pro_until: c.pro_until
            })
        );
        // Rounded up: 6 days and one second is still "7 days".
        assert_eq!(
            renewal_notice(valid(&c, t(DAY - 1)), true, None, true, t(DAY - 1)),
            Some(RenewalNotice::Renew {
                days_left: 7,
                pro_until: c.pro_until
            })
        );
    }

    #[test]
    fn two_opens_remind_twice_a_quick_return_does_not() {
        let c = jws(t(0), t(5 * DAY));
        // Cold start, then the user leaves and comes back an hour later.
        let first = is_new_open(None, t(0));
        let second = is_new_open(Some(t(0)), t(3600));
        let quick = is_new_open(Some(t(3600)), t(3600 + 29 * 60));
        assert!(first && second && !quick);
        assert!(renewal_notice(valid(&c, t(0)), true, None, first, t(0)).is_some());
        assert!(renewal_notice(valid(&c, t(3600)), true, None, second, t(3600)).is_some());
        assert_eq!(
            renewal_notice(valid(&c, t(5400)), true, None, quick, t(5400)),
            None
        );
    }

    #[test]
    fn expiry_is_told_exactly_once() {
        let end = t(10 * DAY);
        let c = jws(t(11 * DAY), end); // fresh token after the term ended
        let now = t(11 * DAY);
        let first = renewal_notice(valid(&c, now), true, None, true, now);
        assert_eq!(
            first,
            Some(RenewalNotice::Expired {
                pro_until: end.unix_timestamp(),
                cloud_delete_at: None
            })
        );
        // The app stores pro_until under EXPIRED_NOTIFIED_KEY; the next open is quiet.
        let stored = Some(end.unix_timestamp());
        assert_eq!(
            renewal_notice(valid(&c, now), true, stored, true, now),
            None
        );
        // A device that never saw Pro is not told either.
        assert_eq!(renewal_notice(valid(&c, now), false, None, true, now), None);
    }

    #[test]
    fn renewing_stops_the_reminder_and_rearms_the_expiry_notice() {
        let old_end = t(10 * DAY);
        let stored = Some(old_end.unix_timestamp());
        // Renewed: the new token runs a year past the old end.
        let new_end = old_end + Duration::days(365);
        let renewed = jws(t(9 * DAY), new_end);
        assert_eq!(
            renewal_notice(valid(&renewed, t(9 * DAY)), true, stored, true, t(9 * DAY)),
            None
        );
        // A year on, the new term lapses: a different pro_until, so told again.
        let later = new_end + Duration::days(1);
        let lapsed = jws(later, new_end);
        assert!(matches!(
            renewal_notice(valid(&lapsed, later), true, stored, true, later),
            Some(RenewalNotice::Expired { .. })
        ));
    }

    #[test]
    fn offline_with_an_expired_token_says_nothing_and_does_not_fail() {
        let c = jws(t(0), t(5 * DAY));
        let now = t(8 * DAY); // token exp passed, no network to refresh it
        assert_eq!(valid(&c, now), None);
        assert_eq!(renewal_notice(valid(&c, now), true, None, true, now), None);
    }

    #[test]
    fn never_had_pro_says_nothing() {
        let c = jws(t(0), t(0));
        let mut c = c;
        c.pro_until = 0;
        assert_eq!(renewal_notice(Some(&c), true, None, true, t(0)), None);
    }
}

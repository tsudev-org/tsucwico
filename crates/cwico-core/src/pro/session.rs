//! What the Account section may say about the session (AUTH_AND_ACCOUNT
//! 14.2 and 14.4.2.5). Pure, so the rule is tested on any host.

/// Where the sign-in stands, from what is on this machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Session {
    SignedOut,
    /// A refresh token is held and nothing says it is stale.
    SignedIn,
    /// The gateway refused the refresh token, or the last answer it gave is
    /// older than the 7 days a token may live offline. The app is a guest
    /// again and says so in the Account section - no dialog.
    Expired,
}

/// * `has_token` - a refresh token is stored.
/// * `has_cached` - an entitlement token is cached.
/// * `cached_valid` - it still verifies and has not expired.
/// * `refused` - the gateway rejected the refresh token earlier.
pub fn session(has_token: bool, has_cached: bool, cached_valid: bool, refused: bool) -> Session {
    match (has_token, has_cached, cached_valid) {
        (true, _, true) => Session::SignedIn,
        // Signed in but the entitlement was never fetched (the network
        // dropped right after sign-in): not an expiry.
        (true, false, false) => Session::SignedIn,
        (true, true, false) => Session::Expired,
        (false, ..) if refused => Session::Expired,
        (false, ..) => Session::SignedOut,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_four_inputs_decide() {
        use Session::*;
        assert_eq!(session(false, false, false, false), SignedOut);
        assert_eq!(session(true, true, true, false), SignedIn);
        assert_eq!(session(true, false, false, false), SignedIn);
        // Offline for longer than the cached token lives.
        assert_eq!(session(true, true, false, false), Expired);
        // The gateway said invalid_grant.
        assert_eq!(session(false, false, false, true), Expired);
        // A cache without its refresh token never signs anyone in.
        assert_eq!(session(false, true, true, false), SignedOut);
    }
}

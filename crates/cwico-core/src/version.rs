//! The release-name ⇄ semver mapping.
//!
//! From 19/11/2026 releases follow tsudev-standards VERSIONING.md 2.2 (v3):
//! the version is `YY.M.DDNN` and the tag is `v` + that version. The semver
//! Cargo, the MSI bundler and the updater use carries day and release-of-day
//! counter in its patch field; semver forbids a leading zero, so days 1-9 drop
//! it there and only there:
//!
//! ```text
//! patch = day × 100 + n            (n starts at 1)
//!
//! v26.11.1901   <->  26.11.1901     (shown to users as 26.11.1901)
//! v26.11.1902   <->  26.11.1902
//! v26.12.0301   <->  26.12.301      (shown to users as 26.12.0301)
//! ```
//!
//! Older tags keep parsing - `tsudev-cwico-v26.8.19` (26.8.1901) and
//! `tsudev-cwico-v26.11.10.2` (26.11.1002) are what installed copies were
//! built from - but nothing new is named with them.
//!
//! `tools/version.py` implements the same mapping for the build tooling. Both
//! are tested against the shared vectors in `data/version-cases.json`, so
//! changing one without the other fails a test in the language that was not
//! changed.

use crate::error::{Error, Result};

/// Machine name of the product (Tsu Cwico), and the prefix of every release name.
pub const PRODUCT: &str = "tsucwico";

/// The prefix releases carried up to 26.11.1002, before the product was renamed
/// Tsu Cwico (docs/adr/0001). Old tags and CHANGELOG headings still use it, so
/// it keeps parsing; nothing new is ever named with it.
pub const LEGACY_PRODUCT: &str = "tsudev-cwico";

/// At most 99 releases in a day, because the counter shares the patch field
/// with the day number.
pub const MAX_RELEASES_PER_DAY: u32 = 99;

/// A release, decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Release {
    /// Two-digit year, as in the release name: 26 for 2026.
    pub year: u32,
    pub month: u32,
    pub day: u32,
    /// Which release of that day this is, from 1.
    pub counter: u32,
}

impl Release {
    /// Parse a release name: the tag `v26.11.1901`, the bare `26.11.1901`,
    /// and every older form (`tsudev-cwico-v26.8.19`, `tsucwico-v26.11.10.2`).
    pub fn parse_name(name: &str) -> Result<Self> {
        let trimmed = name.trim();
        let rest = trimmed
            .strip_prefix(&format!("{PRODUCT}-"))
            .or_else(|| trimmed.strip_prefix(&format!("{LEGACY_PRODUCT}-")))
            .unwrap_or(trimmed);
        let rest = rest.strip_prefix('v').unwrap_or(rest);

        let parts: Vec<&str> = rest.split('.').collect();
        if parts.len() < 3 || parts.len() > 4 {
            return Err(Error::other(format!(
                "`{name}` is not a release name; expected vYY.M.DDNN"
            )));
        }
        // A 3-4 digit third field is the current DDNN form; a 1-2 digit one is
        // the old day-only form, optionally followed by a counter.
        let current = parts.len() == 3 && (3..=4).contains(&parts[2].len());
        if parts.len() == 3 && parts[2].len() > 4 {
            return Err(Error::other(format!(
                "`{name}` is not a release name; expected vYY.M.DDNN"
            )));
        }

        let number = |index: usize| -> Result<u32> {
            parts[index]
                .parse::<u32>()
                .map_err(|_| Error::other(format!("`{name}`: `{}` is not a number", parts[index])))
        };

        let year = number(0)?;
        let month = number(1)?;
        let (day, counter) = if current {
            let patch = number(2)?;
            (patch / 100, patch % 100)
        } else {
            (number(2)?, if parts.len() == 4 { number(3)? } else { 1 })
        };

        Self::validate(year, month, day, counter, name)?;
        Ok(Self {
            year,
            month,
            day,
            counter,
        })
    }

    /// Decode the semver a build carries back into a release.
    pub fn parse_semver(semver: &str) -> Result<Self> {
        let parts: Vec<&str> = semver.trim().split('.').collect();
        if parts.len() != 3 {
            return Err(Error::other(format!(
                "`{semver}` is not a three-component semver"
            )));
        }
        let number = |index: usize| -> Result<u32> {
            parts[index].parse::<u32>().map_err(|_| {
                Error::other(format!("`{semver}`: `{}` is not a number", parts[index]))
            })
        };

        let year = number(0)?;
        let month = number(1)?;
        let patch = number(2)?;
        let day = patch / 100;
        let counter = patch % 100;

        if counter == 0 {
            return Err(Error::other(format!(
                "`{semver}`: patch {patch} decodes to release-of-day 0, but the \
                 counter starts at 1 - this version was not produced by this scheme"
            )));
        }
        Self::validate(year, month, day, counter, semver)?;
        Ok(Self {
            year,
            month,
            day,
            counter,
        })
    }

    fn validate(year: u32, month: u32, day: u32, counter: u32, source: &str) -> Result<()> {
        let _ = year;
        if !(1..=12).contains(&month) {
            return Err(Error::other(format!(
                "`{source}`: month {month} is out of range"
            )));
        }
        if !(1..=31).contains(&day) {
            return Err(Error::other(format!(
                "`{source}`: day {day} is out of range"
            )));
        }
        if !(1..=MAX_RELEASES_PER_DAY).contains(&counter) {
            return Err(Error::other(format!(
                "`{source}`: release counter {counter} is outside 1..{MAX_RELEASES_PER_DAY}"
            )));
        }
        Ok(())
    }

    /// The semver Cargo, the MSI and the updater use.
    pub fn to_semver(self) -> String {
        format!(
            "{}.{}.{}",
            self.year,
            self.month,
            self.day * 100 + self.counter
        )
    }

    /// The git tag: `v26.11.1901`.
    pub fn to_name(self) -> String {
        format!("v{}", self.display())
    }

    /// The version users see (About screen, release title, file names):
    /// `26.11.1901`, `26.12.0301`.
    pub fn display(self) -> String {
        format!(
            "{}.{}.{:02}{:02}",
            self.year, self.month, self.day, self.counter
        )
    }

    /// `2026-08-19`, for display next to a release.
    pub fn iso_date(self) -> String {
        format!("20{:02}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// Convenience: semver string to the version users see (`26.11.1901`).
///
/// Falls back to the input unchanged when it was not produced by this scheme,
/// so a stray version never turns into an error dialog in the UI.
pub fn name_for_semver(semver: &str) -> String {
    Release::parse_semver(semver)
        .map(Release::display)
        .unwrap_or_else(|_| semver.to_string())
}

/// This build's version as users see it (`26.11.1901`).
pub fn current_release_name() -> String {
    name_for_semver(crate::VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Cases {
        round_trip: Vec<RoundTrip>,
        ascending_order: Vec<String>,
        rejected_names: Vec<String>,
        rejected_semvers: Vec<String>,
        legacy_names: Vec<RoundTrip>,
        updater_order: Vec<String>,
    }

    #[derive(Deserialize)]
    struct RoundTrip {
        name: String,
        semver: String,
    }

    /// The same vectors `tools/test_version.py` uses. Changing the mapping in
    /// one language without the other fails here.
    fn cases() -> Cases {
        serde_json::from_str(include_str!("../../../data/version-cases.json"))
            .expect("data/version-cases.json must parse")
    }

    #[test]
    fn round_trips_every_shared_case() {
        for case in cases().round_trip {
            let release = Release::parse_name(&case.name).unwrap_or_else(|e| {
                panic!("`{}` should parse: {e}", case.name);
            });
            assert_eq!(release.to_semver(), case.semver, "name -> semver");
            assert_eq!(
                Release::parse_semver(&case.semver).unwrap().to_name(),
                case.name,
                "semver -> name"
            );
        }
    }

    #[test]
    fn semver_ordering_matches_release_order() {
        // The updater compares these to decide whether a user is out of date,
        // so a mapping that round-trips but sorts wrongly would silently stop
        // delivering updates.
        let semvers: Vec<(u32, u32, u32)> = cases()
            .ascending_order
            .iter()
            .map(|name| {
                let r = Release::parse_name(name).unwrap();
                (r.year, r.month, r.day * 100 + r.counter)
            })
            .collect();

        let mut sorted = semvers.clone();
        sorted.sort_unstable();
        assert_eq!(
            semvers, sorted,
            "release order is not preserved by the mapping"
        );
    }

    #[test]
    fn the_tenth_release_of_a_day_sorts_above_the_second() {
        // Naive string comparison gets this wrong: "…19.10" < "…19.2".
        let tenth = Release::parse_name("v26.8.1910").unwrap();
        let second = Release::parse_name("v26.8.1902").unwrap();
        assert!(tenth.to_semver() > second.to_semver() || tenth > second);
        assert!(tenth.day * 100 + tenth.counter > second.day * 100 + second.counter);
    }

    #[test]
    fn rejects_what_the_shared_cases_say_it_should() {
        for name in cases().rejected_names {
            assert!(
                Release::parse_name(&name).is_err(),
                "`{name}` should have been rejected"
            );
        }
        for semver in cases().rejected_semvers {
            assert!(
                Release::parse_semver(&semver).is_err(),
                "semver `{semver}` should have been rejected"
            );
        }
    }

    #[test]
    fn release_names_from_before_the_rename_still_parse() {
        for case in cases().legacy_names {
            let release = Release::parse_name(&case.name).unwrap();
            assert_eq!(release.to_semver(), case.semver, "{}", case.name);
            assert_eq!(release.to_name(), format!("v{}", release.display()), "{}", case.name);
        }
    }

    #[test]
    fn accepts_the_short_forms_a_maintainer_would_type() {
        for input in ["v26.8.1901", "26.8.1901", "tsucwico-v26.8.19", "v26.8.19", "26.8.19"] {
            assert_eq!(Release::parse_name(input).unwrap().to_semver(), "26.8.1901");
        }
    }

    #[test]
    fn an_unrecognised_version_degrades_instead_of_failing() {
        // A stray version must not become an error dialog in the UI.
        assert_eq!(name_for_semver("1.0.0"), "1.0.0");
    }

    #[test]
    fn this_build_reports_a_valid_release_name() {
        let name = current_release_name();
        Release::parse_name(&name)
            .unwrap_or_else(|e| panic!("this build's own version does not parse: {e}"));
    }

    #[test]
    fn iso_date_is_rendered_for_display() {
        let r = Release::parse_name("v26.8.1901").unwrap();
        assert_eq!(r.iso_date(), "2026-08-19");
    }

    #[test]
    fn days_one_to_nine_keep_the_zero_users_see_but_not_in_semver() {
        let r = Release::parse_name("v26.12.0301").unwrap();
        assert_eq!(r.to_semver(), "26.12.301");
        assert_eq!(r.display(), "26.12.0301");
        assert_eq!(name_for_semver("26.12.301"), "26.12.0301");
    }

    #[test]
    fn installed_builds_see_the_next_release_as_newer() {
        // VERSIONING.md 2.4: the Tauri updater in 26.8.1901 and 26.11.1002
        // compares `semver::Version`s. Each release must be strictly newer
        // than everything before it, whatever its tag looks like.
        let order = cases().updater_order;
        let parsed: Vec<semver::Version> = order
            .iter()
            .map(|s| semver::Version::parse(s).unwrap())
            .collect();
        for pair in parsed.windows(2) {
            assert!(pair[1] > pair[0], "{} must be newer than {}", pair[1], pair[0]);
        }
        let next = Release::parse_semver(order.last().unwrap()).unwrap();
        assert_eq!(Release::parse_name(&next.to_name()).unwrap(), next);
    }
}

//! Which release first shipped each migration (`migrations/RELEASED`).
//!
//! Two things rest on it: a released migration is never edited (the hashes
//! are checked by a test), and a database from a newer Homeport is refused
//! with the name of the version to install.

/// One line of `migrations/RELEASED`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Released {
    /// The migration's file name, such as `0004_registries.sql`.
    pub file: &'static str,
    /// The SHA-256 of the file as it was released, in lower-case hex.
    pub sha256: &'static str,
    /// The first release that shipped it, such as `0.7.0`.
    pub release: &'static str,
}

const RELEASED: &str = include_str!("migrations/RELEASED");

/// The released migrations, in the order they run.
pub fn released() -> impl Iterator<Item = Released> {
    RELEASED
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let mut fields = line.split_whitespace();
            let mut field = || {
                fields
                    .next()
                    .unwrap_or_else(|| panic!("RELEASED: three fields expected in {line:?}"))
            };
            Released {
                file: field(),
                sha256: field(),
                release: field(),
            }
        })
}

/// The oldest Homeport that can open a database whose schema is at
/// migration `version`: the release that shipped that migration, or this
/// build's own version while the migration is yet to be released.
pub fn release_of(version: usize) -> &'static str {
    version
        .checked_sub(1)
        .and_then(|index| released().nth(index))
        .map_or(env!("CARGO_PKG_VERSION"), |m| m.release)
}

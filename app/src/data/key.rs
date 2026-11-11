//! What a cached resource is filed under: a path of parts, read from the
//! widest to the narrowest (`host`, its id, `containers`, one's id…). A key
//! that starts with another is in its scope, so one invalidation of
//! `keys::host::scope(id)` reaches everything that Host has.

use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Key(Vec<String>);

impl Key {
    /// The key made of `parts`, as `["host", id, "containers"]`.
    pub fn new<P: Into<String>>(parts: impl IntoIterator<Item = P>) -> Self {
        Self(parts.into_iter().map(Into::into).collect())
    }

    /// This key with `part` added: what a feature files under a shared list
    /// (`host::images(id).with("search").with(term)`).
    #[must_use]
    pub fn with(mut self, part: impl Into<String>) -> Self {
        self.0.push(part.into());
        self
    }

    /// This key with `part` added, or an empty part for `None`: a filter
    /// that may be absent (every Namespace) still has its place in the key.
    #[must_use]
    pub fn with_optional(self, part: Option<&str>) -> Self {
        self.with(part.unwrap_or_default())
    }

    /// Whether this key is `scope` or is filed under it.
    pub fn is_in(&self, scope: &Self) -> bool {
        self.0.starts_with(&scope.0)
    }

    pub fn parts(&self) -> &[String] {
        &self.0
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key({})", self.0.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_key_is_in_the_scopes_it_starts_with() {
        let scope = Key::new(["host", "h1"]);
        let containers = scope.clone().with("containers");
        assert!(containers.is_in(&scope));
        assert!(containers.is_in(&containers));
        assert!(scope.is_in(&scope));
        assert!(!scope.is_in(&containers));
        assert!(!containers.is_in(&Key::new(["host", "h2"])));
        // A part is whole: `h1` is not under `h`.
        assert!(!containers.is_in(&Key::new(["host", "h"])));
    }

    #[test]
    fn an_absent_filter_keeps_its_place() {
        let jobs = Key::new(["nomad", "c1", "jobs"]);
        assert_eq!(
            jobs.clone().with_optional(None).parts(),
            ["nomad", "c1", "jobs", ""]
        );
        assert_eq!(
            jobs.with_optional(Some("default")).parts(),
            ["nomad", "c1", "jobs", "default"]
        );
    }
}

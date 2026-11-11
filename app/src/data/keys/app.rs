//! Keys of what belongs to the app itself, not to a Host or a Cluster.

use crate::data::Key;

/// The user's settings, as they are stored.
pub fn settings() -> Key {
    Key::new(["settings"])
}

/// Homeport's version and where it keeps its files.
pub fn info() -> Key {
    Key::new(["app-info"])
}

/// What this machine has that Homeport uses (compose, helm, git…).
pub fn capabilities() -> Key {
    Key::new(["capabilities"])
}

/// Whether an update is published.
pub fn updates() -> Key {
    Key::new(["updates"])
}

/// Whether the OS starts Homeport at login.
pub fn start_at_login() -> Key {
    Key::new(["start-at-login"])
}

/// The licences of what Homeport is built from.
pub fn licenses() -> Key {
    Key::new(["licenses"])
}

/// The diagnostics bundle's summary, with or without names.
pub fn diagnostics(include_names: bool) -> Key {
    Key::new(["diagnostics"]).with(include_names.to_string())
}

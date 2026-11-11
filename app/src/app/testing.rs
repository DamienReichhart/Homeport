//! An [`AppState`] for tests: a new database in a temporary folder, secrets
//! sealed with a random key instead of the OS keychain, and the receiving
//! half of its events.

use tempfile::TempDir;
use tokio::sync::mpsc::{self, UnboundedReceiver};

use homeport_storage::crypto::{random_key, SecretBox};

use super::{
    events::{Event, Events},
    setup, AppState,
};

/// A started app with no Host; dropping it removes its folder.
pub struct TestApp {
    pub state: AppState,
    pub events: UnboundedReceiver<Event>,
    pub dir: TempDir,
}

/// Builds a [`TestApp`]. Must run inside a Tokio runtime (`#[tokio::test]`).
pub fn app() -> TestApp {
    let dir = tempfile::tempdir().expect("a temporary folder");
    let (sender, events) = mpsc::unbounded_channel();
    let state = setup::state_in(
        dir.path().join("config"),
        dir.path().join("data"),
        &Events::new(sender),
        |_| SecretBox::from_key(&random_key()),
    );
    let Ok(state) = state else {
        panic!("the test app's state couldn't be built");
    };
    TestApp { state, events, dir }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn a_test_app_starts_with_no_host() {
        let app = super::app();
        assert!(app.state.hosts.store.list().unwrap().is_empty());
    }
}

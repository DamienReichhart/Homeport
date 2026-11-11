use homeport_core::error::ErrorCode;

use super::*;
use crate::app::testing;

/// A Host the app has never heard of.
pub(super) const NO_HOST: &str = "no-such-host";

#[track_caller]
pub(super) fn not_connected<T>(result: AppResult<T>) {
    assert_eq!(
        result.err().map(|e| e.code),
        Some(ErrorCode::HostNotConnected)
    );
}

/// A token that never expires, named `homeport`, for an Incus at
/// `127.0.0.1:1`, where nothing listens.
const TOKEN: &str = "eyJjbGllbnRfbmFtZSI6ImhvbWVwb3J0IiwiZmluZ2VycHJpbnQiOiI4RDZBMUYwQTNDIiwiYWRkcmVzc2VzIjpbIjEyNy4wLjAuMToxIl0sInNlY3JldCI6IjVlMWMwZiIsImV4cGlyZXNfYXQiOiIwMDAxLTAxLTAxVDAwOjAwOjAwWiJ9";

#[test]
fn a_pasted_token_is_summarised() {
    let summary = incus_trust_token_read(format!("  {TOKEN}\n")).unwrap();
    assert_eq!(
        summary,
        TrustTokenSummary {
            client_name: "homeport".into(),
            fingerprint: "8d6a1f0a3c".into(),
            addresses: vec!["127.0.0.1:1".into()],
            expires_at: None,
        }
    );
}

#[test]
fn a_garbled_token_is_refused() {
    let e = incus_trust_token_read("not a token".into()).unwrap_err();
    assert_eq!(e.code, ErrorCode::IncusTokenInvalid);
    assert!(e.hint.is_some());
}

#[tokio::test]
async fn a_garbled_token_adds_no_host() {
    let app = testing::app();
    let added = incus_trust_add(&app.state, "lab".into(), "not a token".into()).await;
    assert_eq!(added.unwrap_err().code, ErrorCode::IncusTokenInvalid);
    assert!(app.state.hosts.store.list().unwrap().is_empty());
}

#[tokio::test]
async fn an_incus_that_does_not_answer_adds_no_host() {
    let app = testing::app();
    let added = incus_trust_add(&app.state, "  ".into(), TOKEN.into()).await;
    let e = added.unwrap_err();
    assert_eq!(e.code, ErrorCode::IncusNotListening, "{e:?}");
    assert!(app.state.hosts.store.list().unwrap().is_empty());
}

#[tokio::test]
async fn projects_need_a_connected_host() {
    let app = testing::app();
    not_connected(incus_projects(&app.state, NO_HOST.into()).await);
}

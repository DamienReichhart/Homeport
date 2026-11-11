use homeport_core::error::ErrorCode;

use super::*;
use crate::app::testing::app;

const IMAGE: &str = "ghcr.io/acme/web:1";

fn draft(address: &str, username: &str, secret: Option<&str>) -> RegistryDraft {
    RegistryDraft {
        address: address.into(),
        username: username.into(),
        secret: secret.map(Into::into),
    }
}

fn refused() -> AppError {
    AppError::new(
        ErrorCode::RegistryAuthRequired,
        homeport_registries::messages::PULL_REFUSED,
    )
}

#[tokio::test]
async fn a_login_is_saved_changed_and_removed() {
    let app = app();
    assert!(registries_list(&app.state).unwrap().is_empty());

    let saved = registries_save(&app.state, None, draft("ghcr.io", "ada", Some("s3cret"))).unwrap();
    assert_eq!(saved.address, "ghcr.io");
    assert_eq!(
        registries_list(&app.state).unwrap(),
        std::slice::from_ref(&saved)
    );

    // No new secret: the stored one is kept.
    let renamed = registries_save(
        &app.state,
        Some(saved.id.clone()),
        draft("ghcr.io", "grace", None),
    )
    .unwrap();
    assert_eq!(renamed.id, saved.id);
    assert_eq!(renamed.username, "grace");
    let auth = auth_for(&app.state, IMAGE).unwrap().unwrap();
    assert_eq!(
        (auth.username.as_str(), auth.secret.as_str()),
        ("grace", "s3cret")
    );

    registries_remove(&app.state, saved.id.clone()).unwrap();
    assert!(registries_list(&app.state).unwrap().is_empty());
    let gone = registries_remove(&app.state, saved.id).unwrap_err();
    assert_eq!(gone.code, ErrorCode::NotFound);
}

#[tokio::test]
async fn a_login_needs_a_secret_and_a_free_address() {
    let app = app();
    let empty = registries_save(&app.state, None, draft("ghcr.io", "ada", None)).unwrap_err();
    assert_eq!(empty.code, ErrorCode::Validation);

    registries_save(&app.state, None, draft("ghcr.io", "ada", Some("one"))).unwrap();
    let twice =
        registries_save(&app.state, None, draft("ghcr.io", "bob", Some("two"))).unwrap_err();
    assert_eq!(twice.code, ErrorCode::NameInUse);

    let unknown = registries_save(
        &app.state,
        Some("nope".into()),
        draft("quay.io", "ada", None),
    )
    .unwrap_err();
    assert_eq!(unknown.code, ErrorCode::NotFound);
}

#[tokio::test]
async fn the_login_of_a_reference_is_found() {
    let app = app();
    assert_eq!(
        registries_login_for(&app.state, IMAGE.into()).unwrap(),
        None
    );
    assert!(auth_for(&app.state, IMAGE).unwrap().is_none());
    assert!(logins(&app.state).unwrap().all().is_empty());

    let saved = registries_save(&app.state, None, draft("ghcr.io", "ada", Some("s3cret"))).unwrap();
    assert_eq!(
        registries_login_for(&app.state, IMAGE.into()).unwrap(),
        Some(saved)
    );
    assert_eq!(
        registries_login_for(&app.state, "nginx".into()).unwrap(),
        None
    );

    let logins = logins(&app.state).unwrap();
    assert_eq!(logins.all().len(), 1);
    assert_eq!(logins.for_reference(IMAGE).unwrap().secret, "s3cret");
    assert!(logins.for_reference("nginx").is_none());
}

/// Only what is refused before any request is checked: nothing here may
/// depend on reaching a registry.
#[tokio::test]
async fn a_check_resolves_the_login_first() {
    let app = app();
    let no_secret = registries_check(&app.state, None, draft("ghcr.io", "ada", None))
        .await
        .unwrap_err();
    assert_eq!(no_secret.code, ErrorCode::Validation);

    let unknown = registries_check(
        &app.state,
        Some("nope".into()),
        draft("ghcr.io", "ada", None),
    )
    .await
    .unwrap_err();
    assert_eq!(unknown.code, ErrorCode::NotFound);
}

#[tokio::test]
async fn tags_are_only_listed_for_docker_hub() {
    let app = app();
    let elsewhere = registries_hub_tags(&app.state, IMAGE.into(), 1, String::new())
        .await
        .unwrap_err();
    assert_eq!(elsewhere.code, ErrorCode::Validation);
}

#[tokio::test]
async fn a_refusal_points_at_the_stored_login() {
    let app = app();
    let anonymous = explain(&app.state, IMAGE, refused());
    assert_eq!(anonymous.code, ErrorCode::RegistryAuthRequired);
    assert!(anonymous.hint.is_some());
    // Built before the login exists, as a stream's would be.
    let pull = explainer(&app.state, IMAGE);
    let push = push_explainer(&app.state, IMAGE);

    registries_save(&app.state, None, draft("ghcr.io", "ada", Some("s3cret"))).unwrap();
    let known = explain(&app.state, IMAGE, refused());
    assert_ne!(known.message, anonymous.message);
    assert!(known.message.contains("ada"));
    assert_eq!(pull(refused()).message, anonymous.message);

    let pushed = push_explainer(&app.state, IMAGE)(refused());
    assert!(pushed.message.contains("ada"));
    assert_ne!(push(refused()).message, pushed.message);
}

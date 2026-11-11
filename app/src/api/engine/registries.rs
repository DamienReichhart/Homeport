//! The stored registry logins, as the Engine takes them, and the hints that
//! point a registry's refusal at the login that would fix it.

use homeport_core::error::{AppError, AppResult};
use homeport_engine::auth::{Logins, RegistryAuth};
use homeport_registries::{hints, Credentials};

use crate::app::AppState;

/// The stored login for the registry of `reference`, as the Engine takes it.
pub(crate) fn auth_for(state: &AppState, reference: &str) -> AppResult<Option<RegistryAuth>> {
    Ok(state.registries.credentials_for(reference)?.map(to_auth))
}

/// Every stored login, and the rule that matches them to image references,
/// for runs that pull several images (Stack deploys, builds).
pub(crate) fn logins(state: &AppState) -> AppResult<Logins> {
    Ok(Logins::new(
        state
            .registries
            .all_credentials()?
            .into_iter()
            .map(to_auth)
            .collect(),
        homeport_registries::registry_of,
    ))
}

/// Rewrites a registry refusal for `reference` to point at the login that
/// would fix it.
pub(crate) fn explain(state: &AppState, reference: &str, error: AppError) -> AppError {
    explainer(state, reference)(error)
}

/// [`explainer`] for a push.
pub(crate) fn push_explainer(
    state: &AppState,
    reference: &str,
) -> impl Fn(AppError) -> AppError + Send + Sync + 'static {
    let login = state.registries.login_for(reference).ok().flatten();
    let reference = reference.to_owned();
    move |error| hints::explain_push(error, &reference, login.as_ref())
}

/// [`explain`] for a stream that outlives the use case, which can't keep `state`.
pub(crate) fn explainer(
    state: &AppState,
    reference: &str,
) -> impl Fn(AppError) -> AppError + Send + Sync + 'static {
    let login = state.registries.login_for(reference).ok().flatten();
    let reference = reference.to_owned();
    move |error| hints::explain(error, &reference, login.as_ref())
}

fn to_auth(c: Credentials) -> RegistryAuth {
    RegistryAuth {
        address: c.address,
        username: c.username,
        secret: c.secret,
    }
}

#[cfg(test)]
mod tests {
    use homeport_core::error::{ErrorCode, Message};
    use homeport_registries::RegistryDraft;

    use super::*;
    use crate::app::testing::app;

    const IMAGE: &str = "ghcr.io/acme/app:1";

    const REFUSED: Message = Message {
        key: "tests.refused",
        en: "refused",
    };
    const GONE: Message = Message {
        key: "tests.gone",
        en: "gone",
    };

    fn refused() -> AppError {
        AppError::new(ErrorCode::RegistryAuthRequired, REFUSED)
    }

    #[tokio::test]
    async fn an_image_with_no_stored_login_is_pulled_anonymously() {
        let app = app();
        assert!(auth_for(&app.state, IMAGE).unwrap().is_none());
        assert!(logins(&app.state).unwrap().all().is_empty());
        // The hint then says to add a login.
        assert!(explain(&app.state, IMAGE, refused()).hint.is_some());
    }

    #[tokio::test]
    async fn a_stored_login_goes_to_the_engine_for_its_registry_only() {
        let app = app();
        let draft = RegistryDraft {
            address: "ghcr.io".into(),
            username: "ada".into(),
            secret: Some("s3cret".into()),
        };
        app.state.registries.save(None, draft).unwrap();

        let auth = auth_for(&app.state, IMAGE).unwrap().expect("a login");
        assert_eq!(
            (auth.username.as_str(), auth.secret.as_str()),
            ("ada", "s3cret")
        );
        assert!(auth_for(&app.state, "quay.io/acme/app:1")
            .unwrap()
            .is_none());
        assert_eq!(logins(&app.state).unwrap().all().len(), 1);
    }

    #[tokio::test]
    async fn only_a_registry_refusal_is_explained() {
        let app = app();
        let pull = explainer(&app.state, IMAGE);
        let push = push_explainer(&app.state, IMAGE);
        assert_ne!(pull(refused()).message, "refused");
        assert_ne!(push(refused()).message, "refused");
        let other = || AppError::not_found(GONE);
        assert_eq!(pull(other()).message, "gone");
        assert_eq!(push(other()).message, "gone");
    }
}

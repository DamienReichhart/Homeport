//! Points a pull or push the registry refused to the login that would fix
//! it, in **Settings → Registries**. The Engine's own classification
//! (`homeport_core::error::engine`) can't know whether Homeport had a login.

use homeport_core::error::{AppError, ErrorCode, Text};

use crate::messages as msg;

use crate::{address, Registry};

/// Rewrites a registry refusal for `reference` knowing which stored login,
/// if any, was sent with it. Other errors pass through untouched.
pub fn explain(error: AppError, reference: &str, login: Option<&Registry>) -> AppError {
    let registry = address::registry_of(reference);
    let (message, hint): (Text, Text) = match (error.code, login) {
        (ErrorCode::RegistryAuthRequired, None) => (
            msg::PULL_REFUSED.into(),
            msg::PULL_REFUSED_HINT.with("registry", &registry),
        ),
        (ErrorCode::RegistryAuthRequired, Some(login)) => (
            msg::PULL_REFUSED_LOGIN.with("username", &login.username),
            msg::PULL_REFUSED_LOGIN_HINT.with("registry", &registry),
        ),
        (ErrorCode::RegistryLoginFailed, Some(login)) => (
            msg::LOGIN_REFUSED
                .with("registry", &registry)
                .with("username", &login.username),
            msg::LOGIN_REFUSED_HINT.with("registry", &registry),
        ),
        _ => return error,
    };
    error.with_message(message).with_hint(hint)
}

/// [`explain`] for a push: the registry refusing means the login is missing
/// or may not push to that repository, not that the image is missing.
pub fn explain_push(error: AppError, reference: &str, login: Option<&Registry>) -> AppError {
    let registry = address::registry_of(reference);
    let (message, hint): (Text, Text) = match (error.code, login) {
        (ErrorCode::RegistryAuthRequired, None) => (
            msg::PUSH_REFUSED.with("registry", &registry),
            msg::PUSH_REFUSED_HINT.with("registry", &registry),
        ),
        (ErrorCode::RegistryAuthRequired, Some(login)) => (
            msg::PUSH_REFUSED_LOGIN
                .with("registry", &registry)
                .with("username", &login.username),
            msg::PUSH_REFUSED_LOGIN_HINT.into(),
        ),
        _ => return explain(error, reference, login),
    };
    error.with_message(message).with_hint(hint)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn denied() -> AppError {
        AppError::new(ErrorCode::RegistryAuthRequired, msg::PULL_REFUSED).with_detail("raw")
    }

    fn login() -> Registry {
        Registry {
            id: "r1".into(),
            address: "ghcr.io".into(),
            username: "alice".into(),
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    #[test]
    fn without_a_login_the_hint_says_to_add_one() {
        let e = explain(denied(), "ghcr.io/org/app:1", None);
        assert!(e.hint.unwrap().text.contains("add a login for ghcr.io"));
        assert_eq!(e.detail.as_deref(), Some("raw"));
    }

    #[test]
    fn with_a_login_the_hint_says_to_check_it() {
        let e = explain(denied(), "ghcr.io/org/app:1", Some(&login()));
        assert!(e.message.contains("alice"));
        assert!(e.hint.unwrap().text.contains("the login for ghcr.io"));

        let refused = AppError::new(ErrorCode::RegistryLoginFailed, msg::LOGIN_FAILED);
        let e = explain(refused, "ghcr.io/org/app:1", Some(&login()));
        assert!(e
            .hint
            .unwrap()
            .text
            .contains("password or token for ghcr.io"));
    }

    #[test]
    fn docker_hub_images_name_docker_io() {
        let e = explain(denied(), "someone/private", None);
        assert!(e.hint.unwrap().text.contains("docker.io"));
    }

    #[test]
    fn refused_pushes_name_the_login() {
        let e = explain_push(denied(), "ghcr.io/org/app:1", None);
        assert_eq!(e.message, "ghcr.io refused the push.");
        assert!(e.hint.unwrap().text.contains("Add a login for ghcr.io"));
        let e = explain_push(denied(), "ghcr.io/org/app:1", Some(&login()));
        assert!(e.message.contains("from alice"));
    }

    #[test]
    fn other_errors_pass_through() {
        let e = AppError::new(ErrorCode::DiskFull, msg::HTTPS_CLIENT);
        assert_eq!(explain(e.clone(), "nginx", None), e);
    }
}

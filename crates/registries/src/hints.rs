//! Points a pull or push the registry refused to the login that would fix
//! it, in **Settings → Registries**. The Engine's own classification
//! (`homeport_core::error::engine`) can't know whether Homeport had a login.

use homeport_core::error::{AppError, ErrorCode, Hint};

use crate::{address, Registry};

/// Rewrites a registry refusal for `reference` knowing which stored login,
/// if any, was sent with it. Other errors pass through untouched.
pub fn explain(error: AppError, reference: &str, login: Option<&Registry>) -> AppError {
    let registry = address::registry_of(reference);
    let (message, hint) = match (error.code, login) {
        (ErrorCode::RegistryAuthRequired, None) => (
            "The image doesn't exist, or it's private.".to_owned(),
            format!(
                "Check the image name. If it's private, add a login for {registry} in Settings → Registries."
            ),
        ),
        (ErrorCode::RegistryAuthRequired, Some(login)) => (
            format!("The image doesn't exist, or {} can't access it.", login.username),
            format!(
                "Check the image name, and the login for {registry} in Settings → Registries."
            ),
        ),
        (ErrorCode::RegistryLoginFailed, Some(login)) => (
            format!("{registry} refused the stored login of {}.", login.username),
            format!("Check the password or token for {registry} in Settings → Registries."),
        ),
        _ => return error,
    };
    AppError {
        message,
        hint: Some(Hint {
            text: hint,
            commands: vec![],
        }),
        ..error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn denied() -> AppError {
        AppError::new(ErrorCode::RegistryAuthRequired, "denied").with_detail("raw")
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

        let refused = AppError::new(ErrorCode::RegistryLoginFailed, "refused");
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
    fn other_errors_pass_through() {
        let e = AppError::new(ErrorCode::DiskFull, "full");
        assert_eq!(explain(e.clone(), "nginx", None), e);
    }
}

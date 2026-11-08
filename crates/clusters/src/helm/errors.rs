//! Explaining helm's failures: missing or outdated, or what it printed on
//! stderr when it exited with an error.

use homeport_core::error::{AppError, ErrorCode};

use crate::{messages as msg, proxy::Failure};

/// No usable `helm`, and how to install it on this OS.
pub fn missing() -> AppError {
    let (hint, command) = install();
    AppError::new(ErrorCode::HelmMissing, msg::HELM_MISSING)
        .with_hint(hint)
        .with_command(command)
}

/// Helm 2, which Homeport doesn't drive.
pub(super) fn outdated(version: &str) -> AppError {
    let (hint, command) = install();
    AppError::new(
        ErrorCode::HelmOutdated,
        msg::HELM_OUTDATED.with("version", version),
    )
    .with_hint(hint)
    .with_command(command)
}

fn install() -> (homeport_core::error::Message, &'static str) {
    if cfg!(target_os = "macos") {
        (msg::HELM_INSTALL_MACOS_HINT, "brew install helm")
    } else if cfg!(windows) {
        (msg::HELM_INSTALL_WINDOWS_HINT, "winget install Helm.Helm")
    } else {
        (
            msg::HELM_INSTALL_LINUX_HINT,
            "sudo snap install helm --classic",
        )
    }
}

/// Why `helm <verb>` failed, run against the Cluster `cluster` if any:
/// it couldn't start, or what it printed on stderr when it exited.
pub fn failure(verb: &str, cluster: Option<&str>, failure: Failure<'_>) -> AppError {
    let (code, stderr) = match failure {
        Failure::Spawn(e) if e.kind() == std::io::ErrorKind::NotFound => return missing(),
        Failure::Spawn(e) => {
            return AppError::new(ErrorCode::HelmFailed, msg::HELM_SPAWN_FAILED)
                .with_hint(msg::HELM_SPAWN_FAILED_HINT)
                .with_cause(e)
        }
        Failure::Exit { code, stderr } => (code, stderr),
    };
    let name = cluster.unwrap_or_default();
    let lower = stderr.to_lowercase();
    let error = if lower.contains("another operation (install/upgrade/rollback) is in progress") {
        AppError::new(ErrorCode::HelmReleaseLocked, msg::HELM_LOCKED)
            .with_hint(msg::HELM_LOCKED_HINT)
    } else if lower.contains("cannot re-use a name that is still in use") {
        AppError::new(ErrorCode::NameInUse, msg::HELM_NAME_IN_USE)
            .with_hint(msg::HELM_NAME_IN_USE_HINT)
    } else if lower.contains("repository name") && lower.contains("already exists") {
        AppError::new(ErrorCode::NameInUse, msg::HELM_REPO_NAME_IN_USE)
            .with_hint(msg::HELM_REPO_NAME_IN_USE_HINT)
    } else if lower.contains("is not a valid chart repository or cannot be reached") {
        AppError::new(ErrorCode::HelmFailed, msg::HELM_REPO_UNREACHABLE)
            .with_hint(msg::HELM_REPO_UNREACHABLE_HINT)
    } else if lower.contains("no repo named") || lower.contains("no repositories") {
        AppError::new(ErrorCode::NotFound, msg::HELM_REPO_NOT_FOUND)
            .with_hint(msg::HELM_REPO_NOT_FOUND_HINT)
    } else if lower.contains("release: not found") || lower.contains("has no deployed releases") {
        AppError::not_found(msg::HELM_RELEASE_NOT_FOUND).with_hint(msg::HELM_RELEASE_NOT_FOUND_HINT)
    } else if lower.contains("chart \"") && lower.contains("not found")
        || lower.contains("no chart version found")
        || lower.contains("no chart name found")
        || lower.contains("failed to fetch") && lower.contains("not found")
        || lower.contains(": not found") && lower.contains("oci://")
    {
        AppError::new(ErrorCode::NotFound, msg::HELM_CHART_NOT_FOUND)
            .with_hint(msg::HELM_CHART_NOT_FOUND_HINT)
    } else if lower.contains("error converting yaml to json")
        || lower.contains("cannot unmarshal")
        || lower.contains("failed to parse") && lower.contains("yaml")
    {
        AppError::validation(msg::HELM_VALUES_INVALID).with_hint(msg::HELM_VALUES_INVALID_HINT)
    } else if lower.contains("forbidden") {
        AppError::new(ErrorCode::Forbidden, msg::HELM_FORBIDDEN.with("name", name))
            .with_hint(msg::HELM_FORBIDDEN_HINT)
    } else if lower.contains("timed out waiting for the condition")
        || lower.contains("context deadline exceeded")
    {
        AppError::new(ErrorCode::Timeout, msg::HELM_TIMEOUT).with_hint(msg::HELM_TIMEOUT_HINT)
    } else if lower.contains("kubernetes cluster unreachable") {
        AppError::new(
            ErrorCode::ClusterNotConnected,
            msg::HELM_UNREACHABLE.with("name", name),
        )
        .with_hint(msg::HELM_UNREACHABLE_HINT)
    } else {
        let message = match code {
            Some(code) => msg::HELM_FAILED.with("verb", verb).with("code", code),
            None => msg::HELM_STOPPED.with("verb", verb),
        };
        AppError::new(ErrorCode::HelmFailed, message).with_hint(msg::HELM_FAILED_HINT)
    };
    if stderr.is_empty() {
        error
    } else {
        error.with_detail(stderr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exit(stderr: &str) -> AppError {
        failure(
            "upgrade",
            Some("kind-dev"),
            Failure::Exit {
                code: Some(1),
                stderr,
            },
        )
    }

    #[test]
    fn helms_errors_are_told_apart() {
        for (stderr, code) in [
            ("Error: UPGRADE FAILED: another operation (install/upgrade/rollback) is in progress", ErrorCode::HelmReleaseLocked),
            ("Error: INSTALLATION FAILED: cannot re-use a name that is still in use", ErrorCode::NameInUse),
            ("Error: repository name (bitnami) already exists, please specify a different name", ErrorCode::NameInUse),
            ("Error: looks like \"https://nope.example\" is not a valid chart repository or cannot be reached: Get \"https://nope.example/index.yaml\": dial tcp: lookup nope.example: no such host", ErrorCode::HelmFailed),
            ("Error: no repo named \"bitnami\" found", ErrorCode::NotFound),
            ("Error: release: not found", ErrorCode::NotFound),
            ("Error: INSTALLATION FAILED: chart \"nginxx\" matching  not found in bitnami index. (try 'helm repo update'): no chart name found", ErrorCode::NotFound),
            ("Error: INSTALLATION FAILED: failed to download \"bitnami/nginx\" at version \"99.0.0\"\nchart \"nginx\" version \"99.0.0\" not found in https://charts.bitnami.com/bitnami repository", ErrorCode::NotFound),
            ("Error: failed to parse values.yaml: error converting YAML to JSON: yaml: line 3: mapping values are not allowed in this context", ErrorCode::Validation),
            ("Error: list: failed to list: secrets is forbidden: User \"dev\" cannot list resource \"secrets\" in API group \"\" at the cluster scope", ErrorCode::Forbidden),
            ("Error: UPGRADE FAILED: context deadline exceeded", ErrorCode::Timeout),
            ("Error: INSTALLATION FAILED: Kubernetes cluster unreachable: Get \"http://127.0.0.1:1/version\": dial tcp", ErrorCode::ClusterNotConnected),
            ("Error: something new", ErrorCode::HelmFailed),
        ] {
            let e = exit(stderr);
            assert_eq!(e.code, code, "{stderr}");
            assert_eq!(e.detail.as_deref(), Some(stderr));
        }
    }

    #[test]
    fn a_helm_that_cant_start_is_missing_or_failed() {
        let gone = std::io::Error::from(std::io::ErrorKind::NotFound);
        assert!(failure("list", None, Failure::Spawn(&gone)).is(ErrorCode::HelmMissing));
        let denied = std::io::Error::from(std::io::ErrorKind::PermissionDenied);
        assert!(failure("list", None, Failure::Spawn(&denied)).is(ErrorCode::HelmFailed));
    }

    #[test]
    fn other_failures_name_the_verb_and_exit() {
        assert_eq!(
            exit("Error: something new").message,
            "helm upgrade exited with code 1."
        );
    }
}

//! Classifies failures of the `docker compose` CLI: missing or outdated,
//! unable to start, or exiting with an error described on stderr.

use homeport_core::error::{engine, AppError, ErrorCode, Message};

use crate::messages as msg;

/// Why no usable compose CLI was found, given what `version --short` printed
/// for the Docker CLI plugin and the standalone `docker-compose`, if anything.
pub(super) fn missing(docker_cli_installed: bool, standalone_version: Option<&str>) -> AppError {
    if let Some(version) = standalone_version {
        return AppError::new(
            ErrorCode::ComposeOutdated,
            msg::COMPOSE_OUTDATED.with("version", version),
        )
        .with_hint(install_hint(docker_cli_installed))
        .with_command(install_command(docker_cli_installed));
    }
    AppError::new(
        ErrorCode::ComposeMissing,
        if docker_cli_installed {
            msg::COMPOSE_PLUGIN_MISSING
        } else {
            msg::COMPOSE_MISSING
        },
    )
    .with_hint(install_hint(docker_cli_installed))
    .with_command(install_command(docker_cli_installed))
}

fn install_hint(docker_cli_installed: bool) -> Message {
    match (cfg!(target_os = "linux"), docker_cli_installed) {
        (true, true) => msg::INSTALL_PLUGIN_HINT,
        (true, false) => msg::INSTALL_CLI_HINT,
        (false, _) if cfg!(target_os = "macos") => msg::INSTALL_MACOS_HINT,
        _ => msg::INSTALL_WINDOWS_HINT,
    }
}

fn install_command(docker_cli_installed: bool) -> &'static str {
    if cfg!(target_os = "linux") {
        if docker_cli_installed {
            "sudo apt-get install docker-compose-plugin"
        } else {
            "sudo apt-get install docker-ce-cli docker-compose-plugin"
        }
    } else if cfg!(target_os = "macos") {
        "brew install docker docker-compose"
    } else {
        "winget install Docker.DockerDesktop"
    }
}

/// The CLI couldn't even be started.
pub(super) fn spawn_failed(program: &str, error: &std::io::Error) -> AppError {
    AppError::new(
        ErrorCode::ComposeSpawnFailed,
        msg::SPAWN_FAILED.with("program", program),
    )
    .with_hint(match error.kind() {
        std::io::ErrorKind::NotFound => msg::SPAWN_FAILED_NOT_FOUND_HINT,
        std::io::ErrorKind::PermissionDenied => msg::SPAWN_FAILED_PERMISSION_HINT,
        _ => msg::SPAWN_FAILED_HINT,
    })
    .with_cause(error)
}

/// Why compose exited with `code` while working on `host`, from the end of its stderr.
pub(super) fn exit_failure(
    host: &str,
    verb: &str,
    code: Option<i32>,
    stderr: &[String],
) -> AppError {
    let text = stderr.join("\n");
    let lower = text.to_lowercase();
    let exit = code.map_or("was stopped".to_owned(), |c| {
        format!("exited with code {c}")
    });
    let failed = match code {
        Some(c) => msg::EXITED.with("verb", verb).with("code", c),
        None => msg::STOPPED.with("verb", verb),
    };

    let classified = if let Some(name) = missing_variable(&text) {
        AppError::new(
            ErrorCode::ComposeMissingVariable,
            msg::MISSING_VARIABLE.with("name", name),
        )
        .with_hint(msg::MISSING_VARIABLE_HINT.with("name", name))
    } else if [
        "yaml:",
        "validating",
        "additional property",
        "is not allowed",
        "must be a mapping",
        "invalid compose project",
        "no service selected",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        AppError::new(ErrorCode::ComposeInvalidDefinition, msg::INVALID_DEFINITION)
            .with_hint(msg::INVALID_DEFINITION_HINT)
    } else if lower.contains("cannot connect to the docker daemon")
        || lower.contains("error during connect")
    {
        AppError::new(
            ErrorCode::ConnectionLost,
            msg::ENGINE_UNREACHABLE.with("host", host),
        )
        .with_hint(msg::ENGINE_UNREACHABLE_HINT.with("host", host))
    } else {
        let from_engine = engine::from_message(None, &text);
        if is_specific(from_engine.code) {
            from_engine
        } else {
            AppError::new(ErrorCode::ComposeFailed, failed).with_hint(msg::FAILED_HINT)
        }
    };
    classified.with_detail(if text.is_empty() {
        format!("docker compose {verb} {exit} without writing an error")
    } else {
        text
    })
}

/// Engine classifications precise enough to replace "compose failed".
fn is_specific(code: ErrorCode) -> bool {
    !matches!(
        code,
        ErrorCode::EngineError
            | ErrorCode::BadRequest
            | ErrorCode::Forbidden
            | ErrorCode::Conflict
            | ErrorCode::NotFound
            | ErrorCode::PathNotFound
            | ErrorCode::AlreadyDone
            | ErrorCode::ContainerNotRunning
    )
}

/// `X` in "required variable X is missing a value".
fn missing_variable(text: &str) -> Option<&str> {
    let start = text.find("required variable ")? + "required variable ".len();
    let name = text[start..].split_whitespace().next()?;
    Some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(text: &str) -> Vec<String> {
        text.lines().map(str::to_owned).collect()
    }

    #[test]
    fn an_old_standalone_compose_is_outdated() {
        let e = missing(false, Some("1.29.2"));
        assert_eq!(e.code, ErrorCode::ComposeOutdated);
        assert!(e.message.contains("1.29.2"));
        assert_eq!(e.hint.unwrap().commands.len(), 1);
    }

    #[test]
    fn a_docker_cli_without_the_plugin_says_so() {
        let e = missing(true, None);
        assert_eq!(e.code, ErrorCode::ComposeMissing);
        assert!(e.message.contains("compose plugin"));
    }

    #[test]
    fn stderr_explains_the_failure() {
        for (stderr, code) in [
            ("yaml: line 3: mapping values are not allowed in this context", ErrorCode::ComposeInvalidDefinition),
            ("validating -: services.web Additional property imagee is not allowed", ErrorCode::ComposeInvalidDefinition),
            ("error while interpolating services.web.environment.[]: required variable DB_PASSWORD is missing a value: set it", ErrorCode::ComposeMissingVariable),
            (" Container shop-web-1  Starting\nError response from daemon: driver failed programming external connectivity on endpoint shop-web-1 (abc): Bind for 0.0.0.0:8080 failed: port is already allocated", ErrorCode::PortInUse),
            ("Error response from daemon: pull access denied for nope, repository does not exist or may require 'docker login'", ErrorCode::RegistryAuthRequired),
            ("Cannot connect to the Docker daemon at unix:///x.sock. Is the docker daemon running?", ErrorCode::ConnectionLost),
            ("something unexpected", ErrorCode::ComposeFailed),
        ] {
            let e = exit_failure("prod", "up", Some(1), &lines(stderr));
            assert_eq!(e.code, code, "{stderr}");
            assert_eq!(e.detail.as_deref(), Some(stderr));
        }
    }

    #[test]
    fn missing_variables_are_named() {
        let e = exit_failure(
            "prod",
            "up",
            Some(1),
            &lines("required variable DB_PASSWORD is missing a value: set it"),
        );
        assert_eq!(e.message, "The variable DB_PASSWORD has no value.");
    }

    #[test]
    fn unexplained_failures_give_the_exit_code() {
        let e = exit_failure("prod", "down", Some(17), &[]);
        assert_eq!(e.message, "docker compose down exited with code 17.");
    }
}

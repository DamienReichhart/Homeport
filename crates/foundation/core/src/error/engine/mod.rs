//! Classifies the Docker Engine's answers: HTTP statuses and the well-known
//! messages behind them (name clashes, busy ports, full disks, registries…).
//! The same text shows up in pull progress and in compose's stderr, so
//! [`from_message`] is shared with them.

use super::{reach, AppError, ErrorCode};
use crate::messages as msg;

impl From<bollard::errors::Error> for AppError {
    fn from(e: bollard::errors::Error) -> Self {
        from_bollard(&e)
    }
}

/// Turns a bollard failure into something the user can act on.
pub fn from_bollard(error: &bollard::errors::Error) -> AppError {
    use bollard::errors::Error;
    // TLS and SSH dialers classify their own failures.
    if let Some(classified) = reach::classified_cause(error) {
        return classified.clone();
    }
    match error {
        Error::DockerResponseServerError {
            status_code,
            message,
        } => from_message(Some(*status_code), message)
            .with_detail(format!("HTTP {status_code}: {message}")),
        Error::DockerStreamError { error: message } => {
            from_message(None, message).with_detail(message)
        }
        Error::RequestTimeoutError => AppError::new(ErrorCode::Timeout, msg::ENGINE_TIMEOUT)
            .with_hint(msg::ENGINE_TIMEOUT_HINT)
            .with_cause(error),
        Error::JsonDataError { .. } | Error::JsonSerdeError { .. } => {
            AppError::new(ErrorCode::EngineError, msg::ENGINE_UNREADABLE)
                .with_hint(msg::ENGINE_UNREADABLE_HINT)
                .with_cause(error)
        }
        Error::APIVersionParseError {} => AppError::new(
            ErrorCode::ApiVersionUnsupported,
            msg::API_VERSION_UNREADABLE,
        )
        .with_cause(error),
        _ => match reach::io_cause(error).map(std::io::Error::kind) {
            Some(std::io::ErrorKind::NotFound | std::io::ErrorKind::ConnectionRefused) => {
                AppError::new(ErrorCode::EngineNotRunning, msg::ENGINE_STOPPED)
                    .with_hint(msg::ENGINE_STOPPED_HINT)
                    .with_cause(error)
            }
            Some(_) => reach::engine_connection_lost().with_cause(error),
            None => AppError::new(ErrorCode::EngineError, msg::REQUEST_FAILED).with_cause(error),
        },
    }
}

/// Classifies an Engine error message, with its HTTP status when known.
/// The caller adds the raw text as detail.
pub fn from_message(status: Option<u16>, message: &str) -> AppError {
    let m = message.to_lowercase();
    let has = |needle: &str| m.contains(needle);

    if has("port is already allocated") || has("address already in use") {
        let what = match port_in(message) {
            Some(port) => msg::PORT_IN_USE.with("port", port),
            None => msg::PUBLISHED_PORT_IN_USE.into(),
        };
        return AppError::new(ErrorCode::PortInUse, what).with_hint(msg::PORT_IN_USE_HINT);
    }
    if has("no space left on device") {
        return AppError::new(ErrorCode::DiskFull, msg::ENGINE_DISK_FULL)
            .with_hint(msg::ENGINE_DISK_FULL_HINT);
    }
    if has("toomanyrequests") || has("pull rate limit") || has("429 too many requests") {
        return AppError::new(ErrorCode::RegistryRateLimited, msg::RATE_LIMITED)
            .with_hint(msg::RATE_LIMITED_HINT);
    }
    if has("incorrect username or password") {
        return AppError::new(ErrorCode::RegistryLoginFailed, msg::LOGIN_FAILED)
            .with_hint(msg::LOGIN_FAILED_HINT);
    }
    if has("pull access denied")
        || has("push access denied")
        || has("no basic auth credentials")
        || has("requested access to the resource is denied")
        || has("authentication required")
        || has("unauthorized: ")
    {
        return AppError::new(ErrorCode::RegistryAuthRequired, msg::AUTH_REQUIRED)
            .with_hint(msg::AUTH_REQUIRED_HINT);
    }
    if has("manifest unknown") || (has("manifest for") && has("not found")) {
        return AppError::new(ErrorCode::ImageNotFound, msg::IMAGE_OR_TAG_MISSING)
            .with_hint(msg::IMAGE_OR_TAG_MISSING_HINT);
    }
    if has("no such image") {
        return AppError::new(ErrorCode::ImageNotFound, msg::IMAGE_GONE);
    }
    if (has("dial tcp")
        || has("i/o timeout")
        || has("tls handshake timeout")
        || has("no such host"))
        && (has("registry") || has("/v2/") || has("https://"))
    {
        return AppError::new(ErrorCode::RegistryUnreachable, msg::REGISTRY_UNREACHABLE)
            .with_hint(msg::REGISTRY_UNREACHABLE_HINT);
    }
    if has("invalid reference format") {
        return AppError::new(ErrorCode::BadRequest, msg::INVALID_IMAGE_NAME)
            .with_hint(msg::INVALID_IMAGE_NAME_HINT);
    }
    if has("client sent an http request to an https server") {
        return AppError::new(ErrorCode::TlsRequired, msg::TLS_REQUIRED)
            .with_hint(msg::TLS_REQUIRED_HINT);
    }
    if has("client version") && (has("too new") || has("too old"))
        || has("minimum supported api version")
    {
        return AppError::new(
            ErrorCode::ApiVersionUnsupported,
            msg::API_VERSION_UNSUPPORTED,
        )
        .with_hint(msg::API_VERSION_UNSUPPORTED_HINT);
    }
    if has("could not select device driver") {
        return AppError::new(ErrorCode::GpuUnavailable, msg::GPU_UNAVAILABLE)
            .with_hint(msg::GPU_UNAVAILABLE_HINT);
    }
    if has("range of cpus is from") {
        let what = match cpus_in(message) {
            Some(1) => msg::CPUS_LIMITED_ONE.with("cpus", 1),
            Some(cpus) => msg::CPUS_LIMITED.with("cpus", cpus),
            None => msg::CPUS_FEWER.into(),
        };
        return AppError::new(ErrorCode::BadRequest, what).with_hint(msg::CPUS_LIMITED_HINT);
    }
    if has("update out of sequence") {
        return AppError::new(ErrorCode::Conflict, msg::OUT_OF_SEQUENCE)
            .with_hint(msg::OUT_OF_SEQUENCE_HINT);
    }
    if has("is already in use by container") || has("conflict. the container name") {
        return AppError::new(ErrorCode::NameInUse, msg::CONTAINER_NAME_IN_USE)
            .with_hint(msg::CONTAINER_NAME_IN_USE_HINT);
    }
    if has("volume is in use") {
        return AppError::new(ErrorCode::InUse, msg::VOLUME_IN_USE)
            .with_hint(msg::VOLUME_IN_USE_HINT);
    }
    if has("has active endpoints") {
        return AppError::new(ErrorCode::InUse, msg::NETWORK_IN_USE)
            .with_hint(msg::NETWORK_IN_USE_HINT);
    }
    if has("referenced in multiple repositories") {
        return AppError::new(ErrorCode::InUse, msg::IMAGE_SEVERAL_TAGS)
            .with_hint(msg::IMAGE_SEVERAL_TAGS_HINT);
    }
    if has("image is being used by") || has("unable to remove repository reference") {
        return AppError::new(ErrorCode::InUse, msg::IMAGE_IN_USE)
            .with_hint(msg::IMAGE_IN_USE_HINT);
    }
    if has("container is running") || has("cannot remove a running container") {
        return AppError::new(ErrorCode::ContainerRunning, msg::CONTAINER_RUNNING)
            .with_hint(msg::CONTAINER_RUNNING_HINT);
    }
    if has("is not running") {
        return AppError::new(ErrorCode::ContainerNotRunning, msg::CONTAINER_NOT_RUNNING)
            .with_hint(msg::CONTAINER_NOT_RUNNING_HINT);
    }
    if has("is already paused") || has("is not paused") || status == Some(304) {
        return AppError::new(ErrorCode::AlreadyDone, msg::ALREADY_IN_STATE);
    }
    if has("executable file not found") {
        return command_not_found();
    }
    if has("could not find the file") || has("no such file or directory") {
        return AppError::new(ErrorCode::PathNotFound, msg::PATH_MISSING);
    }
    if has("no such container") {
        return AppError::not_found(msg::CONTAINER_GONE).with_hint(msg::CONTAINER_GONE_HINT);
    }
    if has("no such volume") {
        return AppError::not_found(msg::VOLUME_GONE);
    }
    if has("network") && has("not found") {
        return AppError::not_found(msg::NETWORK_GONE);
    }

    // Unrecognised answers: the Engine's own words stay in the detail.
    match status {
        Some(400) => {
            AppError::new(ErrorCode::BadRequest, msg::BAD_REQUEST).with_hint(msg::FAILED_HINT)
        }
        Some(401 | 403) => {
            AppError::new(ErrorCode::Forbidden, msg::FORBIDDEN).with_hint(msg::FORBIDDEN_HINT)
        }
        Some(404) => AppError::not_found(msg::GONE).with_hint(msg::GONE_HINT),
        Some(409) => AppError::new(ErrorCode::Conflict, msg::CONFLICT).with_hint(msg::FAILED_HINT),
        _ => AppError::new(ErrorCode::EngineError, msg::FAILED).with_hint(msg::FAILED_HINT),
    }
}

/// A command that doesn't exist in the container, often a shell missing from
/// a minimal image.
pub fn command_not_found() -> AppError {
    AppError::new(ErrorCode::NoShell, msg::COMMAND_NOT_FOUND).with_hint(msg::COMMAND_NOT_FOUND_HINT)
}

/// The CPU count in "Range of CPUs is from 0.01 to 4.00, as there are only 4 CPUs available".
fn cpus_in(message: &str) -> Option<u32> {
    let rest = &message[message.find("only ")? + "only ".len()..];
    rest.split(' ').next()?.parse().ok()
}

/// The host port in "Bind for 0.0.0.0:8080 failed" or "listen tcp4 0.0.0.0:80: bind…".
fn port_in(message: &str) -> Option<u16> {
    [
        "Bind for ",
        "listen tcp ",
        "listen tcp4 ",
        "listen tcp6 ",
        "listen udp ",
    ]
    .iter()
    .find_map(|marker| {
        let rest = &message[message.find(marker)? + marker.len()..];
        let address = rest.split([' ', ',']).next()?.trim_end_matches(':');
        address.rsplit(':').next()?.parse().ok()
    })
}

#[cfg(test)]
mod tests;

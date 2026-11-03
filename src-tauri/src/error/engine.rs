//! Classifies the Docker Engine's answers: HTTP statuses and the well-known
//! messages behind them (name clashes, busy ports, full disks, registries…).
//! The same text shows up in pull progress and in compose's stderr, so
//! [`from_message`] is shared with them.

use super::{reach, AppError, ErrorCode};

/// Turns a bollard failure into something the user can act on.
pub fn from_bollard(error: &bollard::errors::Error) -> AppError {
    use bollard::errors::Error;
    match error {
        Error::DockerResponseServerError {
            status_code,
            message,
        } => from_message(Some(*status_code), message)
            .with_detail(format!("HTTP {status_code}: {message}")),
        Error::DockerStreamError { error: message } => {
            from_message(None, message).with_detail(message)
        }
        Error::RequestTimeoutError => AppError::new(
            ErrorCode::Timeout,
            "The Docker Engine didn't answer in time.",
        )
        .with_hint("It may be busy. Try again in a moment.")
        .with_cause(error),
        Error::JsonDataError { .. } | Error::JsonSerdeError { .. } => AppError::new(
            ErrorCode::EngineError,
            "The Docker Engine sent an answer Homeport can't read.",
        )
        .with_hint("The Engine may be much newer or older than Homeport supports. Updating both usually helps.")
        .with_cause(error),
        Error::APIVersionParseError {} => AppError::new(
            ErrorCode::ApiVersionUnsupported,
            "The Docker Engine reported an API version Homeport can't read.",
        )
        .with_cause(error),
        _ => match reach::io_cause(error).map(std::io::Error::kind) {
            Some(std::io::ErrorKind::NotFound | std::io::ErrorKind::ConnectionRefused) => {
                AppError::new(
                    ErrorCode::EngineNotRunning,
                    "The Docker Engine stopped answering.",
                )
                .with_hint("Check that it's still running. Homeport reconnects on its own.")
                .with_cause(error)
            }
            Some(_) => reach::connection_lost("the Docker Engine").with_cause(error),
            None => AppError::new(ErrorCode::EngineError, "The Docker Engine request failed.")
                .with_cause(error),
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
            Some(port) => format!("Port {port} is already used on the host."),
            None => "A published port is already used on the host.".to_owned(),
        };
        return AppError::new(ErrorCode::PortInUse, what).with_hint(
            "Stop the container or program that uses it, or publish this one on another port.",
        );
    }
    if has("no space left on device") {
        return AppError::new(ErrorCode::DiskFull, "The host is out of disk space.").with_hint(
            "Remove unused images, containers and volumes (Prune on the Overview page), or free space on the server.",
        );
    }
    if has("toomanyrequests") || has("pull rate limit") {
        return AppError::new(
            ErrorCode::RegistryRateLimited,
            "The registry's pull limit was reached.",
        )
        .with_hint("Docker Hub limits anonymous pulls. Wait for the limit to reset (up to 6 hours), or pull while logged in from a terminal:")
        .with_command("docker login");
    }
    if has("pull access denied")
        || has("requested access to the resource is denied")
        || has("authentication required")
        || has("unauthorized: ")
    {
        return AppError::new(
            ErrorCode::RegistryAuthRequired,
            "The image doesn't exist, or it's private.",
        )
        .with_hint("Check the image name. Homeport can't use registry logins yet, so pull private images from a terminal on the server:")
        .with_command("docker login <registry>\ndocker pull <image>");
    }
    if has("manifest unknown") || (has("manifest for") && has("not found")) {
        return AppError::new(ErrorCode::ImageNotFound, "This image or tag doesn't exist.")
            .with_hint("Check the image name and tag on the registry.");
    }
    if has("no such image") {
        return AppError::new(
            ErrorCode::ImageNotFound,
            "This image isn't on the host anymore.",
        );
    }
    if (has("dial tcp")
        || has("i/o timeout")
        || has("tls handshake timeout")
        || has("no such host"))
        && (has("registry") || has("/v2/") || has("https://"))
    {
        return AppError::new(
            ErrorCode::RegistryUnreachable,
            "The host can't reach the image registry.",
        )
        .with_hint("Check the server's internet access, DNS and proxy settings.");
    }
    if has("client sent an http request to an https server") {
        return AppError::new(
            ErrorCode::TlsRequired,
            "This Docker Engine only accepts TLS connections.",
        )
        .with_hint("Change the host's connection to TLS and add its certificates.");
    }
    if has("client version") && (has("too new") || has("too old"))
        || has("minimum supported api version")
    {
        return AppError::new(
            ErrorCode::ApiVersionUnsupported,
            "This Docker Engine's API version isn't supported.",
        )
        .with_hint("Update the Docker Engine on the host.");
    }
    if has("is already in use by container") || has("conflict. the container name") {
        return AppError::new(ErrorCode::NameInUse, "That container name is already used.")
            .with_hint("Pick another name, or remove or rename the other container first.");
    }
    if has("volume is in use") {
        return AppError::new(ErrorCode::InUse, "This volume is used by a container.")
            .with_hint("Remove the containers that use it first.");
    }
    if has("has active endpoints") {
        return AppError::new(
            ErrorCode::InUse,
            "This network still has containers attached.",
        )
        .with_hint("Disconnect or remove its containers first.");
    }
    if has("referenced in multiple repositories") {
        return AppError::new(ErrorCode::InUse, "This image has several tags.")
            .with_hint("Force the removal to delete every tag, or remove the tags one by one.");
    }
    if has("image is being used by") || has("unable to remove repository reference") {
        return AppError::new(ErrorCode::InUse, "This image is used by a container.")
            .with_hint("Remove the containers that use it, or force the removal.");
    }
    if has("container is running") || has("cannot remove a running container") {
        return AppError::new(ErrorCode::ContainerRunning, "The container is running.")
            .with_hint("Stop it first, or remove it with force.");
    }
    if has("is not running") {
        return AppError::new(
            ErrorCode::ContainerNotRunning,
            "The container isn't running.",
        )
        .with_hint("Start it first.");
    }
    if has("is already paused") || has("is not paused") || status == Some(304) {
        return AppError::new(
            ErrorCode::AlreadyDone,
            "The container is already in that state.",
        );
    }
    if has("could not find the file") || has("no such file or directory") {
        return AppError::new(
            ErrorCode::PathNotFound,
            "That path doesn't exist in the container.",
        );
    }
    if has("no such container") {
        return AppError::not_found("This container no longer exists.")
            .with_hint("It was probably removed elsewhere. The list refreshes on its own.");
    }
    if has("no such volume") {
        return AppError::not_found("This volume no longer exists.");
    }
    if has("network") && has("not found") {
        return AppError::not_found("This network no longer exists.");
    }

    let said = sentence(message);
    match status {
        Some(400) => AppError::new(
            ErrorCode::BadRequest,
            format!("The Docker Engine rejected the request: {said}"),
        ),
        Some(401 | 403) => AppError::new(
            ErrorCode::Forbidden,
            format!("The Docker Engine refused: {said}"),
        )
        .with_hint("An authorization plugin or a rootless Engine may forbid this."),
        Some(404) => AppError::not_found(said),
        Some(409) => AppError::new(ErrorCode::Conflict, said),
        _ => AppError::new(
            ErrorCode::EngineError,
            format!("The Docker Engine failed: {said}"),
        ),
    }
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

/// The Engine's text as a sentence: capitalized and ending with a period.
fn sentence(message: &str) -> String {
    let trimmed = message.trim().trim_end_matches('.');
    let mut chars = trimmed.chars();
    match chars.next() {
        Some(first) => format!("{}{}.", first.to_uppercase(), chars.as_str()),
        None => "No reason given.".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine(status: u16, message: &str) -> AppError {
        from_bollard(&bollard::errors::Error::DockerResponseServerError {
            status_code: status,
            message: message.to_owned(),
        })
    }

    #[test]
    fn engine_messages_map_to_codes() {
        for (status, message, code) in [
            (500, "driver failed programming external connectivity on endpoint web (abc): Bind for 0.0.0.0:8080 failed: port is already allocated", ErrorCode::PortInUse),
            (409, "Conflict. The container name \"/web\" is already in use by container \"abc\". You have to remove (or rename) that container to be able to reuse that name.", ErrorCode::NameInUse),
            (409, "remove shop_db: volume is in use - [abc]", ErrorCode::InUse),
            (409, "conflict: unable to delete 1234 (must be forced) - image is being used by stopped container abc", ErrorCode::InUse),
            (409, "conflict: unable to delete 1234 (must be forced) - image is referenced in multiple repositories", ErrorCode::InUse),
            (409, "cannot remove container \"/web\": container is running: stop the container before removing or force remove", ErrorCode::ContainerRunning),
            (409, "Container abc is not running", ErrorCode::ContainerNotRunning),
            (304, "", ErrorCode::AlreadyDone),
            (404, "No such container: abc", ErrorCode::NotFound),
            (404, "pull access denied for nope/nope, repository does not exist or may require 'docker login': denied: requested access to the resource is denied", ErrorCode::RegistryAuthRequired),
            (404, "manifest for alpine:nope not found: manifest unknown: manifest unknown", ErrorCode::ImageNotFound),
            (500, "toomanyrequests: You have reached your pull rate limit.", ErrorCode::RegistryRateLimited),
            (500, "Get \"https://registry-1.docker.io/v2/\": dial tcp: lookup registry-1.docker.io: no such host", ErrorCode::RegistryUnreachable),
            (500, "write /var/lib/docker/tmp/x: no space left on device", ErrorCode::DiskFull),
            (400, "Client sent an HTTP request to an HTTPS server.", ErrorCode::TlsRequired),
            (403, "authorization denied by plugin opa", ErrorCode::Forbidden),
            (500, "something odd", ErrorCode::EngineError),
        ] {
            let e = engine(status, message);
            assert_eq!(e.code, code, "{message}");
            assert_eq!(
                e.detail.as_deref(),
                Some(format!("HTTP {status}: {message}").as_str())
            );
        }
    }

    #[test]
    fn busy_ports_are_named() {
        let e = engine(
            500,
            "Bind for 0.0.0.0:8080 failed: port is already allocated",
        );
        assert_eq!(e.message, "Port 8080 is already used on the host.");
        let e = engine(500, "listen tcp4 0.0.0.0:80: bind: address already in use");
        assert_eq!(e.message, "Port 80 is already used on the host.");
    }

    #[test]
    fn unknown_messages_are_quoted_as_a_sentence() {
        let e = engine(400, "invalid reference format");
        assert_eq!(
            e.message,
            "The Docker Engine rejected the request: Invalid reference format."
        );
    }

    #[test]
    fn timeouts_are_timeouts() {
        assert_eq!(
            from_bollard(&bollard::errors::Error::RequestTimeoutError).code,
            ErrorCode::Timeout
        );
    }

    #[test]
    fn dropped_connections_say_the_engine_stopped() {
        let e = from_bollard(&bollard::errors::Error::IOError {
            err: std::io::Error::from(std::io::ErrorKind::ConnectionRefused),
        });
        assert_eq!(e.code, ErrorCode::EngineNotRunning);
    }
}

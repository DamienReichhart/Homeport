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
            Some(port) => format!("Port {port} is already in use."),
            None => "A published port is already in use.".to_owned(),
        };
        return AppError::new(ErrorCode::PortInUse, what).with_hint(
            "Stop the container or program that uses it, or publish this container on another port.",
        );
    }
    if has("no space left on device") {
        return AppError::new(ErrorCode::DiskFull, "The Docker Engine is out of disk space.").with_hint(
            "Remove unused images, containers and volumes (Prune on the Overview page), or free space on the machine it runs on.",
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
            "This image no longer exists on the Docker Engine.",
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
            "The Docker Engine can't reach the image registry.",
        )
        .with_hint("Check the internet access, DNS and proxy settings of the machine it runs on.");
    }
    if has("invalid reference format") {
        return AppError::new(ErrorCode::BadRequest, "That image name isn't valid.").with_hint(
            "Use a name like nginx:1.27 or registry.example.com/team/app:tag, in lower case.",
        );
    }
    if has("client sent an http request to an https server") {
        return AppError::new(
            ErrorCode::TlsRequired,
            "This Docker Engine only accepts TLS connections.",
        )
        .with_hint(
            "Change the connection to TLS on the host page and add the Engine's certificates.",
        );
    }
    if has("client version") && (has("too new") || has("too old"))
        || has("minimum supported api version")
    {
        return AppError::new(
            ErrorCode::ApiVersionUnsupported,
            "This Docker Engine's API version isn't supported.",
        )
        .with_hint("Update the Docker Engine.");
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
    if has("executable file not found") {
        return AppError::new(
            ErrorCode::NoShell,
            "That command doesn't exist in the container.",
        )
        .with_hint("Minimal images may have no shell or tools. Try /bin/sh, or another command.");
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

    // Unrecognised answers: the Engine's own words stay in the detail.
    let details = "The Docker Engine's exact answer is in Details.";
    match status {
        Some(400) => AppError::new(
            ErrorCode::BadRequest,
            "The Docker Engine rejected the request as invalid.",
        )
        .with_hint(details),
        Some(401 | 403) => AppError::new(
            ErrorCode::Forbidden,
            "The Docker Engine refused to do this.",
        )
        .with_hint("An authorisation plugin or a rootless Docker Engine may forbid it. The exact answer is in Details."),
        Some(404) => AppError::not_found("This no longer exists on the Docker Engine.")
            .with_hint("It was probably removed elsewhere. The lists refresh on their own."),
        Some(409) => AppError::new(
            ErrorCode::Conflict,
            "This clashes with the current state of the Docker Engine.",
        )
        .with_hint(details),
        _ => AppError::new(ErrorCode::EngineError, "The Docker Engine failed to do this.")
            .with_hint(details),
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
            (500, "OCI runtime exec failed: exec failed: unable to start container process: exec: \"bash\": executable file not found in $PATH: unknown", ErrorCode::NoShell),
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
        assert_eq!(e.message, "Port 8080 is already in use.");
        let e = engine(500, "listen tcp4 0.0.0.0:80: bind: address already in use");
        assert_eq!(e.message, "Port 80 is already in use.");
    }

    #[test]
    fn unknown_answers_keep_the_engines_words_out_of_the_message() {
        for status in [400, 403, 404, 409, 500] {
            let e = engine(status, "some internal reason xyz");
            assert!(!e.message.contains("xyz"), "{status}: {}", e.message);
            assert!(e.detail.unwrap().contains("some internal reason xyz"));
            assert!(e.hint.is_some(), "{status}");
        }
    }

    #[test]
    fn invalid_image_names_say_how_to_write_one() {
        let e = engine(400, "invalid reference format");
        assert_eq!(e.message, "That image name isn't valid.");
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

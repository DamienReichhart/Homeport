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
            (500, "rpc error: code = Unknown desc = update out of sequence", ErrorCode::Conflict),
            (304, "", ErrorCode::AlreadyDone),
            (404, "No such container: abc", ErrorCode::NotFound),
            (404, "pull access denied for nope/nope, repository does not exist or may require 'docker login': denied: requested access to the resource is denied", ErrorCode::RegistryAuthRequired),
            (404, "manifest for alpine:nope not found: manifest unknown: manifest unknown", ErrorCode::ImageNotFound),
            (500, "push access denied, repository does not exist or may require authorization: authorization failed: no basic auth credentials", ErrorCode::RegistryAuthRequired),
            (500, "toomanyrequests: You have reached your pull rate limit.", ErrorCode::RegistryRateLimited),
            (500, "unknown: failed to resolve reference \"docker.io/library/alpine:3.22\": unexpected status from HEAD request to https://registry-1.docker.io/v2/library/alpine/manifests/3.22: 429 Too Many Requests", ErrorCode::RegistryRateLimited),
            (500, "Head \"https://registry-1.docker.io/v2/library/nginx/manifests/latest\": unauthorized: incorrect username or password", ErrorCode::RegistryLoginFailed),
            (500, "Get \"https://registry-1.docker.io/v2/\": dial tcp: lookup registry-1.docker.io: no such host", ErrorCode::RegistryUnreachable),
            (500, "write /var/lib/docker/tmp/x: no space left on device", ErrorCode::DiskFull),
            (400, "Client sent an HTTP request to an HTTPS server.", ErrorCode::TlsRequired),
            (403, "authorization denied by plugin opa", ErrorCode::Forbidden),
            (500, "OCI runtime exec failed: exec failed: unable to start container process: exec: \"bash\": executable file not found in $PATH: unknown", ErrorCode::NoShell),
            (500, "could not select device driver \"\" with capabilities: [[gpu]]", ErrorCode::GpuUnavailable),
            (400, "Range of CPUs is from 0.01 to 4.00, as there are only 4 CPUs available", ErrorCode::BadRequest),
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
    assert_eq!(e.key, "errors.engine.portInUse");
    assert_eq!(e.params["port"], super::super::Param::Number(8080));
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
fn too_many_cpus_names_the_hosts_count() {
    let e = engine(
        400,
        "Range of CPUs is from 0.01 to 4.00, as there are only 4 CPUs available",
    );
    assert_eq!(e.message, "This host has only 4 CPUs to give a container.");
    assert_eq!(e.params["cpus"], super::super::Param::Number(4));
    let e = engine(
        400,
        "Range of CPUs is from 0.01 to 1.00, as there are only 1 CPUs available",
    );
    assert_eq!(e.message, "This host has only 1 CPU to give a container.");
}

#[test]
fn invalid_image_names_say_how_to_write_one() {
    let e = engine(400, "invalid reference format");
    assert_eq!(e.message, "That image name isn't valid.");
}

#[test]
fn errors_classified_by_a_dialer_survive_bollard() {
    let classified = reach::connection_lost("prod (10.0.0.2)");
    let e = bollard::errors::Error::IOError {
        err: std::io::Error::new(std::io::ErrorKind::ConnectionRefused, classified.clone()),
    };
    assert_eq!(from_bollard(&e), classified);
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

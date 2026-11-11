use super::*;
use crate::testing::UnixDialer;

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn dialed_streams_carry_the_engine_api() {
    let docker = docker_over(Arc::new(UnixDialer));
    docker.ping().await.unwrap();
    docker.version().await.unwrap();
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn local_socket_hosts_connect() {
    let connected = connect(
        "local",
        &HostConnection::Local {
            socket_path: "/var/run/docker.sock".into(),
        },
        None,
        None,
    )
    .await
    .unwrap();
    connected.docker.ping().await.unwrap();
    // The dialer beside bollard's connector reaches the same Engine.
    let ping = http::send(&*connected.dialer, http::Request::get("/_ping"))
        .await
        .unwrap();
    assert!(ping.status.is_success());
}

async fn connect_local(socket_path: &str) -> ConnectError {
    connect(
        "local",
        &HostConnection::Local {
            socket_path: socket_path.into(),
        },
        None,
        None,
    )
    .await
    .err()
    .expect("nothing should answer")
}

#[tokio::test]
async fn a_closed_tcp_port_refuses_the_connection() {
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let address = format!("127.0.0.1:{port}");
    let err = connect(
        "prod",
        &HostConnection::Tcp {
            address: address.clone(),
        },
        None,
        None,
    )
    .await
    .err()
    .expect("the port is closed");
    let ConnectError::Unreachable(e) = err else {
        panic!("expected unreachable, got {err:?}");
    };
    assert_eq!(e.code, ErrorCode::ConnectionRefused, "{e:?}");
    assert!(e.message.contains(&format!("prod ({address})")), "{e:?}");
}

#[tokio::test]
async fn a_missing_socket_means_no_engine_is_running_there() {
    let err = connect_local("/nonexistent/docker.sock").await;
    let ConnectError::Unreachable(e) = err else {
        panic!("expected unreachable, got {err:?}");
    };
    assert!(
        matches!(
            e.code,
            ErrorCode::EngineNotRunning | ErrorCode::EngineNotInstalled
        ),
        "{e:?}"
    );
    assert!(e.message.contains("/nonexistent/docker.sock"), "{e:?}");
    assert!(e.hint.is_some_and(|h| !h.text.is_empty()));
    assert!(e.detail.is_some());
}

#[tokio::test]
async fn a_socket_nobody_listens_on_means_the_engine_is_stopped() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("docker.sock");
    drop(std::os::unix::net::UnixListener::bind(&path).unwrap());
    let err = connect_local(path.to_str().unwrap()).await;
    let ConnectError::Unreachable(e) = err else {
        panic!("expected unreachable, got {err:?}");
    };
    assert_eq!(e.code, ErrorCode::EngineNotRunning, "{e:?}");
}

#[tokio::test]
async fn a_socket_without_permission_refuses_access() {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};
    if std::fs::metadata("/proc/self").is_ok_and(|m| m.uid() == 0) {
        return; // root ignores file permissions
    }
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("docker.sock");
    let _listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000)).unwrap();
    let err = connect_local(path.to_str().unwrap()).await;
    let ConnectError::Unauthorized(e) = err else {
        panic!("expected unauthorized, got {err:?}");
    };
    assert_eq!(e.code, ErrorCode::PermissionDenied, "{e:?}");
    assert!(e.message.contains(path.to_str().unwrap()), "{e:?}");
}

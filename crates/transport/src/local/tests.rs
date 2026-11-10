use homeport_core::error::{reach::classified_cause, ErrorCode};

use super::*;

#[cfg(unix)]
#[tokio::test]
async fn a_local_socket_nobody_listens_on_is_classified() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("podman.sock");
    let dialer = LocalDialer::new(path.to_str().unwrap());
    let err = dialer.dial().await.err().unwrap();
    let classified = classified_cause(&err).expect("the dialer classifies its failures");
    assert!(
        matches!(
            classified.code,
            ErrorCode::EngineNotRunning | ErrorCode::EngineNotInstalled
        ),
        "{classified:?}"
    );
    assert!(matches!(
        dialer.connect().await,
        Err(ConnectError::Unreachable(_))
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn a_listening_local_socket_is_dialed() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("engine.sock");
    let _listener = tokio::net::UnixListener::bind(&path).unwrap();
    let dialer = LocalDialer::new(path.to_str().unwrap());
    dialer.connect().await.unwrap();
    dialer.dial().await.unwrap();
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn local_linux_sockets_run_commands_on_this_computer() {
    let dialer = LocalDialer::new("/run/podman/podman.sock");
    assert!(dialer.runs_commands());
    let out = dialer
        .run(
            "echo out; echo err >&2; exit 3".into(),
            Duration::from_secs(5),
        )
        .await
        .unwrap();
    assert_eq!(out.stdout, "out\n");
    assert_eq!(out.stderr, "err\n");
    assert_eq!(out.status, Some(3));
    assert!(!out.success());
    let slow = dialer
        .run("sleep 5".into(), Duration::from_millis(50))
        .await;
    assert_eq!(slow.unwrap_err().kind(), io::ErrorKind::TimedOut);
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn spawned_commands_stream_their_input_and_output() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let dialer = LocalDialer::new("/run/containerd/containerd.sock");
    let mut cat = dialer
        .spawn("echo ready; cat; echo bye >&2; exit 4".into())
        .await
        .unwrap();
    let mut line = [0; 6];
    cat.io.read_exact(&mut line).await.unwrap();
    assert_eq!(&line, b"ready\n");
    cat.io.write_all(b"ping").await.unwrap();
    cat.io.shutdown().await.unwrap();
    let mut rest = Vec::new();
    cat.io.read_to_end(&mut rest).await.unwrap();
    assert_eq!(rest, b"ping");
    let exit = cat.exit.await;
    assert_eq!(exit.status, Some(4));
    assert_eq!(exit.stderr, "bye\n");

    // Dropping the stream ends a command that would run on.
    let sleeping = dialer.spawn("sleep 30".into()).await.unwrap();
    drop(sleeping.io);
    let exit = tokio::time::timeout(Duration::from_secs(5), sleeping.exit)
        .await
        .expect("the command was killed");
    assert!(!exit.success());
}

#[tokio::test]
async fn tcp_hosts_run_no_commands() {
    let dialer = TcpDialer::new("lab", "127.0.0.1:1");
    assert!(!dialer.runs_commands());
    let err = dialer
        .run("true".into(), Duration::from_secs(1))
        .await
        .unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::Unsupported);
}

#[tokio::test]
async fn a_closed_tcp_port_is_refused_and_named() {
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let address = format!("127.0.0.1:{port}");
    let dialer = TcpDialer::new(&format!("lab ({address})"), &address);
    let err = dialer.dial().await.err().unwrap();
    let classified = classified_cause(&err).unwrap();
    assert_eq!(classified.code, ErrorCode::ConnectionRefused);
    assert!(classified.message.contains("lab ("), "{classified:?}");
}

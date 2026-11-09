use std::sync::{Arc, Mutex};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use super::*;
use crate::model::{ClusterConnection, ClusterKind};

/// The credential the Cluster's own client sends; it must never reach the
/// kubeconfig, and the proxy's token must never reach the Cluster.
const CLUSTER_TOKEN: &str = "cluster-secret-token";

/// What the fake API server saw of each request: its Authorization and
/// Host headers, and its path.
type Seen = Arc<Mutex<Vec<(Option<String>, Option<String>, String)>>>;

/// A fake API server: answers `/upgrade` with 101 and then echoes bytes,
/// anything else with the request's path in a body sent in two chunks.
async fn api_server() -> (SocketAddr, Seen) {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
    let address = listener.local_addr().unwrap();
    let seen: Seen = Arc::default();
    let log = seen.clone();
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let log = log.clone();
            tokio::spawn(async move {
                let service = hyper::service::service_fn(move |mut req: Request<Incoming>| {
                    let header = |name| {
                        req.headers()
                            .get(name)
                            .map(|v: &HeaderValue| v.to_str().unwrap().to_owned())
                    };
                    let path = req.uri().path_and_query().unwrap().to_string();
                    log.lock().unwrap().push((
                        header(header::AUTHORIZATION),
                        header(header::HOST),
                        path.clone(),
                    ));
                    let response = if path == "/upgrade" {
                        let upgrade = hyper::upgrade::on(&mut req);
                        tokio::spawn(async move {
                            let mut io = TokioIo::new(upgrade.await.unwrap());
                            let mut buf = [0u8; 4];
                            io.read_exact(&mut buf).await.unwrap();
                            io.write_all(&buf).await.unwrap();
                        });
                        Response::builder()
                            .status(StatusCode::SWITCHING_PROTOCOLS)
                            .header(header::CONNECTION, "Upgrade")
                            .header(header::UPGRADE, "SPDY/3.1")
                            .body(Full::new(Bytes::new()))
                            .unwrap()
                    } else {
                        Response::new(Full::new(Bytes::from(format!("path={path}"))))
                    };
                    async move { Ok::<_, Infallible>(response) }
                });
                let _ = hyper::server::conn::http1::Builder::new()
                    .serve_connection(TokioIo::new(stream), service)
                    .with_upgrades()
                    .await;
            });
        }
    });
    (address, seen)
}

fn cluster() -> Cluster {
    Cluster {
        id: "c".into(),
        name: "kind-dev".into(),
        kind: ClusterKind::Kubernetes,
        connection: ClusterConnection::Kubernetes(crate::model::KubernetesConnection {
            server: "http://127.0.0.1:1".into(),
            namespace: Some("shop".into()),
            ..Default::default()
        }),
        enabled: true,
        created_at: String::new(),
    }
}

fn clients(api: SocketAddr) -> ClientSource {
    let mut config = kube::Config::new(format!("http://{api}").parse().unwrap());
    config.auth_info.token = Some(CLUSTER_TOKEN.to_owned().into());
    let client = kube::Client::try_from(config).unwrap();
    Arc::new(move || Ok(client.clone()))
}

/// The proxy's token, read back from its kubeconfig.
fn proxy_token(proxy: &ClusterProxy) -> String {
    let text = std::fs::read_to_string(proxy.kubeconfig()).unwrap();
    let line = text.lines().find(|l| l.contains("token:")).unwrap();
    line.split("token:").nth(1).unwrap().trim().to_owned()
}

/// Sends `request` as is over a fresh connection and reads the answer:
/// until the connection closes, or for an upgrade until its headers end.
async fn raw(address: SocketAddr, request: &str) -> (TcpStream, String) {
    let upgrade = request.contains("Upgrade:");
    let mut stream = TcpStream::connect(address).await.unwrap();
    stream.write_all(request.as_bytes()).await.unwrap();
    let mut answer = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        answer.extend_from_slice(&buf[..n]);
        let text = String::from_utf8_lossy(&answer);
        if n == 0 || upgrade && text.contains("\r\n\r\n") {
            return (stream, text.into_owned());
        }
    }
}

#[tokio::test]
async fn a_request_without_the_token_is_refused() {
    let (api, seen) = api_server().await;
    let proxy = open(&cluster(), clients(api)).await.unwrap();
    let (_, answer) = raw(
        proxy.address(),
        "GET /api HTTP/1.1\r\nHost: x\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(answer.starts_with("HTTP/1.1 401"), "{answer}");
    let (_, answer) = raw(
        proxy.address(),
        "GET /api HTTP/1.1\r\nHost: x\r\nAuthorization: Bearer wrong\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(answer.starts_with("HTTP/1.1 401"), "{answer}");
    assert!(seen.lock().unwrap().is_empty());
}

#[tokio::test]
async fn requests_go_on_with_the_clusters_credentials_only() {
    let (api, seen) = api_server().await;
    let proxy = open(&cluster(), clients(api)).await.unwrap();
    let token = proxy_token(&proxy);
    let (_, answer) = raw(
        proxy.address(),
        &format!("GET /api/v1/namespaces?limit=1 HTTP/1.1\r\nHost: {}\r\nAuthorization: Bearer {token}\r\nConnection: close\r\n\r\n", proxy.address()),
    )
    .await;
    assert!(answer.starts_with("HTTP/1.1 200"), "{answer}");
    assert!(
        answer.contains("path=/api/v1/namespaces?limit=1"),
        "{answer}"
    );
    let seen = seen.lock().unwrap();
    let (authorization, host, _) = &seen[0];
    assert_eq!(
        authorization.as_deref(),
        Some(&*format!("Bearer {CLUSTER_TOKEN}"))
    );
    assert_ne!(host.as_deref(), Some(&*proxy.address().to_string()));
}

#[tokio::test]
async fn upgraded_connections_pass_through() {
    let (api, _) = api_server().await;
    let proxy = open(&cluster(), clients(api)).await.unwrap();
    let token = proxy_token(&proxy);
    let (mut stream, answer) = raw(
        proxy.address(),
        &format!("POST /upgrade HTTP/1.1\r\nHost: x\r\nAuthorization: Bearer {token}\r\nConnection: Upgrade\r\nUpgrade: SPDY/3.1\r\nContent-Length: 0\r\n\r\n"),
    )
    .await;
    assert!(answer.starts_with("HTTP/1.1 101"), "{answer}");
    stream.write_all(b"ping").await.unwrap();
    let mut echo = [0u8; 4];
    stream.read_exact(&mut echo).await.unwrap();
    assert_eq!(&echo, b"ping");
}

#[tokio::test]
async fn the_kubeconfig_is_private_holds_no_credential_and_goes_with_the_proxy() {
    let (api, _) = api_server().await;
    let proxy = open(&cluster(), clients(api)).await.unwrap();
    let path = proxy.kubeconfig().to_owned();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(!text.contains(CLUSTER_TOKEN));
    assert!(text.contains(&format!("server: http://{}", proxy.address())));
    assert!(text.contains("namespace: \"shop\""));
    assert_eq!(proxy_token(&proxy).len(), 43, "256 bits, base64");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = |p: &Path| std::fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&path), 0o600);
        assert_eq!(mode(path.parent().unwrap()), 0o700);
    }
    drop(proxy);
    assert!(!path.exists());
    assert!(!path.parent().unwrap().exists());
}

#[cfg(unix)]
#[tokio::test]
async fn a_cli_gets_the_kubeconfig_and_its_stdin() {
    let (api, _) = api_server().await;
    let access = ClusterAccess {
        cluster: cluster(),
        clients: clients(api),
    };
    let command = CliCommand {
        program: "sh".into(),
        args: vec![
            "-c".into(),
            r#"test "$2" = "$KUBECONFIG" && test -f "$KUBECONFIG" && echo "$1" && cat"#.into(),
            "sh".into(),
        ],
        stdin: Some("from stdin".into()),
        ..Default::default()
    };
    let fail = |f: Failure<'_>| AppError::validation(msg::GONE).with_detail(format!("{f:?}"));
    let out = output(&command, Some(&access), fail).await.unwrap();
    assert_eq!(out, "--kubeconfig\nfrom stdin");

    let events = Mutex::new(Vec::new());
    let failing = CliCommand {
        program: "sh".into(),
        args: vec!["-c".into(), "echo out; echo oops >&2; exit 3".into()],
        ..Default::default()
    };
    run_cli(failing, None, fail, |e| {
        events.lock().unwrap().push(e);
        true
    })
    .await;
    let events = events.into_inner().unwrap();
    assert!(events.contains(&CliEvent::Line { text: "out".into() }));
    match events.last().unwrap() {
        CliEvent::Exit {
            success: false,
            code: Some(3),
            error: Some(e),
        } => assert!(e.detail.as_deref().unwrap().contains("oops")),
        other => panic!("{other:?}"),
    }
}

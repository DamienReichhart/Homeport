//! The client against a tiny HTTP server on a temporary Unix socket.

use std::collections::HashMap;

use futures_util::StreamExt;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::UnixListener,
    sync::mpsc,
};

use super::*;
use crate::LocalDialer;

/// A request as the server saw it.
#[derive(Debug)]
struct Seen {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

/// What the server answers: raw bytes, and whether to echo afterwards.
struct Answer {
    raw: Vec<u8>,
    echo: bool,
}

fn answer(status: &str, content_type: &str, body: &str) -> Answer {
    Answer {
        raw: format!(
            "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        )
        .into_bytes(),
        echo: false,
    }
}

/// Serves each connection with `handler`, reporting the requests it got.
fn serve(
    handler: fn(&Seen) -> Answer,
) -> (
    tempfile::TempDir,
    LocalDialer,
    mpsc::UnboundedReceiver<Seen>,
) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("engine.sock");
    let listener = UnixListener::bind(&path).unwrap();
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let tx = tx.clone();
            tokio::spawn(async move {
                let (read, mut write) = stream.into_split();
                let mut reader = BufReader::new(read);
                let mut first = String::new();
                reader.read_line(&mut first).await.unwrap();
                let mut parts = first.split_whitespace();
                let method = parts.next().unwrap_or_default().to_owned();
                let path = parts.next().unwrap_or_default().to_owned();
                let mut headers = HashMap::new();
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).await.unwrap();
                    let line = line.trim_end();
                    if line.is_empty() {
                        break;
                    }
                    if let Some((k, v)) = line.split_once(':') {
                        headers.insert(k.trim().to_lowercase(), v.trim().to_owned());
                    }
                }
                let length = headers
                    .get("content-length")
                    .and_then(|l| l.parse().ok())
                    .unwrap_or(0);
                let mut body = vec![0; length];
                reader.read_exact(&mut body).await.unwrap();
                let seen = Seen {
                    method,
                    path,
                    headers,
                    body,
                };
                let reply = handler(&seen);
                tx.send(seen).ok();
                write.write_all(&reply.raw).await.unwrap();
                if reply.echo {
                    let mut buf = [0; 64];
                    while let Ok(n) = reader.read(&mut buf).await {
                        if n == 0 || write.write_all(&buf[..n]).await.is_err() {
                            break;
                        }
                    }
                }
            });
        }
    });
    let dialer = LocalDialer::new(path.to_str().unwrap());
    (dir, dialer, rx)
}

#[tokio::test]
async fn reads_json_in_origin_form_with_a_host_header() {
    let (_dir, dialer, mut seen) =
        serve(|_| answer("200 OK", "application/json", r#"{"Version":"5.8.1"}"#));
    #[derive(Deserialize)]
    struct Version {
        #[serde(rename = "Version")]
        version: String,
    }
    let v: Version = json(&dialer, Request::get("/v5.0.0/libpod/version?x=1"))
        .await
        .unwrap();
    assert_eq!(v.version, "5.8.1");
    let req = seen.recv().await.unwrap();
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/v5.0.0/libpod/version?x=1");
    assert_eq!(req.headers.get("host").map(String::as_str), Some("docker"));
}

#[tokio::test]
async fn sends_a_json_body_and_keeps_the_answer_headers() {
    let (_dir, dialer, mut seen) = serve(|_| Answer {
        raw: b"HTTP/1.1 201 Created\r\nLibpod-Api-Version: 5.8.1\r\nContent-Length: 2\r\n\r\n{}"
            .to_vec(),
        echo: false,
    });
    let request = Request::post("/libpod/pods/create")
        .json(&serde_json::json!({ "name": "web" }))
        .unwrap();
    let answer = send(&dialer, request).await.unwrap().ok().unwrap();
    assert_eq!(answer.status, StatusCode::CREATED);
    assert_eq!(answer.header("libpod-api-version"), Some("5.8.1"));
    let req = seen.recv().await.unwrap();
    assert_eq!(req.method, "POST");
    assert_eq!(
        req.headers.get("content-type").map(String::as_str),
        Some("application/json")
    );
    assert_eq!(req.body, br#"{"name":"web"}"#);
}

#[tokio::test]
async fn failures_are_classified_with_the_engines_words_as_detail() {
    let (_dir, dialer, _) = serve(|req| match req.path.as_str() {
        "/missing" => answer(
            "404 Not Found",
            "application/json",
            r#"{"cause":"no such pod","message":"no pod with name or ID web found: no such pod","response":404}"#,
        ),
        "/busy" => answer(
            "409 Conflict",
            "application/json",
            r#"{"cause":"pod is in use","message":"pod web is running","response":409}"#,
        ),
        _ => answer("500 Internal Server Error", "text/plain", "boom"),
    });
    let missing = json::<serde_json::Value>(&dialer, Request::get("/missing"))
        .await
        .unwrap_err();
    assert!(missing.is(ErrorCode::NotFound), "{missing:?}");
    assert!(missing.detail.unwrap().contains("no pod with name"));

    let busy = send(&dialer, Request::delete("/busy"))
        .await
        .unwrap()
        .ok()
        .unwrap_err();
    assert!(busy.is(ErrorCode::Conflict), "{busy:?}");

    let broken = send(&dialer, Request::get("/other"))
        .await
        .unwrap()
        .ok()
        .unwrap_err();
    assert!(broken.is(ErrorCode::EngineError), "{broken:?}");
    assert_eq!(broken.detail.as_deref(), Some("HTTP 500: boom"));
}

#[tokio::test]
async fn answers_that_arent_json_are_unreadable() {
    let (_dir, dialer, _) = serve(|_| answer("200 OK", "application/json", "not json"));
    let err = json::<serde_json::Value>(&dialer, Request::get("/x"))
        .await
        .unwrap_err();
    assert!(err.is(ErrorCode::EngineError));
    assert_eq!(err.key, "errors.engine.unreadableAnswer");
}

#[tokio::test]
async fn streams_lines_split_across_chunks() {
    let (_dir, dialer, _) = serve(|_| Answer {
        raw: b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n\
            7\r\n{\"a\":1}\r\n\
            5\r\n\n{\"b\"\r\n\
            5\r\n:2}\r\n\r\n\
            5\r\n{\"c\"}\r\n\
            0\r\n\r\n"
            .to_vec(),
        echo: false,
    });
    let lines: Vec<Bytes> = lines(&dialer, Request::get("/events"))
        .await
        .unwrap()
        .map(Result::unwrap)
        .collect()
        .await;
    assert_eq!(
        lines,
        [
            Bytes::from_static(br#"{"a":1}"#),
            Bytes::from_static(br#"{"b":2}"#),
            Bytes::from_static(br#"{"c"}"#),
        ]
    );
}

#[tokio::test]
async fn a_stream_that_fails_says_why() {
    let (_dir, dialer, _) = serve(|_| {
        answer(
            "404 Not Found",
            "application/json",
            r#"{"message":"no such container"}"#,
        )
    });
    let err = lines(&dialer, Request::get("/logs")).await.err().unwrap();
    assert!(err.is(ErrorCode::NotFound));
}

#[tokio::test]
async fn upgraded_connections_stay_open() {
    let (_dir, dialer, mut seen) = serve(|_| Answer {
        raw: b"HTTP/1.1 101 UPGRADED\r\nConnection: Upgrade\r\nUpgrade: tcp\r\n\r\n".to_vec(),
        echo: true,
    });
    let (status, _, mut io) = upgrade(&dialer, Request::post("/exec/1/start"), "tcp")
        .await
        .unwrap();
    assert_eq!(status, StatusCode::SWITCHING_PROTOCOLS);
    io.write_all(b"ping").await.unwrap();
    let mut buf = [0; 4];
    io.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"ping");
    let req = seen.recv().await.unwrap();
    assert_eq!(req.headers.get("upgrade").map(String::as_str), Some("tcp"));
}

#[tokio::test]
async fn nothing_listening_is_classified_by_the_dialer() {
    let dir = tempfile::tempdir().unwrap();
    let dialer = LocalDialer::new(dir.path().join("none.sock").to_str().unwrap());
    let err = send(&dialer, Request::get("/_ping")).await.unwrap_err();
    assert!(
        matches!(
            err.code,
            ErrorCode::EngineNotRunning | ErrorCode::EngineNotInstalled
        ),
        "{err:?}"
    );
}

#[test]
fn values_are_percent_encoded() {
    assert_eq!(encode("web-1.app_x~"), "web-1.app_x~");
    assert_eq!(encode("my pod/a?b&c"), "my%20pod%2Fa%3Fb%26c");
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn reaches_the_local_engine() {
    let dialer = LocalDialer::new("/var/run/docker.sock");
    let ping = send(&dialer, Request::get("/_ping"))
        .await
        .unwrap()
        .ok()
        .unwrap();
    assert_eq!(&ping.body[..], b"OK");
    let version: serde_json::Value = json(&dialer, Request::get("/version")).await.unwrap();
    assert!(version["ApiVersion"].is_string(), "{version}");
}

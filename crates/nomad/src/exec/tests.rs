use std::sync::{Arc, Mutex};

use tokio::net::TcpListener;
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{Request, Response},
};

use serde_json::Value;

use super::*;

fn options(command: Option<&str>) -> NomadExecOptions {
    NomadExecOptions {
        task: "server".into(),
        command: command.map(str::to_owned),
        cols: 80,
        rows: 24,
    }
}

#[test]
fn the_command_defaults_to_sh_and_is_split_like_a_shell() {
    assert_eq!(command(&options(None)).unwrap(), ["/bin/sh"]);
    assert_eq!(
        command(&options(Some("bash -c 'echo hi'"))).unwrap(),
        ["bash", "-c", "echo hi"]
    );
    assert!(command(&options(Some("echo 'open")))
        .unwrap_err()
        .is(ErrorCode::Validation));
}

#[test]
fn the_url_carries_the_task_and_command_but_never_the_token() {
    let url = exec_url("https://nomad:4646", "a1", Some("eu"), &options(None)).unwrap();
    assert!(url.starts_with("wss://nomad:4646/v1/client/allocation/a1/exec?"));
    assert!(url.contains("task=server") && url.contains("tty=true"));
    assert!(url.contains("command=%5B%22%2Fbin%2Fsh%22%5D"), "{url}");
    assert!(url.contains("region=eu"));
    assert!(!url.contains("token"));
    assert!(exec_url("http://nomad:4646", "a1", None, &options(None))
        .unwrap()
        .starts_with("ws://"));
}

#[test]
fn frames_are_encoded_and_read_as_nomad_speaks_them() {
    let stdin: Value = serde_json::from_str(&stdin_frame(b"ls\n")).unwrap();
    assert_eq!(stdin["stdin"]["data"], "bHMK");
    let size: Value = serde_json::from_str(&size_frame(120, 0)).unwrap();
    assert_eq!(size["tty_size"], json!({ "width": 120, "height": 1 }));
    assert_eq!(
        read_frame(&output_frame("stdout", "hi")),
        Received::Output(b"hi".to_vec())
    );
    assert_eq!(
        read_frame(&output_frame("stderr", "oops")),
        Received::Output(b"oops".to_vec())
    );
    assert_eq!(
        read_frame(r#"{"exited":true,"result":{"exit_code":3}}"#),
        Received::Exited(Some(3))
    );
    assert_eq!(
        read_frame(r#"{"stdout":{"close":true}}"#),
        Received::Nothing
    );
}

/// A Nomad exec endpoint that echoes what it gets, then exits with 0 on
/// `exit`; it records the token header it was opened with.
// tungstenite's handshake callback returns its large error type.
#[allow(clippy::result_large_err)]
async fn fake_exec() -> (String, Arc<Mutex<Option<String>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let token = Arc::new(Mutex::new(None));
    let seen = token.clone();
    tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let socket = accept_hdr_async(tcp, |request: &Request, response: Response| {
            *seen.lock().unwrap() = request
                .headers()
                .get(TOKEN_HEADER)
                .map(|v| v.to_str().unwrap().to_owned());
            Ok(response)
        })
        .await
        .unwrap();
        let (mut sink, mut stream) = socket.split();
        while let Some(Ok(Message::Text(text))) = stream.next().await {
            let frame: Value = serde_json::from_str(text.as_str()).unwrap();
            let Some(data) = frame["stdin"]["data"].as_str() else {
                continue;
            };
            let input = base64::engine::general_purpose::STANDARD
                .decode(data)
                .unwrap();
            if input == b"exit\n" {
                sink.send(Message::text(r#"{"exited":true,"result":{"exit_code":0}}"#))
                    .await
                    .unwrap();
                return;
            }
            let echoed = String::from_utf8(input).unwrap();
            sink.send(Message::text(output_frame("stdout", &echoed)))
                .await
                .unwrap();
        }
    });
    (address, token)
}

#[tokio::test]
async fn a_terminal_sends_input_and_reports_its_output_and_exit_code() {
    let (address, token) = fake_exec().await;
    let nomad = Nomad::new(&crate::NomadConfig {
        name: "dev".into(),
        address,
        token: Some("secret-token".into()),
        ..Default::default()
    })
    .unwrap();
    let (terminal, output) = NomadTerminal::open(&nomad, "a1", &options(None))
        .await
        .unwrap();
    assert_eq!(token.lock().unwrap().as_deref(), Some("secret-token"));
    let events = Arc::new(Mutex::new(vec![]));
    let seen = events.clone();
    let pumped = tokio::spawn(pump(
        nomad,
        "a1".into(),
        "server".into(),
        terminal.clone(),
        output,
        move |e| {
            seen.lock().unwrap().push(e);
            true
        },
    ));
    terminal.write(b"echo hi\n").await.unwrap();
    terminal.write(b"exit\n").await.unwrap();
    tokio::time::timeout(Duration::from_secs(5), pumped)
        .await
        .unwrap()
        .unwrap();
    let events = events.lock().unwrap();
    assert!(events.contains(&ExecEvent::Output {
        data: "echo hi\n".into()
    }));
    assert_eq!(
        events.last(),
        Some(&ExecEvent::Exit {
            code: Some(0),
            error: None
        })
    );
}

#[tokio::test]
async fn a_refused_upgrade_says_why() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move {
        use tokio::io::AsyncWriteExt;
        let (mut tcp, _) = listener.accept().await.unwrap();
        let body = "Permission denied";
        let reply = format!(
            "HTTP/1.1 403 Forbidden\r\ncontent-length: {}\r\n\r\n{body}",
            body.len()
        );
        tcp.write_all(reply.as_bytes()).await.unwrap();
    });
    let nomad = Nomad::new(&crate::NomadConfig {
        name: "dev".into(),
        address,
        ..Default::default()
    })
    .unwrap();
    let Err(e) = NomadTerminal::open(&nomad, "a1", &options(None)).await else {
        panic!("opened");
    };
    assert_eq!(e.key, "errors.nomad.execDenied");
}

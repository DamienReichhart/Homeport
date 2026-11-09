use std::time::Duration;

use super::*;
use crate::instances::{
    self,
    tests::{live, live_instance},
    InstanceAction,
};

fn options(command: Option<&str>) -> IncusExecOptions {
    IncusExecOptions {
        command: command.map(str::to_owned),
        user: None,
        cwd: None,
        cols: 100,
        rows: 30,
    }
}

#[test]
fn a_shell_runs_when_no_command_is_given() {
    let body = exec_body(&options(None)).unwrap();
    assert_eq!(body["command"][0], "/bin/sh");
    assert_eq!(body["command"][2], DEFAULT_SHELL);
    assert_eq!(body["interactive"], true);
    assert_eq!(body["wait-for-websocket"], true);
    assert_eq!(body["width"], 100);
    assert_eq!(body["environment"]["TERM"], "xterm-256color");
    assert!(body.get("user").is_none());
}

#[test]
fn commands_split_like_a_shell_would() {
    let mut with_user = options(Some("ls -la '/var/log/my app'"));
    with_user.user = Some(1000);
    with_user.cwd = Some("/srv".into());
    let body = exec_body(&with_user).unwrap();
    assert_eq!(
        body["command"],
        serde_json::json!(["ls", "-la", "/var/log/my app"])
    );
    assert_eq!(body["user"], 1000);
    assert_eq!(body["cwd"], "/srv");
    let e = exec_body(&options(Some("echo 'oops"))).unwrap_err();
    assert_eq!(e.code, ErrorCode::Validation);
}

#[test]
fn control_messages_are_incus_shapes() {
    let resize: serde_json::Value = serde_json::from_str(&resize_message(120, 0)).unwrap();
    assert_eq!(
        resize,
        serde_json::json!({ "command": "window-resize", "args": { "width": "120", "height": "1" } })
    );
    let signal: serde_json::Value = serde_json::from_str(&signal_message(1)).unwrap();
    assert_eq!(
        signal,
        serde_json::json!({ "command": "signal", "signal": 1 })
    );
}

/// Runs `session`'s output into a channel.
fn collect(
    session: Arc<IncusTerminal>,
    output: SplitStream<Socket>,
) -> tokio::sync::mpsc::UnboundedReceiver<ExecEvent> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(pump(session, output, move |e| tx.send(e).is_ok()));
    rx
}

/// Everything printed until `until` shows up or the terminal ends.
async fn read_until(
    rx: &mut tokio::sync::mpsc::UnboundedReceiver<ExecEvent>,
    until: &str,
) -> (String, Option<ExecEvent>) {
    let mut text = String::new();
    loop {
        match tokio::time::timeout(Duration::from_secs(20), rx.recv()).await {
            Ok(Some(ExecEvent::Output { data })) => {
                text.push_str(&data);
                if text.contains(until) {
                    return (text, None);
                }
            }
            Ok(Some(other)) => return (text, Some(other)),
            Ok(None) | Err(_) => return (text, None),
        }
    }
}

#[tokio::test]
#[ignore = "needs a local Incus"]
async fn incus_live_exec_and_console_open_on_a_running_instance() {
    let incus = live().await;
    let name = format!("homeport-term-{}", std::process::id());
    live_instance(&incus, &name).await;

    // A stopped Instance has no terminal.
    let stopped = IncusTerminal::exec(&incus, "default", &name, &options(Some("true")))
        .await
        .err()
        .expect("refused");
    assert_eq!(stopped.code, ErrorCode::ContainerNotRunning, "{stopped:?}");

    instances::act(&incus, "default", &name, InstanceAction::Start, false)
        .await
        .unwrap();

    let (session, output) = IncusTerminal::exec(
        &incus,
        "default",
        &name,
        &options(Some("sh -c 'echo homeport-$((40+2))'")),
    )
    .await
    .unwrap();
    let mut rx = collect(session, output);
    let (text, _) = read_until(&mut rx, "homeport-42").await;
    assert!(text.contains("homeport-42"), "{text:?}");
    let (_, end) = read_until(&mut rx, "\u{0}never").await;
    assert_eq!(
        end,
        Some(ExecEvent::Exit {
            code: Some(0),
            error: None
        })
    );

    // A shell takes input, resizes, and is hung up on close.
    let (shell, output) = IncusTerminal::exec(&incus, "default", &name, &options(None))
        .await
        .unwrap();
    let mut rx = collect(shell.clone(), output);
    shell.resize(132, 40).await.unwrap();
    shell.write(b"stty size\n").await.unwrap();
    let (text, _) = read_until(&mut rx, "40 132").await;
    assert!(text.contains("40 132"), "{text:?}");
    shell.close().await;
    let (_, end) = read_until(&mut rx, "\u{0}never").await;
    assert!(matches!(end, Some(ExecEvent::Exit { .. })), "{end:?}");

    let missing = IncusTerminal::exec(&incus, "default", &name, &options(Some("/no/such/program")))
        .await
        .unwrap();
    let mut rx = collect(missing.0, missing.1);
    let (_, end) = read_until(&mut rx, "\u{0}never").await;
    assert!(
        matches!(&end, Some(ExecEvent::Exit { error: Some(e), .. }) if e.code == ErrorCode::NoShell),
        "{end:?}"
    );

    let (console, output) = IncusTerminal::console(&incus, "default", &name, 80, 24)
        .await
        .unwrap();
    let mut rx = collect(console.clone(), output);
    console.write(b"\n").await.unwrap();
    let (text, _) = read_until(&mut rx, "login").await;
    assert!(text.contains("login"), "{text:?}");
    console.close().await;

    instances::delete(&incus, "default", &name, true)
        .await
        .unwrap();
}

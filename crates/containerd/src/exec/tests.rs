use super::*;

fn spec() -> serde_json::Value {
    serde_json::json!({
        "ociVersion": "1.2.0",
        "process": {
            "args": ["nginx", "-g", "daemon off;"],
            "env": ["PATH=/usr/bin:/bin", "TERM=dumb"],
            "cwd": "/srv",
            "user": { "uid": 101, "gid": 101 },
            "capabilities": { "bounding": ["CAP_CHOWN"] }
        }
    })
}

#[test]
fn an_exec_starts_from_the_containers_own_process() {
    let process = process_spec(
        &spec(),
        &ContainerdExecOptions {
            cols: 120,
            rows: 40,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(process["args"][0], "sh");
    assert_eq!(process["args"][2], DEFAULT_SHELL);
    assert_eq!(process["terminal"], true);
    assert_eq!(
        process["consoleSize"],
        serde_json::json!({ "height": 40, "width": 120 })
    );
    assert_eq!(
        process["env"],
        serde_json::json!(["PATH=/usr/bin:/bin", "TERM=xterm-256color"])
    );
    assert_eq!(process["cwd"], "/srv");
    assert_eq!(process["user"]["uid"], 101);
    assert_eq!(process["capabilities"]["bounding"][0], "CAP_CHOWN");
}

#[test]
fn the_command_user_and_folder_can_be_chosen() {
    let process = process_spec(
        &spec(),
        &ContainerdExecOptions {
            command: Some("ls -la '/var/my dir'".into()),
            user: Some("0:10".into()),
            working_dir: Some("/tmp".into()),
            cols: 80,
            rows: 24,
        },
    )
    .unwrap();
    assert_eq!(
        process["args"],
        serde_json::json!(["ls", "-la", "/var/my dir"])
    );
    assert_eq!(process["user"], serde_json::json!({ "uid": 0, "gid": 10 }));
    assert_eq!(process["cwd"], "/tmp");

    for bad in [
        ContainerdExecOptions {
            user: Some("www-data".into()),
            ..Default::default()
        },
        ContainerdExecOptions {
            command: Some("echo 'open".into()),
            ..Default::default()
        },
    ] {
        assert!(process_spec(&spec(), &bad)
            .unwrap_err()
            .is(ErrorCode::Validation));
    }
}

#[test]
fn a_missing_program_is_told_apart() {
    assert!(no_shell(&tonic::Status::unknown(
        "OCI runtime exec failed: exec: \"sh\": executable file not found in $PATH"
    )));
    assert!(!no_shell(&tonic::Status::unknown("permission denied")));
}

mod live {
    use std::sync::Mutex as StdMutex;

    use super::super::*;
    use crate::testing::{self, NAMESPACE};

    async fn output_until(output: &StdMutex<String>, needle: &str) -> bool {
        for _ in 0..100 {
            if output.lock().unwrap().contains(needle) {
                return true;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        false
    }

    #[tokio::test]
    #[ignore = "needs a local containerd"]
    async fn containerd_live_exec_runs_a_shell_and_leaves_nothing_behind() {
        let client = testing::client().await;
        let image = testing::pulled();
        let id = testing::unique("homeport-test-exec");
        let _cleanup = testing::Cleanup(id.clone());
        testing::ctr(&["run", "-d", &image, &id, "sleep", "600"]);

        let (session, output) = start(
            &client,
            NAMESPACE,
            &id,
            ContainerdExecOptions {
                cols: 80,
                rows: 24,
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let seen = Arc::new(StdMutex::new(String::new()));
        let exit = Arc::new(StdMutex::new(None));
        let pumped = {
            let (seen, exit) = (seen.clone(), exit.clone());
            tokio::spawn(pump(session.clone(), output, move |event| {
                match event {
                    ExecEvent::Output { data } => seen.lock().unwrap().push_str(&data),
                    ExecEvent::Exit { code, .. } => *exit.lock().unwrap() = Some(code),
                    ExecEvent::Error { error } => panic!("{error:?}"),
                }
                true
            }))
        };
        session.write(b"echo homeport-$((40+2))\n").await.unwrap();
        assert!(
            output_until(&seen, "homeport-42").await,
            "{}",
            seen.lock().unwrap()
        );
        session.resize(100, 30).await.unwrap();
        session.write(b"stty size\n").await.unwrap();
        assert!(
            output_until(&seen, "30 100").await,
            "{}",
            seen.lock().unwrap()
        );
        session.write(b"exit 3\n").await.unwrap();
        pumped.await.unwrap();
        assert_eq!(*exit.lock().unwrap(), Some(Some(3)));
        drop(session);

        // No exec process or FIFO folder is left.
        tokio::time::sleep(Duration::from_millis(300)).await;
        assert!(!testing::ctr(&["task", "ps", &id]).contains("homeport-"));
        let leftovers = std::fs::read_dir(std::env::temp_dir())
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with("homeport-exec.")
            })
            .count();
        assert_eq!(leftovers, 0);

        // A program the image doesn't have says so.
        let missing = start(
            &client,
            NAMESPACE,
            &id,
            ContainerdExecOptions {
                command: Some("bash".into()),
                cols: 80,
                rows: 24,
                ..Default::default()
            },
        )
        .await;
        crate::containers::remove(&client, NAMESPACE, &id, true, false)
            .await
            .unwrap();
        let e = missing.err().expect("alpine has no bash");
        assert_eq!(e.code, ErrorCode::NoShell, "{e:?}");
    }
}

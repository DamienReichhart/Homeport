use super::*;

#[test]
fn snapshots_read_as_incus_lists_them() {
    // From GET /1.0/instances/t1/snapshots?recursion=1 on Incus 6.0.
    let raw: RawSnapshot = serde_json::from_value(serde_json::json!({
        "expires_at": "0001-01-01T00:00:00Z",
        "architecture": "x86_64",
        "created_at": "2026-11-08T23:06:10.95971139Z",
        "name": "snapA",
        "stateful": false,
        "size": -1
    }))
    .unwrap();
    let snapshot = raw.into_snapshot();
    assert_eq!(snapshot.name, "snapA");
    assert_eq!(snapshot.expires_at, None);
    assert_eq!(snapshot.size, None);
}

#[test]
fn a_new_snapshot_may_expire_but_not_in_the_past() {
    let mut request = SnapshotRequest {
        name: " before-upgrade ".into(),
        stateful: true,
        expires_at: None,
    };
    assert_eq!(
        create_body(&request).unwrap(),
        serde_json::json!({ "name": "before-upgrade", "stateful": true })
    );
    request.expires_at = Some("2999-01-01T00:00:00Z".into());
    assert_eq!(
        create_body(&request).unwrap()["expires_at"],
        "2999-01-01T00:00:00Z"
    );
    request.expires_at = Some("2020-01-01T00:00:00Z".into());
    assert_eq!(
        create_body(&request).unwrap_err().code,
        ErrorCode::Validation
    );
    request.expires_at = Some("tomorrow".into());
    assert_eq!(
        create_body(&request).unwrap_err().code,
        ErrorCode::Validation
    );
}

#[tokio::test]
#[ignore = "needs a local Incus"]
async fn incus_live_snapshots_are_taken_restored_and_deleted() {
    use crate::instances::{
        terminal::{IncusExecOptions, IncusTerminal},
        tests::live_instance,
        InstanceAction,
    };
    let incus = crate::instances::tests::live().await;
    let name = format!("homeport-snap-{}", std::process::id());
    live_instance(&incus, &name).await;
    instances::act(&incus, "default", &name, InstanceAction::Start, false)
        .await
        .unwrap();
    let run = |command: &str| IncusExecOptions {
        command: Some(command.to_owned()),
        user: None,
        cwd: None,
        cols: 80,
        rows: 24,
    };
    let exec = |command: &'static str| {
        let incus = incus.clone();
        let name = name.clone();
        async move {
            let (session, output) = IncusTerminal::exec(&incus, "default", &name, &run(command))
                .await
                .unwrap();
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            tokio::spawn(crate::instances::terminal::pump(
                session,
                output,
                move |e| tx.send(e).is_ok(),
            ));
            let mut text = String::new();
            while let Some(e) = rx.recv().await {
                match e {
                    homeport_core::terminal::ExecEvent::Output { data } => text.push_str(&data),
                    _ => break,
                }
            }
            text
        }
    };
    exec("touch /root/kept").await;

    let request = SnapshotRequest {
        name: String::new(),
        stateful: false,
        expires_at: None,
    };
    create(&incus, "default", &name, &request).await.unwrap();
    let taken = list(&incus, "default", &name).await.unwrap();
    assert_eq!(taken.len(), 1);
    assert_eq!(taken[0].name, "snap0");

    exec("rm /root/kept").await;
    restore(&incus, "default", &name, "snap0", false)
        .await
        .unwrap();
    let back = exec("ls /root").await;
    assert!(back.contains("kept"), "{back:?}");

    let again = create(
        &incus,
        "default",
        &name,
        &SnapshotRequest {
            name: "snap0".into(),
            stateful: false,
            expires_at: None,
        },
    )
    .await
    .unwrap_err();
    assert_eq!(again.code, ErrorCode::NameInUse, "{again:?}");

    delete(&incus, "default", &name, "snap0").await.unwrap();
    assert!(list(&incus, "default", &name).await.unwrap().is_empty());
    instances::delete(&incus, "default", &name, true)
        .await
        .unwrap();
}

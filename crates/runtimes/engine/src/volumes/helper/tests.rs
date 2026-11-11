use super::*;
use crate::{containers::files, testing};

#[test]
fn mounts_say_whether_they_are_read_only() {
    let ro = Mount {
        volume: "db",
        target: ROOT,
        read_only: true,
    };
    assert_eq!(ro.bind(), "db:/volume:ro");
    let rw = Mount {
        read_only: false,
        ..ro
    };
    assert_eq!(rw.bind(), "db:/volume:rw");
}

#[test]
fn helpers_are_told_apart_by_their_label() {
    assert!(is_helper(&HashMap::from([(LABEL.into(), "true".into())])));
    assert!(!is_helper(&HashMap::new()));
}

#[test]
fn a_missing_image_says_how_to_get_it() {
    let e = image_unavailable(None);
    assert_eq!(e.code, ErrorCode::HelperImageUnavailable);
    assert_eq!(e.hint.unwrap().commands, ["docker pull busybox:1.37"]);
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and busybox:1.37"]
async fn a_helper_lists_a_volume_and_leaves_nothing_behind() {
    let _serial = testing::HELPERS.lock().await;
    let docker = testing::docker();
    let volume = testing::filled_volume(&docker).await;
    let id = start(
        &docker,
        &[Mount {
            volume: &volume.name,
            target: ROOT,
            read_only: true,
        }],
    )
    .await
    .unwrap();

    // busybox's find and stat answer in the shape the file browser reads.
    let listed = files::list(&docker, &id, "/volume/etc").await;
    let dir = tempfile::tempdir().unwrap();
    let local = dir.path().join("new.txt");
    std::fs::write(&local, "nope").unwrap();
    let written = files::upload(&docker, &id, "/volume", &[local]).await;
    remove(&docker, &id).await;
    let listed = listed.unwrap();
    assert_eq!(listed.len(), 1, "{listed:?}");
    assert_eq!(listed[0].name, "note.txt");
    assert_eq!(listed[0].mode, "-rw-r-----");
    assert_eq!(listed[0].size, 6);
    assert!(written.is_err(), "the volume is mounted read-only");
    assert!(helpers(&docker, true).await.unwrap().is_empty());
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and busybox:1.37"]
async fn leftovers_of_this_computer_are_removed() {
    let _serial = testing::HELPERS.lock().await;
    let docker = testing::docker();
    let volume = testing::filled_volume(&docker).await;
    let mounts = [Mount {
        volume: &volume.name,
        target: ROOT,
        read_only: true,
    }];
    let open = start(&docker, &mounts).await.unwrap();
    start(&docker, &mounts).await.unwrap();
    create_stopped(&docker, &mounts).await.unwrap();
    assert_eq!(
        remove_leftovers(&docker, std::slice::from_ref(&open))
            .await
            .unwrap(),
        2
    );
    assert_eq!(
        helpers(&docker, true).await.unwrap(),
        std::slice::from_ref(&open)
    );
    remove(&docker, &open).await;
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and busybox:1.37"]
async fn a_run_reports_its_exit_code_and_stderr() {
    let _serial = testing::HELPERS.lock().await;
    let docker = testing::docker();
    let volume = testing::filled_volume(&docker).await;
    let mounts = [Mount {
        volume: &volume.name,
        target: ROOT,
        read_only: true,
    }];
    let (code, stderr) = run(&docker, &mounts, &["touch", "/volume/x"])
        .await
        .unwrap();
    assert_ne!(code, 0);
    assert!(stderr.contains("Read-only file system"), "{stderr}");
    let (code, _) = run(&docker, &mounts, &["test", "-f", "/volume/etc/note.txt"])
        .await
        .unwrap();
    assert_eq!(code, 0);
    assert!(helpers(&docker, true).await.unwrap().is_empty());
}

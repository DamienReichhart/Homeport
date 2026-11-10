use std::sync::{Arc, Mutex};

use super::*;
use crate::testing;

fn recorder() -> (
    impl Fn(TransferEvent) -> bool,
    Arc<Mutex<Vec<TransferEvent>>>,
) {
    let events = Arc::new(Mutex::new(Vec::new()));
    let sink = {
        let events = events.clone();
        move |e| {
            events.lock().unwrap().push(e);
            true
        }
    };
    (sink, events)
}

fn last(events: &Mutex<Vec<TransferEvent>>) -> TransferEvent {
    events.lock().unwrap().last().cloned().unwrap()
}

#[test]
fn only_tar_archives_are_restored() {
    let dir = tempfile::tempdir().unwrap();
    let garbage = dir.path().join("notes.tar");
    std::fs::write(&garbage, "not an archive").unwrap();
    assert_eq!(
        check_tar(&garbage).unwrap_err().code,
        ErrorCode::NotATarArchive
    );

    let archive = dir.path().join("ok.tar");
    let mut builder = tar::Builder::new(std::fs::File::create(&archive).unwrap());
    let mut header = tar::Header::new_ustar();
    header.set_size(2);
    header.set_cksum();
    builder.append_data(&mut header, "./a", &b"hi"[..]).unwrap();
    builder.finish().unwrap();
    check_tar(&archive).unwrap();
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and busybox:1.37"]
async fn a_backup_restores_into_a_new_volume_with_owners_and_modes() {
    let _serial = testing::HELPERS.lock().await;
    let docker = testing::docker();
    let original = testing::filled_volume(&docker).await;
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("backup.tar");

    let (sink, events) = recorder();
    backup(
        docker.clone(),
        original.name.clone(),
        file.clone(),
        false,
        sink,
    )
    .await;
    assert_eq!(last(&events), TransferEvent::Done { images: vec![] });
    // The layout of `tar -C <volume> -c .`.
    let mut names: Vec<String> = tar::Archive::new(std::fs::File::open(&file).unwrap())
        .entries()
        .unwrap()
        .map(|e| e.unwrap().path().unwrap().display().to_string())
        .collect();
    names.sort();
    assert_eq!(names, ["./", "./etc/", "./etc/note.txt"]);

    let copy = testing::VolumeGuard {
        docker: docker.clone(),
        name: testing::unique("restored"),
    };
    let (sink, events) = recorder();
    let spec = RestoreSpec {
        volume: copy.name.clone(),
        create: true,
        replace: false,
    };
    restore(docker.clone(), file.clone(), spec.clone(), false, sink).await;
    assert_eq!(last(&events), TransferEvent::Done { images: vec![] });
    let listing = testing::output_with_volume(
        &docker,
        &copy.name,
        "stat -c '%u:%g %a %s' /data/etc/note.txt",
    )
    .await;
    assert_eq!(listing.trim(), "1234:5678 640 6");

    // Into it again: it now holds files, so only with `replace`.
    let (sink, events) = recorder();
    restore(
        docker.clone(),
        file.clone(),
        RestoreSpec {
            create: false,
            ..spec.clone()
        },
        false,
        sink,
    )
    .await;
    let TransferEvent::Error { error } = last(&events) else {
        panic!("restored into a volume holding files");
    };
    assert_eq!(error.code, ErrorCode::VolumeNotEmpty);

    testing::run_with_volume(&docker, &copy.name, "echo stale > /data/stale").await;
    let (sink, events) = recorder();
    let replace = RestoreSpec {
        create: false,
        replace: true,
        ..spec
    };
    restore(docker.clone(), file, replace, false, sink).await;
    assert_eq!(last(&events), TransferEvent::Done { images: vec![] });
    let listing = testing::output_with_volume(&docker, &copy.name, "ls -A /data").await;
    assert_eq!(listing.trim(), "etc");
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and busybox:1.37"]
async fn a_running_user_stops_a_restore_but_not_a_backup() {
    let _serial = testing::HELPERS.lock().await;
    let docker = testing::docker();
    let volume = testing::filled_volume(&docker).await;
    let user = testing::container_with_volume(&docker, &volume.name).await;
    docker.start_container(&user.id, None).await.unwrap();
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("backup.tar");

    assert_eq!(running_users(&docker, &volume.name).await.unwrap().len(), 1);
    let (sink, events) = recorder();
    backup(
        docker.clone(),
        volume.name.clone(),
        file.clone(),
        false,
        sink,
    )
    .await;
    assert_eq!(last(&events), TransferEvent::Done { images: vec![] });

    let (sink, events) = recorder();
    let spec = RestoreSpec {
        volume: volume.name.clone(),
        create: false,
        replace: true,
    };
    restore(docker.clone(), file, spec, false, sink).await;
    let TransferEvent::Error { error } = last(&events) else {
        panic!("restored into a volume a running container uses");
    };
    assert_eq!(error.code, ErrorCode::ContainerRunning);
}

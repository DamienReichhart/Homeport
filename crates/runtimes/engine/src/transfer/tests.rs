use std::sync::Mutex;

use super::*;
use crate::{images, testing};

fn recorder() -> (
    impl Fn(TransferEvent) -> bool,
    std::sync::Arc<Mutex<Vec<TransferEvent>>>,
) {
    let events = std::sync::Arc::new(Mutex::new(Vec::new()));
    let sink = {
        let events = events.clone();
        move |e| {
            events.lock().unwrap().push(e);
            true
        }
    };
    (sink, events)
}

#[test]
fn unreadable_archives_are_explained() {
    let e = invalid_archive(
        AppError::new(ErrorCode::EngineError, msg::TRANSFER_EMPTY_ARCHIVE)
            .with_detail("archive/tar: invalid tar header"),
        Path::new("/tmp/x.tar"),
    );
    assert_eq!(e.code, ErrorCode::BadRequest);
    assert!(e.message.contains("/tmp/x.tar"));
    let other = AppError::new(ErrorCode::DiskFull, msg::TRANSFER_EMPTY_ARCHIVE)
        .with_detail("no space left on device");
    assert_eq!(invalid_archive(other.clone(), Path::new("x")), other);
}

#[tokio::test]
async fn a_failed_save_over_a_file_leaves_that_file_as_it_was() {
    let dir = tempfile::tempdir().unwrap();
    // Last week's backup of a volume, picked again in the save dialog.
    let path = dir.path().join("db.tar");
    std::fs::write(&path, "last week's backup").unwrap();
    let broken = futures_util::stream::iter(vec![
        Ok(Bytes::from_static(b"the first part")),
        Err(bollard::errors::Error::IOError {
            err: io::Error::from(io::ErrorKind::ConnectionReset),
        }),
    ]);
    let (sink, _) = recorder();
    assert!(write_file(broken, &path, &sink).await.is_err());
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        "last week's backup"
    );
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);

    let whole = futures_util::stream::iter(vec![Ok(Bytes::from_static(b"this week's"))]);
    write_file(whole, &path, &sink).await.unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "this week's");
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}

#[tokio::test]
async fn saving_to_a_folder_that_does_not_exist_fails_cleanly() {
    let (sink, events) = recorder();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing").join("a.tar");
    save(testing::offline(), vec!["x".into()], path.clone(), sink).await;
    let events = events.lock().unwrap();
    assert!(
        matches!(events.last(), Some(TransferEvent::Error { .. })),
        "{events:?}"
    );
    assert!(!path.exists());
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn images_saved_to_a_file_load_back_with_their_names() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let name = format!("{}:v1", testing::unique("save"));
    images::tag(&docker, testing::IMAGE, &name).await.unwrap();
    let _cleanup = testing::ImageName(docker.clone(), name.clone());
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("images.tar");

    let (sink, events) = recorder();
    save(docker.clone(), vec![name.clone()], path.clone(), sink).await;
    assert_eq!(
        events.lock().unwrap().last(),
        Some(&TransferEvent::Done {
            images: vec![name.clone()]
        })
    );
    assert!(std::fs::metadata(&path).unwrap().len() > 0);

    images::remove(&docker, &name, false).await.unwrap();
    let (sink, events) = recorder();
    load(docker.clone(), path, sink).await;
    let last = events.lock().unwrap().last().cloned();
    assert_eq!(
        last,
        Some(TransferEvent::Done {
            images: vec![name.clone()]
        })
    );
    assert!(docker.inspect_image(&name).await.is_ok());
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn loading_a_file_that_is_not_an_archive_says_so() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("notes.tar");
    std::fs::write(&path, "these are not image layers").unwrap();
    let (sink, events) = recorder();
    load(testing::docker(), path, sink).await;
    let events = events.lock().unwrap();
    match events.last() {
        Some(TransferEvent::Error { error }) => {
            assert_eq!(error.code, ErrorCode::BadRequest, "{error:?}");
        }
        other => panic!("{other:?}"),
    }
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn an_exported_container_imports_back_as_a_runnable_image() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let guard = testing::container(
        &docker,
        &testing::unique("export"),
        &["sh", "-c", "echo exported > /exported.txt && sleep 60"],
    )
    .await;
    crate::containers::act(
        &docker,
        &guard.id,
        crate::containers::ContainerAction::Start,
    )
    .await
    .unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("rootfs.tar");

    let (sink, events) = recorder();
    export(docker.clone(), guard.id.clone(), path.clone(), sink).await;
    assert_eq!(
        events.lock().unwrap().last(),
        Some(&TransferEvent::Done { images: vec![] })
    );
    let mut archive = tar::Archive::new(std::fs::File::open(&path).unwrap());
    assert!(archive.entries().unwrap().any(|e| e
        .unwrap()
        .path()
        .unwrap()
        .ends_with("exported.txt")));

    let name = format!("{}:1", testing::unique("imported"));
    let _cleanup = testing::ImageName(docker.clone(), name.clone());
    let (sink, events) = recorder();
    import(
        docker.clone(),
        path,
        ImportSpec {
            reference: name.clone(),
            changes: vec![r#"CMD ["cat", "/exported.txt"]"#.into()],
        },
        sink,
    )
    .await;
    assert_eq!(
        events.lock().unwrap().last(),
        Some(&TransferEvent::Done {
            images: vec![name.clone()]
        })
    );
    let check = crate::containers::run::run(
        &docker,
        crate::containers::run::RunSpec {
            image: name.clone(),
            name: Some(testing::unique("import-check")),
            ..Default::default()
        },
        None,
    )
    .await
    .unwrap();
    let exit = loop {
        let state = docker
            .inspect_container(&check, None)
            .await
            .unwrap()
            .state
            .unwrap();
        if state.running != Some(true) {
            break state.exit_code;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    };
    crate::containers::remove(&docker, &check, true, true)
        .await
        .unwrap();
    assert_eq!(exit, Some(0));
}

#[tokio::test]
async fn exporting_to_a_folder_that_does_not_exist_fails_cleanly() {
    let (sink, events) = recorder();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing").join("rootfs.tar");
    export(testing::offline(), "nope".into(), path.clone(), sink).await;
    assert!(matches!(
        events.lock().unwrap().last(),
        Some(TransferEvent::Error { .. })
    ));
    assert!(!path.exists());
}

#[tokio::test]
async fn imports_need_a_name() {
    let (sink, events) = recorder();
    import(
        testing::offline(),
        "/nonexistent.tar".into(),
        ImportSpec {
            reference: " ".into(),
            changes: vec![],
        },
        sink,
    )
    .await;
    let last = events.lock().unwrap().last().cloned();
    match last {
        Some(TransferEvent::Error { error }) => assert_eq!(error.code, ErrorCode::Validation),
        other => panic!("{other:?}"),
    }
}

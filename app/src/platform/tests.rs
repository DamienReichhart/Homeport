use std::{ffi::OsString, io, path::PathBuf};

use homeport_core::error::ErrorCode;

use super::{
    dialogs::{Filter, OpenFile, OpenFolder, SaveFile},
    fake::{Asked, Fake},
};
use crate::test_support::tasks::messages;

#[tokio::test]
async fn a_cancelled_dialog_answers_nothing_and_is_no_failure() {
    let fake = Fake::new();
    let platform = fake.platform();
    // No answer was given to the fake: every dialog is cancelled.
    assert_eq!(
        messages(platform.pick_file(OpenFile::new("Open"))).await,
        [None]
    );
    assert_eq!(
        messages(platform.pick_files(OpenFile::new("Open"))).await,
        [Vec::<PathBuf>::new()]
    );
    assert_eq!(
        messages(platform.pick_folder(OpenFolder::new("Pick"))).await,
        [None]
    );
    assert_eq!(
        messages(platform.save_file(SaveFile::new("Save", "logs.txt"))).await,
        [None]
    );
    assert_eq!(fake.took().len(), 4);
}

#[tokio::test]
async fn a_dialog_is_asked_as_requested_and_answers_what_was_picked() {
    let fake = Fake::new();
    let platform = fake.platform();
    let yaml = Filter::new("Compose files", &["yml", "yaml"]);
    let open = OpenFile::new("Choose a compose file")
        .filter(yaml.clone())
        .starting_in("/srv/shop");

    fake.answers(vec![
        "/srv/shop/compose.yml".into(),
        "/srv/shop/extra.yml".into(),
    ]);
    let one = messages(platform.pick_file(open.clone())).await;
    assert_eq!(
        one,
        [Some(PathBuf::from("/srv/shop/compose.yml"))],
        "one file was asked for"
    );

    fake.answers(vec!["/a".into(), "/b".into()]);
    let many = messages(platform.pick_files(open.clone())).await;
    assert_eq!(many, [vec![PathBuf::from("/a"), PathBuf::from("/b")]]);

    fake.answer("/srv");
    let folder = messages(platform.pick_folder(OpenFolder::new("Pick a folder"))).await;
    assert_eq!(folder, [Some(PathBuf::from("/srv"))]);

    fake.answer("/home/ada/web.tar");
    let save = SaveFile::new("Export", "web/1.tar").filter(Filter::new("Archives", &["tar"]));
    let saved = messages(platform.save_file(save.clone())).await;
    assert_eq!(saved, [Some(PathBuf::from("/home/ada/web.tar"))]);
    assert_eq!(save.name, "web_1.tar");

    assert_eq!(
        fake.took(),
        [
            Asked::Open(open.clone(), false),
            Asked::Open(open, true),
            Asked::Folder(OpenFolder::new("Pick a folder")),
            Asked::Save(save),
        ]
    );
}

#[tokio::test]
async fn a_link_goes_to_the_os_and_anything_else_never_does() {
    let fake = Fake::new();
    let platform = fake.platform();
    platform
        .open_url("http://localhost:8080".to_owned())
        .await
        .unwrap();
    assert_eq!(fake.took(), [Asked::Opened("http://localhost:8080".into())]);

    let refused = platform
        .open_url("file:///etc/passwd".to_owned())
        .await
        .unwrap_err();
    assert_eq!(refused.message, "file:///etc/passwd can't be opened.");
    assert!(fake.took().is_empty(), "the OS was never asked");

    // The OS has no browser: the failure names the link and keeps why.
    fake.refuse(io::ErrorKind::NotFound);
    let failed = platform
        .open_url("https://example.com".to_owned())
        .await
        .unwrap_err();
    assert_eq!(failed.code, ErrorCode::Io);
    assert_eq!(failed.message, "https://example.com can't be opened.");
    assert_eq!(failed.key, "errors.file.linkNotOpened");
    assert_eq!(
        failed.hint.map(|hint| hint.text).as_deref(),
        Some("Copy the address and open it in your browser instead.")
    );
    assert!(failed.detail.is_some());
}

#[tokio::test]
async fn folders_are_made_then_opened_and_a_failure_names_the_folder() {
    let fake = Fake::new();
    let platform = fake.platform();
    let logs = platform.log_dir().to_owned();
    assert!(logs.starts_with(fake.dir()) && !logs.exists());
    platform.open_log_folder().await.unwrap();
    assert!(logs.is_dir(), "the folder is made so it can be shown");
    assert_eq!(fake.took(), [Asked::Opened(OsString::from(&logs))]);

    let exports = fake.dir().join("exports/today");
    platform.open_folder(exports.clone()).await.unwrap();
    assert!(exports.is_dir());
    assert_eq!(fake.took(), [Asked::Opened(OsString::from(&exports))]);

    fake.refuse(io::ErrorKind::NotFound);
    let failed = platform.open_log_folder().await.unwrap_err();
    assert_eq!(
        failed.message,
        format!("{} can't be opened.", logs.display())
    );
    assert_eq!(
        failed.hint.map(|hint| hint.text).as_deref(),
        Some("Open it in your file manager instead.")
    );
}

#[tokio::test]
async fn the_data_folder_opened_is_the_one_the_database_is_in() {
    let app = crate::app::testing::app();
    let state = std::sync::Arc::new(app.state);
    let fake = Fake::new();
    fake.platform().open_data_folder(&state).await.unwrap();
    assert!(state.data_dir.join("homeport.db").exists());
    assert_eq!(
        fake.took(),
        [Asked::Opened(OsString::from(&state.data_dir))]
    );
}

#[tokio::test]
async fn only_what_exists_is_opened_or_shown_in_its_folder() {
    let fake = Fake::new();
    let platform = fake.platform();
    let file = fake.dir().join("logs.txt");
    let gone = platform.reveal(file.clone()).await.unwrap_err();
    assert_eq!(gone.message, format!("{} doesn't exist.", file.display()));
    assert!(platform.open_path(file.clone()).await.is_err());
    assert!(fake.took().is_empty());

    std::fs::write(&file, "one\n").unwrap();
    platform.reveal(file.clone()).await.unwrap();
    platform.open_path(file.clone()).await.unwrap();
    assert_eq!(
        fake.took(),
        [
            Asked::Revealed(file.clone()),
            Asked::Opened(file.clone().into())
        ]
    );

    fake.refuse(io::ErrorKind::PermissionDenied);
    let failed = platform.reveal(file.clone()).await.unwrap_err();
    assert_eq!(
        failed.message,
        format!("{} can't be opened.", file.display())
    );
}

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
        [Ok(None)]
    );
    assert_eq!(
        messages(platform.pick_files(OpenFile::new("Open"))).await,
        [Ok(Vec::<PathBuf>::new())]
    );
    assert_eq!(
        messages(platform.pick_folder(OpenFolder::new("Pick"))).await,
        [Ok(None)]
    );
    assert_eq!(
        messages(platform.save_file(SaveFile::new("Save", "logs.txt"))).await,
        [Ok(None)]
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
        [Ok(Some(PathBuf::from("/srv/shop/compose.yml")))],
        "one file was asked for"
    );

    fake.answers(vec!["/a".into(), "/b".into()]);
    let many = messages(platform.pick_files(open.clone())).await;
    assert_eq!(many, [Ok(vec![PathBuf::from("/a"), PathBuf::from("/b")])]);

    fake.answer("/srv");
    let folder = messages(platform.pick_folder(OpenFolder::new("Pick a folder"))).await;
    assert_eq!(folder, [Ok(Some(PathBuf::from("/srv")))]);

    fake.answer("/home/ada/web.tar");
    let save = SaveFile::new("Export", "web/1.tar").filter(Filter::new("Archives", &["tar"]));
    let saved = messages(platform.save_file(save.clone())).await;
    assert_eq!(saved, [Ok(Some(PathBuf::from("/home/ada/web.tar")))]);
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
    assert_eq!(refused.key, "errors.file.linkRefused");
    assert!(refused.hint.is_none(), "there is nowhere else to open it");
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
async fn only_what_exists_is_shown_in_its_folder() {
    let fake = Fake::new();
    let platform = fake.platform();
    let file = fake.dir().join("logs.txt");
    let gone = platform.reveal(file.clone()).await.unwrap_err();
    assert_eq!(gone.message, format!("{} doesn't exist.", file.display()));
    assert!(fake.took().is_empty());

    std::fs::write(&file, "one\n").unwrap();
    platform.reveal(file.clone()).await.unwrap();
    assert_eq!(fake.took(), [Asked::Revealed(file.clone())]);

    // A path from here is handed over whole.
    let here = std::env::current_dir().unwrap();
    let relative = pathdiff(&file, &here);
    assert!(relative.is_relative());
    platform.reveal(relative).await.unwrap();
    let asked = fake.took();
    let [Asked::Revealed(whole)] = asked.as_slice() else {
        panic!("{asked:?}");
    };
    assert!(whole.is_absolute(), "{}", whole.display());
    assert_eq!(
        std::fs::canonicalize(whole).unwrap(),
        std::fs::canonicalize(&file).unwrap()
    );

    fake.refuse(io::ErrorKind::PermissionDenied);
    let failed = platform.reveal(file.clone()).await.unwrap_err();
    assert_eq!(
        failed.message,
        format!("{} can't be opened.", file.display())
    );
}

/// The OS runs a file handed to it when it is a program, a script or a
/// shortcut: nothing of the platform hands it one.
#[tokio::test]
async fn a_file_is_never_handed_to_the_os_to_open() {
    let fake = Fake::new();
    let platform = fake.platform();
    for name in [
        "run.sh",
        "setup.exe",
        "Homeport.desktop",
        "link.url",
        "notes.txt",
    ] {
        let file = fake.dir().join(name);
        std::fs::write(&file, "#!/bin/sh\ntouch /tmp/ran\n").unwrap();
        let refused = platform.open_folder(file.clone()).await.unwrap_err();
        assert!(refused.message.contains(name), "{}", refused.message);
        assert!(file.is_file(), "left as it was");
    }
    // A link that leads to a file is a file too.
    #[cfg(unix)]
    {
        let link = fake.dir().join("folder");
        std::os::unix::fs::symlink(fake.dir().join("run.sh"), &link).unwrap();
        platform.open_folder(link).await.unwrap_err();
    }
    assert!(fake.took().is_empty(), "the OS was never asked");

    // And a link is only ever a web, mail or phone one: never a path.
    for path in [
        fake.dir().join("run.sh").display().to_string(),
        "run.sh".to_owned(),
    ] {
        platform.open_url(path).await.unwrap_err();
    }
    assert!(fake.took().is_empty(), "the OS was never asked");
}

#[tokio::test]
async fn a_desktop_that_can_show_no_dialog_is_a_failure_and_never_a_cancel() {
    let fake = Fake::new();
    let platform = fake.platform();
    fake.without_dialogs();
    // An answer is waiting: it must not be what comes back.
    fake.answer("/home/ada/logs.txt");
    let saved = messages(platform.save_file(SaveFile::new("Save", "logs.txt"))).await;
    let [Err(failed)] = saved.as_slice() else {
        panic!("a failure: {saved:?}");
    };
    assert_eq!(failed.key, "errors.file.dialogUnavailable");
    assert_eq!(failed.message, "Homeport couldn't open a file dialog.");
    let hint = failed.hint.as_ref().expect("what to install");
    assert!(hint.text.contains("xdg-desktop-portal"), "{}", hint.text);
    for asked in [
        messages(platform.pick_file(OpenFile::new("Open")))
            .await
            .pop()
            .map(|r| r.is_err()),
        messages(platform.pick_folder(OpenFolder::new("Pick")))
            .await
            .pop()
            .map(|r| r.is_err()),
        messages(platform.pick_files(OpenFile::new("Open")))
            .await
            .pop()
            .map(|r| r.is_err()),
    ] {
        assert_eq!(asked, Some(true));
    }
}

#[tokio::test]
async fn a_dialog_asked_for_while_one_is_open_answers_as_cancelled() {
    let fake = Fake::new();
    let platform = fake.platform();
    fake.answer("/home/ada/first.txt");
    fake.answer("/home/ada/second.txt");
    // The first dialog is open: its task has not answered yet.
    let first = platform.save_file(SaveFile::new("Save", "first.txt"));
    // A second press, on any dialog of the window, even from a copy of
    // the platform another page holds.
    let again = messages(
        platform
            .clone()
            .save_file(SaveFile::new("Save", "again.txt")),
    )
    .await;
    assert_eq!(again, [Ok(None)]);
    let files = messages(platform.pick_files(OpenFile::new("Open"))).await;
    assert_eq!(files, [Ok(vec![])]);
    assert_eq!(fake.took().len(), 1, "the OS was asked for one dialog");
    // The first answers what the user picked in it.
    assert_eq!(
        messages(first).await,
        [Ok(Some(PathBuf::from("/home/ada/first.txt")))]
    );
    // It is closed: the next one opens.
    let next = messages(platform.save_file(SaveFile::new("Save", "next.txt"))).await;
    assert_eq!(next, [Ok(Some(PathBuf::from("/home/ada/second.txt")))]);

    // A dialog whose task is dropped without having run holds nothing.
    drop(platform.pick_folder(OpenFolder::new("Pick")));
    fake.answer("/srv");
    let folder = messages(platform.pick_folder(OpenFolder::new("Pick"))).await;
    assert_eq!(folder, [Ok(Some(PathBuf::from("/srv")))]);
}

/// `path` as it is written from `here`: up to the root, then down.
fn pathdiff(path: &std::path::Path, here: &std::path::Path) -> PathBuf {
    let up: PathBuf = here.components().skip(1).map(|_| "..").collect();
    up.join(path.strip_prefix("/").unwrap_or(path))
}

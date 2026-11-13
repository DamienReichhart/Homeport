//! The Files tab's transfers against an Engine: the archive saved where
//! the dialog said, files put one at a time with their progress, a
//! failure part way, and a transfer stopped.

use std::time::Duration;

use super::{streams::Live, *};
use crate::{
    test_support::{engine::Stream, tasks::messages},
    ui::{
        features::docker::containers::detail::tabs::files::{self, Doing},
        shell::page::testing::Asked,
        widgets::data::file_browser,
    },
};

fn browse(message: file_browser::Message) -> Message {
    Message::Files(files::Message::Browser(message))
}

/// The page on its Files tab, `/` holding what `entries` says.
fn live(entries: Vec<homeport_engine::containers::files::FileEntry>) -> Live {
    let mut live = Live::at("files");
    let host = live.rig.state.host_id.clone();
    let key = crate::ui::features::docker::containers::detail::queries::files_key(&host, WEB, "/");
    live.rig.bench.data.set(&key, entries);
    live.rig.refresh();
    let _ = live.rig.bench.took();
    live
}

fn entry(name: &str, folder: bool) -> homeport_engine::containers::files::FileEntry {
    use homeport_engine::containers::files::{FileEntry, FileKind};
    FileEntry {
        name: name.to_owned(),
        path: format!("/{name}"),
        kind: if folder {
            FileKind::Directory
        } else {
            FileKind::File
        },
        size: 6,
        modified: 1_700_000_000,
        mode: "-rw-r--r--".to_owned(),
    }
}

#[test]
fn a_file_is_saved_as_itself_and_a_folder_as_an_archive() {
    let _english = crate::test_support::snapshot::in_language("en");
    let mut live = live(vec![entry("etc", true), entry("note.txt", false)]);
    live.engine.file("/note.txt", b"hello\n");
    live.engine.folder("/etc");
    let saved = live.rig.bench.os.dir().join("note.txt");
    live.rig.bench.os.answer(&saved);
    live.send(browse(file_browser::Message::Download(
        "/note.txt".to_owned(),
    )));
    assert_eq!(std::fs::read(&saved).unwrap(), b"hello\n");
    assert_eq!(
        live.rig.bench.took(),
        [Asked::Said("Saved note.txt".to_owned())]
    );
    let archive = live.rig.bench.os.dir().join("etc.tar");
    live.rig.bench.os.answer(&archive);
    live.send(browse(file_browser::Message::Download("/etc".to_owned())));
    let mut tar = tar::Archive::new(std::fs::File::open(&archive).unwrap());
    let names: Vec<String> = tar
        .entries()
        .unwrap()
        .map(|e| e.unwrap().path().unwrap().display().to_string())
        .collect();
    assert_eq!(names, ["etc/inside.txt"]);
    assert_eq!(
        live.rig.bench.took(),
        [Asked::Said("Saved etc as a .tar archive".to_owned())]
    );
    assert!(live.rig.state.files.transfer.is_none());
}

/// Three files of this computer to put in the container.
fn sources(live: &Live) -> Vec<std::path::PathBuf> {
    ["a.txt", "b.txt", "c.txt"]
        .iter()
        .map(|name| {
            let path = live.rig.bench.os.dir().join(name);
            std::fs::write(&path, name).unwrap();
            path
        })
        .collect()
}

#[test]
fn files_are_put_one_at_a_time_and_a_failure_is_said_with_the_file() {
    let _english = crate::test_support::snapshot::in_language("en");
    let mut live = live(vec![]);
    let picked = sources(&live);
    live.engine.refuse_upload_of("b.txt");
    live.rig.bench.os.answers(picked);
    live.send(Message::Files(files::Message::Upload));
    let uploaded = live.engine.uploaded();
    let names: Vec<&[String]> = uploaded.iter().map(|(_, names)| names.as_slice()).collect();
    assert_eq!(
        names,
        [&["a.txt".to_owned()][..], &["c.txt".to_owned()][..]]
    );
    assert!(uploaded.iter().all(|(folder, _)| folder == "/"));
    let told = live.rig.bench.took();
    let host = live.rig.state.host_id.clone();
    let listing =
        crate::ui::features::docker::containers::detail::queries::files_key(&host, WEB, "/");
    assert!(
        told.contains(&Asked::Said("Uploaded 2 files to /".to_owned())),
        "{told:?}"
    );
    assert!(told.contains(&Asked::Invalidate(listing)), "{told:?}");
    let failed = told.iter().find_map(|asked| match asked {
        Asked::Failed(None, error) => Some(error.clone()),
        _ => None,
    });
    let failed = failed.expect("the failure of b.txt");
    assert!(failed.detail.unwrap_or_default().starts_with("b.txt: "));
    assert!(live.rig.state.files.transfer.is_none());
}

#[test]
fn a_transfer_shows_how_far_it_is_and_cancel_stops_it_where_it_is() {
    let english = crate::test_support::snapshot::in_language("en");
    let mut live = live(vec![]);
    let picked = sources(&live);
    live.engine.hold_transfers();
    live.rig.bench.os.answers(picked);
    // The dialog's answer, then the transfer, which waits on the Engine.
    let ctx = live.rig.bench.ctx();
    let asked = update(
        &mut live.rig.state,
        Message::Files(files::Message::Upload),
        &ctx,
    );
    let answer = live.rig.bench.run(asked);
    let ctx = live.rig.bench.ctx();
    let mut running = vec![];
    for message in answer {
        running.push(update(&mut live.rig.state, message, &ctx));
    }
    let transfer = live
        .rig
        .bench
        .app
        .runtime
        .spawn(messages(iced::Task::batch(running)));
    assert!(matches!(
        live.rig.state.files.transfer.as_ref().map(|t| &t.doing),
        Some(Doing::Upload {
            done: 0,
            count: 3,
            ..
        })
    ));
    assert!(live.rig.shows("Uploading 0 of 3 to \u{2066}/\u{2069}…"));
    drop(english);
    let theme = crate::ui::theme::Theme::Light;
    crate::test_support::snapshot::assert_snapshot(
        "container-files-uploading-en",
        "en",
        &theme,
        SIZE,
        || live.rig.view(),
    );
    crate::test_support::snapshot::assert_snapshot(
        "container-files-uploading-ar",
        "ar",
        &theme,
        SIZE,
        || live.rig.view(),
    );
    let _english = crate::test_support::snapshot::in_language("en");
    let engine = &live.engine;
    assert!(live
        .rig
        .bench
        .app
        .runtime
        .block_on(engine.settles_at(Stream::Transfer, 1)));
    // Cancel: the request that waits is dropped, nothing more is put.
    live.rig.frame(|frame| frame.click("Cancel"));
    let ended = live
        .rig
        .bench
        .app
        .runtime
        .block_on(async { tokio::time::timeout(Duration::from_secs(5), transfer).await })
        .expect("the transfer ends")
        .expect("it ran");
    for message in ended {
        live.send(message);
    }
    assert!(live.rig.state.files.transfer.is_none());
    assert_eq!(live.engine.abandoned(), 1);
    live.engine.release();
    assert!(live.engine.uploaded().is_empty());
    // Nothing was put: nothing is said, nothing is read again.
    assert!(live.rig.bench.took().is_empty());
}

#[test]
fn the_count_moves_as_each_file_is_put() {
    let mut live = live(vec![]);
    let picked = sources(&live);
    live.rig.bench.os.answers(picked);
    let ctx = live.rig.bench.ctx();
    let asked = update(
        &mut live.rig.state,
        Message::Files(files::Message::Upload),
        &ctx,
    );
    let answer = live.rig.bench.run(asked);
    let ctx = live.rig.bench.ctx();
    let task = iced::Task::batch(
        answer
            .into_iter()
            .map(|m| update(&mut live.rig.state, m, &ctx))
            .collect::<Vec<_>>(),
    );
    let produced = live.rig.bench.run(task);
    let steps: Vec<usize> = produced
        .iter()
        .filter_map(|message| match message {
            Message::Files(files::Message::Progress(_, done)) => Some(*done),
            _ => None,
        })
        .collect();
    assert!(
        !steps.is_empty() && steps.windows(2).all(|pair| pair[0] < pair[1]),
        "{steps:?}"
    );
    assert_eq!(steps.last(), Some(&3));
    // A step of a transfer that is not the one that runs changes nothing.
    for message in produced {
        live.send(message);
    }
    live.rig.state.files.progress(99, 1);
    assert!(live.rig.state.files.transfer.is_none());
}

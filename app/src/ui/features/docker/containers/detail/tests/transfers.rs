//! The Files tab's transfers against an Engine: the archive saved where
//! the dialog said, files put one at a time with their progress, a
//! failure part way, and a transfer stopped.

use std::time::Duration;

use super::{streams::Live, *};
use crate::{
    test_support::{
        engine::{Stall, Stream},
        tasks::messages,
    },
    ui::{
        features::docker::containers::detail::tabs::files::{self, Doing},
        shell::page::testing::Asked,
        widgets::{basic::technical, data::file_browser},
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
    let told = live.rig.bench.took();
    assert!(
        told.contains(&Asked::Said("Saved note.txt".to_owned())),
        "{told:?}"
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

/// Answers the dialog `ask` opens and starts the transfer it leads to,
/// which runs on the side while the test goes on.
fn begin(live: &mut Live, ask: Message) -> tokio::task::JoinHandle<Vec<Message>> {
    let runtime = live.rig.bench.app.runtime.handle().clone();
    let _inside = runtime.enter();
    let ctx = live.rig.bench.ctx();
    let asked = update(&mut live.rig.state, ask, &ctx);
    let answer = live.rig.bench.run(asked);
    let ctx = live.rig.bench.ctx();
    let running: Vec<_> = answer
        .into_iter()
        .map(|message| update(&mut live.rig.state, message, &ctx))
        .collect();
    runtime.spawn(messages(iced::Task::batch(running)))
}

/// Waits for the transfer to end, and gives the page what it sent.
fn finish(live: &mut Live, transfer: tokio::task::JoinHandle<Vec<Message>>) {
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
}

/// Waits until the Engine holds `count` transfers.
fn holds(live: &Live, count: usize) {
    let engine = &live.engine;
    let settled = live
        .rig
        .bench
        .app
        .runtime
        .block_on(engine.settles_at(Stream::Transfer, count));
    assert!(settled, "{count} transfers held");
}

#[test]
fn an_upload_shows_how_far_it_is_and_cancel_stops_before_the_next_file() {
    let english = crate::test_support::snapshot::in_language("en");
    let mut live = live(vec![]);
    let picked = sources(&live);
    live.engine.hold_transfers();
    live.rig.bench.os.answers(picked);
    let transfer = begin(&mut live, Message::Files(files::Message::Upload));
    assert!(matches!(
        live.rig.state.files.transfer.as_ref().map(|t| &t.doing),
        Some(Doing::Upload {
            done: 0,
            count: 3,
            ..
        })
    ));
    assert!(live.rig.shows("Uploading to \u{2066}/\u{2069}: 0 of 3"));
    drop(english);
    let theme = crate::ui::theme::Theme::Light;
    for language in ["en", "ar"] {
        let name = format!("container-files-uploading-{language}");
        crate::test_support::snapshot::assert_snapshot(&name, language, &theme, SIZE, || {
            live.rig.view()
        });
    }
    let _english = crate::test_support::snapshot::in_language("en");
    holds(&live, 1);
    // Cancel: the file being put goes in whole, and none after it.
    live.rig.frame(|frame| frame.click("Cancel"));
    assert!(live
        .rig
        .state
        .files
        .transfer
        .as_ref()
        .is_some_and(|t| t.stopping));
    live.engine.release();
    finish(&mut live, transfer);
    assert!(live.rig.state.files.transfer.is_none());
    let names: Vec<Vec<String>> = live.engine.uploaded().into_iter().map(|(_, n)| n).collect();
    assert_eq!(names, [vec!["a.txt".to_owned()]]);
    assert_eq!(live.engine.abandoned(), 0, "no file is cut short");
    // What went in is said, and the folder read again.
    let told = live.rig.bench.took();
    assert!(
        told.contains(&Asked::Said("Uploaded 1 file to /".to_owned())),
        "{told:?}"
    );
    let listing = files_key(&live, "/");
    assert!(told.contains(&Asked::Invalidate(listing)), "{told:?}");
}

#[test]
fn cancel_drops_a_download_and_leaves_no_file() {
    let _english = crate::test_support::snapshot::in_language("en");
    let mut live = live(vec![entry("note.txt", false)]);
    live.engine.file("/note.txt", b"hello\n");
    live.engine.hold_transfers();
    let saved = live.rig.bench.os.dir().join("note.txt");
    live.rig.bench.os.answer(&saved);
    let download = browse(file_browser::Message::Download("/note.txt".to_owned()));
    let transfer = begin(&mut live, download);
    holds(&live, 1);
    // How far it is comes as the bytes do (a test runs the transfer's
    // messages at its end): its name and the bytes so far show at once.
    let saving = format!("Saving {} · 0 B", technical("note.txt"));
    assert!(live.rig.shows(&saving));
    live.rig.frame(|frame| frame.click("Cancel"));
    finish(&mut live, transfer);
    assert!(live
        .rig
        .bench
        .app
        .runtime
        .block_on(live.engine.settles_at(Stream::Transfer, 0)));
    assert_eq!(live.engine.abandoned(), 1);
    assert!(!saved.exists());
    assert!(no_part_left(&live), "nothing unfinished beside it");
    assert!(live.rig.bench.took().is_empty(), "a cancel says nothing");
    assert!(live.rig.state.files.transfer.is_none());
}

/// The unfinished file beside the destination, once it holds `bytes`:
/// what a download writes as the bytes come.
fn part_with(live: &Live, bytes: u64) -> bool {
    for _ in 0..200 {
        let grown = std::fs::read_dir(live.rig.bench.os.dir())
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().ends_with(".part"))
            .any(|entry| entry.metadata().is_ok_and(|found| found.len() >= bytes));
        if grown {
            return true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    false
}

/// A large file, and the page about to download it over a file there.
fn large_over_last_week(live: &mut Live) -> std::path::PathBuf {
    live.engine.file("/note.txt", &[b'x'; 256 * 1024]);
    let saved = live.rig.bench.os.dir().join("note.txt");
    std::fs::write(&saved, b"last week's").unwrap();
    live.rig.bench.os.answer(&saved);
    saved
}

#[test]
fn a_download_that_breaks_after_most_of_the_file_came_leaves_the_file_there() {
    let _english = crate::test_support::snapshot::in_language("en");
    let mut live = live(vec![entry("note.txt", false)]);
    let saved = large_over_last_week(&mut live);
    live.engine.stall(Stall::BeforeLastByte);
    let download = browse(file_browser::Message::Download("/note.txt".to_owned()));
    let transfer = begin(&mut live, download);
    // The bytes are on disk as they come, beside the file, which is
    // untouched (a download held in memory would show nothing here).
    assert!(part_with(&live, 256 * 1024 - 1), "written as it comes");
    assert_eq!(std::fs::read(&saved).unwrap(), b"last week's");
    // The connection drops before the last byte.
    live.engine.cut_downloads();
    live.engine.release();
    finish(&mut live, transfer);
    let told = live.rig.bench.took();
    assert!(
        told.iter()
            .any(|asked| matches!(asked, Asked::Failed(None, _))),
        "{told:?}"
    );
    assert_eq!(std::fs::read(&saved).unwrap(), b"last week's");
    assert!(no_part_left(&live));
}

#[test]
fn a_file_is_in_place_once_its_bytes_came_without_waiting_for_the_archive_s_end() {
    let _english = crate::test_support::snapshot::in_language("en");
    let mut live = live(vec![entry("note.txt", false)]);
    let saved = large_over_last_week(&mut live);
    live.engine.stall(Stall::AfterData);
    let download = browse(file_browser::Message::Download("/note.txt".to_owned()));
    let transfer = begin(&mut live, download);
    // Read out of the archive in one pass: whole while the Engine has
    // not even sent the archive's end.
    let whole = (0..200).any(|_| {
        let done = std::fs::metadata(&saved).is_ok_and(|found| found.len() == 256 * 1024);
        if !done {
            std::thread::sleep(Duration::from_millis(10));
        }
        done
    });
    assert!(whole, "in place before the archive ended");
    live.engine.release();
    finish(&mut live, transfer);
    assert!(no_part_left(&live));
}

#[test]
fn a_download_takes_away_what_a_killed_one_left_a_day_ago() {
    let _english = crate::test_support::snapshot::in_language("en");
    let mut live = live(vec![entry("note.txt", false)]);
    live.engine.file("/note.txt", b"hello\n");
    let dir = live.rig.bench.os.dir().to_owned();
    let week = std::time::SystemTime::now() - Duration::from_secs(7 * 24 * 60 * 60);
    for (name, old) in [
        (".homeport-killedByAnOS.part", true),
        (".homeport-runningNow00.part", false),
        ("notes.part", true),
        (".homeport-notes.part", true),
    ] {
        let path = dir.join(name);
        std::fs::write(&path, "left").unwrap();
        if old {
            let file = std::fs::File::options().write(true).open(&path).unwrap();
            file.set_modified(week).unwrap();
        }
    }
    live.rig.bench.os.answer(dir.join("note.txt"));
    live.send(browse(file_browser::Message::Download(
        "/note.txt".to_owned(),
    )));
    assert!(!dir.join(".homeport-killedByAnOS.part").exists());
    // One of now may be a write that still runs; those not ours stay.
    assert!(dir.join(".homeport-runningNow00.part").exists());
    assert!(dir.join("notes.part").exists());
    assert!(dir.join(".homeport-notes.part").exists());
}

#[test]
fn cancel_at_the_last_byte_leaves_no_file_and_nothing_beside_it() {
    let mut live = live(vec![entry("note.txt", false)]);
    let saved = large_over_last_week(&mut live);
    live.engine.stall(Stall::BeforeLastByte);
    let download = browse(file_browser::Message::Download("/note.txt".to_owned()));
    let transfer = begin(&mut live, download);
    assert!(part_with(&live, 256 * 1024 - 1));
    live.send(Message::Files(files::Message::Cancel));
    finish(&mut live, transfer);
    live.engine.release();
    assert_eq!(std::fs::read(&saved).unwrap(), b"last week's");
    assert!(no_part_left(&live));
    assert!(live.rig.bench.took().is_empty(), "a cancel says nothing");
}

/// Leaves the page while a transfer runs: what holds its Cancel goes.
fn leave(live: &mut Live) {
    let (other, _) = open(&live.rig.bench.ctx());
    drop(std::mem::replace(&mut live.rig.state, other));
}

#[test]
fn leaving_the_page_lets_a_download_end_and_the_shell_say_it() {
    let _english = crate::test_support::snapshot::in_language("en");
    let mut live = live(vec![entry("note.txt", false)]);
    live.engine.file("/note.txt", b"hello\n");
    live.engine.hold_transfers();
    let saved = live.rig.bench.os.dir().join("note.txt");
    live.rig.bench.os.answer(&saved);
    let download = browse(file_browser::Message::Download("/note.txt".to_owned()));
    let transfer = begin(&mut live, download);
    holds(&live, 1);
    leave(&mut live);
    live.engine.release();
    finish(&mut live, transfer);
    assert_eq!(std::fs::read(&saved).unwrap(), b"hello\n");
    let told = live.rig.bench.took();
    let said = Asked::Said("Saved note.txt".to_owned());
    assert!(told.contains(&said), "{told:?}");
}

#[test]
fn leaving_the_page_lets_an_upload_end_and_the_shell_say_it() {
    let _english = crate::test_support::snapshot::in_language("en");
    let mut live = live(vec![]);
    let picked = sources(&live);
    live.engine.hold_transfers();
    live.rig.bench.os.answers(picked);
    let transfer = begin(&mut live, Message::Files(files::Message::Upload));
    holds(&live, 1);
    leave(&mut live);
    live.engine.release();
    finish(&mut live, transfer);
    assert_eq!(live.engine.uploaded().len(), 3, "every file went in");
    let told = live.rig.bench.took();
    assert!(
        told.contains(&Asked::Said("Uploaded 3 files to /".to_owned())),
        "{told:?}"
    );
}

#[test]
fn a_transfer_s_messages_are_heard_while_the_container_is_not_in_the_list() {
    let mut live = live(vec![entry("note.txt", false)]);
    live.engine.file("/note.txt", b"hello\n");
    live.engine.hold_transfers();
    live.rig
        .bench
        .os
        .answer(live.rig.bench.os.dir().join("note.txt"));
    let download = browse(file_browser::Message::Download("/note.txt".to_owned()));
    let transfer = begin(&mut live, download);
    holds(&live, 1);
    // The list is read again without it, for a moment.
    live.rig.list(vec![]);
    live.engine.release();
    finish(&mut live, transfer);
    assert!(live.rig.state.files.transfer.is_none(), "its end was heard");
    live.rig.list(sample());
    assert!(
        !live.rig.state.files.is_busy(),
        "Upload can be pressed again"
    );
}

/// Whether the folder files are saved in holds nothing unfinished.
fn no_part_left(live: &Live) -> bool {
    std::fs::read_dir(live.rig.bench.os.dir())
        .unwrap()
        .filter_map(Result::ok)
        .all(|entry| !entry.file_name().to_string_lossy().ends_with(".part"))
}

fn files_key(live: &Live, path: &str) -> crate::data::Key {
    let host = live.rig.state.host_id.clone();
    crate::ui::features::docker::containers::detail::queries::files_key(&host, WEB, path)
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
            Message::Files(files::Message::Progress(_, files::Step::Files(done))) => Some(*done),
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
    live.rig.state.files.progress(99, files::Step::Files(1));
    assert!(live.rig.state.files.transfer.is_none());
}

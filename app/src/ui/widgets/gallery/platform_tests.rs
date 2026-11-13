//! What the gallery, standing for a page, asks of the system: the save
//! dialog of its log viewer and the file it then writes, and the log
//! folder of the Details dialog. The system is a fake: no dialog of the
//! OS opens, and what is written goes to a folder of the test's.

use std::{fs, path::Path, sync::Arc};

use iced::keyboard::key::Named;

use super::{log, Gallery, Message, Page};
use crate::{
    platform::{
        dialogs::{Filter, SaveFile},
        fake::{Asked, Fake},
    },
    test_support::{snapshot::in_language, stage::Stage, tasks},
    ui::widgets::{
        feedback::{error_details, toast::Kind, Details, Surfaced},
        log as viewer,
    },
};

/// The gallery on its page of logs, on a system that notes what it is asked.
fn gallery() -> (Gallery, Arc<Fake>) {
    let os = Fake::new();
    let gallery = Gallery {
        page: Page::Log,
        platform: os.platform(),
        ..Gallery::default()
    };
    (gallery, os)
}

/// Hands `message` to the gallery, then every message its tasks produce,
/// and gives them all back in the order they were handled.
fn play(gallery: &mut Gallery, message: Message) -> Vec<Message> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let (mut pending, mut handled) = (std::collections::VecDeque::from([message]), vec![]);
    while let Some(message) = pending.pop_front() {
        handled.push(message.clone());
        let task = gallery.update(message);
        pending.extend(runtime.block_on(tasks::messages(task)));
    }
    handled
}

fn download() -> Message {
    Message::Log(viewer::Message::Download)
}

/// The dialog a log viewer's download opens.
fn save_logs() -> Asked {
    Asked::Save(SaveFile::new("Save logs", "logs.txt").filter(Filter::new("Text", &["txt", "log"])))
}

fn toasts(gallery: &Gallery) -> Vec<(Kind, String)> {
    gallery
        .toasts
        .visible()
        .iter()
        .map(|toast| (toast.kind, toast.text().0))
        .collect()
}

fn files_in(folder: &Path) -> usize {
    fs::read_dir(folder).unwrap().count()
}

#[test]
fn the_download_button_opens_the_save_dialog_and_a_cancel_does_nothing() {
    let _english = in_language("en");
    let (mut gallery, os) = gallery();
    // The button is an icon: the keyboard goes to it, Tab after Tab. What
    // the other buttons on the way send is not handed to the gallery.
    let mut stage = Stage::new((1100.0, 700.0));
    let is_download =
        |message: &Message| matches!(message, Message::Log(viewer::Message::Download));
    let pressed = (0..40).find_map(|_| {
        let sent = stage.frame(gallery.view(), |frame| {
            frame.tab();
            frame.tap_key(Named::Enter);
        });
        sent.into_iter().find(is_download)
    });
    play(
        &mut gallery,
        pressed.expect("the toolbar's Download button"),
    );
    assert_eq!(os.took(), [save_logs()]);
    // Closed without a path: no file, no toast, nothing left waiting.
    assert!(toasts(&gallery).is_empty());
    assert!(gallery.log_saving.wrote.is_none());
    assert_eq!(files_in(os.dir()), 0);
}

#[test]
fn the_lines_are_written_where_the_dialog_said_and_a_toast_says_so() {
    let _english = in_language("en");
    let (mut gallery, os) = gallery();
    let path = os.dir().join("logs.txt");
    fs::write(&path, "last week's").unwrap();
    os.answer(&path);
    play(&mut gallery, download());
    let saved = fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = saved.lines().collect();
    assert_eq!(lines.len(), log::sample().len());
    // As the viewer saves them: the source, the timestamp, no colour.
    assert_eq!(
        lines[0],
        "web.1 | 2026-05-01T10:00:00.123456789Z listening on :8080"
    );
    assert!(saved.ends_with('\n') && !saved.contains('\x1b'));
    assert_eq!(toasts(&gallery), [(Kind::Success, "Logs saved".to_owned())]);
    assert!(gallery.log_saving.wrote.is_none());
    assert_eq!(files_in(os.dir()), 1, "nothing is left beside the file");
}

#[test]
fn a_save_that_fails_is_said_and_leaves_the_file_that_was_there() {
    let _english = in_language("en");
    let (mut gallery, os) = gallery();
    // A folder stands where the file should go.
    let path = os.dir().join("logs.txt");
    fs::create_dir(&path).unwrap();
    os.answer(&path);
    play(&mut gallery, download());
    let said = toasts(&gallery);
    let [(Kind::Error, said)] = said.as_slice() else {
        panic!("one failure: {said:?}");
    };
    assert!(said.contains("logs.txt"), "{said}");
    assert!(path.is_dir());
    assert!(gallery.log_saving.wrote.is_none(), "the page stops waiting");
    assert_eq!(files_in(os.dir()), 1);
}

#[test]
fn a_large_save_is_written_off_the_window_s_thread_and_shows_how_far_it_is() {
    let _english = in_language("en");
    const LINES: usize = 300_000;
    let (mut gallery, os) = gallery();
    gallery.log = log::filled(log::CAP, log::many(0, LINES));
    let path = os.dir().join("big.txt");
    os.answer(&path);

    // The dialog answers; `update` only starts the write and returns.
    let asked = gallery.update(download());
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut answered = runtime.block_on(tasks::messages(asked));
    let to = answered.pop().expect("the dialog's answer");
    assert!(matches!(&to, Message::LogSave(log::Save::To(Some(_)))));
    assert!(answered.is_empty());
    let writing = gallery.update(to);
    assert!(!path.exists(), "nothing was written during `update`");
    assert_eq!(
        gallery.log_saving.wrote.map(|wrote| wrote.percent()),
        Some(0)
    );
    let mut stage = Stage::new((1100.0, 700.0));
    stage.frame(gallery.view(), |frame| {
        assert!(frame.find("Saving… 0 %").is_some());
    });

    // The steps come while it is written, a hundred at most, in order.
    let mut steps = vec![];
    for message in runtime.block_on(tasks::messages(writing)) {
        if let Message::LogSave(log::Save::Wrote(wrote)) = &message {
            steps.push(*wrote);
        }
        let _ = gallery.update(message);
    }
    // 0 %, then each percent once.
    assert!(steps.len() > 1 && steps.len() <= 101, "{}", steps.len());
    assert!(steps.windows(2).all(|pair| pair[0].done < pair[1].done));
    assert!(steps.iter().all(|step| step.total == LINES as u64));
    assert_eq!(fs::read_to_string(&path).unwrap().lines().count(), LINES);
    assert_eq!(toasts(&gallery), [(Kind::Success, "Logs saved".to_owned())]);
    assert!(gallery.log_saving.wrote.is_none());
    stage.frame(gallery.view(), |frame| {
        assert!(frame.find("Saving… 100 %").is_none());
    });
}

#[test]
fn the_details_dialog_opens_the_log_folder_of_the_platform() {
    let _english = in_language("en");
    let (mut gallery, os) = gallery();
    let surfaced = Surfaced::at(super::failure(), None, jiff::Timestamp::UNIX_EPOCH);
    gallery.details = Some(Details::new(surfaced));
    let mut stage = Stage::new((1100.0, 900.0));
    let pressed = stage.frame(gallery.view(), |frame| frame.click("Open log folder"));
    assert!(!pressed.is_empty());
    for message in pressed {
        play(&mut gallery, message);
    }
    let logs = os.dir().join("logs");
    assert_eq!(os.took(), [Asked::Opened(logs.clone().into())]);
    assert!(logs.is_dir());

    // The file manager doesn't answer: the dialog says it, in place.
    os.refuse(std::io::ErrorKind::NotFound);
    play(
        &mut gallery,
        Message::Details(error_details::Message::OpenLogFolder),
    );
    let refused = format!("{} can't be opened.", logs.display());
    stage.frame(gallery.view(), |frame| {
        assert!(frame.find(&refused).is_some(), "{refused}");
    });
}

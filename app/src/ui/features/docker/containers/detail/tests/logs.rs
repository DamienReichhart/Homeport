//! The Logs tab: what its controls change, and its lines saved to a file
//! through the system's dialog.

use homeport_core::{
    error::ErrorCode,
    logs::{LogEvent, LogLine},
};

use super::*;
use crate::{
    data::streams::Update,
    platform::{dialogs::SaveFile, fake::Asked as AskedOfOs},
    test_support::snapshot::{assert_snapshot, in_language},
    ui::{
        features::docker::containers::detail::{restarts::StartRead, tabs::logs},
        shell::page::testing::Asked,
        theme::Theme,
        widgets::{feedback::error_message::Event, form::select, log},
    },
};

fn line(stream: &str, text: &str) -> LogLine {
    LogLine {
        source: None,
        stream: stream.to_owned(),
        text: text.to_owned(),
    }
}

/// What the page's stream sends it: these lines, in one batch.
fn batch(rig: &Rig, lines: Vec<LogLine>) -> Message {
    let update = Update::Batch {
        items: vec![LogEvent::Lines { lines }],
        dropped: 0,
    };
    Message::Logs(logs::Message::Lines(rig.state.logs.generation(), update))
}

/// A web server's start, a request and a failure, as a container writes
/// them.
fn written() -> Vec<LogLine> {
    vec![
        line("stdout", "listening on \x1b[1m:8080\x1b[0m"),
        line(
            "stdout",
            "\x1b[32mINFO\x1b[0m  GET /api/v1/containers \x1b[36m200\x1b[0m in 12 ms",
        ),
        line(
            "stderr",
            "\x1b[1;31mERROR\x1b[0m failed to reach db:5432: connection refused",
        ),
        line("stdout", "\x1b[33mWARN\x1b[0m  retrying in 5 s"),
    ]
}

fn filled() -> Rig {
    let mut rig = Rig::at(WEB, "logs");
    let lines = batch(&rig, written());
    rig.send(lines);
    rig
}

fn logs(message: logs::Message) -> Message {
    Message::Logs(message)
}

#[test]
fn the_viewer_starts_with_react_s_options_and_shows_what_the_stream_sends() {
    let rig = filled();
    let asked = rig.state.logs.options();
    assert_eq!(
        (asked.tail, asked.since, asked.timestamps, asked.follow),
        (Some(1000), None, false, true)
    );
    assert!(asked.stdout && asked.stderr);
    assert_eq!(rig.state.logs.view.len(), 4);
    assert_eq!(logs::KEPT_LINES, 100_000);
    assert_eq!(logs::tail_labels().len(), logs::TAILS.len());
    assert_eq!(logs::since_labels().len(), logs::SINCE.len());
}

#[test]
fn the_history_and_how_far_back_are_asked_of_the_engine() {
    let mut rig = filled();
    rig.send(logs(logs::Message::Tail(select::Message::Choose(3))));
    assert_eq!(rig.state.logs.options().tail, None, "everything");
    assert_eq!(rig.state.logs.view.len(), 0);
    let before = jiff::Timestamp::now().as_second();
    rig.send(logs(logs::Message::Since(select::Message::Choose(2))));
    let since = i64::from(rig.state.logs.options().since.expect("an hour ago"));
    assert!((before - 3600..=before - 3590).contains(&since), "{since}");
    // The moment is the stream's: it is not worked out again at each
    // look, which would make another stream of it every second.
    let again = rig.state.logs.options().since;
    assert_eq!(again.map(i64::from), Some(since));
    rig.send(logs(logs::Message::Since(select::Message::Choose(0))));
    assert_eq!(rig.state.logs.options().since, None);
}

#[test]
fn a_stream_that_fails_is_said_over_the_lines_and_tried_again() {
    let _english = in_language("en");
    let mut rig = filled();
    let refused = homeport_core::error::AppError::new(
        ErrorCode::HostNotConnected,
        homeport_hosts::messages::NOT_CONNECTED_UNKNOWN,
    );
    let generation = rig.state.logs.generation();
    let failed = logs::Message::Lines(generation, Update::Failed(refused.clone()));
    rig.send(logs(failed));
    assert!(rig.shows("This host isn't connected."));
    rig.send(logs(logs::Message::Failure(Event::Details)));
    assert_eq!(rig.bench.took(), [Asked::Details(refused)]);
    // Retry is another stream, on an empty viewer.
    rig.send(logs(logs::Message::Failure(Event::Retry)));
    assert_eq!(rig.state.logs.generation(), generation + 1);
    assert!(rig.state.logs.failure.is_none());
}

#[test]
fn the_selection_is_copied_through_the_shell() {
    let mut rig = filled();
    rig.send(logs(logs::Message::Viewer(log::Message::SelectAll)));
    rig.send(logs(logs::Message::Viewer(log::Message::Copy)));
    let asked = rig.bench.took();
    let [Asked::Copy(text)] = asked.as_slice() else {
        panic!("{asked:?}");
    };
    assert!(text.contains("listening on :8080") && text.contains("retrying in 5 s"));
}

fn download(rig: &mut Rig) {
    rig.send(logs(logs::Message::Viewer(log::Message::Download)));
}

#[test]
fn the_lines_are_saved_where_the_dialog_says_and_the_shell_says_so() {
    let _english = in_language("en");
    let mut rig = filled();
    // Cancelled: nothing is written, nothing is said.
    download(&mut rig);
    assert_eq!(
        rig.bench.os.took(),
        [AskedOfOs::Save(SaveFile::new("Save logs", "logs.txt"))]
    );
    assert!(rig.bench.took().is_empty());

    let path = rig.bench.os.dir().join("logs.txt");
    rig.bench.os.answer(&path);
    download(&mut rig);
    let saved = std::fs::read_to_string(&path).expect("the file");
    // Without the colours' sequences, a line a line.
    assert_eq!(saved.lines().count(), 4);
    assert!(saved.starts_with("listening on :8080\n"), "{saved}");
    assert_eq!(rig.bench.took(), [Asked::Said("Logs saved".to_owned())]);
    assert!(rig.state.logs.saving().is_none(), "the save is over");

    // A folder that isn't there: the shell says why, and nothing is left.
    rig.bench
        .os
        .answer(rig.bench.os.dir().join("missing/logs.txt"));
    download(&mut rig);
    let said = rig.bench.took();
    assert!(
        matches!(said.as_slice(), [Asked::Failed(None, _)]),
        "{said:?}"
    );
    assert!(rig.state.logs.saving().is_none());

    // A desktop that can show no dialog: said, not taken for a cancel.
    rig.bench.os.without_dialogs();
    download(&mut rig);
    let said = rig.bench.took();
    assert!(
        matches!(said.as_slice(), [Asked::Failed(None, error)] if error.key == "errors.file.dialogUnavailable"),
        "{said:?}"
    );
}

#[test]
fn the_logs_tab_drawn() {
    let rig = filled();
    assert_snapshot("container-logs-en", "en", &Theme::Light, SIZE, || {
        rig.view()
    });
    assert_snapshot("container-logs-ar", "ar", &Theme::Light, SIZE, || {
        rig.view()
    });
    assert_snapshot("container-logs-dark-en", "en", &Theme::Dark, SIZE, || {
        rig.view()
    });
}

#[test]
fn a_stopped_container_s_logs_and_a_stream_that_failed_drawn() {
    // A stopped container still has its logs, and their end.
    let mut rig = Rig::at("shop-db-1-0123456789abcdef", "logs");
    let lines = batch(&rig, written());
    rig.send(lines);
    let generation = rig.state.logs.generation();
    let ended = Update::Batch {
        items: vec![LogEvent::End],
        dropped: 0,
    };
    rig.send(logs(logs::Message::Lines(generation, ended)));
    assert!(rig.state.logs.is_over());
    assert_snapshot(
        "container-logs-stopped-en",
        "en",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
    // No Engine answers a test: the stream can't start.
    let mut rig = Rig::at(WEB, "logs");
    let refused = homeport_core::error::AppError::new(
        ErrorCode::HostNotConnected,
        homeport_hosts::messages::NOT_CONNECTED_UNKNOWN,
    );
    let generation = rig.state.logs.generation();
    rig.send(logs(logs::Message::Lines(
        generation,
        Update::Failed(refused),
    )));
    assert_snapshot("container-logs-error-en", "en", &Theme::Light, SIZE, || {
        rig.view()
    });
    let rig = Rig::at(WEB, "logs");
    assert_snapshot("container-logs-empty-en", "en", &Theme::Light, SIZE, || {
        rig.view()
    });
}

fn end(rig: &mut Rig) {
    let generation = rig.state.logs.generation();
    let ended = Update::Batch {
        items: vec![LogEvent::End],
        dropped: 0,
    };
    rig.send(logs(logs::Message::Lines(generation, ended)));
}

/// When the container started, as its inspect says.
const STARTED: &str = "2026-05-01T10:00:00.000000001Z";
/// When it started again.
const RESTARTED: &str = "2026-05-01T13:00:00.5Z";

/// The container's start, read for the stream that shows now.
fn started(rig: &mut Rig, at: &str) {
    let asked = rig.state.restarts.last_asked();
    rig.send(Message::Started(asked, Some(at.to_owned())));
}

/// The sample, `web` up for `status`.
fn up(status: &str) -> Vec<homeport_engine::containers::Container> {
    let mut list = sample();
    status.clone_into(&mut list[0].status);
    list
}

#[test]
fn a_restart_seen_before_the_end_opens_the_logs_again() {
    let mut rig = filled();
    let generation = rig.state.logs.generation();
    started(&mut rig, STARTED);
    // Restart: the list says it is up anew before the stream's end comes.
    rig.list(up("Up Less than a second (healthy)"));
    assert_eq!(rig.state.logs.generation(), generation);
    end(&mut rig);
    assert!(rig.state.logs.has_ended());
    // The start read at the end is another one: another stream, on an
    // empty viewer.
    started(&mut rig, RESTARTED);
    assert_eq!(rig.state.logs.generation(), generation + 1);
    assert!(!rig.state.logs.has_ended() && rig.state.logs.view.is_empty());
    // Read for the new stream, the new start opens nothing more.
    started(&mut rig, RESTARTED);
    rig.list(up("Up 1 second (healthy)"));
    started(&mut rig, RESTARTED);
    assert_eq!(rig.state.logs.generation(), generation + 1);
}

#[test]
fn a_running_container_whose_status_changes_is_never_opened_again() {
    let mut rig = filled();
    let generation = rig.state.logs.generation();
    started(&mut rig, STARTED);
    end(&mut rig);
    started(&mut rig, STARTED);
    let _ = rig.bench.took();
    // Its status changes as the list is read again; its start does not.
    // (Each read waits for the last to be two seconds old.)
    for status in ["Up 4 hours (healthy)", "Up 4 hours (unhealthy)"] {
        rig.list(up(status));
        started(&mut rig, STARTED);
        assert_eq!(rig.state.logs.generation(), generation, "{status}");
        assert!(rig.state.logs.has_ended());
    }
    assert!(rig.bench.took().is_empty(), "the list is not asked for");
}

#[test]
fn a_start_that_could_not_be_read_opens_nothing() {
    let mut rig = filled();
    let generation = rig.state.logs.generation();
    started(&mut rig, STARTED);
    end(&mut rig);
    let asked = rig.state.restarts.last_asked();
    rig.send(Message::Started(asked, None));
    rig.list(up("Up 4 hours (healthy)"));
    let asked = rig.state.restarts.last_asked();
    rig.send(Message::Started(asked, None));
    assert_eq!(rig.state.logs.generation(), generation);
    // An answer for a stream before this one is not this one's.
    let before = StartRead {
        generation: generation - 1,
        ..asked
    };
    rig.send(Message::Started(before, Some(RESTARTED.to_owned())));
    assert_eq!(rig.state.logs.generation(), generation);
}

#[test]
fn without_the_start_read_as_it_opened_a_start_later_than_the_opening_is_a_restart() {
    // The read as the stream opened failed (no Engine answers here).
    let mut rig = filled();
    let generation = rig.state.logs.generation();
    end(&mut rig);
    // Started before the stream opened: the same run, followed.
    started(&mut rig, STARTED);
    assert_eq!(rig.state.logs.generation(), generation);
    // Once that is known, a start after it is a restart, as ever.
    rig.list(up("Up 1 second (healthy)"));
    started(&mut rig, RESTARTED);
    assert_eq!(rig.state.logs.generation(), generation + 1);

    // Again with nothing read before the end: a start later than the
    // moment the stream opened is a restart between the two.
    let mut rig = filled();
    let generation = rig.state.logs.generation();
    end(&mut rig);
    let now = jiff::Timestamp::now().to_string();
    started(&mut rig, &now);
    assert_eq!(rig.state.logs.generation(), generation + 1);
}

#[test]
fn a_stream_that_failed_on_a_running_container_is_never_opened_again_by_itself() {
    let mut rig = filled();
    let generation = rig.state.logs.generation();
    started(&mut rig, STARTED);
    // A log driver that serves no logs: the Engine's error, then the end.
    let refused = homeport_core::error::AppError::new(
        ErrorCode::EngineError,
        homeport_hosts::messages::NOT_CONNECTED_UNKNOWN,
    );
    let failed = Update::Batch {
        items: vec![LogEvent::Error { error: refused }, LogEvent::End],
        dropped: 0,
    };
    rig.send(logs(logs::Message::Lines(generation, failed)));
    let asked = rig.bench.took();
    assert!(
        !asked.contains(&Asked::Invalidate(keys::host::containers("h1"))),
        "{asked:?}"
    );
    // Even a start that is another one once it failed.
    for list in [sample(), up("Up 1 second"), up("Up 2 seconds")] {
        rig.list(list);
        started(&mut rig, RESTARTED);
        assert_eq!(rig.state.logs.generation(), generation);
        assert!(rig.state.logs.failure.is_some(), "the failure stays");
    }
    // Its Retry is the user's.
    rig.send(logs(logs::Message::Failure(Event::Retry)));
    assert_eq!(rig.state.logs.generation(), generation + 1);
}

#[test]
fn a_container_that_stopped_keeps_its_end_of_logs() {
    let mut rig = filled();
    let generation = rig.state.logs.generation();
    end(&mut rig);
    let mut stopped = sample();
    "exited".clone_into(&mut stopped[0].state);
    rig.list(stopped);
    assert_eq!(rig.state.logs.generation(), generation);
    assert!(rig.state.logs.has_ended());
}

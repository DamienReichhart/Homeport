//! Export against an Engine: the save dialog, the file, its progress,
//! Cancel, and what the shell says.

use std::time::Duration;

use homeport_core::error::ErrorCode;

use super::super::{Actions, Message as Action, Target};
use super::{Exported, Message};
use crate::{
    platform::{dialogs::Filter, fake::Asked as AskedOfOs},
    test_support::{
        engine::{Engine, Stream, EXPORTED},
        fixtures::container,
        snapshot::{assert_snapshot, in_language},
        tasks::messages,
    },
    ui::{
        shell::{
            page::testing::{Asked, Bench},
            router::Route,
        },
        theme::Theme,
        widgets::overlay,
    },
};

/// Actions of a page whose Host is the fake Engine, and `web` on it.
fn bench() -> (Bench, Engine, Target) {
    let bench = Bench::at(Route::Containers);
    let (engine, host) = bench.app.runtime.block_on(async {
        let engine = Engine::start();
        let host = engine.host(&bench.app.state).await;
        (engine, host)
    });
    let web = container("web", None, "running", 0);
    (bench, engine, Target::of(&host, "fake", &web))
}

/// Sends `message` and what it started, to their end.
fn send(actions: &mut Actions, bench: &Bench, message: Action) {
    let runtime = bench.app.runtime.handle().clone();
    let _inside = runtime.enter();
    let task = actions.update(message, &bench.ctx());
    for answer in bench.run(task) {
        send(actions, bench, answer);
    }
}

#[test]
fn the_save_dialog_asks_for_a_tar_named_after_the_container_and_a_cancel_does_nothing() {
    let _english = in_language("en");
    let (mut bench, _engine, web) = bench();
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskExport(web));
    let asked = bench.os.took();
    let [AskedOfOs::Save(request)] = asked.as_slice() else {
        panic!("{asked:?}");
    };
    assert_eq!(request.name, "web.tar");
    assert_eq!(request.title, "Export the filesystem of web");
    assert_eq!(request.filters, [Filter::new("Tar archive", &["tar"])]);
    assert!(!actions.has_dialog() && bench.took().is_empty());
}

#[test]
fn the_filesystem_is_saved_and_the_shell_says_where() {
    let _english = in_language("en");
    let (mut bench, engine, web) = bench();
    let mut actions = Actions::of_host();
    let path = bench.os.dir().join("web.tar");
    bench.os.answer(&path);
    send(&mut actions, &bench, Action::AskExport(web));
    assert_eq!(std::fs::metadata(&path).unwrap().len(), EXPORTED as u64);
    let shown = actions.exports.shown.as_ref().expect("the dialog stays");
    assert!(shown.done, "{shown:?}");
    assert!(shown.written.bytes >= 2 * 1024 * 1024, "{shown:?}");
    let said = format!("Exported to \u{2066}{}\u{2069}", path.display());
    assert_eq!(bench.took(), [Asked::Said(said)]);
    assert_eq!(engine.asked("/export").len(), 1);
    send(&mut actions, &bench, Action::Export(Message::Close));
    assert!(!actions.has_dialog());
}

#[test]
fn cancel_stops_the_export_and_leaves_no_file() {
    let (mut bench, engine, web) = bench();
    let mut actions = Actions::of_host();
    let path = bench.os.dir().join("web.tar");
    bench.os.answer(&path);
    engine.hold_transfers();
    let runtime = bench.app.runtime.handle().clone();
    let _inside = runtime.enter();
    let asked = actions.update(Action::AskExport(web), &bench.ctx());
    let answer = bench.run(asked);
    let ctx = bench.ctx();
    let running: Vec<_> = answer
        .into_iter()
        .map(|message| actions.update(message, &ctx))
        .collect();
    let export = runtime.spawn(messages(iced::Task::batch(running)));
    assert!(bench
        .app
        .runtime
        .block_on(engine.settles_at(Stream::Transfer, 1)));
    // The dialog is up, with Cancel; drawn while it waits, at a path
    // that is the same wherever the test runs.
    assert!(actions.has_dialog());
    if let Some(shown) = actions.exports.shown.as_mut() {
        shown.path = std::path::PathBuf::from("/home/me/web.tar");
    }
    let theme = Theme::Light;
    for language in ["en", "ar"] {
        let name = format!("container-export-{language}");
        assert_snapshot(&name, language, &theme, (828.0, 420.0), || {
            overlay::over(iced::widget::Space::new(), actions.dialog())
        });
    }
    send(&mut actions, &bench, Action::Export(Message::Close));
    assert!(!actions.has_dialog(), "Cancel closes at once");
    let ended = bench
        .app
        .runtime
        .block_on(async { tokio::time::timeout(Duration::from_secs(5), export).await })
        .expect("the export ends")
        .expect("it ran");
    assert!(ended.iter().any(|message| matches!(
        message,
        Action::Export(Message::Ended(_, Ok(Exported::Stopped)))
    )));
    assert!(bench
        .app
        .runtime
        .block_on(engine.settles_at(Stream::Transfer, 0)));
    assert!(!path.exists(), "no file is left");
    assert!(bench.took().is_empty(), "a cancel says nothing");
}

#[test]
fn a_host_that_is_down_and_a_desktop_with_no_dialog_are_said() {
    let (mut bench, _engine, mut web) = bench();
    let mut actions = Actions::of_host();
    "no-such-host".clone_into(&mut web.host_id);
    bench.os.answer(bench.os.dir().join("web.tar"));
    send(&mut actions, &bench, Action::AskExport(web.clone()));
    let said = bench.took();
    assert!(
        matches!(said.as_slice(), [Asked::Failed(None, error)] if error.code == ErrorCode::HostNotConnected),
        "{said:?}"
    );
    // The dialog says it failed, and closes.
    assert!(actions
        .exports
        .shown
        .as_ref()
        .is_some_and(|e| e.failure.is_some()));
    send(&mut actions, &bench, Action::Export(Message::Close));
    bench.os.without_dialogs();
    send(&mut actions, &bench, Action::AskExport(web));
    let said = bench.took();
    assert!(
        matches!(said.as_slice(), [Asked::Failed(None, error)] if error.key == "errors.file.dialogUnavailable"),
        "{said:?}"
    );
    assert!(!actions.has_dialog());
}

/// Starts an export that waits on the Engine, on the side.
fn begin(
    actions: &mut Actions,
    bench: &Bench,
    web: Target,
) -> tokio::task::JoinHandle<Vec<Action>> {
    let runtime = bench.app.runtime.handle().clone();
    let _inside = runtime.enter();
    let asked = actions.update(Action::AskExport(web), &bench.ctx());
    let answer = bench.run(asked);
    let ctx = bench.ctx();
    let running: Vec<_> = answer
        .into_iter()
        .map(|message| actions.update(message, &ctx))
        .collect();
    runtime.spawn(messages(iced::Task::batch(running)))
}

#[test]
fn leaving_the_page_lets_the_export_end_and_the_shell_say_it() {
    let _english = in_language("en");
    let (mut bench, engine, web) = bench();
    let mut actions = Actions::of_host();
    let path = bench.os.dir().join("web.tar");
    bench.os.answer(&path);
    engine.hold_transfers();
    let export = begin(&mut actions, &bench, web);
    assert!(bench
        .app
        .runtime
        .block_on(engine.settles_at(Stream::Transfer, 1)));
    // The page goes, and what it held with it.
    drop(actions);
    engine.release();
    bench
        .app
        .runtime
        .block_on(async { tokio::time::timeout(Duration::from_secs(5), export).await })
        .expect("the export ends")
        .expect("it ran");
    assert_eq!(std::fs::metadata(&path).unwrap().len(), EXPORTED as u64);
    let said = format!("Exported to \u{2066}{}\u{2069}", path.display());
    assert!(bench.took().contains(&Asked::Said(said)));
}

#[test]
fn escape_and_the_scrim_never_stop_an_export_that_runs() {
    use iced::keyboard::key::Named;

    let (bench, engine, web) = bench();
    let mut actions = Actions::of_host();
    bench.os.answer(bench.os.dir().join("web.tar"));
    engine.hold_transfers();
    let export = begin(&mut actions, &bench, web);
    let mut stage = crate::test_support::stage::Stage::new((828.0, 420.0));
    let sent = stage.frame(
        overlay::over(iced::widget::Space::new(), actions.dialog()),
        |frame| {
            frame.tap_key(Named::Escape);
            frame.point_at((10.0, 10.0));
            frame.press();
        },
    );
    assert!(sent.is_empty(), "{sent:?}");
    assert!(actions.has_dialog());
    send(&mut actions, &bench, Action::Export(Message::Close));
    engine.release();
    let _ = bench
        .app
        .runtime
        .block_on(async { tokio::time::timeout(Duration::from_secs(5), export).await });
}

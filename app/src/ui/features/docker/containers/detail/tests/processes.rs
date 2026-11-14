//! The Processes tab: what it reads, when, and how it looks.

use homeport_engine::containers::Processes;

use super::*;
use crate::{
    test_support::{
        following::Following,
        snapshot::{assert_snapshot, in_language},
    },
    ui::{
        features::docker::containers::detail::{queries, subscription, tabs::processes},
        shell::page::testing::Asked,
        theme::Theme,
        widgets::{basic::technical, feedback::error_message::Event},
    },
};

fn key() -> crate::data::Key {
    queries::processes_key("h1", WEB)
}

fn top() -> Processes {
    let row = |cells: &[&str]| cells.iter().map(|cell| (*cell).to_owned()).collect();
    Processes {
        titles: ["UID", "PID", "PPID", "C", "STIME", "TTY", "TIME", "CMD"]
            .map(str::to_owned)
            .to_vec(),
        processes: vec![
            row(&[
                "root",
                "1",
                "0",
                "0",
                "10:00",
                "?",
                "00:00:01",
                "nginx: master process nginx -g daemon off;",
            ]),
            row(&[
                "101",
                "29",
                "1",
                "0",
                "10:00",
                "?",
                "00:00:00",
                "nginx: worker process",
            ]),
        ],
    }
}

fn listed() -> Rig {
    let mut rig = Rig::at(WEB, "processes");
    rig.bench.data.set(&key(), top());
    rig.refresh();
    rig
}

#[test]
fn the_processes_show_under_the_titles_the_engine_sends() {
    let _english = in_language("en");
    let rig = listed();
    assert!(rig.shows(&technical("CMD")) && rig.shows(&technical("PID")));
    assert!(rig.shows(&technical("nginx: worker process")));
    assert_eq!(rig.state.processes.rows.len(), 2);
}

#[test]
fn a_running_container_s_processes_are_read_when_the_tab_shows_and_every_five_seconds() {
    let mut bench = Bench::on_host(Route::Containers, connected_host());
    bench.data.set(&keys::host::containers("h1"), sample());
    bench.history.push(route(WEB, Some("logs")).into());
    let mut rig = Rig::open_on(bench);
    assert!(!rig.bench.watched().contains(&key()));
    rig.send(Message::Tab(Tab::Processes));
    assert_eq!(rig.bench.watched(), [key()]);
    rig.send(Message::Processes(processes::Message::Poll));
    assert_eq!(rig.bench.took(), [Asked::Invalidate(key())]);
    assert_eq!(processes::EVERY, std::time::Duration::from_secs(5));
    // The timer runs while the tab shows a running container only.
    let mut following = Following::default();
    let _runtime = rig.bench.app.runtime.enter();
    let mut count = |rig: &Rig| {
        following.sync(subscription(&rig.state, &rig.bench.ctx()));
        following.count()
    };
    // The window's size, and the timer.
    assert_eq!(count(&rig), 2);
    rig.send(Message::Tab(Tab::Inspect));
    assert_eq!(count(&rig), 1);
}

#[test]
fn a_stopped_container_s_processes_are_never_asked_for() {
    let rig = Rig::at("shop-db-1-0123456789abcdef", "processes");
    assert!(rig.bench.watched().is_empty());
    {
        let _english = in_language("en");
        assert!(rig.shows("The container isn't running."));
    }
    assert_snapshot(
        "container-processes-stopped-en",
        "en",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
}

#[test]
fn a_listing_that_failed_is_said_and_read_again() {
    let english = in_language("en");
    let mut bench = Bench::on_host(Route::Containers, connected_host());
    bench.data.set(&keys::host::containers("h1"), sample());
    bench.history.push(route(WEB, Some("processes")).into());
    let (state, _) = open(&bench.ctx());
    // No Engine answers a test.
    bench.read_watched();
    let mut rig = Rig {
        bench,
        state,
        stage: crate::test_support::stage::Stage::new(SIZE),
    };
    assert!(rig.shows("Retry"));
    rig.send(Message::Processes(processes::Message::Failure(
        Event::Retry,
    )));
    assert_eq!(rig.bench.took(), [Asked::Invalidate(key())]);
    drop(english);
    assert_snapshot(
        "container-processes-error-en",
        "en",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
}

#[test]
fn the_processes_tab_drawn() {
    let rig = listed();
    assert_snapshot("container-processes-en", "en", &Theme::Light, SIZE, || {
        rig.view()
    });
    assert_snapshot("container-processes-ar", "ar", &Theme::Light, SIZE, || {
        rig.view()
    });
    assert_snapshot(
        "container-processes-dark-en",
        "en",
        &Theme::Dark,
        SIZE,
        || rig.view(),
    );
}

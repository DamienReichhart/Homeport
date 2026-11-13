//! The Inspect tab: what it reads, its sections, the raw document, and
//! how it looks.

use homeport_core::error::ErrorCode;

use super::*;
use crate::{
    t,
    test_support::snapshot::{assert_snapshot, in_language},
    ui::{
        features::docker::containers::detail::{
            model::{inspect::Inspected, tests::document},
            queries,
            tabs::inspect,
        },
        shell::page::testing::Asked,
        theme::Theme,
        widgets::{basic::technical, data::json, feedback::error_message::Event, table},
    },
};

fn key(state: &str) -> crate::data::Key {
    queries::inspect_key("h1", WEB, state)
}

/// The page on its Inspect tab, the document read.
fn read() -> Rig {
    let mut rig = Rig::at(WEB, "inspect");
    rig.bench
        .data
        .set(&key("running"), Inspected::of(&document()));
    rig.refresh();
    rig
}

#[test]
fn the_document_is_read_when_its_tab_shows_and_once() {
    let mut bench = Bench::on_host(Route::Containers, connected_host());
    bench.data.set(&keys::host::containers("h1"), sample());
    bench.history.push(route(WEB, Some("logs")).into());
    let (mut state, _) = open(&bench.ctx());
    assert!(!bench.watched().contains(&key("running")));
    let _ = update(&mut state, Message::Tab(Tab::Inspect), &bench.ctx());
    assert_eq!(bench.watched(), [key("running")]);
    // Looking again asks for nothing more.
    let _ = refresh(&mut state, &bench.ctx());
    let _ = update(&mut state, Message::Tab(Tab::Logs), &bench.ctx());
    let _ = update(&mut state, Message::Tab(Tab::Inspect), &bench.ctx());
    assert!(bench.watched().is_empty());
}

#[test]
fn the_document_is_read_again_when_the_container_s_state_changes() {
    let mut rig = read();
    assert!(rig.state.inspect.json.is_some());
    let _ = rig.bench.took();
    let mut stopped = sample();
    "exited".clone_into(&mut stopped[0].state);
    rig.list(stopped);
    assert_eq!(rig.bench.watched(), [key("exited")]);
    // Until it is read, nothing of the document of before shows.
    let _english = in_language("en");
    assert!(!rig.shows(&technical("nginx:1.27")));
}

#[test]
fn the_sections_show_what_the_document_says() {
    let _english = in_language("en");
    let rig = read();
    for said in [
        "nginx:1.27",
        "web-0123456789abcdef",
        "MODE",
        "production",
        "0.0.0.0:8080",
        "*:8081",
        "web-data",
        "org.example.team",
        "172.17.0.2",
        "nginx -g daemon off;",
    ] {
        assert!(rig.shows_in(TALL, &technical(said)), "{said}");
    }
    let policy = t!(
        "containers.inspect.restartedMax",
        policy = technical("on-failure"),
        max = 5,
        count = 2
    );
    assert!(rig.shows(&policy), "{policy}");
    for said in [
        "image default",
        "not published",
        "read-only",
        "3 failing in a row",
        "Variable",
        "Published on",
    ] {
        assert!(rig.shows_in(TALL, said), "{said}");
    }
}

#[test]
fn the_raw_document_is_a_tab_away_and_its_copy_goes_through_the_shell() {
    let _english = in_language("en");
    let mut rig = read();
    rig.frame(|frame| frame.click("Raw JSON"));
    assert!(rig.state.inspect.raw);
    let json = rig.state.inspect.json.as_ref().expect("the tree");
    assert!(json.rows() > 10);
    assert!(!rig.shows("Variable"), "the sections are another view");
    rig.send(Message::Inspect(inspect::Message::Json(
        json::Message::Copy(json::Copy::Document),
    )));
    let asked = rig.bench.took();
    let [Asked::Copy(text)] = asked.as_slice() else {
        panic!("{asked:?}");
    };
    assert!(text.contains("\"NetworkMode\": \"bridge\""), "{text}");
    rig.frame(|frame| frame.click("Details"));
    assert!(!rig.state.inspect.raw && rig.shows("Variable"));
}

#[test]
fn a_table_of_a_section_keeps_what_was_done_to_it() {
    let mut rig = read();
    let resize = table::Message::Resize {
        column: "name",
        width: 222.0,
    };
    rig.send(Message::Inspect(inspect::Message::Table(
        inspect::Section::Environment,
        resize,
    )));
    assert_eq!(
        rig.state.inspect.tables.environment.widths(),
        [("name", 222.0)]
    );
    assert!(rig.state.inspect.tables.labels.widths().is_empty());
}

#[test]
fn a_document_that_could_not_be_read_is_said_and_read_again() {
    let english = in_language("en");
    let mut bench = Bench::on_host(Route::Containers, connected_host());
    bench.data.set(&keys::host::containers("h1"), sample());
    bench.history.push(route(WEB, Some("inspect")).into());
    let mut rig = Rig::open_on(bench);
    // Nothing was read yet: the tab waits.
    assert!(!rig.shows("Details") && !rig.shows("Raw JSON"));
    let _ = update(&mut rig.state, Message::Tab(Tab::Logs), &rig.bench.ctx());
    rig.state.watched.clear();
    let _ = update(&mut rig.state, Message::Tab(Tab::Inspect), &rig.bench.ctx());
    // No Engine answers a test.
    rig.bench.read_watched();
    assert!(rig.shows("Retry"));
    rig.send(Message::Inspect(inspect::Message::Failure(Event::Retry)));
    assert_eq!(rig.bench.took(), [Asked::Invalidate(key("running"))]);
    rig.send(Message::Inspect(inspect::Message::Failure(Event::Details)));
    let asked = rig.bench.took();
    assert!(
        matches!(asked.as_slice(), [Asked::Details(error)] if error.code == ErrorCode::HostNotConnected),
        "{asked:?}"
    );
    // The snapshot sets the language itself.
    drop(english);
    assert_snapshot(
        "container-inspect-error-en",
        "en",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
}

/// The whole of the sections, which are taller than the window.
const TALL: (f32, f32) = (828.0, 1500.0);

#[test]
fn the_inspect_tab_drawn() {
    let rig = read();
    assert_snapshot("container-inspect-en", "en", &Theme::Light, TALL, || {
        rig.view()
    });
    assert_snapshot("container-inspect-ar", "ar", &Theme::Light, TALL, || {
        rig.view()
    });
    assert_snapshot(
        "container-inspect-dark-en",
        "en",
        &Theme::Dark,
        TALL,
        || rig.view(),
    );
}

#[test]
fn the_raw_document_and_the_wait_for_it_drawn() {
    let mut rig = read();
    rig.send(Message::Inspect(inspect::Message::Show(1)));
    assert_snapshot(
        "container-inspect-raw-en",
        "en",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
    assert_snapshot(
        "container-inspect-raw-ar",
        "ar",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
    let rig = Rig::at(WEB, "inspect");
    assert_snapshot(
        "container-inspect-loading-en",
        "en",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
}

#[test]
fn a_container_with_nothing_to_list_and_a_narrow_window_drawn() {
    // A stopped container that never started, with no healthcheck, no
    // variable, no mount, no port, no network and no label.
    let mut rig = Rig::at("shop-db-1-0123456789abcdef", "inspect");
    let bare = serde_json::json!({
        "Id": "shop-db-1-0123456789abcdef",
        "Created": "2026-01-02T03:04:05Z",
        "State": {
            "Status": "exited",
            "Running": false,
            "ExitCode": 137,
            "OOMKilled": true,
            "StartedAt": "0001-01-01T00:00:00Z",
        },
        "Config": { "Image": "postgres:17", "Entrypoint": ["docker-entrypoint.sh"] },
        "HostConfig": { "NetworkMode": "none" },
    });
    let key = queries::inspect_key("h1", "shop-db-1-0123456789abcdef", "exited");
    rig.bench.data.set(&key, Inspected::of(&bare));
    rig.refresh();
    {
        let _english = in_language("en");
        assert!(rig.shows("The image defines no health check."));
        assert!(rig.shows_in((828.0, 1500.0), &t!("common.none")));
    }
    rig.send(Message::Resized(iced::Size::new(900.0, 700.0)));
    let narrow = (628.0, 1300.0);
    assert_snapshot(
        "container-inspect-bare-en",
        "en",
        &Theme::Light,
        narrow,
        || rig.view(),
    );
}

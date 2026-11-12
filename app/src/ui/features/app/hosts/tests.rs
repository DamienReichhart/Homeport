//! The Hosts page: what it reads, what a press and a header do, and how
//! it looks in each of its states.

use homeport_core::error::{AppError, ErrorCode};
use homeport_hosts::registry::HostStatus;

use super::*;
use crate::{
    t,
    test_support::{
        fixtures::{connected, host_view},
        snapshot::{assert_snapshot, in_language},
        window::{drive, window},
    },
    ui::{
        shell::{page::testing::Bench, router::Route},
        theme::Theme,
    },
};

fn bench(list: Vec<HostView>) -> Bench {
    let mut bench = Bench::at(Route::Hosts);
    bench.data.set(&keys::host::list(), list);
    bench
}

fn three() -> Vec<HostView> {
    vec![
        host_view("prod", connected()),
        host_view("staging", HostStatus::Connecting),
        host_view("backup", HostStatus::Disabled),
    ]
}

#[test]
fn opening_reads_the_hosts_through_the_data_layer() {
    let bench = Bench::at(Route::Hosts);
    let _ = open(&bench.ctx());
    assert_eq!(bench.watched(), [keys::host::list()]);
}

#[test]
fn a_failure_leads_to_its_details_and_retry_reads_again() {
    let mut bench = bench(three());
    let locked = AppError::new(ErrorCode::DbLocked, homeport_storage::messages::DB_LOCKED);
    bench.data.fail(&keys::host::list(), &locked);
    let mut state = State::new();
    let _ = update(&mut state, Message::Failure(Event::Details), &bench.ctx());
    let _ = update(&mut state, Message::Failure(Event::Retry), &bench.ctx());
    assert_eq!(bench.asked(), ["details", "invalidate"]);
}

const SIZE: (f32, f32) = (720.0, 260.0);

fn send(state: &mut State, bench: &Bench, messages: Vec<Message>) {
    for message in messages {
        let _ = update(state, message, &bench.ctx());
    }
}

#[test]
fn a_press_on_a_host_opens_it() {
    let _english = in_language("en");
    let bench = bench(three());
    let mut state = State::new();
    let messages = drive(
        SIZE,
        || view(&state, &bench.ctx()),
        |window| {
            window.click("staging").unwrap();
        },
    );
    send(&mut state, &bench, messages);
    assert_eq!(bench.asked(), ["open"]);
}

#[test]
fn a_header_sorts_the_hosts() {
    let _english = in_language("en");
    let bench = bench(three());
    let mut state = State::new();
    let top_of = |state: &State, name: &str| {
        let mut window = window(SIZE, view(state, &bench.ctx()));
        window.find(name).unwrap().bounds().y
    };
    // As they were read: prod, staging, backup.
    assert!(top_of(&state, "prod") < top_of(&state, "backup"));
    let messages = drive(
        SIZE,
        || view(&state, &bench.ctx()),
        |window| {
            window.click("Name").unwrap();
        },
    );
    send(&mut state, &bench, messages);
    assert!(top_of(&state, "backup") < top_of(&state, "prod"));
    assert!(top_of(&state, "prod") < top_of(&state, "staging"));
    assert!(bench.asked().is_empty(), "sorting opens nothing");
}

#[test]
fn names_sort_as_the_react_table_sorted_them() {
    let _english = in_language("en");
    // Whatever the case, and digits by their value.
    let bench = bench(vec![
        host_view("host-10", connected()),
        host_view("Host-2", connected()),
        host_view("alpha", connected()),
    ]);
    let mut state = State::new();
    let messages = drive(
        SIZE,
        || view(&state, &bench.ctx()),
        |window| {
            window.click("Name").unwrap();
        },
    );
    send(&mut state, &bench, messages);
    let mut window = window(SIZE, view(&state, &bench.ctx()));
    let mut top = |name: &str| window.find(name).unwrap().bounds().y;
    let (alpha, two, ten) = (top("alpha"), top("Host-2"), top("host-10"));
    assert!(alpha < two && two < ten, "{alpha} {two} {ten}");
}

#[test]
fn the_status_sorts_by_what_it_is_and_not_by_what_it_is_called() {
    // Connected, connecting, disabled: the order of the states' names
    // in the code, in Arabic as in English.
    for language in ["en", "ar"] {
        let _language = in_language(language);
        let bench = bench(three());
        let mut state = State::new();
        let header = t!("hosts.column.status");
        let messages = drive(
            SIZE,
            || view(&state, &bench.ctx()),
            |window| {
                window.click(header.as_str()).unwrap();
            },
        );
        send(&mut state, &bench, messages);
        let mut window = window(SIZE, view(&state, &bench.ctx()));
        let mut top = |name: &str| window.find(name).unwrap().bounds().y;
        let (prod, staging, backup) = (top("prod"), top("staging"), top("backup"));
        assert!(prod < staging && staging < backup, "in {language}");
    }
}

#[test]
fn a_list_that_fails_shows_its_failure_and_not_what_was_read_before() {
    let _english = in_language("en");
    let mut bench = bench(three());
    let locked = AppError::new(ErrorCode::DbLocked, homeport_storage::messages::DB_LOCKED);
    bench.data.fail(&keys::host::list(), &locked);
    let state = State::new();
    let mut window = window(SIZE, view(&state, &bench.ctx()));
    assert!(window.find("Retry").is_ok(), "the failure, in place");
    assert!(window.find("prod").is_err(), "and no table under it");
}

#[test]
fn the_page_while_the_hosts_are_read() {
    let bench = Bench::at(Route::Hosts);
    let _ = open(&bench.ctx());
    let state = State::new();
    assert_snapshot("hosts-loading-en", "en", &Theme::Light, SIZE, || {
        view(&state, &bench.ctx())
    });
}

#[test]
fn the_page_in_english() {
    let bench = bench(three());
    let state = State::new();
    assert_snapshot("hosts-en", "en", &Theme::Light, SIZE, || {
        view(&state, &bench.ctx())
    });
}

#[test]
fn the_page_in_arabic_is_mirrored() {
    let bench = bench(three());
    let state = State::new();
    assert_snapshot("hosts-ar", "ar", &Theme::Light, SIZE, || {
        view(&state, &bench.ctx())
    });
}

#[test]
fn the_page_with_no_host() {
    let bench = bench(vec![]);
    let state = State::new();
    assert_snapshot(
        "hosts-empty-en",
        "en",
        &Theme::Light,
        (720.0, 260.0),
        || view(&state, &bench.ctx()),
    );
}

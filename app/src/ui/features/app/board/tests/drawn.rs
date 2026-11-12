//! How the Board looks in each of its states.

use super::*;
use crate::{test_support::snapshot::assert_snapshot, ui::theme::Theme};

fn snapshot(rig: &Rig, name: &str, language: &str) {
    assert_snapshot(name, language, &Theme::Light, SIZE, || rig.view());
}

#[test]
fn the_board_over_two_hosts_and_one_that_is_down() {
    let rig = Rig::populated();
    snapshot(&rig, "board-populated-en", "en");
}

#[test]
fn the_board_in_arabic_is_mirrored() {
    let rig = Rig::populated();
    snapshot(&rig, "board-populated-ar", "ar");
}

#[test]
fn the_board_filtered_with_a_selection_in_the_dark() {
    let mut rig = Rig::populated();
    // Two of the four: those of the server.
    rig.send(Message::Search("prod".to_owned()));
    assert!(rig.shows("api") && rig.shows("cache") && !rig.shows("web"));
    let key = "prod-server-id/api-0123456789abcdef".to_owned();
    rig.send(Message::Table(crate::ui::widgets::table::Message::Toggle {
        key,
        index: 0,
    }));
    assert_snapshot("board-filtered-dark-en", "en", &Theme::Dark, SIZE, || {
        rig.view()
    });
}

#[test]
fn the_board_with_no_connected_host() {
    let rig = Rig::over(vec![hosts().remove(2)]);
    snapshot(&rig, "board-empty-en", "en");
}

#[test]
fn the_board_when_no_host_runs_a_container() {
    let mut rig = Rig::over(hosts());
    rig.read("h1", vec![]);
    rig.read(PROD, vec![]);
    snapshot(&rig, "board-no-containers-en", "en");
}

#[test]
fn the_board_while_its_hosts_are_read() {
    let rig = Rig::over(hosts());
    snapshot(&rig, "board-loading-en", "en");
}

#[test]
fn the_board_when_a_host_s_list_failed() {
    let mut rig = Rig::populated();
    rig.bench
        .data
        .fail(&keys::host::containers(PROD), &refused());
    rig.refresh();
    snapshot(&rig, "board-error-en", "en");
}

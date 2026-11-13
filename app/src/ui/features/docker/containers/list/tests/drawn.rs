//! How the page looks in each of its states.

use homeport_core::error::{AppError, ErrorCode};

use super::*;
use crate::{test_support::snapshot::assert_snapshot, ui::theme::Theme};

fn snapshot(rig: &Rig, name: &str, language: &str) {
    assert_snapshot(name, language, &Theme::Light, SIZE, || rig.view());
}

#[test]
fn the_page_with_stacks_usage_and_a_newer_image() {
    let rig = Rig::populated();
    snapshot(&rig, "containers-populated-en", "en");
}

#[test]
fn the_page_in_arabic_is_mirrored() {
    let rig = Rig::populated();
    snapshot(&rig, "containers-populated-ar", "ar");
}

#[test]
fn the_page_in_the_dark_with_a_selection() {
    let mut rig = Rig::populated();
    let key = "redis-0123456789abcdef".to_owned();
    rig.send(Message::Table(crate::ui::widgets::table::Message::Toggle {
        key,
        index: 5,
    }));
    assert_snapshot(
        "containers-selected-dark-en",
        "en",
        &Theme::Dark,
        SIZE,
        || rig.view(),
    );
}

#[test]
fn a_host_with_no_container() {
    let rig = Rig::with(vec![]);
    snapshot(&rig, "containers-empty-en", "en");
}

#[test]
fn the_page_while_its_containers_are_read() {
    let rig = Rig::on(connected_host(), None);
    snapshot(&rig, "containers-loading-en", "en");
}

#[test]
fn the_page_when_its_containers_could_not_be_read() {
    let mut rig = Rig::populated();
    let down = AppError::new(
        ErrorCode::HostNotConnected,
        homeport_hosts::messages::NOT_CONNECTED_UNKNOWN,
    );
    rig.bench.data.fail(&keys::host::containers("h1"), &down);
    snapshot(&rig, "containers-error-en", "en");
}

#[test]
fn the_rename_dialog_over_the_page() {
    use crate::ui::domain::containers::{actions, Target};
    let mut rig = Rig::populated();
    let redis = Target::of("h1", "local", &sample()[3]);
    rig.send(Message::Actions(actions::Message::AskRename(redis)));
    snapshot(&rig, "containers-rename-en", "en");
}

/// The search field holding a query, at the page's least width: its
/// button that empties it shows, and the line of filters still fits.
#[test]
fn the_page_filtered_by_a_search_that_can_be_emptied() {
    let mut rig = Rig::populated();
    rig.send(Message::Search("redis".to_owned()));
    snapshot(&rig, "containers-searched-en", "en");
    snapshot(&rig, "containers-searched-ar", "ar");
}

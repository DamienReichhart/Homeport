//! How the page looks in each of its states.

use homeport_engine::images::freshness::{Freshness, FreshnessResult};

use super::*;
use crate::{test_support::snapshot::assert_snapshot, ui::theme::Theme};

fn snapshot(rig: &Rig, name: &str, language: &str) {
    assert_snapshot(name, language, &Theme::Light, SIZE, || rig.view());
}

fn dark(rig: &Rig, name: &str) {
    assert_snapshot(name, "en", &Theme::Dark, SIZE, || rig.view());
}

/// The sample's `web`, with a newer image published for it.
fn web(tab: &str) -> Rig {
    let mut rig = Rig::at(WEB, tab);
    let newer = FreshnessResult {
        reference: rig.container().image,
        image_id: "sha256:abc".to_owned(),
        freshness: Freshness::Newer,
    };
    rig.bench
        .data
        .set(&keys::host::freshness("h1"), vec![newer]);
    rig.refresh();
    rig
}

#[test]
fn a_container_that_is_gone() {
    let mut rig = Rig::at(WEB, "logs");
    rig.list(vec![]);
    snapshot(&rig, "container-gone-en", "en");
    snapshot(&rig, "container-gone-ar", "ar");
}

#[test]
fn the_page_while_the_containers_are_read_and_when_they_could_not_be() {
    let rig = Rig::on(connected_host(), route(WEB, None).into(), None);
    snapshot(&rig, "container-loading-en", "en");
    // No Engine answers a test: the read fails as a Host that is down.
    let rig = Rig::unread(route(WEB, None).into());
    snapshot(&rig, "container-error-en", "en");
}

#[test]
fn the_exec_tab_stands_in_for_the_terminal_of_its_ticket() {
    let rig = web("exec");
    snapshot(&rig, "container-exec-en", "en");
    snapshot(&rig, "container-exec-ar", "ar");
    dark(&rig, "container-exec-dark-en");
}

#[test]
fn a_stopped_container_has_no_shell() {
    let rig = Rig::at("shop-db-1-0123456789abcdef", "exec");
    {
        let _english = crate::test_support::snapshot::in_language("en");
        assert!(rig.shows("Start the container to open a terminal."));
    }
    snapshot(&rig, "container-exec-stopped-en", "en");
}

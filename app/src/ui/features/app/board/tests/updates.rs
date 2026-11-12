//! What the Board reads and what each message changes.

use homeport_engine::containers::ContainerAction;

use super::*;
use crate::{
    test_support::{snapshot::in_language, tasks},
    ui::{
        domain::containers::{actions, Bulk},
        shell::{page::testing::Asked, registry, selection::Selection},
        widgets::{feedback::error_message::Event, form::select, table},
    },
};

const NAMES: [&str; 4] = ["api", "cache", "db", "web"];

#[test]
fn it_lists_the_containers_of_every_connected_host_each_with_its_host() {
    let _english = in_language("en");
    let rig = Rig::populated();
    // By Host, then by name; the Host that doesn't answer is named, not
    // shown, and doesn't keep the others waiting.
    assert_eq!(rig.order(&NAMES), ["db", "web", "api", "cache"]);
    assert!(rig.shows("3 of 4 containers running on 2 hosts"));
    assert!(rig.shows("nas (Unreachable) isn’t shown."));
    assert!(rig.top("local") < rig.top("prod-server"));
}

#[test]
fn it_reads_every_connected_docker_host_and_those_that_connect_later() {
    let list = |prod: HostStatus| {
        let mut list = hosts();
        list[1].status = prod;
        let mut incus = host_view("vms", connected());
        incus.host.engine = homeport_core::host::EngineKind::Incus;
        list.push(incus);
        list
    };
    let mut rig = Rig::over(list(HostStatus::Connecting));
    // Not the one that is connecting, the one that is down, or Incus.
    assert_eq!(rig.bench.watched(), [keys::host::containers("h1")]);
    rig.refresh();
    assert!(
        rig.bench.watched().is_empty(),
        "nothing changed, nothing asked"
    );
    rig.bench.set_hosts(list(connected()));
    rig.refresh();
    assert_eq!(rig.bench.watched(), [keys::host::containers(PROD)]);
    assert_eq!(rig.state.sources.len(), 2);
}

#[test]
fn the_rows_are_made_again_only_when_a_list_is_another_one() {
    let mut rig = Rig::populated();
    let revision = rig.state.revision;
    rig.refresh();
    rig.refresh();
    assert_eq!(rig.state.revision, revision);
    // A usage report changes what the rows show, not which or in what
    // order, unless the Board is sorted by it.
    rig.send(Message::Usage((PROD.to_owned(), usage(&prod()))));
    assert_eq!(rig.state.revision, revision);
    rig.read(PROD, vec![container("api", None, "exited", now())]);
    assert_eq!(rig.state.revision, revision + 1);
    assert_eq!(rig.state.counts, (1, 3));
    // A Host that disconnects takes its rows and its usage with it.
    let mut list = hosts();
    list[1].status = HostStatus::Connecting;
    rig.bench.set_hosts(list);
    rig.refresh();
    assert_eq!(rig.state.counts, (1, 2));
    assert!(!rig.state.usage.contains_key(PROD));
}

#[test]
fn it_filters_by_host_state_stack_and_search_and_keeps_the_filter() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.send(Message::ToggleHost(PROD.to_owned()));
    assert_eq!(rig.order(&NAMES), ["api", "cache"]);
    assert!(rig.shows("1 host") && rig.shows("Clear filters"));
    // The Host that is down is filtered out, so it is no longer named.
    assert!(!rig.shows("nas (Unreachable) isn’t shown."));
    rig.send(Message::ToggleHost(PROD.to_owned()));
    rig.send(Message::StateSelect(select::Message::Choose(2)));
    assert_eq!(rig.order(&NAMES), ["db"]);
    rig.send(Message::StateSelect(select::Message::Choose(0)));
    // "Any Stack", then blog and shop.
    rig.send(Message::StackSelect(select::Message::Choose(2)));
    assert_eq!(rig.order(&NAMES), ["db", "web"]);
    rig.send(Message::Search("PROD".to_owned()));
    assert!(rig.order(&NAMES).is_empty());
    assert!(rig.shows("No container matches these filters."));
    rig.send(Message::ClearFilters);
    assert_eq!(rig.order(&NAMES).len(), 4);
    assert!(!rig.shows("Clear filters"));
    // Every change is kept in the location.
    let kept = rig.bench.took();
    assert!(kept.iter().all(|asked| *asked == Asked::Keep) && kept.len() == 7);
}

#[test]
fn it_keeps_its_filters_after_opening_a_container_and_coming_back() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.send(Message::Search("ap".to_owned()));
    rig.send(Message::StateSelect(select::Message::Choose(1)));
    let _ = rig.bench.took();
    // The window goes to the container and comes back: the page is
    // opened anew on its location.
    let (state, _) = open(&rig.bench.ctx());
    rig.state = state;
    rig.refresh();
    assert_eq!(rig.state.filter.query, "ap");
    assert_eq!(rig.order(&NAMES), ["api"]);
    // Opened from a Host's card of the Overview: `/board?host=h1`.
    let rig = Rig::populated_at(Request::to(Route::Board).with_param("host", "h1"));
    assert_eq!(rig.order(&NAMES), ["db", "web"]);
}

#[test]
fn a_header_sorts_and_ties_keep_the_host_then_the_name() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.frame(|frame| frame.click("Name"));
    assert_eq!(rig.order(&NAMES), ["api", "cache", "db", "web"]);
    rig.frame(|frame| frame.click("CPU"));
    // Ascending first: the one that reports nothing, then by its use.
    assert_eq!(rig.order(&NAMES)[0], "db");
    rig.frame(|frame| frame.click("CPU"));
    assert_eq!(rig.order(&NAMES)[3], "db");
}

#[test]
fn a_press_on_a_container_selects_its_host_and_opens_its_logs() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.frame(|frame| frame.click("api"));
    let logs = Route::Container {
        id: "api-0123456789abcdef".to_owned(),
        tab: Some("logs".to_owned()),
    };
    assert_eq!(
        rig.bench.took(),
        [
            Asked::Select(Selection::Host {
                id: PROD.to_owned()
            }),
            Asked::Open(logs.into()),
        ]
    );
}

fn tick(rig: &mut Rig, keys: &[&str]) {
    for (index, key) in keys.iter().enumerate() {
        rig.send(Message::Table(table::Message::Toggle {
            key: (*key).to_owned(),
            index,
        }));
    }
}

#[test]
fn a_selection_that_spans_two_hosts_asks_each_host_and_names_each_failure() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    let keys = [
        "h1/web-0123456789abcdef",
        "prod-server-id/api-0123456789abcdef",
    ];
    tick(&mut rig, &keys);
    assert!(rig.shows("2 selected"));
    // No Engine answers in a test: each Host refuses its own.
    rig.send(Message::Bulk(Bulk::Stop));
    let asked = rig.bench.took();
    let Asked::Failed(None, error) = &asked[0] else {
        panic!("{asked:?}");
    };
    assert_eq!(error.code, ErrorCode::HostNotConnected);
    let named = error.detail.as_deref().unwrap_or_default();
    assert!(named.contains("local/web: ") && named.contains("prod-server/api: "));
    assert_eq!(
        asked[1..],
        [
            Asked::Invalidate(keys::host::containers("h1")),
            Asked::Invalidate(keys::host::containers(PROD)),
        ]
    );
    // Remove names what goes, Host by Host.
    rig.send(Message::Bulk(Bulk::Remove));
    assert!(rig.shows("Remove 2 containers?"));
}

#[test]
fn a_row_that_changes_keeps_the_selection_and_the_scroll() {
    let mut rig = Rig::over(hosts());
    let many = |state: &str| -> Vec<Container> {
        (0..80)
            .map(|n| container(&format!("job-{n:02}"), None, state, now()))
            .collect()
    };
    rig.read("h1", many("running"));
    rig.frame(|_| {});
    rig.send(Message::Table(table::Message::Scrolled(900.0)));
    tick(&mut rig, &["h1/job-30-0123456789abcdef"]);
    let scrolled = rig.state.table.offset();
    assert!(scrolled > 0.0);
    // Engine events: every container exited, and another Host answered.
    rig.read("h1", many("exited"));
    rig.read(PROD, prod());
    rig.frame(|_| {});
    assert!((rig.state.table.offset() - scrolled).abs() < f32::EPSILON);
    let selection = rig.state.table.selection();
    assert!(selection.contains(&"h1/job-30-0123456789abcdef".to_owned()));
    assert_eq!(selection.len(), 1);
    drop(selection);
    assert_eq!(rig.state.counts, (2, 82));
}

#[test]
fn a_host_whose_list_failed_is_named_and_read_again() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.bench
        .data
        .fail(&keys::host::containers(PROD), &refused());
    rig.refresh();
    assert!(rig.shows("Couldn’t list the containers of prod-server."));
    assert!(rig.shows("web"), "the other Host's are still listed");
    rig.send(Message::Failure(PROD.to_owned(), Event::Details));
    rig.send(Message::Failure(PROD.to_owned(), Event::Retry));
    let host = Route::Host {
        host_id: "nas-id".to_owned(),
    };
    rig.send(Message::Go(host.clone()));
    assert_eq!(
        rig.bench.took(),
        [
            Asked::Details(refused()),
            Asked::Invalidate(keys::host::containers(PROD)),
            Asked::Open(host.into()),
        ]
    );
}

#[test]
fn the_search_shortcut_gives_the_field_focus() {
    let mut rig = Rig::populated();
    let task = super::super::request(&mut rig.state, registry::SEARCH, &rig.bench.ctx());
    assert!(!tasks::is_none(task));
    let other = super::super::request(&mut rig.state, "run", &rig.bench.ctx());
    assert!(tasks::is_none(other));
}

#[test]
fn a_bulk_action_that_half_worked_says_both_and_keeps_the_page_as_it_is() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    let keys = [
        "h1/db-0123456789abcdef",
        "h1/web-0123456789abcdef",
        "prod-server-id/api-0123456789abcdef",
        "prod-server-id/cache-0123456789abcdef",
    ];
    tick(&mut rig, &keys);
    // Two Hosts, four containers: one of each Host is stopped, the other
    // refused. The Engine's answer, as the shell and the page get it.
    let targets: Vec<_> = rig.state.rows.iter().map(|row| row.target()).collect();
    let results: actions::Results = targets
        .into_iter()
        .enumerate()
        .map(|(at, target)| (target, if at % 2 == 0 { Ok(()) } else { Err(refused()) }))
        .collect();
    rig.bench
        .told(actions::acted(ContainerAction::Stop, &results));
    rig.send(Message::Actions(actions::Message::Acted(
        ContainerAction::Stop,
        results,
    )));
    let asked = rig.bench.took();
    assert_eq!(asked[0], Asked::Said("Stopped 2 containers".to_owned()));
    let Asked::Failed(None, error) = &asked[1] else {
        panic!("{asked:?}");
    };
    assert_eq!(
        error.detail.as_deref(),
        Some("local/web: connection refused\nprod-server/cache: connection refused")
    );
    assert_eq!(
        asked[2..],
        [
            Asked::Invalidate(keys::host::containers("h1")),
            Asked::Invalidate(keys::host::containers(PROD)),
        ]
    );
    // Every one stays selected, for another try at those that failed.
    assert_eq!(rig.state.table.selection().len(), 4);
}

//! The palette's items and ranking, as `items.test.ts` held them, and what
//! is highlighted and chosen.

use homeport_core::error::{AppError, ErrorCode, Message as Text};
use homeport_hosts::{registry::HostStatus, service::HostView};

use super::{
    items::{items, recent_items, remember, score, sections, shown, Act, Group, Item, Source},
    Message, State,
};
use crate::{
    test_support::{
        fixtures::{connected_host, host_view},
        snapshot::in_language,
    },
    ui::shell::{
        registry::{feature, nav, Capability, Feature, FeatureId, Run, Scope, FEATURES},
        router::Route,
    },
};

fn unreachable() -> HostView {
    let error = AppError::new(
        ErrorCode::Timeout,
        Text {
            key: "errors.test.timeout",
            en: "Timed out",
        },
    );
    let status = HostStatus::Unreachable {
        error,
        retry_in_secs: 4,
    };
    host_view("prod-server", status)
}

fn two() -> Vec<&'static Feature> {
    vec![feature(FeatureId::Containers), feature(FeatureId::Volumes)]
}

fn source<'a>(hosts: &'a [HostView], selected: Option<&'a HostView>) -> Source<'a> {
    Source {
        features: two(),
        app_commands: &[],
        hosts,
        selected,
        nav_keys: vec![("/containers".to_owned(), "Mod+2".to_owned())],
    }
}

fn listed(items: &[Item]) -> Vec<(String, String)> {
    items
        .iter()
        .map(|item| (item.group.label(), item.label.clone()))
        .collect()
}

#[test]
fn the_pages_come_first_then_every_host_with_its_status_then_the_commands() {
    let _english = in_language("en");
    let hosts = [connected_host(), unreachable()];
    let items = items(&source(&hosts, hosts.first()));
    let pair = |group: &str, label: &str| (group.to_owned(), label.to_owned());
    assert_eq!(
        listed(&items),
        [
            pair("Go to", "Go to Containers"),
            pair("Go to", "Go to Volumes"),
            pair("Hosts", "Switch to local"),
            pair("Hosts", "Switch to prod-server"),
            pair("Actions", "Run a container"),
        ]
    );
    assert_eq!(items[0].shortcut.as_deref(), Some("Mod+2"));
    assert_eq!(items[3].status.as_ref().unwrap().0, "Unreachable");
}

#[test]
fn commands_that_need_a_connected_host_hide_when_the_selected_one_isn_t() {
    let _english = in_language("en");
    let hosts = [connected_host(), unreachable()];
    let labels = |selected| -> Vec<String> {
        let mut source = source(&hosts, selected);
        source.features.push(feature(FeatureId::Hosts));
        items(&source).into_iter().map(|i| i.label).collect()
    };
    assert!(!labels(hosts.get(1)).contains(&"Run a container".to_owned()));
    assert!(labels(hosts.get(1)).contains(&"Add host".to_owned()));
    assert!(labels(hosts.first()).contains(&"Run a container".to_owned()));
    assert!(!labels(None).contains(&"Run a container".to_owned()));
}

#[test]
fn pages_rank_first_so_vol_opens_volumes() {
    let _english = in_language("en");
    let hosts = [connected_host(), unreachable()];
    let items = items(&source(&hosts, hosts.first()));
    let best = sections(&items, "vol", &[])[0].items[0];
    assert_eq!(best.label, "Go to Volumes");
    assert!(score(&items[0], "vol") == 0.0);
    // A host is found by its name.
    let best = sections(&items, "prod", &[])[0].items[0];
    assert_eq!(best.label, "Switch to prod-server");
}

#[test]
fn a_match_ranks_by_where_it_starts() {
    let _english = in_language("en");
    let hosts = [connected_host()];
    let items = items(&source(&hosts, hosts.first()));
    let run = items
        .iter()
        .find(|i| i.id == "command:containers:run")
        .unwrap();
    assert!(
        (score(run, "run") - 0.9).abs() < 1e-9,
        "its label starts with it"
    );
    assert!(
        (score(run, "cont") - 0.8).abs() < 1e-9,
        "a word starts with it"
    );
    assert!((score(run, "tain") - 0.5).abs() < 1e-9, "it holds it");
    assert!((score(run, "new") - 0.9 * 0.95).abs() < 1e-9, "a keyword");
    let page = &items[0];
    assert!(
        (score(page, "containers") - 0.875).abs() < 1e-9,
        "a keyword, and a page"
    );
    assert!((score(page, "  ") - 1.0).abs() < 1e-9);
}

#[test]
fn the_last_few_items_are_remembered_most_recent_first() {
    let _english = in_language("en");
    let mut recent: Vec<String> = vec![];
    for id in ["a", "b", "c", "a", "d", "e", "f"] {
        recent = remember(&recent, id);
    }
    assert_eq!(recent, ["f", "e", "d", "a", "c"]);
    let hosts = [connected_host()];
    let items = items(&source(&hosts, hosts.first()));
    let kept = ["nav:/volumes".to_owned(), "gone".to_owned()];
    let labels: Vec<&str> = recent_items(&items, &kept)
        .iter()
        .map(|i| i.label.as_str())
        .collect();
    assert_eq!(labels, ["Go to Volumes"]);
    let browsed = sections(&items, "", &kept);
    assert_eq!(browsed[0].group, Group::Recent);
    assert_eq!(browsed[1].group, Group::GoTo);
}

#[test]
fn the_pages_of_what_isn_t_selected_and_their_commands_aren_t_offered() {
    let all = nav::entries_of(FEATURES);
    let cluster = nav::visible(&all, Some(Scope::Cluster), |_| false, None);
    let ids: Vec<FeatureId> = shown(FEATURES, &cluster).iter().map(|f| f.id).collect();
    assert!(!ids.contains(&FeatureId::Containers));
    assert!(ids.contains(&FeatureId::Pods));
    assert!(ids.contains(&FeatureId::Hosts), "no sidebar entry: always");
    let docker = nav::visible(&all, Some(Scope::Host), |c| c == Capability::Docker, None);
    let ids: Vec<FeatureId> = shown(FEATURES, &docker).iter().map(|f| f.id).collect();
    assert!(ids.contains(&FeatureId::Containers));
    assert!(!ids.contains(&FeatureId::Quadlets));
}

#[test]
fn the_arrows_move_the_highlight_and_enter_runs_the_row() {
    let _english = in_language("en");
    let hosts = [connected_host(), unreachable()];
    let items = items(&source(&hosts, hosts.first()));
    let mut palette = State::default();
    assert_eq!(palette.update(Message::Toggle, &items), None);
    assert!(palette.is_open());
    palette.update(Message::Step(-1), &items);
    assert_eq!(palette.active(), 0, "it stops at the top");
    palette.update(Message::Step(1), &items);
    let act = palette.update(Message::Submit, &items);
    assert_eq!(act, Some(Act::Open(Route::Volumes)));
    assert!(!palette.is_open());
    assert_eq!(palette.recent(), ["nav:/volumes"]);
    // Typing starts again from the best match.
    palette.update(Message::Toggle, &items);
    palette.update(Message::Step(3), &items);
    palette.update(Message::Query("run".to_owned()), &items);
    assert_eq!(palette.active(), 0);
    let act = palette.update(Message::Submit, &items);
    assert_eq!(
        act,
        Some(Act::Run(Run::Open(Route::Containers, Some("run"))))
    );
    // Closing forgets what was typed; the highlight stops at the last row.
    palette.update(Message::Toggle, &items);
    palette.update(Message::Query("zzz".to_owned()), &items);
    assert_eq!(palette.update(Message::Submit, &items), None);
    palette.update(Message::Close, &items);
    assert_eq!(palette.query(), "");
    palette.update(Message::Toggle, &items);
    palette.update(Message::Step(99), &items);
    assert_eq!(palette.active(), palette.listed(&items).len() - 1);
}

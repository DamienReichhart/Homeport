//! What the page reads and what each message changes.

use homeport_core::error::{AppError, ErrorCode};
use homeport_engine::containers::ContainerAction;

use super::*;
use crate::{
    t,
    test_support::snapshot::in_language,
    ui::{
        domain::containers::{actions, Target, CAME_FROM},
        shell::page::testing::Asked,
        widgets::{basic::technical, feedback::error_message::Event, overlay::menu},
    },
};

fn target(rig: &Rig) -> Target {
    Target::of("h1", "local", &rig.container())
}

#[test]
fn opening_reads_the_host_s_containers_and_shows_the_tab_of_the_route() {
    let mut bench = Bench::on_host(Route::Containers, connected_host());
    bench.history.push(route(WEB, Some("stats")).into());
    let (state, _) = open(&bench.ctx());
    assert_eq!(
        bench.watched()[..2],
        [keys::host::containers("h1"), keys::host::freshness("h1")]
    );
    assert_eq!((state.tab, state.id.as_str()), (Tab::Stats, WEB));
    // `containers/:id`, and a tab there isn't, are the logs.
    for tab in [None, Some("nope")] {
        bench.history.push(route(WEB, tab).into());
        assert_eq!(open(&bench.ctx()).0.tab, Tab::Logs);
    }
    // Every tab is found again from what its route calls it.
    for tab in Tab::ALL {
        assert_eq!(Tab::of(Some(tab.slug())), tab);
    }
}

#[test]
fn a_tab_is_kept_in_the_route_and_adds_nothing_to_the_history() {
    let mut rig = Rig::at(WEB, "logs");
    let entries = rig.bench.history.count();
    rig.send(Message::Tab(Tab::Inspect));
    assert_eq!(rig.state.tab, Tab::Inspect);
    let asked = rig.bench.took();
    assert!(
        asked.contains(&Asked::KeepRoute(route(WEB, Some("inspect")))),
        "{asked:?}"
    );
    assert!(!asked.iter().any(|asked| matches!(asked, Asked::Open(_))));
    assert_eq!(rig.bench.history.count(), entries);
    assert_eq!(
        rig.bench.history.current().route,
        route(WEB, Some("inspect"))
    );
    // The tab that shows, pressed again, asks for nothing.
    rig.send(Message::Tab(Tab::Inspect));
    assert!(rig.bench.took().is_empty());
}

#[test]
fn the_tabs_are_react_s_and_health_shows_only_with_a_healthcheck() {
    let _english = in_language("en");
    let rig = Rig::at(WEB, "logs");
    let labels = [
        "Logs",
        "Inspect",
        "Health",
        "Exec",
        "Files",
        "Stats",
        "Processes",
    ];
    assert_eq!(Tab::ALL.map(Tab::label), labels);
    assert_eq!(Tab::shown(&rig.container()), Tab::ALL);
    assert!(rig.shows("Health"));
    let rig = Rig::at("redis-0123456789abcdef", "logs");
    assert!(!Tab::shown(&rig.container()).contains(&Tab::Health));
    assert!(!rig.shows("Health") && rig.shows("Processes"));
}

#[test]
fn a_press_on_a_tab_shows_it() {
    let _english = in_language("en");
    let mut rig = Rig::at(WEB, "logs");
    rig.frame(|frame| frame.click("Processes"));
    assert_eq!(rig.state.tab, Tab::Processes);
}

#[test]
fn back_goes_to_the_list_or_to_where_the_page_was_opened_from() {
    let mut rig = Rig::at(WEB, "logs");
    rig.send(Message::Back);
    assert_eq!(rig.bench.took(), [Asked::Open(Route::Containers.into())]);
    // Opened from the Board: back there, with the filters it kept.
    let from_board = Request::to(route(WEB, Some("logs"))).with_param(CAME_FROM, "board");
    let mut rig = Rig::on(connected_host(), from_board, Some(sample()));
    rig.send(Message::Back);
    assert_eq!(rig.bench.took(), [Asked::Back]);
}

#[test]
fn the_board_the_page_came_from_survives_a_tab_and_a_reopen() {
    let from_board = Request::to(route(WEB, Some("logs"))).with_param(CAME_FROM, "board");
    let mut rig = Rig::on(connected_host(), from_board, Some(sample()));
    let key = rig.bench.history.current().key;
    rig.send(Message::Tab(Tab::Stats));
    // The shell keeps the tab in the location, and the location keeps
    // where it came from.
    let _ = rig.bench.took();
    let location = rig.bench.history.current();
    assert_eq!(location.route, route(WEB, Some("stats")));
    assert_eq!(location.param(CAME_FROM), Some("board"));
    assert_eq!(location.key, key);
    // The window comes back to the page (back from another, a Host
    // selected again): opened anew from that location, Back still goes
    // back to the Board, not to the list.
    let (reopened, _) = open(&rig.bench.ctx());
    rig.state = reopened;
    let _ = rig.bench.took();
    assert_eq!(rig.state.tab, Tab::Stats);
    rig.send(Message::Back);
    assert_eq!(rig.bench.took(), [Asked::Back]);
}

#[test]
fn an_id_a_name_or_a_start_no_other_id_shares_finds_the_container() {
    let mut list = sample();
    let mut twin = list[0].clone();
    "shop-web-1-0123999".clone_into(&mut twin.id);
    "twin".clone_into(&mut twin.name);
    list.push(twin);
    let found = |id: &str| {
        let rig = Rig::on(connected_host(), route(id, None).into(), Some(list.clone()));
        rig.state.find(&list).map(|c| c.id.clone())
    };
    assert_eq!(found(WEB).as_deref(), Some(WEB));
    assert_eq!(found("redis").as_deref(), Some("redis-0123456789abcdef"));
    assert_eq!(
        found("shop-db").as_deref(),
        Some("shop-db-1-0123456789abcdef")
    );
    // Two ids start so: neither is the page's, and the page says why.
    assert_eq!(found("shop-web-1-0123"), None);
    {
        let _english = in_language("en");
        let rig = Rig::on(
            connected_host(),
            route("shop-web-1-0123", None).into(),
            Some(list.clone()),
        );
        let said = format!(
            "More than one container's id starts with {}.",
            technical("shop-web-1-0123")
        );
        assert!(rig.shows(&said));
    }
    // Once found, a tab's route names it whole.
    let mut rig = Rig::at("shop-db", "logs");
    rig.send(Message::Tab(Tab::Stats));
    let _ = rig.bench.took();
    assert_eq!(
        rig.bench.history.current().route,
        route("shop-db-1-0123456789abcdef", Some("stats"))
    );
}

#[test]
fn the_start_of_an_id_finds_its_container_and_an_empty_one_finds_none() {
    let _english = in_language("en");
    let rig = Rig::at("shop-web-1-0123", "logs");
    assert_eq!(rig.container().id, WEB);
    assert!(rig.shows(&technical("shop-web-1")));
    let rig = Rig::at("", "logs");
    assert!(rig.shows(&t!("containers.detail.gone")));
}

#[test]
fn a_container_that_is_gone_says_so_and_leads_back_to_the_list() {
    let _english = in_language("en");
    let mut rig = Rig::at(WEB, "logs");
    rig.list(vec![sample().remove(2)]);
    assert!(rig.shows("This container no longer exists."));
    assert!(!rig.shows("Logs"), "no tab of a container that is gone");
    rig.frame(|frame| frame.click("Back to containers"));
    assert_eq!(rig.bench.took(), [Asked::Open(Route::Containers.into())]);
    // Its Details say the same: no button of the page does nothing.
    rig.frame(|frame| frame.click("Details"));
    let asked = rig.bench.took();
    assert!(
        matches!(asked.as_slice(), [Asked::Details(error)] if error.key == "containers.detail.gone"),
        "{asked:?}"
    );
}

#[test]
fn a_list_that_could_not_be_read_is_said_and_read_again() {
    let _english = in_language("en");
    let mut rig = Rig::unread(route(WEB, None).into());
    let down = rig.state.containers(&rig.bench.ctx());
    let down = down.error().expect("no Engine answers a test").clone();
    assert_eq!(down.code, ErrorCode::HostNotConnected);
    assert!(!rig.shows("This container no longer exists."));
    assert!(rig.shows("Retry"));
    rig.send(Message::Failure(Event::Retry));
    assert_eq!(
        rig.bench.took(),
        [Asked::Invalidate(keys::host::containers("h1"))]
    );
    rig.send(Message::Failure(Event::Details));
    assert_eq!(rig.bench.took(), [Asked::Details(down)]);
}

#[test]
fn the_header_s_buttons_are_the_shared_actions_and_tell_the_shell() {
    let _english = in_language("en");
    let mut rig = Rig::at(WEB, "logs");
    // A running container: Stop, Restart and Pause.
    assert!(rig.shows("Stop") && !rig.shows("Start") && rig.shows("Pause"));
    rig.frame(|frame| frame.click("Stop"));
    // No Engine in a test: the action fails, and the shell is the one
    // told, with the list to read again.
    let asked = rig.bench.took();
    assert!(
        matches!(asked.as_slice(), [Asked::Failed(None, error), Asked::Invalidate(key)]
            if error.code == ErrorCode::HostNotConnected && *key == keys::host::containers("h1")),
        "{asked:?}"
    );
    // A stopped one: Start, and nothing to restart or to pause.
    let mut rig = Rig::at("shop-db-1-0123456789abcdef", "logs");
    assert!(rig.shows("Start") && !rig.shows("Stop"));
    rig.frame(|frame| frame.click("Restart"));
    rig.frame(|frame| frame.click("Pause"));
    assert!(rig.bench.took().is_empty(), "both are disabled");
    // A paused one resumes.
    let mut paused = sample();
    "paused".clone_into(&mut paused[0].state);
    rig.list(paused);
    let rig = Rig::on(
        connected_host(),
        route(WEB, None).into(),
        Some({
            let mut list = sample();
            "paused".clone_into(&mut list[0].state);
            list
        }),
    );
    assert!(rig.shows("Resume") && rig.shows("Stop"));
}

#[test]
fn the_menu_renames_kills_exports_and_leaves_the_run_ticket_s_items_disabled() {
    let _english = in_language("en");
    let mut rig = Rig::at(WEB, "logs");
    rig.send(Message::More(menu::Event::Toggle));
    let (commit, export) = ("Commit to image…", "Export filesystem…");
    for label in ["Rename", "Kill", "Edit", "Duplicate", commit, export] {
        assert!(rig.shows(label), "{label}");
    }
    // What waits for its ticket can't be chosen: the menu stays.
    rig.frame(|frame| frame.click("Edit"));
    assert!(rig.shows("Rename") && dialog(&rig.state, &rig.bench.ctx()).is_none());
    // Export asks where to save, through the system's dialog.
    rig.frame(|frame| frame.click(export));
    assert!(matches!(
        rig.bench.os.took().as_slice(),
        [crate::platform::fake::Asked::Save(request)] if request.name == "shop-web-1.tar"
    ));
    rig.send(Message::More(menu::Event::Toggle));
    rig.frame(|frame| frame.click("Rename"));
    assert!(rig.state.actions.has_dialog());
    assert!(rig.shows("Rename shop-web-1"));
}

#[test]
fn a_removed_container_s_page_goes_to_the_list() {
    let mut rig = Rig::at(WEB, "logs");
    let web = target(&rig);
    rig.send(Message::Actions(actions::Message::AskRemove {
        targets: vec![web.clone()],
        by_host: false,
    }));
    assert!(rig.state.actions.has_dialog());
    // The Engine refused: the page stays.
    let refused = AppError::new(
        ErrorCode::HostNotConnected,
        homeport_hosts::messages::NOT_CONNECTED_UNKNOWN,
    );
    rig.send(Message::Actions(actions::Message::Removed(vec![(
        web.clone(),
        Err(refused),
    )])));
    assert!(rig.bench.took().is_empty());
    rig.send(Message::Actions(actions::Message::Removed(vec![(
        web,
        Ok(()),
    )])));
    assert_eq!(rig.bench.took(), [Asked::Open(Route::Containers.into())]);
    // An action that is not a removal leaves the page where it is.
    let web = target(&rig);
    rig.send(Message::Actions(actions::Message::Acted(
        ContainerAction::Stop,
        vec![(web, Ok(()))],
    )));
    assert!(rig.bench.took().is_empty());
}

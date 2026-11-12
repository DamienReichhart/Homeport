//! The container actions: what the menu offers for each state, what an
//! action asks of each Host and how it reports, one container at a time.

use homeport_core::error::{AppError, ErrorCode, Message as Text};

use super::*;
use crate::{
    test_support::{
        fixtures::{connected, connected_host, container, host_view},
        snapshot::in_language,
    },
    ui::{
        shell::page::testing::{Asked, Bench},
        widgets::overlay::menu::Item,
    },
};

fn bench() -> Bench {
    Bench::on_host(Route::Containers, connected_host())
}

fn target(name: &str, state: &str) -> Target {
    Target::of("h1", "local", &container(name, None, state, 0))
}

fn on_prod(name: &str) -> Target {
    Target::of("prod-id", "prod", &container(name, None, "running", 0))
}

fn refused() -> AppError {
    AppError::new(
        ErrorCode::Conflict,
        Text {
            key: "errors.test.conflict",
            en: "the container is being removed",
        },
    )
}

/// The items of a menu that can be chosen, by label.
fn offered(items: &[Item<Message>]) -> Vec<&str> {
    items
        .iter()
        .filter_map(|item| match item {
            Item::Action {
                label,
                message: Some(_),
                ..
            } => Some(label.as_str()),
            _ => None,
        })
        .collect()
}

#[test]
fn the_menu_offers_what_the_container_s_state_allows() {
    let _english = in_language("en");
    let running = Actions::menu(&target("web", "running"));
    assert_eq!(
        offered(&running),
        [
            "Logs",
            "Open terminal",
            "Restart",
            "Pause",
            "Kill",
            "Rename",
            "Remove"
        ]
    );
    let paused = Actions::menu(&target("web", "paused"));
    assert_eq!(
        offered(&paused),
        ["Logs", "Restart", "Resume", "Rename", "Remove"]
    );
    let exited = Actions::menu(&target("web", "exited"));
    assert_eq!(offered(&exited), ["Logs", "Rename", "Remove"]);
    // Every item of the React menu is there, those of other tickets too.
    assert_eq!(running.len(), 13);
}

#[test]
fn an_action_asks_the_host_and_names_the_container_it_failed_for() {
    let _english = in_language("en");
    let mut bench = bench();
    let mut actions = Actions::of_host();
    let web = target("web", "exited");
    // No Engine answers in a test: the Host refuses.
    let task = actions.update(
        Message::Act(ContainerAction::Start, vec![web.clone()]),
        &bench.ctx(),
    );
    assert!(actions.is_pending("h1", &web.id));
    for message in bench.run(task) {
        let _ = actions.update(message, &bench.ctx());
    }
    assert!(!actions.is_pending("h1", &web.id));
    let asked = bench.took();
    let [Asked::Failed(Some(title), error), Asked::Invalidate(key)] = asked.as_slice() else {
        panic!("{asked:?}");
    };
    assert_eq!(title, "web");
    assert_eq!(error.code, ErrorCode::HostNotConnected);
    assert_eq!(*key, keys::host::containers("h1"));
}

#[test]
fn what_worked_is_counted_and_each_failure_is_said_host_by_host() {
    let _english = in_language("en");
    let mut bench = bench();
    let mut actions = Actions::across_hosts();
    let results: Results = vec![
        (target("web", "running"), Ok(())),
        (target("db", "running"), Err(refused())),
        (target("cache", "running"), Err(refused())),
        (on_prod("api"), Ok(())),
        (on_prod("queue"), Err(refused())),
    ];
    let _ = actions.update(Message::Acted(ContainerAction::Stop, results), &bench.ctx());
    let asked = bench.took();
    assert_eq!(asked[0], Asked::Said("Stopped 2 containers".to_owned()));
    // The two the same Host refused for the same reason share a toast.
    assert_eq!(
        asked[1..3],
        [
            Asked::Failed(Some("On local: db, cache".to_owned()), refused()),
            Asked::Failed(Some("On prod: queue".to_owned()), refused()),
        ]
    );
    let mut read_again: Vec<_> = asked[3..].to_vec();
    read_again.sort_by_key(|asked| format!("{asked:?}"));
    assert_eq!(
        read_again,
        [
            Asked::Invalidate(keys::host::containers("h1")),
            Asked::Invalidate(keys::host::containers("prod-id")),
        ]
    );
}

#[test]
fn killing_is_confirmed_and_its_dialog_waits_for_the_answer() {
    let _english = in_language("en");
    let mut bench = bench();
    let mut actions = Actions::of_host();
    let web = target("web", "running");
    let _ = actions.update(Message::AskKill(web.clone()), &bench.ctx());
    let Some(Dialog::Confirm { what, .. }) = &actions.dialog else {
        panic!("no confirmation");
    };
    let ask = Actions::ask(what);
    assert_eq!((ask.title.as_str(), ask.destructive), ("Kill web?", true));
    // Confirmed: the Host is asked, and the dialog is busy until it answers.
    let task = actions.update(Message::Confirm(confirm::Message::Confirm), &bench.ctx());
    let answers = bench.run(task);
    assert!(matches!(
        answers.as_slice(),
        [Message::Acted(ContainerAction::Kill, _)]
    ));
    // It failed: the dialog offers it again.
    for message in answers {
        let _ = actions.update(message, &bench.ctx());
    }
    assert!(matches!(&actions.dialog, Some(Dialog::Confirm { state, .. }) if !state.is_busy()));
    // It worked: the dialog goes.
    let _ = actions.update(Message::Confirm(confirm::Message::Confirm), &bench.ctx());
    let killed = Message::Acted(ContainerAction::Kill, vec![(web, Ok(()))]);
    let _ = actions.update(killed, &bench.ctx());
    assert!(!actions.has_dialog());
    assert!(bench
        .took()
        .contains(&Asked::Said("Killed container".to_owned())));
}

#[test]
fn removing_says_what_goes_and_keeps_its_dialog_only_when_nothing_went() {
    let _english = in_language("en");
    let mut bench = bench();
    let mut actions = Actions::of_host();
    let (web, db) = (target("web", "running"), target("db", "exited"));
    let ask = |actions: &Actions| match &actions.dialog {
        Some(Dialog::Confirm { what, .. }) => Actions::ask(what),
        _ => panic!("no confirmation"),
    };
    let remove = |targets: Vec<Target>, by_host| Message::AskRemove { targets, by_host };
    let _ = actions.update(remove(vec![web.clone()], false), &bench.ctx());
    assert_eq!(ask(&actions).title, "Remove web?");
    assert_eq!(
        ask(&actions).description,
        "The container and its writable layer and anonymous volumes are deleted. \
         1 running will be stopped first. Named volumes are kept."
    );
    let _ = actions.update(remove(vec![web.clone(), db.clone()], false), &bench.ctx());
    assert_eq!(ask(&actions).title, "Remove 2 containers?");
    // Confirmed with no Engine: nothing went, the dialog stays for another try.
    let task = actions.update(Message::Confirm(confirm::Message::Confirm), &bench.ctx());
    for message in bench.run(task) {
        let _ = actions.update(message, &bench.ctx());
    }
    assert!(actions.has_dialog());
    // The two a Host refused for one reason are one toast.
    let failures = |asked: &[Asked]| {
        asked
            .iter()
            .filter(|asked| matches!(asked, Asked::Failed(..)))
            .count()
    };
    assert_eq!(failures(&bench.took()), 1);
    // One went, one didn't: the dialog goes, and both are said.
    let _ = actions.update(Message::Confirm(confirm::Message::Confirm), &bench.ctx());
    let results = vec![(web, Ok(())), (db, Err(refused()))];
    let _ = actions.update(Message::Removed(results), &bench.ctx());
    assert!(!actions.has_dialog());
    assert_eq!(
        bench.took()[..2],
        [
            Asked::Said("Removed container".to_owned()),
            Asked::Failed(Some("db".to_owned()), refused()),
        ]
    );
    // Nothing selected, nothing asked.
    let _ = actions.update(remove(vec![], true), &bench.ctx());
    assert!(!actions.has_dialog());
}

#[test]
fn a_selection_across_hosts_is_summed_up_host_by_host() {
    let _english = in_language("en");
    let mut targets = vec![on_prod("api"), target("web", "running")];
    targets.extend((0..10).map(|n| target(&format!("job-{n}"), "exited")));
    let (title, description) = view::removal_by_host(&targets);
    assert_eq!(title, "Remove 12 containers?");
    let lines: Vec<&str> = description.lines().collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(
        lines[1],
        "On local: web, job-0, job-1, job-2, job-3, job-4, job-5, job-6 +3"
    );
    assert_eq!(lines[2], "On prod: api");
}

#[test]
fn renaming_shows_its_failure_in_place_and_closes_once_it_worked() {
    let _english = in_language("en");
    let mut bench = bench();
    let mut actions = Actions::of_host();
    let _ = actions.update(Message::AskRename(target("web", "running")), &bench.ctx());
    let _ = actions.update(Message::RenameInput("site".to_owned()), &bench.ctx());
    let task = actions.update(Message::RenameSubmit, &bench.ctx());
    // While it is on its way a second Enter starts nothing.
    let again = actions.update(Message::RenameSubmit, &bench.ctx());
    assert!(crate::test_support::tasks::is_none(again));
    for message in bench.run(task) {
        let _ = actions.update(message, &bench.ctx());
    }
    let Some(Dialog::Rename { name, failure, .. }) = &actions.dialog else {
        panic!("the dialog closed on a failure");
    };
    assert_eq!(name, "site");
    assert_eq!(
        failure.as_ref().map(|f| f.error.code),
        Some(ErrorCode::HostNotConnected)
    );
    assert!(bench.took().is_empty(), "said in place, not in a toast");
    let details = Message::RenameFailure(error_message::Event::Details);
    let _ = actions.update(details, &bench.ctx());
    assert_eq!(bench.asked(), ["details"]);
    // Another try, which the Engine takes.
    let _ = actions.update(Message::RenameSubmit, &bench.ctx());
    let Some(Dialog::Rename { action, .. }) = &actions.dialog else {
        panic!("no dialog");
    };
    let renamed = Message::Renamed(action.ending_with(Ok(())));
    let _ = actions.update(renamed, &bench.ctx());
    assert!(!actions.has_dialog());
    assert_eq!(
        bench.took(),
        [Asked::Invalidate(keys::host::containers("h1"))]
    );
}

#[test]
fn opening_a_container_of_another_host_selects_that_host_first() {
    let mut bench = Bench::at(Route::Board);
    bench.set_hosts(vec![connected_host(), host_view("prod", connected())]);
    let mut actions = Actions::across_hosts();
    let api = on_prod("api");
    let _ = actions.update(Message::Open(api.clone(), "logs"), &bench.ctx());
    let logs = Route::Container {
        id: api.id.clone(),
        tab: Some("logs".to_owned()),
    };
    assert_eq!(
        bench.took(),
        [
            Asked::Select(Selection::Host {
                id: "prod-id".to_owned()
            }),
            Asked::Open(logs.clone().into()),
        ]
    );
    // On its own Host's list, the selection stays.
    let mut actions = Actions::of_host();
    let _ = actions.update(Message::Open(api, "logs"), &bench.ctx());
    assert_eq!(bench.took(), [Asked::Open(logs.into())]);
}

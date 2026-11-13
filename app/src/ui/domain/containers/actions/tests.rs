//! The container actions: what the menu offers for each state, what an
//! action asks of each Host and how it reports, one container at a time.

use homeport_core::error::{AppError, ErrorCode, Message as Text};

use super::*;
use crate::ui::domain::containers::CAME_FROM;
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
            "Export filesystem…",
            "Remove"
        ]
    );
    let paused = Actions::menu(&target("web", "paused"));
    assert_eq!(
        offered(&paused),
        [
            "Logs",
            "Restart",
            "Resume",
            "Rename",
            "Export filesystem…",
            "Remove"
        ]
    );
    let exited = Actions::menu(&target("web", "exited"));
    assert_eq!(
        offered(&exited),
        ["Logs", "Rename", "Export filesystem…", "Remove"]
    );
    // Every item of the React menu is there, those of other tickets too.
    assert_eq!(running.len(), 13);
}

#[test]
fn an_action_asks_the_host_and_its_failure_is_the_shell_s_to_report() {
    let _english = in_language("en");
    let mut bench = bench();
    let mut actions = Actions::of_host();
    let web = target("web", "exited");
    // No Engine answers in a test: the Host refuses.
    let start = Message::Act(ContainerAction::Start, vec![web.clone()]);
    let task = actions.update(start, &bench.ctx());
    assert!(actions.is_pending("h1", &web.id));
    let answers = bench.run(task);
    // The outcome is told before the page hears of it, and without it.
    let asked = bench.took();
    let [Asked::Failed(None, error), Asked::Invalidate(key)] = asked.as_slice() else {
        panic!("{asked:?}");
    };
    assert_eq!(error.code, ErrorCode::HostNotConnected);
    assert!(error
        .detail
        .as_deref()
        .is_some_and(|d| d.starts_with("local/web: ")));
    assert_eq!(*key, keys::host::containers("h1"));
    for message in answers {
        let _ = actions.update(message, &bench.ctx());
    }
    assert!(!actions.is_pending("h1", &web.id));
    assert!(bench.took().is_empty(), "the page itself reports nothing");
}

#[test]
fn what_worked_is_counted_and_what_failed_is_one_toast_that_lists_each() {
    let _english = in_language("en");
    let mut bench = bench();
    let mut gone = refused();
    gone.detail = Some("No such container: queue".to_owned());
    let results: Results = vec![
        (target("web", "running"), Ok(())),
        (target("db", "running"), Err(refused())),
        (on_prod("api"), Ok(())),
        (on_prod("queue"), Err(gone)),
    ];
    bench.told(acted(ContainerAction::Stop, &results));
    let asked = bench.took();
    assert_eq!(asked[0], Asked::Said("Stopped 2 containers".to_owned()));
    // One toast, as React's one toastError: the first failure's message,
    // and every container that failed in its Details.
    let Asked::Failed(None, error) = &asked[1] else {
        panic!("{asked:?}");
    };
    assert_eq!(error.message, refused().message);
    assert_eq!(
        error.detail.as_deref(),
        Some(
            "local/db: the container is being removed\n\
             prod/queue: the container is being removed (No such container: queue)"
        )
    );
    assert_eq!(
        asked[2..],
        [
            Asked::Invalidate(keys::host::containers("h1")),
            Asked::Invalidate(keys::host::containers("prod-id")),
        ]
    );
    // Nothing failed: nothing but the count. Nothing worked: no count.
    bench.told(removed(&results[..1].to_vec()));
    assert_eq!(bench.took()[0], Asked::Said("Removed container".to_owned()));
    bench.told(acted(ContainerAction::Kill, &results[1..2].to_vec()));
    assert!(matches!(bench.took()[0], Asked::Failed(..)));
}

#[test]
fn killing_is_confirmed_and_its_dialog_waits_for_the_answer() {
    let _english = in_language("en");
    let bench = bench();
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
    // Both failed, in one toast.
    let failures = |asked: &[Asked]| {
        asked
            .iter()
            .filter(|asked| matches!(asked, Asked::Failed(..)))
            .count()
    };
    assert_eq!(failures(&bench.took()), 1);
    // One went, one didn't: the dialog goes.
    let _ = actions.update(Message::Confirm(confirm::Message::Confirm), &bench.ctx());
    let _ = bench.took();
    let results = vec![(web, Ok(())), (db, Err(refused()))];
    let _ = actions.update(Message::Removed(results), &bench.ctx());
    assert!(!actions.has_dialog());
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
        "On \u{2066}local\u{2069}: web, job-0, job-1, job-2, job-3, job-4, job-5, and job-6…"
    );
    assert_eq!(lines[2], "On \u{2066}prod\u{2069}: api");
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
    // Another try, which the Engine takes: the list is read again and
    // the dialog closes.
    let again = actions.update(Message::RenameSubmit, &bench.ctx());
    drop(again);
    let web_id = target("web", "running").id;
    let _ = actions.update(Message::Renamed(web_id, Ok(())), &bench.ctx());
    assert!(!actions.has_dialog());
}

#[test]
fn a_rename_whose_dialog_was_closed_still_has_the_list_read_again() {
    let mut bench = bench();
    let mut actions = Actions::of_host();
    let web = target("web", "running");
    let _ = actions.update(Message::AskRename(web.clone()), &bench.ctx());
    let task = actions.update(Message::RenameSubmit, &bench.ctx());
    // Escape while the Engine is asked, then the dialog of another one.
    let _ = actions.update(Message::CloseDialog, &bench.ctx());
    let _ = actions.update(Message::AskRename(target("db", "running")), &bench.ctx());
    let answers = bench.run(task);
    // No Engine here: it fails, and a failure invalidates nothing.
    assert!(bench.took().is_empty());
    for message in answers {
        let _ = actions.update(message, &bench.ctx());
    }
    // The answer was the first dialog's: the second shows no failure.
    assert!(matches!(
        &actions.dialog,
        Some(Dialog::Rename { failure: None, .. })
    ));
    // What a rename that worked tells the shell needs no dialog.
    let renamed = Told::quiet().invalidating(keys::host::containers("h1"));
    bench.told(renamed);
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
            // Its Back returns to the list it was opened from.
            Asked::Open(Request::to(logs.clone()).with_param(CAME_FROM, "board")),
        ]
    );
    // On its own Host's list, the selection stays.
    let mut actions = Actions::of_host();
    let _ = actions.update(Message::Open(api, "logs"), &bench.ctx());
    assert_eq!(bench.took(), [Asked::Open(logs.into())]);
}

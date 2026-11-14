//! Edit, Duplicate and Commit against the fake Engine: what each reads,
//! what it says before it acts (every kind of loss), what it asks, and
//! where the window goes after.

use homeport_core::error::ErrorCode;
use iced_test::selector::Candidate;
use serde_json::json;

use super::super::{Actions, Message as Action, Target};
use super::{commit, edit, Message};
use crate::{
    data::keys,
    test_support::{
        engine::Engine,
        fixtures::container,
        snapshot::{assert_snapshot, in_language},
        window::window,
    },
    ui::{
        domain::containers::run::{self, form::Line},
        shell::{
            page::testing::{Asked, Bench},
            router::{Request, Route},
        },
        theme::Theme,
        widgets::{feedback::confirm, overlay},
    },
};

const SIZE: (f32, f32) = (828.0, 900.0);

/// Actions of a page whose Host is the fake Engine, and `web` on it.
fn bench() -> (Bench, Engine, Target) {
    let bench = Bench::at(Route::Containers);
    let (engine, host) = bench.app.runtime.block_on(async {
        let engine = Engine::start();
        let host = engine.host(&bench.app.state).await;
        (engine, host)
    });
    let mut web = container("web", None, "running", 0);
    "c1".clone_into(&mut web.id);
    (bench, engine, Target::of(&host, "fake", &web))
}

/// Sends `message` and what it started, to their end.
fn send(actions: &mut Actions, bench: &Bench, message: Action) {
    let runtime = bench.app.runtime.handle().clone();
    let _inside = runtime.enter();
    let task = actions.update(message, &bench.ctx());
    for answer in bench.run(task) {
        send(actions, bench, answer);
    }
}

/// What the shell was asked but the reads let go of as a form closed.
fn settled(bench: &mut Bench) -> Vec<Asked> {
    let took = bench.took().into_iter();
    took.filter(|asked| !matches!(asked, Asked::Unwatch(_)))
        .collect()
}

fn edit(actions: &mut Actions, bench: &Bench, message: edit::Message) {
    send(actions, bench, Action::Forms(Message::Edit(message)));
}

/// What the open dialog says, as drawn.
fn says(actions: &Actions, bench: &Bench, text: &str) -> bool {
    let ctx = bench.ctx();
    let dialog = actions.dialog(&ctx);
    window(SIZE, overlay::over(iced::widget::Space::new(), dialog))
        .find(text)
        .is_ok()
}

/// A container's inspect document: `running` or not, with an anonymous
/// volume, in a Stack.
fn document(running: bool, volume: bool, stack: bool) -> serde_json::Value {
    let labels = if stack {
        json!({ "com.docker.compose.project": "shop" })
    } else {
        json!({})
    };
    let mounts = if volume {
        json!([{ "Type": "volume", "Name": "a".repeat(64), "Destination": "/var/cache" }])
    } else {
        json!([])
    };
    json!({
        "Id": "c1",
        "Name": "/web",
        "Image": "sha256:1111",
        "State": { "Status": if running { "running" } else { "exited" }, "Running": running },
        "Config": { "Image": "nginx:1.27", "Labels": labels },
        "HostConfig": {},
        "Mounts": mounts,
    })
}

/// Edit opened on `web`, its settings read, and Apply pressed: the
/// confirmation, with what is lost.
fn confirming(actions: &mut Actions, bench: &Bench, web: &Target) {
    send(actions, bench, Action::AskEdit(web.clone()));
    edit(actions, bench, edit::Message::Submit);
}

#[test]
fn edit_says_each_kind_of_loss_and_turns_red_for_all_but_the_downtime() {
    let _english = in_language("en");
    let (bench, engine, web) = bench();
    // (running, anonymous volume, Stack, changed files), what is said,
    // and whether the confirmation is red.
    let cases = [
        (
            (false, true, false, 0),
            "The anonymous volume at \u{2066}/var/cache\u{2069}: the new container starts with an empty one.",
            true,
        ),
        (
            (false, false, false, 2),
            "2 files changed inside the container since it was created.",
            true,
        ),
        (
            (true, false, false, 0),
            "It stops while the two are swapped, usually for a few seconds.",
            false,
        ),
        (
            (false, false, true, 0),
            "It belongs to the \u{2066}shop\u{2069} Stack:",
            true,
        ),
    ];
    for ((running, volume, stack, changed), said, red) in cases {
        engine.inspects_as(document(running, volume, stack));
        engine.changed_paths(changed);
        let mut actions = Actions::of_host();
        confirming(&mut actions, &bench, &web);
        assert!(says(&actions, &bench, "Recreate web?"), "{said}");
        assert!(
            says(
                &actions,
                &bench,
                "Its name, networks and named volumes are kept, but not:"
            ),
            "{said}"
        );
        let ctx = bench.ctx();
        let dialog = actions.dialog(&ctx);
        let mut shown = window(SIZE, overlay::over(iced::widget::Space::new(), dialog));
        let found = shown
            .find(|candidate: Candidate<'_>| match candidate {
                Candidate::Text { content, .. } if content.contains(said) => Some(()),
                _ => None,
            })
            .is_ok();
        assert!(found, "{said}");
        drop(shown);
        let Some(super::Shown::Edit(open)) = actions.forms.shown.as_ref() else {
            panic!("Edit is open");
        };
        assert_eq!(open.is_red(), red, "{said}");
        // Only changed files offer a commit first.
        let commit_first = says(&actions, &bench, "Commit to an image first…");
        assert_eq!(commit_first, changed > 0, "{said}");
    }
}

#[test]
fn edit_with_nothing_lost_says_so() {
    let _english = in_language("en");
    let (bench, engine, web) = bench();
    engine.inspects_as(document(false, false, false));
    let mut actions = Actions::of_host();
    confirming(&mut actions, &bench, &web);
    assert!(says(
        &actions,
        &bench,
        "Nothing else is lost: its name, networks and named volumes are kept."
    ));
}

#[test]
fn edit_recreates_the_container_and_shows_the_new_one() {
    let _english = in_language("en");
    let (mut bench, engine, web) = bench();
    engine.inspects_as(document(true, false, false));
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskEdit(web.clone()));
    // The name is kept: its field can't be typed in.
    {
        let Some(super::Shown::Edit(open)) = actions.forms.shown.as_ref() else {
            panic!("Edit is open");
        };
        assert!(open.fields().lock_name);
        assert_eq!(open.fields().form.name, "web");
    }
    let typed = run::form::Message::Typed(Line::Memory, "256m".to_owned());
    edit(&mut actions, &bench, edit::Message::Form(typed));
    edit(&mut actions, &bench, edit::Message::Submit);
    let _ = bench.took();
    edit(
        &mut actions,
        &bench,
        edit::Message::Confirm(confirm::Message::Confirm),
    );
    assert!(!actions.has_dialog());
    let created = engine.created();
    assert_eq!(created.len(), 1);
    assert_eq!(created[0].body["HostConfig"]["Memory"], 256 * 1024 * 1024);
    assert_eq!(engine.asked("DELETE /containers/c1").len(), 1);
    let logs = Route::Container {
        id: "new-1".to_owned(),
        tab: Some("logs".to_owned()),
    };
    assert_eq!(
        settled(&mut bench),
        [
            Asked::Open(Request::to(logs)),
            Asked::Said("Container recreated".to_owned()),
            Asked::Invalidate(keys::host::containers(&web.host_id)),
        ]
    );
}

#[test]
fn commit_first_leaves_edit_for_commit() {
    let _english = in_language("en");
    let (bench, engine, web) = bench();
    engine.inspects_as(document(false, false, false));
    engine.changed_paths(1);
    let mut actions = Actions::of_host();
    confirming(&mut actions, &bench, &web);
    edit(&mut actions, &bench, edit::Message::CommitFirst);
    assert!(says(&actions, &bench, "Commit web to an image"));
}

#[test]
fn settings_that_could_not_be_read_are_said_with_retry() {
    let _english = in_language("en");
    let (bench, _engine, _web) = bench();
    let gone = Target {
        host_id: "nowhere".to_owned(),
        ..Target::of("nowhere", "none", &container("db", None, "running", 0))
    };
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskDuplicate(gone));
    assert!(says(&actions, &bench, "Duplicate db"));
    assert!(says(&actions, &bench, "Retry"));
    let Some(super::Shown::Loading { failure, .. }) = actions.forms.shown.as_ref() else {
        panic!("the read is said");
    };
    let code = failure.as_ref().map(|f| f.error.code);
    assert_eq!(code, Some(ErrorCode::HostNotConnected));
    send(&mut actions, &bench, Action::Forms(Message::Close));
    assert!(!actions.has_dialog());
}

#[test]
fn a_copy_is_named_after_the_free_name_and_runs() {
    let _english = in_language("en");
    let (mut bench, engine, web) = bench();
    engine.inspects_as(document(true, false, false));
    let taken = vec![
        container("web", None, "running", 0),
        container("web-copy", None, "running", 0),
    ];
    bench.data.set(&keys::host::containers(&web.host_id), taken);
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskDuplicate(web.clone()));
    assert!(says(&actions, &bench, "Duplicate web"));
    let Some(super::Shown::Duplicate(dialog)) = actions.forms.shown.as_ref() else {
        panic!("the copy's Run dialog is open");
    };
    assert_eq!(dialog.fields.form.name, "web-copy-2");
    let submit = Message::Run(run::dialog::Message::Submit);
    send(&mut actions, &bench, Action::Forms(submit));
    assert!(!actions.has_dialog());
    let created = engine.created();
    assert_eq!(created[0].name.as_deref(), Some("web-copy-2"));
}

#[test]
fn commit_keeps_the_container_as_an_image_and_the_images_are_read_again() {
    let _english = in_language("en");
    let (mut bench, engine, web) = bench();
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskCommit(web.clone()));
    let watched = bench.watched();
    assert!(watched.contains(&keys::host::images(&web.host_id)));
    let named = commit::Message::Reference("web:backup".to_owned());
    send(&mut actions, &bench, Action::Forms(Message::Commit(named)));
    let paused = commit::Message::Pause(false);
    send(&mut actions, &bench, Action::Forms(Message::Commit(paused)));
    let submit = commit::Message::Submit;
    send(&mut actions, &bench, Action::Forms(Message::Commit(submit)));
    assert!(!actions.has_dialog());
    let commits = engine.commits();
    assert!(commits[0].contains("repo=web") && commits[0].contains("pause=false"));
    assert_eq!(
        settled(&mut bench),
        [
            Asked::Said("Committed to \u{2066}web:backup\u{2069}".to_owned()),
            Asked::Going("Open images".to_owned(), Request::to(Route::Images)),
            Asked::Invalidate(keys::host::images(&web.host_id)),
        ]
    );
}

#[test]
fn the_recreate_confirmation_and_the_commit_dialog_look_as_react_s() {
    let (bench, engine, web) = bench();
    engine.inspects_as(document(true, true, true));
    engine.changed_paths(3);
    let mut actions = Actions::of_host();
    confirming(&mut actions, &bench, &web);
    for language in ["en", "ar"] {
        let ctx = bench.ctx();
        let name = format!("container-recreate-{language}");
        assert_snapshot(&name, language, &Theme::Light, (828.0, 560.0), || {
            overlay::over(iced::widget::Space::new(), actions.dialog(&ctx))
        });
        // A line of what is lost hugs its words: one told to fill is drawn
        // out of place in Arabic (where this one is shorter than the room).
        let _language = in_language(language);
        let short = edit::loss_text(&homeport_engine::containers::recreate::Loss::Downtime);
        let width = |view: crate::ui::theme::Element<'_, Action>| {
            let found = window(SIZE, view).find(short.as_str());
            found.map(|t| t.bounds().width).unwrap_or_default()
        };
        let line = width(overlay::over(
            iced::widget::Space::new(),
            actions.dialog(&ctx),
        ));
        let words = width(crate::ui::widgets::paragraph(short.clone()).into());
        let hugs = language != "ar" || (line - words).abs() < 1.0;
        assert!(hugs, "{language}: {line} {words}");
    }
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskCommit(web));
    if let Some(super::Shown::Commit(open)) = actions.forms.shown.as_mut() {
        // The same name whatever the day the test runs.
        "web:2026-11-04".clone_into(&mut open.form.reference);
    }
    let ctx = bench.ctx();
    assert_snapshot(
        "container-commit-en",
        "en",
        &Theme::Light,
        (828.0, 700.0),
        || overlay::over(iced::widget::Space::new(), actions.dialog(&ctx)),
    );
}

#[test]
fn the_loss_confirmation_starts_on_cancel_whenever_the_losses_come() {
    use crate::test_support::stage::Stage;
    let _english = in_language("en");
    let (bench, engine, web) = bench();
    engine.inspects_as(document(true, false, false));
    engine.changed_paths(2);
    // The losses came before the confirmation was first drawn.
    let mut actions = Actions::of_host();
    confirming(&mut actions, &bench, &web);
    let mut stage = Stage::new(SIZE);
    let ctx = bench.ctx();
    stage.frame(
        overlay::over(iced::widget::Space::new(), actions.dialog(&ctx)),
        |frame| {
            assert!(frame.find("Commit to an image first…").is_some());
            assert!(
                frame.is_focused("Cancel"),
                "Enter must not throw the edit away"
            );
        },
    );
    // And after: drawn with its spinner first.
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskEdit(web.clone()));
    let runtime = bench.app.runtime.handle().clone();
    let _inside = runtime.enter();
    let read = actions.update(
        Action::Forms(Message::Edit(edit::Message::Submit)),
        &bench.ctx(),
    );
    let mut stage = Stage::new(SIZE);
    let ctx = bench.ctx();
    stage.frame(
        overlay::over(iced::widget::Space::new(), actions.dialog(&ctx)),
        |frame| assert!(frame.is_focused("Cancel")),
    );
    for answer in bench.run(read) {
        send(&mut actions, &bench, answer);
    }
    let ctx = bench.ctx();
    stage.frame(
        overlay::over(iced::widget::Space::new(), actions.dialog(&ctx)),
        |frame| {
            assert!(frame.find("Commit to an image first…").is_some());
            assert!(frame.is_focused("Cancel"));
        },
    );
}

#[test]
fn an_older_edit_s_losses_never_show_on_a_newer_confirmation() {
    let _english = in_language("en");
    let (bench, engine, web) = bench();
    engine.inspects_as(document(true, false, false));
    engine.changed_paths(2);
    let runtime = bench.app.runtime.handle().clone();
    let _inside = runtime.enter();
    let submit = || Action::Forms(Message::Edit(edit::Message::Submit));
    // The first Edit asks what is lost, and is closed before it hears.
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskEdit(web.clone()));
    let first = bench.run(actions.update(submit(), &bench.ctx()));
    edit(&mut actions, &bench, edit::Message::Close);
    // A second one asks again; the first's answer comes now.
    send(&mut actions, &bench, Action::AskEdit(web.clone()));
    let second = actions.update(submit(), &bench.ctx());
    for answer in first {
        send(&mut actions, &bench, answer);
    }
    assert!(
        !says_part(&actions, &bench, "changed inside the container"),
        "the first Edit's answer"
    );
    for answer in bench.run(second) {
        send(&mut actions, &bench, answer);
    }
    assert!(says_part(&actions, &bench, "changed inside the container"));
}

#[test]
fn edit_says_which_login_a_pull_of_the_image_would_use() {
    let _english = in_language("en");
    let (mut bench, engine, web) = bench();
    engine.inspects_as(document(true, false, false));
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskEdit(web.clone()));
    let runtime = bench.app.runtime.handle().clone();
    {
        let _inside = runtime.enter();
        bench.read_watched();
    }
    // The Host has other images than nginx:1.27: it would be pulled.
    let other = homeport_engine::images::Image {
        id: "sha256:b".to_owned(),
        tags: vec!["redis:7".to_owned()],
        digests: vec![],
        created: 0,
        size: 0,
        containers: 0,
        labels: Default::default(),
    };
    bench
        .data
        .set(&keys::host::images(&web.host_id), vec![other]);
    assert!(says(&actions, &bench, "Pulled anonymously"));
}

#[test]
fn a_form_that_closes_lets_go_of_what_it_read() {
    let (mut bench, engine, web) = bench();
    engine.inspects_as(document(true, false, false));
    let host = web.host_id.clone();
    let unwatched = |bench: &mut Bench| -> Vec<crate::data::Key> {
        bench
            .took()
            .into_iter()
            .filter_map(|asked| match asked {
                Asked::Unwatch(key) => Some(key),
                _ => None,
            })
            .collect()
    };
    let login = keys::stacks::registry_login_for("nginx:1.27");
    // Edit.
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskEdit(web.clone()));
    let _ = bench.took();
    edit(&mut actions, &bench, edit::Message::Close);
    let gone = unwatched(&mut bench);
    for key in [
        keys::host::images(&host),
        keys::host::networks(&host),
        login.clone(),
    ] {
        assert!(gone.contains(&key), "{key:?} in {gone:?}");
    }
    // Duplicate.
    send(&mut actions, &bench, Action::AskDuplicate(web.clone()));
    let _ = bench.took();
    send(
        &mut actions,
        &bench,
        Action::Forms(Message::Run(run::dialog::Message::Close)),
    );
    let gone = unwatched(&mut bench);
    for key in [
        keys::host::images(&host),
        keys::host::networks(&host),
        login,
    ] {
        assert!(gone.contains(&key), "{key:?} in {gone:?}");
    }
    // Commit.
    send(&mut actions, &bench, Action::AskCommit(web));
    let _ = bench.took();
    send(
        &mut actions,
        &bench,
        Action::Forms(Message::Commit(commit::Message::Close)),
    );
    assert_eq!(unwatched(&mut bench), [keys::host::images(&host)]);
}

#[test]
fn a_recreate_the_engine_refuses_is_said_in_the_form_which_stays() {
    let _english = in_language("en");
    let (mut bench, engine, web) = bench();
    engine.inspects_as(document(true, false, false));
    engine.refuse("POST /containers/c1/stop", "cannot stop container");
    let mut actions = Actions::of_host();
    confirming(&mut actions, &bench, &web);
    let _ = bench.took();
    edit(
        &mut actions,
        &bench,
        edit::Message::Confirm(confirm::Message::Confirm),
    );
    // The confirmation closed, the form is there with why.
    assert!(says(&actions, &bench, "Edit web"));
    assert!(!says(&actions, &bench, "Recreate web?"));
    assert!(says(&actions, &bench, "Details"));
    // Said in the form, not in a toast; nothing is read again.
    assert!(settled(&mut bench).is_empty());
}

#[test]
fn a_copy_says_the_host_ports_its_original_publishes_too() {
    let _english = in_language("en");
    let (bench, engine, web) = bench();
    let mut document = document(true, false, false);
    document["HostConfig"] = json!({
        "PortBindings": { "80/tcp": [{ "HostIp": "", "HostPort": "8080" }] }
    });
    engine.inspects_as(document);
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskDuplicate(web));
    let notice = "web already publishes host port 8080.";
    assert!(says_part(&actions, &bench, notice), "the notice shows");
    let freed = run::form::Message::Typed(Line::HostPort(0), String::new());
    let freed = Message::Run(run::dialog::Message::Form(freed));
    send(&mut actions, &bench, Action::Forms(freed));
    assert!(
        !says_part(&actions, &bench, notice),
        "and goes once the port is freed"
    );
}

#[test]
fn a_commit_the_engine_refuses_is_said_in_the_dialog_which_stays() {
    let _english = in_language("en");
    let (mut bench, engine, web) = bench();
    engine.refuse("POST /commit", "invalid reference format");
    let mut actions = Actions::of_host();
    send(&mut actions, &bench, Action::AskCommit(web));
    let _ = bench.took();
    send(
        &mut actions,
        &bench,
        Action::Forms(Message::Commit(commit::Message::Submit)),
    );
    assert!(actions.has_dialog());
    assert!(says(&actions, &bench, "Details"));
    assert!(
        says(&actions, &bench, "Commit"),
        "Commit can be pressed again"
    );
    assert!(
        settled(&mut bench).is_empty(),
        "no toast, nothing read again"
    );
}

/// Whether some text of the open dialog holds `part`.
fn says_part(actions: &Actions, bench: &Bench, part: &str) -> bool {
    let ctx = bench.ctx();
    let dialog = actions.dialog(&ctx);
    window(SIZE, overlay::over(iced::widget::Space::new(), dialog))
        .find(|candidate: Candidate<'_>| match candidate {
            Candidate::Text { content, .. } if content.contains(part) => Some(()),
            _ => None,
        })
        .is_ok()
}

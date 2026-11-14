//! The Run dialog: what its fields change, what it asks the Engine, what
//! it says when the Engine refuses, and how it looks (`drawn`). `parity`
//! holds the dialog to building the request the Tauri app built from the
//! same form.

mod drawn;
mod parity;

use homeport_core::error::ErrorCode;
use homeport_engine::{containers::run::RestartPolicy, networks::NetworkView, podman::pods::Pod};

use super::{
    dialog::{Dialog, Message, Step},
    form::{Caps, Choice, Hint, Line, Message as Field, Rows, Switch},
    model::advanced::Field as Checked,
};
use crate::{
    data::keys,
    test_support::{engine::Engine, snapshot::in_language, window::window},
    ui::{
        shell::{
            page::testing::{Asked, Bench},
            router::Route,
        },
        widgets::{
            form::{select, suggest},
            overlay,
        },
    },
};

/// A page's bench whose Host is the fake Engine, and its id.
pub(super) fn bench() -> (Bench, Engine, String) {
    let bench = Bench::at(Route::Containers);
    let (engine, host) = bench.app.runtime.block_on(async {
        let engine = Engine::start();
        let host = engine.host(&bench.app.state).await;
        (engine, host)
    });
    (bench, engine, host)
}

/// A window tall enough for the whole dialog.
const SIZE: (f32, f32) = (828.0, 1400.0);

/// The dialog as the shell lays it over a page.
pub(super) fn shown<'a>(
    dialog: &'a Dialog,
    ctx: &crate::ui::shell::Context<'a>,
) -> crate::ui::theme::Element<'a, Message> {
    overlay::over(iced::widget::Space::new(), Some(dialog.view(ctx)))
}

/// Sends `message` and what it started, to their end; the last step
/// that was not [`Step::Stay`].
pub(super) fn send(dialog: &mut Dialog, bench: &Bench, message: Message) -> Step {
    let runtime = bench.app.runtime.handle().clone();
    let _inside = runtime.enter();
    let (task, mut step) = dialog.update(message, &bench.ctx());
    for answer in bench.run(task) {
        let next = send(dialog, bench, answer);
        if next != Step::Stay {
            step = next;
        }
    }
    step
}

/// Types `value` into `line`.
pub(super) fn type_in(dialog: &mut Dialog, bench: &Bench, line: Line, value: &str) {
    let typed = Message::Form(Field::Typed(line, value.to_owned()));
    assert_eq!(send(dialog, bench, typed), Step::Stay);
}

/// Picks the option of `choice` that says `label`.
pub(super) fn pick(dialog: &mut Dialog, bench: &Bench, choice: Choice, label: &str) {
    let lists = dialog.fields.lists(&bench.ctx());
    let (labels, _) = dialog.fields.options(choice, &lists);
    let index = labels
        .iter()
        .position(|option| option == label)
        .unwrap_or_else(|| panic!("{label} among {labels:?}"));
    let chosen = Message::Form(Field::Choose(choice, select::Message::Choose(index)));
    send(dialog, bench, chosen);
}

/// The Host's networks, `bridge` among them, as the data layer has them.
fn networks(bench: &mut Bench, host: &str) {
    let network = |name: &str| NetworkView {
        id: name.to_owned(),
        name: name.to_owned(),
        driver: "bridge".to_owned(),
        scope: "local".to_owned(),
        internal: false,
        attachable: false,
        subnets: vec![],
        created: None,
        labels: Default::default(),
        stack: None,
        built_in: name == "bridge" || name == "host",
        containers: vec![],
    };
    let list = vec![network("bridge"), network("host"), network("proxy")];
    bench.data.set(&keys::host::networks(host), list);
}

#[test]
fn run_waits_for_an_image_and_for_what_it_can_read() {
    let _english = in_language("en");
    let (bench, _engine, host) = bench();
    let mut dialog = Dialog::run(&host, "", "", false, &bench.ctx());
    assert!(!dialog.fields.is_ready(), "no image");
    type_in(&mut dialog, &bench, Line::Image, "nginx");
    assert!(dialog.fields.is_ready());
    assert!(!dialog.fields.shows_advanced());
    // A limit it can't read opens the Advanced area, says why, and
    // keeps Run from being pressed.
    type_in(&mut dialog, &bench, Line::Cpus, "half");
    assert!(!dialog.fields.is_ready() && dialog.fields.shows_advanced());
    assert!(dialog.fields.errors().has(Checked::Cpus));
    let ctx = bench.ctx();
    let mut drawn = window(SIZE, shown(&dialog, &ctx));
    assert!(drawn.find("Write a number of CPUs like 0.5 or 2.").is_ok());
    drop(drawn);
    // Run can't be pressed: sending it asks nothing.
    assert_eq!(send(&mut dialog, &bench, Message::Submit), Step::Stay);
}

#[test]
fn a_container_removed_when_it_exits_is_never_restarted() {
    let _english = in_language("en");
    let (bench, _engine, host) = bench();
    let mut dialog = Dialog::run(&host, "nginx", "", false, &bench.ctx());
    pick(&mut dialog, &bench, Choice::Restart, "Always");
    assert_eq!(dialog.fields.form.restart, RestartPolicy::Always);
    let on = Message::Form(Field::Switch(Switch::AutoRemove, true));
    send(&mut dialog, &bench, on);
    assert_eq!(dialog.fields.form.restart, RestartPolicy::No);
    assert!(dialog.fields.form.auto_remove);
}

#[test]
fn rows_are_added_typed_in_and_removed() {
    let (bench, _engine, host) = bench();
    let mut dialog = Dialog::run(&host, "nginx", "", false, &bench.ctx());
    for _ in 0..2 {
        send(&mut dialog, &bench, Message::Form(Field::Add(Rows::Ports)));
    }
    type_in(&mut dialog, &bench, Line::ContainerPort(1), "53");
    pick(&mut dialog, &bench, Choice::Protocol(1), "udp");
    send(
        &mut dialog,
        &bench,
        Message::Form(Field::Remove(Rows::Ports, 0)),
    );
    let ports = &dialog.fields.form.ports;
    assert_eq!(ports.len(), 1);
    assert_eq!(
        (ports[0].container_port.as_str(), ports[0].protocol.as_str()),
        ("53", "udp")
    );
    // A port that is none is said under its row, and Run waits.
    type_in(&mut dialog, &bench, Line::HostPort(0), "dns");
    assert_eq!(dialog.fields.errors().ports, [0]);
    assert!(!dialog.fields.is_ready());
}

#[test]
fn capabilities_are_typed_a_name_at_a_time() {
    let (bench, _engine, host) = bench();
    let mut dialog = Dialog::run(&host, "nginx", "", false, &bench.ctx());
    let add = Line::Capability(Caps::Add);
    type_in(&mut dialog, &bench, add, "cap_net_admin");
    send(
        &mut dialog,
        &bench,
        Message::Form(Field::AddCapability(Caps::Add)),
    );
    // A comma ends a name too; one already there is not added again.
    type_in(&mut dialog, &bench, add, "sys_time,");
    type_in(&mut dialog, &bench, add, "NET_ADMIN,");
    assert_eq!(
        dialog.fields.form.advanced.security.cap_add,
        ["NET_ADMIN", "SYS_TIME"]
    );
    assert!(dialog.fields.drafts[0].is_empty());
    // Backspace in the empty field takes the last away; not while typing.
    type_in(&mut dialog, &bench, add, "x");
    send(
        &mut dialog,
        &bench,
        Message::Form(Field::RemoveLastCapability(Caps::Add)),
    );
    assert_eq!(dialog.fields.form.advanced.security.cap_add.len(), 2);
    type_in(&mut dialog, &bench, add, "");
    send(
        &mut dialog,
        &bench,
        Message::Form(Field::RemoveLastCapability(Caps::Add)),
    );
    send(
        &mut dialog,
        &bench,
        Message::Form(Field::RemoveCapability(Caps::Add, 0)),
    );
    assert!(dialog.fields.form.advanced.security.cap_add.is_empty());
}

#[test]
fn the_networks_offered_are_the_host_s_but_bridge_and_the_one_named() {
    let _english = in_language("en");
    let (mut bench, _engine, host) = bench();
    networks(&mut bench, &host);
    let mut dialog = Dialog::run(&host, "nginx", "", false, &bench.ctx());
    let lists = dialog.fields.lists(&bench.ctx());
    let (labels, chosen) = dialog.fields.options(Choice::Network, &lists);
    assert_eq!(labels, ["Default (bridge)", "host", "proxy"]);
    assert_eq!(chosen, Some(0));
    pick(&mut dialog, &bench, Choice::Network, "proxy");
    assert_eq!(dialog.fields.form.network, "proxy");
    pick(&mut dialog, &bench, Choice::Network, "Default (bridge)");
    assert_eq!(dialog.fields.form.network, "");
    // A network the Host no longer has still shows as chosen.
    dialog.fields.form.network = "gone".to_owned();
    let (labels, chosen) = dialog.fields.options(Choice::Network, &lists);
    assert_eq!(labels.last().map(String::as_str), Some("gone"));
    assert_eq!(chosen, Some(3));
}

#[test]
fn a_run_starts_the_container_and_the_shell_says_so() {
    let _english = in_language("en");
    let (mut bench, engine, host) = bench();
    let mut dialog = Dialog::run(&host, "", "", false, &bench.ctx());
    let watched = bench.watched();
    assert!(watched.contains(&keys::host::images(&host)), "{watched:?}");
    type_in(&mut dialog, &bench, Line::Image, "nginx");
    type_in(&mut dialog, &bench, Line::Name, "web");
    let _ = bench.took();
    let step = send(&mut dialog, &bench, Message::Submit);
    assert_eq!(step, Step::Started("new-1".to_owned()));
    assert_eq!(engine.created().len(), 1);
    assert_eq!(engine.asked("/containers/new-1/start").len(), 1);
    assert_eq!(
        bench.took(),
        [
            Asked::Said("Container started".to_owned()),
            Asked::Invalidate(keys::host::containers(&host)),
        ]
    );
}

#[test]
fn a_refused_run_is_said_in_the_dialog_which_stays_for_another_try() {
    let _english = in_language("en");
    let (mut bench, engine, host) = bench();
    engine.refuse_create("conflict: the name web is already in use");
    let mut dialog = Dialog::run(&host, "nginx", "", false, &bench.ctx());
    let _ = bench.took();
    assert_eq!(send(&mut dialog, &bench, Message::Submit), Step::Stay);
    // Said in place, not in a toast (`meta: { silent: true }`).
    assert!(bench.took().is_empty());
    let ctx = bench.ctx();
    let mut drawn = window(SIZE, shown(&dialog, &ctx));
    assert!(drawn.find("Details").is_ok());
    drop(drawn);
    assert!(dialog.fields.is_ready(), "Run can be pressed again");
    let closed = send(&mut dialog, &bench, Message::Close);
    assert_eq!(closed, Step::Closed);
}

#[test]
fn a_container_run_into_a_pod_leaves_it_its_ports_and_network() {
    let _english = in_language("en");
    let (mut bench, engine, host) = bench();
    networks(&mut bench, &host);
    let shop = Pod {
        id: "p1".to_owned(),
        name: "shop".to_owned(),
        status: "running".to_owned(),
        created: String::new(),
        infra_id: None,
        labels: Default::default(),
        networks: vec![],
        containers: vec![],
    };
    bench.data.set(&keys::host::pods(&host), vec![shop]);
    let mut dialog = Dialog::run(&host, "nginx", "", true, &bench.ctx());
    send(&mut dialog, &bench, Message::Form(Field::Add(Rows::Ports)));
    type_in(&mut dialog, &bench, Line::ContainerPort(0), "80");
    pick(&mut dialog, &bench, Choice::Network, "proxy");
    pick(&mut dialog, &bench, Choice::Pod, "shop");
    assert!(dialog.fields.in_pod());
    let spec = dialog.fields.form.to_spec();
    assert!(spec.ports.is_empty() && spec.network.is_none());
    assert_eq!(dialog.fields.form.pod().as_deref(), Some("shop"));
    let ctx = bench.ctx();
    let mut drawn = window(SIZE, shown(&dialog, &ctx));
    assert!(drawn.find("Add port").is_err(), "the ports are the pod's");
    drop(drawn);
    // The fake is a Docker Engine: the pod is asked of Podman, which it
    // is not, and nothing is created.
    send(&mut dialog, &bench, Message::Submit);
    assert!(engine.created().is_empty());
    let refused = dialog.failure().map(|failure| failure.error.code);
    assert_eq!(refused, Some(ErrorCode::EngineUnsupported));
}

fn image(tags: &[&str]) -> homeport_engine::images::Image {
    homeport_engine::images::Image {
        id: format!("sha256:{}", tags.join("-")),
        tags: tags.iter().map(|t| (*t).to_owned()).collect(),
        digests: vec![],
        created: 0,
        size: 0,
        containers: 0,
        labels: Default::default(),
    }
}

fn hinted(dialog: &mut Dialog, bench: &Bench, hint: Hint, message: suggest::Message) -> Step {
    send(dialog, bench, Message::Form(Field::Suggest(hint, message)))
}

#[test]
fn the_image_is_suggested_from_the_host_s_tags_and_typed_freely() {
    let (mut bench, engine, host) = bench();
    let images = vec![
        image(&["nginx:1.27", "nginx:latest"]),
        image(&["postgres:16"]),
    ];
    bench.data.set(&keys::host::images(&host), images);
    let mut dialog = Dialog::run(&host, "", "", false, &bench.ctx());
    let lists = dialog.fields.lists(&bench.ctx());
    let offered = dialog.fields.suggestions(Hint::Image, &lists);
    assert_eq!(offered, ["nginx:1.27", "nginx:latest", "postgres:16"]);
    hinted(
        &mut dialog,
        &bench,
        Hint::Image,
        suggest::Message::Typed("ngi".to_owned()),
    );
    assert_eq!(dialog.fields.form.image, "ngi");
    assert!(dialog.fields.hint(Hint::Image).is_open());
    hinted(&mut dialog, &bench, Hint::Image, suggest::Message::Step(1));
    hinted(&mut dialog, &bench, Hint::Image, suggest::Message::Step(1));
    hinted(&mut dialog, &bench, Hint::Image, suggest::Message::Submit);
    assert_eq!(dialog.fields.form.image, "nginx:latest");
    assert!(
        engine.created().is_empty(),
        "taking a suggestion runs nothing"
    );
    // Enter with nothing highlighted runs, as the form's Enter.
    let step = hinted(&mut dialog, &bench, Hint::Image, suggest::Message::Submit);
    assert_eq!(step, Step::Started("new-1".to_owned()));
}

#[test]
fn the_capabilities_not_chosen_and_the_usual_log_drivers_are_suggested() {
    let (bench, _engine, host) = bench();
    let mut dialog = Dialog::run(&host, "nginx", "", false, &bench.ctx());
    let lists = dialog.fields.lists(&bench.ctx());
    let add = Hint::Capability(Caps::Add);
    let all = dialog.fields.suggestions(add, &lists);
    assert_eq!(
        all.len(),
        homeport_engine::containers::run::CAPABILITIES.len()
    );
    hinted(
        &mut dialog,
        &bench,
        add,
        suggest::Message::Typed("net_ad".to_owned()),
    );
    hinted(&mut dialog, &bench, add, suggest::Message::Step(1));
    hinted(&mut dialog, &bench, add, suggest::Message::Submit);
    assert_eq!(dialog.fields.drafts[0], "NET_ADMIN");
    // Enter adds what the field holds, as `TokenInput` did.
    hinted(&mut dialog, &bench, add, suggest::Message::Submit);
    assert_eq!(dialog.fields.form.advanced.security.cap_add, ["NET_ADMIN"]);
    let lists = dialog.fields.lists(&bench.ctx());
    let left = dialog.fields.suggestions(add, &lists);
    assert!(!left.contains(&"NET_ADMIN".to_owned()) && left.len() == all.len() - 1);
    let drivers = dialog.fields.suggestions(Hint::LogDriver, &lists);
    assert_eq!(
        drivers,
        ["json-file", "local", "journald", "syslog", "none"]
    );
    hinted(
        &mut dialog,
        &bench,
        Hint::LogDriver,
        suggest::Message::Typed("jou".to_owned()),
    );
    hinted(
        &mut dialog,
        &bench,
        Hint::LogDriver,
        suggest::Message::Pick("journald".to_owned()),
    );
    assert_eq!(dialog.fields.form.advanced.logging.driver, "journald");
}

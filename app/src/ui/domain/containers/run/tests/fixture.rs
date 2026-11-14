//! The container the Run dialog creates is the one the Tauri app created
//! from the same form, held against React itself:
//! `app/tests/fixtures/run-specs.json` is what React's `toRunSpec` (and
//! `fromRunSpec`, `copyName` for Duplicate and Edit) made of a table of
//! forms, written and checked by `runSpecs.fixture.test.ts` (`bun run
//! check` fails when React's output drifts from it). Each form is filled
//! into the iced dialog as a user would (typed, pasted, picked, switched,
//! a capability left in its field), and:
//!
//! - the request the dialog makes is React's, field for field;
//! - Run is pressed, and the create request the fake Engine receives is
//!   the crate's `create_body` of React's request: what the Tauri command
//!   `containers_run` sent, as it handed React's request to `run::run`;
//! - a form React sent with a port that is no number (Tauri refused it,
//!   or published on any port) is refused in the dialog and creates
//!   nothing;
//! - a container run into a pod is asked of that pod;
//! - Edit's form, Apply pressed and the loss confirmed, has the Engine
//!   create the replacement Tauri's `containers_recreate` created from
//!   React's request, the original's identity kept.
//!
//! What this can't show without an Engine: that the Engine makes of the
//! two requests the same container (`docker inspect`). The request is the
//! whole of what it is given.

use std::sync::Arc;

use iced::widget::text_editor::{Action, Edit};
use serde::Deserialize;

use homeport_engine::containers::run::{create_body, Base, RestartPolicy, RunSpec};

use super::{bench, networks, pick, send, type_in};
use crate::{
    data::keys,
    test_support::{engine::IMAGE_INSPECTED, snapshot::in_language},
    ui::{
        domain::containers::{
            actions::{
                forms::edit::{Edit as EditForm, Message as EditMessage, Step as EditStep},
                Target,
            },
            run::{
                dialog::{Dialog, Message, Step},
                form::{Caps, Choice, Line, Message as Field, Rows, Switch, Text},
                model::{
                    advanced::{GpuMode, HealthMode},
                    copy_name, restart_label, RunForm,
                },
            },
        },
        shell::page::testing::Bench,
        widgets::feedback::confirm,
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Case {
    name: String,
    via: String,
    form: Option<Form>,
    from: Option<RunSpec>,
    taken: Option<Vec<String>>,
    container: Option<String>,
    typed_not_added: Option<NotAdded>,
    spec: Option<RunSpec>,
    pod: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NotAdded {
    cap_add: Option<String>,
    cap_drop: Option<String>,
}

/// React's `RunForm`, as the fixture writes it.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Form {
    image: String,
    name: String,
    command: String,
    env: String,
    ports: Vec<Port>,
    mounts: Vec<Mount>,
    network: String,
    restart: RestartPolicy,
    auto_remove: bool,
    advanced: Advanced,
    pod: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Port {
    host_port: String,
    container_port: String,
    protocol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Mount {
    source: String,
    target: String,
    read_only: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Advanced {
    resources: Resources,
    labels: String,
    security: Security,
    healthcheck: Health,
    identity: Identity,
    logging: Logging,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Resources {
    cpus: String,
    cpu_shares: String,
    memory: String,
    memory_reservation: String,
    memory_swap: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Security {
    cap_add: Vec<String>,
    cap_drop: Vec<String>,
    privileged: bool,
    devices: Vec<Device>,
    gpus: Gpus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Device {
    host_path: String,
    container_path: String,
    permissions: String,
}

#[derive(Debug, Deserialize)]
struct Gpus {
    mode: String,
    count: String,
    ids: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Health {
    mode: String,
    command: String,
    interval: String,
    timeout: String,
    retries: String,
    start_period: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identity {
    hostname: String,
    domainname: String,
    dns: String,
    dns_search: String,
    extra_hosts: String,
}

#[derive(Debug, Deserialize)]
struct Logging {
    driver: String,
    options: String,
}

fn cases() -> Vec<Case> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/run-specs.json");
    let text = std::fs::read_to_string(path).expect("the fixture React wrote");
    serde_json::from_str(&text).expect("the fixture reads")
}

fn field(dialog: &mut Dialog, bench: &Bench, message: Field) {
    send(dialog, bench, Message::Form(message));
}

/// Types `value` in `line` when there is something to type.
fn typed(dialog: &mut Dialog, bench: &Bench, line: Line, value: &str) {
    if !value.is_empty() {
        type_in(dialog, bench, line, value);
    }
}

fn paste(dialog: &mut Dialog, bench: &Bench, which: Text, text: &str) {
    if !text.is_empty() {
        let pasted = Action::Edit(Edit::Paste(Arc::new(text.to_owned())));
        field(dialog, bench, Field::Text(which, pasted));
    }
}

/// Fills the dialog with React's form, as a user would.
fn fill(d: &mut Dialog, b: &Bench, form: &Form, not_added: &NotAdded) {
    typed(d, b, Line::Image, &form.image);
    typed(d, b, Line::Name, &form.name);
    typed(d, b, Line::Command, &form.command);
    if !form.pod.is_empty() {
        pick(d, b, Choice::Pod, &form.pod);
    }
    for (i, port) in form.ports.iter().enumerate() {
        field(d, b, Field::Add(Rows::Ports));
        typed(d, b, Line::HostPort(i), &port.host_port);
        typed(d, b, Line::ContainerPort(i), &port.container_port);
        pick(d, b, Choice::Protocol(i), &port.protocol);
    }
    for (i, mount) in form.mounts.iter().enumerate() {
        field(d, b, Field::Add(Rows::Mounts));
        typed(d, b, Line::Source(i), &mount.source);
        typed(d, b, Line::Target(i), &mount.target);
        field(d, b, Field::Switch(Switch::ReadOnly(i), mount.read_only));
    }
    paste(d, b, Text::Env, &form.env);
    if !form.network.is_empty() {
        pick(d, b, Choice::Network, &form.network);
    }
    pick(d, b, Choice::Restart, &restart_label(form.restart));
    field(d, b, Field::Switch(Switch::AutoRemove, form.auto_remove));
    let a = &form.advanced;
    let r = &a.resources;
    typed(d, b, Line::Cpus, &r.cpus);
    typed(d, b, Line::CpuShares, &r.cpu_shares);
    typed(d, b, Line::Memory, &r.memory);
    typed(d, b, Line::MemoryReservation, &r.memory_reservation);
    typed(d, b, Line::MemorySwap, &r.memory_swap);
    paste(d, b, Text::Labels, &a.labels);
    let s = &a.security;
    for (caps, list, left) in [
        (Caps::Add, &s.cap_add, &not_added.cap_add),
        (Caps::Drop, &s.cap_drop, &not_added.cap_drop),
    ] {
        for name in list {
            type_in(d, b, Line::Capability(caps), name);
            field(d, b, Field::AddCapability(caps));
        }
        if let Some(left) = left {
            // Left in its field as Run is pressed.
            type_in(d, b, Line::Capability(caps), left);
        }
    }
    field(d, b, Field::Switch(Switch::Privileged, s.privileged));
    for (i, device) in s.devices.iter().enumerate() {
        field(d, b, Field::Add(Rows::Devices));
        typed(d, b, Line::HostDevice(i), &device.host_path);
        typed(d, b, Line::ContainerDevice(i), &device.container_path);
        // A new row says `rwm`: what React's row held is typed over it.
        type_in(d, b, Line::Permissions(i), &device.permissions);
    }
    let mode = match s.gpus.mode.as_str() {
        "all" => GpuMode::All,
        "count" => GpuMode::Count,
        "ids" => GpuMode::Ids,
        _ => GpuMode::None,
    };
    pick(d, b, Choice::Gpus, &mode.label());
    type_in(d, b, Line::GpuCount, &s.gpus.count);
    typed(d, b, Line::GpuIds, &s.gpus.ids);
    let h = &a.healthcheck;
    let mode = match h.mode.as_str() {
        "custom" => HealthMode::Custom,
        "disabled" => HealthMode::Disabled,
        _ => HealthMode::Image,
    };
    pick(d, b, Choice::Health, &mode.label());
    typed(d, b, Line::HealthCommand, &h.command);
    typed(d, b, Line::Interval, &h.interval);
    typed(d, b, Line::Timeout, &h.timeout);
    typed(d, b, Line::Retries, &h.retries);
    typed(d, b, Line::StartPeriod, &h.start_period);
    let i = &a.identity;
    typed(d, b, Line::Hostname, &i.hostname);
    typed(d, b, Line::Domainname, &i.domainname);
    paste(d, b, Text::Dns, &i.dns);
    paste(d, b, Text::DnsSearch, &i.dns_search);
    paste(d, b, Text::ExtraHosts, &i.extra_hosts);
    typed(d, b, Line::LogDriver, &a.logging.driver);
    paste(d, b, Text::LogOptions, &a.logging.options);
}

/// The fake Engine's container, which a copy is based on and Edit
/// replaces: a Compose service, known on its network by other names too,
/// which a copy leaves to the original and a replacement keeps.
fn original() -> serde_json::Value {
    let labels = serde_json::json!({
        "team": "web",
        "com.docker.compose.project": "shop",
        "com.docker.compose.service": "web",
    });
    serde_json::json!({
        "Id": "c1",
        "Name": "/web",
        "Image": "sha256:1111",
        "State": { "Status": "running", "Running": true },
        "Config": { "Image": "nginx:1.27", "Labels": labels },
        "HostConfig": { "NetworkMode": "proxy" },
        "NetworkSettings": {
            "Networks": { "proxy": { "Aliases": ["web", "www", "c1"] } },
        },
    })
}

/// A Run of a new container, or of a copy: the request, and what the
/// Engine was asked to create.
fn runs(case: &Case) {
    let (mut bench, engine, host) = bench();
    networks(&mut bench, &host);
    let in_pod = case.pod.is_some();
    if in_pod {
        let shop = homeport_engine::podman::pods::Pod {
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
    }
    let document = original();
    engine.inspects_as(document.clone());
    let mut dialog = if case.via == "duplicate" {
        let from = case.from.as_ref().expect("what was read back");
        let container = case.container.as_deref().unwrap_or_default();
        let taken: Vec<&str> = case.taken.iter().flatten().map(String::as_str).collect();
        let name = copy_name(container, &taken);
        Dialog::duplicate(&host, container, RunForm::of_spec(from), name, &bench.ctx())
    } else {
        let mut dialog = Dialog::run(&host, "", "", in_pod, &bench.ctx());
        let not_added = case
            .typed_not_added
            .as_ref()
            .map_or_else(NotAdded::default, |n| NotAdded {
                cap_add: n.cap_add.clone(),
                cap_drop: n.cap_drop.clone(),
            });
        fill(
            &mut dialog,
            &bench,
            case.form.as_ref().expect("a form"),
            &not_added,
        );
        dialog
    };
    let name = &case.name;
    let step = send(&mut dialog, &bench, Message::Submit);
    let Some(react) = &case.spec else {
        // React sent what Tauri refused: the dialog says it and waits.
        assert_eq!(step, Step::Stay, "{name}");
        assert!(!dialog.fields.is_ready(), "{name}");
        assert!(engine.created().is_empty(), "{name}");
        return;
    };
    assert_eq!(&dialog.fields.form.to_spec(), react, "{name}");
    assert_eq!(dialog.fields.form.pod(), case.pod, "{name}");
    if in_pod {
        // A Docker Engine: the pod is asked of Podman, which it isn't.
        assert!(engine.created().is_empty(), "{name}");
        return;
    }
    assert_eq!(step, Step::Started("new-1".to_owned()), "{name}");
    let inspect = serde_json::from_value(document).unwrap();
    let image = serde_json::from_str(IMAGE_INSPECTED).unwrap();
    let base = react.based_on.as_ref().map(|_| Base {
        inspect: &inspect,
        image: Some(&image),
        same_identity: false,
    });
    let tauri = serde_json::to_value(create_body(react, base.as_ref()).unwrap()).unwrap();
    let created = engine.created();
    assert_eq!(created.len(), 1, "{name}");
    assert_eq!(created[0].body, tauri, "{name}");
    assert_eq!(created[0].name, react.name, "{name}");
}

/// Sends `message` to Edit and what it started, to their end; the last
/// step that was not [`EditStep::Stay`].
fn edit_send(edit: &mut EditForm, bench: &Bench, message: EditMessage) -> EditStep {
    let runtime = bench.app.runtime.handle().clone();
    let _inside = runtime.enter();
    let (task, mut step) = edit.update(message, &bench.ctx());
    for answer in bench.run(task) {
        let next = edit_send(edit, bench, answer);
        if next != EditStep::Stay {
            step = next;
        }
    }
    step
}

/// Edit's form: the request it recreates the container from, and, Apply
/// pressed and the loss confirmed, the container the Engine is asked to
/// create in its place: what Tauri's `containers_recreate` asked with
/// React's request, the original's identity kept.
fn edits(case: &Case) {
    let (bench, engine, host) = bench();
    let document = original();
    engine.inspects_as(document.clone());
    let from = case.from.as_ref().expect("what was read back");
    let container = case.container.as_deref().unwrap_or_default();
    let mut web = crate::test_support::fixtures::container(container, None, "running", 0);
    "c1".clone_into(&mut web.id);
    let target = Target::of(&host, "fake", &web);
    let mut edit = EditForm::new(target, RunForm::of_spec(from), &bench.ctx());
    let react = case.spec.as_ref().expect("Edit's request");
    let name = &case.name;
    assert_eq!(&edit.fields().form.to_spec(), react, "{name}");
    edit_send(&mut edit, &bench, EditMessage::Submit);
    let confirmed = EditMessage::Confirm(confirm::Message::Confirm);
    let step = edit_send(&mut edit, &bench, confirmed);
    assert!(matches!(step, EditStep::Recreated(..)), "{name}: {step:?}");
    let inspect = serde_json::from_value(document).unwrap();
    let image = serde_json::from_str(IMAGE_INSPECTED).unwrap();
    let base = Base {
        inspect: &inspect,
        image: Some(&image),
        same_identity: true,
    };
    let tauri = serde_json::to_value(create_body(react, Some(&base)).unwrap()).unwrap();
    let created = engine.created();
    assert_eq!(created.len(), 1, "{name}");
    assert_eq!(created[0].body, tauri, "{name}");
    let swapped = created[0].name.as_deref().unwrap_or_default();
    assert!(swapped.starts_with("web-new-"), "{name}: {swapped}");
}

#[test]
fn every_form_makes_the_request_react_made_and_the_container_tauri_created() {
    let _english = in_language("en");
    let cases = cases();
    assert!(cases.len() >= 15, "the whole table");
    for case in &cases {
        match case.via.as_str() {
            "edit" => edits(case),
            _ => runs(case),
        }
    }
}

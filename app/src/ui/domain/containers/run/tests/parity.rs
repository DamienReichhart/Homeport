//! The container the Run dialog creates is the one the Tauri app created
//! from the same form. For each form of the table the dialog is filled in
//! as a user would (typed, picked, pasted, switched), Run is pressed, and
//! the create request the fake Engine received is compared with what the
//! Tauri command built from React's request for that form:
//! `containers_run` handed `toRunSpec(form)` to the crate's `run::run`,
//! whose request is `create_body(spec, base)`. The expected requests are
//! written as `toRunSpec` makes them (`runForm.test.ts`,
//! `fromRunSpec.test.ts`), and the body is built by the crate's own
//! function, not again here.
//!
//! What this can't show without an Engine: that the Engine makes of the
//! two requests the same container (`docker inspect`). The request is
//! the whole of what it is given, so the same request is the same
//! container.

use std::sync::Arc;

use iced::widget::text_editor::{Action, Edit};

use homeport_engine::containers::run::{
    create_body, Base, DeviceSpec, GpuSpec, HealthcheckSpec, LogSpec, MountSpec, PortSpec,
    RestartPolicy, RunSpec,
};

use super::{bench, networks, pick, send, type_in};
use crate::{
    test_support::{engine::IMAGE_INSPECTED, snapshot::in_language},
    ui::{
        domain::containers::run::{
            dialog::{Dialog, Message, Step},
            form::{Caps, Choice, Line, Message as Field, Rows, Switch, Text},
            model::RunForm,
        },
        shell::page::testing::Bench,
    },
};

/// Pastes `text` into an empty field of several lines.
fn paste(dialog: &mut Dialog, bench: &Bench, which: Text, text: &str) {
    let pasted = Action::Edit(Edit::Paste(Arc::new(text.to_owned())));
    send(dialog, bench, Message::Form(Field::Text(which, pasted)));
}

fn field(dialog: &mut Dialog, bench: &Bench, message: Field) {
    send(dialog, bench, Message::Form(message));
}

fn port(host: Option<u16>, container: u16, protocol: &str) -> PortSpec {
    PortSpec {
        host_ip: None,
        host_port: host,
        container_port: container,
        protocol: protocol.to_owned(),
    }
}

fn strings(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_owned()).collect()
}

const MIB: i64 = 1024 * 1024;

/// A form of the table: what the user does, and the request React made.
struct Case {
    name: &'static str,
    fill: fn(&mut Dialog, &Bench),
    react: fn() -> RunSpec,
}

// A table, read row by row.
#[allow(clippy::too_many_lines)]
fn cases() -> Vec<Case> {
    vec![
        Case {
            name: "an image alone",
            fill: |d, b| type_in(d, b, Line::Image, "nginx"),
            react: || RunSpec {
                image: "nginx".to_owned(),
                ..RunSpec::default()
            },
        },
        Case {
            name: "name, command, environment, ports, volumes, network, restart",
            fill: |d, b| {
                type_in(d, b, Line::Image, " nginx:1.27 ");
                type_in(d, b, Line::Name, "web");
                type_in(d, b, Line::Command, "nginx -g 'daemon off;'");
                paste(d, b, Text::Env, "A=1\n\n# comment\nB=two words\n");
                for _ in 0..3 {
                    field(d, b, Field::Add(Rows::Ports));
                }
                type_in(d, b, Line::HostPort(0), "8080");
                type_in(d, b, Line::ContainerPort(0), "80");
                type_in(d, b, Line::ContainerPort(1), "53");
                pick(d, b, Choice::Protocol(1), "udp");
                for _ in 0..3 {
                    field(d, b, Field::Add(Rows::Mounts));
                }
                type_in(d, b, Line::Source(0), "/srv");
                type_in(d, b, Line::Target(0), "/data");
                field(d, b, Field::Switch(Switch::ReadOnly(0), true));
                type_in(d, b, Line::Source(1), " site ");
                type_in(d, b, Line::Target(1), "/var/www");
                pick(d, b, Choice::Network, "proxy");
                pick(d, b, Choice::Restart, "Unless stopped");
            },
            react: || RunSpec {
                image: "nginx:1.27".to_owned(),
                name: Some("web".to_owned()),
                command: Some("nginx -g 'daemon off;'".to_owned()),
                env: strings(&["A=1", "B=two words"]),
                ports: vec![port(Some(8080), 80, "tcp"), port(None, 53, "udp")],
                mounts: vec![
                    MountSpec {
                        source: "/srv".to_owned(),
                        target: "/data".to_owned(),
                        read_only: true,
                    },
                    MountSpec {
                        source: "site".to_owned(),
                        target: "/var/www".to_owned(),
                        read_only: false,
                    },
                ],
                network: Some("proxy".to_owned()),
                restart: RestartPolicy::UnlessStopped,
                ..RunSpec::default()
            },
        },
        Case {
            name: "removed when it exits",
            fill: |d, b| {
                type_in(d, b, Line::Image, "alpine");
                pick(d, b, Choice::Restart, "Always");
                field(d, b, Field::Switch(Switch::AutoRemove, true));
            },
            react: || RunSpec {
                image: "alpine".to_owned(),
                auto_remove: true,
                ..RunSpec::default()
            },
        },
        Case {
            name: "limits and labels",
            fill: |d, b| {
                type_in(d, b, Line::Image, "nginx");
                type_in(d, b, Line::Cpus, "0.5");
                type_in(d, b, Line::CpuShares, "512");
                type_in(d, b, Line::Memory, "512m");
                type_in(d, b, Line::MemoryReservation, "256MiB");
                type_in(d, b, Line::MemorySwap, "-1");
                paste(d, b, Text::Labels, "traefik.enable=true\n\nteam=web");
            },
            react: || RunSpec {
                image: "nginx".to_owned(),
                cpus: Some(0.5),
                cpu_shares: Some(512),
                memory: Some(512 * MIB),
                memory_reservation: Some(256 * MIB),
                memory_swap: Some(-1),
                labels: strings(&["traefik.enable=true", "team=web"]),
                ..RunSpec::default()
            },
        },
        Case {
            name: "capabilities, privileged, devices, a number of GPUs",
            fill: |d, b| {
                type_in(d, b, Line::Image, "wireguard");
                type_in(d, b, Line::Capability(Caps::Add), "net_admin");
                field(d, b, Field::AddCapability(Caps::Add));
                type_in(d, b, Line::Capability(Caps::Add), "cap_sys_module,");
                type_in(d, b, Line::Capability(Caps::Drop), "ALL");
                field(d, b, Field::AddCapability(Caps::Drop));
                field(d, b, Field::Switch(Switch::Privileged, true));
                field(d, b, Field::Add(Rows::Devices));
                field(d, b, Field::Add(Rows::Devices));
                type_in(d, b, Line::HostDevice(0), "/dev/dri");
                type_in(d, b, Line::Permissions(0), "");
                type_in(d, b, Line::HostDevice(1), "/dev/ttyUSB0");
                type_in(d, b, Line::ContainerDevice(1), "/dev/zigbee");
                type_in(d, b, Line::Permissions(1), "rw");
                pick(d, b, Choice::Gpus, "A number of GPUs");
                type_in(d, b, Line::GpuCount, "2");
            },
            react: || RunSpec {
                image: "wireguard".to_owned(),
                cap_add: strings(&["NET_ADMIN", "SYS_MODULE"]),
                cap_drop: strings(&["ALL"]),
                privileged: true,
                devices: vec![
                    DeviceSpec {
                        host_path: "/dev/dri".to_owned(),
                        container_path: None,
                        permissions: "rwm".to_owned(),
                    },
                    DeviceSpec {
                        host_path: "/dev/ttyUSB0".to_owned(),
                        container_path: Some("/dev/zigbee".to_owned()),
                        permissions: "rw".to_owned(),
                    },
                ],
                gpus: Some(GpuSpec::Count { count: 2 }),
                ..RunSpec::default()
            },
        },
        Case {
            name: "a healthcheck, chosen GPUs, the network identity and logging",
            fill: |d, b| {
                type_in(d, b, Line::Image, "nginx");
                pick(d, b, Choice::Gpus, "Chosen GPUs");
                type_in(d, b, Line::GpuIds, "0, GPU-abc");
                pick(d, b, Choice::Health, "A command");
                type_in(d, b, Line::HealthCommand, " curl -f localhost ");
                type_in(d, b, Line::Interval, "30s");
                type_in(d, b, Line::Timeout, "5");
                type_in(d, b, Line::Retries, "3");
                type_in(d, b, Line::StartPeriod, "1m30s");
                type_in(d, b, Line::Hostname, "web");
                type_in(d, b, Line::Domainname, "home.arpa");
                paste(d, b, Text::Dns, "1.1.1.1\n9.9.9.9");
                paste(d, b, Text::DnsSearch, "home.arpa");
                paste(d, b, Text::ExtraHosts, "nas:192.168.1.10\n# the NAS\n");
                type_in(d, b, Line::LogDriver, "json-file");
                paste(d, b, Text::LogOptions, "max-size=10m\nmax-file=3");
            },
            react: || RunSpec {
                image: "nginx".to_owned(),
                gpus: Some(GpuSpec::Ids {
                    ids: strings(&["0", "GPU-abc"]),
                }),
                healthcheck: Some(HealthcheckSpec {
                    command: "curl -f localhost".to_owned(),
                    interval_s: Some(30),
                    timeout_s: Some(5),
                    retries: Some(3),
                    start_period_s: Some(90),
                    disabled: false,
                }),
                hostname: Some("web".to_owned()),
                domainname: Some("home.arpa".to_owned()),
                dns: strings(&["1.1.1.1", "9.9.9.9"]),
                dns_search: strings(&["home.arpa"]),
                extra_hosts: strings(&["nas:192.168.1.10"]),
                log: Some(LogSpec {
                    driver: "json-file".to_owned(),
                    options: strings(&["max-size=10m", "max-file=3"]),
                }),
                ..RunSpec::default()
            },
        },
        Case {
            name: "the image's healthcheck turned off",
            fill: |d, b| {
                type_in(d, b, Line::Image, "nginx");
                pick(d, b, Choice::Health, "None (turn off the image's)");
            },
            react: || RunSpec {
                image: "nginx".to_owned(),
                healthcheck: Some(HealthcheckSpec {
                    disabled: true,
                    ..HealthcheckSpec::default()
                }),
                ..RunSpec::default()
            },
        },
    ]
}

#[test]
fn every_form_creates_the_container_the_tauri_app_created() {
    let _english = in_language("en");
    for case in cases() {
        let (mut bench, engine, host) = bench();
        networks(&mut bench, &host);
        let mut dialog = Dialog::run(&host, "", "", false, &bench.ctx());
        (case.fill)(&mut dialog, &bench);
        let react = (case.react)();
        assert_eq!(dialog.fields.form.to_spec(), react, "{}", case.name);
        let step = send(&mut dialog, &bench, Message::Submit);
        assert_eq!(step, Step::Started("new-1".to_owned()), "{}", case.name);
        let created = engine.created();
        assert_eq!(created.len(), 1, "{}", case.name);
        let tauri = serde_json::to_value(create_body(&react, None).unwrap()).unwrap();
        assert_eq!(created[0].body, tauri, "{}", case.name);
        assert_eq!(created[0].name, react.name, "{}", case.name);
    }
}

/// The container a copy is made of, as `containers_run_spec` read it.
fn original() -> RunSpec {
    RunSpec {
        image: "nginx:1.27".to_owned(),
        name: Some("web".to_owned()),
        env: strings(&["MODE=prod"]),
        ports: vec![port(Some(8080), 80, "tcp")],
        mounts: vec![MountSpec {
            source: "site".to_owned(),
            target: "/srv".to_owned(),
            read_only: true,
        }],
        restart: RestartPolicy::Always,
        memory: Some(512 * MIB),
        healthcheck: Some(HealthcheckSpec {
            command: "curl -f localhost".to_owned(),
            interval_s: Some(90),
            ..HealthcheckSpec::default()
        }),
        based_on: Some("c1".to_owned()),
        ..RunSpec::default()
    }
}

#[test]
fn a_copy_creates_what_the_tauri_app_created_from_its_original() {
    let _english = in_language("en");
    let (bench, engine, host) = bench();
    let document = serde_json::json!({
        "Id": "c1",
        "Name": "/web",
        "Image": "sha256:1111",
        "State": { "Status": "running", "Running": true },
        "Config": { "Image": "nginx:1.27", "Env": ["MODE=prod"], "Labels": { "team": "web" } },
        "HostConfig": { "NetworkMode": "bridge", "RestartPolicy": { "Name": "always" } },
    });
    engine.inspects_as(document.clone());
    let form = RunForm::of_spec(&original());
    let mut dialog = Dialog::duplicate(&host, "web", form, "web-copy".to_owned(), &bench.ctx());
    // The copy can't take the original's host port: it is let go.
    type_in(&mut dialog, &bench, Line::HostPort(0), "");
    let step = send(&mut dialog, &bench, Message::Submit);
    assert_eq!(step, Step::Started("new-1".to_owned()));
    let react = RunSpec {
        name: Some("web-copy".to_owned()),
        ports: vec![port(None, 80, "tcp")],
        ..original()
    };
    assert_eq!(dialog.fields.form.to_spec(), react);
    // The crate's types (bollard's, which the app doesn't name), read from
    // the documents the fake Engine answered.
    let inspect = serde_json::from_value(document).unwrap();
    let image = serde_json::from_str(IMAGE_INSPECTED).unwrap();
    let base = Base {
        inspect: &inspect,
        image: Some(&image),
        same_identity: false,
    };
    let tauri = serde_json::to_value(create_body(&react, Some(&base)).unwrap()).unwrap();
    let created = engine.created();
    assert_eq!(created.len(), 1);
    assert_eq!(created[0].body, tauri);
    assert_eq!(created[0].name.as_deref(), Some("web-copy"));
}

//! The React model's tests, input for input (`runForm.test.ts`,
//! `fromRunSpec.test.ts`, `commit.test.ts`, `parse.test.ts`): the same
//! form makes the same request.

use std::collections::HashMap;

use homeport_engine::{
    containers::run::{
        DeviceSpec, GpuSpec, HealthcheckSpec, LogSpec, MountSpec, PortSpec, RestartPolicy, RunSpec,
    },
    images::Image,
};

use super::{
    advanced::{self, DeviceRow, Field, GpuMode, Gpus, HealthMode, Healthcheck, Resources},
    commit_name, copy_name, image_named,
    parse::{format_duration, format_size, parse_duration, parse_positive, parse_size, Unreadable},
    shared_host_ports, CommitForm, MountRow, PortRow, RunForm,
};

const MIB: i64 = 1024 * 1024;

fn port(host: &str, container: &str, protocol: &str) -> PortRow {
    PortRow {
        host_port: host.to_owned(),
        container_port: container.to_owned(),
        protocol: protocol.to_owned(),
    }
}

fn with_advanced(patch: impl FnOnce(&mut advanced::Advanced)) -> RunForm {
    let mut form = RunForm::new("nginx");
    patch(&mut form.advanced);
    form
}

#[test]
fn ports_and_the_network_are_the_pod_s_for_a_container_run_in_one() {
    let form = RunForm {
        ports: vec![port("8080", "80", "tcp")],
        network: "web".to_owned(),
        pod: "shop".to_owned(),
        ..RunForm::new("nginx")
    };
    assert_eq!(form.pod().as_deref(), Some("shop"));
    let spec = form.to_spec();
    assert!(spec.ports.is_empty());
    assert_eq!(spec.network, None);
    let blank = RunForm {
        pod: " ".to_owned(),
        ..form.clone()
    };
    assert_eq!(blank.pod(), None);
    let none = RunForm {
        pod: String::new(),
        ..form
    };
    assert_eq!(none.to_spec().network.as_deref(), Some("web"));
}

#[test]
fn the_form_s_rows_make_the_run_request() {
    let form = RunForm {
        name: "web".to_owned(),
        env: "A=1\n\n# comment\nB=two words\n".to_owned(),
        ports: vec![
            port("8080", "80", "tcp"),
            port("", "53", "udp"),
            port("", "", "tcp"),
        ],
        mounts: vec![
            MountRow {
                source: "/srv".to_owned(),
                target: "/data".to_owned(),
                read_only: true,
            },
            MountRow::default(),
        ],
        restart: RestartPolicy::UnlessStopped,
        ..RunForm::new(" nginx ")
    };
    assert!(form.errors().is_empty());
    let expected = RunSpec {
        image: "nginx".to_owned(),
        name: Some("web".to_owned()),
        env: vec!["A=1".to_owned(), "B=two words".to_owned()],
        ports: vec![
            PortSpec {
                host_ip: None,
                host_port: Some(8080),
                container_port: 80,
                protocol: "tcp".to_owned(),
            },
            PortSpec {
                host_ip: None,
                host_port: None,
                container_port: 53,
                protocol: "udp".to_owned(),
            },
        ],
        mounts: vec![MountSpec {
            source: "/srv".to_owned(),
            target: "/data".to_owned(),
            read_only: true,
        }],
        restart: RestartPolicy::UnlessStopped,
        ..RunSpec::default()
    };
    assert_eq!(form.to_spec(), expected);
}

#[test]
fn a_port_that_is_none_is_said_under_its_row() {
    let form = RunForm {
        ports: vec![
            port("8080", "80", "tcp"),
            port("", "http", "tcp"),
            port("70000", "443", "tcp"),
            port("x", "", "tcp"),
        ],
        ..RunForm::new("nginx")
    };
    // A row without a container port is not sent, whatever its host port.
    assert_eq!(form.errors().ports, vec![1, 2]);
}

#[test]
fn limits_and_labels_are_sent_in_bytes_and_cpus() {
    let form = with_advanced(|a| {
        a.resources = Resources {
            cpus: "0.5".to_owned(),
            cpu_shares: "512".to_owned(),
            memory: "512m".to_owned(),
            memory_reservation: "256m".to_owned(),
            memory_swap: "-1".to_owned(),
        };
        a.labels = "traefik.enable=true\n\nteam=web".to_owned();
    });
    assert!(form.errors().is_empty());
    let spec = form.to_spec();
    assert_eq!(spec.cpus, Some(0.5));
    assert_eq!(spec.cpu_shares, Some(512));
    assert_eq!(spec.memory, Some(512 * MIB));
    assert_eq!(spec.memory_reservation, Some(256 * MIB));
    assert_eq!(spec.memory_swap, Some(-1));
    assert_eq!(spec.labels, ["traefik.enable=true", "team=web"]);
}

#[test]
fn capabilities_devices_and_gpus_are_sent() {
    let form = with_advanced(|a| {
        a.security.cap_add = vec!["NET_ADMIN".to_owned()];
        a.security.cap_drop = vec!["ALL".to_owned()];
        a.security.devices = vec![
            DeviceRow {
                host_path: "/dev/dri".to_owned(),
                container_path: String::new(),
                permissions: String::new(),
            },
            DeviceRow::default(),
        ];
        a.security.gpus = Gpus {
            mode: GpuMode::Ids,
            count: "1".to_owned(),
            ids: "0, GPU-abc".to_owned(),
        };
    });
    let spec = form.to_spec();
    assert_eq!(spec.cap_add, ["NET_ADMIN"]);
    assert_eq!(spec.cap_drop, ["ALL"]);
    assert_eq!(
        spec.devices,
        [DeviceSpec {
            host_path: "/dev/dri".to_owned(),
            container_path: None,
            permissions: "rwm".to_owned(),
        }]
    );
    let ids = vec!["0".to_owned(), "GPU-abc".to_owned()];
    assert_eq!(spec.gpus, Some(GpuSpec::Ids { ids }));
}

#[test]
fn a_healthcheck_the_network_identity_and_logging_are_sent() {
    let form = with_advanced(|a| {
        a.healthcheck = Healthcheck {
            mode: HealthMode::Custom,
            command: "curl -f localhost".to_owned(),
            interval: "30s".to_owned(),
            timeout: "5s".to_owned(),
            retries: "3".to_owned(),
            start_period: "1m30s".to_owned(),
        };
        a.identity.hostname = "web".to_owned();
        a.identity.dns = "1.1.1.1\n9.9.9.9".to_owned();
        a.identity.dns_search = "home.arpa".to_owned();
        a.identity.extra_hosts = "nas:192.168.1.10".to_owned();
        a.logging.driver = "json-file".to_owned();
        a.logging.options = "max-size=10m\nmax-file=3".to_owned();
    });
    let spec = form.to_spec();
    let health = HealthcheckSpec {
        command: "curl -f localhost".to_owned(),
        interval_s: Some(30),
        timeout_s: Some(5),
        retries: Some(3),
        start_period_s: Some(90),
        disabled: false,
    };
    assert_eq!(spec.healthcheck, Some(health));
    assert_eq!(spec.hostname.as_deref(), Some("web"));
    assert_eq!(spec.domainname, None);
    assert_eq!(spec.dns, ["1.1.1.1", "9.9.9.9"]);
    assert_eq!(spec.dns_search, ["home.arpa"]);
    assert_eq!(spec.extra_hosts, ["nas:192.168.1.10"]);
    let options = vec!["max-size=10m".to_owned(), "max-file=3".to_owned()];
    let log = LogSpec {
        driver: "json-file".to_owned(),
        options,
    };
    assert_eq!(spec.log, Some(log));
}

#[test]
fn the_image_s_healthcheck_can_be_turned_off() {
    let form = with_advanced(|a| a.healthcheck.mode = HealthMode::Disabled);
    assert!(form.to_spec().healthcheck.is_some_and(|h| h.disabled));
}

#[test]
fn values_it_cannot_read_are_said() {
    let form = with_advanced(|a| {
        a.resources.cpus = "half".to_owned();
        a.resources.memory = "512x".to_owned();
        a.resources.memory_swap = "lots".to_owned();
        a.security.gpus = Gpus {
            mode: GpuMode::Count,
            count: "0".to_owned(),
            ids: String::new(),
        };
        a.healthcheck.mode = HealthMode::Custom;
        a.healthcheck.interval = "30 sec".to_owned();
        a.healthcheck.retries = "1.5".to_owned();
    });
    let mut errors = form.errors().advanced;
    errors.sort();
    let mut expected = vec![
        Field::Cpus,
        Field::GpuCount,
        Field::Interval,
        Field::Memory,
        Field::MemorySwap,
        Field::Retries,
    ];
    expected.sort();
    assert_eq!(errors, expected);
}

#[test]
fn the_durations_of_a_healthcheck_left_to_the_image_are_not_read() {
    let form = with_advanced(|a| a.healthcheck.interval = "nonsense".to_owned());
    assert!(form.errors().is_empty());
}

/// `full` of `fromRunSpec.test.ts`.
fn full() -> RunSpec {
    RunSpec {
        image: "nginx:1.27".to_owned(),
        name: None,
        command: Some("nginx -c '/etc/nginx/alt conf'".to_owned()),
        env: vec!["MODE=prod".to_owned()],
        ports: vec![PortSpec {
            host_ip: None,
            host_port: Some(8080),
            container_port: 80,
            protocol: "tcp".to_owned(),
        }],
        mounts: vec![MountSpec {
            source: "site".to_owned(),
            target: "/srv".to_owned(),
            read_only: true,
        }],
        network: Some("proxy".to_owned()),
        restart: RestartPolicy::UnlessStopped,
        auto_remove: false,
        cpus: Some(1.5),
        cpu_shares: Some(512),
        memory: Some(512 * MIB),
        memory_reservation: Some(256 * MIB),
        memory_swap: Some(-1),
        labels: vec!["team=web".to_owned(), "traefik.enable=true".to_owned()],
        cap_add: vec!["NET_ADMIN".to_owned()],
        cap_drop: vec!["ALL".to_owned()],
        privileged: false,
        devices: vec![DeviceSpec {
            host_path: "/dev/ttyUSB0".to_owned(),
            container_path: Some("/dev/zigbee".to_owned()),
            permissions: "rw".to_owned(),
        }],
        gpus: Some(GpuSpec::Count { count: 2 }),
        healthcheck: Some(HealthcheckSpec {
            command: "curl -f localhost".to_owned(),
            interval_s: Some(90),
            timeout_s: Some(5),
            retries: Some(3),
            start_period_s: None,
            disabled: false,
        }),
        hostname: Some("web".to_owned()),
        domainname: None,
        dns: vec!["1.1.1.1".to_owned()],
        dns_search: vec!["home.arpa".to_owned()],
        extra_hosts: vec!["nas:192.168.1.10".to_owned()],
        log: Some(LogSpec {
            driver: "local".to_owned(),
            options: vec!["max-size=10m".to_owned()],
        }),
        based_on: Some("abc123".to_owned()),
    }
}

#[test]
fn a_form_read_back_from_a_container_sends_what_it_was_read_from() {
    assert_eq!(RunForm::of_spec(&full()).to_spec(), full());
}

#[test]
fn sizes_and_durations_read_back_are_written_as_they_are_typed() {
    let form = RunForm::of_spec(&full());
    assert_eq!(form.advanced.resources.memory, "512m");
    assert_eq!(form.advanced.healthcheck.interval, "1m30s");
    assert_eq!(form.advanced.security.gpus.mode, GpuMode::Count);
    assert_eq!(form.advanced.security.gpus.count, "2");
    assert!(form.has_advanced());
    assert!(!RunForm::new("nginx").has_advanced());
}

#[test]
fn a_copy_is_given_a_free_name() {
    assert_eq!(copy_name("web", &["web"]), "web-copy");
    let taken = ["web", "web-copy", "web-copy-2"];
    assert_eq!(copy_name("web", &taken), "web-copy-3");
}

#[test]
fn the_host_ports_the_original_publishes_are_found() {
    let original = RunForm::of_spec(&full());
    let copy = RunForm {
        ports: vec![
            port("8080", "80", "tcp"),
            port("8080", "81", "udp"),
            port("", "443", "tcp"),
        ],
        ..original.clone()
    };
    assert_eq!(shared_host_ports(&copy, &original), ["8080"]);
}

fn image(tags: &[&str]) -> Image {
    Image {
        id: "sha256:a".to_owned(),
        tags: tags.iter().map(|t| (*t).to_owned()).collect(),
        digests: vec![],
        created: 0,
        size: 0,
        containers: 0,
        labels: HashMap::new(),
    }
}

#[test]
fn a_commit_is_named_after_its_container_and_the_day_in_lower_case() {
    assert_eq!(
        commit_name("Shop_Web-1", "2026-11-04"),
        "shop_web-1:2026-11-04"
    );
    assert_eq!(super::today().len(), "2026-11-04".len());
}

#[test]
fn a_commit_sends_its_changes_one_a_line_without_blanks() {
    let form = CommitForm {
        reference: " web:backup ".to_owned(),
        comment: "Before the upgrade".to_owned(),
        changes: "ENV MODE=prod\n\nEXPOSE 8080\n".to_owned(),
        ..CommitForm::new("web", "2026-11-04")
    };
    let spec = form.to_spec();
    assert_eq!(spec.reference, "web:backup");
    assert_eq!(spec.comment.as_deref(), Some("Before the upgrade"));
    assert_eq!(spec.author, None);
    assert!(spec.pause);
    assert_eq!(spec.changes, ["ENV MODE=prod", "EXPOSE 8080"]);
}

#[test]
fn the_image_whose_tag_a_commit_moves_is_found() {
    let images = [
        image(&["web:backup"]),
        image(&["registry.home:5000/app:latest"]),
    ];
    assert!(image_named("web:backup", &images).is_some_and(|i| i.tags == ["web:backup"]));
    let app = image_named("registry.home:5000/app", &images);
    assert!(app.is_some_and(|i| i.tags == ["registry.home:5000/app:latest"]));
    assert!(image_named("web:other", &images).is_none());
}

#[test]
fn sizes_are_read_as_docker_run_reads_them() {
    assert_eq!(parse_size("512m"), Ok(Some(512 * MIB)));
    assert_eq!(parse_size("2g"), Ok(Some(2 * 1024 * MIB)));
    assert_eq!(parse_size(" 1.5GB "), Ok(Some(1536 * MIB)));
    assert_eq!(parse_size("64MiB"), Ok(Some(64 * MIB)));
    assert_eq!(parse_size("1024"), Ok(Some(1024)));
    assert_eq!(parse_size("100b"), Ok(Some(100)));
    assert_eq!(parse_size(""), Ok(None));
    for bad in ["m", "12x", "-1g", "0", "1,5g"] {
        assert_eq!(parse_size(bad), Err(Unreadable), "{bad}");
    }
}

#[test]
fn sizes_are_written_back_in_the_largest_exact_unit() {
    assert_eq!(format_size(512 * MIB), "512m");
    assert_eq!(format_size(2 * 1024 * MIB), "2g");
    assert_eq!(format_size(1500), "1500");
    assert_eq!(parse_size(&format_size(3 * 1024)), Ok(Some(3 * 1024)));
}

#[test]
fn durations_are_read_in_hours_minutes_and_seconds() {
    assert_eq!(parse_duration("30s"), Ok(Some(30)));
    assert_eq!(parse_duration("1m30s"), Ok(Some(90)));
    assert_eq!(parse_duration("2h"), Ok(Some(7200)));
    assert_eq!(parse_duration("45"), Ok(Some(45)));
    assert_eq!(parse_duration(""), Ok(None));
    for bad in ["0s", "0", "1.5s", "s", "10 minutes", "-5s"] {
        assert_eq!(parse_duration(bad), Err(Unreadable), "{bad}");
    }
    assert_eq!(format_duration(90), "1m30s");
    assert_eq!(format_duration(7200), "2h");
    assert_eq!(format_duration(5), "5s");
}

#[test]
fn positive_numbers_are_read() {
    assert_eq!(parse_positive("0.5", false), Ok(Some(0.5)));
    assert_eq!(parse_positive("3", true), Ok(Some(3.0)));
    assert_eq!(parse_positive("1.5", true), Err(Unreadable));
    assert_eq!(parse_positive("0", false), Err(Unreadable));
    assert_eq!(parse_positive("abc", false), Err(Unreadable));
    assert_eq!(parse_positive(" ", false), Ok(None));
}

#[test]
fn a_capability_is_written_as_the_list_writes_it() {
    assert_eq!(advanced::capability("cap_net_admin"), "NET_ADMIN");
    assert_eq!(advanced::capability("sys_time"), "SYS_TIME");
}

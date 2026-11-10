use homeport_core::error::ErrorCode;

use super::*;
use crate::containers::run::{DeviceSpec, MountSpec, PortSpec};

fn spec() -> RunSpec {
    RunSpec {
        image: "nginx:1.27".into(),
        ..Default::default()
    }
}

fn host(spec: RunSpec) -> HostConfig {
    create_body(&spec, None).unwrap().host_config.unwrap()
}

#[test]
fn run_options_map_to_the_engine_request() {
    let body = create_body(
        &RunSpec {
            command: Some(r#"sh -c "echo 'hi there'""#.into()),
            env: vec!["MODE=prod".into()],
            ports: vec![
                PortSpec {
                    host_ip: None,
                    host_port: Some(8080),
                    container_port: 80,
                    protocol: "tcp".into(),
                },
                PortSpec {
                    host_ip: Some("127.0.0.1".into()),
                    host_port: None,
                    container_port: 53,
                    protocol: "udp".into(),
                },
            ],
            mounts: vec![
                MountSpec {
                    source: "/srv/site".into(),
                    target: "/usr/share/nginx/html".into(),
                    read_only: true,
                },
                MountSpec {
                    source: "cache".into(),
                    target: "/cache".into(),
                    read_only: false,
                },
            ],
            restart: RestartPolicy::UnlessStopped,
            ..spec()
        },
        None,
    )
    .unwrap();
    assert_eq!(body.cmd.unwrap(), ["sh", "-c", "echo 'hi there'"]);
    assert_eq!(body.env.unwrap(), ["MODE=prod"]);
    let host = body.host_config.unwrap();
    let ports = host.port_bindings.unwrap();
    assert_eq!(
        ports["80/tcp"].as_ref().unwrap()[0].host_port.as_deref(),
        Some("8080")
    );
    let dns = &ports["53/udp"].as_ref().unwrap()[0];
    assert_eq!(
        (dns.host_ip.as_deref(), dns.host_port.as_deref()),
        (Some("127.0.0.1"), Some(""))
    );
    assert_eq!(
        host.binds.unwrap(),
        ["/srv/site:/usr/share/nginx/html:ro", "cache:/cache"]
    );
    assert_eq!(
        host.restart_policy.unwrap().name,
        Some(RestartPolicyNameEnum::UNLESS_STOPPED)
    );
}

#[test]
fn limits_and_labels_map_to_the_engine_request() {
    let body = create_body(
        &RunSpec {
            cpus: Some(0.5),
            cpu_shares: Some(512),
            memory: Some(512 * 1024 * 1024),
            memory_reservation: Some(256 * 1024 * 1024),
            memory_swap: Some(-1),
            labels: vec!["traefik.enable=true".into(), "team=web".into()],
            ..spec()
        },
        None,
    )
    .unwrap();
    let labels = body.labels.unwrap();
    assert_eq!(labels["traefik.enable"], "true");
    assert_eq!(labels["team"], "web");
    let host = body.host_config.unwrap();
    assert_eq!(host.nano_cpus, Some(500_000_000));
    assert_eq!(host.cpu_shares, Some(512));
    assert_eq!(host.memory, Some(512 * 1024 * 1024));
    assert_eq!(host.memory_reservation, Some(256 * 1024 * 1024));
    assert_eq!(host.memory_swap, Some(-1));
}

#[test]
fn security_options_map_to_the_engine_request() {
    let host = host(RunSpec {
        cap_add: vec!["net_admin".into()],
        cap_drop: vec!["ALL".into()],
        privileged: true,
        devices: vec![
            DeviceSpec {
                host_path: "/dev/dri".into(),
                container_path: None,
                permissions: "rwm".into(),
            },
            DeviceSpec {
                host_path: "/dev/ttyUSB0".into(),
                container_path: Some("/dev/zigbee".into()),
                permissions: "rw".into(),
            },
        ],
        gpus: Some(GpuSpec::All),
        ..spec()
    });
    assert_eq!(host.cap_add.unwrap(), ["CAP_NET_ADMIN"]);
    assert_eq!(host.cap_drop.unwrap(), ["ALL"]);
    assert_eq!(host.privileged, Some(true));
    let devices = host.devices.unwrap();
    assert_eq!(devices[0].path_in_container.as_deref(), Some("/dev/dri"));
    assert_eq!(devices[1].path_in_container.as_deref(), Some("/dev/zigbee"));
    assert_eq!(devices[1].cgroup_permissions.as_deref(), Some("rw"));
    let gpu = &host.device_requests.unwrap()[0];
    assert_eq!(gpu.count, Some(-1));
    assert_eq!(gpu.driver, None);
    assert_eq!(gpu.capabilities, Some(vec![vec!["gpu".to_owned()]]));
}

#[test]
fn gpus_can_be_counted_or_named() {
    let requests = |gpus| {
        host(RunSpec {
            gpus: Some(gpus),
            ..spec()
        })
        .device_requests
        .unwrap()
    };
    assert_eq!(requests(GpuSpec::Count { count: 2 })[0].count, Some(2));
    let named = requests(GpuSpec::Ids {
        ids: vec!["0".into(), " GPU-abc ".into()],
    });
    assert_eq!(named[0].count, None);
    assert_eq!(
        named[0].device_ids.as_deref(),
        Some(&["0".to_owned(), "GPU-abc".to_owned()][..])
    );
}

#[test]
fn healthchecks_network_identity_and_logging_map_to_the_engine_request() {
    let body = create_body(
        &RunSpec {
            healthcheck: Some(HealthcheckSpec {
                command: "wget -qO- localhost || exit 1".into(),
                interval_s: Some(30),
                timeout_s: Some(5),
                retries: Some(3),
                start_period_s: Some(90),
                disabled: false,
            }),
            hostname: Some("web".into()),
            domainname: Some("home.arpa".into()),
            dns: vec!["1.1.1.1".into(), "".into()],
            dns_search: vec!["home.arpa".into()],
            extra_hosts: vec!["nas:192.168.1.10".into(), "gw:host-gateway".into()],
            log: Some(LogSpec {
                driver: "json-file".into(),
                options: vec!["max-size=10m".into(), "max-file=3".into()],
            }),
            ..spec()
        },
        None,
    )
    .unwrap();
    let health = body.healthcheck.unwrap();
    assert_eq!(
        health.test.unwrap(),
        ["CMD-SHELL", "wget -qO- localhost || exit 1"]
    );
    assert_eq!(health.interval, Some(30_000_000_000));
    assert_eq!(health.timeout, Some(5_000_000_000));
    assert_eq!(health.retries, Some(3));
    assert_eq!(health.start_period, Some(90_000_000_000));
    assert_eq!(body.hostname.as_deref(), Some("web"));
    assert_eq!(body.domainname.as_deref(), Some("home.arpa"));
    let host = body.host_config.unwrap();
    assert_eq!(host.dns.unwrap(), ["1.1.1.1"]);
    assert_eq!(host.dns_search.unwrap(), ["home.arpa"]);
    assert_eq!(
        host.extra_hosts.unwrap(),
        ["nas:192.168.1.10", "gw:host-gateway"]
    );
    let log = host.log_config.unwrap();
    assert_eq!(log.typ.as_deref(), Some("json-file"));
    assert_eq!(log.config.unwrap()["max-size"], "10m");
}

#[test]
fn disabling_the_healthcheck_sends_none() {
    let body = create_body(
        &RunSpec {
            healthcheck: Some(HealthcheckSpec {
                disabled: true,
                ..Default::default()
            }),
            ..spec()
        },
        None,
    )
    .unwrap();
    assert_eq!(body.healthcheck.unwrap().test.unwrap(), ["NONE"]);
}

#[test]
fn unset_options_leave_the_engine_defaults() {
    let body = create_body(&spec(), None).unwrap();
    assert_eq!(body.labels, None);
    assert_eq!(body.healthcheck, None);
    let host = body.host_config.unwrap();
    assert_eq!(
        (host.nano_cpus, host.memory, host.cap_add, host.privileged),
        (None, None, None, None)
    );
    assert_eq!(
        (host.devices, host.device_requests, host.log_config),
        (None, None, None)
    );
}

#[test]
fn invalid_run_options_are_explained() {
    let mb = 1024 * 1024;
    let device = |host_path: &str, permissions: &str| DeviceSpec {
        host_path: host_path.into(),
        container_path: None,
        permissions: permissions.into(),
    };
    let bad = [
        RunSpec {
            image: " ".into(),
            ..spec()
        },
        RunSpec {
            env: vec!["NOVALUE".into()],
            ..spec()
        },
        RunSpec {
            command: Some("echo 'oops".into()),
            ..spec()
        },
        RunSpec {
            auto_remove: true,
            restart: RestartPolicy::Always,
            ..spec()
        },
        RunSpec {
            mounts: vec![MountSpec {
                source: "data".into(),
                target: "relative".into(),
                read_only: false,
            }],
            ..spec()
        },
        RunSpec {
            cpus: Some(0.0),
            ..spec()
        },
        RunSpec {
            cpus: Some(f64::NAN),
            ..spec()
        },
        RunSpec {
            cpu_shares: Some(1),
            ..spec()
        },
        RunSpec {
            memory: Some(mb),
            ..spec()
        },
        RunSpec {
            memory: Some(-5),
            ..spec()
        },
        RunSpec {
            memory: Some(64 * mb),
            memory_reservation: Some(128 * mb),
            ..spec()
        },
        RunSpec {
            memory_swap: Some(128 * mb),
            ..spec()
        },
        RunSpec {
            memory: Some(128 * mb),
            memory_swap: Some(64 * mb),
            ..spec()
        },
        RunSpec {
            labels: vec!["novalue".into()],
            ..spec()
        },
        RunSpec {
            labels: vec!["com.docker.compose.project=shop".into()],
            ..spec()
        },
        RunSpec {
            cap_add: vec!["NOT_A_CAP".into()],
            ..spec()
        },
        RunSpec {
            devices: vec![device("dev/dri", "rwm")],
            ..spec()
        },
        RunSpec {
            devices: vec![device("/dev/dri", "rwx")],
            ..spec()
        },
        RunSpec {
            gpus: Some(GpuSpec::Count { count: 0 }),
            ..spec()
        },
        RunSpec {
            gpus: Some(GpuSpec::Ids {
                ids: vec![" ".into()],
            }),
            ..spec()
        },
        RunSpec {
            healthcheck: Some(HealthcheckSpec {
                command: " ".into(),
                ..Default::default()
            }),
            ..spec()
        },
        RunSpec {
            healthcheck: Some(HealthcheckSpec {
                command: "true".into(),
                interval_s: Some(0),
                ..Default::default()
            }),
            ..spec()
        },
        RunSpec {
            hostname: Some("not_valid".into()),
            ..spec()
        },
        RunSpec {
            dns: vec!["dns.example".into()],
            ..spec()
        },
        RunSpec {
            extra_hosts: vec!["nas".into()],
            ..spec()
        },
        RunSpec {
            log: Some(LogSpec {
                driver: "json-file".into(),
                options: vec!["max-size".into()],
            }),
            ..spec()
        },
        RunSpec {
            log: Some(LogSpec {
                driver: " ".into(),
                options: vec![],
            }),
            ..spec()
        },
    ];
    for s in bad {
        assert!(
            matches!(create_body(&s, None), Err(ref e) if e.is(ErrorCode::Validation)),
            "{s:?}"
        );
    }
}

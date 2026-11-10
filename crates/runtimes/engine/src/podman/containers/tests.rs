use homeport_core::error::ErrorCode;

use super::*;
use crate::containers::run::{
    DeviceSpec, GpuSpec, HealthcheckSpec, LogSpec, MountSpec, PortSpec, RestartPolicy,
};

fn spec() -> RunSpec {
    RunSpec {
        image: " docker.io/library/nginx:1.29 ".into(),
        name: Some(" web-app ".into()),
        ..Default::default()
    }
}

#[test]
fn a_bare_spec_names_its_image_and_pod() {
    assert_eq!(
        create_body(&spec(), "web").unwrap(),
        json!({
            "image": "docker.io/library/nginx:1.29",
            "name": "web-app",
            "pod": "web",
        })
    );
}

#[test]
fn the_dialogs_settings_become_podmans() {
    let full = RunSpec {
        command: Some("nginx -g 'daemon off;'".into()),
        env: vec!["A=1".into(), "B=x=y".into()],
        labels: vec!["app=web".into()],
        mounts: vec![
            MountSpec {
                source: "/srv/www".into(),
                target: "/usr/share/nginx/html".into(),
                read_only: true,
            },
            MountSpec {
                source: "cache".into(),
                target: "/var/cache/nginx".into(),
                read_only: false,
            },
        ],
        restart: RestartPolicy::UnlessStopped,
        cpus: Some(0.5),
        cpu_shares: Some(512),
        memory: Some(64 * 1024 * 1024),
        cap_add: vec!["NET_ADMIN".into()],
        cap_drop: vec!["MKNOD".into()],
        devices: vec![DeviceSpec {
            host_path: "/dev/dri".into(),
            container_path: None,
            permissions: "rw".into(),
        }],
        healthcheck: Some(HealthcheckSpec {
            command: "curl -f localhost".into(),
            interval_s: Some(30),
            retries: Some(3),
            ..Default::default()
        }),
        log: Some(LogSpec {
            driver: "journald".into(),
            options: vec!["tag=web".into()],
        }),
        ..spec()
    };
    let body = create_body(&full, "web").unwrap();
    assert_eq!(body["command"], json!(["nginx", "-g", "daemon off;"]));
    assert_eq!(body["env"], json!({ "A": "1", "B": "x=y" }));
    assert_eq!(body["labels"], json!({ "app": "web" }));
    assert_eq!(
        body["mounts"],
        json!([{
            "destination": "/usr/share/nginx/html",
            "type": "bind",
            "source": "/srv/www",
            "options": ["rbind", "ro"],
        }])
    );
    assert_eq!(
        body["volumes"],
        json!([{ "Name": "cache", "Dest": "/var/cache/nginx", "Options": [] }])
    );
    assert_eq!(body["restart_policy"], "unless-stopped");
    assert_eq!(
        body["resource_limits"],
        json!({
            "cpu": { "quota": 50_000, "period": 100_000, "shares": 512 },
            "memory": { "limit": 64 * 1024 * 1024 },
        })
    );
    assert_eq!(body["cap_add"], json!(["CAP_NET_ADMIN"]));
    assert_eq!(body["cap_drop"], json!(["CAP_MKNOD"]));
    assert_eq!(body["devices"], json!([{ "path": "/dev/dri:/dev/dri:rw" }]));
    assert_eq!(
        body["healthconfig"]["Test"],
        json!(["CMD-SHELL", "curl -f localhost"])
    );
    assert_eq!(body["healthconfig"]["Interval"], 30_000_000_000_i64);
    assert_eq!(body["healthconfig"]["Retries"], 3);
    assert_eq!(
        body["log_configuration"],
        json!({ "driver": "journald", "options": { "tag": "web" } })
    );
    assert!(body.get("remove").is_none() && body.get("privileged").is_none());
}

#[test]
fn removal_on_exit_and_privileges_carry_over() {
    let body = create_body(
        &RunSpec {
            auto_remove: true,
            privileged: true,
            ..spec()
        },
        "web",
    )
    .unwrap();
    assert_eq!(body["remove"], true);
    assert_eq!(body["privileged"], true);
    assert!(body.get("restart_policy").is_none());
}

#[test]
fn what_the_pod_owns_is_refused() {
    let port = PortSpec {
        host_ip: None,
        host_port: Some(8080),
        container_port: 80,
        protocol: "tcp".into(),
    };
    let cases = [
        RunSpec {
            ports: vec![port],
            ..spec()
        },
        RunSpec {
            network: Some("podman".into()),
            ..spec()
        },
        RunSpec {
            hostname: Some("web".into()),
            ..spec()
        },
        RunSpec {
            dns: vec!["1.1.1.1".into()],
            ..spec()
        },
        RunSpec {
            extra_hosts: vec!["nas:10.0.0.2".into()],
            ..spec()
        },
        RunSpec {
            gpus: Some(GpuSpec::All),
            ..spec()
        },
        RunSpec {
            based_on: Some("abc".into()),
            ..spec()
        },
    ];
    for case in cases {
        let err = create_body(&case, "web").unwrap_err();
        assert!(err.is(ErrorCode::Validation), "{case:?}");
    }
    let ports = create_body(
        &RunSpec {
            ports: vec![PortSpec {
                host_ip: None,
                host_port: None,
                container_port: 80,
                protocol: "tcp".into(),
            }],
            ..spec()
        },
        "web",
    )
    .unwrap_err();
    assert_eq!(ports.key, "errors.pods.containerPorts");
    assert!(ports.hint.is_some());
}

#[test]
fn the_docker_checks_still_apply() {
    let bad = RunSpec {
        env: vec!["NOEQUALS".into()],
        ..spec()
    };
    assert_eq!(
        create_body(&bad, "web").unwrap_err().key,
        "errors.run.envNotKeyValue"
    );
    let none = RunSpec {
        image: " ".into(),
        ..spec()
    };
    assert!(create_body(&none, "web").is_err());
}

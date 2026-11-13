//! The tabs' pure logic, on the inputs and outputs of the React tests
//! (`health.test.ts`, `parse.test.ts`) and on the inspect document of
//! its fixtures.

use homeport_engine::containers::health::HealthConfig;

use super::{
    check_duration, format_duration, health_timing,
    inspect::{unix, Details, Inspected},
};
use crate::test_support::snapshot::in_language;

#[test]
fn unset_timings_are_the_engine_s_defaults() {
    let _english = in_language("en");
    let config = HealthConfig {
        command: "true".to_owned(),
        interval_s: Some(10),
        timeout_s: None,
        retries: Some(1),
        start_period_s: None,
    };
    assert_eq!(
        health_timing(&config),
        "every 10s, timeout 30s, 1 retry, start period 0s"
    );
}

#[test]
fn a_check_says_how_long_it_took() {
    let took = |start, end| check_duration(start, end);
    assert_eq!(
        took("2026-11-04T10:00:00.000Z", "2026-11-04T10:00:00.250Z").as_deref(),
        Some("250 ms")
    );
    assert_eq!(
        took("2026-11-04T10:00:00Z", "2026-11-04T10:00:02.5Z").as_deref(),
        Some("2.5 s")
    );
    assert_eq!(took("", "x"), None);
    // An end before its start is no duration.
    assert_eq!(took("2026-11-04T10:00:02Z", "2026-11-04T10:00:00Z"), None);
}

#[test]
fn durations_are_written_back_as_docker_run_reads_them() {
    assert_eq!(format_duration(90), "1m30s");
    assert_eq!(format_duration(7200), "2h");
    assert_eq!(format_duration(5), "5s");
    assert_eq!(format_duration(0), "0s");
    assert_eq!(format_duration(3661), "1h1m1s");
}

/// `inspectDocument('web')` of the React fixtures, with a healthcheck's
/// runs, a port that isn't published and a variable without a value.
pub fn document() -> serde_json::Value {
    serde_json::json!({
        "Id": "web-0123456789abcdef",
        "Name": "/web",
        "Created": "2026-01-02T03:04:05Z",
        "Path": "nginx",
        "Args": ["-g", "daemon off;"],
        "RestartCount": 2,
        "State": {
            "Status": "running",
            "Running": true,
            "StartedAt": "2026-01-02T03:04:06Z",
            "FinishedAt": "0001-01-01T00:00:00Z",
            "ExitCode": 0,
            "OOMKilled": false,
            "Health": {
                "Status": "unhealthy",
                "FailingStreak": 3,
                "Log": (0..7).map(|n| serde_json::json!({
                    "Start": format!("2026-01-02T03:0{n}:00Z"),
                    "End": format!("2026-01-02T03:0{n}:01Z"),
                    "ExitCode": i32::from(n >= 4),
                    "Output": format!("  check {n}\n"),
                })).collect::<Vec<_>>(),
            },
        },
        "Config": {
            "Hostname": "0123456789ab",
            "User": "",
            "Env": ["PATH=/usr/bin", "MODE=production", "EMPTY", "URL=postgres://a=b"],
            "Cmd": ["nginx", "-g", "daemon off;"],
            "Entrypoint": null,
            "Image": "nginx:1.27",
            "WorkingDir": "",
            "Labels": { "org.example.team": "web", "a.first": "1" },
            "Tty": false,
        },
        "HostConfig": {
            "RestartPolicy": { "Name": "on-failure", "MaximumRetryCount": 5 },
            "NetworkMode": "bridge",
        },
        "Mounts": [
            {
                "Type": "volume",
                "Name": "web-data",
                "Source": "/var/lib/docker/volumes/web-data/_data",
                "Destination": "/data",
                "Mode": "z",
                "RW": true,
            },
            { "Type": "bind", "Source": "/etc/nginx", "Destination": "/etc/nginx", "RW": false },
        ],
        "NetworkSettings": {
            "Ports": {
                "80/tcp": [
                    { "HostIp": "0.0.0.0", "HostPort": "8080" },
                    { "HostIp": "", "HostPort": "8081" },
                ],
                "443/tcp": null,
            },
            "Networks": {
                "bridge": {
                    "IPAddress": "172.17.0.2",
                    "Gateway": "172.17.0.1",
                    "MacAddress": "",
                    "Aliases": null,
                },
                "shop_default": { "IPAddress": "", "Gateway": "", "Aliases": ["web", "shop-web-1"] },
            },
        },
    })
}

#[test]
fn the_sections_are_read_from_the_inspect_document() {
    let details = Details::of(&document());
    assert_eq!(details.id, "web-0123456789abcdef");
    assert_eq!(details.image, "nginx:1.27");
    assert_eq!(details.created, unix("2026-01-02T03:04:05Z"));
    assert_eq!(details.started, unix("2026-01-02T03:04:06Z"));
    assert!(details.running && !details.out_of_memory);
    assert_eq!(details.command.as_deref(), Some("nginx -g daemon off;"));
    assert_eq!(
        (details.entrypoint, details.working_dir, details.user),
        (None, None, None)
    );
    assert_eq!(details.restart, Some(("on-failure".to_owned(), 5)));
    assert_eq!(details.restart_count, 2);
    assert_eq!(details.network_mode, "bridge");
    // A variable is cut at its first `=`; one without has no value.
    let env: Vec<(&str, &str)> = details
        .env
        .iter()
        .map(|pair| (pair.name.as_str(), pair.value.as_str()))
        .collect();
    assert_eq!(
        env,
        [
            ("PATH", "/usr/bin"),
            ("MODE", "production"),
            ("EMPTY", ""),
            ("URL", "postgres://a=b"),
        ]
    );
    // A named volume is called by its name, a bind by its path.
    let mounts: Vec<(&str, &str, bool)> = details
        .mounts
        .iter()
        .map(|m| (m.kind.as_str(), m.source.as_str(), m.writable))
        .collect();
    assert_eq!(
        mounts,
        [("volume", "web-data", true), ("bind", "/etc/nginx", false)]
    );
    // A row a binding; no address is every address; none is said.
    let ports: Vec<(&str, Option<&str>)> = details
        .ports
        .iter()
        .map(|p| (p.port.as_str(), p.published.as_deref()))
        .collect();
    assert_eq!(
        ports,
        [
            ("443/tcp", None),
            ("80/tcp", Some("0.0.0.0:8080")),
            ("80/tcp", Some("*:8081")),
        ]
    );
    assert_eq!(details.networks[0].ip.as_deref(), Some("172.17.0.2"));
    assert_eq!(details.networks[1].ip, None);
    assert_eq!(details.networks[1].aliases, ["web", "shop-web-1"]);
    let labels: Vec<&str> = details.labels.iter().map(|l| l.name.as_str()).collect();
    assert_eq!(labels, ["a.first", "org.example.team"], "by name");
    // The last five runs, the newest first, their output trimmed.
    let health = details.health.expect("a healthcheck");
    assert_eq!(
        (health.status.as_str(), health.failing_streak),
        ("unhealthy", 3)
    );
    let runs: Vec<&str> = health.checks.iter().map(|c| c.output.as_str()).collect();
    assert_eq!(
        runs,
        ["check 6", "check 5", "check 4", "check 3", "check 2"]
    );
    assert_eq!(health.checks[0].exit_code, 1);
    assert_eq!(health.checks[4].exit_code, 0);
}

#[test]
fn a_container_that_never_started_and_a_document_that_says_little() {
    let never = serde_json::json!({
        "State": { "Status": "created", "StartedAt": "0001-01-01T00:00:00Z", "OOMKilled": true },
        "HostConfig": { "RestartPolicy": { "Name": "", "MaximumRetryCount": 0 } },
    });
    let details = Details::of(&never);
    assert_eq!(details.started, None);
    assert!(details.out_of_memory);
    assert_eq!(details.restart, None);
    assert!(details.health.is_none() && details.env.is_empty() && details.ports.is_empty());
    // A document of another shape shows empty sections, and all of it raw.
    let odd = serde_json::json!({ "Config": "not an object", "Mounts": 3 });
    let inspected = Inspected::of(&odd);
    assert_eq!(inspected.details, Details::default());
    assert!(inspected.document.len() > 1);
}

use serde_json::json;

use super::*;

fn inputs() -> Inputs {
    Inputs {
        app: AppFacts {
            version: "1.0.0".into(),
            bundle: "deb".into(),
            os: "linux".into(),
            arch: "x86_64".into(),
            schema_version: 18,
            keychain: true,
        },
        settings: json!({
            "theme": "dark",
            "lastHostId": "h1",
            "lastSelection": { "kind": "host", "id": "h1" },
            "clusterNamespaces": { "c1": "payments" },
            "checkForUpdates": true,
        }),
        hosts: vec![
            HostFacts {
                name: "prod".into(),
                address: Some("prod.example.com:2376".into()),
                connection: "tls".into(),
                bastions: 0,
                engine: "docker".into(),
                engine_version: Some("29.1.0".into()),
                api_version: Some("1.52".into()),
                status: "connected".into(),
                error: None,
            },
            HostFacts {
                name: "laptop".into(),
                address: None,
                connection: "local".into(),
                bastions: 0,
                engine: "podman".into(),
                engine_version: None,
                api_version: None,
                status: "unreachable".into(),
                error: Some(ErrorCode::EngineNotRunning),
            },
        ],
        clusters: vec![ClusterFacts {
            name: "kind-dev".into(),
            address: "https://10.1.2.3:6443".into(),
            kind: "kubernetes".into(),
            version: Some("v1.33.1".into()),
            status: "connected".into(),
            error: None,
        }],
        stacks: StackCounts {
            managed: 3,
            linked: 1,
            synced: 2,
        },
    }
}

#[test]
fn names_and_addresses_are_left_out_unless_asked_for() {
    let (summary, aliases) = summary(inputs(), false);
    let text = serde_json::to_string_pretty(&summary).unwrap();
    for private in [
        "prod",
        "laptop",
        "kind-dev",
        "example.com",
        "10.1.2.3",
        "payments",
    ] {
        assert!(!text.contains(private), "{private} in {text}");
    }
    assert_eq!(summary.hosts[0].name, "host-1");
    assert_eq!(summary.hosts[1].name, "host-2");
    assert_eq!(summary.clusters[0].name, "cluster-1");
    assert!(!aliases.is_empty());
}

#[test]
fn names_and_addresses_are_kept_when_asked_for() {
    let (summary, aliases) = summary(inputs(), true);
    assert_eq!(summary.hosts[0].name, "prod");
    assert_eq!(
        summary.hosts[0].address.as_deref(),
        Some("prod.example.com:2376")
    );
    assert_eq!(
        summary.clusters[0].address.as_deref(),
        Some("https://10.1.2.3:6443")
    );
    assert!(summary.names_included);
    assert!(aliases.is_empty());
}

#[test]
fn it_says_what_runs_where_and_why_a_host_is_down() {
    let (summary, _) = summary(inputs(), false);
    let value = serde_json::to_value(&summary).unwrap();
    assert_eq!(value["app"]["version"], "1.0.0");
    assert_eq!(value["app"]["schemaVersion"], 18);
    assert_eq!(value["hosts"][0]["engineVersion"], "29.1.0");
    assert_eq!(value["hosts"][0]["connection"], "tls");
    assert_eq!(value["hosts"][1]["error"], "engine_not_running");
    assert_eq!(value["clusters"][0]["kind"], "kubernetes");
    assert_eq!(
        value["stacks"],
        json!({ "managed": 3, "linked": 1, "synced": 2 })
    );
}

#[test]
fn settings_lose_the_last_selection_and_ids() {
    let (summary, _) = summary(inputs(), true);
    assert_eq!(
        summary.settings,
        json!({ "theme": "dark", "clusterNamespaces": 1, "checkForUpdates": true })
    );
}

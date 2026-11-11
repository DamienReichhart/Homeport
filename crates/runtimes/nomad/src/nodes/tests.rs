use super::*;
use crate::api::client::testing::{agent, ok};

const NODES: &str = r#"[
  { "ID": "f00d0001-0000-0000-0000-000000000000", "Name": "client-2", "Datacenter": "dc1",
    "NodePool": "default", "NodeClass": "", "Address": "10.0.0.12", "Version": "1.10.3",
    "Drain": true, "SchedulingEligibility": "ineligible", "Status": "ready", "StatusDescription": "" },
  { "ID": "f00d0002-0000-0000-0000-000000000000", "Name": "client-1", "Datacenter": "dc1",
    "NodePool": "default", "NodeClass": "gpu", "Address": "10.0.0.11", "Version": "1.10.3",
    "Drain": false, "SchedulingEligibility": "eligible", "Status": "ready", "StatusDescription": "" }
]"#;

const PLACEMENTS: &str = r#"[
  { "NodeID": "f00d0002-0000-0000-0000-000000000000", "ClientStatus": "running" },
  { "NodeID": "f00d0002-0000-0000-0000-000000000000", "ClientStatus": "running" },
  { "NodeID": "f00d0001-0000-0000-0000-000000000000", "ClientStatus": "complete" }
]"#;

const NODE: &str = r#"{
  "ID": "f00d0001-0000-0000-0000-000000000000", "Name": "client-2", "Datacenter": "dc1",
  "NodePool": "default", "HTTPAddr": "10.0.0.12:4646", "Status": "ready",
  "SchedulingEligibility": "ineligible",
  "DrainStrategy": { "Deadline": 3600000000000, "ForceDeadline": "2026-11-09T11:00:00Z",
    "IgnoreSystemJobs": true, "StartedAt": "2026-11-09T10:00:00Z" },
  "LastDrain": { "Status": "draining" },
  "Attributes": { "os.name": "ubuntu", "kernel.name": "linux", "nomad.version": "1.10.3", "secret.thing": "x" },
  "Drivers": { "docker": { "Detected": true, "Healthy": true }, "java": { "Detected": false, "Healthy": false } }
}"#;

const NODE_ALLOCATIONS: &str = r#"[
  { "ID": "a1", "Name": "web.api[0]", "Namespace": "default", "JobID": "web", "NodeID": "f00d0001",
    "TaskGroup": "api", "ClientStatus": "running", "DesiredStatus": "run", "Job": { "Version": 2 },
    "CreateTime": 1762000000000000000 }
]"#;

#[tokio::test]
async fn nodes_are_listed_by_name_with_their_running_allocations() {
    let fake = agent(vec![
        ok("GET /v1/nodes", NODES),
        ok("GET /v1/allocations", PLACEMENTS),
    ])
    .await;
    let nodes = fake.client(None).nodes().await.unwrap();
    assert_eq!(nodes[0].name, "client-1");
    assert_eq!(nodes[0].running, Some(2));
    assert!(nodes[0].eligible && !nodes[0].draining);
    assert_eq!(nodes[0].node_class.as_deref(), Some("gpu"));
    assert_eq!(nodes[1].running, Some(0));
    assert!(nodes[1].draining && !nodes[1].eligible);
    let target = &fake.received()[1].target;
    assert!(
        target.contains("namespace=%2A") || target.contains("namespace=*"),
        "{target}"
    );
}

#[tokio::test]
async fn nodes_are_listed_without_counts_when_the_token_cant_see_allocations() {
    let fake = agent(vec![
        ok("GET /v1/nodes", NODES),
        (
            "GET /v1/allocations",
            403,
            vec![],
            "Permission denied".into(),
        ),
    ])
    .await;
    let nodes = fake.client(Some("t")).nodes().await.unwrap();
    assert!(nodes.iter().all(|n| n.running.is_none()));
}

#[tokio::test]
async fn a_nodes_detail_shows_its_drain_drivers_and_allocations() {
    let fake = agent(vec![
        ok("GET /v1/node/f00d0001-0000-0000-0000-000000000000", NODE),
        ok(
            "GET /v1/node/f00d0001-0000-0000-0000-000000000000/allocations",
            NODE_ALLOCATIONS,
        ),
    ])
    .await;
    let detail = fake
        .client(None)
        .node("f00d0001-0000-0000-0000-000000000000")
        .await
        .unwrap();
    assert_eq!(
        detail.drain,
        Some(NomadDrain {
            deadline: Some("2026-11-09T11:00:00Z".into()),
            forced: false,
            ignore_system_jobs: true,
            started: Some("2026-11-09T10:00:00Z".into()),
        })
    );
    assert!(detail.node.draining && !detail.node.eligible);
    assert_eq!(detail.node.running, Some(1));
    assert_eq!(detail.node.version.as_deref(), Some("1.10.3"));
    assert_eq!(detail.drivers.len(), 2);
    assert!(!detail.attributes.iter().any(|(k, _)| k == "secret.thing"));
    assert_eq!(detail.allocations[0].job_version, 2);
}

#[test]
fn a_drain_sends_its_deadline_in_nanoseconds_and_a_cancel_none() {
    let start = drain_body(
        Some(&NomadDrainSpec {
            deadline_secs: Some(3600),
            ignore_system_jobs: true,
        }),
        false,
    );
    assert_eq!(start["DrainSpec"]["Deadline"], 3_600_000_000_000_i64);
    assert_eq!(start["DrainSpec"]["IgnoreSystemJobs"], true);
    let no_deadline = drain_body(
        Some(&NomadDrainSpec {
            deadline_secs: None,
            ignore_system_jobs: false,
        }),
        false,
    );
    assert_eq!(no_deadline["DrainSpec"]["Deadline"], 0);
    let cancel = drain_body(None, true);
    assert!(cancel["DrainSpec"].is_null());
    assert_eq!(cancel["MarkEligible"], true);
}

#[tokio::test]
async fn eligibility_is_sent_by_name() {
    let fake = agent(vec![ok("POST /v1/node/n1/eligibility", "{}")]).await;
    fake.client(None)
        .set_eligibility("n1", false)
        .await
        .unwrap();
    let body: Value = serde_json::from_str(&fake.received()[0].body).unwrap();
    assert_eq!(body["Eligibility"], "ineligible");
}

#[tokio::test]
async fn a_token_without_node_write_gets_the_rule_it_needs() {
    let fake = agent(vec![(
        "POST /v1/node/n1/drain",
        403,
        vec![],
        "Permission denied".into(),
    )])
    .await;
    let e = fake
        .client(Some("t"))
        .drain_node(
            "n1",
            Some(&NomadDrainSpec {
                deadline_secs: Some(3600),
                ignore_system_jobs: false,
            }),
            false,
        )
        .await
        .unwrap_err();
    assert!(e.is(ErrorCode::NomadPermissionDenied));
    assert_eq!(e.key, "errors.nomad.nodeWriteDenied");
    assert!(e.hint.unwrap().commands[0].contains("policy = \"write\""));
}

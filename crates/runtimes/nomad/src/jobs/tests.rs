use super::*;
use crate::api::client::testing::{agent, ok};

const JOBS: &str = r#"[
  { "ID": "web", "ParentID": "", "Name": "web", "Namespace": "default", "Type": "service",
    "Priority": 50, "Periodic": false, "ParameterizedJob": false, "Stop": false,
    "Status": "running", "StatusDescription": "", "SubmitTime": 1762000000000000000,
    "JobSummary": { "Summary": { "api": { "Queued": 0, "Complete": 0, "Failed": 1,
      "Running": 2, "Starting": 1, "Lost": 0, "Unknown": 0 } } } },
  { "ID": "backup", "ParentID": "", "Name": "backup", "Namespace": "default", "Type": "batch",
    "Priority": 50, "Periodic": true, "ParameterizedJob": false, "Stop": false,
    "Status": "running", "StatusDescription": "", "SubmitTime": 0,
    "JobSummary": { "Summary": {} } }
]"#;

const WEB: &str = r#"{
  "ID": "web", "Name": "web", "Namespace": "default", "Type": "service", "Priority": 50,
  "Status": "running", "Stop": false, "Version": 3, "SubmitTime": 1762000000000000000,
  "Datacenters": ["dc1"], "NodePool": "default", "JobModifyIndex": 77,
  "TaskGroups": [ { "Name": "api", "Count": 3,
    "Scaling": { "Min": 1, "Max": 5, "Enabled": true },
    "Tasks": [ { "Name": "server", "Driver": "docker" } ] } ]
}"#;

const SUMMARY: &str = r#"{ "Summary": { "api": { "Running": 2, "Starting": 1, "Failed": 1 } } }"#;

const ALLOCATIONS: &str = r#"[
  { "ID": "a1b2c3d4-0000-0000-0000-000000000001", "Name": "web.api[0]", "Namespace": "default",
    "JobID": "web", "NodeID": "n1", "NodeName": "client-1", "TaskGroup": "api",
    "ClientStatus": "running", "DesiredStatus": "run", "JobVersion": 3,
    "CreateTime": 1762000000000000000, "ModifyTime": 1762000060000000000,
    "TaskStates": { "server": { "State": "running", "Failed": false, "Restarts": 2,
      "StartedAt": "2025-11-01T12:26:41Z" } } },
  { "ID": "a1b2c3d4-0000-0000-0000-000000000002", "Name": "web.api[1]", "Namespace": "default",
    "JobID": "web", "NodeID": "n2", "NodeName": "client-2", "TaskGroup": "api",
    "ClientStatus": "pending", "DesiredStatus": "run", "JobVersion": 3,
    "CreateTime": 1762000100000000000, "ModifyTime": 1762000100000000000,
    "TaskStates": { "server": { "State": "pending", "StartedAt": "0001-01-01T00:00:00Z" } } }
]"#;

#[tokio::test]
async fn jobs_are_listed_with_their_running_and_desired_counts() {
    let fake = agent(vec![
        ok("GET /v1/jobs", JOBS),
        ok("GET /v1/job/web", WEB),
        ("GET /v1/job/backup", 404, vec![], "job not found".into()),
    ])
    .await;
    let jobs = fake.client(None).jobs(None).await.unwrap();
    assert_eq!(
        jobs.iter().map(|j| j.name.as_str()).collect::<Vec<_>>(),
        ["backup", "web"]
    );
    let web = &jobs[1];
    assert_eq!(web.job_type, "service");
    assert_eq!(web.submitted.as_deref(), Some("2025-11-01T12:26:40Z"));
    assert_eq!(
        web.groups,
        [NomadGroupCounts {
            name: "api".into(),
            desired: Some(3),
            running: 2,
            starting: 1,
            queued: 0,
            failed: 1,
            lost: 0,
            complete: 0,
        }]
    );
    assert!(jobs[0].periodic && jobs[0].submitted.is_none());
}

#[tokio::test]
async fn the_namespace_goes_in_the_query() {
    let fake = agent(vec![ok("GET /v1/jobs", "[]")]).await;
    fake.client(None).jobs(Some("*")).await.unwrap();
    fake.client(None).jobs(None).await.unwrap();
    let sent = fake.received();
    assert!(sent[0].target.contains("namespace=%2A") || sent[0].target.contains("namespace=*"));
    assert!(!sent[1].target.contains("namespace"));
}

#[tokio::test]
async fn a_jobs_detail_has_its_groups_scaling_and_allocations_newest_first() {
    let fake = agent(vec![
        ok("GET /v1/job/web", WEB),
        ok("GET /v1/job/web/summary", SUMMARY),
        ok("GET /v1/job/web/allocations", ALLOCATIONS),
    ])
    .await;
    let detail = fake.client(None).job(None, "web").await.unwrap();
    assert_eq!(detail.version, 3);
    assert_eq!(detail.node_pool.as_deref(), Some("default"));
    let group = &detail.groups[0];
    assert_eq!(group.counts.desired, Some(3));
    assert_eq!(
        group.scaling,
        Some(NomadScaling {
            min: Some(1),
            max: Some(5),
            enabled: true
        })
    );
    assert_eq!(group.tasks[0].driver, "docker");
    assert_eq!(detail.allocations[0].name, "web.api[1]");
    assert_eq!(
        detail.allocations[0].tasks[0].started, None,
        "Nomad's zero time"
    );
    assert_eq!(detail.allocations[1].tasks[0].restarts, 2);
}

#[tokio::test]
async fn a_missing_job_is_named() {
    let fake = agent(vec![(
        "GET /v1/job/gone",
        404,
        vec![],
        "job not found".into(),
    )])
    .await;
    let e = fake.client(None).job(None, "gone").await.unwrap_err();
    assert!(e.is(ErrorCode::NomadJobNotFound));
    assert!(e.message.contains("gone"));
    assert!(e.hint.is_some());
}

#[tokio::test]
async fn a_token_that_cant_read_jobs_says_so() {
    let fake = agent(vec![(
        "GET /v1/jobs",
        403,
        vec![],
        "Permission denied".into(),
    )])
    .await;
    let e = fake.client(Some("t")).jobs(None).await.unwrap_err();
    assert!(e.is(ErrorCode::NomadPermissionDenied));
    assert_eq!(e.key, "errors.nomad.readDenied");
}

#[tokio::test]
async fn stopping_purges_only_when_asked() {
    let fake = agent(vec![ok("DELETE /v1/job/web", r#"{"EvalID":"e1"}"#)]).await;
    let nomad = fake.client(None);
    nomad.stop_job(None, "web", false).await.unwrap();
    nomad.stop_job(Some("shop"), "web", true).await.unwrap();
    let sent = fake.received();
    assert!(!sent[0].target.contains("purge"));
    assert!(sent[1].target.contains("purge=true") && sent[1].target.contains("namespace=shop"));
}

#[tokio::test]
async fn scaling_sends_the_group_and_count() {
    let fake = agent(vec![
        ok("GET /v1/job/web", WEB),
        ok("POST /v1/job/web/scale", r#"{"EvalID":"e1"}"#),
    ])
    .await;
    fake.client(None)
        .scale_task_group(None, "web", "api", 4)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_str(&fake.received()[1].body).unwrap();
    assert_eq!(body["Count"], 4);
    assert_eq!(body["Target"]["Group"], "api");
}

#[tokio::test]
async fn a_count_outside_the_scaling_policy_is_refused_before_asking_nomad() {
    let fake = agent(vec![ok("GET /v1/job/web", WEB)]).await;
    let nomad = fake.client(None);
    let above = nomad
        .scale_task_group(None, "web", "api", 6)
        .await
        .unwrap_err();
    assert!(above.is(ErrorCode::NomadScaleOutOfBounds));
    assert!(above.message.contains('5'), "{above:?}");
    let below = nomad
        .scale_task_group(None, "web", "api", 0)
        .await
        .unwrap_err();
    assert_eq!(below.key, "errors.nomad.scaleBelowMin");
    assert!(fake.received().iter().all(|r| r.method == "GET"));
}

#[tokio::test]
async fn nomads_refusal_to_scale_keeps_its_reason() {
    let fake = agent(vec![
        ok("GET /v1/job/web", WEB),
        (
            "POST /v1/job/web/scale",
            400,
            vec![],
            "group \"api\" has no count to scale".into(),
        ),
    ])
    .await;
    let e = fake
        .client(None)
        .scale_task_group(None, "web", "api", 2)
        .await
        .unwrap_err();
    assert!(e.is(ErrorCode::BadRequest));
    assert!(e.detail.unwrap().contains("no count"));
    assert_eq!(e.hint.unwrap().key, "errors.nomad.scaleRefused.hint");
}

/// What Nomad 2.1 answers when a job is re-run, then scaled at once.
#[tokio::test]
async fn scaling_during_a_deployment_says_to_wait_for_it() {
    let fake = agent(vec![
        ok("GET /v1/job/web", WEB),
        (
            "POST /v1/job/web/scale",
            400,
            vec![],
            "job scaling blocked due to active deployment".into(),
        ),
    ])
    .await;
    let e = fake
        .client(None)
        .scale_task_group(None, "web", "api", 2)
        .await
        .unwrap_err();
    assert!(e.is(ErrorCode::Conflict));
    assert_eq!(e.key, "errors.nomad.scaleDeploying");
    assert!(e.message.contains("web"));
}

use super::*;
use crate::api::client::testing::{agent, ok};

const PARSED: &str = r#"{ "ID": "web", "Name": "web", "Type": "service",
  "TaskGroups": [ { "Name": "api", "Count": 2 } ] }"#;

const PLAN: &str = r#"{
  "JobModifyIndex": 77,
  "Warnings": "",
  "Diff": { "Type": "Edited", "TaskGroups": [ {
    "Name": "api", "Type": "Edited",
    "Fields": [ { "Type": "Edited", "Name": "Count", "Old": "2", "New": "3" } ],
    "Objects": null,
    "Tasks": [ { "Name": "server", "Type": "Edited",
      "Fields": null,
      "Objects": [ { "Type": "Edited", "Name": "Config",
        "Fields": [ { "Type": "Edited", "Name": "image", "Old": "nginx:1.27", "New": "nginx:1.28" },
                    { "Type": "None", "Name": "ports", "Old": "http", "New": "http" } ] } ] } ]
  } ] },
  "Annotations": { "DesiredTGUpdates": { "api": { "Place": 1, "Stop": 0,
    "InPlaceUpdate": 0, "DestructiveUpdate": 2, "Migrate": 0, "Canary": 0, "Ignore": 0 } } },
  "FailedTGAllocs": { "api": { "NodesEvaluated": 3, "NodesFiltered": 1, "NodesExhausted": 2,
    "DimensionExhausted": { "memory": 2 }, "ConstraintFiltered": { "${attr.kernel.name} = linux": 1 },
    "CoalescedFailures": 0 } }
}"#;

#[test]
fn the_format_is_told_by_the_first_character() {
    assert_eq!(SpecFormat::detect("  {\"Job\": {}}"), SpecFormat::Json);
    assert_eq!(SpecFormat::detect("job \"web\" {}"), SpecFormat::Hcl2);
    assert_eq!(
        SpecFormat::detect("# a comment\njob \"web\" {}"),
        SpecFormat::Hcl2
    );
}

#[test]
fn nomads_parse_errors_name_their_line_and_column() {
    assert_eq!(
        parse_position("HTTP 400: error parsing: input.hcl:3,5-9: Unsupported argument; An argument named \"cont\" is not expected here."),
        Some((3, 5))
    );
    assert_eq!(parse_position("HTTP 400: something else"), None);
}

#[test]
fn json_jobs_are_taken_bare_or_wrapped_in_job() {
    let wrapped = job_of_json(serde_json::from_str(r#"{"Job": {"ID": "web"}}"#).unwrap()).unwrap();
    assert_eq!(wrapped["ID"], "web");
    let bare =
        job_of_json(serde_json::from_str(r#"{"ID": "web", "TaskGroups": []}"#).unwrap()).unwrap();
    assert_eq!(bare["ID"], "web");
    let e = job_of_json(serde_json::from_str(r#"{"job": {"web": {}}}"#).unwrap()).unwrap_err();
    assert!(e.is(ErrorCode::NomadParseFailed));
}

#[test]
fn a_plan_is_summarised_by_task_group() {
    let groups = plan_groups(&serde_json::from_str(PLAN).unwrap());
    assert_eq!(groups.len(), 1);
    let api = &groups[0];
    assert_eq!(
        (api.change.as_str(), api.place, api.destructive),
        ("Edited", 1, 2)
    );
    assert_eq!(
        api.fields,
        [
            NomadFieldChange {
                path: "Count".into(),
                change: "Edited".into(),
                old: Some("2".into()),
                new: Some("3".into()),
            },
            NomadFieldChange {
                path: "Task \"server\" › Config › image".into(),
                change: "Edited".into(),
                old: Some("nginx:1.27".into()),
                new: Some("nginx:1.28".into()),
            },
        ]
    );
    let failure = api.failure.as_ref().unwrap();
    assert_eq!(failure.exhausted, ["memory (2)"]);
    assert_eq!(failure.nodes_evaluated, 3);
}

#[tokio::test]
async fn hcl_is_parsed_by_nomad_and_points_at_the_line_it_names() {
    let fake = agent(vec![(
        "POST /v1/jobs/parse",
        400,
        vec![],
        "error parsing: input.hcl:4,7-11: Unsupported argument".into(),
    )])
    .await;
    let e = fake
        .client(None)
        .parse_job("job \"web\" {\n}\n")
        .await
        .unwrap_err();
    assert!(e.is(ErrorCode::NomadParseFailed));
    assert_eq!(e.params["line"].to_string(), "4");
    assert_eq!(e.params["column"].to_string(), "7");
    let sent: Value = serde_json::from_str(&fake.received()[0].body).unwrap();
    assert_eq!(sent["Canonicalize"], false);
    assert!(sent["JobHCL"].as_str().unwrap().starts_with("job"));
}

#[tokio::test]
async fn json_that_doesnt_parse_points_at_its_line() {
    let fake = agent(vec![]).await;
    let e = fake
        .client(None)
        .parse_job("{\n  \"Job\": {\n    \"ID\": \"web\",,\n  }\n}")
        .await
        .unwrap_err();
    assert_eq!(e.key, "errors.nomad.parseFailedAt");
    assert_eq!(e.params["line"].to_string(), "3");
}

#[tokio::test]
async fn a_job_without_a_namespace_is_planned_in_the_one_selected() {
    let fake = agent(vec![
        ok("POST /v1/jobs/parse", PARSED),
        ok("POST /v1/job/web/plan", PLAN),
    ])
    .await;
    let nomad = fake.client(None);
    let (job, format) = nomad.parse_job("job \"web\" {}").await.unwrap();
    let plan = nomad.plan_job(job, format, Some("shop")).await.unwrap();
    assert_eq!(plan.namespace, "shop");
    assert_eq!(plan.job["Namespace"], "shop");
    assert_eq!(plan.job_modify_index, 77);
    assert!(!plan.new_job);
    assert!(plan
        .placement
        .as_ref()
        .unwrap()
        .is(ErrorCode::NomadPlacementFailed));
    let sent = &fake.received()[1];
    assert!(sent.target.contains("namespace=shop"));
    let body: Value = serde_json::from_str(&sent.body).unwrap();
    assert_eq!(body["Diff"], true);
}

#[tokio::test]
async fn a_job_naming_its_namespace_keeps_it() {
    let fake = agent(vec![ok(
        "POST /v1/job/web/plan",
        r#"{"JobModifyIndex": 0, "Diff": {"Type": "Added"}}"#,
    )])
    .await;
    let job = serde_json::json!({ "ID": "web", "Namespace": "billing" });
    let plan = fake
        .client(None)
        .plan_job(job, SpecFormat::Json, Some("shop"))
        .await
        .unwrap();
    assert_eq!(plan.namespace, "billing");
    assert!(plan.new_job);
    assert!(plan.placement.is_none());
}

fn to_run(index: u32) -> NomadPlanToRun {
    NomadPlanToRun {
        job_id: "web".into(),
        namespace: "default".into(),
        format: SpecFormat::Hcl2,
        job: serde_json::json!({ "ID": "web" }),
        job_modify_index: index,
    }
}

#[tokio::test]
async fn running_enforces_the_planned_index_and_keeps_the_source() {
    let fake = agent(vec![ok(
        "POST /v1/jobs",
        r#"{"EvalID": "e1", "Warnings": ""}"#,
    )])
    .await;
    let result = fake
        .client(None)
        .run_job(&to_run(77), Some("job \"web\" {}"))
        .await
        .unwrap();
    assert_eq!(result.eval_id.as_deref(), Some("e1"));
    let body: Value = serde_json::from_str(&fake.received()[0].body).unwrap();
    assert_eq!(body["EnforceIndex"], true);
    assert_eq!(body["JobModifyIndex"], 77);
    assert_eq!(body["Submission"]["Format"], "hcl2");
    assert_eq!(body["Submission"]["Source"], "job \"web\" {}");
}

#[tokio::test]
async fn a_job_changed_since_its_plan_is_not_overwritten() {
    let fake = agent(vec![(
        "POST /v1/jobs",
        500,
        vec![],
        "Enforcing job modify index 77: job exists with conflicting job modify index: 80".into(),
    )])
    .await;
    let e = fake
        .client(None)
        .run_job(&to_run(77), None)
        .await
        .unwrap_err();
    assert!(e.is(ErrorCode::NomadJobModified));
    assert_eq!(e.key, "errors.nomad.jobModified");
    let fresh = fake
        .client(None)
        .run_job(&to_run(0), None)
        .await
        .unwrap_err();
    assert_eq!(fresh.key, "errors.nomad.jobExists");
}

#[tokio::test]
async fn a_jobs_source_is_the_submitted_one_when_nomad_kept_it() {
    let fake = agent(vec![
        ok("GET /v1/job/web", r#"{"ID": "web", "Version": 4}"#),
        ok(
            "GET /v1/job/web/submission",
            r#"{"Source": "job \"web\" {}", "Format": "hcl2"}"#,
        ),
    ])
    .await;
    let source = fake.client(None).job_source(None, "web").await.unwrap();
    assert!(source.submitted);
    assert_eq!(source.format, SpecFormat::Hcl2);
    assert!(fake.received()[1].target.contains("version=4"));
}

#[tokio::test]
async fn without_a_submission_the_source_is_the_jobs_json() {
    let fake = agent(vec![ok(
        "GET /v1/job/web",
        r#"{"ID": "web", "Version": 4}"#,
    )])
    .await;
    let source = fake.client(None).job_source(None, "web").await.unwrap();
    assert!(!source.submitted);
    assert_eq!(source.format, SpecFormat::Json);
    let json: Value = serde_json::from_str(&source.source).unwrap();
    assert_eq!(json["Job"]["ID"], "web");
}

use k8s_openapi::api::{apps::v1::Deployment, batch::v1::Job, core::v1::Pod};

use super::*;

fn from<K: DeserializeOwned>(json: serde_json::Value) -> K {
    serde_json::from_value(json).unwrap()
}

fn deployment(replicas: i32, ready: i32, updated: i32, progressing: &str) -> Deployment {
    from(serde_json::json!({
        "metadata": { "name": "web", "namespace": "shop", "creationTimestamp": "2026-11-01T10:00:00Z" },
        "spec": {
            "replicas": replicas,
            "selector": { "matchLabels": { "app": "web", "tier": "front" } },
            "template": { "spec": { "containers": [
                { "name": "web", "image": "nginx:1.27", "ports": [{ "containerPort": 80 }] }
            ] } }
        },
        "status": {
            "readyReplicas": ready,
            "updatedReplicas": updated,
            "conditions": [{ "type": "Progressing", "status": progressing, "reason": "ProgressDeadlineExceeded" }]
        }
    }))
}

#[test]
fn a_deployments_health_follows_its_replicas_and_rollout() {
    let ready = workloads::deployment(&deployment(3, 3, 3, "True"));
    assert_eq!(
        (ready.ready, ready.desired, ready.health),
        (3, 3, WorkloadHealth::Ready)
    );
    assert_eq!(ready.images, ["nginx:1.27"]);
    assert_eq!(ready.created.as_deref(), Some("2026-11-01T10:00:00Z"));
    assert_eq!(
        workloads::deployment(&deployment(3, 1, 3, "True")).health,
        WorkloadHealth::Progressing
    );
    assert_eq!(
        workloads::deployment(&deployment(3, 3, 1, "True")).health,
        WorkloadHealth::Progressing,
        "an old ReplicaSet still serves"
    );
    assert_eq!(
        workloads::deployment(&deployment(3, 1, 3, "False")).health,
        WorkloadHealth::Degraded
    );
    assert_eq!(
        workloads::deployment(&deployment(0, 0, 0, "True")).health,
        WorkloadHealth::Paused
    );
    let detail = workloads::deployment_detail(&deployment(3, 3, 3, "True"));
    assert_eq!(detail.selector, "app=web,tier=front");
    assert_eq!(detail.containers[0].ports, ["80/TCP"]);
    assert_eq!(
        detail.conditions[0].reason.as_deref(),
        Some("ProgressDeadlineExceeded")
    );
}

#[test]
fn jobs_are_done_failed_or_running() {
    let job = |conditions: serde_json::Value, succeeded: i32| -> Job {
        from(serde_json::json!({
            "metadata": { "name": "migrate", "namespace": "shop",
                "ownerReferences": [{ "apiVersion": "batch/v1", "kind": "CronJob", "name": "nightly", "uid": "u", "controller": true }] },
            "spec": { "completions": 1, "template": { "spec": { "containers": [{ "name": "m", "image": "app" }] } } },
            "status": { "succeeded": succeeded, "conditions": conditions }
        }))
    };
    let done = workloads::job(&job(
        serde_json::json!([{ "type": "Complete", "status": "True" }]),
        1,
    ));
    assert_eq!(
        (done.ready, done.desired, done.health),
        (1, 1, WorkloadHealth::Ready)
    );
    assert_eq!(done.owner.unwrap().name, "nightly");
    let failed = workloads::job(&job(
        serde_json::json!([{ "type": "Failed", "status": "True" }]),
        0,
    ));
    assert_eq!(failed.health, WorkloadHealth::Failed);
    let running = workloads::job(&job(serde_json::json!([]), 0));
    assert_eq!(running.health, WorkloadHealth::Progressing);
}

fn pod(status: serde_json::Value) -> Pod {
    from(serde_json::json!({
        "metadata": { "name": "web-1", "namespace": "shop",
            "ownerReferences": [{ "apiVersion": "apps/v1", "kind": "ReplicaSet", "name": "web-5d", "uid": "u", "controller": true }] },
        "spec": { "nodeName": "node-a", "containers": [
            { "name": "web", "image": "nginx" }, { "name": "proxy", "image": "envoy" }
        ] },
        "status": status
    }))
}

#[test]
fn a_pods_status_reads_like_kubectls() {
    let running = pods::view(&pod(serde_json::json!({
        "phase": "Running", "podIP": "10.1.0.4",
        "containerStatuses": [
            { "name": "web", "ready": true, "restartCount": 0, "image": "nginx", "imageID": "", "state": { "running": {} } },
            { "name": "proxy", "ready": true, "restartCount": 2, "image": "envoy", "imageID": "", "state": { "running": {} } }
        ]
    })));
    assert_eq!(
        (running.status.as_str(), running.health),
        ("Running", PodHealth::Running)
    );
    assert_eq!(
        (running.ready, running.containers, running.restarts),
        (2, 2, 2)
    );
    assert_eq!(running.node.as_deref(), Some("node-a"));
    assert_eq!(running.owner.unwrap().kind, "ReplicaSet");

    let crashing = pods::view(&pod(serde_json::json!({
        "phase": "Running",
        "containerStatuses": [
            { "name": "web", "ready": false, "restartCount": 7, "image": "nginx", "imageID": "",
              "state": { "waiting": { "reason": "CrashLoopBackOff" } },
              "lastState": { "terminated": { "exitCode": 1, "reason": "Error" } } }
        ]
    })));
    assert_eq!(
        (crashing.status.as_str(), crashing.health),
        ("CrashLoopBackOff", PodHealth::Crashing)
    );

    let init = pods::view(&pod(serde_json::json!({
        "phase": "Pending",
        "initContainerStatuses": [
            { "name": "migrate", "ready": false, "restartCount": 0, "image": "app", "imageID": "",
              "state": { "waiting": { "reason": "ImagePullBackOff" } } }
        ]
    })));
    assert_eq!(init.status, "Init:ImagePullBackOff");
    assert_eq!(init.health, PodHealth::Crashing);

    let done = pods::view(&pod(serde_json::json!({
        "phase": "Succeeded",
        "containerStatuses": [
            { "name": "web", "ready": false, "restartCount": 0, "image": "nginx", "imageID": "",
              "state": { "terminated": { "exitCode": 0, "reason": "Completed" } } }
        ]
    })));
    assert_eq!(
        (done.status.as_str(), done.health),
        ("Succeeded", PodHealth::Succeeded)
    );
}

#[test]
fn a_pods_detail_lists_init_containers_first_with_their_state() {
    let mut p = pod(serde_json::json!({
        "phase": "Running", "qosClass": "Burstable",
        "containerStatuses": [
            { "name": "web", "ready": true, "restartCount": 1, "image": "nginx", "imageID": "",
              "state": { "running": { "startedAt": "2026-11-01T10:00:00Z" } },
              "lastState": { "terminated": { "exitCode": 137, "reason": "OOMKilled" } } }
        ]
    }));
    p.spec.as_mut().unwrap().init_containers = Some(vec![from(
        serde_json::json!({ "name": "migrate", "image": "app" }),
    )]);
    let detail = pods::detail(&p);
    let names: Vec<_> = detail
        .containers
        .iter()
        .map(|c| (c.name.as_str(), c.init))
        .collect();
    assert_eq!(names, [("migrate", true), ("web", false), ("proxy", false)]);
    let web = &detail.containers[1];
    assert_eq!(
        (web.state.as_str(), web.ready, web.restarts),
        ("running", true, 1)
    );
    assert_eq!(web.last_reason.as_deref(), Some("OOMKilled"));
    assert_eq!(detail.qos_class.as_deref(), Some("Burstable"));
}

#[test]
fn yaml_leaves_out_managed_fields_and_secret_values() {
    let mut secret = serde_json::json!({
        "apiVersion": "v1", "kind": "Secret",
        "metadata": { "name": "db", "managedFields": [{ "manager": "kubectl" }],
            "annotations": { "kubectl.kubernetes.io/last-applied-configuration": "{\"data\":{\"password\":\"aHVudGVyMg==\"}}" } },
        "data": { "password": "aHVudGVyMg==" },
        "stringData": { "user": "admin" }
    });
    tidy(&mut secret);
    let text = serde_yaml_ng::to_string(&secret).unwrap();
    assert!(!text.contains("managedFields"));
    assert!(!text.contains("aHVudGVyMg=="), "{text}");
    assert!(!text.contains("admin"));
    assert!(text.contains("password: (hidden)"));
}

#[test]
fn forbidden_lists_name_the_verb_resource_and_namespace() {
    let e = forbidden("list", ResourceKind::Secret, Some("shop"));
    assert!(e.is(ErrorCode::Forbidden));
    assert_eq!(e.message, "Your credentials can't list secrets in shop.");
    assert_eq!(
        e.hint.unwrap().commands,
        ["kubectl auth can-i list secrets --namespace shop"]
    );
    let e = forbidden("list", ResourceKind::Pod, None);
    assert_eq!(
        e.message,
        "Your credentials can't list pods across all namespaces."
    );
    let e = forbidden("list", ResourceKind::Node, Some("shop"));
    assert_eq!(e.message, "Your credentials can't list nodes.");
}

#[test]
fn events_put_warnings_first_then_the_newest() {
    let event = |kind: &str, last: &str| EventView {
        namespace: "shop".into(),
        name: format!("{kind}-{last}"),
        kind: kind.into(),
        reason: String::new(),
        message: String::new(),
        object: EventObject {
            kind: "Pod".into(),
            namespace: Some("shop".into()),
            name: "web".into(),
        },
        count: 1,
        first_seen: None,
        last_seen: Some(last.into()),
        source: None,
    };
    let mut list = vec![
        event("Normal", "2026-11-01T10:05:00Z"),
        event("Warning", "2026-11-01T10:00:00Z"),
        event("Normal", "2026-11-01T10:10:00Z"),
    ];
    events::sort(&mut list);
    let order: Vec<_> = list.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(
        order,
        [
            "Warning-2026-11-01T10:00:00Z",
            "Normal-2026-11-01T10:10:00Z",
            "Normal-2026-11-01T10:05:00Z"
        ]
    );
}

//! Pods against a fake Podman answering with JSON as Podman 5 writes it.

use super::*;
use crate::podman::testing;

/// Trimmed from Podman 5.8's `GET /libpod/pods/json`.
const LISTED: &str = r#"[
  {
    "Cgroup": "user.slice",
    "Containers": [
      { "Id": "c2", "Names": "web-app", "Status": "running", "RestartCount": 0 },
      { "Id": "infra1", "Names": "a1b2-infra", "Status": "running", "RestartCount": 0 },
      { "Id": "c1", "Names": "web-db", "Status": "exited", "RestartCount": 0 }
    ],
    "Created": "2026-11-08T10:00:00.123456789Z",
    "Id": "p1",
    "InfraId": "infra1",
    "Name": "web",
    "Namespace": "",
    "Networks": ["podman"],
    "Status": "Degraded",
    "Labels": { "app": "web" }
  },
  {
    "Containers": [],
    "Created": "2026-11-07T10:00:00Z",
    "Id": "p0",
    "InfraId": "",
    "Name": "batch",
    "Networks": null,
    "Status": "Created",
    "Labels": null
  }
]"#;

/// Trimmed from Podman 5.8's `GET /libpod/pods/web/json`.
const INSPECTED: &str = r#"{
  "Id": "p1",
  "Name": "web",
  "Created": "2026-11-08T10:00:00.123456789Z",
  "CreateCommand": ["podman", "pod", "create", "--name", "web", "-p", "8080:80"],
  "ExitPolicy": "continue",
  "State": "Running",
  "Hostname": "web",
  "Labels": {},
  "CreateCgroup": true,
  "CreateInfra": true,
  "InfraContainerID": "infra1",
  "InfraConfig": {
    "PortBindings": {
      "80/tcp": [{ "HostIp": "", "HostPort": "8080" }],
      "53/udp": [{ "HostIp": "127.0.0.1", "HostPort": "5353" }]
    },
    "HostNetwork": false,
    "Networks": ["podman"]
  },
  "SharedNamespaces": ["ipc", "net", "uts"],
  "NumContainers": 2,
  "Containers": [
    { "Id": "c2", "Name": "web-app", "State": "running" },
    { "Id": "infra1", "Name": "a1b2-infra", "State": "running" }
  ]
}"#;

#[tokio::test]
async fn lists_pods_by_name_with_their_containers_infra_first() {
    let fake = testing::podman("5.8.1", |_| (200, LISTED.into()));
    let pods = list(&fake.podman).await.unwrap();
    assert_eq!(pods.len(), 2);
    let (batch, web) = (&pods[0], &pods[1]);
    assert_eq!(batch.name, "batch");
    assert_eq!(batch.infra_id, None);
    assert!(batch.containers.is_empty() && batch.labels.is_empty());

    assert_eq!(web.status, "degraded");
    assert_eq!(web.infra_id.as_deref(), Some("infra1"));
    assert_eq!(web.networks, ["podman"]);
    assert_eq!(web.labels.get("app").map(String::as_str), Some("web"));
    let names: Vec<_> = web
        .containers
        .iter()
        .map(|c| (c.name.as_str(), c.infra))
        .collect();
    assert_eq!(
        names,
        [("a1b2-infra", true), ("web-app", false), ("web-db", false)]
    );
    assert_eq!(web.containers[2].state, "exited");
    assert_eq!(fake.requests()[0].path, "/v5.0.0/libpod/pods/json");
}

#[tokio::test]
async fn an_empty_list_can_be_null() {
    let fake = testing::podman("4.9.4", |_| (200, "null".into()));
    assert!(list(&fake.podman).await.unwrap().is_empty());
}

#[tokio::test]
async fn inspects_a_pod_with_its_ports_and_namespaces() {
    let fake = testing::podman("5.8.1", |_| (200, INSPECTED.into()));
    let pod = inspect(&fake.podman, "web").await.unwrap();
    assert_eq!(pod.pod.status, "running");
    assert_eq!(pod.shared_namespaces, ["ipc", "net", "uts"]);
    assert_eq!(pod.hostname, "web");
    assert_eq!(pod.create_command[2], "create");
    assert_eq!(
        pod.ports,
        [
            PodPort {
                container_port: 53,
                protocol: "udp".into(),
                host_ip: Some("127.0.0.1".into()),
                host_port: Some(5353),
            },
            PodPort {
                container_port: 80,
                protocol: "tcp".into(),
                host_ip: None,
                host_port: Some(8080),
            },
        ]
    );
    assert!(pod.pod.containers[0].infra);
    assert_eq!(fake.requests()[0].path, "/v5.0.0/libpod/pods/web/json");
}

#[tokio::test]
async fn a_pod_that_is_gone_says_so() {
    let fake = testing::podman("5.8.1", |_| {
        (
            404,
            r#"{"cause":"no such pod","message":"no pod with name or ID web found: no such pod","response":404}"#.into(),
        )
    });
    let err = inspect(&fake.podman, "web").await.unwrap_err();
    assert!(err.is(ErrorCode::NotFound));
    assert_eq!(err.key, "errors.pods.gone");
    let err = act(&fake.podman, "web", PodAction::Start)
        .await
        .unwrap_err();
    assert_eq!(err.key, "errors.pods.gone");
}

#[tokio::test]
async fn actions_post_to_the_pods_verb() {
    let fake = testing::podman("5.8.1", |_| (200, r#"{"Errs":[],"Id":"p1"}"#.into()));
    for (action, verb) in [
        (PodAction::Start, "start"),
        (PodAction::Stop, "stop"),
        (PodAction::Restart, "restart"),
        (PodAction::Pause, "pause"),
        (PodAction::Unpause, "unpause"),
    ] {
        act(&fake.podman, "p1", action).await.unwrap();
        let last = fake.requests().pop().unwrap();
        assert_eq!(last.method, "POST");
        assert_eq!(last.path, format!("/v5.0.0/libpod/pods/p1/{verb}"));
    }
}

#[tokio::test]
async fn actions_report_what_podman_refused() {
    let already = testing::podman("5.8.1", |_| (304, String::new()));
    let err = act(&already.podman, "p1", PodAction::Start)
        .await
        .unwrap_err();
    assert!(err.is(ErrorCode::AlreadyDone));

    let partly = testing::podman("5.8.1", |_| {
        (
            409,
            r#"{"Errs":["starting container c1: exec format error"],"Id":"p1"}"#.into(),
        )
    });
    let err = act(&partly.podman, "p1", PodAction::Start)
        .await
        .unwrap_err();
    assert!(err.is(ErrorCode::Conflict));
    assert!(err.message.contains("start every container"), "{err:?}");
    assert!(err.detail.unwrap().contains("exec format error"));
}

#[tokio::test]
async fn removing_counts_the_containers_that_went_with_it() {
    let fake = testing::podman("5.8.1", |_| {
        (
            200,
            r#"{"Err":null,"Id":"p1","RemovedCtrs":{"c1":null,"c2":null,"infra1":null}}"#.into(),
        )
    });
    assert_eq!(remove(&fake.podman, "p1", true).await.unwrap(), 3);
    let req = fake.requests().pop().unwrap();
    assert_eq!(req.method, "DELETE");
    assert_eq!(req.path, "/v5.0.0/libpod/pods/p1?force=true");
}

#[tokio::test]
async fn a_running_pod_needs_force_to_be_removed() {
    let fake = testing::podman("5.8.1", |_| {
        (
            500,
            r#"{"cause":"container state improper","message":"not all containers could be removed from pod p1: removing pod containers: cannot remove container c2 as it is running - running or paused containers cannot be removed without force: container state improper","response":500}"#.into(),
        )
    });
    let err = remove(&fake.podman, "p1", false).await.unwrap_err();
    assert!(err.is(ErrorCode::PodRunning), "{err:?}");
    assert!(err.hint.is_some());
}

#[tokio::test]
async fn creates_a_pod_with_its_ports_and_labels() {
    let fake = testing::podman("5.8.1", |_| (201, r#"{"Id":"p9"}"#.into()));
    let spec = PodSpec {
        name: " web ".into(),
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
                protocol: "UDP".into(),
            },
        ],
        labels: vec!["app=web".into()],
    };
    assert_eq!(create(&fake.podman, &spec).await.unwrap(), "p9");
    let req = fake.requests().pop().unwrap();
    assert_eq!(req.path, "/v5.0.0/libpod/pods/create");
    assert_eq!(
        req.json(),
        serde_json::json!({
            "name": "web",
            "labels": { "app": "web" },
            "portmappings": [
                { "container_port": 80, "host_port": 8080, "protocol": "tcp" },
                { "container_port": 53, "host_ip": "127.0.0.1", "protocol": "udp" }
            ]
        })
    );
}

#[tokio::test]
async fn a_taken_name_is_named() {
    let fake = testing::podman("5.8.1", |_| {
        (
            409,
            r#"{"cause":"pod already exists","message":"adding pod to state: name \"web\" is in use: pod already exists","response":409}"#.into(),
        )
    });
    let spec = PodSpec {
        name: "web".into(),
        ..Default::default()
    };
    let err = create(&fake.podman, &spec).await.unwrap_err();
    assert!(err.is(ErrorCode::NameInUse));
    assert!(err.message.contains("web"));
}

#[test]
fn pod_specs_are_checked_before_sending() {
    for name in ["", "-web", "my pod", "wé"] {
        let spec = PodSpec {
            name: name.into(),
            ..Default::default()
        };
        assert!(
            create_body(&spec).unwrap_err().is(ErrorCode::Validation),
            "{name}"
        );
    }
    let bad_label = PodSpec {
        name: "web".into(),
        labels: vec!["nokey".into()],
        ..Default::default()
    };
    assert!(create_body(&bad_label).is_err());
    let bad_protocol = PodSpec {
        name: "web".into(),
        ports: vec![PortSpec {
            host_ip: None,
            host_port: None,
            container_port: 80,
            protocol: "icmp".into(),
        }],
        ..Default::default()
    };
    assert!(create_body(&bad_protocol).is_err());
}

#[tokio::test]
#[ignore = "needs a local Podman Engine"]
async fn creates_runs_into_lists_and_removes_a_pod_on_podman() {
    let Some((podman, docker)) = testing::local().await else {
        return;
    };
    crate::testing::ensure_image(&docker).await;
    let name = crate::testing::unique("pod");
    let spec = PodSpec {
        name: name.clone(),
        ports: vec![PortSpec {
            host_ip: Some("127.0.0.1".into()),
            host_port: None,
            container_port: 8080,
            protocol: "tcp".into(),
        }],
        labels: vec!["test=homeport".into()],
    };
    let id = create(&podman, &spec).await.unwrap();
    let run = crate::containers::run::RunSpec {
        image: crate::testing::IMAGE.into(),
        name: Some(format!("{name}-app")),
        command: Some("sleep 60".into()),
        ..Default::default()
    };
    let result = async {
        crate::podman::containers::run_in_pod(&docker, &podman, run, &name, None).await?;
        let pods = list(&podman).await?;
        let pod = pods.iter().find(|p| p.id == id).expect("listed");
        assert_eq!(pod.status, "running");
        assert_eq!(pod.containers.len(), 2, "{pod:?}");
        assert!(pod.containers[0].infra);
        let detail = inspect(&podman, &name).await?;
        assert_eq!(detail.ports.len(), 1);
        assert!(detail.ports[0].host_port.is_some());
        act(&podman, &id, PodAction::Stop).await?;
        assert_eq!(inspect(&podman, &id).await?.pod.status, "exited");
        AppResult::Ok(())
    }
    .await;
    let removed = remove(&podman, &id, true).await;
    result.unwrap();
    assert_eq!(removed.unwrap(), 2);
}

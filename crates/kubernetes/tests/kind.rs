//! Against a real kind cluster: `kind create cluster --name homeport`, then
//! `cargo test -p homeport-clusters --test kind -- --ignored`. The context
//! defaults to `kind-homeport`; set `HOMEPORT_TEST_CONTEXT` for another.
//! Without that context in the kubeconfig (as in CI, which runs the
//! ignored Engine tests), it says so and passes.

use homeport_kubernetes::{
    client,
    kubeconfig::{self, Source},
    resources::{On, ResourceKind, WorkloadKind},
    KubeCluster,
};

fn context() -> String {
    std::env::var("HOMEPORT_TEST_CONTEXT").unwrap_or_else(|_| "kind-homeport".into())
}

/// The test cluster, or `None` (and why) when the kubeconfig doesn't reach it.
fn connect() -> Option<On> {
    let Some((connection, secrets)) = Source::local()
        .ok()
        .and_then(|source| kubeconfig::resolve(&source, &context()).ok())
    else {
        eprintln!("skipped: no kubeconfig context {}", context());
        return None;
    };
    let cluster = KubeCluster {
        id: "test".into(),
        name: context(),
        connection,
    };
    Some(On {
        client: client::client(&cluster, &secrets, None).unwrap(),
        cluster,
    })
}

#[tokio::test]
#[ignore = "needs a kind cluster"]
async fn a_kind_cluster_is_browsed() {
    let Some(on) = connect() else { return };
    let namespaces = on.namespaces().await.unwrap();
    assert!(namespaces.iter().any(|n| n.name == "kube-system"));
    let system = on.pods(Some("kube-system"), None, None).await.unwrap();
    assert!(system.iter().any(|p| p.name.starts_with("kube-apiserver")));
    let coredns = on
        .workloads(WorkloadKind::Deployment, Some("kube-system"))
        .await
        .unwrap();
    let coredns = coredns.iter().find(|d| d.name == "coredns").unwrap();
    assert!(coredns.desired >= 1);
    let detail = on
        .workload(WorkloadKind::Deployment, "kube-system", "coredns")
        .await
        .unwrap();
    let pods = on
        .pods(Some("kube-system"), Some(&detail.selector), None)
        .await
        .unwrap();
    assert_eq!(pods.len() as u32, coredns.desired);
    let yaml = on
        .yaml(ResourceKind::Deployment, Some("kube-system"), "coredns")
        .await
        .unwrap();
    assert!(!yaml.contains("managedFields"));

    let types = on.discover_for(Some("kube-system")).await.unwrap();
    let pods = types.iter().find(|t| t.api.plural == "pods").unwrap();
    assert!(pods.short_names.contains(&"po".to_owned()));
    let table = on.api_list(&pods.api, Some("kube-system")).await.unwrap();
    assert!(table.columns.iter().any(|c| c.name == "Status"));
    assert!(table
        .rows
        .iter()
        .any(|r| r.name.starts_with("kube-apiserver")));
}

#[tokio::test]
#[ignore = "needs a kind cluster"]
async fn a_pod_gives_its_logs_and_a_terminal_then_is_deleted() {
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

    use homeport_core::{
        logs::{LogEvent, LogOptions},
        terminal::ExecEvent,
    };
    use homeport_kubernetes::{
        logs::{PodLogSource, PodLogTarget},
        terminal::{self, PodExecOptions},
    };
    use k8s_openapi::api::core::v1::Pod;
    use kube::api::{Api, PostParams};

    let Some(on) = connect() else { return };
    let name = format!("homeport-test-{}", std::process::id());
    let pod: Pod = serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": { "name": name },
        "spec": { "containers": [{
            "name": "box",
            "image": "busybox:1.37",
            "command": ["sh", "-c", "echo started; sleep 300"],
        }] },
    }))
    .unwrap();
    let pods: Api<Pod> = Api::namespaced(on.client.clone(), "default");
    pods.create(&PostParams::default(), &pod).await.unwrap();
    for _ in 0..120 {
        let detail = on.pod("default", &name).await.unwrap();
        if detail.view.status == "Running" {
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    let events = Arc::new(Mutex::new(vec![]));
    let sink = {
        let events = events.clone();
        move |e| {
            events.lock().unwrap().push(e);
            true
        }
    };
    let options = LogOptions {
        tail: None,
        timestamps: false,
        since: None,
        stdout: true,
        stderr: true,
        follow: false,
    };
    on.clone()
        .stream_logs(
            "default".into(),
            vec![PodLogSource {
                pod: name.clone(),
                label: None,
            }],
            PodLogTarget::default(),
            options,
            sink,
        )
        .await;
    let text: Vec<String> = events
        .lock()
        .unwrap()
        .iter()
        .flat_map(|e| match e {
            LogEvent::Lines { lines } => lines.iter().map(|l| l.text.clone()).collect(),
            _ => vec![],
        })
        .collect();
    assert_eq!(text, ["started"]);

    let (session, process) = on
        .exec(
            "default",
            &name,
            PodExecOptions {
                cols: 80,
                rows: 24,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let pump = tokio::spawn(terminal::pump(process, move |e| tx.send(e).is_ok()));
    session.resize(100, 30).await.unwrap();
    session.write(b"echo hel''lo; exit 3\n").await.unwrap();
    let mut output = String::new();
    let code = tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            match rx.recv().await.unwrap() {
                ExecEvent::Output { data } => output.push_str(&data),
                ExecEvent::Exit { code, .. } => return code,
                ExecEvent::Error { error } => panic!("{error}"),
            }
        }
    })
    .await
    .unwrap();
    pump.await.unwrap();
    assert!(output.contains("hello"), "{output}");
    assert_eq!(code, Some(3));

    on.delete(
        &ResourceKind::Pod.api_type(),
        Some("default"),
        &name,
        Some(0),
    )
    .await
    .unwrap();
}

#[tokio::test]
#[ignore = "needs a kind cluster"]
async fn a_forwarded_port_reaches_the_pod_until_stopped() {
    use std::{sync::Arc, time::Duration};

    use homeport_kubernetes::forward::{ForwardTarget, PortForwards};
    use k8s_openapi::api::core::v1::Pod;
    use kube::api::{Api, PostParams};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let Some(on) = connect() else { return };
    let name = format!("homeport-forward-{}", std::process::id());
    let pod: Pod = serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": { "name": name },
        "spec": { "containers": [{
            "name": "web",
            "image": "busybox:1.37",
            "command": ["sh", "-c", "echo hello > /tmp/index.html; httpd -f -p 8080 -h /tmp"],
            "ports": [{ "containerPort": 8080 }],
        }] },
    }))
    .unwrap();
    let pods: Api<Pod> = Api::namespaced(on.client.clone(), "default");
    pods.create(&PostParams::default(), &pod).await.unwrap();
    for _ in 0..120 {
        if on.pod("default", &name).await.unwrap().view.status == "Running" {
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    let forwards = PortForwards::new(Arc::new(|_| {}));
    let target = ForwardTarget::Pod {
        namespace: "default".into(),
        name: name.clone(),
    };
    let forward = forwards
        .start(on.clone(), target, 8080, None)
        .await
        .unwrap();
    assert!(forward
        .addresses
        .iter()
        .all(|a| a.starts_with("127.0.0.1:") || a.starts_with("[::1]:")));
    let mut socket = tokio::net::TcpStream::connect(("127.0.0.1", forward.local_port))
        .await
        .unwrap();
    socket
        .write_all(b"GET /index.html HTTP/1.0\r\n\r\n")
        .await
        .unwrap();
    let mut answer = String::new();
    socket.read_to_string(&mut answer).await.unwrap();
    assert!(answer.contains("hello"), "{answer}");

    forwards.stop(forward.id);
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(
        tokio::net::TcpStream::connect(("127.0.0.1", forward.local_port))
            .await
            .is_err()
    );
    on.delete(
        &ResourceKind::Pod.api_type(),
        Some("default"),
        &name,
        Some(0),
    )
    .await
    .unwrap();
}

#[tokio::test]
#[ignore = "needs a kind cluster"]
async fn applying_the_same_manifest_twice_changes_nothing_the_second_time() {
    use homeport_kubernetes::apply::ApplyAction;

    let Some(on) = connect() else { return };
    let name = format!("homeport-apply-{}", std::process::id());
    let yaml = format!(
        "apiVersion: apps/v1\nkind: Deployment\nmetadata:\n  name: {name}\nspec:\n  replicas: 0\n  selector:\n    matchLabels: {{ app: {name} }}\n  template:\n    metadata:\n      labels: {{ app: {name} }}\n    spec:\n      containers:\n      - name: web\n        image: busybox:1.37\n"
    );
    let preview = on.apply_preview(&yaml, Some("default")).await.unwrap();
    assert_eq!(preview.documents[0].action, ApplyAction::Created);
    // A dry run changes nothing.
    assert!(on
        .workload(WorkloadKind::Deployment, "default", &name)
        .await
        .is_err());
    let applied = on.apply(&yaml, Some("default"), false).await.unwrap();
    assert_eq!(applied.documents[0].action, ApplyAction::Created);
    let again = on.apply_preview(&yaml, Some("default")).await.unwrap();
    assert_eq!(again.documents[0].action, ApplyAction::Unchanged);

    let edited = on
        .edit_yaml(&ResourceKind::Deployment.api_type(), Some("default"), &name)
        .await
        .unwrap()
        .replace("replicas: 0", "replicas: 1");
    let preview = on.apply_preview(&edited, Some("default")).await.unwrap();
    assert_eq!(preview.documents[0].action, ApplyAction::Changed);

    on.delete(
        &ResourceKind::Deployment.api_type(),
        Some("default"),
        &name,
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[ignore = "needs a kind cluster and kubectl"]
async fn kubectl_reaches_the_cluster_through_the_private_proxy() {
    let Some(on) = connect() else { return };
    let Some(kubectl) = homeport_core::programs::find("kubectl") else {
        eprintln!("skipped: no kubectl on this machine");
        return;
    };
    let client = on.client.clone();
    let access = homeport_kubernetes::proxy::ClusterAccess {
        cluster: on.cluster.clone(),
        clients: std::sync::Arc::new(move || Ok(client.clone())),
    };
    let command = homeport_kubernetes::proxy::CliCommand {
        program: kubectl,
        args: vec!["get".into(), "ns".into(), "-o".into(), "name".into()],
        ..Default::default()
    };
    let out = homeport_kubernetes::proxy::output(&command, Some(&access), |f| {
        panic!("kubectl failed: {f:?}")
    })
    .await
    .unwrap();
    assert!(out.lines().any(|l| l == "namespace/kube-system"), "{out}");
}

#[tokio::test]
#[ignore = "needs a kind cluster"]
async fn metrics_come_from_metrics_server_or_say_it_isnt_installed() {
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

    use homeport_kubernetes::metrics::{MetricsEvent, MetricsTarget, EVERY};

    let Some(on) = connect() else { return };
    let served = on.serves_metrics().await.unwrap();
    eprintln!("serves the Metrics API: {served}");
    let events: Arc<Mutex<Vec<MetricsEvent>>> = Arc::default();
    let sink = {
        let events = events.clone();
        move |e| {
            events.lock().unwrap().push(e);
            false
        }
    };
    tokio::time::timeout(
        Duration::from_secs(30),
        on.poll_metrics(MetricsTarget::Nodes, EVERY, sink),
    )
    .await
    .unwrap();
    let events = events.lock().unwrap();
    match &events[..] {
        [MetricsEvent::NotInstalled] => assert!(!served),
        [MetricsEvent::Nodes { nodes }] => {
            assert!(served);
            assert!(nodes.iter().all(|n| n.usage.memory > 0), "{nodes:?}");
        }
        // metrics-server just installed, still starting.
        [MetricsEvent::NotResponding { .. }] => assert!(served),
        other => panic!("{other:?}"),
    }
}

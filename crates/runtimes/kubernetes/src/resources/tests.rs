use k8s_openapi::api::{apps::v1::Deployment, batch::v1::Job, core::v1::Pod};

use super::kinds::{config, events, network, nodes, pods, workloads};
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
    assert_eq!(
        e.message,
        "Your credentials don't allow “list” on secrets in shop."
    );
    assert_eq!(
        e.hint.unwrap().commands,
        ["kubectl auth can-i list secrets --namespace shop"]
    );
    let e = forbidden("list", ResourceKind::Pod, None);
    assert_eq!(
        e.message,
        "Your credentials don't allow “list” on pods across all namespaces."
    );
    let e = forbidden("list", ResourceKind::Node, Some("shop"));
    assert_eq!(e.message, "Your credentials don't allow “list” on nodes.");
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

#[test]
fn a_secrets_list_view_carries_no_value() {
    let secret: k8s_openapi::api::core::v1::Secret = from(serde_json::json!({
        "metadata": { "name": "db", "namespace": "shop" },
        "type": "Opaque",
        "data": { "password": "aHVudGVyMg==", "cert": "/w==" }
    }));
    let view = config::secret(&secret);
    let json = serde_json::to_string(&view).unwrap();
    assert!(
        !json.contains("aHVudGVyMg==") && !json.contains("hunter2"),
        "{json}"
    );
    let detail = serde_json::to_string(&config::secret_detail(&secret)).unwrap();
    assert!(!detail.contains("hunter2") && !detail.contains("aHVudGVyMg=="));
    assert_eq!(
        view.keys
            .iter()
            .map(|k| (k.key.as_str(), k.size, k.binary))
            .collect::<Vec<_>>(),
        [("cert", 1, true), ("password", 7, false)]
    );
    let revealed = config::value(&secret, "password").unwrap();
    assert_eq!(
        (revealed.value.as_str(), revealed.binary),
        ("hunter2", false)
    );
    assert!(
        !format!("{revealed:?}").contains("hunter2"),
        "Debug hides it"
    );
    assert_eq!(config::value(&secret, "cert").unwrap().value, "/w==");
    assert!(config::value(&secret, "missing").is_none());
}

#[test]
fn service_ports_and_external_addresses_read_like_kubectl() {
    let svc: k8s_openapi::api::core::v1::Service = from(serde_json::json!({
        "metadata": { "name": "web", "namespace": "shop" },
        "spec": {
            "type": "LoadBalancer", "clusterIP": "10.96.0.12",
            "selector": { "app": "web" },
            "ports": [
                { "name": "http", "port": 80, "targetPort": "http", "nodePort": 30080, "protocol": "TCP" },
                { "port": 9090, "targetPort": 9090 }
            ]
        },
        "status": { "loadBalancer": { "ingress": [{ "ip": "203.0.113.9" }] } }
    }));
    let view = network::service(&svc);
    assert_eq!(view.kind, "LoadBalancer");
    assert_eq!(view.ports, ["80:30080/TCP", "9090/TCP"]);
    assert_eq!(view.external, ["203.0.113.9"]);
    assert_eq!(view.selector, "app=web");
    let detail = network::service_detail(&svc);
    assert_eq!(detail.ports[0].target_port, "http");
    assert_eq!(detail.ports[1].target_port, "9090");
    let headless: k8s_openapi::api::core::v1::Service = from(serde_json::json!({
        "metadata": { "name": "db" }, "spec": { "clusterIP": "None" }
    }));
    assert_eq!(network::service(&headless).cluster_ip, None);
}

#[test]
fn ingress_rules_list_each_path_and_its_backend() {
    let ing: k8s_openapi::api::networking::v1::Ingress = from(serde_json::json!({
        "metadata": { "name": "web", "namespace": "shop" },
        "spec": {
            "ingressClassName": "nginx",
            "defaultBackend": { "service": { "name": "fallback", "port": { "number": 80 } } },
            "tls": [{ "hosts": ["shop.example.com"], "secretName": "shop-tls" }],
            "rules": [{ "host": "shop.example.com", "http": { "paths": [
                { "path": "/api", "pathType": "Prefix", "backend": { "service": { "name": "api", "port": { "name": "http" } } } },
                { "path": "/", "pathType": "Prefix", "backend": { "service": { "name": "web", "port": { "number": 8080 } } } }
            ] } }]
        },
        "status": { "loadBalancer": { "ingress": [{ "hostname": "lb.example.com" }] } }
    }));
    let view = network::ingress(&ing);
    assert_eq!(view.class.as_deref(), Some("nginx"));
    assert_eq!(view.hosts, ["shop.example.com"]);
    assert_eq!(view.addresses, ["lb.example.com"]);
    assert!(view.tls);
    let detail = network::ingress_detail(&ing);
    let paths: Vec<_> = detail
        .paths
        .iter()
        .map(|p| {
            (
                p.host.as_str(),
                p.path.as_str(),
                p.service.as_deref(),
                p.port.as_deref(),
            )
        })
        .collect();
    assert_eq!(
        paths,
        [
            ("*", "*", Some("fallback"), Some("80")),
            ("shop.example.com", "/api", Some("api"), Some("http")),
            ("shop.example.com", "/", Some("web"), Some("8080")),
        ]
    );
    assert_eq!(detail.tls[0].secret.as_deref(), Some("shop-tls"));
}

#[test]
fn a_nodes_readiness_roles_and_capacity_are_read() {
    let node: k8s_openapi::api::core::v1::Node = from(serde_json::json!({
        "metadata": { "name": "node-a", "labels": {
            "node-role.kubernetes.io/control-plane": "", "kubernetes.io/os": "linux" } },
        "spec": { "unschedulable": true, "taints": [{ "key": "node-role.kubernetes.io/control-plane", "effect": "NoSchedule" }] },
        "status": {
            "conditions": [{ "type": "Ready", "status": "True" }, { "type": "MemoryPressure", "status": "False" }],
            "addresses": [{ "type": "InternalIP", "address": "10.0.0.4" }, { "type": "Hostname", "address": "node-a" }],
            "allocatable": { "cpu": "4", "memory": "16302048Ki", "pods": "110" },
            "capacity": { "cpu": "4" },
            "nodeInfo": { "kubeletVersion": "v1.33.1", "operatingSystem": "linux", "architecture": "arm64",
                "osImage": "Debian", "kernelVersion": "6.1", "containerRuntimeVersion": "containerd://2.0",
                "bootID": "", "machineID": "", "systemUUID": "", "kubeProxyVersion": "" }
        }
    }));
    let view = nodes::view(&node);
    assert!(view.ready && view.unschedulable);
    assert_eq!(view.roles, ["control-plane"]);
    assert_eq!(
        (view.version.as_str(), view.arch.as_str()),
        ("v1.33.1", "arm64")
    );
    assert_eq!(view.internal_ip.as_deref(), Some("10.0.0.4"));
    assert_eq!((view.cpu.as_deref(), view.taints), (Some("4"), 1));
    assert_eq!(
        (view.cpu_millicores, view.memory_bytes),
        (Some(4000), Some(16_693_297_152))
    );
    let detail = nodes::detail(&node);
    assert_eq!(detail.allocatable["pods"], "110");
    assert_eq!(detail.taints[0].effect, "NoSchedule");
    assert_eq!(detail.addresses, ["InternalIP 10.0.0.4", "Hostname node-a"]);
    let not_ready: k8s_openapi::api::core::v1::Node = from(serde_json::json!({
        "metadata": { "name": "node-b" },
        "status": { "conditions": [{ "type": "Ready", "status": "Unknown" }] }
    }));
    assert!(!nodes::view(&not_ready).ready);
}

#[test]
fn a_pod_sums_what_its_containers_request_and_are_limited_to() {
    let pod = |containers: serde_json::Value, init: serde_json::Value| -> Pod {
        from(serde_json::json!({
            "metadata": { "name": "web-1", "namespace": "shop" },
            "spec": { "containers": containers, "initContainers": init }
        }))
    };
    let both = pod(
        serde_json::json!([
            { "name": "web", "resources": {
                "requests": { "cpu": "250m", "memory": "256Mi" },
                "limits": { "cpu": "1", "memory": "512Mi" } } },
            { "name": "proxy", "resources": {
                "requests": { "cpu": "50m", "memory": "64Mi" },
                "limits": { "cpu": "200m", "memory": "128Mi" } } }
        ]),
        serde_json::json!([
            // A sidecar runs alongside the containers; a plain init container doesn't.
            { "name": "log-shipper", "restartPolicy": "Always", "resources": {
                "requests": { "cpu": "10m" }, "limits": { "cpu": "100m", "memory": "32Mi" } } },
            { "name": "migrate", "resources": { "requests": { "cpu": "2" } } }
        ]),
    );
    let view = pods::view(&both);
    assert_eq!(
        view.cpu,
        ResourceBounds {
            request: Some(310),
            limit: Some(1300)
        }
    );
    assert_eq!(
        view.memory,
        ResourceBounds {
            request: Some(320 * 1024 * 1024),
            limit: Some(672 * 1024 * 1024)
        }
    );
    let detail = pods::detail(&both);
    let web = detail.containers.iter().find(|c| c.name == "web").unwrap();
    assert_eq!(web.cpu.request, Some(250));
    assert_eq!(web.memory.limit, Some(512 * 1024 * 1024));

    // One container without a limit may use the whole Node: so may the Pod.
    let partly = pods::view(&pod(
        serde_json::json!([
            { "name": "web", "resources": { "limits": { "memory": "512Mi" }, "requests": { "memory": "1Gi" } } },
            { "name": "proxy" }
        ]),
        serde_json::json!([]),
    ));
    assert_eq!(partly.memory.limit, None);
    assert_eq!(partly.memory.request, Some(1024 * 1024 * 1024));
    assert_eq!(partly.cpu, ResourceBounds::default());
}

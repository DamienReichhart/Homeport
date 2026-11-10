//! The Metrics API's answers as metrics-server sends them, served by a fake
//! API server in memory: what's read, and when polling stops.

use std::{
    convert::Infallible,
    future::Future,
    sync::{Arc, Mutex},
    time::Duration,
};

use bytes::Bytes;
use http_body_util::Full;
use serde_json::{json, Value};

use homeport_core::{error::ErrorCode, streams::Streams};

use super::*;
use crate::model::{Cluster, ClusterConnection, ClusterKind, KubernetesConnection};

/// A tick short enough for tests.
const TICK: Duration = Duration::from_millis(10);
const PACE: Pace = Pace {
    every: TICK,
    wait: Duration::from_secs(2),
};

/// The RFC 3339 time `seconds` after 2026-11-10 10:00:00 UTC, as metrics-server writes it.
fn at(seconds: i64) -> String {
    jiff::Timestamp::from_second(START + seconds)
        .unwrap()
        .to_string()
}

/// 2026-11-10 10:00:00 UTC.
const START: i64 = 1_794_304_800;

/// `/apis` of a Cluster that serves the Metrics API, or not.
fn groups(metrics: bool) -> Value {
    let group = |name: &str, version: &str| {
        let gv = format!("{name}/{version}");
        json!({ "name": name, "versions": [{ "groupVersion": gv, "version": version }],
                "preferredVersion": { "groupVersion": gv, "version": version } })
    };
    let mut list = vec![group("apps", "v1"), group("batch", "v1")];
    if metrics {
        list.push(group("metrics.k8s.io", "v1beta1"));
    }
    json!({ "kind": "APIGroupList", "apiVersion": "v1", "groups": list })
}

/// A `PodMetrics` as metrics-server sends it, read at `time`.
fn pod_metrics(name: &str, time: &str, web: (&str, &str), proxy: (&str, &str)) -> Value {
    json!({
        "kind": "PodMetrics", "apiVersion": "metrics.k8s.io/v1beta1",
        "metadata": { "name": name, "namespace": "shop", "creationTimestamp": "2026-11-10T10:00:20Z",
                      "labels": { "app": "web" } },
        "timestamp": time, "window": "15.022s",
        "containers": [
            { "name": "web", "usage": { "cpu": web.0, "memory": web.1 } },
            { "name": "proxy", "usage": { "cpu": proxy.0, "memory": proxy.1 } }
        ]
    })
}

fn node_metrics(name: &str, time: &str, cpu: &str, memory: &str) -> Value {
    json!({
        "metadata": { "name": name, "creationTimestamp": "2026-11-10T10:00:20Z" },
        "timestamp": time, "window": "20.031s",
        "usage": { "cpu": cpu, "memory": memory }
    })
}

fn list(kind: &str, items: Vec<Value>) -> Value {
    json!({ "kind": kind, "apiVersion": "metrics.k8s.io/v1beta1", "metadata": {}, "items": items })
}

fn status(code: u16, reason: &str, message: &str) -> Value {
    json!({ "kind": "Status", "apiVersion": "v1", "status": "Failure",
            "message": message, "reason": reason, "code": code })
}

/// The paths the fake API server was asked, in order.
type Seen = Arc<Mutex<Vec<String>>>;

/// A connected Cluster whose API server answers each request with
/// `answer(path, how many requests came before)`.
fn fake(answer: impl Fn(&str, usize) -> (u16, Value) + Send + Sync + 'static) -> (On, Seen) {
    let seen: Seen = Arc::default();
    let log = seen.clone();
    let answer = Arc::new(answer);
    let service = tower::service_fn(move |request: http::Request<kube::client::Body>| {
        let path = request.uri().path().to_owned();
        let before = {
            let mut paths = log.lock().unwrap();
            paths.push(path.clone());
            paths.len() - 1
        };
        let (code, body) = answer(&path, before);
        async move {
            Ok::<_, Infallible>(
                http::Response::builder()
                    .status(code)
                    .header("content-type", "application/json")
                    .body(Full::new(Bytes::from(body.to_string())))
                    .unwrap(),
            )
        }
    });
    let cluster = Cluster {
        id: "c".into(),
        name: "kind-dev".into(),
        kind: ClusterKind::Kubernetes,
        connection: ClusterConnection::Kubernetes(KubernetesConnection {
            server: "https://127.0.0.1:6443".into(),
            ..Default::default()
        }),
        enabled: true,
        created_at: String::new(),
    };
    let on = On {
        client: kube::Client::new(service, "default"),
        cluster,
    };
    (on, seen)
}

/// The events a stream sent, and a sink that keeps them while `more` says so.
type Events = Arc<Mutex<Vec<MetricsEvent>>>;

fn keep(more: impl Fn(usize) -> bool) -> (Events, impl Fn(MetricsEvent) -> bool) {
    let events: Events = Arc::default();
    let kept = events.clone();
    (events, move |event| {
        let mut events = kept.lock().unwrap();
        events.push(event);
        more(events.len())
    })
}

/// Runs a stream that must end by itself.
async fn ends(stream: impl Future<Output = ()>) {
    tokio::time::timeout(Duration::from_secs(5), stream)
        .await
        .expect("the stream should have ended");
}

fn requests(seen: &Seen) -> usize {
    seen.lock().unwrap().len()
}

#[test]
fn recorded_pod_metrics_sum_their_containers() {
    let metrics: PodMetrics = serde_json::from_value(pod_metrics(
        "web-1",
        "2026-11-10T10:01:05Z",
        ("123456789n", "20480Ki"),
        ("2m", "8Mi"),
    ))
    .unwrap();
    let pod = pod_usage(&metrics);
    assert_eq!(
        (pod.namespace.as_str(), pod.name.as_str()),
        ("shop", "web-1")
    );
    assert_eq!(pod.time, START + 65);
    assert_eq!(pod.containers.len(), 2);
    assert!((pod.containers[0].usage.cpu - 123.456_789).abs() < 1e-9);
    assert_eq!(pod.containers[0].usage.memory, 20_971_520);
    assert_eq!(pod.containers[1].usage.memory, 8_388_608);
    assert!((pod.usage.cpu - 125.456_789).abs() < 1e-9);
    assert_eq!(pod.usage.memory, 20_971_520 + 8_388_608);
}

#[test]
fn recorded_node_metrics_read_in_millicores_and_bytes() {
    let metrics: NodeMetrics = serde_json::from_value(node_metrics(
        "node-a",
        "2026-11-10T10:01:00Z",
        "1250m",
        "1.5Gi",
    ))
    .unwrap();
    let node = node_usage(&metrics);
    assert_eq!(node.name, "node-a");
    assert_eq!(node.usage.cpu, 1250.0);
    assert_eq!(node.usage.memory, 1_610_612_736);
    // Unreadable or missing figures are none used rather than a failure.
    let odd: NodeMetrics = serde_json::from_value(json!({
        "metadata": { "name": "node-b" }, "usage": { "cpu": "lots" }
    }))
    .unwrap();
    assert_eq!(node_usage(&odd).usage, ResourceUsage::default());
}

#[test]
fn discovery_says_whether_the_metrics_api_is_served() {
    let read = |v: Value| serde_json::from_value::<APIGroupList>(v).unwrap();
    assert!(super::serves_metrics(&read(groups(true))));
    assert!(!super::serves_metrics(&read(groups(false))));
    let other_version = json!({ "groups": [{ "name": "metrics.k8s.io",
        "versions": [{ "groupVersion": "metrics.k8s.io/v2alpha1", "version": "v2alpha1" }] }] });
    assert!(!super::serves_metrics(&read(other_version)));
}

#[tokio::test]
async fn without_metrics_server_the_stream_says_so_once_and_ends() {
    let (on, seen) = fake(|path, _| match path {
        "/apis" => (200, groups(false)),
        _ => (
            404,
            status(
                404,
                "NotFound",
                "the server could not find the requested resource",
            ),
        ),
    });
    let (events, sink) = keep(|_| true);
    ends(on.poll_metrics(MetricsTarget::Pods { namespace: None }, PACE, sink)).await;
    let events = events.lock().unwrap();
    assert!(matches!(events[..], [MetricsEvent::NotInstalled]));
    assert_eq!(*seen.lock().unwrap(), ["/apis"]);
}

#[tokio::test]
async fn a_pod_list_gets_every_pods_latest_sample() {
    let (on, seen) = fake(|path, _| match path {
        "/apis" => (200, groups(true)),
        "/apis/metrics.k8s.io/v1beta1/namespaces/shop/pods" => (
            200,
            list(
                "PodMetricsList",
                vec![
                    pod_metrics(
                        "web-2",
                        "2026-11-10T10:01:05Z",
                        ("5m", "10Mi"),
                        ("1m", "1Mi"),
                    ),
                    pod_metrics("web-1", "2026-11-10T10:01:05Z", ("250m", "1Gi"), ("0", "0")),
                ],
            ),
        ),
        other => panic!("unexpected request for {other}"),
    });
    let (events, sink) = keep(|n| n < 1);
    let target = MetricsTarget::Pods {
        namespace: Some("shop".into()),
    };
    ends(on.poll_metrics(target, PACE, sink)).await;
    let events = events.lock().unwrap();
    let [MetricsEvent::Pods { pods }] = &events[..] else {
        panic!("{events:?}")
    };
    let names: Vec<&str> = pods.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["web-1", "web-2"]);
    assert_eq!(pods[0].usage.cpu, 250.0);
    assert_eq!(pods[1].usage.memory, 11 * 1024 * 1024);
    assert_eq!(requests(&seen), 2, "discovery once, then one read");
}

#[tokio::test]
async fn a_pods_page_keeps_the_samples_read_since_it_opened() {
    let (on, seen) = fake(|path, before| match path {
        "/apis" => (200, groups(true)),
        "/apis/metrics.k8s.io/v1beta1/namespaces/shop/pods/web-1" => {
            // A new sample every other read: metrics-server's resolution
            // isn't always met by a poll.
            let time = at(15 * (before as i64 / 2));
            let memory = format!("{}Mi", 100 + before);
            (
                200,
                pod_metrics("web-1", &time, ("10m", &memory), ("1m", "1Mi")),
            )
        }
        other => panic!("unexpected request for {other}"),
    });
    let (events, sink) = keep(|n| n < 4);
    let target = MetricsTarget::Pod {
        namespace: "shop".into(),
        name: "web-1".into(),
    };
    ends(on.poll_metrics(target, PACE, sink)).await;
    let events = events.lock().unwrap();
    let Some(MetricsEvent::Pod { containers }) = events.last() else {
        panic!("{events:?}")
    };
    let names: Vec<&str> = containers.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["web", "proxy"]);
    // Reads 1 to 4 came at 0 s, 15 s, 15 s and 30 s.
    let times: Vec<i64> = containers[0].samples.iter().map(|s| s.time).collect();
    assert_eq!(times.len(), 3);
    assert_eq!(times[1] - times[0], 15);
    let memory: Vec<u64> = containers[0]
        .samples
        .iter()
        .map(|s| s.usage.memory / (1024 * 1024))
        .collect();
    assert_eq!(memory, [101, 102, 104]);
    assert_eq!(requests(&seen), 5);
}

#[tokio::test]
async fn a_pod_with_no_sample_yet_has_an_empty_history_and_polling_goes_on() {
    let (on, _) = fake(|path, _| match path {
        "/apis" => (200, groups(true)),
        _ => (
            404,
            status(
                404,
                "NotFound",
                "podmetrics.metrics.k8s.io \"web-1\" not found",
            ),
        ),
    });
    let (events, sink) = keep(|n| n < 2);
    let target = MetricsTarget::Pod {
        namespace: "shop".into(),
        name: "web-1".into(),
    };
    ends(on.poll_metrics(target, PACE, sink)).await;
    let events = events.lock().unwrap();
    assert_eq!(events.len(), 2);
    assert!(events
        .iter()
        .all(|e| matches!(e, MetricsEvent::Pod { containers } if containers.is_empty())));
}

#[tokio::test]
async fn a_nodes_page_keeps_its_samples_and_the_list_has_every_node() {
    let (on, _) = fake(|path, before| match path {
        "/apis" => (200, groups(true)),
        "/apis/metrics.k8s.io/v1beta1/nodes" => (
            200,
            list(
                "NodeMetricsList",
                vec![
                    node_metrics("node-b", "2026-11-10T10:01:00Z", "2", "2Gi"),
                    node_metrics("node-a", "2026-11-10T10:01:00Z", "500m", "1Gi"),
                ],
            ),
        ),
        "/apis/metrics.k8s.io/v1beta1/nodes/node-a" => {
            let time = at(15 * before as i64);
            (200, node_metrics("node-a", &time, "500m", "1Gi"))
        }
        other => panic!("unexpected request for {other}"),
    });
    let (events, sink) = keep(|n| n < 1);
    ends(on.clone().poll_metrics(MetricsTarget::Nodes, PACE, sink)).await;
    let names: Vec<String> = match &events.lock().unwrap()[..] {
        [MetricsEvent::Nodes { nodes }] => nodes.iter().map(|n| n.name.clone()).collect(),
        other => panic!("{other:?}"),
    };
    assert_eq!(names, ["node-a", "node-b"]);

    let (events, sink) = keep(|n| n < 3);
    let target = MetricsTarget::Node {
        name: "node-a".into(),
    };
    ends(on.poll_metrics(target, PACE, sink)).await;
    let events = events.lock().unwrap();
    let Some(MetricsEvent::Node { samples }) = events.last() else {
        panic!("{events:?}")
    };
    assert_eq!(samples.len(), 3);
    assert_eq!(samples[0].usage.cpu, 500.0);
}

#[tokio::test]
async fn forbidden_metrics_are_said_once_and_polling_stops() {
    let (on, seen) = fake(|path, _| match path {
        "/apis" => (200, groups(true)),
        _ => (
            403,
            status(
                403,
                "Forbidden",
                "pods.metrics.k8s.io is forbidden: User \"dev\" cannot list resource \"pods\"",
            ),
        ),
    });
    let (events, sink) = keep(|_| true);
    let target = MetricsTarget::Pods {
        namespace: Some("shop".into()),
    };
    ends(on.poll_metrics(target, PACE, sink)).await;
    let events = events.lock().unwrap();
    let [MetricsEvent::Forbidden { error }] = &events[..] else {
        panic!("{events:?}")
    };
    assert_eq!(error.code, ErrorCode::Forbidden);
    assert!(
        error.message.contains("pods.metrics.k8s.io"),
        "{}",
        error.message
    );
    assert!(error.message.contains("shop"));
    assert_eq!(requests(&seen), 2);
}

#[tokio::test]
async fn a_metrics_server_that_doesnt_answer_yet_keeps_the_stream_going() {
    let (on, _) = fake(|path, before| match (path, before) {
        ("/apis", _) => (200, groups(true)),
        (_, 1) => (
            503,
            status(
                503,
                "ServiceUnavailable",
                "the server is currently unable to handle the request",
            ),
        ),
        _ => (200, list("NodeMetricsList", vec![])),
    });
    let (events, sink) = keep(|n| n < 2);
    ends(on.poll_metrics(MetricsTarget::Nodes, PACE, sink)).await;
    let events = events.lock().unwrap();
    assert!(
        matches!(
            &events[..],
            [MetricsEvent::NotResponding { .. }, MetricsEvent::Nodes { nodes }] if nodes.is_empty()
        ),
        "{events:?}"
    );
}

#[tokio::test]
async fn a_read_the_api_server_holds_counts_as_not_answering() {
    // The API server holds a request for a registered metrics-server it
    // can't reach until its own timeout.
    let held = Arc::new(Mutex::new(0usize));
    let reads = held.clone();
    let service = tower::service_fn(move |request: http::Request<kube::client::Body>| {
        let discovery = request.uri().path() == "/apis";
        if !discovery {
            *reads.lock().unwrap() += 1;
        }
        async move {
            if !discovery {
                std::future::pending::<()>().await;
            }
            Ok::<_, Infallible>(
                http::Response::builder()
                    .body(Full::new(Bytes::from(groups(true).to_string())))
                    .unwrap(),
            )
        }
    });
    let (on, _) = fake(|_, _| (200, groups(true)));
    let on = On {
        client: kube::Client::new(service, "default"),
        ..on
    };
    let (events, sink) = keep(|n| n < 2);
    let pace = Pace {
        every: TICK,
        wait: Duration::from_millis(50),
    };
    ends(on.poll_metrics(MetricsTarget::Nodes, pace, sink)).await;
    let events = events.lock().unwrap();
    assert!(
        events.iter().all(|e| matches!(
            e,
            MetricsEvent::NotResponding { error } if error.code == ErrorCode::Timeout
        )),
        "{events:?}"
    );
    assert_eq!(*held.lock().unwrap(), 2, "each tick reads again");
}

#[tokio::test]
async fn a_failed_discovery_is_tried_again() {
    let (on, seen) = fake(|path, before| match (path, before) {
        ("/apis", 0) => (500, status(500, "InternalError", "etcd is down")),
        ("/apis", _) => (200, groups(false)),
        (other, _) => panic!("unexpected request for {other}"),
    });
    let (events, sink) = keep(|_| true);
    ends(on.poll_metrics(MetricsTarget::Nodes, PACE, sink)).await;
    let events = events.lock().unwrap();
    assert!(
        matches!(
            &events[..],
            [MetricsEvent::Failed { .. }, MetricsEvent::NotInstalled]
        ),
        "{events:?}"
    );
    assert_eq!(*seen.lock().unwrap(), ["/apis", "/apis"]);
}

#[tokio::test]
async fn polling_stops_when_the_stream_is_cancelled() {
    let (on, seen) = fake(|path, _| match path {
        "/apis" => (200, groups(true)),
        _ => (200, list("NodeMetricsList", vec![])),
    });
    let (_events, sink) = keep(|_| true);
    let streams = Streams::default();
    let id = streams.spawn(on.poll_metrics(MetricsTarget::Nodes, PACE, sink));
    tokio::time::timeout(Duration::from_secs(5), async {
        while requests(&seen) < 4 {
            tokio::time::sleep(TICK).await;
        }
    })
    .await
    .expect("the stream should poll");
    streams.cancel(id);
    // A read under way when it was cancelled may still land.
    tokio::time::sleep(TICK * 3).await;
    let after = requests(&seen);
    tokio::time::sleep(TICK * 20).await;
    assert_eq!(
        requests(&seen),
        after,
        "nothing is read once the page closed"
    );
}

#[tokio::test]
async fn polling_stops_when_the_channel_is_dropped() {
    let (on, seen) = fake(|path, _| match path {
        "/apis" => (200, groups(true)),
        _ => (200, list("NodeMetricsList", vec![])),
    });
    // The sink says the channel is gone from the first event.
    let (events, sink) = keep(|_| false);
    ends(on.poll_metrics(MetricsTarget::Nodes, PACE, sink)).await;
    assert_eq!(events.lock().unwrap().len(), 1);
    assert_eq!(requests(&seen), 2);
}

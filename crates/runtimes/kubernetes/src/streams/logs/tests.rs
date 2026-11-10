use std::sync::{Arc, Mutex};

use super::*;

fn options() -> LogOptions {
    LogOptions {
        tail: Some(100),
        timestamps: true,
        since: Some(1_000),
        stdout: true,
        stderr: true,
        follow: true,
    }
}

#[test]
fn options_become_the_query_of_pods_log() {
    let target = PodLogTarget {
        container: Some("api".into()),
        previous: false,
    };
    let p = log_params(&options(), &target, 1_600);
    assert_eq!(p.container.as_deref(), Some("api"));
    assert!(p.follow && p.timestamps && !p.previous);
    assert_eq!(p.tail_lines, Some(100));
    assert_eq!(p.since_seconds, Some(600));
}

#[test]
fn a_previous_instance_isnt_followed() {
    let target = PodLogTarget {
        container: Some(String::new()),
        previous: true,
    };
    let p = log_params(
        &LogOptions {
            tail: None,
            since: None,
            ..options()
        },
        &target,
        0,
    );
    assert!(p.previous && !p.follow);
    assert_eq!(p.container, None);
    assert_eq!((p.tail_lines, p.since_seconds), (None, None));
}

#[test]
fn the_usual_refusals_are_explained() {
    let e = refused(
        "api-1",
        "api",
        "previous terminated container \"api\" in pod \"api-1\" not found",
    )
    .unwrap();
    assert_eq!(e.code, ErrorCode::NotFound);
    assert_eq!(e.key, "errors.clusters.logs.noPrevious");
    let e = refused(
        "api-1",
        "api",
        "container \"api\" in pod \"api-1\" is waiting to start: CrashLoopBackOff",
    )
    .unwrap();
    assert_eq!(e.code, ErrorCode::ContainerNotRunning);
    assert!(e.message.contains("CrashLoopBackOff"));
    assert!(refused("api-1", "api", "something else").is_none());
}

fn source(label: &str, chunks: &[&str]) -> (Option<String>, Chunks) {
    let chunks: Vec<std::io::Result<Vec<u8>>> =
        chunks.iter().map(|c| Ok(c.as_bytes().to_vec())).collect();
    (Some(label.into()), stream::iter(chunks).boxed())
}

#[tokio::test]
async fn several_pods_interleave_their_lines_each_with_its_label() {
    let events = Arc::new(Mutex::new(vec![]));
    let sink = {
        let events = events.clone();
        move |e| {
            events.lock().unwrap().push(e);
            true
        }
    };
    pump(
        vec![
            source("api-1", &["one\ntw", "o\n"]),
            source("api-2", &["three\nfour"]),
        ],
        sink,
    )
    .await;
    let events = events.lock().unwrap();
    let mut lines: Vec<(String, String)> = events
        .iter()
        .flat_map(|e| match e {
            LogEvent::Lines { lines } => lines.clone(),
            _ => vec![],
        })
        .map(|l| (l.source.unwrap(), l.text))
        .collect();
    lines.sort();
    assert_eq!(
        lines,
        [
            ("api-1".to_owned(), "one".to_owned()),
            ("api-1".to_owned(), "two".to_owned()),
            ("api-2".to_owned(), "four".to_owned()),
            ("api-2".to_owned(), "three".to_owned()),
        ]
    );
    assert_eq!(events.last(), Some(&LogEvent::End));
}

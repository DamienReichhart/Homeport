use std::sync::{Arc, Mutex};

use super::*;
use crate::client::testing::agent;

fn frame(text: &str) -> String {
    format!(
        r#"{{"File":"alloc/logs/server.stdout.0","Offset":1,"Data":"{}"}}"#,
        base64::engine::general_purpose::STANDARD.encode(text)
    )
}

#[test]
fn frames_split_anywhere_decode_to_the_logs_bytes() {
    let stream = format!("{}{{}}{}\n", frame("hello\nwor"), frame("ld\n"));
    let mut decoder = FrameDecoder::default();
    let mut out = vec![];
    for byte in stream.as_bytes() {
        out.extend(decoder.push(&[*byte]));
    }
    assert_eq!(String::from_utf8(out).unwrap(), "hello\nworld\n");
}

#[test]
fn history_from_the_end_is_asked_in_bytes() {
    let options = LogOptions {
        tail: Some(100),
        timestamps: false,
        since: None,
        stdout: true,
        stderr: false,
        follow: true,
    };
    let q = query("server", "stdout", &options);
    assert!(q.contains(&("origin", "end".into())));
    assert!(q.contains(&("offset", "25600".into())));
    assert!(q.contains(&("follow", "true".into())));
}

#[tokio::test]
async fn both_streams_reach_the_sink_and_then_the_end() {
    let fake = agent(vec![(
        "GET /v1/client/fs/logs/a1",
        200,
        vec![],
        format!(
            "{}{}",
            frame("cut-off part\nfirst\nsecond\n"),
            frame("third\n")
        ),
    )])
    .await;
    let events = Arc::new(Mutex::new(vec![]));
    let seen = events.clone();
    fake.client(None)
        .stream_logs(
            "a1".into(),
            "server".into(),
            LogOptions {
                tail: Some(2),
                timestamps: false,
                since: None,
                stdout: true,
                stderr: true,
                follow: false,
            },
            move |e| {
                seen.lock().unwrap().push(e);
                true
            },
        )
        .await;
    let events = events.lock().unwrap();
    let lines: Vec<(String, String)> = events
        .iter()
        .flat_map(|e| match e {
            LogEvent::Lines { lines } => lines
                .iter()
                .map(|l| (l.stream.clone(), l.text.clone()))
                .collect(),
            _ => vec![],
        })
        .collect();
    // The cut-off line is dropped; each stream keeps the last two of the rest.
    for stream in ["stdout", "stderr"] {
        let of: Vec<&str> = lines
            .iter()
            .filter(|(s, _)| s == stream)
            .map(|(_, text)| text.as_str())
            .collect();
        assert_eq!(of, ["second", "third"], "{stream}");
    }
    assert!(matches!(events.last(), Some(LogEvent::End)));
    let sent = fake.received();
    assert!(sent.iter().any(|r| r.target.contains("type=stderr")));
}

#[tokio::test]
async fn history_is_read_whole_before_following_from_the_end() {
    let fake = agent(vec![(
        "GET /v1/client/fs/logs/a1",
        200,
        vec![],
        frame("old\nnew\n"),
    )])
    .await;
    fake.client(None)
        .stream_logs(
            "a1".into(),
            "server".into(),
            LogOptions {
                tail: Some(10),
                timestamps: false,
                since: None,
                stdout: true,
                stderr: false,
                follow: true,
            },
            |_| true,
        )
        .await;
    let sent = fake.received();
    assert_eq!(sent.len(), 2);
    assert!(sent[0].target.contains("follow=false") && sent[0].target.contains("offset=2560"));
    assert!(sent[1].target.contains("follow=true") && sent[1].target.contains("offset=0"));
    assert!(sent[1].target.contains("origin=end"));
}

#[tokio::test]
async fn a_token_without_read_logs_says_so() {
    let fake = agent(vec![(
        "GET /v1/client/fs/logs/a1",
        403,
        vec![],
        "Permission denied".into(),
    )])
    .await;
    let events = Arc::new(Mutex::new(vec![]));
    let seen = events.clone();
    fake.client(Some("t"))
        .stream_logs(
            "a1".into(),
            "server".into(),
            LogOptions {
                tail: None,
                timestamps: false,
                since: None,
                stdout: true,
                stderr: false,
                follow: true,
            },
            move |e| {
                seen.lock().unwrap().push(e);
                true
            },
        )
        .await;
    let events = events.lock().unwrap();
    let Some(LogEvent::Error { error }) = events.last() else {
        panic!("{events:?}");
    };
    assert_eq!(error.key, "errors.nomad.logsDenied");
}

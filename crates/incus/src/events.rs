//! Incus's lifecycle events, from every project, read from `/1.0/events`.

use std::collections::HashMap;

use futures_util::{Stream, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::tungstenite::Message;

use homeport_core::error::AppResult;

use crate::{client::Incus, websocket};

/// One lifecycle event, in the shape the Host's event history keeps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncusEvent {
    pub time_nano: i64,
    /// The kind of object: `instance`, `image`, `profile`, `network`,
    /// `storage-pool`, `storage-volume`, `project`, …
    pub kind: String,
    /// What happened, in Incus's words: `instance-started`.
    pub action: String,
    /// The object's name, read from its API path.
    pub actor_id: String,
    /// `project`, `source` (the object's API path) and, for snapshots,
    /// `instance`.
    pub attributes: HashMap<String, String>,
}

#[derive(Deserialize)]
struct RawEvent {
    #[serde(rename = "type", default)]
    kind: String,
    #[serde(default)]
    timestamp: String,
    #[serde(default)]
    metadata: RawLifecycle,
    #[serde(default)]
    project: String,
}

#[derive(Default, Deserialize)]
struct RawLifecycle {
    #[serde(default)]
    action: String,
    #[serde(default)]
    source: String,
    #[serde(default)]
    project: String,
}

/// The kinds of object Incus names its actions after, longest first so
/// `storage-volume-created` isn't read as a storage pool's.
const KINDS: [&str; 15] = [
    "instance-snapshot",
    "instance-backup",
    "instance-log",
    "instance",
    "image-alias",
    "image",
    "profile",
    "network-acl",
    "network-zone",
    "network",
    "storage-volume-snapshot",
    "storage-volume",
    "storage-pool",
    "project",
    "certificate",
];

/// One message of the events websocket; `None` for anything but a
/// lifecycle event Homeport can read.
pub fn parse(text: &str) -> Option<IncusEvent> {
    let raw: RawEvent = serde_json::from_str(text)
        .map_err(|e| log::debug!("skipping an Incus event Homeport can't read: {e}"))
        .ok()?;
    if raw.kind != "lifecycle" || raw.metadata.action.is_empty() {
        return None;
    }
    let action = raw.metadata.action;
    let kind = KINDS
        .iter()
        .find(|k| action.starts_with(&format!("{k}-")))
        .map_or("other", |k| k)
        .to_owned();
    let path = raw.metadata.source.split('?').next().unwrap_or_default();
    let segments: Vec<String> = path
        .split('/')
        .filter(|s| !s.is_empty())
        .map(decode)
        .collect();
    let mut attributes = HashMap::new();
    let project = if raw.metadata.project.is_empty() {
        raw.project
    } else {
        raw.metadata.project
    };
    attributes.insert("project".to_owned(), project);
    attributes.insert("source".to_owned(), raw.metadata.source.clone());
    // /1.0/instances/web/snapshots/snap0: the snapshot belongs to `web`.
    if let [_, collection, owner, _, _] = segments.as_slice() {
        if collection == "instances" {
            attributes.insert("instance".to_owned(), owner.clone());
        }
    }
    let time_nano = raw
        .timestamp
        .parse::<jiff::Timestamp>()
        .map_or_else(|_| now_nano(), nanos);
    Some(IncusEvent {
        time_nano,
        kind,
        action,
        actor_id: segments.last().cloned().unwrap_or_default(),
        attributes,
    })
}

fn nanos(time: jiff::Timestamp) -> i64 {
    i64::try_from(time.as_nanosecond()).unwrap_or(i64::MAX)
}

fn now_nano() -> i64 {
    nanos(jiff::Timestamp::now())
}

/// A percent-decoded path segment.
fn decode(segment: &str) -> String {
    let bytes = segment.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Some(byte) = segment
                .get(i + 1..i + 3)
                .and_then(|hex| u8::from_str_radix(hex, 16).ok())
            {
                out.push(byte);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Every lifecycle event of every project, until the websocket closes.
pub async fn subscribe(incus: &Incus) -> AppResult<impl Stream<Item = IncusEvent>> {
    let socket = websocket::open(incus, "/1.0/events?type=lifecycle&all-projects=true").await?;
    Ok(socket.filter_map(|message| async move {
        match message {
            Ok(Message::Text(text)) => parse(&text),
            Ok(Message::Binary(bytes)) => parse(&String::from_utf8_lossy(&bytes)),
            _ => None,
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_events_name_their_object_and_project() {
        // As Incus 6.0 writes them to /1.0/events.
        let started = parse(
            r#"{"type":"lifecycle","timestamp":"2026-11-08T10:15:42.123456789Z","metadata":{"action":"instance-started","source":"/1.0/instances/web","context":{},"requestor":{"username":"root","protocol":"unix","address":"@"},"project":"lab"},"location":"none","project":"lab"}"#,
        )
        .unwrap();
        assert_eq!(started.kind, "instance");
        assert_eq!(started.action, "instance-started");
        assert_eq!(started.actor_id, "web");
        assert_eq!(started.attributes["project"], "lab");
        assert_eq!(started.time_nano, 1_794_132_942_123_456_789);

        let snapshot = parse(
            r#"{"type":"lifecycle","timestamp":"2026-11-08T10:15:42Z","metadata":{"action":"instance-snapshot-created","source":"/1.0/instances/web/snapshots/before%20upgrade","project":"default"},"project":"default"}"#,
        )
        .unwrap();
        assert_eq!(snapshot.kind, "instance-snapshot");
        assert_eq!(snapshot.actor_id, "before upgrade");
        assert_eq!(snapshot.attributes["instance"], "web");

        let volume = parse(
            r#"{"type":"lifecycle","timestamp":"2026-11-08T10:15:42Z","metadata":{"action":"storage-volume-created","source":"/1.0/storage-pools/default/volumes/custom/data","project":"default"}}"#,
        )
        .unwrap();
        assert_eq!(volume.kind, "storage-volume");
        assert_eq!(volume.actor_id, "data");
    }

    #[test]
    fn other_messages_are_skipped() {
        assert!(parse(r#"{"type":"logging","metadata":{"message":"hi"}}"#).is_none());
        assert!(parse("not json").is_none());
    }
}

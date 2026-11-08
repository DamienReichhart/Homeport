//! containerd's event stream, as one kind, action and actor per event.

use std::collections::HashMap;

use futures_util::{Stream, StreamExt};
use prost::Message;

use homeport_core::error::AppResult;

use crate::{
    api::containerd::{
        events as ev,
        services::{events::v1::SubscribeRequest, images::v1 as images},
        types::Envelope,
    },
    error, Containerd,
};

/// One event, in every namespace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerdEvent {
    /// Unix time in nanoseconds.
    pub time_nano: i64,
    /// `task`, `container`, `image`, `namespace`, `snapshot` or `content`.
    pub kind: String,
    /// What happened: `start`, `exit`, `create`, `delete`, `exec-added`…
    pub action: String,
    /// The container, image, namespace, snapshot or blob it's about.
    pub actor_id: String,
    /// The event's namespace (`namespace`), and what else it says.
    pub attributes: HashMap<String, String>,
}

/// containerd's events from now on, until the connection breaks.
pub async fn subscribe(
    client: &Containerd,
) -> AppResult<impl Stream<Item = ContainerdEvent> + Send + 'static> {
    let stream = client
        .events()
        .subscribe(SubscribeRequest::default())
        .await
        .map_err(|s| error::status(&s))?
        .into_inner();
    Ok(stream
        .take_while(|item| std::future::ready(item.is_ok()))
        .filter_map(|item| std::future::ready(item.ok().map(event))))
}

/// `/tasks/exit` → (`task`, `exit`); `/snapshot/prepare` → (`snapshot`, `prepare`).
fn kind_and_action(topic: &str) -> (String, String) {
    let topic = topic.trim_start_matches('/');
    let (kind, action) = topic.split_once('/').unwrap_or((topic, ""));
    let kind = match kind {
        "tasks" => "task",
        "containers" => "container",
        "images" => "image",
        "namespaces" => "namespace",
        other => other,
    };
    (kind.to_owned(), action.to_owned())
}

/// Reads an envelope: its kind and action from the topic, its actor and
/// attributes from the event inside, decoded by its type.
pub(crate) fn event(envelope: Envelope) -> ContainerdEvent {
    let (kind, action) = kind_and_action(&envelope.topic);
    let time_nano = envelope.timestamp.map_or(0, |t| {
        t.seconds
            .saturating_mul(1_000_000_000)
            .saturating_add(i64::from(t.nanos))
    });
    let mut attributes = HashMap::from([("namespace".to_owned(), envelope.namespace)]);
    let mut set = |key: &str, value: String| {
        if !value.is_empty() {
            attributes.insert(key.to_owned(), value);
        }
    };
    let actor_id = match envelope.event {
        Some(any) => {
            let name = any
                .type_url
                .rsplit('/')
                .next()
                .unwrap_or_default()
                .to_owned();
            let bytes = any.value.as_slice();
            macro_rules! decode {
                ($t:ty) => {
                    <$t>::decode(bytes).ok()
                };
            }
            // Image events live in the images service's package, the rest in events'.
            match name.rsplit('.').next().unwrap_or_default() {
                "TaskCreate" => decode!(ev::TaskCreate).map(|e| e.container_id),
                "TaskStart" => decode!(ev::TaskStart).map(|e| e.container_id),
                "TaskDelete" => decode!(ev::TaskDelete).map(|e| {
                    set("exitCode", e.exit_status.to_string());
                    e.container_id
                }),
                "TaskExit" => decode!(ev::TaskExit).map(|e| {
                    set("exitCode", e.exit_status.to_string());
                    if e.id != e.container_id {
                        set("execID", e.id);
                    }
                    e.container_id
                }),
                "TaskOOM" => decode!(ev::TaskOom).map(|e| e.container_id),
                "TaskExecAdded" => decode!(ev::TaskExecAdded).map(|e| {
                    set("execID", e.exec_id);
                    e.container_id
                }),
                "TaskExecStarted" => decode!(ev::TaskExecStarted).map(|e| {
                    set("execID", e.exec_id);
                    e.container_id
                }),
                "TaskPaused" => decode!(ev::TaskPaused).map(|e| e.container_id),
                "TaskResumed" => decode!(ev::TaskResumed).map(|e| e.container_id),
                "TaskCheckpointed" => decode!(ev::TaskCheckpointed).map(|e| e.container_id),
                "ContainerCreate" => decode!(ev::ContainerCreate).map(|e| {
                    set("image", e.image);
                    e.id
                }),
                "ContainerUpdate" => decode!(ev::ContainerUpdate).map(|e| {
                    set("image", e.image);
                    e.id
                }),
                "ContainerDelete" => decode!(ev::ContainerDelete).map(|e| e.id),
                "ImageCreate" => decode!(images::ImageCreate).map(|e| e.name),
                "ImageUpdate" => decode!(images::ImageUpdate).map(|e| e.name),
                "ImageDelete" => decode!(images::ImageDelete).map(|e| e.name),
                "NamespaceCreate" => decode!(ev::NamespaceCreate).map(|e| e.name),
                "NamespaceUpdate" => decode!(ev::NamespaceUpdate).map(|e| e.name),
                "NamespaceDelete" => decode!(ev::NamespaceDelete).map(|e| e.name),
                "SnapshotPrepare" => decode!(ev::SnapshotPrepare).map(|e| {
                    set("snapshotter", e.snapshotter);
                    set("parent", e.parent);
                    e.key
                }),
                "SnapshotCommit" => decode!(ev::SnapshotCommit).map(|e| {
                    set("snapshotter", e.snapshotter);
                    set("name", e.name);
                    e.key
                }),
                "SnapshotRemove" => decode!(ev::SnapshotRemove).map(|e| {
                    set("snapshotter", e.snapshotter);
                    e.key
                }),
                "ContentCreate" => decode!(ev::ContentCreate).map(|e| e.digest),
                "ContentDelete" => decode!(ev::ContentDelete).map(|e| e.digest),
                _ => None,
            }
            .unwrap_or_default()
        }
        None => String::new(),
    };
    ContainerdEvent {
        time_nano,
        kind,
        action,
        actor_id,
        attributes,
    }
}

#[cfg(test)]
mod tests {
    use prost::Name;

    use super::*;

    fn envelope(topic: &str, event: impl Name) -> Envelope {
        Envelope {
            timestamp: Some(prost_types::Timestamp {
                seconds: 1_794_162_522,
                nanos: 479,
            }),
            namespace: "default".into(),
            topic: topic.into(),
            event: Some(prost_types::Any {
                type_url: FullName::full_name(&event),
                value: event.encode_to_vec(),
            }),
        }
    }

    trait FullName {
        fn full_name(&self) -> String;
    }
    impl<T: Name> FullName for T {
        fn full_name(&self) -> String {
            T::full_name()
        }
    }

    #[test]
    fn topics_name_the_kind_and_the_action() {
        assert_eq!(
            kind_and_action("/tasks/exec-added"),
            ("task".into(), "exec-added".into())
        );
        assert_eq!(
            kind_and_action("/snapshot/commit"),
            ("snapshot".into(), "commit".into())
        );
        assert_eq!(
            kind_and_action("/content/delete"),
            ("content".into(), "delete".into())
        );
    }

    #[test]
    fn events_are_read_by_their_type() {
        let exit = event(envelope(
            "/tasks/exit",
            ev::TaskExit {
                container_id: "web".into(),
                id: "web".into(),
                pid: 42,
                exit_status: 137,
                exited_at: None,
            },
        ));
        assert_eq!((exit.kind.as_str(), exit.action.as_str()), ("task", "exit"));
        assert_eq!(exit.actor_id, "web");
        assert_eq!(exit.attributes["exitCode"], "137");
        assert_eq!(exit.attributes["namespace"], "default");
        assert!(!exit.attributes.contains_key("execID"));
        assert_eq!(exit.time_nano, 1_794_162_522_000_000_479);

        let snapshot = event(envelope(
            "/snapshot/remove",
            ev::SnapshotRemove {
                key: "web".into(),
                snapshotter: "overlayfs".into(),
            },
        ));
        assert_eq!(snapshot.actor_id, "web");
        assert_eq!(snapshot.attributes["snapshotter"], "overlayfs");

        let unknown = event(Envelope {
            topic: "/sandboxes/create".into(),
            ..Default::default()
        });
        assert_eq!(unknown.kind, "sandboxes");
        assert!(unknown.actor_id.is_empty());
    }
}

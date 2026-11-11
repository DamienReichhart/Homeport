//! Kubernetes Events: what happened to a resource, and why.

use k8s_openapi::api::core::v1::Event;
use kube::api::ListParams;
use serde::Serialize;

use homeport_core::error::AppResult;

use super::events;
use crate::resources::common::time;
use crate::resources::{On, ResourceKind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct EventView {
    pub namespace: String,
    pub name: String,
    /// `Normal` or `Warning`.
    #[serde(rename = "type")]
    pub kind: String,
    pub reason: String,
    pub message: String,
    pub object: EventObject,
    pub count: u32,
    pub first_seen: Option<String>,
    pub last_seen: Option<String>,
    /// What reported it (`kubelet`, `deployment-controller`…).
    pub source: Option<String>,
}

/// The resource an Event is about.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct EventObject {
    pub kind: String,
    pub namespace: Option<String>,
    pub name: String,
}

pub fn view(e: &Event) -> EventView {
    let last = e
        .last_timestamp
        .as_ref()
        .map(|t| t.0.to_string())
        .or_else(|| e.event_time.as_ref().map(|t| t.0.to_string()))
        .or_else(|| time(e.metadata.creation_timestamp.as_ref()));
    EventView {
        namespace: e.metadata.namespace.clone().unwrap_or_default(),
        name: e.metadata.name.clone().unwrap_or_default(),
        kind: e.type_.clone().unwrap_or_else(|| "Normal".into()),
        reason: e.reason.clone().unwrap_or_default(),
        message: e.message.clone().unwrap_or_default(),
        object: EventObject {
            kind: e.involved_object.kind.clone().unwrap_or_default(),
            namespace: e.involved_object.namespace.clone(),
            name: e.involved_object.name.clone().unwrap_or_default(),
        },
        count: e
            .count
            .or_else(|| e.series.as_ref().and_then(|s| s.count))
            .unwrap_or(1)
            .max(1) as u32,
        first_seen: time(e.first_timestamp.as_ref()).or_else(|| last.clone()),
        last_seen: last,
        source: e
            .reporting_component
            .clone()
            .filter(|c| !c.is_empty())
            .or_else(|| e.source.as_ref().and_then(|s| s.component.clone())),
    }
}

/// Warnings first, then the most recent.
pub fn sort(events: &mut [EventView]) {
    events.sort_by(|a, b| {
        (b.kind == "Warning")
            .cmp(&(a.kind == "Warning"))
            .then_with(|| b.last_seen.cmp(&a.last_seen))
    });
}

impl On {
    /// The Events in `namespace` (every one with `None`), warnings first.
    pub async fn events(&self, namespace: Option<&str>) -> AppResult<Vec<EventView>> {
        let items: Vec<Event> = self
            .list(
                self.api(namespace),
                ResourceKind::Event,
                namespace,
                ListParams::default(),
            )
            .await?;
        let mut views: Vec<_> = items.iter().map(events::view).collect();
        events::sort(&mut views);
        Ok(views)
    }

    /// The Events about one object, warnings first.
    pub async fn object_events(
        &self,
        kind: ResourceKind,
        namespace: Option<&str>,
        name: &str,
    ) -> AppResult<Vec<EventView>> {
        let fields = format!(
            "involvedObject.kind={},involvedObject.name={name}",
            kind.kind()
        );
        let items: Vec<Event> = self
            .list(
                self.api(namespace),
                ResourceKind::Event,
                namespace,
                ListParams::default().fields(&fields),
            )
            .await?;
        let mut views: Vec<_> = items.iter().map(events::view).collect();
        events::sort(&mut views);
        Ok(views)
    }
}

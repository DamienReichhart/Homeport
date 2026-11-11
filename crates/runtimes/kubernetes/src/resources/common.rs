//! What every resource view shares: names, times, labels, conditions, owners.

use std::collections::BTreeMap;

use k8s_openapi::apimachinery::pkg::apis::meta::v1::{ObjectMeta, Time};
use serde::Serialize;

/// When something happened, in RFC 3339 UTC.
pub(crate) fn time(t: Option<&Time>) -> Option<String> {
    t.map(|t| t.0.to_string())
}

pub(crate) fn created(meta: &ObjectMeta) -> Option<String> {
    time(meta.creation_timestamp.as_ref())
}

pub(crate) fn labels(meta: &ObjectMeta) -> BTreeMap<String, String> {
    meta.labels.clone().unwrap_or_default()
}

/// A resource's annotations, without kubectl's copy of its last applied
/// configuration, which repeats the whole object.
pub(crate) fn annotations(meta: &ObjectMeta) -> BTreeMap<String, String> {
    let mut annotations = meta.annotations.clone().unwrap_or_default();
    annotations.remove("kubectl.kubernetes.io/last-applied-configuration");
    annotations
}

/// One condition of a resource's status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ConditionView {
    #[serde(rename = "type")]
    pub kind: String,
    /// `True`, `False` or `Unknown`.
    pub status: String,
    pub reason: Option<String>,
    pub message: Option<String>,
    pub last_transition: Option<String>,
}

/// Builds the condition views of any `*Condition` type, which all share these fields.
macro_rules! conditions {
    ($list:expr) => {
        $list
            .into_iter()
            .flatten()
            .map(|c| $crate::resources::common::ConditionView {
                kind: c.type_.clone(),
                status: c.status.clone(),
                reason: c.reason.clone().filter(|r| !r.is_empty()),
                message: c.message.clone().filter(|m| !m.is_empty()),
                last_transition: $crate::resources::common::time(c.last_transition_time.as_ref()),
            })
            .collect::<Vec<_>>()
    };
}
pub(crate) use conditions;

/// The resource that controls another, such as a Pod's ReplicaSet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct OwnerView {
    pub kind: String,
    pub name: String,
}

/// The controller among a resource's owners.
pub(crate) fn owner(meta: &ObjectMeta) -> Option<OwnerView> {
    let owners = meta.owner_references.as_ref()?;
    let owner = owners
        .iter()
        .find(|o| o.controller == Some(true))
        .or_else(|| owners.first())?;
    Some(OwnerView {
        kind: owner.kind.clone(),
        name: owner.name.clone(),
    })
}

/// A label selector's `matchLabels`, as a `key=value,…` selector string.
pub(crate) fn selector_string(labels: &BTreeMap<String, String>) -> String {
    labels
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(",")
}

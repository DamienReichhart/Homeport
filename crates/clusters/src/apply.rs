//! Changing a Cluster with YAML, like `kubectl apply` and `kubectl edit`:
//! one or more documents applied with server-side apply under the `homeport`
//! field manager, after a dry run that shows what each would change.

use kube::{
    api::{DynamicObject, Patch, PatchParams},
    core::response::StatusCause,
};
use serde::{Deserialize as _, Serialize};
use serde_json::Value;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    discovery::ApiResourceInfo,
    messages as msg,
    resources::{actions::FIELD_MANAGER, strip, tidy, ApiType, On, ResourceKind},
};

/// What applying a document does, or did.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ApplyAction {
    Created,
    Changed,
    Unchanged,
    /// It can't be applied: see its error.
    Failed,
}

/// Fields of the object another field manager owns, which applying would take over.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FieldConflict {
    /// `.spec.replicas`.
    pub field: String,
    /// What the API server said: `conflict with "kubectl-client-side-apply" using apps/v1`.
    pub message: String,
}

/// One document of the YAML, and what applying it does.
#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ApplyDocument {
    pub api_version: String,
    pub kind: String,
    pub namespace: Option<String>,
    pub name: String,
    pub action: ApplyAction,
    /// The object as it is, without what the server manages; `None` when new.
    pub before: Option<String>,
    /// As it would be, or is, after applying.
    pub after: Option<String>,
    pub conflicts: Vec<FieldConflict>,
    pub error: Option<AppError>,
}

/// Every document of the YAML, in order.
#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ApplyResult {
    pub documents: Vec<ApplyDocument>,
}

/// The documents of `yaml`, empty ones skipped; an error names the line.
pub fn documents(yaml: &str) -> AppResult<Vec<Value>> {
    let mut out = vec![];
    for document in serde_yaml_ng::Deserializer::from_str(yaml) {
        let value = Value::deserialize(document).map_err(|e| {
            let line = e.location().map_or(0, |l| l.line());
            AppError::validation(msg::YAML_INVALID.with("line", line)).with_detail(e.to_string())
        })?;
        if !value.is_null() {
            out.push(value);
        }
    }
    if out.is_empty() {
        return Err(AppError::validation(msg::YAML_EMPTY));
    }
    Ok(out)
}

/// What the server manages on an object, left out of what is compared and
/// shown: `managedFields`, `resourceVersion`, `generation`, `status`…
pub fn normalize(value: &mut Value) {
    strip(value);
    if let Some(meta) = value.get_mut("metadata").and_then(Value::as_object_mut) {
        for field in ["resourceVersion", "generation", "selfLink"] {
            meta.remove(field);
        }
    }
    if let Some(object) = value.as_object_mut() {
        object.remove("status");
    }
}

/// An object as YAML for the diff, a Secret's values hidden.
fn shown(value: &Value) -> String {
    let mut value = value.clone();
    tidy(&mut value);
    serde_yaml_ng::to_string(&value).unwrap_or_default()
}

/// What an object holds for editing: without what the server manages, but
/// with its `resourceVersion`, so a change made meanwhile is refused.
pub fn editable(mut value: Value) -> Value {
    let version = value.pointer("/metadata/resourceVersion").cloned();
    normalize(&mut value);
    if let Some(meta) = value.get_mut("metadata").and_then(Value::as_object_mut) {
        for field in ["uid", "creationTimestamp"] {
            meta.remove(field);
        }
        if let Some(version) = version {
            meta.insert("resourceVersion".into(), version);
        }
    }
    value
}

/// A document's identity, or why it has none.
struct Identity {
    api_version: String,
    kind: String,
    name: String,
    namespace: Option<String>,
}

fn identity(doc: &Value) -> Result<Identity, AppError> {
    let text = |pointer: &str| {
        doc.pointer(pointer)
            .and_then(Value::as_str)
            .map(str::to_owned)
            .filter(|s| !s.is_empty())
    };
    let api_version = text("/apiVersion");
    let kind = text("/kind");
    let name = text("/metadata/name");
    match (api_version, kind, name) {
        (Some(api_version), Some(kind), Some(name)) => Ok(Identity {
            api_version,
            kind,
            name,
            namespace: text("/metadata/namespace"),
        }),
        (a, k, _) => Err(AppError::validation(
            msg::DOCUMENT_INCOMPLETE
                .with("apiVersion", a.unwrap_or_default())
                .with("kind", k.unwrap_or_default()),
        )),
    }
}

/// The fields another manager owns, from a 409 answer to an apply.
pub fn conflicts(causes: &[StatusCause]) -> Vec<FieldConflict> {
    causes
        .iter()
        .filter(|c| c.reason == "FieldManagerConflict")
        .map(|c| FieldConflict {
            field: c.field.clone(),
            message: c.message.clone(),
        })
        .collect()
}

impl On {
    /// What applying `yaml` would do to each of its documents, from a dry
    /// run; nothing changes. Namespaced objects without a namespace go to
    /// `namespace`, else the Cluster's default one.
    pub async fn apply_preview(
        &self,
        yaml: &str,
        namespace: Option<&str>,
    ) -> AppResult<ApplyResult> {
        self.apply_all(yaml, namespace, true, false).await
    }

    /// Applies `yaml` with server-side apply; `force` takes over the fields
    /// other managers own.
    pub async fn apply(
        &self,
        yaml: &str,
        namespace: Option<&str>,
        force: bool,
    ) -> AppResult<ApplyResult> {
        self.apply_all(yaml, namespace, false, force).await
    }

    /// An object as YAML to edit and apply back.
    pub async fn edit_yaml(
        &self,
        api: &ApiType,
        namespace: Option<&str>,
        name: &str,
    ) -> AppResult<String> {
        if api.group.is_empty() && api.kind == ResourceKind::Secret.kind() {
            return Err(AppError::validation(msg::SECRET_NOT_EDITABLE));
        }
        let object = api
            .api(self.client.clone(), namespace)
            .get(name)
            .await
            .map_err(|e| self.failed_on("get", api, namespace, Some(name), &e))?;
        let mut value = serde_json::to_value(object)?;
        // A dynamic object's kind and version come from where it was read.
        value["apiVersion"] = api.api_version().into();
        value["kind"] = api.kind.clone().into();
        serde_yaml_ng::to_string(&editable(value)).map_err(|e| {
            AppError::new(ErrorCode::Json, homeport_core::messages::UNREADABLE_DATA).with_cause(&e)
        })
    }

    async fn apply_all(
        &self,
        yaml: &str,
        namespace: Option<&str>,
        dry_run: bool,
        force: bool,
    ) -> AppResult<ApplyResult> {
        let docs = documents(yaml)?;
        let default_namespace = match namespace.filter(|n| !n.is_empty()) {
            Some(ns) => ns.to_owned(),
            None => self.client.default_namespace().to_owned(),
        };
        let mut resolved: Vec<(String, String, ApiResourceInfo)> = vec![];
        let mut documents = Vec::with_capacity(docs.len());
        for doc in docs {
            let result = match identity(&doc) {
                Err(error) => failed_document(&doc, None, error),
                Ok(id) => {
                    let known = resolved
                        .iter()
                        .find(|(v, k, _)| *v == id.api_version && *k == id.kind)
                        .map(|(_, _, t)| t.clone());
                    let info = match known {
                        Some(info) => Ok(info),
                        None => self.resolve_kind(&id.api_version, &id.kind).await,
                    };
                    match info {
                        Err(error) => failed_document(&doc, Some(&id), error),
                        Ok(info) => {
                            resolved.push((id.api_version.clone(), id.kind.clone(), info.clone()));
                            self.apply_one(doc, id, &info.api, &default_namespace, dry_run, force)
                                .await
                        }
                    }
                }
            };
            documents.push(result);
        }
        Ok(ApplyResult { documents })
    }

    async fn apply_one(
        &self,
        mut doc: Value,
        id: Identity,
        api: &ApiType,
        default_namespace: &str,
        dry_run: bool,
        force: bool,
    ) -> ApplyDocument {
        let namespace = api.namespaced.then(|| {
            id.namespace
                .clone()
                .unwrap_or_else(|| default_namespace.to_owned())
        });
        if let Some(meta) = doc.get_mut("metadata").and_then(Value::as_object_mut) {
            match &namespace {
                Some(ns) => meta.insert("namespace".into(), ns.clone().into()),
                None => meta.remove("namespace"),
            };
        }
        let dynamic = api.api(self.client.clone(), namespace.as_deref());
        let base = ApplyDocument {
            api_version: id.api_version.clone(),
            kind: id.kind.clone(),
            namespace: namespace.clone(),
            name: id.name.clone(),
            action: ApplyAction::Failed,
            before: None,
            after: None,
            conflicts: vec![],
            error: None,
        };
        let live = match dynamic.get_opt(&id.name).await {
            Ok(live) => live,
            Err(e) => {
                return ApplyDocument {
                    error: Some(self.failed_on("get", api, namespace.as_deref(), None, &e)),
                    ..base
                }
            }
        };
        let mut params = PatchParams::apply(FIELD_MANAGER);
        params.dry_run = dry_run;
        params.force = force;
        let answer = dynamic.patch(&id.name, &params, &Patch::Apply(&doc)).await;
        let after: DynamicObject = match answer {
            Ok(after) => after,
            Err(kube::Error::Api(status)) if status.code == 409 => {
                let found = conflicts(
                    status
                        .details
                        .as_ref()
                        .map(|d| d.causes.as_slice())
                        .unwrap_or_default(),
                );
                let error = if found.is_empty() {
                    self.failed_on(
                        "patch",
                        api,
                        namespace.as_deref(),
                        Some(&id.name),
                        &kube::Error::Api(status),
                    )
                } else {
                    AppError::new(
                        ErrorCode::Conflict,
                        msg::APPLY_CONFLICTS
                            .with("kind", &id.kind)
                            .with("name", &id.name),
                    )
                    .with_hint(msg::APPLY_CONFLICTS_HINT)
                };
                return ApplyDocument {
                    conflicts: found,
                    error: Some(error),
                    before: live.as_ref().map(|l| shown(&to_value(l))),
                    ..base
                };
            }
            Err(kube::Error::Api(status)) if matches!(status.code, 400 | 422) => {
                let detail = status
                    .details
                    .as_ref()
                    .map(|d| {
                        d.causes
                            .iter()
                            .map(|c| format!("{}: {}", c.field, c.message))
                            .collect::<Vec<_>>()
                            .join("\n")
                    })
                    .filter(|d| !d.is_empty())
                    .unwrap_or_else(|| status.message.clone());
                let error = AppError::validation(
                    msg::APPLY_INVALID
                        .with("kind", &id.kind)
                        .with("name", &id.name),
                )
                .with_detail(detail);
                return ApplyDocument {
                    error: Some(error),
                    ..base
                };
            }
            Err(e) => {
                return ApplyDocument {
                    error: Some(self.failed_on(
                        "patch",
                        api,
                        namespace.as_deref(),
                        Some(&id.name),
                        &e,
                    )),
                    ..base
                }
            }
        };
        let after = to_value(&after);
        let before = live.as_ref().map(to_value);
        let action = match &before {
            None => ApplyAction::Created,
            Some(before) if same(before, &after) => ApplyAction::Unchanged,
            Some(_) => ApplyAction::Changed,
        };
        ApplyDocument {
            action,
            before: before.as_ref().map(shown_normalized),
            after: Some(shown_normalized(&after)),
            ..base
        }
    }
}

fn to_value(object: &DynamicObject) -> Value {
    serde_json::to_value(object).unwrap_or_default()
}

/// Whether two versions of an object hold the same, once normalized.
fn same(a: &Value, b: &Value) -> bool {
    let (mut a, mut b) = (a.clone(), b.clone());
    normalize(&mut a);
    normalize(&mut b);
    a == b
}

fn shown_normalized(value: &Value) -> String {
    let mut value = value.clone();
    normalize(&mut value);
    shown(&value)
}

fn failed_document(doc: &Value, id: Option<&Identity>, error: AppError) -> ApplyDocument {
    let text = |pointer: &str| {
        doc.pointer(pointer)
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned()
    };
    ApplyDocument {
        api_version: id.map_or_else(|| text("/apiVersion"), |i| i.api_version.clone()),
        kind: id.map_or_else(|| text("/kind"), |i| i.kind.clone()),
        namespace: id.and_then(|i| i.namespace.clone()),
        name: id.map_or_else(|| text("/metadata/name"), |i| i.name.clone()),
        action: ApplyAction::Failed,
        before: None,
        after: None,
        conflicts: vec![],
        error: Some(error),
    }
}

#[cfg(test)]
mod tests;

//! Objects of any type the Cluster serves, CRDs included: listed with the
//! columns the API server suggests (its Table rendering, the columns of
//! `kubectl get`, a CRD's `additionalPrinterColumns` among them), and each
//! one's metadata, conditions and YAML.

use k8s_openapi::api::core::v1::Event;
use kube::{
    api::{DynamicObject, ListParams},
    core::Request,
    Resource,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::{
    common::ConditionView,
    kinds::events::{self, EventView},
    tidy, ApiType, On, ResourceKind, PAGE,
};

/// A column of an object list, as the API server describes it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TableColumn {
    pub name: String,
    /// `string`, `integer`, `date`…
    #[serde(rename = "type")]
    pub kind: String,
    pub description: String,
    /// 0 for the columns `kubectl get` shows; more for those of `-o wide`.
    pub priority: i32,
}

/// One object of a list, with its cells in the columns' order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    pub name: String,
    pub namespace: Option<String>,
    pub cells: Vec<String>,
    pub created: Option<String>,
}

/// The objects of a type, as `kubectl get` lists them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ObjectTable {
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
}

/// An object it refers to as its owner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct OwnerRef {
    pub api_version: String,
    pub kind: String,
    pub name: String,
    pub controller: bool,
}

/// An object of any type, as its page shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ObjectDetail {
    pub name: String,
    pub namespace: Option<String>,
    pub created: Option<String>,
    pub labels: std::collections::BTreeMap<String, String>,
    pub annotations: std::collections::BTreeMap<String, String>,
    pub owners: Vec<OwnerRef>,
    /// Its `status.conditions`, when it has them.
    pub conditions: Vec<ConditionView>,
    pub yaml: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawTable {
    #[serde(default)]
    column_definitions: Vec<RawColumn>,
    #[serde(default)]
    rows: Vec<RawRow>,
    #[serde(default)]
    metadata: RawListMeta,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct RawListMeta {
    #[serde(rename = "continue")]
    continue_: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawColumn {
    name: String,
    #[serde(rename = "type", default)]
    kind: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    priority: i32,
}

#[derive(Deserialize)]
struct RawRow {
    #[serde(default)]
    cells: Vec<Value>,
    #[serde(default)]
    object: Value,
}

/// A cell as text: numbers and booleans as written, `<none>` for nothing.
fn cell(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        Value::Array(items) => items.iter().map(cell).collect::<Vec<_>>().join(", "),
        other => other.to_string(),
    }
}

/// The columns and rows of a Table answer; the `Name` column is kept out of
/// the cells, since each row carries its name.
pub fn table(body: Value) -> Result<(ObjectTable, Option<String>), serde_json::Error> {
    let raw: RawTable = serde_json::from_value(body)?;
    let name_at = raw
        .column_definitions
        .iter()
        .position(|c| c.name.eq_ignore_ascii_case("name"));
    let columns = raw
        .column_definitions
        .into_iter()
        .enumerate()
        .filter(|(i, _)| Some(*i) != name_at)
        .map(|(_, c)| TableColumn {
            name: c.name,
            kind: c.kind,
            description: c.description,
            priority: c.priority,
        })
        .collect();
    let rows = raw
        .rows
        .into_iter()
        .map(|row| {
            let meta = &row.object["metadata"];
            let text = |key: &str| meta[key].as_str().map(str::to_owned);
            TableRow {
                name: text("name")
                    .or_else(|| name_at.and_then(|i| row.cells.get(i)).map(cell))
                    .unwrap_or_default(),
                namespace: text("namespace"),
                created: text("creationTimestamp"),
                cells: row
                    .cells
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| Some(*i) != name_at)
                    .map(|(_, v)| cell(v))
                    .collect(),
            }
        })
        .collect();
    let next = raw.metadata.continue_.filter(|c| !c.is_empty());
    Ok((ObjectTable { columns, rows }, next))
}

/// The conditions in an object's `status.conditions`, of any shape that
/// has a type and a status.
pub fn conditions(object: &Value) -> Vec<ConditionView> {
    let text = |c: &Value, key: &str| c[key].as_str().map(str::to_owned).filter(|s| !s.is_empty());
    object["status"]["conditions"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|c| {
            Some(ConditionView {
                kind: text(c, "type")?,
                status: text(c, "status")?,
                reason: text(c, "reason"),
                message: text(c, "message"),
                last_transition: text(c, "lastTransitionTime"),
            })
        })
        .collect()
}

impl On {
    /// The objects of `api` in `namespace` (every one with `None`), as a table.
    pub async fn api_list(&self, api: &ApiType, namespace: Option<&str>) -> AppResult<ObjectTable> {
        let ar = api.api_resource();
        let path = DynamicObject::url_path(&ar, namespace.filter(|_| api.namespaced));
        let mut out = ObjectTable {
            columns: vec![],
            rows: vec![],
        };
        let mut params = ListParams::default().limit(PAGE);
        loop {
            let mut request = Request::new(&path).list(&params).map_err(|e| {
                AppError::new(
                    ErrorCode::ClusterApiError,
                    homeport_core::messages::UNREADABLE_DATA,
                )
                .with_cause(&e)
            })?;
            request.headers_mut().insert(
                http::header::ACCEPT,
                http::HeaderValue::from_static(
                    "application/json;as=Table;g=meta.k8s.io;v=v1,application/json",
                ),
            );
            let body: Value = self
                .client
                .request(request)
                .await
                .map_err(|e| self.failed_on("list", api, namespace, None, &e))?;
            let (page, next) = table(body).map_err(|e| {
                AppError::new(
                    ErrorCode::ClusterApiError,
                    homeport_core::messages::UNREADABLE_DATA,
                )
                .with_cause(&e)
            })?;
            if out.columns.is_empty() {
                out.columns = page.columns;
            }
            out.rows.extend(page.rows);
            match next {
                Some(token) => params = params.continue_token(&token),
                None => break,
            }
        }
        out.rows
            .sort_by(|a, b| (&a.namespace, &a.name).cmp(&(&b.namespace, &b.name)));
        Ok(out)
    }

    /// One object of `api`, for its page.
    pub async fn api_object(
        &self,
        api: &ApiType,
        namespace: Option<&str>,
        name: &str,
    ) -> AppResult<ObjectDetail> {
        let object = api
            .api(self.client.clone(), namespace)
            .get(name)
            .await
            .map_err(|e| self.failed_on("get", api, namespace, Some(name), &e))?;
        let meta = object.metadata.clone();
        let mut value = serde_json::to_value(&object)?;
        value["apiVersion"] = api.api_version().into();
        value["kind"] = api.kind.clone().into();
        let found = conditions(&value);
        tidy(&mut value);
        let yaml = serde_yaml_ng::to_string(&value).map_err(|e| {
            AppError::new(ErrorCode::Json, homeport_core::messages::UNREADABLE_DATA).with_cause(&e)
        })?;
        Ok(ObjectDetail {
            name: meta.name.clone().unwrap_or_default(),
            namespace: meta.namespace.clone(),
            created: super::common::created(&meta),
            labels: super::common::labels(&meta),
            annotations: super::common::annotations(&meta),
            owners: meta
                .owner_references
                .unwrap_or_default()
                .into_iter()
                .map(|o| OwnerRef {
                    api_version: o.api_version,
                    kind: o.kind,
                    name: o.name,
                    controller: o.controller == Some(true),
                })
                .collect(),
            conditions: found,
            yaml,
        })
    }

    /// The Events about an object of any kind, warnings first.
    pub async fn api_object_events(
        &self,
        kind: &str,
        namespace: Option<&str>,
        name: &str,
    ) -> AppResult<Vec<EventView>> {
        let fields = format!("involvedObject.kind={kind},involvedObject.name={name}");
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn a_table_answer_becomes_columns_and_rows() {
        let body = json!({
            "kind": "Table",
            "columnDefinitions": [
                { "name": "Name", "type": "string", "priority": 0 },
                { "name": "Ready", "type": "string", "priority": 0, "description": "Ready condition" },
                { "name": "Secret", "type": "string", "priority": 1 },
                { "name": "Age", "type": "date", "priority": 0 },
            ],
            "rows": [
                {
                    "cells": ["web-tls", "True", "web-tls-secret", "3d"],
                    "object": { "metadata": {
                        "name": "web-tls", "namespace": "shop",
                        "creationTimestamp": "2026-11-05T10:00:00Z"
                    } }
                },
                { "cells": ["api-tls", null, 2, "1h"], "object": { "metadata": { "name": "api-tls" } } }
            ],
            "metadata": { "continue": "next-page" }
        });
        let (t, next) = table(body).unwrap();
        assert_eq!(next.as_deref(), Some("next-page"));
        let names: Vec<_> = t.columns.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["Ready", "Secret", "Age"]);
        assert_eq!(t.columns[1].priority, 1);
        assert_eq!(t.columns[0].description, "Ready condition");
        assert_eq!(t.rows[0].name, "web-tls");
        assert_eq!(t.rows[0].namespace.as_deref(), Some("shop"));
        assert_eq!(t.rows[0].cells, ["True", "web-tls-secret", "3d"]);
        assert_eq!(t.rows[0].created.as_deref(), Some("2026-11-05T10:00:00Z"));
        assert_eq!(t.rows[1].cells, ["", "2", "1h"]);
    }

    #[test]
    fn conditions_are_read_from_any_status() {
        let object = json!({ "status": { "conditions": [
            { "type": "Ready", "status": "False", "reason": "Pending", "message": "Issuing" },
            { "status": "True" },
        ] } });
        let found = conditions(&object);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].kind, "Ready");
        assert_eq!(found[0].reason.as_deref(), Some("Pending"));
        assert!(conditions(&json!({ "spec": {} })).is_empty());
    }
}

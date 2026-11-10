//! An Instance's snapshots: listing, taking (stateful when the Instance
//! allows it, with an expiry), restoring and deleting them.

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::http::Request;

use crate::{
    client::{scoped, segment, Incus},
    instances, messages as msg, operations,
};

/// One snapshot of an Instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusSnapshot {
    pub name: String,
    /// RFC 3339.
    pub created_at: String,
    /// When Incus deletes it on its own; `None` for never.
    pub expires_at: Option<String>,
    /// Whether it holds the running state (memory) too.
    pub stateful: bool,
    /// Its size in bytes, when the storage driver can tell.
    pub size: Option<i64>,
}

#[derive(Deserialize)]
struct RawSnapshot {
    name: String,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    expires_at: String,
    #[serde(default)]
    stateful: bool,
    #[serde(default)]
    size: Option<i64>,
}

fn never(time: &str) -> bool {
    time.is_empty() || time.starts_with("0001-")
}

impl RawSnapshot {
    fn into_snapshot(self) -> IncusSnapshot {
        IncusSnapshot {
            name: self.name,
            created_at: self.created_at,
            expires_at: (!never(&self.expires_at)).then_some(self.expires_at),
            stateful: self.stateful,
            size: self.size.filter(|s| *s >= 0),
        }
    }
}

/// What the Take snapshot dialog asks for.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotRequest {
    /// Empty lets Incus name it `snapN`.
    pub name: String,
    pub stateful: bool,
    /// RFC 3339; `None` keeps it until deleted.
    pub expires_at: Option<String>,
}

fn path(instance: &str) -> String {
    format!("{}/snapshots", instances::path(instance))
}

/// The snapshots of the Instance `instance`, oldest first.
pub async fn list(incus: &Incus, project: &str, instance: &str) -> AppResult<Vec<IncusSnapshot>> {
    let raw: Vec<RawSnapshot> = incus
        .sync(Request::get(scoped(
            &format!("{}?recursion=1", path(instance)),
            project,
        )))
        .await?;
    let mut snapshots: Vec<IncusSnapshot> =
        raw.into_iter().map(RawSnapshot::into_snapshot).collect();
    snapshots.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    Ok(snapshots)
}

/// The body of `POST …/snapshots` for `request`.
pub(crate) fn create_body(request: &SnapshotRequest) -> AppResult<serde_json::Value> {
    let mut body = serde_json::json!({
        "name": request.name.trim(),
        "stateful": request.stateful,
    });
    if let Some(expiry) = request
        .expires_at
        .as_deref()
        .filter(|e| !e.trim().is_empty())
    {
        let parsed: jiff::Timestamp = expiry
            .trim()
            .parse()
            .map_err(|_| AppError::validation(msg::SNAPSHOT_EXPIRY_INVALID.with("time", expiry)))?;
        if parsed <= jiff::Timestamp::now() {
            return Err(AppError::validation(msg::SNAPSHOT_EXPIRY_PAST));
        }
        body["expires_at"] = parsed.to_string().into();
    }
    Ok(body)
}

/// Takes a snapshot of the Instance `instance`.
pub async fn create(
    incus: &Incus,
    project: &str,
    instance: &str,
    request: &SnapshotRequest,
) -> AppResult<()> {
    let name = request.name.trim();
    if name.contains('/') {
        return Err(AppError::validation(
            msg::SNAPSHOT_NAME_INVALID.with("name", name),
        ));
    }
    let body = create_body(request)?;
    let http = Request::post(scoped(&path(instance), project)).json(&body)?;
    incus
        .run(project, http, operations::LONG)
        .await
        .map(drop)
        .map_err(|e| {
            if e.is(ErrorCode::NameInUse) {
                AppError::new(
                    ErrorCode::NameInUse,
                    msg::SNAPSHOT_EXISTS.with("name", name),
                )
                .with_hint(msg::ALREADY_EXISTS_HINT)
                .with_detail(e.detail.unwrap_or(e.message))
            } else if request.stateful
                && !e.is(ErrorCode::StatefulUnsupported)
                && !e.is(ErrorCode::NotFound)
            {
                // Stateful snapshots fail in many words; all mean the same.
                AppError::new(ErrorCode::StatefulUnsupported, msg::STATEFUL_UNSUPPORTED)
                    .with_hint(msg::STATEFUL_UNSUPPORTED_HINT)
                    .with_command(format!(
                        "incus config set {instance} migration.stateful=true"
                    ))
                    .with_detail(e.detail.unwrap_or(e.message))
            } else {
                e
            }
        })
}

/// Restores the Instance `instance` to its snapshot `snapshot`: what
/// changed since is lost. `stateful` restores the running state it holds.
pub async fn restore(
    incus: &Incus,
    project: &str,
    instance: &str,
    snapshot: &str,
    stateful: bool,
) -> AppResult<()> {
    let body = serde_json::json!({ "restore": snapshot, "stateful": stateful });
    let http = Request::put(scoped(&instances::path(instance), project)).json(&body)?;
    incus
        .run(project, http, operations::LONG)
        .await
        .map(drop)
        .map_err(|e| not_found(e, snapshot))
}

/// Deletes the snapshot `snapshot` of the Instance `instance`.
pub async fn delete(incus: &Incus, project: &str, instance: &str, snapshot: &str) -> AppResult<()> {
    let http = Request::delete(scoped(
        &format!("{}/{}", path(instance), segment(snapshot)),
        project,
    ));
    incus
        .run(project, http, operations::LONG)
        .await
        .map(drop)
        .map_err(|e| not_found(e, snapshot))
}

fn not_found(error: AppError, name: &str) -> AppError {
    if error.is(ErrorCode::NotFound) {
        AppError::new(
            ErrorCode::NotFound,
            msg::SNAPSHOT_NOT_FOUND.with("name", name),
        )
        .with_detail(error.detail.unwrap_or(error.message))
    } else {
        error
    }
}

#[cfg(test)]
mod tests;

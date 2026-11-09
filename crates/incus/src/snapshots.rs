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
mod tests {
    use super::*;

    #[test]
    fn snapshots_read_as_incus_lists_them() {
        // From GET /1.0/instances/t1/snapshots?recursion=1 on Incus 6.0.
        let raw: RawSnapshot = serde_json::from_value(serde_json::json!({
            "expires_at": "0001-01-01T00:00:00Z",
            "architecture": "x86_64",
            "created_at": "2026-11-08T23:06:10.95971139Z",
            "name": "snapA",
            "stateful": false,
            "size": -1
        }))
        .unwrap();
        let snapshot = raw.into_snapshot();
        assert_eq!(snapshot.name, "snapA");
        assert_eq!(snapshot.expires_at, None);
        assert_eq!(snapshot.size, None);
    }

    #[test]
    fn a_new_snapshot_may_expire_but_not_in_the_past() {
        let mut request = SnapshotRequest {
            name: " before-upgrade ".into(),
            stateful: true,
            expires_at: None,
        };
        assert_eq!(
            create_body(&request).unwrap(),
            serde_json::json!({ "name": "before-upgrade", "stateful": true })
        );
        request.expires_at = Some("2999-01-01T00:00:00Z".into());
        assert_eq!(
            create_body(&request).unwrap()["expires_at"],
            "2999-01-01T00:00:00Z"
        );
        request.expires_at = Some("2020-01-01T00:00:00Z".into());
        assert_eq!(
            create_body(&request).unwrap_err().code,
            ErrorCode::Validation
        );
        request.expires_at = Some("tomorrow".into());
        assert_eq!(
            create_body(&request).unwrap_err().code,
            ErrorCode::Validation
        );
    }

    #[tokio::test]
    #[ignore = "needs a local Incus"]
    async fn incus_live_snapshots_are_taken_restored_and_deleted() {
        use crate::{
            instances::{tests::live_instance, InstanceAction},
            terminal::{IncusExecOptions, IncusTerminal},
        };
        let incus = crate::instances::tests::live().await;
        let name = format!("homeport-snap-{}", std::process::id());
        live_instance(&incus, &name).await;
        instances::act(&incus, "default", &name, InstanceAction::Start, false)
            .await
            .unwrap();
        let run = |command: &str| IncusExecOptions {
            command: Some(command.to_owned()),
            user: None,
            cwd: None,
            cols: 80,
            rows: 24,
        };
        let exec = |command: &'static str| {
            let incus = incus.clone();
            let name = name.clone();
            async move {
                let (session, output) =
                    IncusTerminal::exec(&incus, "default", &name, &run(command))
                        .await
                        .unwrap();
                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                tokio::spawn(crate::terminal::pump(session, output, move |e| {
                    tx.send(e).is_ok()
                }));
                let mut text = String::new();
                while let Some(e) = rx.recv().await {
                    match e {
                        homeport_core::terminal::ExecEvent::Output { data } => text.push_str(&data),
                        _ => break,
                    }
                }
                text
            }
        };
        exec("touch /root/kept").await;

        let request = SnapshotRequest {
            name: String::new(),
            stateful: false,
            expires_at: None,
        };
        create(&incus, "default", &name, &request).await.unwrap();
        let taken = list(&incus, "default", &name).await.unwrap();
        assert_eq!(taken.len(), 1);
        assert_eq!(taken[0].name, "snap0");

        exec("rm /root/kept").await;
        restore(&incus, "default", &name, "snap0", false)
            .await
            .unwrap();
        let back = exec("ls /root").await;
        assert!(back.contains("kept"), "{back:?}");

        let again = create(
            &incus,
            "default",
            &name,
            &SnapshotRequest {
                name: "snap0".into(),
                stateful: false,
                expires_at: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(again.code, ErrorCode::NameInUse, "{again:?}");

        delete(&incus, "default", &name, "snap0").await.unwrap();
        assert!(list(&incus, "default", &name).await.unwrap().is_empty());
        instances::delete(&incus, "default", &name, true)
            .await
            .unwrap();
    }
}

//! Incus's background operations: what an action answers with when it
//! takes time, followed here until it ends.

use std::{collections::HashMap, time::Duration};

use serde::Deserialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::http::Request;

use crate::{
    client::{scoped, segment, Incus},
    error, messages as msg,
};

/// How often a running operation is asked about.
const POLL: Duration = Duration::from_millis(300);

/// How long a short action (start, stop, delete) may take.
pub const SHORT: Duration = Duration::from_secs(120);
/// How long an action that may download or copy data may take.
pub const LONG: Duration = Duration::from_secs(30 * 60);

/// One operation, as Incus describes it.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Operation {
    #[serde(default)]
    pub id: String,
    /// `task`, `websocket` or `token`.
    #[serde(default)]
    pub class: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub status_code: u16,
    /// What the operation reports while it runs (progress) or once done
    /// (the websockets' secrets, a command's exit code).
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    /// Incus's sentence when it failed.
    #[serde(default)]
    pub err: String,
    #[serde(default)]
    pub resources: Option<HashMap<String, Vec<String>>>,
}

impl Operation {
    /// A synchronous answer, which needs no waiting.
    pub(crate) fn done() -> Self {
        Self {
            status: "Success".into(),
            status_code: 200,
            ..Self::default()
        }
    }

    /// Whether it has ended, one way or another.
    pub fn finished(&self) -> bool {
        self.status_code >= 200
    }

    /// A string field of its metadata.
    pub fn field(&self, name: &str) -> Option<&str> {
        self.metadata.as_ref()?.get(name)?.as_str()
    }

    /// How far along a download or copy is, as Incus words it: for
    /// example "rootfs: 42% (12.30MB/s)".
    pub fn progress(&self) -> Option<&str> {
        let metadata = self.metadata.as_ref()?;
        [
            "download_progress",
            "create_instance_from_image_unpack_progress",
            "progress",
        ]
        .iter()
        .find_map(|key| match metadata.get(*key)? {
            serde_json::Value::String(s) if !s.is_empty() => Some(s.as_str()),
            serde_json::Value::Object(o) => o.get("text").and_then(|t| t.as_str()),
            _ => None,
        })
    }

    /// Its result: itself when it succeeded, or why it didn't.
    pub(crate) fn outcome(self) -> AppResult<Self> {
        match self.status_code {
            200 => Ok(self),
            401 => {
                Err(AppError::new(ErrorCode::Conflict, msg::OPERATION_CANCELLED)
                    .with_detail(&self.err))
            }
            _ => Err(error::operation(if self.err.is_empty() {
                &self.status
            } else {
                &self.err
            })),
        }
    }
}

/// Waits for `operation` to end, calling `progress` with each state seen
/// while it runs; fails with Incus's reason if it fails, or after `timeout`.
pub(crate) async fn wait(
    incus: &Incus,
    project: &str,
    operation: Operation,
    timeout: Duration,
    mut progress: impl FnMut(&Operation),
) -> AppResult<Operation> {
    let started = tokio::time::Instant::now();
    let mut current = operation;
    loop {
        if current.finished() {
            return current.outcome();
        }
        progress(&current);
        if started.elapsed() >= timeout {
            return Err(AppError::new(
                ErrorCode::Timeout,
                msg::OPERATION_TIMED_OUT.with("seconds", timeout.as_secs()),
            )
            .with_hint(msg::OPERATION_TIMED_OUT_HINT)
            .with_detail(format!("operation {}", current.id)));
        }
        tokio::time::sleep(POLL).await;
        current = get(incus, project, &current.id).await?;
    }
}

/// The current state of the operation `id`.
pub(crate) async fn get(incus: &Incus, project: &str, id: &str) -> AppResult<Operation> {
    incus
        .sync(Request::get(scoped(
            &format!("/1.0/operations/{}", segment(id)),
            project,
        )))
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(json: serde_json::Value) -> Operation {
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn progress_is_read_from_the_metadata() {
        let running = operation(serde_json::json!({
            "id": "6916c8a6",
            "class": "task",
            "status": "Running",
            "status_code": 103,
            "metadata": { "download_progress": "rootfs: 42% (12.30MB/s)" },
            "err": ""
        }));
        assert!(!running.finished());
        assert_eq!(running.progress(), Some("rootfs: 42% (12.30MB/s)"));
    }

    #[test]
    fn a_failed_operation_says_why() {
        let failed = operation(serde_json::json!({
            "id": "6916c8a6",
            "status": "Failure",
            "status_code": 400,
            "err": "Failed getting remote image info: The requested image couldn't be found"
        }));
        assert!(failed.finished());
        assert_eq!(failed.outcome().unwrap_err().code, ErrorCode::ImageNotFound);

        let done = operation(serde_json::json!({ "status": "Success", "status_code": 200 }));
        assert!(done.outcome().is_ok());
    }
}

//! What every image pull or push sends the frontend: a Docker Engine's,
//! and containerd's through its Transfer service.

use serde::Serialize;

use crate::error::AppError;

/// Progress of a pull or a push.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum PullEvent {
    /// Progress of one layer (or a general status when `id` is absent).
    /// Pushes never name the layer: the Engine API client drops its id.
    Progress {
        id: Option<String>,
        status: String,
        current: Option<i64>,
        total: Option<i64>,
    },
    Done,
    Error {
        error: AppError,
    },
}

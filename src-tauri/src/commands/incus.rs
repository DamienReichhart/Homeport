//! What Incus Hosts offer, through Incus's REST API (ADR 0021). Every
//! command about a project-scoped object names its project; a Host whose
//! Engine isn't Incus answers `EngineUnsupported`.

use tauri::State;

use homeport_incus::projects::{self, IncusProject};

use crate::{app::AppState, error::AppResult};

/// The projects the Host's connection may use, with what each holds.
#[tauri::command]
#[specta::specta]
pub async fn incus_projects(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<Vec<IncusProject>> {
    projects::list(&state.hosts.registry.incus(&host_id)?).await
}

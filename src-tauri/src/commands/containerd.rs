//! What containerd Hosts offer, through containerd's gRPC API (ADR 0020).
//! Every command but the namespaces' own names the namespace it's about;
//! a Host whose Engine isn't containerd answers `EngineUnsupported`.

use tauri::State;

use homeport_containerd::namespaces::{self, ContainerdNamespace};

use crate::{app::AppState, error::AppResult};

/// Every namespace of the Host, with what each holds.
#[tauri::command]
#[specta::specta]
pub async fn containerd_namespaces(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<Vec<ContainerdNamespace>> {
    namespaces::list(&state.hosts.registry.containerd(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn containerd_namespace_create(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<()> {
    namespaces::create(&state.hosts.registry.containerd(&host_id)?, &name).await
}

/// Removes an empty namespace; one that holds anything is refused.
#[tauri::command]
#[specta::specta]
pub async fn containerd_namespace_remove(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<()> {
    namespaces::remove(&state.hosts.registry.containerd(&host_id)?, &name).await
}

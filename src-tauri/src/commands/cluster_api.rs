//! Any type of object a Cluster serves, CRDs included: discovering the
//! types, listing their objects as `kubectl get` does, and each object.

use tauri::{ipc::Channel, State};

use homeport_clusters::{
    discovery::ApiResourceInfo,
    resources::{self, ApiType, EventView, ObjectDetail, ObjectTable},
};
use homeport_core::streams::StreamId;

use super::clusters::on;
use crate::{app::AppState, error::AppResult};

/// Every type the Cluster serves, at each group's preferred version, each
/// marked with whether the credentials may list it in `namespace`.
#[tauri::command]
#[specta::specta]
pub async fn clusters_api_resources(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<ApiResourceInfo>> {
    on(&state, &id)?.discover_for(namespace.as_deref()).await
}

/// The objects of `api` in `namespace` (every one without), with the
/// columns the API server suggests.
#[tauri::command]
#[specta::specta]
pub async fn clusters_api_list(
    state: State<'_, AppState>,
    id: String,
    api: ApiType,
    namespace: Option<String>,
) -> AppResult<ObjectTable> {
    on(&state, &id)?.api_list(&api, namespace.as_deref()).await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_api_get(
    state: State<'_, AppState>,
    id: String,
    api: ApiType,
    namespace: Option<String>,
    name: String,
) -> AppResult<ObjectDetail> {
    on(&state, &id)?
        .api_object(&api, namespace.as_deref(), &name)
        .await
}

/// The Events about an object of any `kind`, warnings first.
#[tauri::command]
#[specta::specta]
pub async fn clusters_api_object_events(
    state: State<'_, AppState>,
    id: String,
    kind: String,
    namespace: Option<String>,
    name: String,
) -> AppResult<Vec<EventView>> {
    on(&state, &id)?
        .api_object_events(&kind, namespace.as_deref(), &name)
        .await
}

/// Told when the objects of a type changed.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ApiChanged {
    pub api: ApiType,
}

/// Watches the objects of `api` in `namespace` (every one without) and
/// says when they change, until cancelled.
#[tauri::command]
#[specta::specta]
pub async fn clusters_api_watch_subscribe(
    state: State<'_, AppState>,
    id: String,
    api: ApiType,
    namespace: Option<String>,
    on_event: Channel<ApiChanged>,
) -> AppResult<StreamId> {
    let (client, _) = state.clusters.registry.client(&id)?;
    let told = api.clone();
    Ok(state
        .streams
        .spawn(resources::watch_api(client, api, namespace, move || {
            on_event.send(ApiChanged { api: told.clone() }).is_ok()
        })))
}

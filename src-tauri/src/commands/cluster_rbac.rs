//! A Cluster's RBAC, read-only: its roles, bindings and subjects, what a
//! subject may do (computed from bindings), and what Homeport's own
//! credentials may do (asked of the API server).

use tauri::State;

use homeport_clusters::rbac::{
    AccessAnswer, AccessCheck, OwnPermissions, RbacIndex, Subject, SubjectPermissions,
};

use super::clusters::on;
use crate::{app::AppState, error::AppResult};

/// The Roles and RoleBindings of `namespace` (every one without), the
/// ClusterRoles and ClusterRoleBindings, and the subjects they name.
#[tauri::command]
#[specta::specta]
pub async fn clusters_rbac_index(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
) -> AppResult<RbacIndex> {
    on(&state, &id)?.rbac_index(namespace.as_deref()).await
}

/// What `subject` may do, from the bindings naming it or its groups.
#[tauri::command]
#[specta::specta]
pub async fn clusters_rbac_subject(
    state: State<'_, AppState>,
    id: String,
    subject: Subject,
    namespace: Option<String>,
) -> AppResult<SubjectPermissions> {
    on(&state, &id)?
        .rbac_subject(&subject, namespace.as_deref())
        .await
}

/// What the credentials may do in `namespace` (the default one without).
#[tauri::command]
#[specta::specta]
pub async fn clusters_rbac_own(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
) -> AppResult<OwnPermissions> {
    on(&state, &id)?.own_permissions(namespace.as_deref()).await
}

/// Whether the credentials may do what `check` asks.
#[tauri::command]
#[specta::specta]
pub async fn clusters_rbac_can_i(
    state: State<'_, AppState>,
    id: String,
    check: AccessCheck,
) -> AppResult<AccessAnswer> {
    on(&state, &id)?.can_i(&check).await
}

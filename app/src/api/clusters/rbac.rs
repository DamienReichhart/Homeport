//! A Cluster's RBAC, read-only: its roles, bindings and subjects, what a
//! subject may do (computed from bindings), and what Homeport's own
//! credentials may do (asked of the API server).

use homeport_core::error::AppResult;
use homeport_kubernetes::rbac::{
    AccessAnswer, AccessCheck, OwnPermissions, RbacIndex, Subject, SubjectPermissions,
};

use super::on;
use crate::app::AppState;

/// The Roles and RoleBindings of `namespace` (every one without), the
/// ClusterRoles and ClusterRoleBindings, and the subjects they name.
pub async fn clusters_rbac_index(
    state: &AppState,
    id: String,
    namespace: Option<String>,
) -> AppResult<RbacIndex> {
    on(state, &id)?.rbac_index(namespace.as_deref()).await
}

/// What `subject` may do, from the bindings naming it or its groups.
pub async fn clusters_rbac_subject(
    state: &AppState,
    id: String,
    subject: Subject,
    namespace: Option<String>,
) -> AppResult<SubjectPermissions> {
    on(state, &id)?
        .rbac_subject(&subject, namespace.as_deref())
        .await
}

/// What the credentials may do in `namespace` (the default one without).
pub async fn clusters_rbac_own(
    state: &AppState,
    id: String,
    namespace: Option<String>,
) -> AppResult<OwnPermissions> {
    on(state, &id)?.own_permissions(namespace.as_deref()).await
}

/// Whether the credentials may do what `check` asks.
pub async fn clusters_rbac_can_i(
    state: &AppState,
    id: String,
    check: AccessCheck,
) -> AppResult<AccessAnswer> {
    on(state, &id)?.can_i(&check).await
}

#[cfg(test)]
mod tests {
    use homeport_kubernetes::rbac::SubjectKind;

    use super::super::tests::{gone, unknown};
    use super::*;
    use crate::app::testing::app;

    #[tokio::test]
    async fn an_unknown_cluster_has_no_rbac_to_read() {
        let app = app();
        let state = &app.state;
        let subject = Subject {
            kind: SubjectKind::User,
            name: "jane".into(),
            namespace: None,
        };
        let check = AccessCheck {
            verb: "list".into(),
            resource: "pods".into(),
            ..Default::default()
        };
        assert!(gone(clusters_rbac_index(state, unknown(), None).await));
        assert!(gone(
            clusters_rbac_subject(state, unknown(), subject, None).await
        ));
        assert!(gone(clusters_rbac_own(state, unknown(), None).await));
        assert!(gone(clusters_rbac_can_i(state, unknown(), check).await));
    }
}

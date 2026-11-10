//! A Cluster's RBAC, read-only: its Roles and ClusterRoles (aggregation
//! resolved), RoleBindings and ClusterRoleBindings, the subjects they name,
//! and what each subject may do where, computed from those bindings.
//!
//! Computing another subject's permissions here, rather than asking the
//! API server while impersonating it, needs no `impersonate` right; it is a
//! best effort that can't see webhook or ABAC authorizers, nor the groups
//! an authenticator gives a user. What Homeport's own credentials may do is
//! the API server's answer instead ([`review`]).

mod aggregate;
pub mod index;
pub mod review;
pub mod rules;
#[cfg(test)]
mod tests;

use std::fmt::Debug;

use k8s_openapi::api::rbac::v1::{ClusterRole, ClusterRoleBinding, Role, RoleBinding};
use kube::api::{Api, ListParams};
use serde::{de::DeserializeOwned, Serialize};

use homeport_core::error::{AppError, AppResult};

use crate::resources::{ApiType, On};

pub use self::{
    index::{
        AppliedBinding, BindingKind, BindingView, PermissionScope, Rbac, RoleKind, RoleView,
        Subject, SubjectEntry, SubjectKind,
    },
    review::{AccessAnswer, AccessCheck, OwnPermissions, UserView},
    rules::{Grants, ResourceGrant, Rule, UrlGrant},
};

/// The API group of RBAC's objects.
pub const GROUP: &str = "rbac.authorization.k8s.io";

fn rbac_type(kind: &str, plural: &str, namespaced: bool) -> ApiType {
    ApiType {
        group: GROUP.into(),
        version: "v1".into(),
        kind: kind.into(),
        plural: plural.into(),
        namespaced,
    }
}

/// The RBAC objects of a Cluster, in a Namespace (with the cluster-wide ones).
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RbacIndex {
    /// The Namespace whose Roles and RoleBindings these are; every one with `None`.
    pub namespace: Option<String>,
    /// ClusterRoles first, then Roles by Namespace.
    pub roles: Vec<RoleView>,
    /// ClusterRoleBindings first, then RoleBindings by Namespace.
    pub bindings: Vec<BindingView>,
    /// Everyone a binding names, Users, then Groups, then ServiceAccounts.
    pub subjects: Vec<SubjectEntry>,
    /// The lists the credentials may not read, each as why: what is shown
    /// is computed without them.
    pub refused: Vec<AppError>,
}

/// What a subject may do, as its bindings grant it.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SubjectPermissions {
    pub subject: Subject,
    /// The Namespace its RoleBindings were read in; every one with `None`.
    pub namespace: Option<String>,
    /// The groups Kubernetes puts it in by itself, whose bindings count too.
    pub groups: Vec<String>,
    /// The bindings that apply, which grants refer to by index.
    pub bindings: Vec<AppliedBinding>,
    /// Cluster-wide first, then by Namespace.
    pub scopes: Vec<PermissionScope>,
    pub refused: Vec<AppError>,
}

/// Every item of `api`, a page at a time.
async fn list_all<K>(api: &Api<K>) -> Result<Vec<K>, kube::Error>
where
    K: Clone + DeserializeOwned + Debug,
{
    let mut items = vec![];
    let mut params = ListParams::default().limit(500);
    loop {
        let page = api.list(&params).await?;
        items.extend(page.items);
        match page.metadata.continue_ {
            Some(token) if !token.is_empty() => params = params.continue_token(&token),
            _ => return Ok(items),
        }
    }
}

impl On {
    /// Lists `api`'s objects; a refusal is kept in `refused` and leaves the
    /// list empty, any other failure fails.
    async fn list_or_refused<K>(
        &self,
        api: Api<K>,
        ty: &ApiType,
        namespace: Option<&str>,
        refused: &mut Vec<AppError>,
    ) -> AppResult<Vec<K>>
    where
        K: Clone + DeserializeOwned + Debug,
    {
        match list_all(&api).await {
            Ok(items) => Ok(items),
            Err(e) => {
                let error = self.failed_on("list", ty, namespace, None, &e);
                match &e {
                    kube::Error::Api(status) if status.code == 403 => {
                        refused.push(error);
                        Ok(vec![])
                    }
                    _ => Err(error),
                }
            }
        }
    }

    /// The Roles and RoleBindings of `namespace` (every one with `None`)
    /// and the cluster-wide ones, and the lists that were refused. Fails
    /// with the refusal when none of them could be read.
    pub async fn rbac(&self, namespace: Option<&str>) -> AppResult<(Rbac, Vec<AppError>)> {
        let c = || self.client.clone();
        let (roles_api, bindings_api) = match namespace {
            Some(ns) => (Api::namespaced(c(), ns), Api::namespaced(c(), ns)),
            None => (Api::all(c()), Api::all(c())),
        };
        let types = [
            rbac_type("Role", "roles", true),
            rbac_type("RoleBinding", "rolebindings", true),
            rbac_type("ClusterRole", "clusterroles", false),
            rbac_type("ClusterRoleBinding", "clusterrolebindings", false),
        ];
        let (mut r1, mut r2, mut r3, mut r4) = (vec![], vec![], vec![], vec![]);
        let (roles, role_bindings, cluster_roles, cluster_role_bindings) = futures_util::join!(
            self.list_or_refused::<Role>(roles_api, &types[0], namespace, &mut r1),
            self.list_or_refused::<RoleBinding>(bindings_api, &types[1], namespace, &mut r2),
            self.list_or_refused::<ClusterRole>(Api::all(c()), &types[2], None, &mut r3),
            self.list_or_refused::<ClusterRoleBinding>(Api::all(c()), &types[3], None, &mut r4),
        );
        let (roles, role_bindings, cluster_roles, cluster_role_bindings) = (
            roles?,
            role_bindings?,
            cluster_roles?,
            cluster_role_bindings?,
        );
        let refused: Vec<AppError> = [r1, r2, r3, r4].into_iter().flatten().collect();
        if refused.len() == types.len() {
            return Err(refused.into_iter().next().expect("four refusals"));
        }
        let mut views: Vec<RoleView> = index::cluster_roles(&cluster_roles);
        views.extend(roles.iter().map(index::role));
        let mut bindings: Vec<BindingView> = cluster_role_bindings
            .iter()
            .map(index::cluster_role_binding)
            .collect();
        bindings.extend(role_bindings.iter().map(index::role_binding));
        Ok((Rbac::new(views, bindings), refused))
    }

    pub async fn rbac_index(&self, namespace: Option<&str>) -> AppResult<RbacIndex> {
        let (rbac, refused) = self.rbac(namespace).await?;
        Ok(RbacIndex {
            namespace: namespace.map(str::to_owned),
            subjects: rbac.subjects(),
            roles: rbac.roles,
            bindings: rbac.bindings,
            refused,
        })
    }

    /// What `subject` may do, from the bindings that name it or its groups.
    pub async fn rbac_subject(
        &self,
        subject: &Subject,
        namespace: Option<&str>,
    ) -> AppResult<SubjectPermissions> {
        let (rbac, refused) = self.rbac(namespace).await?;
        let bindings = rbac.applied(subject);
        Ok(SubjectPermissions {
            subject: subject.clone(),
            namespace: namespace.map(str::to_owned),
            groups: subject.implied_groups(),
            scopes: rbac.permissions(&bindings),
            bindings,
            refused,
        })
    }
}

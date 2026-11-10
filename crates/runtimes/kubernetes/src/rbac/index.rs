//! The RBAC objects of a Cluster as views, and the index from a subject to
//! the bindings naming it and what their roles grant, by Namespace.

use std::collections::{BTreeMap, BTreeSet};

use k8s_openapi::api::rbac::v1::{
    ClusterRole, ClusterRoleBinding, Role, RoleBinding, RoleRef, Subject as K8sSubject,
};
use serde::{Deserialize, Serialize};

use super::{
    aggregate,
    rules::{merge, Grants, Rule},
};
use crate::resources::common::{created, labels};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, specta::Type,
)]
pub enum RoleKind {
    Role,
    ClusterRole,
}

/// A Role (in its Namespace) or a ClusterRole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RoleView {
    pub kind: RoleKind,
    pub name: String,
    /// A Role's; `None` for a ClusterRole.
    pub namespace: Option<String>,
    /// With aggregation resolved, for an aggregated ClusterRole.
    pub rules: Vec<Rule>,
    /// The label selectors of its `aggregationRule`, as kubectl writes them;
    /// empty when it isn't aggregated.
    pub aggregation: Vec<String>,
    /// The ClusterRoles those selectors match, whose rules it holds.
    pub aggregated_from: Vec<String>,
    pub labels: BTreeMap<String, String>,
    pub created: Option<String>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, specta::Type,
)]
pub enum SubjectKind {
    User,
    Group,
    ServiceAccount,
}

/// Who a binding grants its role to.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub kind: SubjectKind,
    pub name: String,
    /// A ServiceAccount's; `None` for a User or Group.
    pub namespace: Option<String>,
}

impl Subject {
    /// The user name a ServiceAccount authenticates as.
    pub fn user_name(&self) -> String {
        match (self.kind, &self.namespace) {
            (SubjectKind::ServiceAccount, Some(ns)) => {
                format!("system:serviceaccount:{ns}:{}", self.name)
            }
            _ => self.name.clone(),
        }
    }

    /// The groups Kubernetes puts it in by itself, which bindings can name:
    /// every ServiceAccount's and its Namespace's, and every signed-in user's.
    /// A user's other groups come from the authenticator and can't be known.
    pub fn implied_groups(&self) -> Vec<String> {
        match (self.kind, &self.namespace) {
            (SubjectKind::ServiceAccount, Some(ns)) => vec![
                "system:serviceaccounts".into(),
                format!("system:serviceaccounts:{ns}"),
                AUTHENTICATED.into(),
            ],
            (SubjectKind::User, _) if self.name == "system:anonymous" => {
                vec!["system:unauthenticated".into()]
            }
            (SubjectKind::User, _) => vec![AUTHENTICATED.into()],
            _ => vec![],
        }
    }
}

const AUTHENTICATED: &str = "system:authenticated";

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, specta::Type,
)]
pub enum BindingKind {
    RoleBinding,
    ClusterRoleBinding,
}

/// A RoleBinding (in its Namespace) or a ClusterRoleBinding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BindingView {
    pub kind: BindingKind,
    pub name: String,
    pub namespace: Option<String>,
    pub role_kind: RoleKind,
    pub role_name: String,
    /// Whether that role was found among those listed.
    pub role_found: bool,
    /// ServiceAccounts' Namespaces filled in as Kubernetes reads them.
    pub subjects: Vec<Subject>,
    pub created: Option<String>,
}

impl BindingView {
    /// The Namespace of the Role it binds: a RoleBinding's, `None` for a ClusterRole.
    fn role_namespace(&self) -> Option<&str> {
        match self.role_kind {
            RoleKind::Role => self.namespace.as_deref(),
            RoleKind::ClusterRole => None,
        }
    }

    /// How it names `subject`: directly (a ServiceAccount also by its user
    /// name), else through one of the groups given, or not at all.
    pub fn names(&self, subject: &Subject, groups: &[String]) -> Option<Through> {
        let user = subject.user_name();
        let direct = self.subjects.iter().any(|s| {
            s == subject
                || (subject.kind != SubjectKind::Group
                    && s.kind == SubjectKind::User
                    && s.name == user)
        });
        if direct {
            return Some(Through::Direct);
        }
        self.subjects
            .iter()
            .find(|s| s.kind == SubjectKind::Group && groups.contains(&s.name))
            .map(|s| Through::Group(s.name.clone()))
    }
}

/// How a binding names a subject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Through {
    Direct,
    Group(String),
}

pub fn role(r: &Role) -> RoleView {
    RoleView {
        kind: RoleKind::Role,
        name: r.metadata.name.clone().unwrap_or_default(),
        namespace: r.metadata.namespace.clone(),
        rules: r.rules.iter().flatten().map(Rule::from).collect(),
        aggregation: vec![],
        aggregated_from: vec![],
        labels: labels(&r.metadata),
        created: created(&r.metadata),
    }
}

/// ClusterRoles as views, each aggregated one holding the rules it selects.
pub fn cluster_roles(roles: &[ClusterRole]) -> Vec<RoleView> {
    roles
        .iter()
        .zip(aggregate::resolve(roles))
        .map(|(r, resolved)| RoleView {
            kind: RoleKind::ClusterRole,
            name: r.metadata.name.clone().unwrap_or_default(),
            namespace: None,
            rules: resolved.rules,
            aggregation: r
                .aggregation_rule
                .iter()
                .flat_map(|a| a.cluster_role_selectors.iter().flatten())
                .map(aggregate::describe)
                .collect(),
            aggregated_from: resolved.aggregated_from,
            labels: labels(&r.metadata),
            created: created(&r.metadata),
        })
        .collect()
}

fn subject(s: &K8sSubject, binding_namespace: Option<&str>) -> Option<Subject> {
    let kind = match s.kind.as_str() {
        "User" => SubjectKind::User,
        "Group" => SubjectKind::Group,
        "ServiceAccount" => SubjectKind::ServiceAccount,
        _ => return None,
    };
    let namespace = match kind {
        // A ServiceAccount without one is in the binding's, as the authorizer reads it.
        SubjectKind::ServiceAccount => s
            .namespace
            .clone()
            .filter(|n| !n.is_empty())
            .or_else(|| binding_namespace.map(str::to_owned)),
        _ => None,
    };
    Some(Subject {
        kind,
        name: s.name.clone(),
        namespace,
    })
}

fn binding(
    kind: BindingKind,
    meta: &k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    role_ref: &RoleRef,
    subjects: Option<&Vec<K8sSubject>>,
) -> BindingView {
    let namespace = meta.namespace.clone();
    BindingView {
        kind,
        name: meta.name.clone().unwrap_or_default(),
        role_kind: if role_ref.kind == "Role" {
            RoleKind::Role
        } else {
            RoleKind::ClusterRole
        },
        role_name: role_ref.name.clone(),
        role_found: false,
        subjects: subjects
            .into_iter()
            .flatten()
            .filter_map(|s| subject(s, namespace.as_deref()))
            .collect(),
        namespace,
        created: created(meta),
    }
}

pub fn role_binding(b: &RoleBinding) -> BindingView {
    binding(
        BindingKind::RoleBinding,
        &b.metadata,
        &b.role_ref,
        b.subjects.as_ref(),
    )
}

pub fn cluster_role_binding(b: &ClusterRoleBinding) -> BindingView {
    binding(
        BindingKind::ClusterRoleBinding,
        &b.metadata,
        &b.role_ref,
        b.subjects.as_ref(),
    )
}

/// The roles and bindings read, which bindings and subjects refer to.
#[derive(Debug, Clone, Default)]
pub struct Rbac {
    pub roles: Vec<RoleView>,
    pub bindings: Vec<BindingView>,
}

/// A subject named by at least one binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SubjectEntry {
    pub subject: Subject,
    /// How many bindings name it directly.
    pub bindings: u32,
}

/// A binding that applies to a subject, and how.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AppliedBinding {
    pub binding: BindingView,
    /// The group it names, when it doesn't name the subject itself.
    pub through_group: Option<String>,
}

/// What a set of bindings grants in one Namespace, or cluster-wide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PermissionScope {
    /// `None` cluster-wide: in every Namespace, on cluster-scoped resources
    /// and non-resource URLs.
    pub namespace: Option<String>,
    pub grants: Grants,
}

impl Rbac {
    /// Sorts roles and bindings (ClusterRoles and ClusterRoleBindings
    /// first, then by Namespace and name) and marks bindings whose role is
    /// among the roles.
    pub fn new(mut roles: Vec<RoleView>, mut bindings: Vec<BindingView>) -> Self {
        roles.sort_by(|a, b| {
            b.kind
                .cmp(&a.kind)
                .then_with(|| (&a.namespace, &a.name).cmp(&(&b.namespace, &b.name)))
        });
        bindings.sort_by(|a, b| {
            b.kind
                .cmp(&a.kind)
                .then_with(|| (&a.namespace, &a.name).cmp(&(&b.namespace, &b.name)))
        });
        let known: BTreeSet<(RoleKind, Option<&str>, &str)> = roles
            .iter()
            .map(|r| (r.kind, r.namespace.as_deref(), r.name.as_str()))
            .collect();
        for b in &mut bindings {
            b.role_found = known.contains(&(b.role_kind, b.role_namespace(), b.role_name.as_str()));
        }
        Self { roles, bindings }
    }

    /// The role `binding` binds, if it was read.
    pub fn role_of(&self, binding: &BindingView) -> Option<&RoleView> {
        self.roles.iter().find(|r| {
            r.kind == binding.role_kind
                && r.name == binding.role_name
                && r.namespace.as_deref() == binding.role_namespace()
        })
    }

    /// Every subject the bindings name, with how many name it.
    pub fn subjects(&self) -> Vec<SubjectEntry> {
        let mut counts: BTreeMap<&Subject, u32> = BTreeMap::new();
        for b in &self.bindings {
            let named: BTreeSet<&Subject> = b.subjects.iter().collect();
            for s in named {
                *counts.entry(s).or_default() += 1;
            }
        }
        counts
            .into_iter()
            .map(|(subject, bindings)| SubjectEntry {
                subject: subject.clone(),
                bindings,
            })
            .collect()
    }

    /// The bindings that apply to `subject`, directly or through its
    /// implied groups.
    pub fn applied(&self, subject: &Subject) -> Vec<AppliedBinding> {
        let groups = subject.implied_groups();
        self.bindings
            .iter()
            .filter_map(|b| {
                b.names(subject, &groups).map(|through| AppliedBinding {
                    binding: b.clone(),
                    through_group: match through {
                        Through::Direct => None,
                        Through::Group(g) => Some(g),
                    },
                })
            })
            .collect()
    }

    /// What `applied` bindings grant, cluster-wide first, then by
    /// Namespace: a ClusterRoleBinding everywhere, a RoleBinding only in
    /// its Namespace, even when it binds a ClusterRole. Each grant names
    /// its bindings by their index in `applied`.
    pub fn permissions(&self, applied: &[AppliedBinding]) -> Vec<PermissionScope> {
        let mut by_scope: BTreeMap<Option<&str>, Vec<(&Rule, Option<u32>)>> = BTreeMap::new();
        for (i, a) in applied.iter().enumerate() {
            let Some(role) = self.role_of(&a.binding) else {
                continue;
            };
            let scope = match a.binding.kind {
                BindingKind::ClusterRoleBinding => None,
                BindingKind::RoleBinding => a.binding.namespace.as_deref(),
            };
            let rules = by_scope.entry(scope).or_default();
            rules.extend(role.rules.iter().map(|r| (r, Some(i as u32))));
        }
        by_scope
            .into_iter()
            .map(|(namespace, rules)| PermissionScope {
                namespace: namespace.map(str::to_owned),
                // Non-resource URLs are only granted cluster-wide.
                grants: merge(rules, namespace.is_none()),
            })
            .filter(|s| !s.grants.resources.is_empty() || !s.grants.urls.is_empty())
            .collect()
    }
}

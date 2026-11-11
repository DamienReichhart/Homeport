//! What Homeport's own credentials may do, as the API server answers it:
//! `SelfSubjectRulesReview` for a Namespace's rules, `SelfSubjectAccessReview`
//! for one request (`kubectl auth can-i`), and `SelfSubjectReview` for who
//! they sign in as.

use std::time::Duration;

use futures_util::{stream::FuturesUnordered, StreamExt};
use k8s_openapi::api::{
    authentication::v1::SelfSubjectReview,
    authorization::v1::{
        ResourceAttributes, SelfSubjectAccessReview, SelfSubjectAccessReviewSpec,
        SelfSubjectRulesReview, SelfSubjectRulesReviewSpec,
    },
};
use kube::api::{Api, PostParams};
use serde::{Deserialize, Serialize};
use tokio::time::timeout;

use homeport_core::error::{AppError, AppResult};

use super::rules::{merge, Grants, Rule};
use crate::{
    api::discovery::{types_of, ApiResourceInfo},
    messages as msg,
    resources::{ApiType, On},
};

/// Who the credentials sign in as.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UserView {
    pub username: String,
    pub groups: Vec<String>,
}

/// What the credentials may do in a Namespace, as the API server says.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct OwnPermissions {
    /// The Namespace asked about: the page's, or the Cluster's default one.
    pub namespace: String,
    /// `None` when the Cluster can't say (before Kubernetes 1.28).
    pub user: Option<UserView>,
    /// Its rules there, cluster-wide ones included, merged.
    pub grants: Grants,
    /// The API server's own caveat: an authorizer (a webhook, ABAC) that
    /// can't list rules may allow more than this.
    pub incomplete: bool,
    pub evaluation_error: Option<String>,
}

/// "Can I …?": a verb on a resource, as `kubectl auth can-i` asks it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AccessCheck {
    pub verb: String,
    /// `secrets`, `deploy`, `deployments.apps`, or `*`.
    pub resource: String,
    /// Its API group, when `resource` doesn't name it.
    pub group: Option<String>,
    /// `log`, `exec`, `scale`…
    pub subresource: Option<String>,
    /// One object, by name.
    pub name: Option<String>,
    /// Every Namespace (or none, for a cluster-wide resource) with `None`.
    pub namespace: Option<String>,
}

/// The API server's answer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AccessAnswer {
    pub allowed: bool,
    /// Explicitly denied, so no other authorizer was asked.
    pub denied: bool,
    pub reason: Option<String>,
    pub evaluation_error: Option<String>,
    /// The resource checked, as kubectl names it (`deployments.apps`).
    pub resource: String,
    /// `false` when the Cluster serves no type of that name: it was asked as typed.
    pub known: bool,
    /// The same question for a terminal.
    pub command: String,
}

fn authorization(kind: &str, plural: &str) -> ApiType {
    ApiType {
        group: "authorization.k8s.io".into(),
        version: "v1".into(),
        kind: kind.into(),
        plural: plural.into(),
        namespaced: false,
    }
}

fn some(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
}

/// `resource` and `group` as typed: `deployments.apps` names its group.
pub fn split_resource(resource: &str, group: Option<&str>) -> (String, Option<String>) {
    let resource = resource.trim();
    match group {
        Some(g) => (resource.to_owned(), Some(g.to_owned())),
        None => match resource.split_once('.') {
            Some((r, g)) if !r.is_empty() && !g.is_empty() => (r.to_owned(), Some(g.to_owned())),
            _ => (resource.to_owned(), None),
        },
    }
}

/// The type `name` stands for, as kubectl resolves it: its plural, a short
/// name or its kind, in `group` when given; the core group comes first.
pub fn find_type<'a>(
    types: &'a [ApiResourceInfo],
    name: &str,
    group: Option<&str>,
) -> Option<&'a ApiResourceInfo> {
    let name = name.to_lowercase();
    types.iter().find(|t| {
        group.is_none_or(|g| t.api.group == g)
            && (t.api.plural == name
                || t.short_names.contains(&name)
                || t.api.kind.to_lowercase() == name)
    })
}

/// The `kubectl auth can-i` command asking the same.
pub fn command(verb: &str, resource: &str, check: &AccessCheck) -> String {
    let mut line = format!("kubectl auth can-i {verb} {resource}");
    if let Some(name) = some(&check.name) {
        line.push('/');
        line.push_str(&name);
    }
    if let Some(sub) = some(&check.subresource) {
        line.push_str(&format!(" --subresource {sub}"));
    }
    match some(&check.namespace) {
        Some(ns) => line.push_str(&format!(" --namespace {ns}")),
        None => line.push_str(" --all-namespaces"),
    }
    line
}

/// What a typed resource name stands for.
enum Resolution {
    Found(ApiType),
    /// No type the Cluster serves has that name.
    Unknown,
    /// Some groups didn't answer, so it may be theirs.
    CantSay,
}

/// How long one discovery request may take: a down aggregated API (a
/// missing metrics-server) answers 503, which the client keeps retrying.
const DISCOVERY_WAIT: Duration = Duration::from_secs(5);

impl On {
    /// Who the credentials sign in as, when the Cluster says.
    async fn whoami(&self) -> Option<UserView> {
        let api: Api<SelfSubjectReview> = Api::all(self.client.clone());
        match api
            .create(&PostParams::default(), &SelfSubjectReview::default())
            .await
        {
            Ok(review) => {
                let info = review.status?.user_info?;
                Some(UserView {
                    username: info.username.unwrap_or_default(),
                    groups: info.groups.unwrap_or_default(),
                })
            }
            Err(e) => {
                log::debug!("couldn't ask who the credentials are: {e}");
                None
            }
        }
    }

    /// The type `name` (in `group`, when given) stands for: looked up in the
    /// core group first, then in every group (or `group`) at its preferred
    /// version, each given a few seconds.
    async fn resolve_type(&self, name: &str, group: Option<&str>) -> Resolution {
        let mut complete = true;
        if group.is_none_or(str::is_empty) {
            match timeout(DISCOVERY_WAIT, self.client.list_core_api_resources("v1")).await {
                Ok(Ok(core)) => {
                    if let Some(t) = find_type(&types_of(&core), name, Some("")) {
                        return Resolution::Found(t.api.clone());
                    }
                }
                _ => complete = false,
            }
            if group.is_some() {
                return if complete {
                    Resolution::Unknown
                } else {
                    Resolution::CantSay
                };
            }
        }
        let Ok(Ok(groups)) = timeout(DISCOVERY_WAIT, self.client.list_api_groups()).await else {
            return Resolution::CantSay;
        };
        let versions: Vec<String> = groups
            .groups
            .iter()
            .filter(|g| group.is_none_or(|wanted| g.name == wanted))
            .filter_map(|g| g.preferred_version.as_ref().or(g.versions.first()))
            .map(|v| v.group_version.clone())
            .collect();
        let mut pending: FuturesUnordered<_> = versions
            .iter()
            .enumerate()
            .map(|(i, gv)| async move {
                let list = timeout(DISCOVERY_WAIT, self.client.list_api_group_resources(gv)).await;
                (i, list.ok().and_then(Result::ok))
            })
            .collect();
        // Each group's answer, in the order the Cluster serves them: the
        // first group with that type wins, as kubectl's does.
        let mut answers: Vec<Option<Option<Vec<ApiResourceInfo>>>> = vec![None; versions.len()];
        while let Some((i, list)) = pending.next().await {
            answers[i] = Some(list.map(|l| types_of(&l)));
            for answer in &answers {
                match answer {
                    None => break,
                    Some(None) => complete = false,
                    Some(Some(types)) => {
                        if let Some(t) = find_type(types, name, group) {
                            return Resolution::Found(t.api.clone());
                        }
                    }
                }
            }
        }
        if complete {
            Resolution::Unknown
        } else {
            Resolution::CantSay
        }
    }

    /// What the credentials may do in `namespace` (the default one with
    /// `None`), as `kubectl auth can-i --list` shows it.
    pub async fn own_permissions(&self, namespace: Option<&str>) -> AppResult<OwnPermissions> {
        let namespace = namespace
            .unwrap_or_else(|| self.client.default_namespace())
            .to_owned();
        let review = SelfSubjectRulesReview {
            spec: SelfSubjectRulesReviewSpec {
                namespace: Some(namespace.clone()),
            },
            ..Default::default()
        };
        let api: Api<SelfSubjectRulesReview> = Api::all(self.client.clone());
        let params = PostParams::default();
        let (answer, user) = futures_util::join!(api.create(&params, &review), self.whoami());
        let ty = authorization("SelfSubjectRulesReview", "selfsubjectrulesreviews");
        let status = answer
            .map_err(|e| self.failed_on("create", &ty, None, None, &e))?
            .status
            .unwrap_or_default();
        let mut rules: Vec<Rule> = status.resource_rules.iter().map(Rule::from).collect();
        rules.extend(status.non_resource_rules.iter().map(Rule::from));
        Ok(OwnPermissions {
            namespace,
            user,
            grants: merge(rules.iter().map(|r| (r, None)), true),
            incomplete: status.incomplete,
            evaluation_error: status.evaluation_error.filter(|e| !e.is_empty()),
        })
    }

    /// Whether the credentials may do what `check` asks, as the API server
    /// answers `kubectl auth can-i`.
    pub async fn can_i(&self, check: &AccessCheck) -> AppResult<AccessAnswer> {
        let verb = check.verb.trim();
        let (resource, group) = split_resource(&check.resource, some(&check.group).as_deref());
        if verb.is_empty() || resource.is_empty() {
            return Err(AppError::validation(msg::RBAC_CHECK_INCOMPLETE));
        }
        // kubectl resolves short names and kinds to a type; so does this.
        let (resource, group, known) = if resource == "*" {
            (resource, group.unwrap_or_default(), true)
        } else {
            match self.resolve_type(&resource, group.as_deref()).await {
                Resolution::Found(api) => (api.plural, api.group, true),
                Resolution::Unknown => (resource, group.unwrap_or_default(), false),
                Resolution::CantSay => (resource, group.unwrap_or_default(), true),
            }
        };
        let qualified = if group.is_empty() {
            resource.clone()
        } else {
            format!("{resource}.{group}")
        };
        let review = SelfSubjectAccessReview {
            spec: SelfSubjectAccessReviewSpec {
                resource_attributes: Some(ResourceAttributes {
                    verb: Some(verb.to_owned()),
                    group: Some(group),
                    resource: Some(resource),
                    subresource: some(&check.subresource),
                    name: some(&check.name),
                    namespace: Some(some(&check.namespace).unwrap_or_default()),
                    ..Default::default()
                }),
                non_resource_attributes: None,
            },
            ..Default::default()
        };
        let api: Api<SelfSubjectAccessReview> = Api::all(self.client.clone());
        let ty = authorization("SelfSubjectAccessReview", "selfsubjectaccessreviews");
        let status = api
            .create(&PostParams::default(), &review)
            .await
            .map_err(|e| self.failed_on("create", &ty, None, None, &e))?
            .status
            .unwrap_or_default();
        Ok(AccessAnswer {
            allowed: status.allowed,
            denied: status.denied == Some(true),
            reason: status.reason.filter(|r| !r.is_empty()),
            evaluation_error: status.evaluation_error.filter(|e| !e.is_empty()),
            command: command(verb, &qualified, check),
            resource: qualified,
            known,
        })
    }
}

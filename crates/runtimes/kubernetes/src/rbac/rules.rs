//! RBAC rules: what one allows, and what several grant together, merged
//! into one row per resource as the permission matrix shows them.

use std::collections::{BTreeMap, BTreeSet};

use k8s_openapi::api::{
    authorization::v1::{NonResourceRule, ResourceRule},
    rbac::v1::PolicyRule,
};
use serde::Serialize;

/// One rule of a Role or ClusterRole, as RBAC writes it: `*` stands for
/// every verb, API group, resource or URL.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub verbs: Vec<String>,
    /// `""` for the core group.
    pub api_groups: Vec<String>,
    /// `pods`, `pods/log` (a subresource), `*/scale`.
    pub resources: Vec<String>,
    /// Only these objects, by name, when not empty.
    pub resource_names: Vec<String>,
    /// Paths that aren't resources (`/healthz`, `/api/*`).
    pub non_resource_urls: Vec<String>,
}

impl From<&PolicyRule> for Rule {
    fn from(r: &PolicyRule) -> Self {
        Self {
            verbs: r.verbs.clone(),
            api_groups: r.api_groups.clone().unwrap_or_default(),
            resources: r.resources.clone().unwrap_or_default(),
            resource_names: r.resource_names.clone().unwrap_or_default(),
            non_resource_urls: r.non_resource_urls.clone().unwrap_or_default(),
        }
    }
}

impl From<&ResourceRule> for Rule {
    fn from(r: &ResourceRule) -> Self {
        Self {
            verbs: r.verbs.clone(),
            api_groups: r.api_groups.clone().unwrap_or_default(),
            resources: r.resources.clone().unwrap_or_default(),
            resource_names: r.resource_names.clone().unwrap_or_default(),
            non_resource_urls: vec![],
        }
    }
}

impl From<&NonResourceRule> for Rule {
    fn from(r: &NonResourceRule) -> Self {
        Self {
            verbs: r.verbs.clone(),
            non_resource_urls: r.non_resource_urls.clone().unwrap_or_default(),
            ..Self::default()
        }
    }
}

/// A request on a resource, as RBAC sees it: what `kubectl auth can-i` asks.
#[derive(Debug, Clone, Default)]
pub struct Request<'a> {
    pub verb: &'a str,
    pub group: &'a str,
    pub resource: &'a str,
    pub subresource: Option<&'a str>,
    pub name: Option<&'a str>,
}

const ALL: &str = "*";

fn has(list: &[String], value: &str) -> bool {
    list.iter().any(|v| v == ALL || v == value)
}

/// Whether `rule` allows `request`, the way Kubernetes' RBAC authorizer
/// matches it: `*` for any verb, group or resource, `*/scale` for that
/// subresource of any resource, and resource names restricting it to them.
pub fn allows(rule: &Rule, request: &Request) -> bool {
    has(&rule.verbs, request.verb)
        && has(&rule.api_groups, request.group)
        && resource_matches(&rule.resources, request.resource, request.subresource)
        && (rule.resource_names.is_empty()
            || request
                .name
                .is_some_and(|n| rule.resource_names.iter().any(|r| r == n)))
}

fn resource_matches(resources: &[String], resource: &str, subresource: Option<&str>) -> bool {
    let combined = match subresource {
        Some(sub) => format!("{resource}/{sub}"),
        None => resource.to_owned(),
    };
    resources.iter().any(|r| {
        r == ALL
            || *r == combined
            || subresource.is_some_and(|sub| r.strip_prefix("*/") == Some(sub))
    })
}

/// Whether `rule` allows `verb` on the non-resource URL `path`; a URL
/// ending in `*` stands for every path it starts.
pub fn allows_url(rule: &Rule, verb: &str, path: &str) -> bool {
    has(&rule.verbs, verb) && rule.non_resource_urls.iter().any(|u| url_covers(u, path))
}

fn url_covers(rule: &str, path: &str) -> bool {
    rule == ALL
        || rule == path
        || rule
            .strip_suffix('*')
            .is_some_and(|prefix| path.starts_with(prefix))
}

/// What a set of rules grants on one resource (or a wildcard of them).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ResourceGrant {
    pub api_group: String,
    /// `pods`, `pods/log`, `*` or `*/scale`.
    pub resource: String,
    /// Only on these objects, when not empty.
    pub resource_names: Vec<String>,
    /// In the order the matrix shows them; `["*"]` for every verb.
    pub verbs: Vec<String>,
    /// The bindings granting it, by their index in the list given with it.
    pub bindings: Vec<u32>,
}

/// What a set of rules grants on one non-resource URL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UrlGrant {
    pub url: String,
    pub verbs: Vec<String>,
    pub bindings: Vec<u32>,
}

/// What a set of rules grants, one row per resource and per URL.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Grants {
    pub resources: Vec<ResourceGrant>,
    pub urls: Vec<UrlGrant>,
}

/// The verbs Kubernetes' own resources take, in the order kubectl lists
/// them; any other verb (`bind`, `escalate`, `impersonate`…) comes after.
pub const VERBS: &[&str] = &[
    "get",
    "list",
    "watch",
    "create",
    "update",
    "patch",
    "delete",
    "deletecollection",
];

fn verb_order(verb: &str) -> (usize, &str) {
    let rank = match verb {
        ALL => 0,
        v => VERBS
            .iter()
            .position(|k| *k == v)
            .map_or(VERBS.len() + 1, |i| i + 1),
    };
    (rank, verb)
}

/// Verbs in the matrix's order; every verb when one is `*`.
fn ordered(verbs: &BTreeSet<String>) -> Vec<String> {
    if verbs.contains(ALL) {
        return vec![ALL.into()];
    }
    let mut list: Vec<String> = verbs.iter().cloned().collect();
    list.sort_by(|a, b| verb_order(a).cmp(&verb_order(b)));
    list
}

#[derive(Default)]
struct Row {
    verbs: BTreeSet<String>,
    bindings: BTreeSet<u32>,
}

type ResourceKey = (String, String, Vec<String>);

/// Merges `rules`, each with the binding granting it (if any), into one row
/// per API group, resource and set of names, then leaves out of each row
/// the verbs a broader row (`*`, `*/scale`, no names) already grants.
/// `with_urls` keeps non-resource URLs: only ClusterRoleBindings grant them.
pub fn merge<'a>(
    rules: impl IntoIterator<Item = (&'a Rule, Option<u32>)>,
    with_urls: bool,
) -> Grants {
    let mut resources: BTreeMap<ResourceKey, Row> = BTreeMap::new();
    let mut urls: BTreeMap<String, Row> = BTreeMap::new();
    for (rule, binding) in rules {
        let add = |row: &mut Row| {
            row.verbs.extend(rule.verbs.iter().cloned());
            row.bindings.extend(binding);
        };
        if !rule.resources.is_empty() {
            let mut names = rule.resource_names.clone();
            names.sort();
            names.dedup();
            for group in &rule.api_groups {
                for resource in &rule.resources {
                    add(resources
                        .entry((group.clone(), resource.clone(), names.clone()))
                        .or_default());
                }
            }
        }
        if with_urls {
            for url in &rule.non_resource_urls {
                add(urls.entry(url.clone()).or_default());
            }
        }
    }
    let resources = resources
        .iter()
        .filter_map(|(key, row)| {
            let broader = resources
                .iter()
                .filter(|(other, _)| *other != key && key_covers(other, key))
                .map(|(_, r)| &r.verbs);
            let (group, resource, names) = key;
            Some(ResourceGrant {
                api_group: group.clone(),
                resource: resource.clone(),
                resource_names: names.clone(),
                verbs: uncovered(row, broader)?,
                bindings: row.bindings.iter().copied().collect(),
            })
        })
        .collect();
    let urls = urls
        .iter()
        .filter_map(|(url, row)| {
            let broader = urls
                .iter()
                .filter(|(other, _)| *other != url && url_covers(other, url))
                .map(|(_, r)| &r.verbs);
            Some(UrlGrant {
                url: url.clone(),
                verbs: uncovered(row, broader)?,
                bindings: row.bindings.iter().copied().collect(),
            })
        })
        .collect();
    Grants { resources, urls }
}

/// Whether the row `broader` stands for every request `narrower` does.
fn key_covers((group, resource, names): &ResourceKey, narrower: &ResourceKey) -> bool {
    (group == ALL || *group == narrower.0)
        && resource_covers(resource, &narrower.1)
        && (names.is_empty()
            || (!narrower.2.is_empty() && narrower.2.iter().all(|n| names.contains(n))))
}

/// Whether a rule's `broader` resource covers every request `narrower` does.
fn resource_covers(broader: &str, narrower: &str) -> bool {
    broader == ALL
        || broader == narrower
        || broader
            .strip_prefix("*/")
            .is_some_and(|sub| narrower.split_once('/').is_some_and(|(_, s)| s == sub))
}

/// The verbs of `row` no broader row grants, or `None` when they all are.
fn uncovered<'a>(
    row: &Row,
    broader: impl Iterator<Item = &'a BTreeSet<String>>,
) -> Option<Vec<String>> {
    let mut verbs = row.verbs.clone();
    for other in broader {
        if other.contains(ALL) {
            return None;
        }
        verbs.retain(|v| v == ALL || !other.contains(v));
    }
    (!verbs.is_empty()).then(|| ordered(&verbs))
}

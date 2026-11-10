//! ClusterRole aggregation: a ClusterRole with an `aggregationRule` holds
//! the rules of every ClusterRole its label selectors match, as Kubernetes'
//! aggregation controller fills them in (`admin`, `edit` and `view` are
//! built this way).

use std::collections::{BTreeMap, BTreeSet, HashSet};

use k8s_openapi::{api::rbac::v1::ClusterRole, apimachinery::pkg::apis::meta::v1::LabelSelector};

use super::rules::Rule;

/// Whether `labels` match `selector`; an empty selector matches everything,
/// as Kubernetes reads it.
pub(super) fn matches(selector: &LabelSelector, labels: &BTreeMap<String, String>) -> bool {
    let by_label = selector
        .match_labels
        .iter()
        .flatten()
        .all(|(k, v)| labels.get(k) == Some(v));
    by_label
        && selector.match_expressions.iter().flatten().all(|e| {
            let values = e.values.as_deref().unwrap_or_default();
            let value = labels.get(&e.key);
            match e.operator.as_str() {
                "In" => value.is_some_and(|v| values.contains(v)),
                "NotIn" => value.is_none_or(|v| !values.contains(v)),
                "Exists" => value.is_some(),
                "DoesNotExist" => value.is_none(),
                _ => false,
            }
        })
}

/// A selector as kubectl writes one: `a=b,c in (d,e),!f`.
pub(super) fn describe(selector: &LabelSelector) -> String {
    let labels = selector
        .match_labels
        .iter()
        .flatten()
        .map(|(k, v)| format!("{k}={v}"));
    let expressions = selector.match_expressions.iter().flatten().map(|e| {
        let values = e.values.as_deref().unwrap_or_default().join(",");
        match e.operator.as_str() {
            "In" => format!("{} in ({values})", e.key),
            "NotIn" => format!("{} notin ({values})", e.key),
            "Exists" => e.key.clone(),
            "DoesNotExist" => format!("!{}", e.key),
            op => format!("{} {op} ({values})", e.key),
        }
    });
    labels.chain(expressions).collect::<Vec<_>>().join(",")
}

/// A ClusterRole's rules with aggregation resolved, and the ClusterRoles
/// its `aggregationRule` selected.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct Resolved {
    pub(super) rules: Vec<Rule>,
    pub(super) aggregated_from: Vec<String>,
}

/// Each of `roles`' rules: those it holds and, for an aggregated one, those
/// of every ClusterRole its selectors match, themselves resolved first (an
/// aggregated role can aggregate another), without repeating a rule.
pub(super) fn resolve(roles: &[ClusterRole]) -> Vec<Resolved> {
    let mut done: Vec<Option<Resolved>> = vec![None; roles.len()];
    for i in 0..roles.len() {
        resolve_one(roles, i, &mut done, &mut BTreeSet::new());
    }
    done.into_iter().map(Option::unwrap_or_default).collect()
}

fn resolve_one(
    roles: &[ClusterRole],
    i: usize,
    done: &mut [Option<Resolved>],
    visiting: &mut BTreeSet<usize>,
) -> Resolved {
    if let Some(resolved) = &done[i] {
        return resolved.clone();
    }
    let role = &roles[i];
    let mut rules: Vec<Rule> = role.rules.iter().flatten().map(Rule::from).collect();
    let mut from = vec![];
    let selectors = role
        .aggregation_rule
        .as_ref()
        .and_then(|a| a.cluster_role_selectors.as_deref());
    // A cycle of aggregations (a selects b selects a) stops at the role it started from.
    if let Some(selectors) = selectors.filter(|_| visiting.insert(i)) {
        for (j, other) in roles.iter().enumerate() {
            let labels = other.metadata.labels.clone().unwrap_or_default();
            if j == i || !selectors.iter().any(|s| matches(s, &labels)) {
                continue;
            }
            from.push(other.metadata.name.clone().unwrap_or_default());
            if visiting.contains(&j) {
                rules.extend(other.rules.iter().flatten().map(Rule::from));
            } else {
                rules.extend(resolve_one(roles, j, done, visiting).rules);
            }
        }
        visiting.remove(&i);
    }
    let mut seen = HashSet::new();
    rules.retain(|r| seen.insert(r.clone()));
    from.sort();
    let resolved = Resolved {
        rules,
        aggregated_from: from,
    };
    if visiting.is_empty() {
        done[i] = Some(resolved.clone());
    }
    resolved
}

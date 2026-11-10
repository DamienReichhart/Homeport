use std::collections::BTreeMap;

use k8s_openapi::{
    api::rbac::v1::{AggregationRule, ClusterRole, PolicyRule, RoleBinding, RoleRef, Subject as S},
    apimachinery::pkg::apis::meta::v1::{LabelSelector, LabelSelectorRequirement, ObjectMeta},
};

use super::{
    aggregate,
    index::{self, Rbac, RoleKind, Subject, SubjectKind},
    review::{command, split_resource, AccessCheck},
    rules::{allows, allows_url, merge, Request, Rule},
};

fn list(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn rule(groups: &[&str], resources: &[&str], verbs: &[&str]) -> Rule {
    Rule {
        verbs: list(verbs),
        api_groups: list(groups),
        resources: list(resources),
        ..Rule::default()
    }
}

fn request<'a>(verb: &'a str, group: &'a str, resource: &'a str) -> Request<'a> {
    Request {
        verb,
        group,
        resource,
        ..Request::default()
    }
}

#[test]
fn a_rule_allows_what_it_names_and_its_wildcards() {
    let pods = rule(&[""], &["pods"], &["get", "list"]);
    assert!(allows(&pods, &request("get", "", "pods")));
    assert!(!allows(&pods, &request("delete", "", "pods")));
    assert!(!allows(&pods, &request("get", "apps", "pods")));
    // A subresource is a resource of its own.
    let logs = Request {
        subresource: Some("log"),
        ..request("get", "", "pods")
    };
    assert!(!allows(&pods, &logs));
    assert!(allows(&rule(&[""], &["pods/log"], &["get"]), &logs));

    let everything = rule(&["*"], &["*"], &["*"]);
    assert!(allows(
        &everything,
        &request("escalate", "rbac.authorization.k8s.io", "roles")
    ));
    assert!(allows(&everything, &logs));
    assert!(allows(
        &rule(&["apps"], &["*"], &["get"]),
        &request("get", "apps", "deployments")
    ));

    let scale = rule(&["apps"], &["*/scale"], &["update"]);
    let scaling = Request {
        subresource: Some("scale"),
        ..request("update", "apps", "deployments")
    };
    assert!(allows(&scale, &scaling));
    assert!(!allows(&scale, &request("update", "apps", "deployments")));
}

#[test]
fn resource_names_restrict_a_rule_to_those_objects() {
    let one = Rule {
        resource_names: list(&["app-config"]),
        ..rule(&[""], &["configmaps"], &["get"])
    };
    let named = |name| Request {
        name: Some(name),
        ..request("get", "", "configmaps")
    };
    assert!(allows(&one, &named("app-config")));
    assert!(!allows(&one, &named("other")));
    // Listing them all isn't asking for one by name.
    assert!(!allows(&one, &request("get", "", "configmaps")));
}

#[test]
fn non_resource_urls_match_exactly_or_by_a_trailing_star() {
    let urls = Rule {
        verbs: list(&["get"]),
        non_resource_urls: list(&["/healthz", "/api/*"]),
        ..Rule::default()
    };
    assert!(allows_url(&urls, "get", "/healthz"));
    assert!(!allows_url(&urls, "get", "/healthz/ready"));
    assert!(allows_url(&urls, "get", "/api/v1"));
    assert!(!allows_url(&urls, "post", "/api/v1"));
    // Non-resource rules allow no resource.
    assert!(!allows(&urls, &request("get", "", "pods")));
}

#[test]
fn rules_merge_into_one_row_per_resource_with_their_verbs() {
    let a = rule(&["", "apps"], &["pods", "deployments"], &["list", "get"]);
    let b = rule(&[""], &["pods"], &["delete", "watch", "get"]);
    let grants = merge([(&a, Some(0)), (&b, Some(1))], true);
    let rows: Vec<(String, String, Vec<String>, Vec<u32>)> = grants
        .resources
        .iter()
        .map(|g| {
            (
                g.api_group.clone(),
                g.resource.clone(),
                g.verbs.clone(),
                g.bindings.clone(),
            )
        })
        .collect();
    assert_eq!(
        rows,
        [
            (
                "".into(),
                "deployments".into(),
                list(&["get", "list"]),
                vec![0]
            ),
            (
                "".into(),
                "pods".into(),
                list(&["get", "list", "watch", "delete"]),
                vec![0, 1]
            ),
            (
                "apps".into(),
                "deployments".into(),
                list(&["get", "list"]),
                vec![0]
            ),
            (
                "apps".into(),
                "pods".into(),
                list(&["get", "list"]),
                vec![0]
            ),
        ]
    );
}

#[test]
fn a_wildcard_row_absorbs_what_it_covers() {
    let admin = rule(&["*"], &["*"], &["*"]);
    let pods = rule(&[""], &["pods"], &["get"]);
    let grants = merge([(&admin, Some(0)), (&pods, Some(1))], true);
    assert_eq!(grants.resources.len(), 1);
    assert_eq!(grants.resources[0].resource, "*");
    assert_eq!(grants.resources[0].verbs, ["*"]);

    // A verb list with `*` is every verb.
    let star = rule(&[""], &["secrets"], &["get", "*"]);
    assert_eq!(merge([(&star, None)], true).resources[0].verbs, ["*"]);

    // A broader row only absorbs the verbs it grants.
    let read_all = rule(&["apps"], &["*"], &["get", "list"]);
    let deploy = rule(&["apps"], &["deployments"], &["get", "delete"]);
    let scale = rule(&["apps"], &["*/scale"], &["update"]);
    let scaling = rule(&["apps"], &["deployments/scale"], &["update", "patch"]);
    let grants = merge(
        [
            (&read_all, None),
            (&deploy, None),
            (&scale, None),
            (&scaling, None),
        ],
        true,
    );
    let rows: BTreeMap<&str, &[String]> = grants
        .resources
        .iter()
        .map(|g| (g.resource.as_str(), g.verbs.as_slice()))
        .collect();
    assert_eq!(rows["deployments"], list(&["delete"]));
    assert_eq!(rows["deployments/scale"], list(&["patch"]));
    assert_eq!(rows["*/scale"], list(&["update"]));
}

#[test]
fn a_named_row_is_absorbed_by_one_for_every_name() {
    let one = Rule {
        resource_names: list(&["b", "a"]),
        ..rule(&[""], &["configmaps"], &["get", "update"])
    };
    let all = rule(&[""], &["configmaps"], &["get"]);
    let grants = merge([(&one, None), (&all, None)], true);
    assert_eq!(grants.resources.len(), 2);
    let named = grants
        .resources
        .iter()
        .find(|g| !g.resource_names.is_empty())
        .unwrap();
    assert_eq!(named.resource_names, ["a", "b"]);
    assert_eq!(named.verbs, ["update"]);
}

#[test]
fn urls_merge_and_are_left_out_where_they_cannot_be_granted() {
    let health = Rule {
        verbs: list(&["get"]),
        non_resource_urls: list(&["/healthz", "/livez"]),
        ..Rule::default()
    };
    let all = Rule {
        verbs: list(&["get"]),
        non_resource_urls: list(&["/*"]),
        ..Rule::default()
    };
    let grants = merge([(&health, None), (&all, None)], true);
    assert_eq!(grants.urls.len(), 1);
    assert_eq!(grants.urls[0].url, "/*");
    assert!(merge([(&health, None)], false).urls.is_empty());
}

fn meta(name: &str, labels: &[(&str, &str)]) -> ObjectMeta {
    ObjectMeta {
        name: Some(name.into()),
        labels: Some(
            labels
                .iter()
                .map(|(k, v)| ((*k).into(), (*v).into()))
                .collect(),
        ),
        ..Default::default()
    }
}

fn policy(resources: &[&str], verbs: &[&str]) -> PolicyRule {
    PolicyRule {
        api_groups: Some(list(&[""])),
        resources: Some(list(resources)),
        verbs: list(verbs),
        ..Default::default()
    }
}

fn selecting(label: &str) -> Option<AggregationRule> {
    Some(AggregationRule {
        cluster_role_selectors: Some(vec![LabelSelector {
            match_labels: Some(BTreeMap::from([(label.into(), "true".into())])),
            ..Default::default()
        }]),
    })
}

fn cluster_role(
    name: &str,
    labels: &[(&str, &str)],
    rules: Vec<PolicyRule>,
    aggregation: Option<AggregationRule>,
) -> ClusterRole {
    ClusterRole {
        metadata: meta(name, labels),
        rules: Some(rules),
        aggregation_rule: aggregation,
    }
}

#[test]
fn an_aggregated_cluster_role_holds_the_rules_its_selectors_match() {
    let roles = [
        // Its stored rules are empty, as before the controller fills them in.
        cluster_role(
            "edit",
            &[("to-admin", "true")],
            vec![],
            selecting("to-edit"),
        ),
        cluster_role("admin", &[], vec![], selecting("to-admin")),
        cluster_role(
            "pods",
            &[("to-edit", "true")],
            vec![policy(&["pods"], &["get"])],
            None,
        ),
        cluster_role(
            "secrets",
            &[("to-edit", "true")],
            vec![policy(&["secrets"], &["get"])],
            None,
        ),
        cluster_role(
            "other",
            &[("to-edit", "false")],
            vec![policy(&["nodes"], &["get"])],
            None,
        ),
    ];
    let views = index::cluster_roles(&roles);
    let edit = &views[0];
    assert_eq!(edit.aggregation, ["to-edit=true"]);
    assert_eq!(edit.aggregated_from, ["pods", "secrets"]);
    assert_eq!(edit.rules.len(), 2);
    // Through edit, which it selects.
    let admin = &views[1];
    assert_eq!(admin.aggregated_from, ["edit"]);
    assert_eq!(admin.rules, edit.rules);
    assert!(views[2].aggregation.is_empty() && views[2].aggregated_from.is_empty());
}

#[test]
fn aggregation_survives_a_cycle_and_repeats_no_rule() {
    let roles = [
        cluster_role(
            "a",
            &[("x", "true")],
            vec![policy(&["pods"], &["get"])],
            selecting("y"),
        ),
        cluster_role(
            "b",
            &[("y", "true")],
            vec![policy(&["pods"], &["get"])],
            selecting("x"),
        ),
    ];
    let views = index::cluster_roles(&roles);
    assert_eq!(views[0].rules.len(), 1);
    assert_eq!(views[1].aggregated_from, ["a"]);
}

#[test]
fn label_selectors_match_labels_and_expressions() {
    let labels = BTreeMap::from([("tier".to_owned(), "web".to_owned())]);
    let expr = |operator: &str, values: &[&str]| LabelSelector {
        match_expressions: Some(vec![LabelSelectorRequirement {
            key: "tier".into(),
            operator: operator.into(),
            values: Some(list(values)),
        }]),
        ..Default::default()
    };
    assert!(aggregate::matches(&LabelSelector::default(), &labels));
    assert!(aggregate::matches(&expr("In", &["web", "db"]), &labels));
    assert!(!aggregate::matches(&expr("NotIn", &["web"]), &labels));
    assert!(aggregate::matches(&expr("Exists", &[]), &labels));
    assert!(!aggregate::matches(&expr("DoesNotExist", &[]), &labels));
    assert_eq!(
        aggregate::describe(&expr("In", &["web", "db"])),
        "tier in (web,db)"
    );
}

fn sa(namespace: &str, name: &str) -> Subject {
    Subject {
        kind: SubjectKind::ServiceAccount,
        name: name.into(),
        namespace: Some(namespace.into()),
    }
}

fn binding(
    namespace: Option<&str>,
    name: &str,
    role: (&str, &str),
    subjects: Vec<S>,
) -> index::BindingView {
    let role_ref = RoleRef {
        api_group: super::GROUP.into(),
        kind: role.0.into(),
        name: role.1.into(),
    };
    let metadata = ObjectMeta {
        name: Some(name.into()),
        namespace: namespace.map(str::to_owned),
        ..Default::default()
    };
    match namespace {
        Some(_) => index::role_binding(&RoleBinding {
            metadata,
            role_ref,
            subjects: Some(subjects),
        }),
        None => index::cluster_role_binding(&k8s_openapi::api::rbac::v1::ClusterRoleBinding {
            metadata,
            role_ref,
            subjects: Some(subjects),
        }),
    }
}

fn subject(kind: &str, name: &str, namespace: Option<&str>) -> S {
    S {
        kind: kind.into(),
        name: name.into(),
        namespace: namespace.map(str::to_owned),
        api_group: None,
    }
}

fn fixture() -> Rbac {
    let mut roles = index::cluster_roles(&[
        cluster_role(
            "edit",
            &[],
            vec![policy(&["secrets"], &["get", "delete"])],
            None,
        ),
        cluster_role(
            "basic",
            &[],
            vec![policy(&["selfsubjectreviews"], &["create"])],
            None,
        ),
        cluster_role("view", &[], vec![policy(&["pods"], &["get", "list"])], None),
    ]);
    roles.push(index::role(&k8s_openapi::api::rbac::v1::Role {
        metadata: ObjectMeta {
            namespace: Some("dev".into()),
            ..meta("reader", &[])
        },
        rules: Some(vec![policy(&["configmaps"], &["get"])]),
    }));
    Rbac::new(
        roles,
        vec![
            binding(
                Some("prod"),
                "ci-edit",
                ("ClusterRole", "edit"),
                vec![subject("ServiceAccount", "ci", None)],
            ),
            binding(
                None,
                "basic",
                ("ClusterRole", "basic"),
                vec![subject("Group", "system:authenticated", None)],
            ),
            binding(
                Some("dev"),
                "ci-read",
                ("Role", "reader"),
                vec![subject("User", "system:serviceaccount:ci:ci", None)],
            ),
            binding(
                Some("dev"),
                "all-sa",
                ("ClusterRole", "view"),
                vec![subject("Group", "system:serviceaccounts:prod", None)],
            ),
            binding(
                Some("dev"),
                "gone",
                ("Role", "missing"),
                vec![subject("ServiceAccount", "ci", Some("prod"))],
            ),
            binding(
                None,
                "same-name",
                ("ClusterRole", "edit"),
                vec![subject("User", "admins", None)],
            ),
        ],
    )
}

/// A resource and the verbs granted on it.
type Verbs<'a> = (&'a str, Vec<String>);

#[test]
fn a_service_account_bound_to_edit_in_a_namespace_may_edit_there_only() {
    let rbac = fixture();
    let ci = sa("prod", "ci");
    let applied = rbac.applied(&ci);
    let names: Vec<(&str, Option<&str>)> = applied
        .iter()
        .map(|a| (a.binding.name.as_str(), a.through_group.as_deref()))
        .collect();
    // ClusterRoleBindings first, then RoleBindings by Namespace.
    assert_eq!(
        names,
        [
            ("basic", Some("system:authenticated")),
            ("all-sa", Some("system:serviceaccounts:prod")),
            ("gone", None),
            ("ci-edit", None),
        ]
    );
    let scopes = rbac.permissions(&applied);
    let summary: Vec<(Option<&str>, Vec<Verbs>)> = scopes
        .iter()
        .map(|s| {
            (
                s.namespace.as_deref(),
                s.grants
                    .resources
                    .iter()
                    .map(|g| (g.resource.as_str(), g.verbs.clone()))
                    .collect(),
            )
        })
        .collect();
    assert_eq!(
        summary,
        [
            (None, vec![("selfsubjectreviews", list(&["create"]))]),
            (Some("dev"), vec![("pods", list(&["get", "list"]))]),
            (Some("prod"), vec![("secrets", list(&["get", "delete"]))]),
        ]
    );
    // Grants name their bindings by index in `applied`.
    assert_eq!(scopes[2].grants.resources[0].bindings, [3]);
}

#[test]
fn a_service_account_named_as_its_user_and_a_missing_role_grant_as_expected() {
    let rbac = fixture();
    let other = sa("ci", "ci");
    let applied = rbac.applied(&other);
    assert!(applied
        .iter()
        .any(|a| a.binding.name == "ci-read" && a.through_group.is_none()));
    let gone = rbac.bindings.iter().find(|b| b.name == "gone").unwrap();
    assert!(!gone.role_found);
    assert!(
        rbac.bindings
            .iter()
            .find(|b| b.name == "ci-read")
            .unwrap()
            .role_found
    );
    assert_eq!(gone.subjects[0].namespace.as_deref(), Some("prod"));
    // A ServiceAccount without a Namespace is in its RoleBinding's.
    let edit = rbac.bindings.iter().find(|b| b.name == "ci-edit").unwrap();
    assert_eq!(edit.subjects[0], sa("prod", "ci"));
}

#[test]
fn a_binding_naming_a_subject_and_its_group_names_it_directly() {
    let both = binding(
        Some("prod"),
        "both",
        ("ClusterRole", "edit"),
        vec![
            subject("Group", "system:serviceaccounts", None),
            subject("ServiceAccount", "ci", None),
        ],
    );
    let ci = sa("prod", "ci");
    assert_eq!(
        both.names(&ci, &ci.implied_groups()),
        Some(index::Through::Direct)
    );
    let other = sa("prod", "other");
    assert_eq!(
        both.names(&other, &other.implied_groups()),
        Some(index::Through::Group("system:serviceaccounts".into()))
    );
}

#[test]
fn a_group_is_not_a_user_of_the_same_name() {
    let rbac = fixture();
    let admins = Subject {
        kind: SubjectKind::Group,
        name: "admins".into(),
        namespace: None,
    };
    assert!(rbac.applied(&admins).is_empty());
    assert!(admins.implied_groups().is_empty());
    let user = Subject {
        kind: SubjectKind::User,
        ..admins
    };
    assert_eq!(rbac.applied(&user).len(), 2);
}

#[test]
fn subjects_are_counted_once_per_binding_and_roles_sorted() {
    let rbac = fixture();
    let subjects = rbac.subjects();
    let ci = subjects
        .iter()
        .find(|s| s.subject == sa("prod", "ci"))
        .unwrap();
    assert_eq!(ci.bindings, 2);
    assert_eq!(subjects[0].subject.kind, SubjectKind::User);
    assert_eq!(rbac.roles.last().unwrap().kind, RoleKind::Role);
}

#[test]
fn a_check_reads_like_kubectl_auth_can_i() {
    assert_eq!(
        split_resource("deployments.apps", None),
        ("deployments".into(), Some("apps".into()))
    );
    assert_eq!(split_resource("secrets", None), ("secrets".into(), None));
    assert_eq!(
        split_resource("scale", Some("apps")),
        ("scale".into(), Some("apps".into()))
    );
    let check = AccessCheck {
        verb: "delete".into(),
        resource: "secrets".into(),
        namespace: Some("prod".into()),
        ..Default::default()
    };
    assert_eq!(
        command("delete", "secrets", &check),
        "kubectl auth can-i delete secrets --namespace prod"
    );
    let everywhere = AccessCheck {
        name: Some("web".into()),
        subresource: Some("log".into()),
        ..check
    };
    let mut everywhere = everywhere;
    everywhere.namespace = None;
    assert_eq!(
        command("get", "pods", &everywhere),
        "kubectl auth can-i get pods/web --subresource log --all-namespaces"
    );
}

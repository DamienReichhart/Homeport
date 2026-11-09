//! Which types of objects a Cluster serves, built-in and from CRDs, as its
//! API's discovery documents list them: for browsing any of them, and for
//! knowing what a YAML document's `apiVersion` and `kind` name.

use futures_util::future::join_all;
use k8s_openapi::{
    api::authorization::v1::{ResourceRule, SelfSubjectRulesReview, SelfSubjectRulesReviewSpec},
    apimachinery::pkg::apis::meta::v1::{APIResource, APIResourceList},
};
use kube::api::{Api, PostParams};
use serde::Serialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    messages as msg,
    resources::{ApiType, On},
};

/// A type of object the Cluster serves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ApiResourceInfo {
    pub api: ApiType,
    /// `pvc`, `deploy`…: what kubectl also accepts.
    pub short_names: Vec<String>,
    /// What can be done with its objects: `list`, `watch`, `delete`, `patch`…
    pub verbs: Vec<String>,
    /// `all`, …
    pub categories: Vec<String>,
    /// Whether the credentials may list its objects in the namespace asked
    /// about; `None` when the cluster can't say (some authorizers don't).
    pub listable: Option<bool>,
}

impl ApiResourceInfo {
    pub fn can(&self, verb: &str) -> bool {
        self.verbs.iter().any(|v| v == verb)
    }
}

/// The types of a group version's resource list, without subresources
/// (`pods/log`), which aren't types of their own.
pub fn types_of(list: &APIResourceList) -> Vec<ApiResourceInfo> {
    let (group, version) = match list.group_version.split_once('/') {
        Some((group, version)) => (group.to_owned(), version.to_owned()),
        None => (String::new(), list.group_version.clone()),
    };
    list.resources
        .iter()
        .filter(|r| !r.name.contains('/'))
        .map(|r: &APIResource| ApiResourceInfo {
            api: ApiType {
                group: r.group.clone().unwrap_or_else(|| group.clone()),
                version: r.version.clone().unwrap_or_else(|| version.clone()),
                kind: r.kind.clone(),
                plural: r.name.clone(),
                namespaced: r.namespaced,
            },
            short_names: r.short_names.clone().unwrap_or_default(),
            verbs: r.verbs.clone(),
            categories: r.categories.clone().unwrap_or_default(),
            listable: None,
        })
        .collect()
}

/// Whether `rules` let `verb` be done on `plural` of `group`.
pub fn allows(rules: &[ResourceRule], group: &str, plural: &str, verb: &str) -> bool {
    let has = |list: &[String], value: &str| list.iter().any(|v| v == "*" || v == value);
    rules.iter().any(|rule| {
        // Rules for some objects by name don't allow listing them all.
        rule.resource_names.as_ref().is_none_or(Vec::is_empty)
            && has(&rule.verbs, verb)
            && has(rule.api_groups.as_deref().unwrap_or_default(), group)
            && has(rule.resources.as_deref().unwrap_or_default(), plural)
    })
}

/// Sorts types by group, the core group first, then by kind.
pub fn sort(types: &mut [ApiResourceInfo]) {
    types.sort_by(|a, b| {
        (!a.api.group.is_empty(), &a.api.group, &a.api.kind).cmp(&(
            !b.api.group.is_empty(),
            &b.api.group,
            &b.api.kind,
        ))
    });
}

impl On {
    /// What the credentials may do in `namespace`, as the API server says;
    /// `None` when it can't say for sure.
    pub async fn rules(&self, namespace: &str) -> Option<Vec<ResourceRule>> {
        let review = SelfSubjectRulesReview {
            spec: SelfSubjectRulesReviewSpec {
                namespace: Some(namespace.to_owned()),
            },
            ..Default::default()
        };
        let api: Api<SelfSubjectRulesReview> = Api::all(self.client.clone());
        match api.create(&PostParams::default(), &review).await {
            Ok(answer) => {
                let status = answer.status?;
                (!status.incomplete).then_some(status.resource_rules)
            }
            Err(e) => {
                log::debug!("couldn't review the credentials' rules: {e}");
                None
            }
        }
    }

    /// Every type the Cluster serves, at each group's preferred version,
    /// each marked with whether the credentials may list it in `namespace`
    /// (the default one for every namespace).
    pub async fn discover_for(&self, namespace: Option<&str>) -> AppResult<Vec<ApiResourceInfo>> {
        let ns = namespace
            .unwrap_or_else(|| self.client.default_namespace())
            .to_owned();
        let (types, rules) = futures_util::join!(self.discover(), self.rules(&ns));
        let mut types = types?;
        if let Some(rules) = rules {
            for t in &mut types {
                t.listable = Some(allows(&rules, &t.api.group, &t.api.plural, "list"));
            }
        }
        Ok(types)
    }

    /// Every type the Cluster serves, at each group's preferred version.
    /// A group whose server doesn't answer (an aggregated API that is down,
    /// like a missing metrics-server) is left out.
    pub async fn discover(&self) -> AppResult<Vec<ApiResourceInfo>> {
        let failed = |e: &kube::Error| classify_discovery(self, e);
        let core = self
            .client
            .list_core_api_resources("v1")
            .await
            .map_err(|e| failed(&e))?;
        let groups = self
            .client
            .list_api_groups()
            .await
            .map_err(|e| failed(&e))?;
        let versions: Vec<String> = groups
            .groups
            .iter()
            .filter_map(|g| {
                g.preferred_version
                    .as_ref()
                    .or_else(|| g.versions.first())
                    .map(|v| v.group_version.clone())
            })
            .collect();
        let lists = join_all(
            versions
                .iter()
                .map(|gv| self.client.list_api_group_resources(gv)),
        )
        .await;
        let mut types = types_of(&core);
        for (gv, list) in versions.iter().zip(lists) {
            match list {
                Ok(list) => types.extend(types_of(&list)),
                Err(e) => log::debug!("left out {gv}, whose discovery failed: {e}"),
            }
        }
        sort(&mut types);
        Ok(types)
    }

    /// The type a document's `apiVersion` and `kind` name, or why there's none.
    pub async fn resolve_kind(&self, api_version: &str, kind: &str) -> AppResult<ApiResourceInfo> {
        let list = if api_version.contains('/') {
            self.client.list_api_group_resources(api_version).await
        } else {
            self.client.list_core_api_resources(api_version).await
        };
        let unknown = || {
            AppError::new(
                ErrorCode::NotFound,
                msg::KIND_UNKNOWN
                    .with("kind", kind)
                    .with("apiVersion", api_version)
                    .with("name", &self.cluster.name),
            )
            .with_hint(msg::KIND_UNKNOWN_HINT)
        };
        let list = match list {
            Ok(list) => list,
            Err(kube::Error::Api(status)) if status.code == 404 => return Err(unknown()),
            Err(e) => return Err(classify_discovery(self, &e)),
        };
        types_of(&list)
            .into_iter()
            .find(|t| t.api.kind == kind)
            .ok_or_else(unknown)
    }
}

fn classify_discovery(on: &On, error: &kube::Error) -> AppError {
    crate::classify::classify(&on.cluster.name, on.cluster.address(), error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resource(name: &str, kind: &str, namespaced: bool, short: &[&str]) -> APIResource {
        APIResource {
            name: name.into(),
            kind: kind.into(),
            namespaced,
            short_names: Some(short.iter().map(|s| (*s).to_owned()).collect()),
            verbs: vec!["list".into(), "get".into()],
            singular_name: String::new(),
            ..Default::default()
        }
    }

    #[test]
    fn a_resource_list_maps_to_types_without_subresources() {
        let list = APIResourceList {
            group_version: "cert-manager.io/v1".into(),
            resources: vec![
                resource("certificates", "Certificate", true, &["cert", "certs"]),
                resource("certificates/status", "Certificate", true, &[]),
                resource("clusterissuers", "ClusterIssuer", false, &[]),
            ],
        };
        let types = types_of(&list);
        assert_eq!(types.len(), 2);
        assert_eq!(types[0].listable, None);
        let cert = &types[0];
        assert_eq!(cert.api.group, "cert-manager.io");
        assert_eq!(cert.api.version, "v1");
        assert_eq!(cert.api.plural, "certificates");
        assert!(cert.api.namespaced);
        assert_eq!(cert.short_names, ["cert", "certs"]);
        assert!(cert.can("list") && !cert.can("delete"));
        assert!(!types[1].api.namespaced);
    }

    #[test]
    fn the_core_group_has_no_name_and_sorts_first() {
        let core = APIResourceList {
            group_version: "v1".into(),
            resources: vec![resource(
                "persistentvolumeclaims",
                "PersistentVolumeClaim",
                true,
                &["pvc"],
            )],
        };
        let apps = APIResourceList {
            group_version: "apps/v1".into(),
            resources: vec![resource("deployments", "Deployment", true, &["deploy"])],
        };
        let mut types = types_of(&apps);
        types.extend(types_of(&core));
        sort(&mut types);
        assert_eq!(types[0].api.group, "");
        assert_eq!(types[0].api.api_version(), "v1");
        assert_eq!(types[1].api.api_version(), "apps/v1");
    }

    fn rule(groups: &[&str], resources: &[&str], verbs: &[&str], names: &[&str]) -> ResourceRule {
        let list = |l: &[&str]| Some(l.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>());
        ResourceRule {
            api_groups: list(groups),
            resources: list(resources),
            verbs: verbs.iter().map(|s| (*s).to_owned()).collect(),
            resource_names: if names.is_empty() { None } else { list(names) },
        }
    }

    #[test]
    fn rules_say_what_may_be_listed() {
        let rules = [
            rule(&[""], &["pods", "services"], &["get", "list"], &[]),
            rule(&["cert-manager.io"], &["*"], &["*"], &[]),
            rule(&[""], &["secrets"], &["list"], &["only-this"]),
        ];
        assert!(allows(&rules, "", "pods", "list"));
        assert!(!allows(&rules, "", "pods", "delete"));
        assert!(allows(&rules, "cert-manager.io", "certificates", "list"));
        assert!(!allows(&rules, "apps", "deployments", "list"));
        assert!(!allows(&rules, "", "secrets", "list"));
    }
}

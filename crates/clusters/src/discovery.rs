//! Which types of objects a Cluster serves, built-in and from CRDs, as its
//! API's discovery documents list them: for browsing any of them, and for
//! knowing what a YAML document's `apiVersion` and `kind` name.

use futures_util::future::join_all;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{APIResource, APIResourceList};
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
        })
        .collect()
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
    crate::classify::classify(&on.cluster.name, &on.cluster.connection.server, error)
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
}

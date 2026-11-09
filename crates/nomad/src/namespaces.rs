//! A Nomad Cluster's namespaces, which the Jobs page picks from. Tokens
//! that can't list them, and agents that answer without any, fall back to
//! the token's default namespace, with nothing to pick.

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppResult, ErrorCode};

use crate::Nomad;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct Namespace {
    name: String,
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadNamespaceView {
    pub name: String,
    pub description: Option<String>,
}

/// The namespaces a page can pick from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadNamespaces {
    /// By name, `default` first.
    pub namespaces: Vec<NomadNamespaceView>,
    /// `false` when the token can't list them or Nomad has none to list:
    /// pages then show the token's default namespace, with no picker.
    pub listable: bool,
}

impl Nomad {
    /// The namespaces the token can see; an empty, unlistable answer
    /// rather than an error when it can't list them.
    pub async fn namespaces(&self) -> AppResult<NomadNamespaces> {
        let listed: Vec<Namespace> = match self.get("/v1/namespaces", &[]).await {
            Ok(listed) => listed,
            Err(e)
                if e.is(ErrorCode::NomadPermissionDenied)
                    || e.is(ErrorCode::NotFound)
                    || e.is(ErrorCode::BadRequest) =>
            {
                return Ok(NomadNamespaces {
                    namespaces: vec![],
                    listable: false,
                })
            }
            Err(e) => return Err(e),
        };
        let mut namespaces: Vec<_> = listed
            .into_iter()
            .map(|n| NomadNamespaceView {
                description: Some(n.description).filter(|d| !d.is_empty()),
                name: n.name,
            })
            .collect();
        namespaces
            .sort_by(|a, b| (a.name != "default", &a.name).cmp(&(b.name != "default", &b.name)));
        Ok(NomadNamespaces {
            listable: !namespaces.is_empty(),
            namespaces,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::testing::{agent, ok};

    #[tokio::test]
    async fn namespaces_are_listed_default_first() {
        let fake = agent(vec![ok(
            "GET /v1/namespaces",
            r#"[{"Name":"shop","Description":"The shop"},{"Name":"default","Description":""},{"Name":"billing"}]"#,
        )])
        .await;
        let listed = fake.client(None).namespaces().await.unwrap();
        assert!(listed.listable);
        let names: Vec<_> = listed.namespaces.iter().map(|n| n.name.as_str()).collect();
        assert_eq!(names, ["default", "billing", "shop"]);
        assert_eq!(
            listed.namespaces[2].description.as_deref(),
            Some("The shop")
        );
    }

    #[tokio::test]
    async fn a_token_that_cant_list_namespaces_falls_back_without_an_error() {
        let fake = agent(vec![(
            "GET /v1/namespaces",
            403,
            vec![],
            "Permission denied".into(),
        )])
        .await;
        let listed = fake.client(Some("t")).namespaces().await.unwrap();
        assert_eq!(
            listed,
            NomadNamespaces {
                namespaces: vec![],
                listable: false
            }
        );
    }

    #[tokio::test]
    async fn an_agent_without_namespaces_falls_back_too() {
        let fake = agent(vec![ok("GET /v1/namespaces", "[]")]).await;
        assert!(!fake.client(None).namespaces().await.unwrap().listable);
    }

    #[tokio::test]
    async fn other_failures_are_errors() {
        let fake = agent(vec![(
            "GET /v1/namespaces",
            500,
            vec![],
            "No cluster leader".into(),
        )])
        .await;
        let e = fake.client(None).namespaces().await.unwrap_err();
        assert!(e.is(ErrorCode::NomadNoLeader));
    }
}

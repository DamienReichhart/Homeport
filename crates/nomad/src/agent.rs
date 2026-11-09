//! A Nomad Cluster's status: whether its servers have a leader, the
//! version and region of the agent Homeport talks to, and whether the ACL
//! token is one Nomad knows.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{client::Nomad, messages as msg};

/// What Homeport shows of a connected Nomad Cluster.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadAgent {
    /// Such as `1.10.3`; `None` when the token can't read the agent.
    pub version: Option<String>,
    /// The region Homeport asks, such as `global`.
    pub region: Option<String>,
    /// The agent's datacenter, such as `dc1`.
    pub datacenter: Option<String>,
    /// The leading server's RPC address, such as `10.0.0.5:4647`.
    pub leader: String,
}

/// The text at `path` in `value`, when there is some.
fn text(value: &Value, path: &[&str]) -> Option<String> {
    path.iter()
        .try_fold(value, |v, key| v.get(key))?
        .as_str()
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
}

/// The version, region and datacenter `GET /v1/agent/self` answered.
fn read_agent(agent: &Value) -> (Option<String>, Option<String>, Option<String>) {
    let version = text(agent, &["config", "Version", "Version"])
        .map(
            |v| match text(agent, &["config", "Version", "VersionPrerelease"]) {
                Some(pre) => format!("{v}-{pre}"),
                None => v,
            },
        )
        .or_else(|| text(agent, &["member", "Tags", "build"]));
    let region =
        text(agent, &["config", "Region"]).or_else(|| text(agent, &["member", "Tags", "region"]));
    let datacenter =
        text(agent, &["config", "Datacenter"]).or_else(|| text(agent, &["member", "Tags", "dc"]));
    (version, region, datacenter)
}

impl Nomad {
    /// The Cluster's status. Fails when its servers have no leader or the
    /// ACL token is unknown; a token that can't read the agent still connects,
    /// without the version.
    pub async fn agent(&self) -> AppResult<NomadAgent> {
        let leader: String = self.get("/v1/status/leader", &[]).await?;
        if leader.trim().is_empty() {
            return Err(AppError::new(
                ErrorCode::NomadNoLeader,
                msg::NO_LEADER.with("name", self.name()),
            )
            .with_hint(msg::NO_LEADER_HINT)
            .with_command("nomad server members"));
        }
        if self.token().is_some() {
            // Answers 400 when ACLs are off, which is fine: the token is ignored.
            if let Err(e) = self.get::<Value>("/v1/acl/token/self", &[]).await {
                if e.is(ErrorCode::NomadTokenInvalid) {
                    return Err(e);
                }
            }
        }
        match self.get::<Value>("/v1/agent/self", &[]).await {
            Ok(agent) => {
                let (version, region, datacenter) = read_agent(&agent);
                Ok(NomadAgent {
                    version,
                    region: self.region().map(str::to_owned).or(region),
                    datacenter,
                    leader,
                })
            }
            Err(e) if e.is(ErrorCode::NomadPermissionDenied) => {
                let regions: Vec<String> = self.get("/v1/regions", &[]).await?;
                Ok(NomadAgent {
                    version: None,
                    region: self
                        .region()
                        .map(str::to_owned)
                        .or_else(|| regions.into_iter().next()),
                    datacenter: None,
                    leader,
                })
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::testing::{agent, ok};

    const AGENT: &str = r#"{
        "config": { "Region": "global", "Datacenter": "dc1",
                    "Version": { "Version": "1.10.3", "VersionPrerelease": "" } },
        "member": { "Tags": { "build": "1.10.3", "region": "global", "dc": "dc1" } }
    }"#;

    #[tokio::test]
    async fn a_connected_cluster_shows_its_version_and_region() {
        let fake = agent(vec![
            ok("GET /v1/status/leader", r#""10.0.0.5:4647""#),
            ok("GET /v1/acl/token/self", "{}"),
            ok("GET /v1/agent/self", AGENT),
        ])
        .await;
        let status = fake.client(Some("t")).agent().await.unwrap();
        assert_eq!(
            status,
            NomadAgent {
                version: Some("1.10.3".into()),
                region: Some("global".into()),
                datacenter: Some("dc1".into()),
                leader: "10.0.0.5:4647".into(),
            }
        );
    }

    #[tokio::test]
    async fn no_leader_is_an_error() {
        let fake = agent(vec![ok("GET /v1/status/leader", r#""""#)]).await;
        let e = fake.client(None).agent().await.unwrap_err();
        assert!(e.is(ErrorCode::NomadNoLeader));
    }

    #[tokio::test]
    async fn an_unknown_token_is_refused() {
        let fake = agent(vec![
            ok("GET /v1/status/leader", r#""10.0.0.5:4647""#),
            (
                "GET /v1/acl/token/self",
                403,
                vec![],
                "ACL token not found".into(),
            ),
            ok("GET /v1/agent/self", AGENT),
        ])
        .await;
        let e = fake.client(Some("wrong")).agent().await.unwrap_err();
        assert!(e.is(ErrorCode::NomadTokenInvalid));
    }

    #[tokio::test]
    async fn a_token_on_a_cluster_without_acls_connects() {
        let fake = agent(vec![
            ok("GET /v1/status/leader", r#""10.0.0.5:4647""#),
            (
                "GET /v1/acl/token/self",
                400,
                vec![],
                "ACL support disabled".into(),
            ),
            ok("GET /v1/agent/self", AGENT),
        ])
        .await;
        assert!(fake.client(Some("t")).agent().await.is_ok());
    }

    #[tokio::test]
    async fn a_token_that_cant_read_the_agent_falls_back_to_the_regions() {
        let fake = agent(vec![
            ok("GET /v1/status/leader", r#""10.0.0.5:4647""#),
            ok("GET /v1/acl/token/self", "{}"),
            (
                "GET /v1/agent/self",
                403,
                vec![],
                "Permission denied".into(),
            ),
            ok("GET /v1/regions", r#"["eu-west","us-east"]"#),
        ])
        .await;
        let status = fake.client(Some("t")).agent().await.unwrap();
        assert_eq!(status.version, None);
        assert_eq!(status.region.as_deref(), Some("eu-west"));
    }

    #[test]
    fn a_prerelease_is_part_of_the_version() {
        let agent: Value = serde_json::from_str(
            r#"{"config":{"Version":{"Version":"1.11.0","VersionPrerelease":"dev"}}}"#,
        )
        .unwrap();
        assert_eq!(read_agent(&agent).0.as_deref(), Some("1.11.0-dev"));
    }
}

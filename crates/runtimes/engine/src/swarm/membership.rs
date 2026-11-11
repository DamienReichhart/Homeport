//! Making a Host a Swarm's first manager, joining one, leaving one, and
//! the join tokens a manager hands out. Tokens pass through Homeport and are
//! never stored or logged (ADR 0016).

use std::fmt;

use bollard::{
    models::{NodeSpecRoleEnum, SwarmInitRequest, SwarmJoinRequest},
    query_parameters::{LeaveSwarmOptionsBuilder, ListNodesOptions},
    Docker,
};
use serde::{Deserialize, Serialize};

use homeport_core::error::{engine::from_bollard, AppError, AppResult, ErrorCode};

use crate::messages as msg;

/// The port managers listen on for other nodes.
pub const MANAGER_PORT: u16 = 2377;

/// What `docker swarm join` needs for each role. `Debug` hides the tokens.
#[derive(Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct JoinInfo {
    pub worker_token: String,
    pub manager_token: String,
    /// `address:2377` of every reachable manager.
    pub manager_addrs: Vec<String>,
}

impl fmt::Debug for JoinInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinInfo")
            .field("worker_token", &"<redacted>")
            .field("manager_token", &"<redacted>")
            .field("manager_addrs", &self.manager_addrs)
            .finish()
    }
}

/// Whether a node joins to run tasks or to manage the Swarm too.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum JoinRole {
    Worker,
    Manager,
}

impl JoinInfo {
    pub fn token(&self, role: JoinRole) -> &str {
        match role {
            JoinRole::Worker => &self.worker_token,
            JoinRole::Manager => &self.manager_token,
        }
    }
}

fn non_empty(s: Option<&str>) -> Option<String> {
    s.map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
}

/// Makes the Engine the first manager of a new Swarm, returning its ID.
/// Without an advertise address, the Engine picks one if it can.
pub async fn init(
    docker: &Docker,
    advertise_addr: Option<&str>,
    listen_addr: Option<&str>,
) -> AppResult<String> {
    docker
        .init_swarm(SwarmInitRequest {
            advertise_addr: non_empty(advertise_addr),
            listen_addr: Some(
                non_empty(listen_addr).unwrap_or_else(|| format!("0.0.0.0:{MANAGER_PORT}")),
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| classify(&e))
}

/// Joins the Swarm the managers at `remote_addrs` belong to, with the token
/// of the role to join as.
pub async fn join(
    docker: &Docker,
    remote_addrs: &[String],
    token: &str,
    advertise_addr: Option<&str>,
) -> AppResult<()> {
    let token = token.trim();
    let addrs: Vec<String> = remote_addrs
        .iter()
        .map(|a| a.trim())
        .filter(|a| !a.is_empty())
        .map(|a| {
            if a.contains(':') {
                a.to_owned()
            } else {
                format!("{a}:{MANAGER_PORT}")
            }
        })
        .collect();
    if token.is_empty() || addrs.is_empty() {
        return Err(AppError::validation(msg::JOIN_NEEDS_TOKEN));
    }
    docker
        .join_swarm(SwarmJoinRequest {
            remote_addrs: Some(addrs),
            join_token: Some(token.to_owned()),
            advertise_addr: non_empty(advertise_addr),
            listen_addr: Some(format!("0.0.0.0:{MANAGER_PORT}")),
            ..Default::default()
        })
        .await
        .map_err(|e| classify(&e))
}

/// Leaves the Swarm; a manager needs `force`, and leaving as the last one
/// loses the Swarm's services, secrets and configs.
pub async fn leave(docker: &Docker, force: bool) -> AppResult<()> {
    docker
        .leave_swarm(Some(
            LeaveSwarmOptionsBuilder::default().force(force).build(),
        ))
        .await
        .map_err(|e| classify(&e))
}

/// The join tokens and manager addresses, read through a manager.
pub async fn join_info(docker: &Docker) -> AppResult<JoinInfo> {
    let swarm = docker.inspect_swarm().await?;
    let tokens = swarm.join_tokens.unwrap_or_default();
    let manager_addrs = docker
        .list_nodes(None::<ListNodesOptions>)
        .await?
        .into_iter()
        .filter(|n| n.spec.as_ref().and_then(|s| s.role) == Some(NodeSpecRoleEnum::MANAGER))
        .filter_map(|n| n.manager_status?.addr)
        .filter(|a| !a.is_empty())
        .collect();
    Ok(JoinInfo {
        worker_token: tokens.worker.unwrap_or_default(),
        manager_token: tokens.manager.unwrap_or_default(),
        manager_addrs,
    })
}

/// `docker swarm join --token … <addr>` for one role, for machines Homeport
/// doesn't manage.
pub fn join_command(info: &JoinInfo, role: JoinRole) -> String {
    let addr = info
        .manager_addrs
        .first()
        .map_or("<manager>:2377", String::as_str);
    format!("docker swarm join --token {} {addr}", info.token(role))
}

/// Turns the Engine's Swarm refusals into what the user can act on.
pub fn classify(e: &bollard::errors::Error) -> AppError {
    let text = e.to_string().to_lowercase();
    let has = |needle: &str| text.contains(needle);
    if has("already part of a swarm") {
        return AppError::new(ErrorCode::AlreadyDone, msg::ALREADY_IN_SWARM)
            .with_hint(msg::ALREADY_IN_SWARM_HINT)
            .with_cause(e);
    }
    if has("not part of a swarm") {
        return AppError::new(ErrorCode::AlreadyDone, msg::NOT_IN_SWARM).with_cause(e);
    }
    if has("participating as a manager") {
        return AppError::new(ErrorCode::Conflict, msg::LEAVE_NEEDS_FORCE).with_cause(e);
    }
    if has("advertise address")
        || has("could not choose an ip address")
        || has("multiple ip addresses")
    {
        return AppError::validation(msg::ADVERTISE_NEEDED)
            .with_hint(msg::ADVERTISE_NEEDED_HINT)
            .with_cause(e);
    }
    if has("invalid join token") || has("token") && has("invalid") {
        return AppError::validation(msg::JOIN_TOKEN_INVALID).with_cause(e);
    }
    if has("timeout was reached")
        || has("could not connect")
        || has("connection refused")
        || has("code = unavailable")
        || has("deadline exceeded")
        || has("no route to host")
    {
        return AppError::new(ErrorCode::SwarmJoinFailed, msg::JOIN_FAILED)
            .with_hint(msg::JOIN_FAILED_HINT)
            .with_cause(e);
    }
    from_bollard(e)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine(status: u16, message: &str) -> bollard::errors::Error {
        bollard::errors::Error::DockerResponseServerError {
            status_code: status,
            message: message.to_owned(),
        }
    }

    #[test]
    fn swarm_refusals_are_classified() {
        for (message, code) in [
            ("This node is already part of a swarm. Use \"docker swarm leave\" to leave this swarm and join another one.", ErrorCode::AlreadyDone),
            ("This node is not part of a swarm", ErrorCode::AlreadyDone),
            ("You are attempting to leave the swarm on a node that is participating as a manager. Removing the last manager erases all current state of the swarm. Use `--force` to ignore this message.", ErrorCode::Conflict),
            ("could not choose an IP address to advertise since this system has multiple addresses on interface eth0 (10.0.0.1 and 10.0.0.2)", ErrorCode::Validation),
            ("Timeout was reached before node joined. The attempt to join the swarm will continue in the background. Use the \"docker info\" command to see the current swarm status of your node.", ErrorCode::SwarmJoinFailed),
            ("rpc error: code = Unavailable desc = connection error: desc = \"transport: Error while dialing dial tcp 10.0.0.9:2377: connect: connection refused\"", ErrorCode::SwarmJoinFailed),
            ("invalid join token", ErrorCode::Validation),
        ] {
            assert_eq!(classify(&engine(503, message)).code, code, "{message}");
        }
        let failed = classify(&engine(503, "Timeout was reached before node joined."));
        assert!(failed.hint.unwrap().text.contains("2377"));
    }

    #[test]
    fn tokens_stay_out_of_debug() {
        let info = JoinInfo {
            worker_token: "SWMTKN-1-worker".into(),
            manager_token: "SWMTKN-1-manager".into(),
            manager_addrs: vec!["10.0.0.1:2377".into()],
        };
        let shown = format!("{info:?}");
        assert!(!shown.contains("SWMTKN"), "{shown}");
        assert_eq!(
            join_command(&info, JoinRole::Worker),
            "docker swarm join --token SWMTKN-1-worker 10.0.0.1:2377"
        );
    }

    #[tokio::test]
    async fn joining_needs_a_token_and_an_address() {
        let docker = crate::testing::offline();
        for (addrs, token) in [
            (vec![], "SWMTKN"),
            (vec![" ".to_string()], "SWMTKN"),
            (vec!["10.0.0.1".to_string()], " "),
        ] {
            let refused = join(&docker, &addrs, token, None).await.unwrap_err();
            assert!(refused.is(ErrorCode::Validation));
        }
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn init_inspect_and_leave() {
        let docker = crate::testing::docker();
        // Hold the Swarm lock with a Swarm, then leave it ourselves.
        let guard = crate::testing::swarm(&docker).await;
        let again = init(&docker, Some("127.0.0.1"), Some("127.0.0.1:2377")).await;
        assert!(
            matches!(&again, Err(e) if e.is(ErrorCode::AlreadyDone)),
            "{again:?}"
        );
        let info = join_info(&docker).await.unwrap();
        assert!(info.worker_token.starts_with("SWMTKN-"));
        assert_ne!(info.worker_token, info.manager_token);
        assert_eq!(info.manager_addrs.len(), 1);
        let refused = leave(&docker, false).await.unwrap_err();
        assert!(refused.is(ErrorCode::Conflict), "{refused:?}");
        if guard.created() {
            // A Swarm of the test's own: leave it, and init a new one the guard leaves.
            leave(&docker, true).await.unwrap();
            assert_eq!(super::super::membership(&docker).await.unwrap(), None);
            let id = init(&docker, Some("127.0.0.1"), Some("127.0.0.1:2377"))
                .await
                .unwrap();
            assert!(!id.is_empty());
        }
        drop(guard);
    }
}

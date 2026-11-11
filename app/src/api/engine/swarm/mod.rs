//! Swarms, found through their manager Hosts and reached through any
//! connected one (ADR 0016).

pub mod nodes;
pub mod objects;
pub mod services;

use homeport_core::error::AppResult;
use homeport_engine::swarm::{
    self,
    membership::{self, JoinInfo, JoinRole},
    SwarmSummary,
};
use homeport_hosts::registry::SwarmView;

use crate::app::AppState;

/// Every Swarm the Hosts belong to, once each.
pub async fn swarms_list(state: &AppState) -> AppResult<Vec<SwarmView>> {
    Ok(state.hosts.registry.swarms().await)
}

/// What a connected manager says about the Swarm `swarm_id`.
pub async fn swarm_inspect(state: &AppState, swarm_id: String) -> AppResult<SwarmSummary> {
    swarm::summary(&state.hosts.registry.manager(&swarm_id)?).await
}

/// Reads every Host's place in a Swarm again, after a change to its nodes.
async fn refresh_members(state: &AppState) {
    for host in state.hosts.store.list().unwrap_or_default() {
        state.hosts.registry.refresh_swarm(&host.id).await;
    }
}

/// Makes `host_id` the first manager of a new Swarm, returning its ID.
pub async fn swarm_init(
    state: &AppState,
    host_id: String,
    advertise_addr: Option<String>,
    listen_addr: Option<String>,
) -> AppResult<String> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let id = membership::init(&docker, advertise_addr.as_deref(), listen_addr.as_deref()).await?;
    state.hosts.registry.refresh_swarm(&host_id).await;
    Ok(id)
}

/// The Swarm a Host joins: one Homeport knows, whose token and manager
/// addresses are fetched through a connected manager, or a pasted token.
#[derive(Clone, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum JoinTarget {
    Known {
        swarm_id: String,
    },
    Token {
        token: String,
        addresses: Vec<String>,
    },
}

impl std::fmt::Debug for JoinTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known { swarm_id } => write!(f, "Known {{ swarm_id: {swarm_id:?} }}"),
            Self::Token { addresses, .. } => write!(f, "Token {{ addresses: {addresses:?} }}"),
        }
    }
}

/// Joins `host_id` to a Swarm as a worker or a manager.
pub async fn swarm_join(
    state: &AppState,
    host_id: String,
    target: JoinTarget,
    role: JoinRole,
    advertise_addr: Option<String>,
) -> AppResult<()> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let (token, addresses) = match target {
        JoinTarget::Known { swarm_id } => {
            let info = membership::join_info(&state.hosts.registry.manager(&swarm_id)?).await?;
            (info.token(role).to_owned(), info.manager_addrs)
        }
        JoinTarget::Token { token, addresses } => (token, addresses),
    };
    let joined = membership::join(&docker, &addresses, &token, advertise_addr.as_deref()).await;
    state.hosts.registry.refresh_swarm(&host_id).await;
    joined
}

/// Has `host_id` leave its Swarm; a manager needs `force`.
pub async fn swarm_leave(state: &AppState, host_id: String, force: bool) -> AppResult<()> {
    let docker = state.hosts.registry.docker(&host_id)?;
    membership::leave(&docker, force).await?;
    state.hosts.registry.refresh_swarm(&host_id).await;
    Ok(())
}

/// The join tokens and manager addresses of a Swarm, for its Join commands;
/// never stored or logged.
pub async fn swarm_join_info(state: &AppState, swarm_id: String) -> AppResult<JoinInfo> {
    membership::join_info(&state.hosts.registry.manager(&swarm_id)?).await
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::api::engine::testing::{assert_all, code, host, swarm};
    use crate::app::testing::app;

    fn token() -> JoinTarget {
        JoinTarget::Token {
            token: "SWMTKN-1-secret".into(),
            addresses: vec!["10.0.0.1:2377".into()],
        }
    }

    #[tokio::test]
    async fn no_host_means_no_swarm() {
        let app = app();
        assert!(swarms_list(&app.state).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn a_swarm_without_a_connected_manager_is_unreachable() {
        let app = app();
        let s = &app.state;
        let results = [
            ("inspect", code(swarm_inspect(s, swarm()).await)),
            ("join_info", code(swarm_join_info(s, swarm()).await)),
        ];
        assert_all(ErrorCode::SwarmUnreachable, &results);
    }

    #[tokio::test]
    async fn a_host_that_isnt_connected_cant_change_its_membership() {
        let app = app();
        let s = &app.state;
        let known = JoinTarget::Known { swarm_id: swarm() };
        let addr = || Some("10.0.0.2".to_owned());
        let results = [
            ("init", code(swarm_init(s, host(), addr(), None).await)),
            (
                "join by token",
                code(swarm_join(s, host(), token(), JoinRole::Worker, addr()).await),
            ),
            (
                "join known",
                code(swarm_join(s, host(), known, JoinRole::Manager, None).await),
            ),
            ("leave", code(swarm_leave(s, host(), true).await)),
        ];
        assert_all(ErrorCode::HostNotConnected, &results);
    }

    #[test]
    fn a_join_token_is_never_printed() {
        let printed = format!("{:?}", token());
        assert!(!printed.contains("SWMTKN"), "{printed}");
        assert!(printed.contains("10.0.0.1:2377"), "{printed}");
    }
}

//! Where a Stack runs: its Host, or its Swarm through a manager Host. The
//! Stacks' crate doesn't know Hosts, so the shell joins the two here.

use homeport_core::error::AppResult;
use homeport_engine::{
    auth::{Logins, RegistryAuth},
    Docker,
};
use homeport_stacks::{
    compose::ComposeAction,
    runtime::{ComposeJob, Reach, SourceKind, StackTarget},
    StackOwner,
};

use crate::app::AppState;

/// Checks a new Stack's Host or Swarm is one Homeport knows.
pub(crate) fn known(state: &AppState, owner: &StackOwner) -> AppResult<()> {
    match owner {
        StackOwner::Host(id) => state.hosts.store.get(id).map(drop),
        StackOwner::Swarm(id) => state.hosts.registry.manager_host(id).map(drop),
    }
}

/// The Engine that answers for `owner`: the Host's, or the one of a
/// connected manager of the Swarm.
pub(crate) fn engine(state: &AppState, owner: &StackOwner) -> AppResult<Docker> {
    match owner {
        StackOwner::Host(id) => state.hosts.registry.docker(id),
        StackOwner::Swarm(id) => state.hosts.registry.manager(id),
    }
}

/// How commands reach the Host `host_id`.
fn reach(state: &AppState, host_id: &str) -> AppResult<Reach> {
    let host = state.hosts.store.get(host_id)?;
    Ok(Reach {
        name: host.name,
        connection: host.connection,
        dialer: Some(state.hosts.registry.dialer(host_id)?),
        docker: state.hosts.registry.docker(host_id)?,
    })
}

/// Where the Stacks of `owner` run: a Host's Compose projects, or a Swarm's
/// swarm stacks through one of its connected managers.
pub(crate) fn of(state: &AppState, owner: &StackOwner) -> AppResult<StackTarget> {
    Ok(match owner {
        StackOwner::Host(host_id) => StackTarget::Host(reach(state, host_id)?),
        StackOwner::Swarm(swarm_id) => {
            let manager = reach(state, &state.hosts.registry.manager_host(swarm_id)?)?;
            StackTarget::Swarm {
                swarm_id: swarm_id.clone(),
                name: format!("the Swarm of {}", manager.name),
                manager,
            }
        }
    })
}

/// Every stored registry login, and the rule that matches them to image
/// references, for a deploy that pulls several images.
fn logins(state: &AppState) -> AppResult<Logins> {
    Ok(Logins::new(
        state
            .registries
            .all_credentials()?
            .into_iter()
            .map(|c| RegistryAuth {
                address: c.address,
                username: c.username,
                secret: c.secret,
            })
            .collect(),
        homeport_registries::registry_of,
    ))
}

/// The compose job for `action` on the Managed, Linked or Synced Stack `id`
/// with the registry logins Homeport holds, recorded as started by
/// auto-deploy when `automatic`; and the commit a Synced Stack uses.
pub(crate) async fn job(
    state: &AppState,
    kind: SourceKind,
    id: &str,
    action: ComposeAction,
    commit: Option<&str>,
    automatic: bool,
) -> AppResult<(ComposeJob, Option<String>)> {
    state
        .stacks
        .job(
            |owner| of(state, owner),
            logins(state)?,
            kind,
            id,
            action,
            commit,
            automatic,
        )
        .await
}

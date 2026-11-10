//! Swarms, found through their manager Hosts and reached through any
//! connected one (ADR 0016).

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_engine::{
    containers::logs::{LogEvent, LogOptions},
    networks::{self, NetworkView},
    swarm::{
        self,
        membership::{self, JoinInfo, JoinRole},
        nodes::{self, NodeChange, NodeView},
        objects::{self, ConfigContent, ObjectInput, SwarmObject},
        services::{self, ServiceView},
        spec::{self, ServiceChange, ServiceSpecInput},
        tasks::{self, TaskFilter, TaskView},
        SwarmSummary,
    },
};
use homeport_hosts::registry::SwarmView;
use homeport_stacks::{
    linked::{files as linked_files, folder, LinkedStack},
    swarm::{self as stacks_swarm, RemoveReport, SwarmStackParts},
    Stack, StackOwner,
};

use crate::{app::AppState, error::AppResult};

/// Every Swarm the Hosts belong to, once each.
#[tauri::command]
#[specta::specta]
pub async fn swarms_list(state: State<'_, AppState>) -> AppResult<Vec<SwarmView>> {
    Ok(state.hosts.registry.swarms().await)
}

/// What a connected manager says about the Swarm `swarm_id`.
#[tauri::command]
#[specta::specta]
pub async fn swarm_inspect(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<SwarmSummary> {
    swarm::summary(&state.hosts.registry.manager(&swarm_id)?).await
}

/// A Swarm's nodes, each with the Host it is, if it's one.
#[tauri::command]
#[specta::specta]
pub async fn swarm_nodes_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<SwarmNode>> {
    let nodes = nodes::list(&state.hosts.registry.manager(&swarm_id)?).await?;
    let hosts = state.hosts.registry.node_hosts();
    Ok(nodes
        .into_iter()
        .map(|node| SwarmNode {
            host_id: hosts.get(&node.id).cloned(),
            node,
        })
        .collect())
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_nodes_inspect(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    nodes::inspect(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Changes a node's availability, role or labels, from the node as read at
/// `version`; refuses to demote the last manager or drain every node.
#[tauri::command]
#[specta::specta]
pub async fn swarm_nodes_update(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    version: u64,
    change: NodeChange,
) -> AppResult<()> {
    let docker = state.hosts.registry.manager(&swarm_id)?;
    nodes::check(&nodes::list(&docker).await?, &id, &change)?;
    nodes::update(&docker, &id, version, change).await?;
    refresh_members(&state).await;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_nodes_remove(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    force: bool,
) -> AppResult<()> {
    nodes::remove(&state.hosts.registry.manager(&swarm_id)?, &id, force).await?;
    refresh_members(&state).await;
    Ok(())
}

/// A node of a Swarm, and the Host it is when Homeport connects to it.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SwarmNode {
    #[serde(flatten)]
    pub node: NodeView,
    pub host_id: Option<String>,
}

/// Reads every Host's place in a Swarm again, after a change to its nodes.
async fn refresh_members(state: &AppState) {
    for host in state.hosts.store.list().unwrap_or_default() {
        state.hosts.registry.refresh_swarm(&host.id).await;
    }
}

/// Every service of a Swarm, with its running and desired tasks.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<ServiceView>> {
    services::list(&state.hosts.registry.manager(&swarm_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_services_inspect(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    services::inspect(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Sets a replicated service's replicas, from the service as read at `version`.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_scale(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    version: u64,
    replicas: u64,
) -> AppResult<()> {
    services::scale(
        &state.hosts.registry.manager(&swarm_id)?,
        &id,
        version,
        replicas,
    )
    .await
}

/// Puts back a service's previous spec.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_rollback(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    services::rollback(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_services_remove(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    services::remove(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Streams the logs of every task of a service, each line labelled with its
/// task and node.
#[tauri::command]
#[specta::specta]
pub async fn swarm_service_logs_subscribe(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    options: LogOptions,
    on_event: Channel<LogEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.manager(&swarm_id)?;
    Ok(state
        .streams
        .spawn(services::stream_logs(docker, id, options, move |e| {
            on_event.send(e).is_ok()
        })))
}

/// The tasks of a Swarm, of one of its services or of one of its nodes,
/// newest first, each with the Host its node is, if it's one.
#[tauri::command]
#[specta::specta]
pub async fn swarm_tasks_list(
    state: State<'_, AppState>,
    swarm_id: String,
    filter: TaskFilter,
) -> AppResult<Vec<SwarmTask>> {
    let tasks = tasks::list(&state.hosts.registry.manager(&swarm_id)?, &filter).await?;
    let hosts = state.hosts.registry.node_hosts();
    Ok(tasks
        .into_iter()
        .map(|task| SwarmTask {
            host_id: task.node_id.as_ref().and_then(|n| hosts.get(n).cloned()),
            task,
        })
        .collect())
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_tasks_inspect(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    tasks::inspect(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// A task of a Swarm, and the Host its node is when Homeport connects to it.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SwarmTask {
    #[serde(flatten)]
    pub task: TaskView,
    pub host_id: Option<String>,
}

/// A Swarm's secrets, never their values.
#[tauri::command]
#[specta::specta]
pub async fn swarm_secrets_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<SwarmObject>> {
    objects::secrets(&state.hosts.registry.manager(&swarm_id)?).await
}

/// Sends a new secret's value to the Swarm once; Homeport keeps none of it.
#[tauri::command]
#[specta::specta]
pub async fn swarm_secrets_create(
    state: State<'_, AppState>,
    swarm_id: String,
    input: ObjectInput,
) -> AppResult<String> {
    objects::create_secret(&state.hosts.registry.manager(&swarm_id)?, input).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_secrets_remove(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    objects::remove_secret(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_configs_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<SwarmObject>> {
    objects::configs(&state.hosts.registry.manager(&swarm_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_configs_create(
    state: State<'_, AppState>,
    swarm_id: String,
    input: ObjectInput,
) -> AppResult<String> {
    objects::create_config(&state.hosts.registry.manager(&swarm_id)?, input).await
}

/// A config's content, as text when it is.
#[tauri::command]
#[specta::specta]
pub async fn swarm_configs_inspect(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<ConfigContent> {
    objects::config_content(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_configs_remove(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    objects::remove_config(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Creates a service from the Create service form, returning its ID.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_create(
    state: State<'_, AppState>,
    swarm_id: String,
    input: ServiceSpecInput,
) -> AppResult<String> {
    spec::create(&state.hosts.registry.manager(&swarm_id)?, &input).await
}

/// Changes a service's image, environment or replicas, from the service as
/// read at `version`; the Swarm rolls its tasks over.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_update(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    version: u64,
    change: ServiceChange,
) -> AppResult<()> {
    spec::update(
        &state.hosts.registry.manager(&swarm_id)?,
        &id,
        version,
        &change,
    )
    .await
}

/// The Swarm's overlay networks, which services attach to.
#[tauri::command]
#[specta::specta]
pub async fn swarm_networks_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<NetworkView>> {
    let all = networks::list(&state.hosts.registry.manager(&swarm_id)?).await?;
    Ok(all.into_iter().filter(|n| n.driver == "overlay").collect())
}

/// Makes `host_id` the first manager of a new Swarm, returning its ID.
#[tauri::command]
#[specta::specta]
pub async fn swarm_init(
    state: State<'_, AppState>,
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
#[derive(Clone, serde::Deserialize, specta::Type)]
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
#[tauri::command]
#[specta::specta]
pub async fn swarm_join(
    state: State<'_, AppState>,
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
#[tauri::command]
#[specta::specta]
pub async fn swarm_leave(
    state: State<'_, AppState>,
    host_id: String,
    force: bool,
) -> AppResult<()> {
    let docker = state.hosts.registry.docker(&host_id)?;
    membership::leave(&docker, force).await?;
    state.hosts.registry.refresh_swarm(&host_id).await;
    Ok(())
}

/// The join tokens and manager addresses of a Swarm, for its Join commands;
/// never stored or logged.
#[tauri::command]
#[specta::specta]
pub async fn swarm_join_info(state: State<'_, AppState>, swarm_id: String) -> AppResult<JoinInfo> {
    membership::join_info(&state.hosts.registry.manager(&swarm_id)?).await
}

/// The Stacks deployed to a Swarm, found by their namespace label.
#[tauri::command]
#[specta::specta]
pub async fn swarm_stacks_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<Stack>> {
    let services = services::list(&state.hosts.registry.manager(&swarm_id)?).await?;
    stacks_swarm::list(
        &swarm_id,
        &services,
        &state.stacks.managed,
        &state.stacks.linked,
        &state.stacks.synced.store,
        &state.stacks.linked_files,
    )
}

/// The networks, secrets and configs of a swarm stack.
#[tauri::command]
#[specta::specta]
pub async fn swarm_stacks_parts(
    state: State<'_, AppState>,
    swarm_id: String,
    name: String,
) -> AppResult<SwarmStackParts> {
    stacks_swarm::parts(&state.hosts.registry.manager(&swarm_id)?, &name).await
}

/// Removes a swarm stack as `docker stack rm` does, through the Engine API.
#[tauri::command]
#[specta::specta]
pub async fn swarm_stacks_remove(
    state: State<'_, AppState>,
    swarm_id: String,
    name: String,
) -> AppResult<RemoveReport> {
    stacks_swarm::remove(&state.hosts.registry.manager(&swarm_id)?, &name).await
}

/// Links the External swarm stack `name` to a folder on this computer,
/// once its compose files are checked to declare the services it runs.
#[tauri::command]
#[specta::specta]
pub async fn swarm_stacks_link(
    state: State<'_, AppState>,
    swarm_id: String,
    name: String,
    working_dir: String,
) -> AppResult<LinkedStack> {
    let services = services::list(&state.hosts.registry.manager(&swarm_id)?).await?;
    let found = folder::inspect(std::path::Path::new(&working_dir))?;
    let texts: Vec<&str> = found.files.iter().map(|f| f.content.as_str()).collect();
    stacks_swarm::check_link(&name, &services, &texts)?;
    let config_files: Vec<String> = found.files.iter().map(|f| f.path.clone()).collect();
    let fingerprint = linked_files::files_fingerprint(&config_files);
    state.stacks.linked.create(
        StackOwner::Swarm(swarm_id),
        &name,
        &found.working_dir,
        &config_files,
        &fingerprint,
    )
}

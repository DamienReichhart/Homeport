//! Nomad Clusters: adding, editing and testing one. What runs on them is
//! read through the same Cluster registry as Kubernetes Clusters.

use tauri::{ipc::Channel, State};

use homeport_clusters::{nomad::NomadForm, service::ClusterView};
use homeport_core::{
    logs::{LogEvent, LogOptions},
    streams::StreamId,
    terminal::ExecEvent,
};
use homeport_nomad::{
    allocations::NomadAllocationDetail,
    exec::{NomadExecOptions, NomadTerminal, TerminalId},
    jobs::{NomadJobDetail, NomadJobView},
    namespaces::NomadNamespaces,
    nodes::{NomadDrainSpec, NomadNodeDetail, NomadNodeView},
    run::{NomadJobSource, NomadPlan, NomadPlanToRun, NomadRunResult},
    watch::{watch, NomadWatch},
    NomadAgent,
};

use crate::{app::AppState, error::AppResult};

/// Adds a Nomad Cluster, its ACL token and TLS key sealed, and starts
/// connecting to it.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_add(
    state: State<'_, AppState>,
    form: NomadForm,
) -> AppResult<ClusterView> {
    state.clusters.add_nomad(&form)
}

/// Changes a Nomad Cluster's name, address and credentials (those left
/// out are kept), then reconnects.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_update(
    state: State<'_, AppState>,
    id: String,
    form: NomadForm,
) -> AppResult<ClusterView> {
    state.clusters.update_nomad(&id, &form)
}

/// Connects once to the Nomad Cluster the form describes, without saving
/// it. With `id`, the credentials left out are that Cluster's.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_test(
    state: State<'_, AppState>,
    form: NomadForm,
    id: Option<String>,
) -> AppResult<NomadAgent> {
    state.clusters.test_nomad(&form, id.as_deref()).await
}

// Jobs and their allocations. `namespace` is the token's default when
// `None`, and every namespace the token can read with `*`.

/// The connected Nomad Cluster `id`'s client.
fn nomad(state: &AppState, id: &str) -> AppResult<homeport_nomad::Nomad> {
    Ok(state.clusters.registry.nomad(id)?.0)
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_jobs(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<NomadJobView>> {
    nomad(&state, &id)?.jobs(namespace.as_deref()).await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_job(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
    job_id: String,
) -> AppResult<NomadJobDetail> {
    nomad(&state, &id)?.job(namespace.as_deref(), &job_id).await
}

/// Stops a job; with `purge`, Nomad forgets it at once.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_job_stop(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
    job_id: String,
    purge: bool,
) -> AppResult<()> {
    nomad(&state, &id)?
        .stop_job(namespace.as_deref(), &job_id, purge)
        .await
}

/// Sets how many allocations a task group runs, within its scaling policy.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_job_scale(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
    job_id: String,
    group: String,
    count: u32,
) -> AppResult<()> {
    nomad(&state, &id)?
        .scale_task_group(namespace.as_deref(), &job_id, &group, count)
        .await
}

/// Sends `true` each time what `topic` covers changes, through Nomad's
/// blocking queries, until the stream is cancelled.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_watch_subscribe(
    state: State<'_, AppState>,
    id: String,
    topic: NomadWatch,
    on_event: Channel<bool>,
) -> AppResult<StreamId> {
    let nomad = nomad(&state, &id)?;
    Ok(state
        .streams
        .spawn(watch(nomad, topic, move || on_event.send(true).is_ok())))
}

// Running a job from its specification.

/// Parses `spec` (HCL2 by Nomad itself, or JSON) and asks Nomad what
/// running it would change. A job naming no namespace goes to `namespace`.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_job_plan(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
    spec: String,
) -> AppResult<NomadPlan> {
    let nomad = nomad(&state, &id)?;
    let (job, format) = nomad.parse_job(&spec).await?;
    nomad.plan_job(job, format, namespace.as_deref()).await
}

/// Runs a planned job unless it changed since, keeping `source` with it.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_job_run(
    state: State<'_, AppState>,
    id: String,
    plan: NomadPlanToRun,
    source: Option<String>,
) -> AppResult<NomadRunResult> {
    nomad(&state, &id)?.run_job(&plan, source.as_deref()).await
}

/// A job's specification to edit: as submitted when Nomad kept it, else its JSON.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_job_source(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
    job_id: String,
) -> AppResult<NomadJobSource> {
    nomad(&state, &id)?
        .job_source(namespace.as_deref(), &job_id)
        .await
}

// An allocation: its detail, its tasks' logs and terminals in them.

#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_allocation(
    state: State<'_, AppState>,
    id: String,
    alloc_id: String,
) -> AppResult<NomadAllocationDetail> {
    nomad(&state, &id)?.allocation(&alloc_id).await
}

/// Streams a task's stdout and stderr, as `options` asks, until cancelled
/// or they end.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_logs_subscribe(
    state: State<'_, AppState>,
    id: String,
    alloc_id: String,
    task: String,
    options: LogOptions,
    on_event: Channel<LogEvent>,
) -> AppResult<StreamId> {
    let nomad = nomad(&state, &id)?;
    Ok(state.streams.spawn(async move {
        nomad
            .stream_logs(alloc_id, task, options, move |e| on_event.send(e).is_ok())
            .await;
    }))
}

/// Opens a terminal in a task, like `nomad alloc exec -t`.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_exec_start(
    state: State<'_, AppState>,
    id: String,
    alloc_id: String,
    options: NomadExecOptions,
    on_event: Channel<ExecEvent>,
) -> AppResult<TerminalId> {
    let nomad = nomad(&state, &id)?;
    let (terminal, output) = NomadTerminal::open(&nomad, &alloc_id, &options).await?;
    Ok(state
        .nomad_terminals
        .open(nomad, alloc_id, options.task, terminal, output, move |e| {
            on_event.send(e).is_ok()
        }))
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_exec_input(
    state: State<'_, AppState>,
    terminal: TerminalId,
    data: String,
) -> AppResult<()> {
    state
        .nomad_terminals
        .get(terminal)?
        .write(data.as_bytes())
        .await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_exec_resize(
    state: State<'_, AppState>,
    terminal: TerminalId,
    cols: u16,
    rows: u16,
) -> AppResult<()> {
    state
        .nomad_terminals
        .get(terminal)?
        .resize(cols, rows)
        .await
}

/// Ends a terminal and its command.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_exec_close(
    state: State<'_, AppState>,
    terminal: TerminalId,
) -> AppResult<()> {
    state.nomad_terminals.close(terminal);
    Ok(())
}

// Client nodes.

#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_nodes(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<Vec<NomadNodeView>> {
    nomad(&state, &id)?.nodes().await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_node(
    state: State<'_, AppState>,
    id: String,
    node_id: String,
) -> AppResult<NomadNodeDetail> {
    nomad(&state, &id)?.node(&node_id).await
}

/// Starts draining a node with `spec`; with none, cancels its drain and,
/// with `mark_eligible`, makes it eligible again.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_node_drain(
    state: State<'_, AppState>,
    id: String,
    node_id: String,
    spec: Option<NomadDrainSpec>,
    mark_eligible: bool,
) -> AppResult<()> {
    nomad(&state, &id)?
        .drain_node(&node_id, spec.as_ref(), mark_eligible)
        .await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_node_eligibility(
    state: State<'_, AppState>,
    id: String,
    node_id: String,
    eligible: bool,
) -> AppResult<()> {
    nomad(&state, &id)?
        .set_eligibility(&node_id, eligible)
        .await
}

/// The namespaces the Jobs page picks from; unlistable rather than an
/// error when the token can't list them.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_namespaces(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<NomadNamespaces> {
    nomad(&state, &id)?.namespaces().await
}

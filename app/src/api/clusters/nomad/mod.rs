//! Nomad Clusters: adding, editing and testing one. What runs on them is
//! read through the same Cluster registry as Kubernetes Clusters.

use homeport_clusters::{nomad::NomadForm, service::ClusterView};
use homeport_core::{
    error::AppResult,
    logs::{LogEvent, LogOptions},
    streams::StreamId,
    terminal::ExecEvent,
};
use homeport_nomad::{
    allocations::{
        exec::{NomadExecOptions, NomadTerminal, TerminalId},
        NomadAllocationDetail,
    },
    api::watch::{watch, NomadWatch},
    jobs::{
        run::{NomadJobSource, NomadPlan, NomadPlanToRun, NomadRunResult},
        NomadJobDetail, NomadJobView,
    },
    namespaces::NomadNamespaces,
    nodes::{NomadDrainSpec, NomadNodeDetail, NomadNodeView},
    NomadAgent,
};

use crate::app::AppState;

/// Adds a Nomad Cluster, its ACL token and TLS key sealed, and starts
/// connecting to it.
pub async fn clusters_nomad_add(state: &AppState, form: NomadForm) -> AppResult<ClusterView> {
    state.clusters.add_nomad(&form)
}

/// Changes a Nomad Cluster's name, address and credentials (those left
/// out are kept), then reconnects.
pub async fn clusters_nomad_update(
    state: &AppState,
    id: String,
    form: NomadForm,
) -> AppResult<ClusterView> {
    state.clusters.update_nomad(&id, &form)
}

/// Connects once to the Nomad Cluster the form describes, without saving
/// it. With `id`, the credentials left out are that Cluster's.
pub async fn clusters_nomad_test(
    state: &AppState,
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

pub async fn clusters_nomad_jobs(
    state: &AppState,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<NomadJobView>> {
    nomad(state, &id)?.jobs(namespace.as_deref()).await
}

pub async fn clusters_nomad_job(
    state: &AppState,
    id: String,
    namespace: Option<String>,
    job_id: String,
) -> AppResult<NomadJobDetail> {
    nomad(state, &id)?.job(namespace.as_deref(), &job_id).await
}

/// Stops a job; with `purge`, Nomad forgets it at once.
pub async fn clusters_nomad_job_stop(
    state: &AppState,
    id: String,
    namespace: Option<String>,
    job_id: String,
    purge: bool,
) -> AppResult<()> {
    nomad(state, &id)?
        .stop_job(namespace.as_deref(), &job_id, purge)
        .await
}

/// Sets how many allocations a task group runs, within its scaling policy.
pub async fn clusters_nomad_job_scale(
    state: &AppState,
    id: String,
    namespace: Option<String>,
    job_id: String,
    group: String,
    count: u32,
) -> AppResult<()> {
    nomad(state, &id)?
        .scale_task_group(namespace.as_deref(), &job_id, &group, count)
        .await
}

/// Sends `true` each time what `topic` covers changes, through Nomad's
/// blocking queries, until the stream is cancelled.
pub async fn clusters_nomad_watch_subscribe(
    state: &AppState,
    id: String,
    topic: NomadWatch,
    on_event: impl Fn(bool) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let nomad = nomad(state, &id)?;
    Ok(state
        .streams
        .spawn(watch(nomad, topic, move || on_event(true))))
}

// Running a job from its specification.

/// Parses `spec` (HCL2 by Nomad itself, or JSON) and asks Nomad what
/// running it would change. A job naming no namespace goes to `namespace`.
pub async fn clusters_nomad_job_plan(
    state: &AppState,
    id: String,
    namespace: Option<String>,
    spec: String,
) -> AppResult<NomadPlan> {
    let nomad = nomad(state, &id)?;
    let (job, format) = nomad.parse_job(&spec).await?;
    nomad.plan_job(job, format, namespace.as_deref()).await
}

/// Runs a planned job unless it changed since, keeping `source` with it.
pub async fn clusters_nomad_job_run(
    state: &AppState,
    id: String,
    plan: NomadPlanToRun,
    source: Option<String>,
) -> AppResult<NomadRunResult> {
    nomad(state, &id)?.run_job(&plan, source.as_deref()).await
}

/// A job's specification to edit: as submitted when Nomad kept it, else its JSON.
pub async fn clusters_nomad_job_source(
    state: &AppState,
    id: String,
    namespace: Option<String>,
    job_id: String,
) -> AppResult<NomadJobSource> {
    nomad(state, &id)?
        .job_source(namespace.as_deref(), &job_id)
        .await
}

// An allocation: its detail, its tasks' logs and terminals in them.

pub async fn clusters_nomad_allocation(
    state: &AppState,
    id: String,
    alloc_id: String,
) -> AppResult<NomadAllocationDetail> {
    nomad(state, &id)?.allocation(&alloc_id).await
}

/// Streams a task's stdout and stderr, as `options` asks, until cancelled
/// or they end.
pub async fn clusters_nomad_logs_subscribe(
    state: &AppState,
    id: String,
    alloc_id: String,
    task: String,
    options: LogOptions,
    on_event: impl Fn(LogEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let nomad = nomad(state, &id)?;
    Ok(state.streams.spawn(async move {
        nomad.stream_logs(alloc_id, task, options, on_event).await;
    }))
}

/// Opens a terminal in a task, like `nomad alloc exec -t`.
pub async fn clusters_nomad_exec_start(
    state: &AppState,
    id: String,
    alloc_id: String,
    options: NomadExecOptions,
    on_event: impl Fn(ExecEvent) -> bool + Send + Sync + 'static,
) -> AppResult<TerminalId> {
    let nomad = nomad(state, &id)?;
    let (terminal, output) = NomadTerminal::open(&nomad, &alloc_id, &options).await?;
    Ok(state
        .terminals
        .nomad
        .open(nomad, alloc_id, options.task, terminal, output, on_event))
}

pub async fn clusters_nomad_exec_input(
    state: &AppState,
    terminal: TerminalId,
    data: String,
) -> AppResult<()> {
    state
        .terminals
        .nomad
        .get(terminal)?
        .write(data.as_bytes())
        .await
}

pub async fn clusters_nomad_exec_resize(
    state: &AppState,
    terminal: TerminalId,
    cols: u16,
    rows: u16,
) -> AppResult<()> {
    state
        .terminals
        .nomad
        .get(terminal)?
        .resize(cols, rows)
        .await
}

/// Ends a terminal and its command.
pub async fn clusters_nomad_exec_close(state: &AppState, terminal: TerminalId) -> AppResult<()> {
    state.terminals.nomad.close(terminal);
    Ok(())
}

// Client nodes.

pub async fn clusters_nomad_nodes(state: &AppState, id: String) -> AppResult<Vec<NomadNodeView>> {
    nomad(state, &id)?.nodes().await
}

pub async fn clusters_nomad_node(
    state: &AppState,
    id: String,
    node_id: String,
) -> AppResult<NomadNodeDetail> {
    nomad(state, &id)?.node(&node_id).await
}

/// Starts draining a node with `spec`; with none, cancels its drain and,
/// with `mark_eligible`, makes it eligible again.
pub async fn clusters_nomad_node_drain(
    state: &AppState,
    id: String,
    node_id: String,
    spec: Option<NomadDrainSpec>,
    mark_eligible: bool,
) -> AppResult<()> {
    nomad(state, &id)?
        .drain_node(&node_id, spec.as_ref(), mark_eligible)
        .await
}

pub async fn clusters_nomad_node_eligibility(
    state: &AppState,
    id: String,
    node_id: String,
    eligible: bool,
) -> AppResult<()> {
    nomad(state, &id)?.set_eligibility(&node_id, eligible).await
}

/// The namespaces the Jobs page picks from; unlistable rather than an
/// error when the token can't list them.
pub async fn clusters_nomad_namespaces(state: &AppState, id: String) -> AppResult<NomadNamespaces> {
    nomad(state, &id)?.namespaces().await
}

#[cfg(test)]
mod tests;

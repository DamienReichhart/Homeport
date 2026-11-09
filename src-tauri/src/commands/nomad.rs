//! Nomad Clusters: adding, editing and testing one. What runs on them is
//! read through the same Cluster registry as Kubernetes Clusters.

use tauri::{ipc::Channel, State};

use homeport_clusters::{nomad::NomadForm, service::ClusterView};
use homeport_core::streams::StreamId;
use homeport_nomad::{
    jobs::{NomadJobDetail, NomadJobView},
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

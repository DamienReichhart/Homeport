//! Incus Instances: listing, acting on, creating and deleting them.

use homeport_core::{error::AppResult, pull::PullEvent, streams::StreamId};
use homeport_incus::instances::{
    self, IncusInstance, IncusInstanceDetail, InstanceAction, InstanceSpec,
};

use crate::app::AppState;

/// The Instances of `project`, with their state.
pub async fn incus_instances(
    state: &AppState,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusInstance>> {
    instances::list(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// One Instance with its configuration, devices and snapshots' names.
pub async fn incus_instance(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<IncusInstanceDetail> {
    instances::get(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

/// Starts, stops, restarts, freezes or unfreezes an Instance; `force` stops
/// or restarts it without waiting for its system to shut down.
pub async fn incus_instance_act(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
    action: InstanceAction,
    force: bool,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    instances::act(&incus, &project, &name, action, force).await
}

/// Makes an Instance from an image, streaming the image download's
/// progress until it's made (and started, if asked).
pub async fn incus_instance_create_subscribe(
    state: &AppState,
    host_id: String,
    project: String,
    spec: InstanceSpec,
    on_event: impl Fn(PullEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let incus = state.hosts.registry.incus(&host_id)?;
    instances::validate_name(&spec.name)?;
    Ok(state.streams.spawn(async move {
        instances::create(&incus, &project, &spec, on_event).await;
    }))
}

/// Deletes an Instance with its snapshots; `stop_first` stops a running one.
pub async fn incus_instance_delete(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
    stop_first: bool,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    instances::delete(&incus, &project, &name, stop_first).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::incus::tests::{not_connected, NO_HOST},
        app::testing,
    };

    fn host() -> String {
        NO_HOST.to_owned()
    }

    fn project() -> String {
        "default".to_owned()
    }

    #[tokio::test]
    async fn instances_need_a_connected_host() {
        let app = testing::app();
        let state = &app.state;
        let name = || "web".to_owned();
        not_connected(incus_instances(state, host(), project()).await);
        not_connected(incus_instance(state, host(), project(), name()).await);
        for action in [
            InstanceAction::Start,
            InstanceAction::Stop,
            InstanceAction::Restart,
            InstanceAction::Freeze,
            InstanceAction::Unfreeze,
        ] {
            not_connected(
                incus_instance_act(state, host(), project(), name(), action, false).await,
            );
        }
        not_connected(incus_instance_delete(state, host(), project(), name(), true).await);
    }

    #[tokio::test]
    async fn a_new_instance_needs_a_connected_host_and_starts_no_stream() {
        let app = testing::app();
        let spec: InstanceSpec = serde_json::from_value(serde_json::json!({
            "name": "web",
            "type": "container",
            "source": { "kind": "local", "fingerprint": "8d6a1f0a3c" },
            "profiles": ["default"],
            "cpu": null,
            "memory": null,
            "disk": null,
            "start": true,
        }))
        .unwrap();
        let created = incus_instance_create_subscribe(&app.state, host(), project(), spec, |_| {
            panic!("no progress may come from an unknown Host")
        })
        .await;
        not_connected(created);
    }
}

//! Incus profiles.

use homeport_core::error::AppResult;
use homeport_incus::profiles::{self, IncusProfile, IncusProfileDetail};

use crate::app::AppState;

/// The profiles of `project`, with the Instances using each.
pub async fn incus_profiles(
    state: &AppState,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusProfile>> {
    profiles::list(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// One profile, with its YAML and the ETag to save it with.
pub async fn incus_profile(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<IncusProfileDetail> {
    profiles::get(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

/// Saves a profile's YAML; refused if it changed since `etag` was read.
pub async fn incus_profile_update(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
    yaml: String,
    etag: Option<String>,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    profiles::update(&incus, &project, &name, &yaml, etag.as_deref()).await
}

pub async fn incus_profile_create(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
    description: String,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    profiles::create(&incus, &project, &name, &description).await
}

/// Deletes an unused profile other than `default`.
pub async fn incus_profile_delete(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<()> {
    profiles::delete(&state.hosts.registry.incus(&host_id)?, &project, &name).await
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
    async fn profiles_need_a_connected_host() {
        let app = testing::app();
        let state = &app.state;
        let name = || "web".to_owned();
        not_connected(incus_profiles(state, host(), project()).await);
        not_connected(incus_profile(state, host(), project(), name()).await);
        let yaml = "config: {}\n".to_owned();
        let etag = Some("abc".to_owned());
        not_connected(incus_profile_update(state, host(), project(), name(), yaml, etag).await);
        let description = "Web servers".to_owned();
        not_connected(incus_profile_create(state, host(), project(), name(), description).await);
        not_connected(incus_profile_delete(state, host(), project(), name()).await);
    }
}

//! Any type of object a Cluster serves, CRDs included: discovering the
//! types, listing their objects as `kubectl get` does, and each object.

use homeport_core::{error::AppResult, streams::StreamId};
use homeport_kubernetes::{
    api::discovery::ApiResourceInfo,
    resources::{self, ApiType, EventView, ObjectDetail, ObjectTable},
};

use super::on;
use crate::app::AppState;

/// Every type the Cluster serves, at each group's preferred version, each
/// marked with whether the credentials may list it in `namespace`.
pub async fn clusters_api_resources(
    state: &AppState,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<ApiResourceInfo>> {
    on(state, &id)?.discover_for(namespace.as_deref()).await
}

/// The objects of `api` in `namespace` (every one without), with the
/// columns the API server suggests.
pub async fn clusters_api_list(
    state: &AppState,
    id: String,
    api: ApiType,
    namespace: Option<String>,
) -> AppResult<ObjectTable> {
    on(state, &id)?.api_list(&api, namespace.as_deref()).await
}

pub async fn clusters_api_get(
    state: &AppState,
    id: String,
    api: ApiType,
    namespace: Option<String>,
    name: String,
) -> AppResult<ObjectDetail> {
    on(state, &id)?
        .api_object(&api, namespace.as_deref(), &name)
        .await
}

/// The Events about an object of any `kind`, warnings first.
pub async fn clusters_api_object_events(
    state: &AppState,
    id: String,
    kind: String,
    namespace: Option<String>,
    name: String,
) -> AppResult<Vec<EventView>> {
    on(state, &id)?
        .api_object_events(&kind, namespace.as_deref(), &name)
        .await
}

/// Told when the objects of a type changed.
#[derive(Debug, Clone)]
pub struct ApiChanged {
    pub api: ApiType,
}

/// Watches the objects of `api` in `namespace` (every one without) and
/// says when they change, until cancelled.
pub async fn clusters_api_watch_subscribe(
    state: &AppState,
    id: String,
    api: ApiType,
    namespace: Option<String>,
    on_event: impl Fn(ApiChanged) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let (client, _) = state.clusters.registry.client(&id)?;
    let told = api.clone();
    Ok(state
        .streams
        .spawn(resources::watch_api(client, api, namespace, move || {
            on_event(ApiChanged { api: told.clone() })
        })))
}

#[cfg(test)]
mod tests {
    use homeport_kubernetes::resources::ResourceKind;

    use super::super::tests::{gone, unknown};
    use super::*;
    use crate::app::testing::app;

    #[tokio::test]
    async fn an_unknown_cluster_serves_no_type_and_is_not_watched() {
        let app = app();
        let state = &app.state;
        let pods = || ResourceKind::Pod.api_type();
        let name = || "web".to_owned();
        assert!(gone(clusters_api_resources(state, unknown(), None).await));
        assert!(gone(
            clusters_api_list(state, unknown(), pods(), None).await
        ));
        assert!(gone(
            clusters_api_get(state, unknown(), pods(), None, name()).await
        ));
        assert!(gone(
            clusters_api_object_events(state, unknown(), "Pod".into(), None, name()).await
        ));
        let watch = clusters_api_watch_subscribe(state, unknown(), pods(), None, |_| {
            panic!("an unknown Cluster's objects changed")
        });
        assert!(gone(watch.await));
    }
}

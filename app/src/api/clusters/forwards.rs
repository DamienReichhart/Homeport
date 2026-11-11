//! Port forwards from this computer to a Cluster's Pods and Services.
//! They listen on loopback only and end when Homeport quits.

use homeport_core::error::AppResult;
use homeport_kubernetes::streams::forward::{self, ForwardId, ForwardTarget, PortForward};

use super::on;
use crate::app::AppState;

/// Forwards `local_port` (a free one without) to `remote_port` of a Pod or Service.
pub async fn clusters_forward_start(
    state: &AppState,
    id: String,
    target: ForwardTarget,
    remote_port: u16,
    local_port: Option<u16>,
) -> AppResult<PortForward> {
    let on = on(state, &id)?;
    state
        .clusters
        .forwards
        .start(on, target, remote_port, local_port)
        .await
}

/// Every active forward, across Clusters.
pub fn clusters_forward_list(state: &AppState) -> Vec<PortForward> {
    state.clusters.forwards.list()
}

pub fn clusters_forward_stop(state: &AppState, forward: ForwardId) {
    state.clusters.forwards.stop(forward);
}

/// Whether `port` is free on this computer, to propose it as the local port.
pub async fn clusters_forward_port_free(port: u16) -> bool {
    forward::is_free(port).await
}

#[cfg(test)]
mod tests {
    use super::super::tests::{gone, unknown};
    use super::*;
    use crate::app::testing::app;

    #[tokio::test]
    async fn no_forward_starts_to_an_unknown_cluster() {
        let app = app();
        let state = &app.state;
        let target = ForwardTarget::Pod {
            namespace: "default".into(),
            name: "web".into(),
        };
        assert!(gone(
            clusters_forward_start(state, unknown(), target, 80, None).await
        ));
        assert!(clusters_forward_list(state).is_empty());
        // Stopping a forward that already ended does nothing.
        clusters_forward_stop(state, 7);
        assert!(clusters_forward_list(state).is_empty());
    }

    #[tokio::test]
    async fn a_port_is_free_until_something_listens_on_it() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        assert!(!clusters_forward_port_free(port).await);
        drop(listener);
        assert!(clusters_forward_port_free(port).await);
    }
}

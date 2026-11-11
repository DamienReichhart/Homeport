//! What the app owes its Hosts as it connects to them and as it quits: the
//! helper containers that open volumes (ADR 0015) don't outlive it, and the
//! last Engine events reach the history.

use homeport_engine::volumes::helper;
use homeport_hosts::registry::HostStatus;

use crate::app::AppState;

/// Removes the helper containers this computer left on a Host it just
/// connected to (Homeport quit or crashed while a volume was open), keeping
/// those of the sessions still open. Does nothing for any other status.
///
/// The window calls it for every [`Event::HostStatus`](crate::app::events::Event).
pub async fn host_status_changed(state: &AppState, host_id: &str, status: &HostStatus) {
    if !matches!(status, HostStatus::Connected { .. }) {
        return;
    }
    let Ok(docker) = state.hosts.registry.docker(host_id) else {
        return;
    };
    let keep = state.volume_browser.helper_ids();
    match helper::remove_leftovers(&docker, &keep).await {
        Ok(0) => {}
        Ok(n) => log::info!("removed {n} leftover helper containers from {host_id}"),
        Err(e) => log::warn!("couldn't look for leftover helper containers: {e}"),
    }
}

/// What must happen before the app exits: a volume being browsed keeps a
/// helper container on its Host, and the last second of Engine events may
/// still be waiting to be written.
pub async fn shutdown(state: &AppState) {
    state.volume_browser.end_all().await;
    state.hosts.events.flush().await;
}

#[cfg(test)]
mod tests {
    use homeport_hosts::registry::HostStatus;

    use crate::app::testing;

    use super::*;

    #[tokio::test]
    async fn a_host_that_is_not_connected_is_left_alone() {
        let app = testing::app();
        host_status_changed(&app.state, "nowhere", &HostStatus::Connecting).await;
        // Connected, but unknown to the registry: nothing to reach.
        let connected = crate::test_support::fixtures::connected();
        host_status_changed(&app.state, "nowhere", &connected).await;
    }

    #[tokio::test]
    async fn shutting_down_with_nothing_open_ends_quietly() {
        let app = testing::app();
        shutdown(&app.state).await;
        assert!(app.state.volume_browser.helper_ids().is_empty());
    }
}

//! Keeping the Engine event history within the retention the user set: the
//! store lives in `homeport-hosts`; this prunes it when Homeport starts,
//! every hour, and when the setting changes.

use std::time::Duration;

use crate::app::AppState;

const PRUNE_EVERY: Duration = Duration::from_secs(3600);

/// Prunes the history now and every hour while Homeport runs: it never
/// ends, so the startup layer (#254) spawns it once the state is built.
pub async fn start(state: &AppState) {
    loop {
        prune(state);
        tokio::time::sleep(PRUNE_EVERY).await;
    }
}

/// Removes the events older than the retention setting, and each Host's
/// beyond the cap.
pub fn prune(state: &AppState) {
    let keep = match state.settings.get() {
        Ok(settings) => settings.keep_events(),
        Err(e) => return log::warn!("couldn't read how long to keep events: {e}"),
    };
    match state.hosts.events.prune(keep) {
        Ok(0) => {}
        Ok(n) => log::info!("removed {n} engine events past their retention"),
        Err(e) => log::warn!("couldn't prune the engine events: {e}"),
    }
}

/// What the tests of the history's pruning share.
#[cfg(test)]
pub(super) mod testing {
    use homeport_core::host::HostConnection;
    use homeport_hosts::{events::now_nano, registry::EngineEvent, store::HostDraft, EngineKind};

    use crate::app::AppState;

    /// Adds a Host, since only the events of a known one are kept.
    pub fn host(state: &AppState) -> String {
        let draft = HostDraft {
            name: "prod".into(),
            connection: HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            engine: EngineKind::Docker,
            secrets: None,
        };
        state.hosts.store.create(draft).unwrap().id
    }

    /// An Engine event of `days` ago.
    pub fn event_of_days_ago(days: i64) -> EngineEvent {
        let time_nano = now_nano() - days * 86_400_000_000_000;
        EngineEvent {
            time: time_nano / 1_000_000_000,
            time_nano,
            kind: "container".to_owned(),
            action: "die".to_owned(),
            actor_id: format!("container-{days}"),
            attributes: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::testing::{event_of_days_ago, host};
    use crate::app::testing;

    #[tokio::test]
    async fn pruning_removes_the_events_past_the_retention() {
        let app = testing::app();
        let keep = app.state.settings.get().unwrap().keep_events_days;
        let events = &app.state.hosts.events;
        let host = host(&app.state);
        events.record(&host, &event_of_days_ago(i64::from(keep) + 1));
        events.record(&host, &event_of_days_ago(0));
        events.flush().await;

        super::prune(&app.state);

        let kept = events.page(&host, 10, None).unwrap();
        assert_eq!(kept.len(), 1, "{kept:?}");
        assert_eq!(kept[0].actor_id, "container-0");
    }

    #[tokio::test]
    async fn starting_prunes_at_once_then_waits() {
        let app = testing::app();
        let events = &app.state.hosts.events;
        let host = host(&app.state);
        events.record(&host, &event_of_days_ago(400));
        events.flush().await;
        assert_eq!(events.page(&host, 10, None).unwrap().len(), 1);

        // It never ends: the hour's wait is what the timeout cuts.
        let ended = tokio::time::timeout(Duration::from_millis(200), super::start(&app.state));
        assert!(ended.await.is_err());

        assert!(events.page(&host, 10, None).unwrap().is_empty());
    }
}

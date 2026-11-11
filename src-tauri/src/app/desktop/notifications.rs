//! OS notifications for a container that died or turned unhealthy, and a
//! Host that became unreachable or came back. They come from the streams
//! Homeport already holds, so nothing is installed on a Host. Driven from
//! Rust only: the frontend is granted no notification permission.

use std::{
    collections::HashMap,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

use homeport_hosts::{
    alerts::{Alert, AlertKind, Alerts},
    registry::{EngineEvent, HostStatus},
};
use homeport_settings::NotificationSettings;

use crate::app::AppState;

/// Turns what the registry reports into notifications, as Settings allows.
#[derive(Default)]
pub struct Notifications {
    alerts: Mutex<Alerts>,
    /// Each Host's name, read again on each status change (renaming a Host
    /// reconnects it), so Engine events don't each read the database.
    names: Mutex<HashMap<String, String>>,
}

impl Notifications {
    pub fn host_status(&self, app: &AppHandle, host_id: &str, status: &HostStatus) {
        let Some(name) = host_name(app, host_id) else {
            return;
        };
        lock(&self.names).insert(host_id.to_owned(), name.clone());
        let alert = lock(&self.alerts).status(host_id, &name, status, now());
        show(app, alert);
    }

    pub fn engine_event(&self, app: &AppHandle, host_id: &str, event: &EngineEvent) {
        if event.kind != "container" {
            return;
        }
        let cached = lock(&self.names).get(host_id).cloned();
        let Some(name) = cached.or_else(|| host_name(app, host_id)) else {
            return;
        };
        let alert = lock(&self.alerts).event(host_id, &name, event, now());
        show(app, alert);
    }
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|e| e.into_inner())
}

/// Whether the user wants notifications of `kind`.
fn allows(settings: &NotificationSettings, kind: AlertKind) -> bool {
    match kind {
        AlertKind::ContainerDied => settings.container_died,
        AlertKind::ContainerUnhealthy => settings.container_unhealthy,
        AlertKind::HostUnreachable => settings.host_unreachable,
        AlertKind::HostBack => settings.host_back,
    }
}

/// Shows `alert` if its kind is on, read from the settings each time so a
/// switch takes effect at once.
fn show(app: &AppHandle, alert: Option<Alert>) {
    let Some(alert) = alert else {
        return;
    };
    let Some(state) = app.try_state::<AppState>() else {
        return;
    };
    let settings = state.settings.get().unwrap_or_default();
    if !allows(&settings.notifications, alert.kind) {
        return;
    }
    let shown = app
        .notification()
        .builder()
        .title(&alert.title)
        .body(&alert.body)
        .show();
    if let Err(e) = shown {
        log::warn!("couldn't show a notification: {e}");
    }
}

fn host_name(app: &AppHandle, host_id: &str) -> Option<String> {
    let state = app.try_state::<AppState>()?;
    state.hosts.store.get(host_id).ok().map(|host| host.name)
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_kind_follows_its_own_switch() {
        let all = NotificationSettings::default();
        let kinds = [
            AlertKind::ContainerDied,
            AlertKind::ContainerUnhealthy,
            AlertKind::HostUnreachable,
            AlertKind::HostBack,
        ];
        assert!(kinds.iter().all(|&k| allows(&all, k)));
        let no_back = NotificationSettings {
            host_back: false,
            ..all
        };
        assert!(!allows(&no_back, AlertKind::HostBack));
        assert!(allows(&no_back, AlertKind::HostUnreachable));
    }
}

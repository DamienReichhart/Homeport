//! What the user can set, and the defaults of a first launch.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// [`Settings::language`] when the app follows the OS's language.
pub const SYSTEM_LANGUAGE: &str = "system";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub theme: Theme,
    /// The app's language: [`SYSTEM_LANGUAGE`] to follow the OS, or a
    /// language tag with a catalogue in the frontend (`en`, `fr`, `pt-BR`…).
    /// A tag the frontend has no catalogue for reads as the OS's language,
    /// so adding a language needs no change here.
    pub language: String,
    /// The Host selected when the app was last closed, kept in step with
    /// [`Settings::last_selection`] so an older Homeport opens on it too.
    pub last_host_id: Option<String>,
    /// The Host or Cluster selected when the app was last closed; `None`
    /// in settings from before Clusters, which only have `last_host_id`.
    pub last_selection: Option<Selection>,
    /// The Namespace each Cluster's pages show, by Cluster id; `""` for
    /// every Namespace. A Cluster without one shows its context's Namespace.
    pub cluster_namespaces: BTreeMap<String, String>,
    /// The namespace each containerd Host's pages show, by Host id. A Host
    /// without one shows `default`, or the only namespace holding something.
    pub containerd_namespaces: BTreeMap<String, String>,
    /// The project each Incus Host's pages show, by Host id. A Host without
    /// one shows `default`, or the first project it may use.
    pub incus_projects: BTreeMap<String, String>,
    /// Look for a new version at launch and every few hours.
    pub check_for_updates: bool,
    /// Minutes between checks of Synced Stacks' repositories while Homeport
    /// runs; at least 1.
    pub sync_interval_minutes: u32,
    /// Days each Host's Engine events are kept for the Events page, from 1
    /// to 365.
    pub keep_events_days: u32,
    /// Closing the window hides Homeport in the system tray instead of
    /// quitting, so its Host connections stay open.
    pub close_to_tray: bool,
    /// When started at login, stay in the system tray without opening the
    /// window. Whether Homeport starts at login is the OS's login entry,
    /// not a setting.
    pub start_hidden: bool,
    /// Which OS notifications Homeport shows.
    pub notifications: NotificationSettings,
}

/// What the resource pages show: one Host or one Cluster, by id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Selection {
    Host { id: String },
    Cluster { id: String },
}

/// The OS notifications Homeport shows, one switch per kind; all on unless
/// turned off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(default, rename_all = "camelCase")]
pub struct NotificationSettings {
    /// A container exited with an error or ran out of memory.
    pub container_died: bool,
    /// A container's health check started failing.
    pub container_unhealthy: bool,
    /// A connected Host stopped answering.
    pub host_unreachable: bool,
    /// An unreachable Host is connected again.
    pub host_back: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            container_died: true,
            container_unhealthy: true,
            host_unreachable: true,
            host_back: true,
        }
    }
}

impl Settings {
    /// The interval between Synced Stacks' polls, never under a minute.
    pub fn sync_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(u64::from(self.sync_interval_minutes.max(1)) * 60)
    }

    /// How long Engine events are kept, at least a day.
    pub fn keep_events(&self) -> std::time::Duration {
        std::time::Duration::from_secs(u64::from(self.keep_events_days.max(1)) * 86_400)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            language: SYSTEM_LANGUAGE.to_owned(),
            last_host_id: None,
            last_selection: None,
            cluster_namespaces: BTreeMap::new(),
            containerd_namespaces: BTreeMap::new(),
            incus_projects: BTreeMap::new(),
            check_for_updates: true,
            sync_interval_minutes: 5,
            keep_events_days: 30,
            close_to_tray: false,
            start_hidden: false,
            notifications: NotificationSettings::default(),
        }
    }
}

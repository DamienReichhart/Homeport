//! The user's preferences (theme, language, last Host, update checks, the desktop), kept as
//! one JSON document in the database.

use std::{collections::BTreeMap, fs, path::Path, sync::Arc};

use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};

use homeport_core::error::AppResult;
use homeport_storage::Db;

const SETTINGS_KEY: &str = "settings";

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

/// Reads and writes [`Settings`] as one JSON document in the `settings` table.
pub struct SettingsService {
    db: Arc<Db>,
}

impl SettingsService {
    /// Wraps `db`, importing a pre-SQLite `settings.json` at `legacy_json` once.
    /// The imported file is renamed to `*.imported` so it is never read again.
    pub fn new(db: Arc<Db>, legacy_json: &Path) -> AppResult<Self> {
        let service = Self { db };
        service.import_legacy(legacy_json)?;
        Ok(service)
    }

    pub fn get(&self) -> AppResult<Settings> {
        let raw: Option<String> = self.db.with(|c| {
            c.query_row(
                "SELECT value FROM settings WHERE key = ?1",
                [SETTINGS_KEY],
                |r| r.get(0),
            )
            .optional()
        })?;
        Ok(match raw {
            Some(raw) => serde_json::from_str(&raw)?,
            None => Settings::default(),
        })
    }

    pub fn update(&self, mut settings: Settings) -> AppResult<Settings> {
        if settings.language.trim().is_empty() {
            SYSTEM_LANGUAGE.clone_into(&mut settings.language);
        }
        settings.sync_interval_minutes = settings.sync_interval_minutes.clamp(1, 24 * 60);
        settings.keep_events_days = settings.keep_events_days.clamp(1, 365);
        let raw = serde_json::to_string(&settings)?;
        self.db.with(|c| {
            c.execute(
                "INSERT INTO settings (key, value) VALUES (?1, ?2)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                (SETTINGS_KEY, raw),
            )
        })?;
        Ok(settings)
    }

    /// Writes `settings` in `tx`, as restoring a backup does.
    pub fn restore(tx: &rusqlite::Transaction<'_>, settings: &Settings) -> AppResult<()> {
        tx.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            (SETTINGS_KEY, serde_json::to_string(settings)?),
        )
        .map_err(homeport_storage::db::sqlite_error)?;
        Ok(())
    }

    fn import_legacy(&self, path: &Path) -> AppResult<()> {
        let Ok(raw) = fs::read_to_string(path) else {
            return Ok(());
        };
        match serde_json::from_str::<Settings>(&raw) {
            Ok(settings) => {
                self.update(settings)?;
                fs::rename(path, path.with_extension("json.imported"))?;
            }
            Err(e) => log::warn!(
                "ignoring unreadable legacy settings {}: {e}",
                path.display()
            ),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service(dir: &Path) -> SettingsService {
        SettingsService::new(
            Arc::new(Db::open_in_memory().unwrap()),
            &dir.join("settings.json"),
        )
        .unwrap()
    }

    #[test]
    fn empty_store_yields_defaults() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(service(dir.path()).get().unwrap(), Settings::default());
    }

    #[test]
    fn the_language_follows_the_os_unless_chosen() {
        let dir = tempfile::tempdir().unwrap();
        let svc = service(dir.path());
        // Settings saved before 0.22 have no language and keep the rest.
        let saved: Settings =
            serde_json::from_str(r#"{ "theme": "dark", "checkForUpdates": false }"#).unwrap();
        assert_eq!(saved.language, SYSTEM_LANGUAGE);
        assert_eq!(saved.theme, Theme::Dark);
        assert!(!saved.check_for_updates);
        let set = svc
            .update(Settings {
                language: "fr".to_owned(),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(set.language, "fr");
        assert_eq!(svc.get().unwrap().language, "fr");
        let blank = svc
            .update(Settings {
                language: String::new(),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(blank.language, SYSTEM_LANGUAGE);
    }

    #[test]
    fn update_checks_are_on_unless_turned_off() {
        let dir = tempfile::tempdir().unwrap();
        let svc = service(dir.path());
        assert!(svc.get().unwrap().check_for_updates);
        // Settings saved before the option existed keep update checks on.
        let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
        assert!(saved.check_for_updates);
        svc.update(Settings {
            check_for_updates: false,
            ..Default::default()
        })
        .unwrap();
        assert!(!svc.get().unwrap().check_for_updates);
    }

    #[test]
    fn synced_stacks_poll_every_five_minutes_and_never_under_one() {
        let dir = tempfile::tempdir().unwrap();
        let svc = service(dir.path());
        assert_eq!(svc.get().unwrap().sync_interval().as_secs(), 300);
        let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
        assert_eq!(saved.sync_interval_minutes, 5, "settings from before 0.11");
        let set = svc
            .update(Settings {
                sync_interval_minutes: 0,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(set.sync_interval_minutes, 1);
        assert_eq!(svc.get().unwrap().sync_interval().as_secs(), 60);
    }

    #[test]
    fn events_are_kept_thirty_days_from_one_to_a_year() {
        let dir = tempfile::tempdir().unwrap();
        let svc = service(dir.path());
        let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
        assert_eq!(saved.keep_events_days, 30, "settings from before 0.12");
        for (asked, kept) in [(0, 1), (90, 90), (1000, 365)] {
            let set = svc
                .update(Settings {
                    keep_events_days: asked,
                    ..Default::default()
                })
                .unwrap();
            assert_eq!(set.keep_events_days, kept);
        }
        assert_eq!(svc.get().unwrap().keep_events().as_secs(), 365 * 86_400);
    }

    #[test]
    fn closing_quits_unless_close_to_tray_is_on() {
        let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
        assert!(!saved.close_to_tray, "settings from before 0.13");
        assert!(!Settings::default().close_to_tray);
    }

    #[test]
    fn starting_at_login_opens_the_window_unless_start_hidden_is_on() {
        let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
        assert!(!saved.start_hidden, "settings from before 0.13");
        assert!(!Settings::default().start_hidden);
    }

    #[test]
    fn every_notification_is_on_unless_turned_off() {
        let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
        assert_eq!(
            saved.notifications,
            NotificationSettings::default(),
            "settings from before 0.13"
        );
        let on = NotificationSettings::default();
        assert!(on.container_died && on.container_unhealthy && on.host_unreachable && on.host_back);
        // One kind turned off keeps the others on, as do kinds added later.
        let saved: Settings =
            serde_json::from_str(r#"{ "notifications": { "hostBack": false } }"#).unwrap();
        assert!(!saved.notifications.host_back);
        assert!(saved.notifications.container_died);
    }

    #[test]
    fn settings_from_before_clusters_keep_their_host() {
        let saved: Settings = serde_json::from_str(r#"{ "lastHostId": "h1" }"#).unwrap();
        assert_eq!(saved.last_host_id.as_deref(), Some("h1"));
        assert_eq!(saved.last_selection, None);
    }

    #[test]
    fn a_selected_cluster_is_read_back() {
        let saved: Settings =
            serde_json::from_str(r#"{ "lastSelection": { "kind": "cluster", "id": "c1" } }"#)
                .unwrap();
        assert_eq!(
            saved.last_selection,
            Some(Selection::Cluster { id: "c1".into() })
        );
    }

    #[test]
    fn update_is_read_back() {
        let dir = tempfile::tempdir().unwrap();
        let svc = service(dir.path());
        svc.update(Settings {
            theme: Theme::Dark,
            ..Default::default()
        })
        .unwrap();
        assert_eq!(svc.get().unwrap().theme, Theme::Dark);
    }

    #[test]
    fn legacy_json_is_imported_once() {
        let dir = tempfile::tempdir().unwrap();
        let legacy = dir.path().join("settings.json");
        fs::write(&legacy, r#"{ "theme": "light" }"#).unwrap();
        let db = Arc::new(Db::open_in_memory().unwrap());

        let svc = SettingsService::new(db.clone(), &legacy).unwrap();
        assert_eq!(svc.get().unwrap().theme, Theme::Light);
        assert!(!legacy.exists());
        assert!(dir.path().join("settings.json.imported").exists());

        svc.update(Settings {
            theme: Theme::Dark,
            ..Default::default()
        })
        .unwrap();
        let again = SettingsService::new(db, &legacy).unwrap();
        assert_eq!(again.get().unwrap().theme, Theme::Dark);
    }

    #[test]
    fn unreadable_legacy_json_is_left_alone() {
        let dir = tempfile::tempdir().unwrap();
        let legacy = dir.path().join("settings.json");
        fs::write(&legacy, "{ not json").unwrap();
        let svc = SettingsService::new(Arc::new(Db::open_in_memory().unwrap()), &legacy).unwrap();
        assert_eq!(svc.get().unwrap(), Settings::default());
        assert!(legacy.exists());
    }
}

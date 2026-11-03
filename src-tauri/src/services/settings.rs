use std::{fs, path::Path, sync::Arc};

use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};

use crate::{error::AppResult, services::db::Db};

const SETTINGS_KEY: &str = "settings";

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
    /// The Host selected when the app was last closed.
    pub last_host_id: Option<String>,
    /// Look for a new version at launch and every few hours.
    pub check_for_updates: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            last_host_id: None,
            check_for_updates: true,
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

    pub fn update(&self, settings: Settings) -> AppResult<Settings> {
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

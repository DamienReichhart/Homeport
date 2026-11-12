//! Reading and saving the settings in the database, and importing the file
//! that held them before it.

use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use rusqlite::OptionalExtension;

use homeport_core::error::AppResult;
use homeport_storage::Db;

use crate::model::{Settings, SYSTEM_LANGUAGE};

const SETTINGS_KEY: &str = "settings";

/// Reads and writes [`Settings`] as one JSON document in the `settings` table.
pub struct SettingsService {
    db: Arc<Db>,
    /// Held while a change reads the stored settings and writes them back,
    /// so two changes never start from the same document.
    patching: Mutex<()>,
}

impl SettingsService {
    /// Wraps `db`, importing a pre-SQLite `settings.json` at `legacy_json` once.
    /// The imported file is renamed to `*.imported` so it is never read again.
    pub fn new(db: Arc<Db>, legacy_json: &Path) -> AppResult<Self> {
        let service = Self {
            db,
            patching: Mutex::new(()),
        };
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

    /// Changes the stored settings in place: reads them, applies `change`
    /// and writes them back, one change at a time. What changes one field
    /// (the selection, the theme) never undoes another's, however their
    /// saves interleave.
    pub fn patch(&self, change: impl FnOnce(&mut Settings)) -> AppResult<Settings> {
        let _one_at_a_time = self
            .patching
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut settings = self.get()?;
        change(&mut settings);
        self.update(settings)
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

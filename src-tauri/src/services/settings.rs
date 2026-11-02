use std::{fs, path::PathBuf, sync::RwLock};

use serde::{Deserialize, Serialize};

use crate::error::AppResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

/// Mirrors `Settings` in `src/features/settings/types.ts`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub theme: Theme,
}

/// Loads/persists [`Settings`] as JSON and caches them in memory.
pub struct SettingsService {
    path: PathBuf,
    current: RwLock<Settings>,
}

impl SettingsService {
    /// Loads settings from `path`; a missing file yields defaults.
    pub fn load(path: PathBuf) -> AppResult<Self> {
        let current = match fs::read_to_string(&path) {
            Ok(raw) => serde_json::from_str(&raw)?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Settings::default(),
            Err(e) => return Err(e.into()),
        };
        Ok(Self {
            path,
            current: RwLock::new(current),
        })
    }

    pub fn get(&self) -> Settings {
        self.current.read().expect("settings lock poisoned").clone()
    }

    pub fn update(&self, settings: Settings) -> AppResult<Settings> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        // Write-then-rename so a crash never leaves a truncated file.
        let tmp = self.path.with_extension("json.tmp");
        fs::write(&tmp, serde_json::to_vec_pretty(&settings)?)?;
        fs::rename(&tmp, &self.path)?;
        *self.current.write().expect("settings lock poisoned") = settings.clone();
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_yields_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let svc = SettingsService::load(dir.path().join("settings.json")).unwrap();
        assert_eq!(svc.get(), Settings::default());
    }

    #[test]
    fn update_persists_and_reloads() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("settings.json");
        let svc = SettingsService::load(path.clone()).unwrap();
        svc.update(Settings { theme: Theme::Dark }).unwrap();

        let reloaded = SettingsService::load(path).unwrap();
        assert_eq!(reloaded.get().theme, Theme::Dark);
    }

    #[test]
    fn corrupt_file_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        fs::write(&path, "{ not json").unwrap();
        assert!(SettingsService::load(path).is_err());
    }
}

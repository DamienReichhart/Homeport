//! Exporting Hosts to a passphrase-protected file and importing such a file
//! (ADR 0013). The passphrase is used, never stored or logged.

use std::path::Path;

use serde::{Deserialize, Serialize};

use homeport_core::error::{local_file, AppError, AppResult, ErrorCode};

use super::HostService;
use crate::{
    bundle::{ExportedHost, Payload},
    import::{free_name, ImportFailure, ImportOutcome},
    messages as msg,
    store::HostDraft,
    HostConnection,
};

/// Larger files aren't exports: a thousand Hosts weigh well under a megabyte.
const MAX_BUNDLE_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ExportOutcome {
    pub exported: u32,
    /// Hosts written without their secrets, which couldn't be opened.
    pub without_secrets: Vec<String>,
}

/// A Host in an export file, as the import preview shows it.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BundleHost {
    /// Its place in the file, which the import refers to.
    pub index: u32,
    pub name: String,
    /// `name`, or a free variant when a Host already has it.
    pub suggested_name: String,
    pub name_taken: bool,
    pub connection: HostConnection,
    /// It needs secrets but was exported without them.
    pub secrets_missing: bool,
}

#[derive(Debug, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BundleChoice {
    pub index: u32,
    pub name: String,
}

impl HostService {
    /// Writes the Hosts `ids`, with their secrets and trusted keys, to `path`.
    pub fn export_hosts(
        &self,
        ids: &[String],
        path: &Path,
        passphrase: &str,
    ) -> AppResult<ExportOutcome> {
        if ids.is_empty() {
            return Err(AppError::validation(msg::EXPORT_NOTHING));
        }
        let mut without_secrets = Vec::new();
        let mut hosts = Vec::with_capacity(ids.len());
        for id in ids {
            let mut host = self.exported(id, &mut without_secrets)?;
            host.id = None;
            hosts.push(host);
        }
        let sealed = Payload::new(hosts).seal(passphrase)?;
        // Written beside it first: a failure keeps the export that was there.
        homeport_core::local_files::write_replacing(path, &sealed)?;
        Ok(ExportOutcome {
            exported: ids.len() as u32,
            without_secrets,
        })
    }

    /// Every Host as a backup holds it, ids included; the names of those
    /// whose secrets couldn't be opened are added to `without_secrets`.
    pub fn backup_hosts(&self, without_secrets: &mut Vec<String>) -> AppResult<Vec<ExportedHost>> {
        self.store
            .list()?
            .iter()
            .map(|host| self.exported(&host.id, without_secrets))
            .collect()
    }

    /// Host `id` with its opened secrets, or without them (named in
    /// `without_secrets`) when they can't be opened.
    fn exported(&self, id: &str, without_secrets: &mut Vec<String>) -> AppResult<ExportedHost> {
        let host = self.store.get(id)?;
        let secrets = if host.connection.needs_secrets() {
            match self.store.secrets(id) {
                Ok(secrets) => secrets,
                Err(e)
                    if e.is(ErrorCode::CredentialsLost) || e.is(ErrorCode::KeychainUnavailable) =>
                {
                    without_secrets.push(host.name.clone());
                    None
                }
                Err(e) => return Err(e),
            }
        } else {
            None
        };
        Ok(ExportedHost {
            id: Some(host.id),
            name: host.name,
            connection: host.connection,
            engine: host.engine,
            enabled: host.enabled,
            host_key: host.host_key,
            secrets,
        })
    }

    /// The Hosts in an export file, checked against the existing ones.
    pub fn preview_bundle(&self, path: &Path, passphrase: &str) -> AppResult<Vec<BundleHost>> {
        let payload = read_bundle(path, passphrase)?;
        let taken: Vec<String> = self.store.list()?.into_iter().map(|h| h.name).collect();
        Ok(payload
            .hosts
            .into_iter()
            .enumerate()
            .map(|(i, h)| {
                let suggested_name = free_name(&h.name, &taken);
                BundleHost {
                    index: i as u32,
                    name_taken: suggested_name != h.name,
                    secrets_missing: h.connection.needs_secrets() && h.secrets.is_none(),
                    name: h.name,
                    suggested_name,
                    connection: h.connection,
                }
            })
            .collect())
    }

    /// Creates the chosen Hosts of an export file, under the names chosen,
    /// sealing their secrets under this computer's key. Their trusted server
    /// keys come along, so they connect without a trust prompt.
    pub fn import_bundle(
        &self,
        path: &Path,
        passphrase: &str,
        choices: Vec<BundleChoice>,
    ) -> AppResult<ImportOutcome> {
        let payload = read_bundle(path, passphrase)?;
        let mut outcome = ImportOutcome {
            created: vec![],
            failed: vec![],
        };
        for choice in choices {
            let Some(host) = payload.hosts.get(choice.index as usize).cloned() else {
                continue;
            };
            let source = host.name.clone();
            match self.create_exported(host, &choice.name) {
                Ok(id) => outcome.created.push(self.get(&id)?),
                Err(error) => outcome.failed.push(ImportFailure { source, error }),
            }
        }
        Ok(outcome)
    }

    fn create_exported(&self, host: ExportedHost, name: &str) -> AppResult<String> {
        let created = self.store.create_imported(HostDraft {
            name: name.to_owned(),
            connection: host.connection,
            engine: host.engine,
            secrets: host.secrets,
        })?;
        if let Some(key) = &host.host_key {
            self.store.set_host_key(&created.id, Some(key))?;
        }
        if !host.enabled {
            self.store.set_enabled(&created.id, false)?;
        }
        self.registry.restart(&created.id);
        Ok(created.id)
    }
}

fn read_bundle(path: &Path, passphrase: &str) -> AppResult<Payload> {
    let size = std::fs::metadata(path)
        .map_err(|e| local_file(path, e))?
        .len();
    if size > MAX_BUNDLE_BYTES {
        return Err(AppError::validation(
            homeport_storage::messages::BUNDLE_NOT_A_BUNDLE,
        ));
    }
    let bytes = std::fs::read(path).map_err(|e| local_file(path, e))?;
    Payload::open(passphrase, &bytes)
}

#[cfg(test)]
mod tests;

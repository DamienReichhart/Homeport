//! The diagnostics bundle for bug reports (`homeport-diagnostics`): a zip
//! with a summary of what Homeport runs as and against, and its logs with
//! secrets removed. It is written where the user chooses and sent nowhere;
//! sealed secrets are never read for it.

use std::{
    io::Write,
    path::{Path, PathBuf},
};

use serde::Serialize;

use homeport_clusters::registry::ClusterStatus;
use homeport_core::{
    error::{local_file, AppError, AppResult, ErrorCode},
    host::HostConnection,
};
use homeport_diagnostics::{
    redact, summary, Aliases, AppFacts, ClusterFacts, HostFacts, Inputs, StackCounts,
};
use homeport_hosts::registry::HostStatus;
use homeport_updates::Bundle as Installer;

use crate::app::AppState;

#[cfg(test)]
mod tests;

/// One file of the bundle, as the preview lists it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsFile {
    /// Its path in the zip, such as `logs/Homeport.log`.
    pub name: String,
    /// Its size once secrets are removed, in bytes.
    pub size: u32,
}

/// What the bundle would hold, for the user to read before saving it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsPreview {
    pub files: Vec<DiagnosticsFile>,
    /// `summary.json`, as it would be written.
    pub summary: String,
}

/// The files of a bundle, by their name in the zip.
struct Bundle(Vec<(String, Vec<u8>)>);

fn status_name<T: Serialize>(status: &T) -> String {
    serde_json::to_value(status)
        .ok()
        .and_then(|v| v.get("state").and_then(|s| s.as_str()).map(str::to_owned))
        .unwrap_or_else(|| "unknown".to_owned())
}

fn host_error(status: &HostStatus) -> Option<ErrorCode> {
    match status {
        HostStatus::Unreachable { error, .. }
        | HostStatus::Unauthorized { error }
        | HostStatus::KeychainUnavailable { error }
        | HostStatus::Invalid { error } => Some(error.code),
        HostStatus::CredentialsLost => Some(ErrorCode::CredentialsLost),
        HostStatus::HostKeyUntrusted { .. } => Some(ErrorCode::HostKeyUntrusted),
        HostStatus::HostKeyMismatch { .. } => Some(ErrorCode::HostKeyChanged),
        _ => None,
    }
}

fn cluster_error(status: &ClusterStatus) -> Option<ErrorCode> {
    match status {
        ClusterStatus::Unreachable { error, .. }
        | ClusterStatus::Unauthorized { error }
        | ClusterStatus::KeychainUnavailable { error }
        | ClusterStatus::Invalid { error } => Some(error.code),
        ClusterStatus::CredentialsLost => Some(ErrorCode::CredentialsLost),
        _ => None,
    }
}

fn bundle_type(installer: Installer) -> &'static str {
    match installer {
        Installer::AppImage => "appimage",
        Installer::Deb => "deb",
        Installer::Rpm => "rpm",
        Installer::Msi => "msi",
        Installer::Nsis => "nsis",
        Installer::App => "app",
        Installer::Unknown => "unknown",
    }
}

fn hosts(state: &AppState) -> AppResult<Vec<HostFacts>> {
    Ok(state
        .hosts
        .store
        .list()?
        .into_iter()
        .map(|host| {
            let status = state.hosts.registry.status(&host.id);
            let (connection, address, bastions) = match &host.connection {
                HostConnection::Local { .. } => ("local", None, 0),
                HostConnection::Tcp { address } => ("tcp", Some(address.clone()), 0),
                HostConnection::Tls { address } => ("tls", Some(address.clone()), 0),
                HostConnection::Ssh {
                    host: address,
                    port,
                    bastions,
                    ..
                } => (
                    "ssh",
                    Some(homeport_core::host::ssh_address(address, *port)),
                    bastions.len(),
                ),
            };
            let (engine_version, api_version) = match &status {
                HostStatus::Connected {
                    engine_version,
                    api_version,
                    ..
                } => (Some(engine_version.clone()), Some(api_version.clone())),
                _ => (None, None),
            };
            let engine = if status.podman_version().is_some() {
                "podman".to_owned()
            } else {
                host.engine.as_str().to_owned()
            };
            HostFacts {
                name: host.name,
                address,
                connection: connection.to_owned(),
                bastions,
                engine,
                engine_version: status
                    .podman_version()
                    .map(str::to_owned)
                    .or(engine_version),
                api_version,
                status: status_name(&status),
                error: host_error(&status),
            }
        })
        .collect())
}

fn clusters(state: &AppState) -> AppResult<Vec<ClusterFacts>> {
    Ok(state
        .clusters
        .store
        .list()?
        .into_iter()
        .map(|cluster| {
            let status = state.clusters.registry.status(&cluster.id);
            let version = match &status {
                ClusterStatus::Connected { server_version, .. } if !server_version.is_empty() => {
                    Some(server_version.clone())
                }
                _ => None,
            };
            ClusterFacts {
                address: cluster.address().to_owned(),
                kind: cluster.kind.as_str().to_owned(),
                name: cluster.name,
                version,
                status: status_name(&status),
                error: cluster_error(&status),
            }
        })
        .collect())
}

fn stack_counts(state: &AppState) -> AppResult<StackCounts> {
    let count = |table: &str| -> AppResult<usize> {
        state
            .db
            .with(|c| {
                c.query_row(&format!("SELECT count(*) FROM {table}"), [], |r| {
                    r.get::<_, i64>(0)
                })
            })
            .map(|n| usize::try_from(n).unwrap_or(0))
    };
    Ok(StackCounts {
        managed: count("stacks")?,
        linked: count("linked_stacks")?,
        synced: count("synced_stacks")?,
    })
}

/// The log files, oldest first, each as `(file name, text)`.
fn logs(dir: &Path) -> Vec<(String, String)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "log"))
        .collect();
    files.sort();
    files
        .into_iter()
        .filter_map(|path| {
            let name = path.file_name()?.to_string_lossy().into_owned();
            // A log cut in the middle of a character still reads.
            let text = String::from_utf8_lossy(&std::fs::read(&path).ok()?).into_owned();
            Some((name, text))
        })
        .collect()
}

fn gather(
    state: &AppState,
    installer: Installer,
    log_dir: &Path,
    include_names: bool,
) -> AppResult<Bundle> {
    let inputs = Inputs {
        app: AppFacts {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            bundle: bundle_type(installer).to_owned(),
            os: std::env::consts::OS.to_owned(),
            arch: std::env::consts::ARCH.to_owned(),
            schema_version: state.db.schema_version()?,
            keychain: state.secrets.is_available(),
        },
        settings: serde_json::to_value(state.settings.get()?)?,
        hosts: hosts(state)?,
        clusters: clusters(state)?,
        stacks: stack_counts(state)?,
    };
    let (summary, aliases) = summary(inputs, include_names);
    Ok(build(
        &serde_json::to_string_pretty(&summary)?,
        logs(log_dir),
        &aliases,
    ))
}

/// The bundle's files: the summary first, then each log without its secrets.
fn build(summary: &str, logs: Vec<(String, String)>, aliases: &Aliases) -> Bundle {
    let mut files = vec![("summary.json".to_owned(), summary.as_bytes().to_vec())];
    for (name, text) in logs {
        files.push((format!("logs/{name}"), redact(&text, aliases).into_bytes()));
    }
    Bundle(files)
}

fn write_zip(path: &Path, bundle: &Bundle) -> AppResult<()> {
    let failed = |e: zip::result::ZipError| AppError::from(std::io::Error::other(e.to_string()));
    let file = std::fs::File::create(path).map_err(|e| local_file(path, e))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (name, bytes) in &bundle.0 {
        zip.start_file(name.as_str(), options).map_err(failed)?;
        zip.write_all(bytes).map_err(|e| local_file(path, e))?;
    }
    zip.finish().map_err(failed)?;
    Ok(())
}

/// What the bundle would hold: its files and `summary.json`.
///
/// `installer` is what this copy of Homeport was installed from, which the
/// updater layer (#255) detects; `log_dir` is where the logging layer (#254)
/// writes the log files.
pub fn diagnostics_preview(
    state: &AppState,
    installer: Installer,
    log_dir: &Path,
    include_names: bool,
) -> AppResult<DiagnosticsPreview> {
    let bundle = gather(state, installer, log_dir, include_names)?;
    Ok(DiagnosticsPreview {
        summary: String::from_utf8_lossy(&bundle.0[0].1).into_owned(),
        files: bundle
            .0
            .iter()
            .map(|(name, bytes)| DiagnosticsFile {
                name: name.clone(),
                size: u32::try_from(bytes.len()).unwrap_or(u32::MAX),
            })
            .collect(),
    })
}

/// Writes the bundle to `path`, which the user picked in a save dialog.
/// `installer` and `log_dir` are those of [`diagnostics_preview`].
pub fn diagnostics_export(
    state: &AppState,
    installer: Installer,
    log_dir: &Path,
    path: String,
    include_names: bool,
) -> AppResult<()> {
    let bundle = gather(state, installer, log_dir, include_names)?;
    write_zip(Path::new(&path), &bundle)
}

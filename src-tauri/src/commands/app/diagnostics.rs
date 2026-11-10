//! The diagnostics bundle for bug reports (`homeport-diagnostics`): a zip
//! with a summary of what Homeport runs as and against, and its logs with
//! secrets removed. It is written where the user chooses and sent nowhere;
//! sealed secrets are never read for it.

use std::{
    io::Write,
    path::{Path, PathBuf},
};

use serde::Serialize;
use tauri::{AppHandle, Manager, State};

use homeport_clusters::registry::ClusterStatus;
use homeport_core::host::HostConnection;
use homeport_diagnostics::{
    redact, summary, Aliases, AppFacts, ClusterFacts, HostFacts, Inputs, StackCounts,
};
use homeport_hosts::registry::HostStatus;

use crate::{
    app::AppState,
    error::{self, AppError, AppResult, ErrorCode},
};

/// One file of the bundle, as the preview lists it.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsFile {
    /// Its path in the zip, such as `logs/Homeport.log`.
    name: String,
    /// Its size once secrets are removed, in bytes.
    size: u32,
}

/// What the bundle would hold, for the user to read before saving it.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsPreview {
    files: Vec<DiagnosticsFile>,
    /// `summary.json`, as it would be written.
    summary: String,
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

fn bundle_type() -> &'static str {
    use tauri::utils::{config::BundleType, platform::bundle_type};
    match bundle_type() {
        Some(BundleType::AppImage) => "appimage",
        Some(BundleType::Deb) => "deb",
        Some(BundleType::Rpm) => "rpm",
        Some(BundleType::Msi) => "msi",
        Some(BundleType::Nsis) => "nsis",
        // macOS reports `App` for every build, release or not.
        Some(BundleType::App) if !cfg!(debug_assertions) => "app",
        _ => "unknown",
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

fn gather(app: &AppHandle, state: &AppState, include_names: bool) -> AppResult<Bundle> {
    let info = app.package_info();
    let inputs = Inputs {
        app: AppFacts {
            version: info.version.to_string(),
            bundle: bundle_type().to_owned(),
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
    let log_dir = app.path().app_log_dir().map_err(error::window)?;
    Ok(build(
        &serde_json::to_string_pretty(&summary)?,
        logs(&log_dir),
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
    let file = std::fs::File::create(path).map_err(|e| error::local_file(path, e))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (name, bytes) in &bundle.0 {
        zip.start_file(name.as_str(), options).map_err(failed)?;
        zip.write_all(bytes)
            .map_err(|e| error::local_file(path, e))?;
    }
    zip.finish().map_err(failed)?;
    Ok(())
}

/// What the bundle would hold: its files and `summary.json`.
#[tauri::command]
#[specta::specta]
pub fn diagnostics_preview(
    app: AppHandle,
    state: State<'_, AppState>,
    include_names: bool,
) -> AppResult<DiagnosticsPreview> {
    let bundle = gather(&app, &state, include_names)?;
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
#[tauri::command]
#[specta::specta]
pub fn diagnostics_export(
    app: AppHandle,
    state: State<'_, AppState>,
    path: String,
    include_names: bool,
) -> AppResult<()> {
    let bundle = gather(&app, &state, include_names)?;
    write_zip(Path::new(&path), &bundle)
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn the_zip_holds_the_summary_and_logs_without_their_secrets() {
        let mut aliases = Aliases::default();
        aliases.add("prod", "host-1");
        let bundle = build(
            r#"{"app":{"version":"1.0.0"}}"#,
            vec![(
                "Homeport.log".to_owned(),
                "[WARN] host prod: Authorization: Bearer abcdefgh12345678 refused\n".to_owned(),
            )],
            &aliases,
        );
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("homeport-diagnostics.zip");
        write_zip(&path, &bundle).unwrap();

        let mut zip = zip::ZipArchive::new(std::fs::File::open(&path).unwrap()).unwrap();
        let names: Vec<String> = zip.file_names().map(str::to_owned).collect();
        assert_eq!(names.len(), 2, "{names:?}");
        let mut read = |name: &str| {
            let mut text = String::new();
            zip.by_name(name)
                .unwrap()
                .read_to_string(&mut text)
                .unwrap();
            text
        };
        assert_eq!(read("summary.json"), r#"{"app":{"version":"1.0.0"}}"#);
        assert_eq!(
            read("logs/Homeport.log"),
            "[WARN] host host-1: Authorization: [redacted] refused\n"
        );
    }

    #[test]
    fn a_missing_log_folder_leaves_the_summary_alone() {
        assert!(logs(Path::new("/nonexistent/homeport/logs")).is_empty());
    }
}

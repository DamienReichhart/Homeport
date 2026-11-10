//! Podman machines: the VMs `podman machine` runs the Engine in on macOS,
//! each with its API socket in a per-user, per-machine place.

use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use serde::Deserialize;

/// How long the `podman` CLI may take to describe its machines.
const CLI_TIMEOUT: Duration = Duration::from_secs(2);

/// A running machine's API socket, labelled for **Add host**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Machine {
    pub label: String,
    pub socket_path: String,
}

fn label(name: &str) -> String {
    format!("Podman ({name})")
}

/// One machine of `podman machine list --format json`.
#[derive(Deserialize)]
struct Listed {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Running", default)]
    running: bool,
}

/// One machine of `podman machine inspect`.
#[derive(Deserialize)]
struct Inspected {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ConnectionInfo")]
    connection_info: Option<ConnectionInfo>,
}

#[derive(Deserialize)]
struct ConnectionInfo {
    #[serde(rename = "PodmanSocket")]
    podman_socket: Option<SocketPath>,
}

#[derive(Deserialize)]
struct SocketPath {
    #[serde(rename = "Path")]
    path: String,
}

/// The running machines' names, from `podman machine list --format json`.
fn running_names(list_json: &str) -> Vec<String> {
    serde_json::from_str::<Vec<Listed>>(list_json)
        .unwrap_or_default()
        .into_iter()
        .filter(|m| m.running)
        // Older Podmans mark the default machine with a trailing `*`.
        .map(|m| m.name.trim_end_matches('*').to_owned())
        .collect()
}

/// The machines' sockets, from `podman machine inspect <names…>`.
fn sockets(inspect_json: &str) -> Vec<Machine> {
    serde_json::from_str::<Vec<Inspected>>(inspect_json)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|m| {
            let path = m.connection_info?.podman_socket?.path;
            (!path.is_empty()).then(|| Machine {
                label: label(&m.name),
                socket_path: path,
            })
        })
        .collect()
}

/// Asks the `podman` CLI for its running machines' sockets; `None` when it
/// isn't installed, fails or is too slow.
pub(crate) async fn from_cli() -> Option<Vec<Machine>> {
    let program = homeport_core::programs::find("podman")?;
    let run = |args: Vec<String>| {
        let program = program.clone();
        async move {
            let out = tokio::time::timeout(
                CLI_TIMEOUT,
                tokio::process::Command::new(program)
                    .args(args)
                    .env("PATH", homeport_core::programs::search_path())
                    .kill_on_drop(true)
                    .output(),
            )
            .await
            .ok()?
            .ok()?;
            out.status
                .success()
                .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
        }
    };
    let list = run(vec![
        "machine".into(),
        "list".into(),
        "--format".into(),
        "json".into(),
    ])
    .await?;
    let names = running_names(&list);
    if names.is_empty() {
        return Some(vec![]);
    }
    let mut args = vec!["machine".to_owned(), "inspect".to_owned()];
    args.extend(names);
    Some(sockets(&run(args).await?))
}

/// Where machines' sockets usually are, for when the CLI can't be asked:
/// `$TMPDIR/podman/<machine>-api.sock` (Podman 5, applehv and libkrun), and
/// `~/.local/share/containers/podman/machine/**/podman.sock` (Podman 4, QEMU).
/// Stopped machines' sockets don't answer, so they're filtered out later.
pub(crate) fn from_paths(tmp: Option<&Path>, home: Option<&Path>) -> Vec<Machine> {
    let mut found = Vec::new();
    if let Some(tmp) = tmp {
        for path in files_in(&tmp.join("podman")) {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if let Some(machine) = name.strip_suffix("-api.sock") {
                found.push(Machine {
                    label: label(machine),
                    socket_path: path.to_string_lossy().into_owned(),
                });
            }
        }
    }
    if let Some(home) = home {
        let root = home.join(".local/share/containers/podman/machine");
        // machine/<vm type>/podman.sock, or machine/<name>/podman.sock.
        for dir in dirs_in(&root) {
            let direct = dir.join("podman.sock");
            let dir_name = dir.file_name().and_then(|n| n.to_str()).unwrap_or_default();
            if direct.exists() {
                let name = if dir_name == "qemu" {
                    "podman-machine-default"
                } else {
                    dir_name
                };
                found.push(Machine {
                    label: label(name),
                    socket_path: direct.to_string_lossy().into_owned(),
                });
            }
            for inner in dirs_in(&dir) {
                let socket = inner.join("podman.sock");
                if socket.exists() {
                    let name = inner
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or_default();
                    found.push(Machine {
                        label: label(name),
                        socket_path: socket.to_string_lossy().into_owned(),
                    });
                }
            }
        }
    }
    found
}

fn entries(dir: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .collect();
    paths.sort();
    paths
}

fn files_in(dir: &Path) -> Vec<PathBuf> {
    entries(dir).into_iter().filter(|p| !p.is_dir()).collect()
}

fn dirs_in(dir: &Path) -> Vec<PathBuf> {
    entries(dir).into_iter().filter(|p| p.is_dir()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn running_machines_are_read_from_the_cli_list() {
        let list = r#"[
            {"Name":"podman-machine-default*","Default":true,"Running":true,"VMType":"applehv"},
            {"Name":"build","Default":false,"Running":true},
            {"Name":"old","Default":false,"Running":false}
        ]"#;
        assert_eq!(running_names(list), ["podman-machine-default", "build"]);
        assert!(running_names("not json").is_empty());
        assert!(running_names("[]").is_empty());
    }

    #[test]
    fn sockets_are_read_from_the_cli_inspect() {
        let inspect = r#"[
            {"Name":"podman-machine-default","ConnectionInfo":{"PodmanSocket":{"Path":"/var/folders/x/T/podman/podman-machine-default-api.sock"},"PodmanPipe":null},"State":"running"},
            {"Name":"build","ConnectionInfo":{"PodmanSocket":{"Path":"/var/folders/x/T/podman/build-api.sock"}}},
            {"Name":"pipe-only","ConnectionInfo":{"PodmanSocket":null,"PodmanPipe":{"Path":"//./pipe/x"}}}
        ]"#;
        assert_eq!(
            sockets(inspect),
            [
                Machine {
                    label: "Podman (podman-machine-default)".into(),
                    socket_path: "/var/folders/x/T/podman/podman-machine-default-api.sock".into(),
                },
                Machine {
                    label: "Podman (build)".into(),
                    socket_path: "/var/folders/x/T/podman/build-api.sock".into(),
                },
            ]
        );
    }

    #[test]
    fn usual_socket_paths_are_found_without_the_cli() {
        let tmp = tempfile::tempdir().unwrap();
        let home = tempfile::tempdir().unwrap();
        let p5 = tmp.path().join("podman");
        std::fs::create_dir_all(&p5).unwrap();
        for f in [
            "podman-machine-default-api.sock",
            "build-api.sock",
            "gvproxy.sock",
        ] {
            std::fs::write(p5.join(f), "").unwrap();
        }
        let p4 = home.path().join(".local/share/containers/podman/machine");
        std::fs::create_dir_all(p4.join("qemu")).unwrap();
        std::fs::write(p4.join("qemu/podman.sock"), "").unwrap();
        std::fs::create_dir_all(p4.join("applehv/dev")).unwrap();
        std::fs::write(p4.join("applehv/dev/podman.sock"), "").unwrap();

        let labels: Vec<_> = from_paths(Some(tmp.path()), Some(home.path()))
            .into_iter()
            .map(|m| m.label)
            .collect();
        assert_eq!(
            labels,
            [
                "Podman (build)",
                "Podman (podman-machine-default)",
                "Podman (dev)",
                "Podman (podman-machine-default)",
            ]
        );
        assert!(from_paths(None, None).is_empty());
    }
}

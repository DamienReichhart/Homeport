//! Finds Docker-compatible Engines running on this machine.

use std::{path::Path, time::Duration};

use serde::Serialize;

use homeport_transport as transport;

use crate::HostConnection;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DetectedEngine {
    /// The product that usually owns this socket, e.g. "Colima".
    pub label: String,
    pub socket_path: String,
    pub engine_version: String,
}

/// Well-known Engine sockets, most common first.
pub fn candidates(home: Option<&Path>, runtime_dir: Option<&Path>) -> Vec<(&'static str, String)> {
    if cfg!(windows) {
        return vec![
            ("Docker Desktop", "//./pipe/docker_engine".into()),
            ("Podman", "//./pipe/podman-machine-default".into()),
        ];
    }
    let mut found = vec![("Docker Engine", "/var/run/docker.sock".to_string())];
    if let Some(run) = runtime_dir {
        found.push(("Docker Engine (rootless)", path(run, "docker.sock")));
        found.push(("Podman", path(run, "podman/podman.sock")));
    }
    if let Some(home) = home {
        found.extend([
            ("Docker Desktop", path(home, ".docker/desktop/docker.sock")),
            ("Docker Desktop", path(home, ".docker/run/docker.sock")),
            ("Colima", path(home, ".colima/default/docker.sock")),
            ("OrbStack", path(home, ".orbstack/run/docker.sock")),
            ("Rancher Desktop", path(home, ".rd/docker.sock")),
        ]);
    }
    found.push(("Podman", "/run/podman/podman.sock".into()));
    found
}

fn path(base: &Path, rel: &str) -> String {
    base.join(rel).to_string_lossy().into_owned()
}

/// Probes every candidate socket and returns the Engines that answer.
/// Sockets that are symlinks to one already found are skipped.
pub async fn detect() -> Vec<DetectedEngine> {
    let home = std::env::var_os("HOME").map(std::path::PathBuf::from);
    let runtime = std::env::var_os("XDG_RUNTIME_DIR").map(std::path::PathBuf::from);
    let mut seen = Vec::new();
    let mut engines = Vec::new();
    for (label, socket_path) in candidates(home.as_deref(), runtime.as_deref()) {
        if cfg!(unix) {
            let Ok(real) = std::fs::canonicalize(&socket_path) else {
                continue;
            };
            if seen.contains(&real) {
                continue;
            }
            seen.push(real);
        }
        let connection = HostConnection::Local {
            socket_path: socket_path.clone(),
        };
        let probe = async {
            let connected = transport::connect(label, &connection, None, None)
                .await
                .ok()?;
            connected.docker.version().await.ok()?.version
        };
        if let Ok(Some(engine_version)) = tokio::time::timeout(Duration::from_secs(3), probe).await
        {
            engines.push(DetectedEngine {
                label: label.into(),
                socket_path,
                engine_version,
            });
        }
    }
    engines
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn candidates_cover_common_engines() {
        let list = candidates(
            Some(Path::new("/home/u")),
            Some(Path::new("/run/user/1000")),
        );
        let has = |p: &str| list.iter().any(|(_, path)| path == p);
        assert_eq!(list[0].1, "/var/run/docker.sock");
        assert!(has("/run/user/1000/docker.sock"));
        assert!(has("/home/u/.colima/default/docker.sock"));
        assert!(has("/home/u/.orbstack/run/docker.sock"));
        assert!(has("/run/user/1000/podman/podman.sock"));
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn finds_the_local_engine() {
        let engines = detect().await;
        assert!(!engines.is_empty());
        assert!(!engines[0].engine_version.is_empty());
    }
}

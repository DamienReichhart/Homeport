//! containerd's sockets on this machine: its own service's, k3s's, and a
//! rootless one, which only answers inside RootlessKit's namespace.

use std::path::Path;

use homeport_containerd::Containerd;
use homeport_core::{
    error::{AppError, ErrorCode},
    host::EngineKind,
};
use homeport_transport as transport;

use super::{path, DetectedEngine};
use crate::{messages as msg, HostConnection};

/// containerd's well-known sockets, with what to call each.
pub(super) fn candidates(runtime_dir: Option<&Path>) -> Vec<(&'static str, String)> {
    if !cfg!(target_os = "linux") {
        return vec![];
    }
    let mut found = vec![
        ("containerd", "/run/containerd/containerd.sock".to_owned()),
        ("k3s", "/run/k3s/containerd/containerd.sock".to_owned()),
    ];
    if let Some(run) = runtime_dir {
        found.push((
            "containerd (rootless)",
            path(run, "containerd/containerd.sock"),
        ));
    }
    found
}

/// Whether rootless containerd runs for this user: `containerd-rootless.sh`
/// keeps RootlessKit's state beside the socket it can't share.
fn rootless_running(runtime_dir: &Path) -> bool {
    runtime_dir.join("containerd-rootless/child_pid").exists()
}

/// The containerd at `socket_path`, if it answers; or why it can't be used,
/// when that's something the user can fix.
pub(super) async fn probe(
    label: &str,
    socket_path: &str,
    runtime_dir: Option<&Path>,
) -> Option<DetectedEngine> {
    let detected = |engine_version: String, problem: Option<AppError>| DetectedEngine {
        label: label.to_owned(),
        socket_path: socket_path.to_owned(),
        engine: EngineKind::Containerd,
        engine_version,
        problem,
    };
    if !Path::new(socket_path).exists() {
        let rootless = runtime_dir.is_some_and(|run| {
            socket_path.starts_with(&*run.to_string_lossy()) && rootless_running(run)
        });
        return rootless.then(|| {
            detected(
                String::new(),
                Some(
                    AppError::new(
                        ErrorCode::EngineUnsupported,
                        msg::DETECT_CONTAINERD_ROOTLESS.with("path", socket_path),
                    )
                    .with_hint(msg::DETECT_CONTAINERD_ROOTLESS_HINT),
                ),
            )
        });
    }
    let connection = HostConnection::Local {
        socket_path: socket_path.to_owned(),
    };
    match transport::dial_engine(label, &connection, None, None, EngineKind::Containerd).await {
        Ok(dialer) => {
            let client = Containerd::connect(label, dialer).await.ok()?;
            Some(detected(client.version().to_owned(), None))
        }
        // A socket the user may not use yet says how to fix it.
        Err(
            transport::ConnectError::Unreachable(error)
            | transport::ConnectError::Unauthorized(error),
        ) if error.is(ErrorCode::PermissionDenied) => Some(detected(String::new(), Some(error))),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn containerds_sockets_are_candidates() {
        let list = candidates(Some(Path::new("/run/user/1000")));
        let paths: Vec<&str> = list.iter().map(|(_, p)| p.as_str()).collect();
        assert_eq!(
            paths,
            [
                "/run/containerd/containerd.sock",
                "/run/k3s/containerd/containerd.sock",
                "/run/user/1000/containerd/containerd.sock"
            ]
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn rootless_containerd_is_explained_instead_of_shown() {
        let run = tempfile::tempdir().unwrap();
        std::fs::create_dir(run.path().join("containerd-rootless")).unwrap();
        std::fs::write(run.path().join("containerd-rootless/child_pid"), "42").unwrap();
        let socket = path(run.path(), "containerd/containerd.sock");
        let found = probe("containerd (rootless)", &socket, Some(run.path()))
            .await
            .expect("rootless containerd runs");
        let problem = found.problem.unwrap();
        assert!(problem.message.contains("RootlessKit"), "{problem:?}");
        assert!(found.engine_version.is_empty());

        // Without RootlessKit's state, a missing socket is just not there.
        let other = tempfile::tempdir().unwrap();
        let socket = path(other.path(), "containerd/containerd.sock");
        assert!(probe("containerd (rootless)", &socket, Some(other.path()))
            .await
            .is_none());
    }
}

//! Incus's sockets on this machine: the admin one, for the `incus-admin`
//! group, and the restricted one `incus-user` serves, for the `incus` group,
//! which only reaches the user's own project.

use std::path::Path;

use homeport_core::{error::ErrorCode, host::EngineKind};
use homeport_incus::Incus;
use homeport_transport as transport;

use super::DetectedEngine;
use crate::HostConnection;

/// Incus's well-known sockets, with what to call each.
pub(super) fn candidates() -> Vec<(&'static str, String)> {
    if !cfg!(target_os = "linux") {
        return vec![];
    }
    vec![
        ("Incus", "/var/lib/incus/unix.socket".to_owned()),
        (
            "Incus (restricted to your project)",
            "/var/lib/incus/unix.socket.user".to_owned(),
        ),
    ]
}

/// The Incus at `socket_path`, if it answers; or why it can't be used,
/// when that's something the user can fix.
pub(super) async fn probe(label: &str, socket_path: &str) -> Option<DetectedEngine> {
    if !Path::new(socket_path).exists() {
        return None;
    }
    let detected = |engine_version: String, problem| DetectedEngine {
        label: label.to_owned(),
        socket_path: socket_path.to_owned(),
        engine: EngineKind::Incus,
        engine_version,
        problem,
    };
    let connection = HostConnection::Local {
        socket_path: socket_path.to_owned(),
    };
    match transport::dial_engine(label, &connection, None, None, EngineKind::Incus).await {
        Ok(dialer) => {
            let client = Incus::connect(label, dialer).await.ok()?;
            Some(detected(client.server().version.clone(), None))
        }
        // A socket the user may not use yet says how to fix it.
        Err(
            transport::ConnectError::Unreachable(error)
            | transport::ConnectError::Unauthorized(error),
        ) if error.is(ErrorCode::PermissionDenied) => Some(detected(String::new(), Some(error))),
        Err(_) => None,
    }
}

// Incus's sockets are only looked for on Linux, where it runs.
#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn both_incus_sockets_are_candidates_and_the_restricted_one_says_so() {
        let list = candidates();
        assert_eq!(list[0].1, "/var/lib/incus/unix.socket");
        assert_eq!(list[1].1, "/var/lib/incus/unix.socket.user");
        assert!(list[1].0.contains("your project"));
    }

    #[tokio::test]
    async fn a_missing_socket_is_not_shown() {
        let dir = tempfile::tempdir().unwrap();
        let socket = dir.path().join("unix.socket");
        assert!(probe("Incus", socket.to_str().unwrap()).await.is_none());
    }
}

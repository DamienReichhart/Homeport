//! A local Incus socket that didn't answer: how to start Incus, or let this
//! user use its socket.

use std::{error::Error as StdError, io};

use homeport_core::error::{AppError, ErrorCode};

use super::io_cause;
use crate::messages as msg;

/// The restricted socket `incus-user` serves, for the `incus` group: it
/// only reaches the user's own project.
pub fn is_restricted_incus_socket(socket_path: &str) -> bool {
    socket_path.ends_with("unix.socket.user")
}

/// Why the Incus socket at `socket_path` didn't answer.
pub fn local_incus(socket_path: &str, error: &(dyn StdError + 'static)) -> AppError {
    let kind = io_cause(error).map(io::Error::kind);
    let path = socket_path;
    let restricted = is_restricted_incus_socket(path);
    let classified = match kind {
        Some(io::ErrorKind::PermissionDenied) => AppError::new(
            ErrorCode::PermissionDenied,
            msg::LOCAL_INCUS_PERMISSION_DENIED.with("path", path),
        )
        .with_hint(if restricted {
            msg::INCUS_USER_GROUP_HINT
        } else {
            msg::INCUS_ADMIN_GROUP_HINT
        })
        .with_command(incus_group_command(restricted, "\"$USER\"")),
        Some(io::ErrorKind::TimedOut) => AppError::new(
            ErrorCode::Timeout,
            msg::LOCAL_INCUS_TIMEOUT.with("path", path),
        )
        .with_hint(msg::LOCAL_TIMEOUT_HINT),
        Some(io::ErrorKind::NotFound | io::ErrorKind::ConnectionRefused) | None => {
            let error = AppError::new(
                ErrorCode::EngineNotRunning,
                msg::LOCAL_INCUS_NOT_RUNNING.with("path", path),
            );
            if restricted {
                error
                    .with_hint(msg::LOCAL_INCUS_USER_NOT_RUNNING_HINT)
                    .with_command("sudo systemctl enable --now incus-user.socket")
            } else {
                error
                    .with_hint(msg::LOCAL_INCUS_NOT_RUNNING_HINT)
                    .with_command("sudo systemctl enable --now incus.socket incus.service")
            }
        }
        Some(_) => AppError::new(
            ErrorCode::EngineNotRunning,
            msg::LOCAL_INCUS_UNREACHABLE.with("path", path),
        )
        .with_hint(msg::LOCAL_INCUS_UNREACHABLE_HINT),
    };
    classified.with_cause(error)
}

/// Adds `user` to the group that may use Incus's socket.
pub(crate) fn incus_group_command(restricted: bool, user: &str) -> String {
    let group = if restricted { "incus" } else { "incus-admin" };
    format!("sudo usermod -aG {group} {user}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stopped_incus_says_which_service_to_start() {
        let missing = io::Error::from(io::ErrorKind::NotFound);
        let admin = local_incus("/var/lib/incus/unix.socket", &missing);
        assert_eq!(admin.code, ErrorCode::EngineNotRunning);
        assert!(admin.message.contains("Incus"), "{admin:?}");
        assert_eq!(
            admin.hint.unwrap().commands,
            ["sudo systemctl enable --now incus.socket incus.service"]
        );
        let user = local_incus("/var/lib/incus/unix.socket.user", &missing);
        assert_eq!(
            user.hint.unwrap().commands,
            ["sudo systemctl enable --now incus-user.socket"]
        );
    }

    #[test]
    fn a_refused_socket_names_its_group() {
        let denied = io::Error::from(io::ErrorKind::PermissionDenied);
        let admin = local_incus("/var/lib/incus/unix.socket", &denied);
        assert_eq!(admin.code, ErrorCode::PermissionDenied);
        assert!(admin.hint.unwrap().commands[0].contains("incus-admin"));
        let user = local_incus("/var/lib/incus/unix.socket.user", &denied);
        let hint = user.hint.unwrap();
        assert!(hint.commands[0].contains("-aG incus "), "{hint:?}");
        assert!(hint.text.contains("own project"));
    }
}

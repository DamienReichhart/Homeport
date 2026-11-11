//! Starting, stopping and restarting a unit's service with `systemctl`,
//! where the Host's connection runs commands: over SSH, or on this
//! computer for a local Linux socket (ADR 0019). Never with `sudo`.

use std::time::Duration;

use serde::Deserialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::{shell_quote, CommandOutput};

use super::{Podman, UnitScope};
use crate::messages as msg;

/// How long a service may take to start: Quadlet pulls a missing image first.
const TIMEOUT: Duration = Duration::from_secs(300);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum UnitAction {
    Start,
    Stop,
    Restart,
}

impl UnitAction {
    /// The `systemctl` subcommand.
    fn verb(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Stop => "stop",
            Self::Restart => "restart",
        }
    }

    /// What to say when systemd refuses this action.
    fn failed(self) -> homeport_core::error::Message {
        match self {
            Self::Start => msg::QUADLET_START_FAILED,
            Self::Stop => msg::QUADLET_STOP_FAILED,
            Self::Restart => msg::QUADLET_RESTART_FAILED,
        }
    }
}

/// A systemd service name: systemd's characters, ending in `.service`.
fn check_unit(unit: &str) -> AppResult<()> {
    let valid = unit.ends_with(".service")
        && !unit.starts_with('-')
        && unit
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, ':' | '_' | '.' | '@' | '-' | '\\'));
    if valid {
        Ok(())
    } else {
        Err(AppError::validation(
            msg::QUADLET_UNIT_NAME.with("name", unit),
        ))
    }
}

/// The exact command Homeport runs for `action` on `unit`.
pub fn unit_command(unit: &str, scope: UnitScope, action: UnitAction) -> AppResult<String> {
    check_unit(unit)?;
    let user = match scope {
        UnitScope::User => " --user",
        UnitScope::System => "",
    };
    Ok(format!(
        "systemctl{user} {} {}",
        action.verb(),
        shell_quote(unit)
    ))
}

/// What systemd's refusal means, from what `systemctl` printed.
pub(super) fn refused(unit: &str, action: UnitAction, out: &CommandOutput) -> AppError {
    let text = out.stderr.to_lowercase();
    let detail = format!(
        "exit status {}\n{}",
        out.status
            .map_or_else(|| "unknown".into(), |s| s.to_string()),
        out.stderr.trim()
    );
    let error = if text.contains("failed to connect to bus")
        || text.contains("failed to connect to user scope bus")
        || text.contains("$dbus_session_bus_address")
        || text.contains("xdg_runtime_dir")
    {
        AppError::new(ErrorCode::UserUnitsNeedLinger, msg::QUADLET_NO_USER_SYSTEMD)
            .with_hint(msg::QUADLET_NO_USER_SYSTEMD_HINT)
            .with_command("loginctl enable-linger \"$USER\"")
    } else if text.contains("interactive authentication required") || text.contains("access denied")
    {
        AppError::new(
            ErrorCode::SystemdRefused,
            msg::QUADLET_NOT_ALLOWED.with("unit", unit),
        )
        .with_hint(msg::QUADLET_NOT_ALLOWED_HINT)
    } else if text.contains("not found") || text.contains("not loaded") {
        AppError::new(
            ErrorCode::NotFound,
            msg::QUADLET_UNIT_NOT_LOADED.with("unit", unit),
        )
        .with_hint(msg::QUADLET_UNIT_NOT_LOADED_HINT)
    } else {
        AppError::new(
            ErrorCode::SystemdRefused,
            action.failed().with("unit", unit),
        )
        .with_hint(msg::QUADLET_ACTION_FAILED_HINT)
    };
    error.with_detail(detail)
}

/// Starts, stops or restarts the service `unit` (`web.service`) of a unit
/// in `scope`, and waits until systemd is done.
pub async fn act(
    podman: &Podman,
    unit: &str,
    scope: UnitScope,
    action: UnitAction,
) -> AppResult<()> {
    if !podman.runs_commands() {
        return Err(
            AppError::new(ErrorCode::EngineUnsupported, msg::QUADLET_NO_COMMANDS)
                .with_hint(msg::QUADLET_NO_COMMANDS_HINT),
        );
    }
    let command = unit_command(unit, scope, action)?;
    let out = podman.run(command.clone(), TIMEOUT).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::TimedOut {
            AppError::new(ErrorCode::Timeout, msg::QUADLET_TIMEOUT.with("unit", unit))
                .with_hint(msg::QUADLET_TIMEOUT_HINT)
        } else if let Some(classified) = homeport_core::error::reach::classified_cause(&e) {
            classified.clone()
        } else {
            AppError::new(
                ErrorCode::SshFailed,
                msg::QUADLET_COMMAND_FAILED.with("unit", unit),
            )
            .with_cause(&e)
        }
    })?;
    if out.success() {
        Ok(())
    } else {
        log::info!("`{command}` failed: {}", out.stderr.trim());
        Err(refused(unit, action, &out))
    }
}

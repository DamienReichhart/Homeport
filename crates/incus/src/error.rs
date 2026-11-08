//! Incus's failures, as errors the user can act on. Incus answers an error
//! with an HTTP status and a sentence of its own; the status decides the
//! code, a few well-known sentences a more precise one, and the sentence
//! itself goes to Details.

use homeport_core::error::{AppError, ErrorCode};

use crate::messages as msg;

/// The failure Incus reported with `status` and its own `message`.
pub(crate) fn answer(status: u16, message: &str) -> AppError {
    let detail = format!("HTTP {status}: {message}");
    if let Some(known) = known(message) {
        return known.with_detail(detail);
    }
    let error = match status {
        404 => AppError::new(ErrorCode::NotFound, msg::NOT_FOUND).with_hint(msg::DETAILS_HINT),
        409 if mentions(message, &["already exists"]) => {
            AppError::new(ErrorCode::NameInUse, msg::ALREADY_EXISTS)
                .with_hint(msg::ALREADY_EXISTS_HINT)
        }
        409 => AppError::new(ErrorCode::Conflict, msg::CONFLICT).with_hint(msg::DETAILS_HINT),
        412 => AppError::new(ErrorCode::ChangedSinceLoaded, msg::CHANGED_SINCE_LOADED)
            .with_hint(msg::CHANGED_SINCE_LOADED_HINT),
        401 | 403 => {
            AppError::new(ErrorCode::Forbidden, msg::FORBIDDEN).with_hint(msg::FORBIDDEN_HINT)
        }
        400 => AppError::new(ErrorCode::BadRequest, msg::INVALID).with_hint(msg::INVALID_HINT),
        _ => AppError::new(ErrorCode::EngineError, msg::FAILED).with_hint(msg::DETAILS_HINT),
    };
    error.with_detail(detail)
}

/// The failure of an operation, which carries only Incus's sentence.
pub(crate) fn operation(message: &str) -> AppError {
    let status = if mentions(message, &["not found", "no such"]) {
        404
    } else if mentions(message, &["already exists"]) {
        409
    } else if mentions(message, &["invalid", "failed to parse"]) {
        400
    } else {
        500
    };
    answer(status, message)
}

/// Whether `message` holds any of `needles`, ignoring case.
fn mentions(message: &str, needles: &[&str]) -> bool {
    let lower = message.to_lowercase();
    needles.iter().any(|n| lower.contains(n))
}

/// The sentences Incus uses for failures that call for their own advice.
fn known(message: &str) -> Option<AppError> {
    use crate::messages as m;
    if mentions(message, &["no space left on device", "quota exceeded"]) {
        return Some(AppError::new(ErrorCode::DiskFull, m::DISK_FULL).with_hint(m::DISK_FULL_HINT));
    }
    if mentions(
        message,
        &["vm agent isn't currently running", "vm agent not running"],
    ) {
        return Some(
            AppError::new(
                ErrorCode::IncusAgentNotRunning,
                crate::messages::AGENT_NOT_RUNNING,
            )
            .with_hint(crate::messages::AGENT_NOT_RUNNING_HINT),
        );
    }
    if mentions(
        message,
        &[
            "kvm",
            "virtualization",
            "qemu-system",
            "virtual-machine is not supported",
            "instance type \"virtual-machine\" is not supported",
        ],
    ) && mentions(
        message,
        &[
            "not supported",
            "isn't supported",
            "unavailable",
            "not available",
            "no such file",
            "couldn't find",
            "not found",
        ],
    ) {
        return Some(
            AppError::new(
                ErrorCode::VirtualizationUnavailable,
                m::VIRTUALIZATION_UNAVAILABLE,
            )
            .with_hint(m::VIRTUALIZATION_UNAVAILABLE_HINT),
        );
    }
    if mentions(message, &["stateful"])
        && mentions(
            message,
            &[
                "migration.stateful",
                "not supported",
                "criu",
                "isn't supported",
            ],
        )
    {
        return Some(
            AppError::new(ErrorCode::StatefulUnsupported, m::STATEFUL_UNSUPPORTED)
                .with_hint(m::STATEFUL_UNSUPPORTED_HINT)
                .with_command("incus config set <instance> migration.stateful=true"),
        );
    }
    if mentions(
        message,
        &["instance is not running", "is not running", "isn't running"],
    ) {
        return Some(
            AppError::new(ErrorCode::ContainerNotRunning, m::INSTANCE_NOT_RUNNING)
                .with_hint(m::INSTANCE_NOT_RUNNING_HINT),
        );
    }
    if mentions(message, &["instance is already running", "already running"]) {
        return Some(AppError::new(
            ErrorCode::AlreadyDone,
            m::INSTANCE_ALREADY_RUNNING,
        ));
    }
    if mentions(
        message,
        &[
            "image not found",
            "couldn't find the requested image",
            "failed getting remote image info",
            "the requested image couldn't be found",
        ],
    ) {
        return Some(
            AppError::new(ErrorCode::ImageNotFound, m::IMAGE_NOT_FOUND)
                .with_hint(m::IMAGE_NOT_FOUND_HINT),
        );
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statuses_become_codes_the_user_can_act_on() {
        for (status, text, code) in [
            (404, "Instance not found", ErrorCode::NotFound),
            (409, "Instance \"web\" already exists", ErrorCode::NameInUse),
            (
                409,
                "Instance is busy running a \"start\" operation",
                ErrorCode::Conflict,
            ),
            (412, "ETag doesn't match", ErrorCode::ChangedSinceLoaded),
            (403, "not authorized", ErrorCode::Forbidden),
            (
                400,
                "Invalid config key \"limits.cpuz\"",
                ErrorCode::BadRequest,
            ),
            (500, "boom", ErrorCode::EngineError),
        ] {
            let e = answer(status, text);
            assert_eq!(e.code, code, "{status} {text}");
            assert!(e.detail.unwrap().contains(text));
            assert!(e.hint.is_some());
        }
    }

    #[test]
    fn well_known_sentences_get_their_own_advice() {
        for (text, code) in [
            ("Failed to connect to lxd-agent: VM agent isn't currently running", ErrorCode::IncusAgentNotRunning),
            ("Instance is not running", ErrorCode::ContainerNotRunning),
            ("Failed creating instance record: Instance type \"virtual-machine\" is not supported on this server: KVM support is missing (no /dev/kvm)", ErrorCode::VirtualizationUnavailable),
            ("Stateful snapshot requires that the instance limits.memory is less than size.state on the root disk device or migration.stateful to be set to true", ErrorCode::StatefulUnsupported),
            ("Failed getting remote image info: Failed getting image: The requested image couldn't be found", ErrorCode::ImageNotFound),
            ("Failed unpacking image: write /var/lib/incus: no space left on device", ErrorCode::DiskFull),
        ] {
            assert_eq!(operation(text).code, code, "{text}");
        }
    }
}

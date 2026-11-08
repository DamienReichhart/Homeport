//! containerd's gRPC failures, as errors the user can act on.

use std::error::Error as StdError;

use tonic::{Code, Status};

use homeport_core::error::{reach::classified_cause, AppError, ErrorCode};

use crate::messages as msg;

/// The failure behind `status`, keeping containerd's own words as the detail.
pub(crate) fn status(status: &Status) -> AppError {
    if let Some(classified) = classified_cause(status as &(dyn StdError + 'static)) {
        return classified.clone();
    }
    let (code, message, hint) = match status.code() {
        Code::NotFound => (ErrorCode::NotFound, msg::NOT_FOUND, msg::DETAILS_HINT),
        Code::AlreadyExists => (ErrorCode::Conflict, msg::ALREADY_EXISTS, msg::DETAILS_HINT),
        Code::FailedPrecondition => (ErrorCode::Conflict, msg::REFUSED, msg::DETAILS_HINT),
        Code::PermissionDenied | Code::Unauthenticated => (
            ErrorCode::PermissionDenied,
            msg::PERMISSION_DENIED,
            msg::DETAILS_HINT,
        ),
        Code::Unavailable | Code::Cancelled => (
            ErrorCode::ConnectionLost,
            msg::UNAVAILABLE,
            msg::UNAVAILABLE_HINT,
        ),
        Code::DeadlineExceeded => (ErrorCode::Timeout, msg::TIMED_OUT, msg::DETAILS_HINT),
        Code::Unimplemented => (
            ErrorCode::EngineUnsupported,
            msg::UNIMPLEMENTED,
            msg::UNIMPLEMENTED_HINT,
        ),
        Code::InvalidArgument | Code::OutOfRange => {
            (ErrorCode::BadRequest, msg::INVALID, msg::DETAILS_HINT)
        }
        _ => (ErrorCode::EngineError, msg::FAILED, msg::DETAILS_HINT),
    };
    AppError::new(code, message)
        .with_hint(hint)
        .with_detail(detail(status))
}

/// containerd's message and gRPC code, for the Details dialog.
pub(crate) fn detail(status: &Status) -> String {
    let mut text = format!("{:?}: {}", status.code(), status.message());
    let mut source = StdError::source(status);
    while let Some(cause) = source {
        text.push_str(&format!(": {cause}"));
        source = cause.source();
    }
    text
}

/// The first call to `name`'s socket failed: the dialer's own reason when
/// it has one, or else something that isn't containerd answered.
pub(crate) fn not_containerd(name: &str, status: &Status) -> AppError {
    if let Some(classified) = classified_cause(status as &(dyn StdError + 'static)) {
        return classified.clone();
    }
    if matches!(status.code(), Code::PermissionDenied | Code::Unimplemented) {
        return self::status(status);
    }
    AppError::new(
        ErrorCode::RemoteEngineUnavailable,
        msg::NOT_CONTAINERD.with("name", name),
    )
    .with_hint(msg::NOT_CONTAINERD_HINT)
    .with_detail(detail(status))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grpc_codes_become_codes_the_user_can_act_on() {
        for (code, expected) in [
            (Code::NotFound, ErrorCode::NotFound),
            (Code::AlreadyExists, ErrorCode::Conflict),
            (Code::FailedPrecondition, ErrorCode::Conflict),
            (Code::Unavailable, ErrorCode::ConnectionLost),
            (Code::Unimplemented, ErrorCode::EngineUnsupported),
            (Code::Internal, ErrorCode::EngineError),
        ] {
            let e = status(&Status::new(code, "containerd says no"));
            assert_eq!(e.code, expected, "{code:?}");
            assert!(e.detail.unwrap().contains("containerd says no"));
            assert!(e.hint.is_some());
        }
    }

    #[test]
    fn a_socket_that_isnt_containerd_says_so() {
        let e = not_containerd("lab", &Status::unknown("h2 protocol error"));
        assert_eq!(e.code, ErrorCode::RemoteEngineUnavailable);
        assert!(e.message.contains("lab"), "{e:?}");
    }
}

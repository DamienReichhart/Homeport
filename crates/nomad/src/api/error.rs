//! Nomad's failures, as errors the user can act on. Nomad answers an error
//! with an HTTP status and a line of text; the status decides the code, a
//! few well-known lines a more precise one, and the text itself goes to
//! Details. Failures to reach the agent are told apart by their cause.

use std::error::Error as StdError;

use homeport_core::error::{chain, reach::as_io, AppError, ErrorCode};
use homeport_transport::reach::network;

use crate::{api::address::host_port, messages as msg};

/// What Nomad's servers print, and Homeport's docs suggest, for a new token.
pub const TOKEN_COMMAND: &str = "nomad acl token create -name=homeport -policy=homeport";

/// Whether Nomad's `text` holds any of `needles`, ignoring case.
pub(crate) fn mentions(text: &str, needles: &[&str]) -> bool {
    let lower = text.to_lowercase();
    needles.iter().any(|n| lower.contains(n))
}

/// The failure Nomad `name` reported with `status` and its own `text`.
pub fn answer(name: &str, status: u16, text: &str) -> AppError {
    let text = text.trim();
    let detail = format!("HTTP {status}: {text}");
    known(name, status, text)
        .unwrap_or_else(|| match status {
            404 => AppError::new(ErrorCode::NotFound, msg::NOT_FOUND.with("name", name))
                .with_hint(msg::DETAILS_HINT),
            400 | 422 => AppError::new(ErrorCode::BadRequest, msg::BAD_REQUEST.with("name", name))
                .with_hint(msg::BAD_REQUEST_HINT),
            403 => permission_denied(name),
            _ => AppError::new(
                ErrorCode::ClusterApiError,
                msg::API_ERROR.with("name", name),
            )
            .with_hint(msg::DETAILS_HINT),
        })
        .with_detail(detail)
}

/// The token lacks a permission; callers name the capability when they know it.
pub fn permission_denied(name: &str) -> AppError {
    AppError::new(
        ErrorCode::NomadPermissionDenied,
        msg::PERMISSION_DENIED.with("name", name),
    )
    .with_hint(msg::PERMISSION_DENIED_HINT)
}

/// The lines Nomad uses for failures that call for their own advice.
fn known(name: &str, status: u16, text: &str) -> Option<AppError> {
    if mentions(
        text,
        &[
            "acl token not found",
            "token not found",
            "acl token expired",
        ],
    ) {
        return Some(token_invalid(name));
    }
    if status == 403 && mentions(text, &["permission denied"]) {
        return Some(permission_denied(name));
    }
    if mentions(text, &["no cluster leader", "no leader"]) {
        return Some(
            AppError::new(ErrorCode::NomadNoLeader, msg::NO_LEADER.with("name", name))
                .with_hint(msg::NO_LEADER_HINT)
                .with_command("nomad server members"),
        );
    }
    if status == 400 && mentions(text, &["http request to an https server"]) {
        return Some(
            AppError::new(
                ErrorCode::TlsNotSupported,
                msg::TLS_REQUIRED.with("name", name),
            )
            .with_hint(msg::TLS_REQUIRED_HINT),
        );
    }
    None
}

/// Nomad doesn't know the ACL token.
pub fn token_invalid(name: &str) -> AppError {
    AppError::new(
        ErrorCode::NomadTokenInvalid,
        msg::TOKEN_INVALID.with("name", name),
    )
    .with_hint(msg::TOKEN_INVALID_HINT)
    .with_command(TOKEN_COMMAND)
}

/// An answer that isn't the JSON Nomad sends.
pub fn unreadable(name: &str, cause: &dyn std::fmt::Display) -> AppError {
    AppError::new(
        ErrorCode::ClusterApiError,
        msg::UNREADABLE.with("name", name),
    )
    .with_hint(msg::UNREADABLE_HINT)
    .with_detail(cause)
}

/// Why a request to `name` at `address` got no answer: a timeout, TLS, or the network.
pub fn unreachable(name: &str, address: &str, error: &(dyn StdError + 'static)) -> AppError {
    if let Some(tls) = rustls_cause(error) {
        return self::tls(name, address, tls).with_cause(error);
    }
    let text = chain(error);
    if text.contains("timed out") || text.contains("deadline has elapsed") {
        return AppError::new(ErrorCode::Timeout, msg::TIMEOUT.with("name", name))
            .with_hint(msg::TIMEOUT_HINT)
            .with_cause(error);
    }
    if text.contains("InvalidContentType") || text.contains("invalid HTTP version") {
        return tls_not_supported(name, address).with_cause(error);
    }
    network(host_port(address), error)
}

/// The first rustls error in `error`'s chain; rustls reports through `io::Error`.
fn rustls_cause<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a rustls::Error> {
    std::iter::successors(Some(error), |&e| e.source()).find_map(|e| {
        e.downcast_ref::<rustls::Error>()
            .or_else(|| as_io(e)?.get_ref()?.downcast_ref::<rustls::Error>())
    })
}

fn tls(name: &str, address: &str, error: &rustls::Error) -> AppError {
    use rustls::{AlertDescription as Alert, CertificateError as Cert, Error as Tls};
    let (code, message, hint) = match error {
        Tls::InvalidCertificate(Cert::Expired | Cert::ExpiredContext { .. }) => (
            ErrorCode::TlsCertificateExpired,
            msg::TLS_CERT_EXPIRED.with("name", name),
            msg::TLS_CERT_EXPIRED_HINT,
        ),
        Tls::InvalidCertificate(Cert::UnknownIssuer | Cert::BadSignature) => (
            ErrorCode::TlsUnknownCa,
            msg::TLS_UNKNOWN_CA.with("name", name),
            msg::TLS_UNKNOWN_CA_HINT,
        ),
        Tls::InvalidCertificate(Cert::NotValidForName | Cert::NotValidForNameContext { .. }) => (
            ErrorCode::TlsNameMismatch,
            msg::TLS_NAME_MISMATCH
                .with("name", name)
                .with("server", host_port(address)),
            msg::TLS_NAME_MISMATCH_HINT,
        ),
        Tls::AlertReceived(
            Alert::BadCertificate
            | Alert::UnknownCA
            | Alert::CertificateRequired
            | Alert::CertificateUnknown
            | Alert::CertificateExpired
            | Alert::AccessDenied
            | Alert::DecryptError
            | Alert::HandshakeFailure,
        ) => (
            ErrorCode::TlsClientRejected,
            msg::TLS_CLIENT_REJECTED.with("name", name),
            msg::TLS_CLIENT_REJECTED_HINT,
        ),
        Tls::InvalidMessage(_) | Tls::InappropriateMessage { .. } => {
            return tls_not_supported(name, address)
        }
        _ => (
            ErrorCode::TlsHandshakeFailed,
            msg::TLS_HANDSHAKE_FAILED.with("name", name),
            msg::TLS_HANDSHAKE_FAILED_HINT,
        ),
    };
    AppError::new(code, message).with_hint(hint)
}

fn tls_not_supported(name: &str, address: &str) -> AppError {
    AppError::new(
        ErrorCode::TlsNotSupported,
        msg::TLS_NOT_SUPPORTED
            .with("name", name)
            .with("server", host_port(address)),
    )
    .with_hint(msg::TLS_NOT_SUPPORTED_HINT)
}

/// Whether a failure needs the user before trying again, rather than a retry.
pub fn needs_user(error: &AppError) -> bool {
    matches!(
        error.code,
        ErrorCode::NomadTokenInvalid
            | ErrorCode::TlsMaterialInvalid
            | ErrorCode::TlsUnknownCa
            | ErrorCode::TlsNameMismatch
            | ErrorCode::TlsClientRejected
            | ErrorCode::TlsCertificateExpired
            | ErrorCode::TlsNotSupported
            | ErrorCode::Validation
    )
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn an_unknown_token_needs_a_new_one() {
        let e = answer("prod", 403, "ACL token not found\n");
        assert!(e.is(ErrorCode::NomadTokenInvalid));
        assert!(e.message.contains("prod"));
        let hint = e.hint.as_ref().unwrap();
        assert_eq!(hint.commands, [TOKEN_COMMAND]);
        assert_eq!(e.detail.as_deref(), Some("HTTP 403: ACL token not found"));
        assert!(needs_user(&e));
    }

    #[test]
    fn a_missing_permission_is_told_apart_from_a_bad_token() {
        let e = answer("prod", 403, "Permission denied");
        assert!(e.is(ErrorCode::NomadPermissionDenied));
        assert!(!needs_user(&e));
    }

    #[test]
    fn no_leader_shows_how_to_check_the_servers() {
        let e = answer("prod", 500, "No cluster leader");
        assert!(e.is(ErrorCode::NomadNoLeader));
        assert_eq!(e.hint.unwrap().commands, ["nomad server members"]);
    }

    #[test]
    fn plain_http_to_an_https_agent_asks_for_https() {
        let e = answer(
            "prod",
            400,
            "Client sent an HTTP request to an HTTPS server.",
        );
        assert!(e.is(ErrorCode::TlsNotSupported));
        assert_eq!(e.key, "errors.nomad.tls.required");
    }

    #[test]
    fn other_statuses_keep_nomads_text_in_the_details() {
        assert!(answer("prod", 404, "job not found").is(ErrorCode::NotFound));
        assert!(answer("prod", 400, "bad").is(ErrorCode::BadRequest));
        let e = answer("prod", 500, "rpc error: something");
        assert!(e.is(ErrorCode::ClusterApiError));
        assert!(e.detail.unwrap().contains("rpc error"));
    }

    #[test]
    fn network_failures_name_the_address() {
        let e = unreachable(
            "prod",
            "http://10.0.0.5:4646",
            &io::Error::from(io::ErrorKind::ConnectionRefused),
        );
        assert!(e.is(ErrorCode::ConnectionRefused));
        assert!(e.message.contains("10.0.0.5:4646"), "{e:?}");
    }

    #[test]
    fn tls_failures_are_told_apart() {
        let wrapped = |e: rustls::Error| io::Error::new(io::ErrorKind::InvalidData, e);
        for (tls, code) in [
            (
                rustls::Error::InvalidCertificate(rustls::CertificateError::UnknownIssuer),
                ErrorCode::TlsUnknownCa,
            ),
            (
                rustls::Error::InvalidCertificate(rustls::CertificateError::NotValidForName),
                ErrorCode::TlsNameMismatch,
            ),
            (
                rustls::Error::AlertReceived(rustls::AlertDescription::CertificateRequired),
                ErrorCode::TlsClientRejected,
            ),
        ] {
            let e = unreachable("prod", "https://nomad:4646", &wrapped(tls));
            assert_eq!(e.code, code);
            assert!(e.key.starts_with("errors.nomad.tls."), "{e:?}");
            assert!(needs_user(&e));
        }
    }
}

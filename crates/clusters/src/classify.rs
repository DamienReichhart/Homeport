//! Turns what kube-rs reports into errors the user can act on.

use std::error::Error as StdError;

use homeport_core::error::{chain, reach::as_io, AppError, ErrorCode};
use homeport_transport::reach::{classified_cause, network};

use crate::messages as msg;

/// Why a request to the Cluster `name`, whose API server is `server`, failed.
pub fn classify(name: &str, server: &str, error: &kube::Error) -> AppError {
    match error {
        kube::Error::Api(status) => api(name, status.code, &status.reason, &status.message),
        kube::Error::SerdeError(_) | kube::Error::FromUtf8(_) => AppError::new(
            ErrorCode::ClusterApiError,
            msg::UNREADABLE.with("name", name),
        )
        .with_cause(error),
        _ => unreachable(name, server, error),
    }
}

/// An answer of the Kubernetes API with an error status.
pub fn api(name: &str, code: u16, reason: &str, message: &str) -> AppError {
    let detail = format!("{code} {reason}: {message}");
    match code {
        401 => unauthorized(name).with_detail(detail),
        _ => AppError::new(
            ErrorCode::ClusterApiError,
            msg::API_ERROR.with("name", name),
        )
        .with_hint(msg::API_ERROR_HINT)
        .with_detail(detail),
    }
}

/// The API server refused the credentials.
pub fn unauthorized(name: &str) -> AppError {
    AppError::new(
        ErrorCode::ClusterUnauthorized,
        msg::UNAUTHORIZED.with("name", name),
    )
    .with_hint(msg::UNAUTHORIZED_HINT)
}

/// The kubeconfig's certificates or keys can't be used.
pub fn credentials_unusable(name: &str, cause: impl std::fmt::Display) -> AppError {
    AppError::new(
        ErrorCode::TlsMaterialInvalid,
        msg::CREDENTIALS_UNUSABLE.with("name", name),
    )
    .with_hint(msg::CREDENTIALS_UNUSABLE_HINT)
    .with_detail(cause)
}

/// The host and port of `server`, as the user knows it.
pub fn address(server: &str) -> &str {
    let rest = server.split_once("://").map_or(server, |(_, rest)| rest);
    rest.split('/').next().unwrap_or(rest)
}

/// The network or TLS failed before the API answered.
fn unreachable(name: &str, server: &str, error: &(dyn StdError + 'static)) -> AppError {
    if let Some(classified) = classified_cause(error) {
        return classified.clone();
    }
    if let Some(tls) = rustls_cause(error) {
        return self::tls(name, server, tls).with_cause(error);
    }
    let text = chain(error);
    if text.contains("deadline has elapsed") || text.contains("timed out") {
        return AppError::new(ErrorCode::Timeout, msg::TIMEOUT.with("name", name))
            .with_hint(msg::TIMEOUT_HINT)
            .with_cause(error);
    }
    if text.contains("InvalidContentType") || text.contains("invalid HTTP version") {
        return tls_not_supported(name, server).with_cause(error);
    }
    network(address(server), error)
}

/// The first rustls error in `error`'s chain; rustls reports through `io::Error`.
fn rustls_cause<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a rustls::Error> {
    std::iter::successors(Some(error), |&e| e.source()).find_map(|e| {
        e.downcast_ref::<rustls::Error>()
            .or_else(|| as_io(e)?.get_ref()?.downcast_ref::<rustls::Error>())
    })
}

fn tls(name: &str, server: &str, error: &rustls::Error) -> AppError {
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
                .with("server", address(server)),
            msg::TLS_NAME_MISMATCH_HINT,
        ),
        Tls::AlertReceived(
            Alert::BadCertificate
            | Alert::UnknownCA
            | Alert::CertificateRequired
            | Alert::CertificateUnknown
            | Alert::CertificateExpired
            | Alert::AccessDenied
            | Alert::DecryptError,
        ) => (
            ErrorCode::TlsClientRejected,
            msg::TLS_CLIENT_REJECTED.with("name", name),
            msg::TLS_CLIENT_REJECTED_HINT,
        ),
        Tls::InvalidMessage(_) | Tls::InappropriateMessage { .. } => {
            return tls_not_supported(name, server)
        }
        _ => (
            ErrorCode::TlsHandshakeFailed,
            msg::TLS_HANDSHAKE_FAILED.with("name", name),
            msg::TLS_HANDSHAKE_FAILED_HINT,
        ),
    };
    AppError::new(code, message).with_hint(hint)
}

fn tls_not_supported(name: &str, server: &str) -> AppError {
    AppError::new(
        ErrorCode::TlsNotSupported,
        msg::TLS_NOT_SUPPORTED
            .with("name", name)
            .with("server", address(server)),
    )
    .with_hint(msg::TLS_NOT_SUPPORTED_HINT)
}

/// Whether a failure needs the user before trying again, rather than a retry.
pub fn needs_user(error: &AppError) -> bool {
    matches!(
        error.code,
        ErrorCode::ClusterUnauthorized
            | ErrorCode::ExecPluginMissing
            | ErrorCode::ExecPluginInteractive
            | ErrorCode::TlsMaterialInvalid
            | ErrorCode::TlsUnknownCa
            | ErrorCode::TlsNameMismatch
            | ErrorCode::TlsClientRejected
            | ErrorCode::TlsCertificateExpired
            | ErrorCode::TlsNotSupported
    )
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn a_401_needs_new_credentials() {
        let e = api("prod", 401, "Unauthorized", "Unauthorized");
        assert!(e.is(ErrorCode::ClusterUnauthorized));
        assert!(needs_user(&e));
        assert_eq!(e.params["name"].to_string(), "prod");
        assert!(e.detail.unwrap().contains("401"));
    }

    #[test]
    fn other_statuses_keep_the_apis_words_in_details() {
        let e = api("prod", 500, "InternalError", "etcd is down");
        assert!(e.is(ErrorCode::ClusterApiError));
        assert!(!needs_user(&e));
        assert!(e.detail.unwrap().contains("etcd is down"));
    }

    #[test]
    fn servers_are_named_by_their_address() {
        assert_eq!(address("https://127.0.0.1:6443"), "127.0.0.1:6443");
        assert_eq!(address("https://k8s.example.com/prefix"), "k8s.example.com");
        assert_eq!(address("127.0.0.1:6443"), "127.0.0.1:6443");
    }

    #[test]
    fn refused_connections_name_the_server_and_are_retried() {
        let e = unreachable(
            "kind-dev",
            "https://127.0.0.1:6443",
            &io::Error::from(io::ErrorKind::ConnectionRefused),
        );
        assert!(e.is(ErrorCode::ConnectionRefused));
        assert!(e.message.contains("127.0.0.1:6443"), "{e:?}");
        assert!(!needs_user(&e));
    }

    #[test]
    fn an_unknown_ca_asks_for_a_refresh() {
        let error = io::Error::other(rustls::Error::InvalidCertificate(
            rustls::CertificateError::UnknownIssuer,
        ));
        let e = unreachable("kind-dev", "https://127.0.0.1:6443", &error);
        assert!(e.is(ErrorCode::TlsUnknownCa));
        assert!(needs_user(&e));
    }

    #[test]
    fn timeouts_are_retried() {
        let e = unreachable(
            "prod",
            "https://k8s.example.com",
            &io::Error::other("deadline has elapsed"),
        );
        assert!(e.is(ErrorCode::Timeout));
        assert!(!needs_user(&e));
    }
}

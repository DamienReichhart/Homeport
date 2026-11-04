//! A TLS handshake that failed: certificates, CAs, and servers that don't
//! speak TLS.

use std::error::Error as StdError;

use homeport_core::error::{reach::as_io, AppError, ErrorCode};

use super::network;
use crate::messages as msg;

/// The first rustls error in `error`'s chain; rustls reports through `io::Error`.
fn rustls_cause<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a rustls::Error> {
    std::iter::successors(Some(error), |&e| e.source()).find_map(|e| {
        e.downcast_ref::<rustls::Error>()
            .or_else(|| as_io(e)?.get_ref()?.downcast_ref::<rustls::Error>())
    })
}

/// Why the TLS connection to `address` failed; network failures fall back to
/// [`network`]. `label` names the Host in messages.
pub fn tls(label: &str, address: &str, error: &(dyn StdError + 'static)) -> AppError {
    use rustls::{AlertDescription as Alert, CertificateError as Cert, Error as Tls};
    let Some(tls) = rustls_cause(error) else {
        return network(label, error);
    };
    let host = address
        .rsplit_once(':')
        .map_or(address, |(host, _)| host)
        .trim_start_matches('[')
        .trim_end_matches(']');
    let client_rejected = || {
        AppError::new(
            ErrorCode::TlsClientRejected,
            msg::TLS_CLIENT_REJECTED.with("host", label),
        )
        .with_hint(msg::TLS_CLIENT_REJECTED_HINT)
    };
    let classified = match tls {
        Tls::InvalidCertificate(Cert::Expired | Cert::ExpiredContext { .. }) => AppError::new(
            ErrorCode::TlsCertificateExpired,
            msg::TLS_CERT_EXPIRED.with("host", label),
        )
        .with_hint(msg::TLS_CERT_EXPIRED_HINT),
        Tls::InvalidCertificate(Cert::NotValidYet | Cert::NotValidYetContext { .. }) => {
            AppError::new(
                ErrorCode::TlsCertificateNotYetValid,
                msg::TLS_CERT_NOT_YET_VALID.with("host", label),
            )
            .with_hint(msg::TLS_CERT_NOT_YET_VALID_HINT)
        }
        Tls::InvalidCertificate(Cert::UnknownIssuer) => AppError::new(
            ErrorCode::TlsUnknownCa,
            msg::TLS_UNKNOWN_CA.with("host", label),
        )
        .with_hint(msg::TLS_UNKNOWN_CA_HINT),
        Tls::InvalidCertificate(Cert::NotValidForName | Cert::NotValidForNameContext { .. }) => {
            AppError::new(
                ErrorCode::TlsNameMismatch,
                msg::TLS_NAME_MISMATCH
                    .with("host", label)
                    .with("name", host),
            )
            .with_hint(msg::TLS_NAME_MISMATCH_HINT)
        }
        Tls::AlertReceived(Alert::CertificateExpired) => AppError::new(
            ErrorCode::TlsCertificateExpired,
            msg::TLS_CLIENT_CERT_EXPIRED,
        )
        .with_hint(msg::TLS_CLIENT_CERT_EXPIRED_HINT),
        Tls::AlertReceived(
            Alert::BadCertificate
            | Alert::UnknownCA
            | Alert::CertificateRequired
            | Alert::CertificateUnknown
            | Alert::AccessDenied
            | Alert::DecryptError,
        ) => client_rejected(),
        Tls::InvalidMessage(_) | Tls::InappropriateMessage { .. } => AppError::new(
            ErrorCode::TlsNotSupported,
            msg::TLS_NOT_SUPPORTED.with("host", label),
        )
        .with_hint(msg::TLS_NOT_SUPPORTED_HINT),
        _ => AppError::new(
            ErrorCode::TlsHandshakeFailed,
            msg::TLS_HANDSHAKE_FAILED.with("host", label),
        )
        .with_hint(msg::TLS_HANDSHAKE_FAILED_HINT),
    };
    classified.with_cause(error)
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    fn tls_failure(error: rustls::Error) -> AppError {
        tls(
            "prod (prod.example:2376)",
            "prod.example:2376",
            &io::Error::new(io::ErrorKind::InvalidData, error),
        )
    }

    #[test]
    fn certificate_problems_are_told_apart() {
        use rustls::{AlertDescription as Alert, CertificateError as Cert, Error as Tls};
        for (error, code) in [
            (
                Tls::InvalidCertificate(Cert::Expired),
                ErrorCode::TlsCertificateExpired,
            ),
            (
                Tls::InvalidCertificate(Cert::NotValidYet),
                ErrorCode::TlsCertificateNotYetValid,
            ),
            (
                Tls::InvalidCertificate(Cert::UnknownIssuer),
                ErrorCode::TlsUnknownCa,
            ),
            (
                Tls::InvalidCertificate(Cert::NotValidForName),
                ErrorCode::TlsNameMismatch,
            ),
            (
                Tls::AlertReceived(Alert::BadCertificate),
                ErrorCode::TlsClientRejected,
            ),
            (
                Tls::AlertReceived(Alert::CertificateExpired),
                ErrorCode::TlsCertificateExpired,
            ),
            (
                Tls::InvalidMessage(rustls::InvalidMessage::InvalidContentType),
                ErrorCode::TlsNotSupported,
            ),
        ] {
            let text = error.to_string();
            let e = tls_failure(error);
            assert_eq!(e.code, code, "{text}");
            assert!(e.hint.is_some(), "{text}");
        }
    }

    #[test]
    fn name_mismatches_name_the_host() {
        let e = tls_failure(rustls::Error::InvalidCertificate(
            rustls::CertificateError::NotValidForName,
        ));
        assert!(e.message.contains("isn't made for prod.example."), "{e:?}");
    }

    #[test]
    fn a_client_certificate_refused_after_the_handshake_is_still_a_rejection() {
        // TLS 1.3 servers send this alert on the first read, inside bollard's request.
        let alert = rustls::Error::AlertReceived(rustls::AlertDescription::CertificateRequired);
        let e = bollard::errors::Error::IOError {
            err: io::Error::new(io::ErrorKind::InvalidData, alert),
        };
        let e = tls("prod (prod.example:2376)", "prod.example:2376", &e);
        assert_eq!(e.code, ErrorCode::TlsClientRejected);
    }

    #[test]
    fn tls_falls_back_to_network_failures() {
        let e = tls(
            "prod (prod.example:2376)",
            "prod.example:2376",
            &io::Error::from(io::ErrorKind::ConnectionRefused),
        );
        assert_eq!(e.code, ErrorCode::ConnectionRefused);
    }
}

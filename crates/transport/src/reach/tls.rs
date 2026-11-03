//! A TLS handshake that failed: certificates, CAs, and servers that don't
//! speak TLS.

use std::error::Error as StdError;

use homeport_core::error::{reach::as_io, AppError, ErrorCode};

use super::network;

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
            format!("{label} rejected Homeport's client certificate."),
        )
        .with_hint("Use a client certificate (cert.pem and key.pem) signed by the CA the Engine trusts, its --tlscacert.")
    };
    let classified = match tls {
        Tls::InvalidCertificate(Cert::Expired | Cert::ExpiredContext { .. }) => AppError::new(
            ErrorCode::TlsCertificateExpired,
            format!("The certificate of {label} has expired."),
        )
        .with_hint("Renew the Docker Engine's TLS certificate, then reconnect."),
        Tls::InvalidCertificate(Cert::NotValidYet | Cert::NotValidYetContext { .. }) => {
            AppError::new(
                ErrorCode::TlsCertificateNotYetValid,
                format!("The certificate of {label} isn't valid yet."),
            )
            .with_hint("Check that this computer's date and time are right.")
        }
        Tls::InvalidCertificate(Cert::UnknownIssuer) => AppError::new(
            ErrorCode::TlsUnknownCa,
            format!("The certificate of {label} isn't signed by the CA certificate on the host page."),
        )
        .with_hint("Paste the ca.pem that signed the Engine's certificate."),
        Tls::InvalidCertificate(Cert::NotValidForName | Cert::NotValidForNameContext { .. }) => {
            AppError::new(
                ErrorCode::TlsNameMismatch,
                format!("The certificate of {label} isn't made for {host}."),
            )
            .with_hint("Connect with a name or IP address listed in the certificate, or reissue it to include this one.")
        }
        Tls::AlertReceived(Alert::CertificateExpired) => AppError::new(
            ErrorCode::TlsCertificateExpired,
            "Homeport's client certificate has expired.",
        )
        .with_hint("Make a new client certificate signed by the Engine's CA and paste it on the host page."),
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
            format!("{label} doesn't speak TLS."),
        )
        .with_hint("Use a TCP connection for this address, or the Engine's TLS port (usually 2376)."),
        _ => AppError::new(
            ErrorCode::TlsHandshakeFailed,
            format!("The secure connection to {label} failed."),
        )
        .with_hint("Check the certificates on the host page against the ones the Engine uses."),
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

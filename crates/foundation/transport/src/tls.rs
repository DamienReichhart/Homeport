//! Mutual-TLS streams to a remote Engine: verified against a CA for a
//! Docker Engine (`dockerd --tlsverify`), or pinned to the one server
//! certificate an Incus Host presents (ADR 0021).

use std::{
    io,
    sync::{Arc, Mutex},
};

use base64::Engine as _;
use rustls::{
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider},
    CertificateError, ClientConfig, DigitallySignedStruct, OtherError, RootCertStore,
    SignatureScheme,
};
use rustls_pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer, ServerName, UnixTime};
use sha2::{Digest, Sha256};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

use homeport_core::error::{AppError, ErrorCode, Message};

use super::{BoxFuture, BoxStream, ConnectError, Dialer, CONNECT_TIMEOUT};
use crate::{messages as msg, reach};

pub struct TlsDialer {
    /// The Host's name and address, for messages.
    label: String,
    address: String,
    server_name: ServerName<'static>,
    connector: TlsConnector,
}

/// The SHA-256 fingerprint of a certificate, as Incus prints it: 64
/// lowercase hex digits.
pub fn fingerprint(der: &[u8]) -> String {
    Sha256::digest(der)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// A certificate as PEM text.
pub fn pem(der: &[u8]) -> String {
    let body = base64::engine::general_purpose::STANDARD.encode(der);
    let mut text = String::from("-----BEGIN CERTIFICATE-----\n");
    for line in body.as_bytes().chunks(64) {
        text.push_str(&String::from_utf8_lossy(line));
        text.push('\n');
    }
    text.push_str("-----END CERTIFICATE-----\n");
    text
}

/// A server that presented another certificate than the one pinned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinMismatch {
    pub expected: String,
    pub presented: String,
}

impl std::fmt::Display for PinMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the server presented certificate {}, not the pinned {}",
            self.presented, self.expected
        )
    }
}

impl std::error::Error for PinMismatch {}

/// Accepts exactly the certificate whose fingerprint is `expected`,
/// whatever the name it's for, and checks the handshake's signatures with
/// it. With `expected` empty it accepts any certificate and keeps the one
/// it saw in `seen`, for reading a server's certificate before trusting it.
#[derive(Debug)]
struct Pinned {
    expected: String,
    seen: Arc<Mutex<Option<Vec<u8>>>>,
    provider: Arc<CryptoProvider>,
}

impl ServerCertVerifier for Pinned {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        *self.seen.lock().expect("certificate lock poisoned") = Some(end_entity.to_vec());
        let presented = fingerprint(end_entity);
        if self.expected.is_empty() || presented == self.expected {
            return Ok(ServerCertVerified::assertion());
        }
        Err(rustls::Error::InvalidCertificate(CertificateError::Other(
            OtherError(Arc::new(PinMismatch {
                expected: self.expected.clone(),
                presented,
            })),
        )))
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider
            .signature_verification_algorithms
            .supported_schemes()
    }
}

fn provider() -> Arc<CryptoProvider> {
    Arc::new(rustls::crypto::ring::default_provider())
}

fn invalid(message: Message, e: &dyn std::fmt::Display) -> ConnectError {
    ConnectError::Invalid(material_invalid(message).with_detail(e))
}

/// The client certificate chain and key in `cert_pem` and `key_pem`.
fn client_auth(
    cert_pem: &str,
    key_pem: &str,
) -> Result<(Vec<CertificateDer<'static>>, PrivateKeyDer<'static>), ConnectError> {
    let chain = CertificateDer::pem_slice_iter(cert_pem.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| invalid(msg::TLS_CLIENT_CERT_UNUSABLE, &e))?;
    if chain.is_empty() {
        return Err(ConnectError::Invalid(material_invalid(
            msg::TLS_CLIENT_CERT_EMPTY,
        )));
    }
    let key = PrivateKeyDer::from_pem_slice(key_pem.as_bytes())
        .map_err(|e| invalid(msg::TLS_CLIENT_KEY_UNUSABLE, &e))?;
    Ok((chain, key))
}

/// The host part of `address`, without the brackets of an IPv6 address.
fn host_of(address: &str) -> &str {
    address
        .rsplit_once(':')
        .map_or(address, |(host, _)| host)
        .trim_start_matches('[')
        .trim_end_matches(']')
}

impl TlsDialer {
    /// Builds a dialer that verifies the server against the CA in
    /// `ca_pem` and the address's name, from PEM text; fails with a
    /// user-facing message when the certificates or key can't be used.
    pub fn new(
        label: &str,
        address: &str,
        ca_pem: &str,
        cert_pem: &str,
        key_pem: &str,
    ) -> Result<Self, ConnectError> {
        let mut roots = RootCertStore::empty();
        for cert in CertificateDer::pem_slice_iter(ca_pem.as_bytes()) {
            let cert = cert.map_err(|e| invalid(msg::TLS_CA_UNUSABLE, &e))?;
            roots
                .add(cert)
                .map_err(|e| invalid(msg::TLS_CA_UNUSABLE, &e))?;
        }
        if roots.is_empty() {
            return Err(ConnectError::Invalid(material_invalid(msg::TLS_CA_EMPTY)));
        }
        let (chain, key) = client_auth(cert_pem, key_pem)?;
        let config = ClientConfig::builder_with_provider(provider())
            .with_safe_default_protocol_versions()
            .map_err(|e| invalid(msg::TLS_CONFIG_UNUSABLE, &e))?
            .with_root_certificates(roots)
            .with_client_auth_cert(chain, key)
            .map_err(|e| invalid(msg::TLS_CLIENT_PAIR_UNUSABLE, &e))?;
        let server_name = ServerName::try_from(host_of(address).to_owned())
            .map_err(|e| invalid(msg::TLS_ADDRESS_UNUSABLE, &e))?;
        Ok(Self {
            label: label.to_owned(),
            address: address.to_owned(),
            server_name,
            connector: TlsConnector::from(Arc::new(config)),
        })
    }

    /// Builds a dialer that accepts exactly the server certificate in
    /// `server_pem`, compared by fingerprint, whatever name it's for: how
    /// Incus's self-signed certificates are trusted (ADR 0021).
    pub fn pinned(
        label: &str,
        address: &str,
        server_pem: &str,
        cert_pem: &str,
        key_pem: &str,
    ) -> Result<Self, ConnectError> {
        let server = CertificateDer::pem_slice_iter(server_pem.as_bytes())
            .next()
            .ok_or_else(|| ConnectError::Invalid(material_invalid(msg::TLS_PINNED_EMPTY)))?
            .map_err(|e| invalid(msg::TLS_PINNED_UNUSABLE, &e))?;
        Self::pinned_to(
            label,
            address,
            &fingerprint(&server),
            Some((cert_pem, key_pem)),
        )
    }

    /// A dialer that accepts the certificate with the fingerprint
    /// `expected` (any, when empty), presenting the client certificate in
    /// `client` if given.
    fn pinned_to(
        label: &str,
        address: &str,
        expected: &str,
        client: Option<(&str, &str)>,
    ) -> Result<Self, ConnectError> {
        Self::pinned_seen(label, address, expected, client, Arc::default())
    }

    fn pinned_seen(
        label: &str,
        address: &str,
        expected: &str,
        client: Option<(&str, &str)>,
        seen: Arc<Mutex<Option<Vec<u8>>>>,
    ) -> Result<Self, ConnectError> {
        let verifier = Arc::new(Pinned {
            expected: expected.to_owned(),
            seen,
            provider: provider(),
        });
        let builder = ClientConfig::builder_with_provider(provider())
            .with_safe_default_protocol_versions()
            .map_err(|e| invalid(msg::TLS_CONFIG_UNUSABLE, &e))?
            .dangerous()
            .with_custom_certificate_verifier(verifier);
        let config = match client {
            Some((cert_pem, key_pem)) => {
                let (chain, key) = client_auth(cert_pem, key_pem)?;
                builder
                    .with_client_auth_cert(chain, key)
                    .map_err(|e| invalid(msg::TLS_CLIENT_PAIR_UNUSABLE, &e))?
            }
            None => builder.with_no_client_auth(),
        };
        // Only for SNI: the certificate isn't checked against it.
        let server_name = ServerName::try_from(host_of(address).to_owned())
            .unwrap_or_else(|_| ServerName::try_from("incus").expect("a valid name"));
        Ok(Self {
            label: label.to_owned(),
            address: address.to_owned(),
            server_name,
            connector: TlsConnector::from(Arc::new(config)),
        })
    }

    async fn open(&self) -> io::Result<BoxStream> {
        let tcp = tokio::time::timeout(CONNECT_TIMEOUT, TcpStream::connect(&self.address))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "connection timed out"))??;
        tcp.set_nodelay(true)?;
        let tls = self
            .connector
            .connect(self.server_name.clone(), tcp)
            .await?;
        Ok(Box::new(tls))
    }
}

/// The certificate the server at `address` presents, as DER, read without
/// trusting it, so its fingerprint can be checked before anything is sent.
/// `label` names it in messages.
pub async fn server_certificate(label: &str, address: &str) -> Result<Vec<u8>, AppError> {
    let seen = Arc::new(Mutex::new(None));
    let dialer =
        TlsDialer::pinned_seen(label, address, "", None, seen.clone()).map_err(AppError::from)?;
    let opened = dialer.open().await;
    let certificate = seen.lock().expect("certificate lock poisoned").take();
    match (certificate, opened) {
        (Some(der), _) => Ok(der),
        (None, Err(e)) => Err(reach::tls(label, address, &e)),
        (None, Ok(_)) => Err(material_invalid(msg::TLS_PINNED_EMPTY)),
    }
}

fn material_invalid(message: impl Into<homeport_core::error::Text>) -> AppError {
    AppError::new(ErrorCode::TlsMaterialInvalid, message).with_hint(msg::TLS_MATERIAL_INVALID_HINT)
}

impl Dialer for TlsDialer {
    fn connect(&self) -> BoxFuture<'_, Result<(), ConnectError>> {
        Box::pin(async move {
            match self.open().await {
                Ok(_) => Ok(()),
                Err(e) => Err(ConnectError::from(reach::tls(
                    &self.label,
                    &self.address,
                    &e,
                ))),
            }
        })
    }

    fn dial(&self) -> BoxFuture<'_, io::Result<BoxStream>> {
        Box::pin(async move {
            // Keep the classification for whoever reads the failed request.
            self.open()
                .await
                .map_err(|e| io::Error::new(e.kind(), reach::tls(&self.label, &self.address, &e)))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_text_that_is_not_a_certificate() {
        let err = TlsDialer::new("prod (host:2376)", "host:2376", "nope", "nope", "nope")
            .err()
            .unwrap();
        assert!(matches!(err, ConnectError::Invalid(e) if e.message.contains("CA certificate")));
        let err = TlsDialer::pinned("lab (lab:8443)", "lab:8443", "nope", "nope", "nope")
            .err()
            .unwrap();
        assert!(
            matches!(err, ConnectError::Invalid(e) if e.message.contains("server certificate"))
        );
    }

    #[test]
    fn fingerprints_are_sha256_in_lowercase_hex() {
        assert_eq!(
            fingerprint(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn pem_wraps_der_in_64_character_lines() {
        let text = pem(&[0u8; 100]);
        assert!(text.starts_with("-----BEGIN CERTIFICATE-----\n"));
        assert!(text.lines().all(|l| l.len() <= 64));
        let back: Vec<_> = CertificateDer::pem_slice_iter(text.as_bytes())
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(back[0].as_ref(), &[0u8; 100]);
    }

    #[test]
    fn a_pin_accepts_only_its_certificate() {
        let verifier = Pinned {
            expected: fingerprint(b"server"),
            seen: Arc::default(),
            provider: provider(),
        };
        let name = ServerName::try_from("10.0.0.5").unwrap();
        let ok = verifier.verify_server_cert(
            &CertificateDer::from(b"server".to_vec()),
            &[],
            &name,
            &[],
            UnixTime::now(),
        );
        assert!(ok.is_ok());
        let err = verifier
            .verify_server_cert(
                &CertificateDer::from(b"other".to_vec()),
                &[],
                &name,
                &[],
                UnixTime::now(),
            )
            .unwrap_err();
        let classified = reach::tls(
            "lab (10.0.0.5:8443)",
            "10.0.0.5:8443",
            &io::Error::new(io::ErrorKind::InvalidData, err),
        );
        assert_eq!(classified.code, ErrorCode::CertificateMismatch);
        assert_eq!(
            classified.params.get("presented").map(ToString::to_string),
            Some(fingerprint(b"other"))
        );
        assert_eq!(
            classified.params.get("expected").map(ToString::to_string),
            Some(fingerprint(b"server"))
        );
    }
}

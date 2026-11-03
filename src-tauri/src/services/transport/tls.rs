//! Mutual-TLS streams to a remote Engine (`dockerd --tlsverify`).

use std::{io, sync::Arc};

use rustls::{ClientConfig, RootCertStore};
use rustls_pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer, ServerName};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

use super::{BoxFuture, BoxStream, ConnectError, Dialer, CONNECT_TIMEOUT};
use crate::error::{AppError, ErrorCode};

pub struct TlsDialer {
    address: String,
    server_name: ServerName<'static>,
    connector: TlsConnector,
}

impl TlsDialer {
    /// Builds a dialer from PEM text; fails with a user-facing message when
    /// the certificates or key can't be used.
    pub fn new(
        address: &str,
        ca_pem: &str,
        cert_pem: &str,
        key_pem: &str,
    ) -> Result<Self, ConnectError> {
        let invalid = |what: &str, e: &dyn std::fmt::Display| {
            ConnectError::Invalid(
                material_invalid(format!("The {what} can't be used.")).with_detail(e),
            )
        };

        let mut roots = RootCertStore::empty();
        for cert in CertificateDer::pem_slice_iter(ca_pem.as_bytes()) {
            let cert = cert.map_err(|e| invalid("CA certificate", &e))?;
            roots.add(cert).map_err(|e| invalid("CA certificate", &e))?;
        }
        if roots.is_empty() {
            return Err(ConnectError::Invalid(material_invalid(
                "The CA certificate contains no certificate.",
            )));
        }
        let chain = CertificateDer::pem_slice_iter(cert_pem.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| invalid("client certificate", &e))?;
        if chain.is_empty() {
            return Err(ConnectError::Invalid(material_invalid(
                "The client certificate contains no certificate.",
            )));
        }
        let key = PrivateKeyDer::from_pem_slice(key_pem.as_bytes())
            .map_err(|e| invalid("client key", &e))?;

        let config =
            ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
                .with_safe_default_protocol_versions()
                .map_err(|e| invalid("TLS configuration", &e))?
                .with_root_certificates(roots)
                .with_client_auth_cert(chain, key)
                .map_err(|e| invalid("client certificate and key", &e))?;

        let host = address
            .rsplit_once(':')
            .map_or(address, |(host, _)| host)
            .trim_start_matches('[')
            .trim_end_matches(']');
        let server_name =
            ServerName::try_from(host.to_owned()).map_err(|e| invalid("server address", &e))?;

        Ok(Self {
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

fn material_invalid(message: impl Into<String>) -> AppError {
    AppError::new(ErrorCode::TlsMaterialInvalid, message).with_hint(
        "Paste PEM files (ca.pem, cert.pem, key.pem) made for this Engine on the host page.",
    )
}

impl Dialer for TlsDialer {
    fn connect(&self) -> BoxFuture<'_, Result<(), ConnectError>> {
        Box::pin(async move {
            match self.open().await {
                Ok(_) => Ok(()),
                // rustls reports handshake and certificate failures as InvalidData.
                Err(e) if e.kind() == io::ErrorKind::InvalidData => {
                    Err(ConnectError::Unauthorized(
                        AppError::new(ErrorCode::TlsHandshakeFailed, "The TLS handshake failed.")
                            .with_cause(&e),
                    ))
                }
                Err(e) => Err(ConnectError::Unreachable(
                    AppError::new(
                        ErrorCode::ConnectionRefused,
                        format!("{} can't be reached.", self.address),
                    )
                    .with_cause(&e),
                )),
            }
        })
    }

    fn dial(&self) -> BoxFuture<'_, io::Result<BoxStream>> {
        Box::pin(self.open())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_text_that_is_not_a_certificate() {
        let err = TlsDialer::new("host:2376", "nope", "nope", "nope")
            .err()
            .unwrap();
        assert!(matches!(err, ConnectError::Invalid(e) if e.message.contains("CA certificate")));
    }
}

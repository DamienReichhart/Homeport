//! The TLS configuration Homeport reaches a Nomad agent with: its CA (or
//! this computer's), an optional client certificate, and an optional name
//! to check the agent's certificate against, as `NOMAD_CACERT`,
//! `NOMAD_CLIENT_CERT`, `NOMAD_CLIENT_KEY` and `NOMAD_TLS_SERVER_NAME` set
//! them for the `nomad` CLI.

use std::sync::Arc;

use rustls::{
    client::{
        danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
        WebPkiServerVerifier,
    },
    crypto::CryptoProvider,
    pki_types::{CertificateDer, PrivateKeyDer, ServerName, UnixTime},
    ClientConfig, DigitallySignedStruct, RootCertStore, SignatureScheme,
};
use rustls_pki_types::pem::PemObject;

use homeport_core::error::{AppError, AppResult, ErrorCode, Message, Text};

use crate::messages as msg;

/// What the TLS configuration is made of.
pub(crate) struct TlsMaterial<'a> {
    pub name: &'a str,
    pub certificate_authority: Option<&'a str>,
    pub client_certificate: Option<&'a str>,
    pub client_key: Option<&'a str>,
    pub server_name: Option<&'a str>,
}

fn provider() -> Arc<CryptoProvider> {
    Arc::new(rustls::crypto::ring::default_provider())
}

fn unusable(message: Text, hint: Message, cause: &dyn std::fmt::Display) -> AppError {
    AppError::new(ErrorCode::TlsMaterialInvalid, message)
        .with_hint(hint)
        .with_detail(cause)
}

/// Empty text counts as none.
fn given(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|v| !v.is_empty())
}

/// The client configuration for `material`; a user-facing error when a
/// certificate or key can't be used.
pub(crate) fn config(material: &TlsMaterial<'_>) -> AppResult<ClientConfig> {
    let provider = provider();
    let name = material.name;
    let verifier: Arc<dyn ServerCertVerifier> = match given(material.certificate_authority) {
        // Only the CA given, as the nomad CLI does with NOMAD_CACERT.
        Some(ca) => {
            let ca_unusable = |e: &dyn std::fmt::Display| {
                unusable(
                    msg::CA_UNUSABLE.with("name", name),
                    msg::CA_UNUSABLE_HINT,
                    e,
                )
            };
            let mut roots = RootCertStore::empty();
            for cert in CertificateDer::pem_slice_iter(ca.as_bytes()) {
                let cert = cert.map_err(|e| ca_unusable(&e))?;
                roots.add(cert).map_err(|e| ca_unusable(&e))?;
            }
            if roots.is_empty() {
                return Err(ca_unusable(&"no certificate in the PEM text"));
            }
            WebPkiServerVerifier::builder_with_provider(Arc::new(roots), provider.clone())
                .build()
                .map_err(|e| ca_unusable(&e))?
        }
        None => Arc::new(
            rustls_platform_verifier::Verifier::new(provider.clone())
                .map_err(|e| AppError::new(ErrorCode::Io, msg::HTTPS_CLIENT).with_cause(&e))?,
        ),
    };
    let verifier = match given(material.server_name) {
        Some(server_name) => Arc::new(Renamed {
            name: ServerName::try_from(server_name.to_owned()).map_err(|_| {
                AppError::validation(msg::SERVER_NAME_INVALID.with("serverName", server_name))
            })?,
            inner: verifier,
        }),
        None => verifier,
    };
    let builder = ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .map_err(|e| AppError::new(ErrorCode::Io, msg::HTTPS_CLIENT).with_cause(&e))?
        .dangerous()
        .with_custom_certificate_verifier(verifier);
    let pair_unusable = |e: &dyn std::fmt::Display| {
        unusable(
            msg::CLIENT_PAIR_UNUSABLE.with("name", name),
            msg::CLIENT_PAIR_UNUSABLE_HINT,
            e,
        )
    };
    match (
        given(material.client_certificate),
        given(material.client_key),
    ) {
        (None, None) => Ok(builder.with_no_client_auth()),
        (Some(cert), Some(key)) => {
            let chain = CertificateDer::pem_slice_iter(cert.as_bytes())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| pair_unusable(&e))?;
            if chain.is_empty() {
                return Err(pair_unusable(&"no certificate in the PEM text"));
            }
            let key =
                PrivateKeyDer::from_pem_slice(key.as_bytes()).map_err(|e| pair_unusable(&e))?;
            builder
                .with_client_auth_cert(chain, key)
                .map_err(|e| pair_unusable(&e))
        }
        _ => Err(AppError::new(
            ErrorCode::TlsMaterialInvalid,
            msg::CLIENT_PAIR_INCOMPLETE,
        )),
    }
}

/// Checks the agent's certificate against another name than the address's,
/// such as `server.global.nomad`, as `NOMAD_TLS_SERVER_NAME` does.
#[derive(Debug)]
struct Renamed {
    name: ServerName<'static>,
    inner: Arc<dyn ServerCertVerifier>,
}

impl ServerCertVerifier for Renamed {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        self.inner
            .verify_server_cert(end_entity, intermediates, &self.name, ocsp_response, now)
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        self.inner.verify_tls12_signature(message, cert, dss)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        self.inner.verify_tls13_signature(message, cert, dss)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.inner.supported_verify_schemes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn material<'a>() -> TlsMaterial<'a> {
        TlsMaterial {
            name: "prod",
            certificate_authority: None,
            client_certificate: None,
            client_key: None,
            server_name: None,
        }
    }

    #[test]
    fn no_material_trusts_this_computers_cas() {
        assert!(config(&material()).is_ok());
    }

    #[test]
    fn a_ca_that_isnt_pem_is_unusable() {
        let e = config(&TlsMaterial {
            certificate_authority: Some("not a certificate"),
            ..material()
        })
        .unwrap_err();
        assert!(e.is(ErrorCode::TlsMaterialInvalid));
        assert!(e.message.contains("prod"));
        assert!(e.hint.is_some());
    }

    #[test]
    fn a_client_certificate_needs_its_key() {
        let e = config(&TlsMaterial {
            client_certificate: Some("-----BEGIN CERTIFICATE-----"),
            ..material()
        })
        .unwrap_err();
        assert!(e.is(ErrorCode::TlsMaterialInvalid));
        assert_eq!(e.key, "errors.nomad.clientPairIncomplete");
    }

    #[test]
    fn empty_text_is_no_material() {
        assert!(config(&TlsMaterial {
            certificate_authority: Some("  "),
            client_key: Some(""),
            ..material()
        })
        .is_ok());
    }

    #[test]
    fn a_server_name_must_be_a_name() {
        let e = config(&TlsMaterial {
            server_name: Some("not a name!"),
            ..material()
        })
        .unwrap_err();
        assert!(e.is(ErrorCode::Validation));
        assert!(config(&TlsMaterial {
            server_name: Some("server.global.nomad"),
            ..material()
        })
        .is_ok());
    }
}

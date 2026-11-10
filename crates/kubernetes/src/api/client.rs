//! A kube-rs client for one Cluster, built in memory from its connection and
//! its unsealed credentials: nothing is written to disk.

use std::time::Duration;

use base64::Engine as _;
use rustls_pki_types::{pem::PemObject, CertificateDer};
use secrecy::SecretString;

use homeport_core::error::AppResult;

use crate::api::classify::{classify, credentials_unusable};
use crate::model::{KubeCluster, KubernetesSecrets};

/// How long connecting to an API server may take.
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Credentials an exec plugin printed, used instead of the stored ones.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IssuedCredentials {
    pub token: Option<String>,
    pub client_certificate: Option<String>,
    pub client_key: Option<String>,
}

/// The kube-rs configuration of `cluster`.
pub fn config(
    cluster: &KubeCluster,
    secrets: &KubernetesSecrets,
    issued: Option<&IssuedCredentials>,
) -> AppResult<kube::Config> {
    let connection = &cluster.connection;
    let uri: http::Uri = connection
        .server
        .parse()
        .map_err(|e| credentials_unusable(&cluster.name, e))?;
    let mut config = kube::Config::new(uri);
    config.connect_timeout = Some(CONNECT_TIMEOUT);
    // kube-rs retries 429, 503 and 504 up to 15 times, backing off to 1000 s
    // between tries: a Cluster with one broken aggregated API (a metrics-server
    // that isn't running) froze discovery and its page for minutes. Homeport
    // shows the failure with a retry and reconnects on its own instead.
    config.default_retry = false;
    if let Some(ns) = &connection.namespace {
        config.default_namespace.clone_from(ns);
    }
    config.accept_invalid_certs = connection.insecure_skip_tls_verify;
    config
        .tls_server_name
        .clone_from(&connection.tls_server_name);
    if let Some(proxy) = &connection.proxy_url {
        config.proxy_url = Some(
            proxy
                .parse()
                .map_err(|e| credentials_unusable(&cluster.name, e))?,
        );
    }
    if let Some(ca) = &secrets.certificate_authority {
        let certs = CertificateDer::pem_slice_iter(ca.as_bytes())
            .map(|c| c.map(|c| c.to_vec()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| credentials_unusable(&cluster.name, e))?;
        if certs.is_empty() {
            return Err(credentials_unusable(
                &cluster.name,
                "no certificate in the CA data",
            ));
        }
        config.root_cert = Some(certs);
    }

    let b64 = |pem: &str| base64::engine::general_purpose::STANDARD.encode(pem);
    let token = issued
        .and_then(|i| i.token.clone())
        .or(secrets.token.clone());
    let (cert, key) = match issued {
        Some(IssuedCredentials {
            client_certificate: Some(cert),
            client_key: Some(key),
            ..
        }) => (Some(cert.clone()), Some(key.clone())),
        _ => (
            secrets.client_certificate.clone(),
            secrets.client_key.clone(),
        ),
    };
    let auth = &mut config.auth_info;
    auth.token = token.map(SecretString::from);
    auth.username.clone_from(&secrets.username);
    auth.password = secrets.password.clone().map(SecretString::from);
    auth.client_certificate_data = cert.as_deref().map(b64);
    auth.client_key_data = key.as_deref().map(|k| SecretString::from(b64(k)));
    if let Some(oidc) = &secrets.oidc {
        // kube-rs refreshes the ID token in memory; nothing is written back.
        auth.auth_provider = Some(kube::config::AuthProviderConfig {
            name: "oidc".into(),
            config: oidc.clone().into_iter().collect(),
            other: Default::default(),
        });
    }
    Ok(config)
}

/// A client for `cluster`. Must run inside a Tokio runtime.
pub fn client(
    cluster: &KubeCluster,
    secrets: &KubernetesSecrets,
    issued: Option<&IssuedCredentials>,
) -> AppResult<kube::Client> {
    let config = config(cluster, secrets, issued)?;
    kube::Client::try_from(config).map_err(|e| match e {
        kube::Error::RustlsTls(_) | kube::Error::Auth(_) => credentials_unusable(&cluster.name, &e),
        e => classify(&cluster.name, cluster.address(), &e),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{KubeCluster, KubernetesConnection};

    fn cluster() -> KubeCluster {
        KubeCluster {
            id: "c".into(),
            name: "kind-dev".into(),
            connection: KubernetesConnection {
                server: "https://127.0.0.1:6443".into(),
                namespace: Some("shop".into()),
                ..Default::default()
            },
        }
    }

    #[test]
    fn the_context_namespace_is_the_default() {
        let c = config(&cluster(), &KubernetesSecrets::default(), None).unwrap();
        assert_eq!(c.default_namespace, "shop");
        assert_eq!(c.cluster_url.to_string(), "https://127.0.0.1:6443/");
    }

    #[test]
    fn issued_credentials_replace_the_stored_token() {
        let secrets = KubernetesSecrets {
            token: Some("stored".into()),
            ..Default::default()
        };
        let issued = IssuedCredentials {
            token: Some("fresh".into()),
            ..Default::default()
        };
        let c = config(&cluster(), &secrets, Some(&issued)).unwrap();
        use secrecy::ExposeSecret;
        assert_eq!(c.auth_info.token.unwrap().expose_secret(), "fresh");
    }

    #[test]
    fn a_ca_that_isnt_pem_is_unusable() {
        let secrets = KubernetesSecrets {
            certificate_authority: Some("not a certificate".into()),
            ..Default::default()
        };
        let e = config(&cluster(), &secrets, None).unwrap_err();
        assert!(e.is(homeport_core::error::ErrorCode::TlsMaterialInvalid));
    }

    #[test]
    fn failed_requests_are_reported_not_retried_for_minutes() {
        let config = config(&cluster(), &KubernetesSecrets::default(), None).unwrap();
        assert!(!config.default_retry);
    }
}

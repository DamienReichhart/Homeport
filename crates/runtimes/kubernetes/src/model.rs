//! A Kubernetes Cluster as this crate needs it: who it is, how its API server
//! is reached, and the credentials its kubeconfig gives, which are only ever
//! stored sealed (ADR 0002).

use serde::{Deserialize, Serialize};

/// How to reach a Kubernetes Cluster's API server, without any credential.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct KubernetesConnection {
    /// The API server's URL, such as `https://127.0.0.1:6443`.
    pub server: String,
    /// The name to check the server's certificate against, when not the URL's.
    pub tls_server_name: Option<String>,
    /// The kubeconfig turns off checking the server's certificate.
    pub insecure_skip_tls_verify: bool,
    /// An HTTP proxy in front of the API server.
    pub proxy_url: Option<String>,
    /// The Namespace the kubeconfig context defaults to.
    pub namespace: Option<String>,
    /// How Homeport authenticates, to tell the user; the credentials themselves are sealed.
    pub auth: AuthMethod,
    /// The kubeconfig context it was imported from, for refreshing it.
    pub source: Option<KubeconfigSource>,
}

/// How a Kubernetes Cluster's user authenticates, as its kubeconfig says.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, specta::Type)]
#[serde(
    tag = "method",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum AuthMethod {
    /// No credential: the API server sees an anonymous user.
    #[default]
    None,
    ClientCertificate,
    Token,
    Basic {
        username: String,
    },
    /// A program on this machine prints the credentials (`aws`, `gke-gcloud-auth-plugin`…).
    Exec {
        command: String,
    },
    /// An OpenID Connect ID token, refreshed with its refresh token.
    Oidc,
}

/// Where a Cluster's kubeconfig context came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct KubeconfigSource {
    /// The context's name in the kubeconfig.
    pub context: String,
    /// Pasted into Homeport rather than read from this machine's kubeconfig,
    /// so it can't be refreshed from there.
    pub pasted: bool,
}

/// A Kubernetes Cluster's credentials, as the kubeconfig gave them (files
/// inlined). Never sent to the frontend, never written to disk unsealed.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct KubernetesSecrets {
    /// PEM certificates of the CAs the API server's certificate is checked against.
    pub certificate_authority: Option<String>,
    /// PEM client certificate and key.
    pub client_certificate: Option<String>,
    pub client_key: Option<String>,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    /// The exec credential plugin and its environment.
    pub exec: Option<ExecConfig>,
    /// The `oidc` auth provider's settings: its ID and refresh tokens, issuer and client.
    pub oidc: Option<std::collections::BTreeMap<String, String>>,
}

/// An exec credential plugin, as `client.authentication.k8s.io` describes it.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ExecConfig {
    pub api_version: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub install_hint: Option<String>,
    /// `Never`, `IfAvailable` or `Always`.
    pub interactive_mode: Option<String>,
    pub provide_cluster_info: bool,
}

/// A Kubernetes Cluster: who it is and how its API server is reached.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KubeCluster {
    /// What Homeport knows it by.
    pub id: String,
    /// What the user calls it, for messages.
    pub name: String,
    pub connection: KubernetesConnection,
}

impl KubeCluster {
    /// The URL of its API server, for messages.
    pub fn address(&self) -> &str {
        &self.connection.server
    }
}

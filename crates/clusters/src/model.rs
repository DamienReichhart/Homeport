//! What Homeport keeps about a Cluster: the clear part shown to the user and
//! stored as JSON, and the credentials, only ever stored sealed (ADR 0002).

use serde::{Deserialize, Serialize};

pub type ClusterId = String;

/// The orchestrator a Cluster runs. Nomad Clusters will share the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum ClusterKind {
    Kubernetes,
}

impl ClusterKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Kubernetes => "kubernetes",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        (s == "kubernetes").then_some(Self::Kubernetes)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub id: ClusterId,
    pub name: String,
    pub kind: ClusterKind,
    pub connection: ClusterConnection,
    pub enabled: bool,
    pub created_at: String,
}

/// How to reach a Cluster's API, without any credential.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct ClusterConnection {
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

/// How a Cluster's user authenticates, as its kubeconfig says.
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

/// A Cluster's credentials, as the kubeconfig gave them (files inlined).
/// Never sent to the frontend, never written to disk unsealed.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ClusterSecrets {
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

impl ClusterSecrets {
    /// Whether there is anything to seal.
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

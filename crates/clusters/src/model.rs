//! What Homeport keeps about a Cluster: the clear part shown to the user and
//! stored as JSON, and the credentials, only ever stored sealed (ADR 0002).

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;

pub type ClusterId = String;

/// What a Cluster runs, which decides what Homeport can show and do on it.
/// Both kinds share the `clusters` table, told apart by its `kind` column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum ClusterKind {
    Kubernetes,
    Nomad,
}

impl ClusterKind {
    /// Its stored and serialized name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Kubernetes => "kubernetes",
            Self::Nomad => "nomad",
        }
    }

    /// Its name for the user, in messages.
    pub fn label(self) -> &'static str {
        match self {
            Self::Kubernetes => "Kubernetes",
            Self::Nomad => "Nomad",
        }
    }

    /// Reads a stored name; `None` for one this version doesn't know.
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "kubernetes" => Some(Self::Kubernetes),
            "nomad" => Some(Self::Nomad),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub id: ClusterId,
    pub name: String,
    /// Always its connection's kind, which the store keeps in its own column.
    pub kind: ClusterKind,
    pub connection: ClusterConnection,
    pub enabled: bool,
    pub created_at: String,
}

impl Cluster {
    /// How a Kubernetes Cluster is reached; `None` for another kind.
    pub fn kubernetes(&self) -> Option<&KubernetesConnection> {
        match &self.connection {
            ClusterConnection::Kubernetes(c) => Some(c),
            ClusterConnection::Nomad(_) => None,
        }
    }

    /// How a Nomad Cluster is reached; `None` for another kind.
    pub fn nomad(&self) -> Option<&NomadConnection> {
        match &self.connection {
            ClusterConnection::Nomad(c) => Some(c),
            ClusterConnection::Kubernetes(_) => None,
        }
    }

    /// How a Kubernetes Cluster is reached; refused for another kind.
    pub fn kubernetes_or_refuse(&self) -> AppResult<&KubernetesConnection> {
        self.kubernetes().ok_or_else(|| self.wrong_kind())
    }

    /// How a Nomad Cluster is reached; refused for another kind.
    pub fn nomad_or_refuse(&self) -> AppResult<&NomadConnection> {
        self.nomad().ok_or_else(|| self.wrong_kind())
    }

    /// Why a command made for another kind of Cluster can't run on this one.
    pub fn wrong_kind(&self) -> AppError {
        AppError::new(
            ErrorCode::Validation,
            msg::WRONG_KIND
                .with("name", &self.name)
                .with("kind", self.kind.label()),
        )
        .with_hint(msg::WRONG_KIND_HINT)
    }

    /// The address of its API, for messages.
    pub fn address(&self) -> &str {
        match &self.connection {
            ClusterConnection::Kubernetes(c) => &c.server,
            ClusterConnection::Nomad(c) => &c.address,
        }
    }
}

/// How to reach a Cluster's API, by kind, without any credential. Stored
/// as the variant's own JSON, its kind in a column of its own; sent to the
/// frontend tagged with `kind`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ClusterConnection {
    Kubernetes(KubernetesConnection),
    Nomad(NomadConnection),
}

impl ClusterConnection {
    pub fn kind(&self) -> ClusterKind {
        match self {
            Self::Kubernetes(_) => ClusterKind::Kubernetes,
            Self::Nomad(_) => ClusterKind::Nomad,
        }
    }

    /// The variant's own JSON, as the store keeps it beside the kind.
    pub fn to_stored(&self) -> serde_json::Result<String> {
        match self {
            Self::Kubernetes(c) => serde_json::to_string(c),
            Self::Nomad(c) => serde_json::to_string(c),
        }
    }

    /// Reads a stored connection of `kind`.
    pub fn from_stored(kind: ClusterKind, json: &str) -> serde_json::Result<Self> {
        Ok(match kind {
            ClusterKind::Kubernetes => Self::Kubernetes(serde_json::from_str(json)?),
            ClusterKind::Nomad => Self::Nomad(serde_json::from_str(json)?),
        })
    }
}

/// How to reach a Nomad Cluster's HTTP API, without any credential.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct NomadConnection {
    /// The API's URL, such as `https://nomad.example.com:4646`.
    pub address: String,
}

/// A Nomad Cluster's credentials. Never sent to the frontend, never written
/// to disk unsealed.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NomadSecrets {}

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

/// A Cluster's credentials, by kind. Sealed as the variant's own JSON.
// Held only while connecting, one at a time: boxing would buy nothing.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClusterSecrets {
    Kubernetes(KubernetesSecrets),
    Nomad(NomadSecrets),
}

impl ClusterSecrets {
    /// No credential of `kind`.
    pub fn none(kind: ClusterKind) -> Self {
        match kind {
            ClusterKind::Kubernetes => Self::Kubernetes(KubernetesSecrets::default()),
            ClusterKind::Nomad => Self::Nomad(NomadSecrets::default()),
        }
    }

    pub fn kind(&self) -> ClusterKind {
        match self {
            Self::Kubernetes(_) => ClusterKind::Kubernetes,
            Self::Nomad(_) => ClusterKind::Nomad,
        }
    }

    /// Whether there is anything to seal.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Kubernetes(s) => *s == KubernetesSecrets::default(),
            Self::Nomad(s) => *s == NomadSecrets::default(),
        }
    }

    /// The variant's own JSON value, as it is sealed and backed up.
    pub fn to_value(&self) -> serde_json::Result<serde_json::Value> {
        match self {
            Self::Kubernetes(s) => serde_json::to_value(s),
            Self::Nomad(s) => serde_json::to_value(s),
        }
    }

    /// Reads the sealed or backed-up credentials of `kind`.
    pub fn from_value(kind: ClusterKind, value: serde_json::Value) -> serde_json::Result<Self> {
        Ok(match kind {
            ClusterKind::Kubernetes => Self::Kubernetes(serde_json::from_value(value)?),
            ClusterKind::Nomad => Self::Nomad(serde_json::from_value(value)?),
        })
    }
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

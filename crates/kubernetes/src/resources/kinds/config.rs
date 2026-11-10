//! ConfigMaps and Secrets. A Secret's values never leave the backend in a
//! list or a detail: only [`crate::resources::On::reveal`] returns one, for one key.

use std::collections::BTreeMap;

use k8s_openapi::api::core::v1::{ConfigMap, Secret};
use kube::api::ListParams;
use serde::Serialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::config;
use crate::resources::common::{annotations, created, labels};
use crate::{
    messages as msg,
    resources::{sorted, On, ResourceKind},
};

/// One key of a ConfigMap or Secret, and how big its value is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DataKey {
    pub key: String,
    /// Bytes.
    pub size: u32,
    /// Binary data, which isn't shown as text.
    pub binary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapView {
    pub namespace: String,
    pub name: String,
    pub keys: Vec<DataKey>,
    pub created: Option<String>,
}

/// A ConfigMap's value; binary ones have none.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ConfigValue {
    pub key: String,
    pub value: Option<String>,
    pub size: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapDetail {
    pub view: ConfigMapView,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
    pub values: Vec<ConfigValue>,
}

/// A Secret, without any of its values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SecretView {
    pub namespace: String,
    pub name: String,
    /// `Opaque`, `kubernetes.io/tls`, `kubernetes.io/dockerconfigjson`…
    #[serde(rename = "type")]
    pub kind: String,
    pub keys: Vec<DataKey>,
    pub created: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SecretDetail {
    pub view: SecretView,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
}

/// One revealed value of a Secret, decoded; binary ones as base64.
#[derive(Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RevealedValue {
    pub value: String,
    pub binary: bool,
}

impl std::fmt::Debug for RevealedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RevealedValue").finish_non_exhaustive()
    }
}

fn size(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

pub fn config_map(c: &ConfigMap) -> ConfigMapView {
    let mut keys: Vec<DataKey> = c
        .data
        .iter()
        .flatten()
        .map(|(k, v)| DataKey {
            key: k.clone(),
            size: size(v.len()),
            binary: false,
        })
        .collect();
    keys.extend(c.binary_data.iter().flatten().map(|(k, v)| DataKey {
        key: k.clone(),
        size: size(v.0.len()),
        binary: true,
    }));
    keys.sort_by(|a, b| a.key.cmp(&b.key));
    ConfigMapView {
        namespace: c.metadata.namespace.clone().unwrap_or_default(),
        name: c.metadata.name.clone().unwrap_or_default(),
        keys,
        created: created(&c.metadata),
    }
}

pub fn config_map_detail(c: &ConfigMap) -> ConfigMapDetail {
    let view = config_map(c);
    let values = view
        .keys
        .iter()
        .map(|k| ConfigValue {
            key: k.key.clone(),
            value: (!k.binary)
                .then(|| c.data.as_ref().and_then(|d| d.get(&k.key)).cloned())
                .flatten(),
            size: k.size,
        })
        .collect();
    ConfigMapDetail {
        view,
        labels: labels(&c.metadata),
        annotations: annotations(&c.metadata),
        values,
    }
}

/// A Secret's keys and their sizes, never their values.
pub fn secret(s: &Secret) -> SecretView {
    let mut keys: Vec<DataKey> = s
        .data
        .iter()
        .flatten()
        .map(|(k, v)| DataKey {
            key: k.clone(),
            size: size(v.0.len()),
            binary: std::str::from_utf8(&v.0).is_err(),
        })
        .collect();
    keys.sort_by(|a, b| a.key.cmp(&b.key));
    SecretView {
        namespace: s.metadata.namespace.clone().unwrap_or_default(),
        name: s.metadata.name.clone().unwrap_or_default(),
        kind: s.type_.clone().unwrap_or_else(|| "Opaque".into()),
        keys,
        created: created(&s.metadata),
    }
}

pub fn secret_detail(s: &Secret) -> SecretDetail {
    SecretDetail {
        view: secret(s),
        labels: labels(&s.metadata),
        annotations: annotations(&s.metadata),
    }
}

/// The value of `key`, decoded, `None` when the Secret has no such key.
pub fn value(s: &Secret, key: &str) -> Option<RevealedValue> {
    use base64::Engine as _;
    let bytes = &s.data.as_ref()?.get(key)?.0;
    Some(match std::str::from_utf8(bytes) {
        Ok(text) => RevealedValue {
            value: text.to_owned(),
            binary: false,
        },
        Err(_) => RevealedValue {
            value: base64::engine::general_purpose::STANDARD.encode(bytes),
            binary: true,
        },
    })
}

impl On {
    pub async fn config_maps(&self, namespace: Option<&str>) -> AppResult<Vec<ConfigMapView>> {
        let items: Vec<ConfigMap> = self
            .list(
                self.api(namespace),
                ResourceKind::ConfigMap,
                namespace,
                ListParams::default(),
            )
            .await?;
        Ok(sorted(
            items.iter().map(config::config_map).collect(),
            |c| (c.namespace.clone(), c.name.clone()),
        ))
    }

    pub async fn config_map(&self, namespace: &str, name: &str) -> AppResult<ConfigMapDetail> {
        let ns = Some(namespace);
        let c: ConfigMap = self
            .get(self.api(ns), ResourceKind::ConfigMap, ns, name)
            .await?;
        Ok(config::config_map_detail(&c))
    }

    /// Secrets' names, types and keys; never a value.
    pub async fn secrets(&self, namespace: Option<&str>) -> AppResult<Vec<SecretView>> {
        let items: Vec<Secret> = self
            .list(
                self.api(namespace),
                ResourceKind::Secret,
                namespace,
                ListParams::default(),
            )
            .await?;
        Ok(sorted(items.iter().map(config::secret).collect(), |s| {
            (s.namespace.clone(), s.name.clone())
        }))
    }

    pub async fn secret(&self, namespace: &str, name: &str) -> AppResult<SecretDetail> {
        let ns = Some(namespace);
        let s: Secret = self
            .get(self.api(ns), ResourceKind::Secret, ns, name)
            .await?;
        Ok(config::secret_detail(&s))
    }

    /// One value of a Secret, decoded, for the user who asked to see it.
    pub async fn reveal(&self, namespace: &str, name: &str, key: &str) -> AppResult<RevealedValue> {
        let ns = Some(namespace);
        let s: Secret = self
            .get(self.api(ns), ResourceKind::Secret, ns, name)
            .await?;
        config::value(&s, key).ok_or_else(|| {
            AppError::new(
                ErrorCode::NotFound,
                msg::SECRET_KEY_GONE.with("key", key).with("name", name),
            )
        })
    }
}

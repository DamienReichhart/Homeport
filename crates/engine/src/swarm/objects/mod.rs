//! A Swarm's secrets and configs: listing, creating and removing them,
//! through a manager. A secret's value goes to the Engine once and is never
//! read back (the Engine doesn't return it); Homeport keeps none of it
//! (ADR 0016). A config's content can be read back.

use std::{collections::HashMap, fmt};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use bollard::{
    models::{ConfigSpec, SecretSpec, Service},
    query_parameters::{ListConfigsOptions, ListSecretsOptions, ListServicesOptions},
    Docker,
};
use serde::{Deserialize, Serialize};

use homeport_core::error::{engine::from_bollard, local_file, AppError, AppResult, ErrorCode};

use crate::messages as msg;

/// The largest secret or config the Swarm accepts (500 KB).
pub const MAX_SIZE: usize = 500 * 1024;

/// What a secret or config is, as the lists show it. Never its value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SwarmObject {
    pub id: String,
    pub name: String,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub labels: HashMap<String, String>,
    /// The names of the services that use it.
    pub services: Vec<String>,
}

/// Where a new secret's or config's value comes from.
#[derive(Clone, Deserialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ObjectValue {
    /// Typed in Homeport.
    Text { text: String },
    /// A file on this computer, read here and sent; its path is all the
    /// webview hands over.
    File { path: String },
}

/// Never shows a value: it may be a secret.
impl fmt::Debug for ObjectValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text { text } => write!(f, "Text {{ {} bytes }}", text.len()),
            Self::File { path } => write!(f, "File {{ path: {path:?} }}"),
        }
    }
}

/// A secret or config to create.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ObjectInput {
    pub name: String,
    pub value: ObjectValue,
    pub labels: HashMap<String, String>,
}

/// A config's content as text, or its size when it isn't text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ConfigContent {
    pub text: Option<String>,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Secret,
    Config,
}

/// The services using each secret or config, by object ID.
fn users(services: &[Service], kind: Kind) -> HashMap<String, Vec<String>> {
    let mut users: HashMap<String, Vec<String>> = HashMap::new();
    for service in services {
        let spec = service.spec.clone().unwrap_or_default();
        let name = spec.name.unwrap_or_default();
        let container = spec
            .task_template
            .and_then(|t| t.container_spec)
            .unwrap_or_default();
        let ids: Vec<String> = match kind {
            Kind::Secret => container
                .secrets
                .unwrap_or_default()
                .into_iter()
                .filter_map(|s| s.secret_id)
                .collect(),
            Kind::Config => container
                .configs
                .unwrap_or_default()
                .into_iter()
                .filter_map(|c| c.config_id)
                .collect(),
        };
        for id in ids {
            let list = users.entry(id).or_default();
            if !list.contains(&name) {
                list.push(name.clone());
            }
        }
    }
    users
}

pub async fn secrets(docker: &Docker) -> AppResult<Vec<SwarmObject>> {
    let services = docker.list_services(None::<ListServicesOptions>).await?;
    let mut used = users(&services, Kind::Secret);
    let mut list: Vec<SwarmObject> = docker
        .list_secrets(None::<ListSecretsOptions>)
        .await?
        .into_iter()
        .map(|s| {
            let spec = s.spec.unwrap_or_default();
            let id = s.id.unwrap_or_default();
            SwarmObject {
                services: used.remove(&id).unwrap_or_default(),
                id,
                name: spec.name.unwrap_or_default(),
                created: s.created_at.map(|d| d.to_string()),
                updated: s.updated_at.map(|d| d.to_string()),
                labels: spec.labels.unwrap_or_default(),
            }
        })
        .collect();
    list.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(list)
}

pub async fn configs(docker: &Docker) -> AppResult<Vec<SwarmObject>> {
    let services = docker.list_services(None::<ListServicesOptions>).await?;
    let mut used = users(&services, Kind::Config);
    let mut list: Vec<SwarmObject> = docker
        .list_configs(None::<ListConfigsOptions>)
        .await?
        .into_iter()
        .map(|c| {
            let spec = c.spec.unwrap_or_default();
            let id = c.id.unwrap_or_default();
            SwarmObject {
                services: used.remove(&id).unwrap_or_default(),
                id,
                name: spec.name.unwrap_or_default(),
                created: c.created_at.map(|d| d.to_string()),
                updated: c.updated_at.map(|d| d.to_string()),
                labels: spec.labels.unwrap_or_default(),
            }
        })
        .collect();
    list.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(list)
}

/// Checks a new object's name and reads its value, from the text or the
/// file, as the base64 the Engine takes.
fn prepare(input: &ObjectInput) -> AppResult<String> {
    let name = input.name.trim();
    let valid = !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        && name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphanumeric());
    if !valid {
        return Err(AppError::validation(msg::OBJECT_NAME));
    }
    let bytes = match &input.value {
        ObjectValue::Text { text } => text.as_bytes().to_vec(),
        ObjectValue::File { path } => {
            let path = std::path::Path::new(path);
            // Read here, sent once, never kept.
            std::fs::read(path).map_err(|e| local_file(path, e))?
        }
    };
    if bytes.is_empty() {
        return Err(AppError::validation(msg::OBJECT_EMPTY));
    }
    if bytes.len() > MAX_SIZE {
        return Err(AppError::validation(msg::OBJECT_TOO_LARGE));
    }
    Ok(STANDARD.encode(bytes))
}

/// Turns the Engine's refusals into what the user can act on: a name in use,
/// or an object a service still uses.
fn object_error(e: &bollard::errors::Error, services: &[String]) -> AppError {
    let text = e.to_string().to_lowercase();
    if text.contains("already exists") {
        return AppError::new(ErrorCode::NameInUse, msg::OBJECT_NAME_IN_USE)
            .with_hint(msg::OBJECT_NAME_IN_USE_HINT)
            .with_cause(e);
    }
    if text.contains("in use") {
        let error = AppError::new(ErrorCode::InUse, msg::OBJECT_IN_USE).with_cause(e);
        return if services.is_empty() {
            error.with_hint(msg::OBJECT_IN_USE_HINT_UNKNOWN)
        } else {
            error.with_hint(msg::OBJECT_IN_USE_HINT.with("services", services.join(", ")))
        };
    }
    from_bollard(e)
}

pub async fn create_secret(docker: &Docker, input: ObjectInput) -> AppResult<String> {
    let data = prepare(&input)?;
    let created = docker
        .create_secret(SecretSpec {
            name: Some(input.name.trim().to_owned()),
            labels: Some(input.labels),
            data: Some(data),
            ..Default::default()
        })
        .await
        .map_err(|e| object_error(&e, &[]))?;
    Ok(created.id)
}

pub async fn create_config(docker: &Docker, input: ObjectInput) -> AppResult<String> {
    let data = prepare(&input)?;
    let created = docker
        .create_config(ConfigSpec {
            name: Some(input.name.trim().to_owned()),
            labels: Some(input.labels),
            data: Some(data),
            ..Default::default()
        })
        .await
        .map_err(|e| object_error(&e, &[]))?;
    Ok(created.id)
}

pub async fn remove_secret(docker: &Docker, id: &str) -> AppResult<()> {
    if let Err(e) = docker.delete_secret(id).await {
        let using = secrets(docker)
            .await
            .ok()
            .and_then(|l| l.into_iter().find(|s| s.id == id))
            .map(|s| s.services)
            .unwrap_or_default();
        return Err(object_error(&e, &using));
    }
    Ok(())
}

pub async fn remove_config(docker: &Docker, id: &str) -> AppResult<()> {
    if let Err(e) = docker.delete_config(id).await {
        let using = configs(docker)
            .await
            .ok()
            .and_then(|l| l.into_iter().find(|c| c.id == id))
            .map(|c| c.services)
            .unwrap_or_default();
        return Err(object_error(&e, &using));
    }
    Ok(())
}

/// A config's content, decoded: text when it's UTF-8, else only its size.
pub async fn config_content(docker: &Docker, id: &str) -> AppResult<ConfigContent> {
    let config = docker.inspect_config(id).await?;
    let data = config.spec.and_then(|s| s.data).unwrap_or_default();
    Ok(decode(&data))
}

pub fn decode(data: &str) -> ConfigContent {
    let bytes = STANDARD.decode(data.trim()).unwrap_or_default();
    ConfigContent {
        size: bytes.len() as u64,
        text: String::from_utf8(bytes).ok(),
    }
}

#[cfg(test)]
mod tests;

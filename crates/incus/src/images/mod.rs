//! The images an Incus Host keeps: listing them, copying one from a remote
//! image server with its progress, refreshing, aliasing and deleting them.
//! Deleting needs no check for users: Instances keep their own copy.

use serde::{Deserialize, Serialize};

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    pull::PullEvent,
};
use homeport_transport::http::Request;

use crate::{
    client::{scoped, segment, Incus},
    instances::InstanceType,
    messages as msg,
    operations::{self, Operation},
};

/// One image kept on the Host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusImage {
    pub fingerprint: String,
    pub aliases: Vec<String>,
    /// "Debian 12 amd64 (20261108_05:24)".
    pub description: String,
    #[serde(rename = "type")]
    pub kind: InstanceType,
    pub architecture: String,
    pub size: i64,
    pub os: String,
    pub release: String,
    pub variant: String,
    /// RFC 3339.
    pub uploaded_at: String,
    pub last_used_at: Option<String>,
    /// Whether Incus keeps it up to date from where it came from.
    pub auto_update: bool,
    /// Whether Incus downloaded it to make an Instance, and deletes it once unused.
    pub cached: bool,
    pub public: bool,
    /// The server and alias it was copied from, if any.
    pub source: Option<ImageSource>,
}

/// Where an image came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImageSource {
    pub server: String,
    pub alias: String,
}

#[derive(Deserialize)]
struct RawImage {
    fingerprint: String,
    #[serde(default)]
    aliases: Vec<RawAlias>,
    #[serde(default)]
    properties: std::collections::BTreeMap<String, String>,
    #[serde(rename = "type", default)]
    kind: String,
    #[serde(default)]
    architecture: String,
    #[serde(default)]
    size: i64,
    #[serde(default)]
    uploaded_at: String,
    #[serde(default)]
    last_used_at: String,
    #[serde(default)]
    auto_update: bool,
    #[serde(default)]
    cached: bool,
    #[serde(default)]
    public: bool,
    #[serde(default)]
    update_source: Option<RawSource>,
}

#[derive(Deserialize)]
struct RawAlias {
    name: String,
}

#[derive(Deserialize)]
struct RawSource {
    #[serde(default)]
    server: String,
    #[serde(default)]
    alias: String,
}

impl RawImage {
    fn into_image(self) -> IncusImage {
        let property = |name: &str| self.properties.get(name).cloned().unwrap_or_default();
        IncusImage {
            aliases: self.aliases.iter().map(|a| a.name.clone()).collect(),
            description: property("description"),
            kind: if self.kind == "virtual-machine" {
                InstanceType::VirtualMachine
            } else {
                InstanceType::Container
            },
            architecture: self.architecture.clone(),
            size: self.size,
            os: property("os"),
            release: property("release"),
            variant: property("variant"),
            uploaded_at: self.uploaded_at.clone(),
            last_used_at: (!self.last_used_at.is_empty()
                && !self.last_used_at.starts_with("0001-"))
            .then(|| self.last_used_at.clone()),
            auto_update: self.auto_update,
            cached: self.cached,
            public: self.public,
            source: self
                .update_source
                .as_ref()
                .filter(|s| !s.server.is_empty())
                .map(|s| ImageSource {
                    server: s.server.clone(),
                    alias: s.alias.clone(),
                }),
            fingerprint: self.fingerprint,
        }
    }
}

fn path(fingerprint: &str) -> String {
    format!("/1.0/images/{}", segment(fingerprint))
}

/// Every image of `project`, newest first.
pub async fn list(incus: &Incus, project: &str) -> AppResult<Vec<IncusImage>> {
    let raw: Vec<RawImage> = incus
        .sync(Request::get(scoped("/1.0/images?recursion=1", project)))
        .await?;
    let mut images: Vec<IncusImage> = raw.into_iter().map(RawImage::into_image).collect();
    images.sort_by(|a, b| b.uploaded_at.cmp(&a.uploaded_at));
    Ok(images)
}

/// What to copy from a remote image server.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImageCopy {
    pub server: String,
    pub alias: String,
    #[serde(rename = "type")]
    pub kind: InstanceType,
    /// Keep it up to date from the server.
    pub auto_update: bool,
    /// Also give it the remote alias here, so Instances can be made from it by name.
    pub add_alias: bool,
}

/// The body of `POST /1.0/images` for `copy`.
pub(crate) fn copy_body(copy: &ImageCopy) -> serde_json::Value {
    let mut body = serde_json::json!({
        "source": {
            "type": "image",
            "mode": "pull",
            "server": copy.server,
            "protocol": "simplestreams",
            "alias": copy.alias,
            "image_type": instance_type(copy.kind),
        },
        "auto_update": copy.auto_update,
    });
    if copy.add_alias {
        body["aliases"] = serde_json::json!([{ "name": copy.alias }]);
    }
    body
}

pub(crate) fn instance_type(kind: InstanceType) -> &'static str {
    match kind {
        InstanceType::Container => "container",
        InstanceType::VirtualMachine => "virtual-machine",
    }
}

/// Progress of an operation, as a pull's event: Incus words it like
/// "rootfs: 42% (12.30MB/s)".
pub(crate) fn progress_event(operation: &Operation) -> Option<PullEvent> {
    let text = operation.progress()?;
    let percent = text
        .split_whitespace()
        .find_map(|word| word.strip_suffix('%')?.parse::<i64>().ok());
    Some(PullEvent::Progress {
        id: None,
        status: text.to_owned(),
        current: percent,
        total: percent.map(|_| 100),
    })
}

/// Copies an image from a remote image server into `project`, sending
/// its download's progress to `sink` until it's done or failed.
pub async fn copy(
    incus: &Incus,
    project: &str,
    copy: &ImageCopy,
    mut sink: impl FnMut(PullEvent) -> bool,
) {
    let result = async {
        let request = Request::post(scoped("/1.0/images", project)).json(&copy_body(copy))?;
        incus
            .run_with(project, request, operations::LONG, |op| {
                if let Some(event) = progress_event(op) {
                    sink(event);
                }
            })
            .await
    }
    .await;
    match result {
        Ok(_) => {
            sink(PullEvent::Done);
        }
        Err(error) => {
            sink(PullEvent::Error { error });
        }
    }
}

/// Asks Incus to fetch a newer build of the image from where it came from.
pub async fn refresh(incus: &Incus, project: &str, fingerprint: &str) -> AppResult<()> {
    let request = Request::post(scoped(&format!("{}/refresh", path(fingerprint)), project));
    incus
        .run(project, request, operations::LONG)
        .await
        .map(drop)
        .map_err(|e| not_found(e, fingerprint))
}

/// Deletes an image. Instances made from it keep their own copy.
pub async fn delete(incus: &Incus, project: &str, fingerprint: &str) -> AppResult<()> {
    incus
        .run(
            project,
            Request::delete(scoped(&path(fingerprint), project)),
            operations::SHORT,
        )
        .await
        .map(drop)
        .map_err(|e| not_found(e, fingerprint))
}

/// Gives the image `fingerprint` the alias `name` in `project`.
pub async fn add_alias(
    incus: &Incus,
    project: &str,
    fingerprint: &str,
    name: &str,
) -> AppResult<()> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::validation(msg::ALIAS_MISSING));
    }
    let body = serde_json::json!({ "name": name, "target": fingerprint });
    incus
        .sync::<serde_json::Value>(
            Request::post(scoped("/1.0/images/aliases", project)).json(&body)?,
        )
        .await
        .map(drop)
        .map_err(|e| {
            if e.is(ErrorCode::NameInUse) || e.is(ErrorCode::Conflict) {
                AppError::new(ErrorCode::NameInUse, msg::ALIAS_IN_USE.with("name", name))
                    .with_hint(msg::ALIAS_IN_USE_HINT)
                    .with_detail(e.detail.unwrap_or(e.message))
            } else {
                e
            }
        })
}

/// Removes the alias `name` from `project`; the image stays.
pub async fn remove_alias(incus: &Incus, project: &str, name: &str) -> AppResult<()> {
    let path = format!("/1.0/images/aliases/{}", segment(name));
    incus
        .sync::<serde_json::Value>(Request::delete(scoped(&path, project)))
        .await
        .map(drop)
}

fn not_found(error: AppError, fingerprint: &str) -> AppError {
    if error.is(ErrorCode::NotFound) {
        let short: String = fingerprint.chars().take(12).collect();
        AppError::new(
            ErrorCode::NotFound,
            msg::LOCAL_IMAGE_NOT_FOUND.with("fingerprint", short),
        )
        .with_detail(error.detail.unwrap_or(error.message))
    } else {
        error
    }
}

#[cfg(test)]
mod tests;

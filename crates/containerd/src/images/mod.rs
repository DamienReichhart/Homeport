//! A containerd namespace's images: listed with their size from the content
//! store, removed when no container uses them, and pulled through
//! containerd's Transfer service (containerd 2.0 and later, ADR 0020).

mod pull;
mod reference;

use std::collections::HashMap;

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    api::containerd::services::{
        containers::v1::ListContainersRequest,
        content::v1::ReadContentRequest,
        images::v1::{DeleteImageRequest, Image, ListImagesRequest},
    },
    client::{call, stream},
    error,
    introspection::{self, ServerInfo},
    messages as msg, Containerd,
};

pub use pull::{pull, PullCredentials};
pub use reference::qualify;

/// The largest manifest or index Homeport reads to size an image.
const MAX_MANIFEST: i64 = 4 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContainerdImage {
    /// Its full reference, as containerd names it: `docker.io/library/alpine:3.22`.
    pub name: String,
    /// The digest of its index or manifest.
    pub digest: String,
    pub media_type: String,
    /// The bytes of its manifest, config and layers for the Host's platform,
    /// as `ctr images ls` counts them; `None` when they aren't all here.
    pub size: Option<i64>,
    /// The platforms it's published for, `os/arch[/variant]`.
    pub platforms: Vec<String>,
    /// RFC 3339.
    pub created_at: Option<String>,
    pub labels: HashMap<String, String>,
    /// The containers of the namespace made from it.
    pub containers: u32,
}

#[derive(Deserialize)]
struct Platform {
    #[serde(default)]
    os: String,
    #[serde(default)]
    architecture: String,
    #[serde(default)]
    variant: Option<String>,
}

impl Platform {
    fn name(&self) -> String {
        match self.variant.as_deref() {
            Some(v) if !v.is_empty() => format!("{}/{}/{v}", self.os, self.architecture),
            _ => format!("{}/{}", self.os, self.architecture),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
    #[serde(default)]
    digest: String,
    #[serde(default)]
    size: i64,
    #[serde(default)]
    platform: Option<Platform>,
}

/// An OCI index or manifest, or a Docker manifest list or manifest.
#[derive(Deserialize)]
struct Manifest {
    #[serde(default)]
    manifests: Vec<Entry>,
    #[serde(default)]
    config: Option<Entry>,
    #[serde(default)]
    layers: Vec<Entry>,
}

async fn read(client: &Containerd, namespace: &str, digest: &str, size: i64) -> AppResult<Vec<u8>> {
    if size > MAX_MANIFEST {
        return Err(AppError::new(ErrorCode::EngineError, msg::FAILED));
    }
    let mut chunks = client
        .content()
        .read(stream(
            namespace,
            ReadContentRequest {
                digest: digest.to_owned(),
                offset: 0,
                size: 0,
            },
        )?)
        .await
        .map_err(|s| error::status(&s))?
        .into_inner();
    let mut data = Vec::new();
    while let Some(chunk) = chunks.next().await {
        data.extend(chunk.map_err(|s| error::status(&s))?.data);
    }
    Ok(data)
}

/// The manifest to size for `info`'s platform: the index's matching entry,
/// or its first; `None` for a manifest, which is its own.
fn chosen<'a>(index: &'a Manifest, info: &ServerInfo) -> Option<&'a Entry> {
    let usable = |e: &&Entry| {
        e.platform
            .as_ref()
            .is_none_or(|p| p.os != "unknown" && p.architecture != "unknown")
    };
    index
        .manifests
        .iter()
        .filter(usable)
        .find(|e| {
            e.platform
                .as_ref()
                .is_some_and(|p| p.os == info.os && p.architecture == info.arch)
        })
        .or_else(|| index.manifests.iter().find(usable))
}

/// The platforms an index lists, without attestations' `unknown/unknown`.
fn platforms(index: &Manifest) -> Vec<String> {
    index
        .manifests
        .iter()
        .filter_map(|e| e.platform.as_ref())
        .filter(|p| p.os != "unknown")
        .map(Platform::name)
        .collect()
}

/// What an image's manifests say for `info`'s platform.
#[derive(Default)]
pub(crate) struct Described {
    pub(crate) size: Option<i64>,
    pub(crate) platforms: Vec<String>,
    /// The digest of its config for the platform, which holds the
    /// garbage-collection reference to its top layer's snapshot.
    pub(crate) config: Option<String>,
}

/// An image's size and config for `info`'s platform, and the platforms it has.
pub(crate) async fn describe(
    client: &Containerd,
    namespace: &str,
    image: &Image,
    info: &ServerInfo,
) -> Described {
    let Some(target) = &image.target else {
        return Described::default();
    };
    let Ok(top) = read(client, namespace, &target.digest, target.size).await else {
        return Described::default();
    };
    let Ok(top_json) = serde_json::from_slice::<Manifest>(&top) else {
        return Described::default();
    };
    let (manifest, mut size, platforms) = match chosen(&top_json, info) {
        Some(entry) => {
            let Ok(bytes) = read(client, namespace, &entry.digest, entry.size).await else {
                return Described {
                    platforms: platforms(&top_json),
                    ..Default::default()
                };
            };
            (bytes, target.size + entry.size, platforms(&top_json))
        }
        None => (top, target.size, vec![]),
    };
    let Ok(manifest) = serde_json::from_slice::<Manifest>(&manifest) else {
        return Described {
            platforms,
            ..Default::default()
        };
    };
    size += manifest.config.as_ref().map_or(0, |c| c.size);
    size += manifest.layers.iter().map(|l| l.size).sum::<i64>();
    let platforms = if platforms.is_empty() {
        vec![info.platform()]
    } else {
        platforms
    };
    Described {
        size: Some(size),
        platforms,
        config: manifest.config.map(|c| c.digest),
    }
}

/// The images of `namespace`, as containerd stores them.
pub(crate) async fn records(client: &Containerd, namespace: &str) -> AppResult<Vec<Image>> {
    Ok(client
        .images()
        .list(call(namespace, ListImagesRequest::default())?)
        .await
        .map_err(|s| error::status(&s))?
        .into_inner()
        .images)
}

/// How many containers of `namespace` each image is the image of.
async fn users(client: &Containerd, namespace: &str) -> AppResult<HashMap<String, Vec<String>>> {
    let containers = client
        .containers()
        .list(call(namespace, ListContainersRequest::default())?)
        .await
        .map_err(|s| error::status(&s))?
        .into_inner()
        .containers;
    let mut users: HashMap<String, Vec<String>> = HashMap::new();
    for c in containers {
        users.entry(c.image).or_default().push(c.id);
    }
    Ok(users)
}

/// The images of `namespace`, by name.
pub async fn list(client: &Containerd, namespace: &str) -> AppResult<Vec<ContainerdImage>> {
    let images = records(client, namespace).await?;
    let info = introspection::describe(client).await?;
    let users = users(client, namespace).await?;
    let mut list = Vec::with_capacity(images.len());
    for image in images {
        let Described {
            size, platforms, ..
        } = describe(client, namespace, &image, &info).await;
        let target = image.target.clone().unwrap_or_default();
        list.push(ContainerdImage {
            containers: users
                .get(&image.name)
                .map_or(0, |u| u32::try_from(u.len()).unwrap_or(u32::MAX)),
            name: image.name,
            digest: target.digest,
            media_type: target.media_type,
            size,
            platforms,
            created_at: crate::containers::timestamp(image.created_at.as_ref()),
            labels: image.labels,
        });
    }
    list.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(list)
}

/// Removes an image's name; refused while a container of the namespace
/// was made from it. containerd collects the content nothing else holds.
pub async fn remove(client: &Containerd, namespace: &str, name: &str) -> AppResult<()> {
    if let Some(using) = users(client, namespace).await?.get(name) {
        return Err(AppError::new(
            ErrorCode::InUse,
            msg::IMAGE_IN_USE
                .with("name", name)
                .with("containers", using.join(", ")),
        )
        .with_hint(msg::IMAGE_IN_USE_HINT));
    }
    client
        .images()
        .delete(call(
            namespace,
            DeleteImageRequest {
                name: name.to_owned(),
                sync: false,
                target: None,
            },
        )?)
        .await
        .map_err(|s| match s.code() {
            tonic::Code::NotFound => AppError::new(
                ErrorCode::ImageNotFound,
                msg::IMAGE_NOT_FOUND.with("name", name),
            )
            .with_detail(error::detail(&s)),
            _ => error::status(&s),
        })?;
    Ok(())
}

#[cfg(test)]
mod tests;

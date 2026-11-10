//! Images: listing, history, pulling and pushing with progress, tagging and removal.

use std::collections::HashMap;

use bollard::{
    query_parameters::{
        CreateImageOptionsBuilder, ListContainersOptionsBuilder, ListImagesOptionsBuilder,
        PruneImagesOptionsBuilder, PushImageOptionsBuilder, RemoveImageOptionsBuilder,
        SearchImagesOptionsBuilder, TagImageOptionsBuilder,
    },
    Docker,
};
use futures_util::StreamExt;
use serde::Serialize;

use homeport_core::error::{AppError, AppResult};

use crate::messages as msg;

use crate::auth::RegistryAuth;

pub use homeport_core::pull::PullEvent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: String,
    /// `repo:tag` names; empty for dangling images.
    pub tags: Vec<String>,
    pub digests: Vec<String>,
    /// Unix time in seconds.
    pub created: i64,
    pub size: i64,
    /// Containers (running or not) created from this image.
    pub containers: u32,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImageLayer {
    pub id: String,
    pub created: i64,
    pub created_by: String,
    pub size: i64,
    pub comment: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PruneReport {
    pub removed: Vec<String>,
    pub space_reclaimed: i64,
}

/// A Docker Hub repository found by a search.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    /// `postgres` for official images, `bitnami/redis` otherwise.
    pub name: String,
    pub description: String,
    pub stars: i64,
    pub official: bool,
}

/// Splits `name[:tag]` into the image and tag the Engine's pull expects,
/// leaving registry ports (`host:5000/app`) and digests (`app@sha256:…`) intact.
pub fn split_reference(reference: &str) -> (String, Option<String>) {
    let reference = reference.trim();
    if reference.contains('@') {
        return (reference.to_owned(), None);
    }
    let name_start = reference.rfind('/').map_or(0, |i| i + 1);
    match reference[name_start..].rfind(':') {
        Some(i) => {
            let at = name_start + i;
            (
                reference[..at].to_owned(),
                Some(reference[at + 1..].to_owned()),
            )
        }
        None => (reference.to_owned(), Some("latest".to_owned())),
    }
}

pub async fn list(docker: &Docker) -> AppResult<Vec<Image>> {
    let images = docker
        .list_images(Some(ListImagesOptionsBuilder::default().build()))
        .await?;
    let mut usage: HashMap<String, u32> = HashMap::new();
    for c in docker
        .list_containers(Some(
            ListContainersOptionsBuilder::default().all(true).build(),
        ))
        .await?
    {
        if let Some(id) = c.image_id {
            *usage.entry(id).or_default() += 1;
        }
    }
    let mut images: Vec<Image> = images
        .into_iter()
        .map(|i| Image {
            containers: usage.get(&i.id).copied().unwrap_or(0),
            tags: i
                .repo_tags
                .into_iter()
                .filter(|t| t != "<none>:<none>")
                .collect(),
            digests: i.repo_digests,
            created: i.created,
            size: i.size,
            labels: i.labels,
            id: i.id,
        })
        .collect();
    images.sort_by(|a, b| {
        a.tags
            .first()
            .cmp(&b.tags.first())
            .then(b.created.cmp(&a.created))
    });
    Ok(images)
}

pub async fn inspect(docker: &Docker, id: &str) -> AppResult<serde_json::Value> {
    Ok(serde_json::to_value(docker.inspect_image(id).await?)?)
}

pub async fn history(docker: &Docker, id: &str) -> AppResult<Vec<ImageLayer>> {
    Ok(docker
        .image_history(id)
        .await?
        .into_iter()
        .map(|l| ImageLayer {
            id: l.id,
            created: l.created,
            created_by: l.created_by,
            size: l.size,
            comment: l.comment,
            tags: l.tags,
        })
        .collect())
}

/// Pulls `reference` with the registry login when there is one, reporting
/// per-layer progress until done.
pub async fn pull(
    docker: Docker,
    reference: String,
    auth: Option<RegistryAuth>,
    sink: impl Fn(PullEvent) -> bool,
) {
    let (image, tag) = split_reference(&reference);
    let mut options = CreateImageOptionsBuilder::default().from_image(&image);
    if let Some(tag) = &tag {
        options = options.tag(tag);
    }
    let credentials = auth.as_ref().map(RegistryAuth::to_docker);
    let mut stream = docker.create_image(Some(options.build()), None, credentials);
    while let Some(item) = stream.next().await {
        let event = match item {
            Ok(info) => match info.error_detail.and_then(|e| e.message) {
                Some(message) => {
                    sink(PullEvent::Error {
                        error: homeport_core::error::engine::from_message(None, &message)
                            .with_detail(&message)
                            .logged("image pull"),
                    });
                    return;
                }
                None => PullEvent::Progress {
                    id: info.id,
                    status: info.status.unwrap_or_default(),
                    current: info.progress_detail.as_ref().and_then(|p| p.current),
                    total: info.progress_detail.as_ref().and_then(|p| p.total),
                },
            },
            Err(e) => {
                sink(PullEvent::Error {
                    error: AppError::from(e).logged("image pull"),
                });
                return;
            }
        };
        if !sink(event) {
            return;
        }
    }
    sink(PullEvent::Done);
}

/// Pushes `reference` (`registry/repo:tag`) with the registry login when
/// there is one, reporting progress until done.
pub async fn push(
    docker: Docker,
    reference: String,
    auth: Option<RegistryAuth>,
    sink: impl Fn(PullEvent) -> bool,
) {
    let (repo, tag) = split_reference(&reference);
    if repo.is_empty() || repo.contains('@') {
        sink(PullEvent::Error {
            error: AppError::validation(msg::PUSH_NAME),
        });
        return;
    }
    let options = PushImageOptionsBuilder::default()
        .tag(tag.as_deref().unwrap_or("latest"))
        .build();
    let credentials = auth.as_ref().map(RegistryAuth::to_docker);
    let mut stream = docker.push_image(&repo, Some(options), credentials);
    while let Some(item) = stream.next().await {
        let event = match item {
            Ok(info) => match info.error_detail.and_then(|e| e.message) {
                Some(message) => {
                    sink(PullEvent::Error {
                        error: push_error(&message).logged("image push"),
                    });
                    return;
                }
                None => PullEvent::Progress {
                    id: None,
                    status: info.status.unwrap_or_default(),
                    current: info.progress_detail.as_ref().and_then(|p| p.current),
                    total: info.progress_detail.as_ref().and_then(|p| p.total),
                },
            },
            Err(e) => {
                sink(PullEvent::Error {
                    error: AppError::from(e).logged("image push"),
                });
                return;
            }
        };
        if !sink(event) {
            return;
        }
    }
    sink(PullEvent::Done);
}

/// A push the registry refused reads as a refusal to push, not as a
/// missing image; everything else is classified like any Engine message.
fn push_error(message: &str) -> AppError {
    let error = homeport_core::error::engine::from_message(None, message).with_detail(message);
    if error.is(homeport_core::error::ErrorCode::RegistryAuthRequired) {
        error
            .with_message(msg::PUSH_REFUSED)
            .with_hint(msg::PUSH_REFUSED_HINT)
    } else {
        error
    }
}

/// Searches Docker Hub through the Engine, so the search follows the Host's
/// network as the pull will. Official images come first, then by stars.
pub async fn search(docker: &Docker, term: &str, limit: u32) -> AppResult<Vec<SearchResult>> {
    let term = term.trim();
    if term.is_empty() {
        return Ok(Vec::new());
    }
    let options = SearchImagesOptionsBuilder::default()
        .term(term)
        .limit(limit.clamp(1, 100) as i32)
        .build();
    let mut results: Vec<SearchResult> = docker
        .search_images(options)
        .await?
        .into_iter()
        .filter_map(|r| {
            Some(SearchResult {
                name: r.name?,
                description: r.description.unwrap_or_default(),
                stars: r.star_count.unwrap_or(0),
                official: r.is_official.unwrap_or(false),
            })
        })
        .collect();
    results.sort_by(|a, b| b.official.cmp(&a.official).then(b.stars.cmp(&a.stars)));
    Ok(results)
}

/// Adds the name `target` (`repo[:tag]`) to an image.
pub async fn tag(docker: &Docker, id: &str, target: &str) -> AppResult<()> {
    let (repo, tag) = split_reference(target);
    if repo.is_empty() || repo.contains('@') {
        return Err(AppError::validation(msg::TAG_NAME));
    }
    let options = TagImageOptionsBuilder::default()
        .repo(&repo)
        .tag(tag.as_deref().unwrap_or("latest"))
        .build();
    Ok(docker.tag_image(id, Some(options)).await?)
}

/// Removes an image (or one of its names); returns what was untagged or deleted.
pub async fn remove(docker: &Docker, id: &str, force: bool) -> AppResult<Vec<String>> {
    let options = RemoveImageOptionsBuilder::default().force(force).build();
    Ok(docker
        .remove_image(id, Some(options), None)
        .await?
        .into_iter()
        .filter_map(|r| r.deleted.or(r.untagged))
        .collect())
}

/// Removes dangling images, or every unused image with `all`.
pub async fn prune(docker: &Docker, all: bool) -> AppResult<PruneReport> {
    let dangling = if all { "false" } else { "true" };
    let filters = HashMap::from([("dangling", vec![dangling])]);
    let report = docker
        .prune_images(Some(
            PruneImagesOptionsBuilder::default()
                .filters(&filters)
                .build(),
        ))
        .await?;
    Ok(PruneReport {
        removed: report
            .images_deleted
            .unwrap_or_default()
            .into_iter()
            .filter_map(|r| r.deleted.or(r.untagged))
            .collect(),
        space_reclaimed: report.space_reclaimed.unwrap_or(0),
    })
}

#[cfg(test)]
mod tests;

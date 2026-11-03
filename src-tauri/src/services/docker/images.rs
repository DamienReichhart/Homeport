//! Images: listing, history, pulling with progress, tagging and removal.

use std::collections::HashMap;

use bollard::{
    query_parameters::{
        CreateImageOptionsBuilder, ListContainersOptionsBuilder, ListImagesOptionsBuilder,
        PruneImagesOptionsBuilder, RemoveImageOptionsBuilder, TagImageOptionsBuilder,
    },
    Docker,
};
use futures_util::StreamExt;
use serde::Serialize;

use crate::error::{AppError, AppResult, ErrorCode};

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
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum PullEvent {
    /// Progress of one layer (or a general status when `id` is absent).
    Progress {
        id: Option<String>,
        status: String,
        current: Option<i64>,
        total: Option<i64>,
    },
    Done,
    Error {
        error: AppError,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PruneReport {
    pub removed: Vec<String>,
    pub space_reclaimed: i64,
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

/// Pulls `reference`, reporting per-layer progress until done.
pub async fn pull(docker: Docker, reference: String, sink: impl Fn(PullEvent) -> bool) {
    let (image, tag) = split_reference(&reference);
    let mut options = CreateImageOptionsBuilder::default().from_image(&image);
    if let Some(tag) = &tag {
        options = options.tag(tag);
    }
    let mut stream = docker.create_image(Some(options.build()), None, None);
    while let Some(item) = stream.next().await {
        let event = match item {
            Ok(info) => match info.error_detail.and_then(|e| e.message) {
                Some(message) => {
                    sink(PullEvent::Error {
                        error: AppError::new(ErrorCode::EngineError, message).logged("image pull"),
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

/// Adds the name `target` (`repo[:tag]`) to an image.
pub async fn tag(docker: &Docker, id: &str, target: &str) -> AppResult<()> {
    let (repo, tag) = split_reference(target);
    if repo.is_empty() || repo.contains('@') {
        return Err(AppError::validation("Enter a name like repo:tag."));
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
mod tests {
    use super::*;
    use crate::services::docker::testing;

    #[test]
    fn references_split_into_image_and_tag() {
        let split = |r| split_reference(r);
        assert_eq!(split("nginx"), ("nginx".into(), Some("latest".into())));
        assert_eq!(split("nginx:1.27"), ("nginx".into(), Some("1.27".into())));
        assert_eq!(
            split("registry.local:5000/team/app"),
            ("registry.local:5000/team/app".into(), Some("latest".into()))
        );
        assert_eq!(
            split("registry.local:5000/team/app:v2"),
            ("registry.local:5000/team/app".into(), Some("v2".into()))
        );
        assert_eq!(split("app@sha256:abc"), ("app@sha256:abc".into(), None));
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn images_can_be_tagged_listed_and_untagged() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let name = format!("{}:v1", testing::unique("img"));
        tag(&docker, testing::IMAGE, &name).await.unwrap();

        let listed = list(&docker).await.unwrap();
        let image = listed.iter().find(|i| i.tags.contains(&name)).unwrap();
        assert!(image.size > 0);
        assert!(!history(&docker, &image.id).await.unwrap().is_empty());

        let removed = remove(&docker, &name, false).await.unwrap();
        assert_eq!(removed, [name]);
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn pulls_report_progress_and_completion() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        pull(testing::docker(), testing::IMAGE.into(), move |e| {
            tx.send(e).is_ok()
        })
        .await;
        let mut events = Vec::new();
        while let Ok(e) = rx.try_recv() {
            events.push(e);
        }
        assert!(matches!(events.first(), Some(PullEvent::Progress { .. })));
        assert_eq!(events.last(), Some(&PullEvent::Done));
    }
}

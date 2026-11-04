//! Whether the tag an image was pulled by now points to a different digest
//! in its registry: "a newer image is available". The registry is asked
//! through the Engine (`GET /distribution/{name}/json`), so the check follows
//! the Host's network, pulls nothing and needs nothing on the Host.
//!
//! Registries count these requests (Docker Hub against its pull limit), so
//! answers are cached per Host and reference, and asked a few at a time.

use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
    time::{Duration, Instant},
};

use bollard::{query_parameters::ListImagesOptionsBuilder, Docker};
use futures_util::{stream, StreamExt};
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult};

use crate::auth::Logins;

/// How long a registry's answer is trusted.
pub const FRESH_FOR: Duration = Duration::from_secs(6 * 60 * 60);
/// How long a failed lookup waits before it is tried again.
const RETRY_AFTER: Duration = Duration::from_secs(15 * 60);
/// Registry lookups at once, per check.
const CONCURRENCY: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum Freshness {
    UpToDate,
    /// The registry has a different image under this tag.
    Newer,
    /// No answer: built locally, pulled by digest, or the registry couldn't be asked.
    Unknown,
}

/// An image reference to check, and the image that runs under it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FreshnessTarget {
    /// `nginx:1.27`, as a container was created from or an image is tagged.
    pub reference: String,
    pub image_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FreshnessResult {
    pub reference: String,
    pub image_id: String,
    pub freshness: Freshness,
}

/// What this Engine knows of where an image came from.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LocalDigests {
    /// `repo@sha256:…`, recorded when the image was pulled or pushed (`RepoDigests`).
    pub repo_digests: Vec<String>,
    /// The digests of the platform manifests the Engine holds, with a
    /// multi-platform image store. A push of only some of an index's
    /// platforms records the index's digest but uploads one of these.
    pub manifests: Vec<String>,
    /// The reference now names another local image that came from the
    /// registry: a newer one was pulled, but this image (still run by a
    /// container) wasn't replaced. Its own `RepoDigests` may be gone.
    pub superseded: bool,
}

/// Decides from what the Engine knows of an image and the registry's
/// current digest for `reference`.
pub fn decide(reference: &str, local: &LocalDigests, registry: Option<&str>) -> Freshness {
    let Some(registry) = registry else {
        return Freshness::Unknown;
    };
    if !is_checkable(reference) {
        return Freshness::Unknown;
    }
    let repo = repository(reference);
    let recorded: Vec<&str> = local
        .repo_digests
        .iter()
        .filter_map(|d| d.split_once('@'))
        .filter(|(r, _)| canonical(r) == repo)
        .map(|(_, digest)| digest)
        .collect();
    if recorded.is_empty() {
        // Built here or only ever tagged locally, unless a pull replaced it.
        if local.superseded {
            Freshness::Newer
        } else {
            Freshness::Unknown
        }
    } else if recorded.contains(&registry) || local.manifests.iter().any(|m| m == registry) {
        Freshness::UpToDate
    } else {
        Freshness::Newer
    }
}

/// References that name a tag in a registry: not an image ID, not a digest,
/// not `<none>`.
pub fn is_checkable(reference: &str) -> bool {
    let r = reference.trim();
    !(r.is_empty()
        || r.contains('@')
        || r.starts_with("sha256:")
        || r.starts_with("<none>")
        || (r.len() == 64 && r.chars().all(|c| c.is_ascii_hexdigit())))
}

/// The repository of a reference, written the way `RepoDigests` may write
/// it: `nginx`, `library/nginx` and `docker.io/library/nginx` are one.
fn repository(reference: &str) -> String {
    let (repo, _) = crate::images::split_reference(reference);
    canonical(&repo)
}

fn canonical(repo: &str) -> String {
    let repo = repo
        .strip_prefix("docker.io/")
        .or_else(|| repo.strip_prefix("index.docker.io/"))
        .unwrap_or(repo);
    repo.strip_prefix("library/").unwrap_or(repo).to_owned()
}

/// Registry answers per Host and reference, shared by every caller.
#[derive(Default)]
pub struct FreshnessCache {
    entries: Mutex<HashMap<(String, String), Entry>>,
}

#[derive(Clone)]
struct Entry {
    at: Instant,
    digest: Option<String>,
}

impl FreshnessCache {
    /// The cached answer for `reference` on `host`, if still trusted.
    fn get(&self, host: &str, reference: &str, now: Instant) -> Option<Option<String>> {
        let entries = self.entries.lock().expect("freshness lock poisoned");
        let entry = entries.get(&(host.to_owned(), reference.to_owned()))?;
        let lifetime = if entry.digest.is_some() {
            FRESH_FOR
        } else {
            RETRY_AFTER
        };
        (now.duration_since(entry.at) < lifetime).then(|| entry.digest.clone())
    }

    fn put(&self, host: &str, reference: &str, digest: Option<String>, at: Instant) {
        self.entries
            .lock()
            .expect("freshness lock poisoned")
            .insert(
                (host.to_owned(), reference.to_owned()),
                Entry { at, digest },
            );
    }

    /// Checks every target on `host`. Only references the cache can't answer
    /// (or all of them with `force`) reach their registry; a registry that
    /// can't be asked makes its targets `Unknown` and is logged, never an error.
    pub async fn check(
        &self,
        docker: &Docker,
        host: &str,
        targets: Vec<FreshnessTarget>,
        logins: &Logins,
        force: bool,
    ) -> Vec<FreshnessResult> {
        let now = Instant::now();
        let references: HashSet<&str> = targets
            .iter()
            .map(|t| t.reference.as_str())
            .filter(|r| is_checkable(r))
            .collect();
        let missing: Vec<&str> = references
            .into_iter()
            .filter(|r| force || self.get(host, r, now).is_none())
            .collect();
        let looked_up: Vec<(&str, Option<String>)> = stream::iter(missing)
            .map(|reference| async move {
                let auth = logins.for_reference(reference).cloned();
                let digest = match registry_digest(docker, reference, auth).await {
                    Ok(digest) => Some(digest),
                    Err(e) => {
                        e.logged(&format!("newer image check for {reference}"));
                        None
                    }
                };
                (reference, digest)
            })
            .buffer_unordered(CONCURRENCY)
            .collect()
            .await;
        for (reference, digest) in looked_up {
            self.put(host, reference, digest, now);
        }

        let manifests = manifests(docker).await;
        let mut local: HashMap<(String, String), LocalDigests> = HashMap::new();
        let mut current: HashMap<String, Option<(String, bool)>> = HashMap::new();
        let mut results = Vec::with_capacity(targets.len());
        for target in targets {
            let registry = if is_checkable(&target.reference) {
                self.get(host, &target.reference, now).flatten()
            } else {
                None
            };
            let key = (target.reference.clone(), target.image_id.clone());
            if registry.is_some() && !local.contains_key(&key) {
                // The list's `RepoDigests` can miss some; inspect has them all.
                let repo_digests = repo_digests(docker, &target.image_id).await;
                if !current.contains_key(&target.reference) {
                    let now_named = match docker.inspect_image(&target.reference).await {
                        Ok(i) => i.id.map(|id| {
                            let pulled = i.repo_digests.is_some_and(|d| !d.is_empty());
                            (id, pulled)
                        }),
                        Err(_) => None,
                    };
                    current.insert(target.reference.clone(), now_named);
                }
                let superseded = matches!(
                    &current[&target.reference],
                    Some((id, true)) if *id != target.image_id
                );
                let image = LocalDigests {
                    repo_digests,
                    manifests: manifests.get(&target.image_id).cloned().unwrap_or_default(),
                    superseded,
                };
                local.insert(key.clone(), image);
            }
            let freshness = match local.get(&key) {
                Some(image) => decide(&target.reference, image, registry.as_deref()),
                None => Freshness::Unknown,
            };
            results.push(FreshnessResult {
                reference: target.reference,
                image_id: target.image_id,
                freshness,
            });
        }
        results
    }
}

async fn repo_digests(docker: &Docker, image_id: &str) -> Vec<String> {
    docker
        .inspect_image(image_id)
        .await
        .ok()
        .and_then(|i| i.repo_digests)
        .unwrap_or_default()
}

/// The platform manifests the Engine holds for each image, by image ID, in
/// one call; empty without a multi-platform image store.
async fn manifests(docker: &Docker) -> HashMap<String, Vec<String>> {
    let options = ListImagesOptionsBuilder::default().manifests(true).build();
    match docker.list_images(Some(options)).await {
        Ok(images) => images
            .into_iter()
            .map(|i| {
                let manifests = i
                    .manifests
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|m| m.available)
                    .map(|m| m.id)
                    .collect();
                (i.id, manifests)
            })
            .collect(),
        Err(e) => {
            AppError::from(e).logged("newer image check: listing images");
            HashMap::new()
        }
    }
}

/// The digest the registry has for `reference` now, asked through the Engine.
pub async fn registry_digest(
    docker: &Docker,
    reference: &str,
    auth: Option<crate::auth::RegistryAuth>,
) -> AppResult<String> {
    let inspect = docker
        .inspect_registry_image(reference, auth.as_ref().map(|a| a.to_docker()))
        .await?;
    inspect.descriptor.digest.ok_or_else(|| {
        AppError::new(
            homeport_core::error::ErrorCode::EngineError,
            "The registry didn't say which image it has.",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing;

    const OLD: &str = "sha256:1111";
    const NEW: &str = "sha256:2222";

    fn recorded(digests: &[String]) -> LocalDigests {
        LocalDigests {
            repo_digests: digests.to_vec(),
            ..Default::default()
        }
    }

    #[test]
    fn a_moved_tag_is_newer_and_a_current_one_is_not() {
        let digests = &recorded(&[format!("nginx@{OLD}")]);
        assert_eq!(decide("nginx:1.27", digests, Some(NEW)), Freshness::Newer);
        assert_eq!(
            decide("nginx:1.27", digests, Some(OLD)),
            Freshness::UpToDate
        );
        assert_eq!(decide("nginx", digests, Some(OLD)), Freshness::UpToDate);
        assert_eq!(
            decide("docker.io/library/nginx:1.27", digests, Some(OLD)),
            Freshness::UpToDate
        );
    }

    #[test]
    fn only_the_references_repository_counts() {
        let digests = &recorded(&[format!("other/app@{NEW}"), format!("ghcr.io/org/app@{OLD}")]);
        assert_eq!(
            decide("ghcr.io/org/app:1", digests, Some(NEW)),
            Freshness::Newer
        );
        assert_eq!(
            decide("ghcr.io/org/app:1", digests, Some(OLD)),
            Freshness::UpToDate
        );
    }

    #[test]
    fn local_builds_digests_and_unanswered_checks_are_unknown() {
        let digests = &recorded(&[format!("nginx@{OLD}")]);
        assert_eq!(
            decide("myapp:dev", &recorded(&[]), Some(NEW)),
            Freshness::Unknown
        );
        assert_eq!(
            decide(&format!("nginx@{OLD}"), digests, Some(NEW)),
            Freshness::Unknown
        );
        assert_eq!(
            decide("sha256:abcd", digests, Some(NEW)),
            Freshness::Unknown
        );
        assert_eq!(
            decide("<none>:<none>", digests, Some(NEW)),
            Freshness::Unknown
        );
        assert_eq!(decide("nginx:1.27", digests, None), Freshness::Unknown);
    }

    #[test]
    fn a_platform_manifest_the_engine_holds_is_current() {
        let local = LocalDigests {
            repo_digests: vec![format!("ghcr.io/org/app@{OLD}")],
            manifests: vec![NEW.into()],
            superseded: false,
        };
        assert_eq!(
            decide("ghcr.io/org/app:1", &local, Some(NEW)),
            Freshness::UpToDate
        );
        assert_eq!(
            decide("ghcr.io/org/app:1", &local, Some("sha256:3333")),
            Freshness::Newer
        );
    }

    #[test]
    fn an_image_a_pull_replaced_is_outdated_even_without_its_digests() {
        let replaced = LocalDigests {
            superseded: true,
            ..Default::default()
        };
        assert_eq!(decide("nginx:1.27", &replaced, Some(NEW)), Freshness::Newer);
        assert_eq!(decide("nginx:1.27", &replaced, None), Freshness::Unknown);
    }

    #[test]
    fn answers_expire_and_failures_retry_sooner() {
        let cache = FreshnessCache::default();
        let start = Instant::now();
        cache.put("h1", "nginx:1", Some(OLD.into()), start);
        cache.put("h1", "private:1", None, start);
        let later = |d: Duration| start + d;
        assert_eq!(
            cache.get("h1", "nginx:1", later(Duration::from_secs(60))),
            Some(Some(OLD.into()))
        );
        assert_eq!(cache.get("h2", "nginx:1", start), None, "per Host");
        assert_eq!(cache.get("h1", "nginx:1", later(FRESH_FOR)), None);
        assert_eq!(
            cache.get("h1", "private:1", later(Duration::from_secs(60))),
            Some(None)
        );
        assert_eq!(cache.get("h1", "private:1", later(RETRY_AFTER)), None);
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn the_test_image_is_checked_against_its_registry() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let registry = testing::registry(&docker, false).await;
        let name = format!("{}/{}:v1", registry.address, testing::unique("fresh"));
        crate::images::tag(&docker, testing::IMAGE, &name)
            .await
            .unwrap();
        let _cleanup = testing::ImageName(docker.clone(), name.clone());
        let pushed = std::sync::Mutex::new(None);
        crate::images::push(docker.clone(), name.clone(), None, |e| {
            *pushed.lock().unwrap() = Some(e);
            true
        })
        .await;
        assert_eq!(
            pushed.into_inner().unwrap(),
            Some(crate::images::PullEvent::Done)
        );
        let id = docker.inspect_image(&name).await.unwrap().id.unwrap();
        let target = FreshnessTarget {
            reference: name.clone(),
            image_id: id,
        };

        let cache = FreshnessCache::default();
        let results = cache
            .check(
                &docker,
                "local",
                vec![target.clone()],
                &Logins::none(),
                false,
            )
            .await;
        assert_eq!(results[0].freshness, Freshness::UpToDate, "{results:?}");

        // Another image pushed under the tag makes the first one outdated.
        crate::images::tag(&docker, "registry:2", &name)
            .await
            .unwrap();
        crate::images::push(docker.clone(), name.clone(), None, |_| true).await;
        let moved = cache
            .check(
                &docker,
                "local",
                vec![target.clone()],
                &Logins::none(),
                true,
            )
            .await;
        assert_eq!(moved[0].freshness, Freshness::Newer, "{moved:?}");

        // A cached answer survives the registry going away; a forced check doesn't.
        drop(registry);
        let cached = cache
            .check(
                &docker,
                "local",
                vec![target.clone()],
                &Logins::none(),
                false,
            )
            .await;
        assert_eq!(cached[0].freshness, Freshness::Newer);
        let forced = cache
            .check(&docker, "local", vec![target], &Logins::none(), true)
            .await;
        assert_eq!(forced[0].freshness, Freshness::Unknown);
    }
}

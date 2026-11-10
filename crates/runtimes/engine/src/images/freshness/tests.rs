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

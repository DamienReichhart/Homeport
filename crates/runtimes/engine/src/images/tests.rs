use super::*;
use crate::testing;

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
#[ignore = "needs a local Docker Engine and access to Docker Hub"]
async fn searches_find_official_images_first() {
    let results = search(&testing::docker(), "postg", 10).await.unwrap();
    let first = results.first().expect("some results");
    assert!(first.official, "{results:?}");
    assert_eq!(first.name, "postgres");
}

#[test]
fn refused_pushes_say_so() {
    let e = push_error("denied: requested access to the resource is denied");
    assert!(e.is(homeport_core::error::ErrorCode::RegistryAuthRequired));
    assert_eq!(e.message, "The registry refused the push.");
    assert!(e.hint.unwrap().text.contains("Settings → Registries"));
    assert_eq!(
        push_error("no space left on device").code,
        homeport_core::error::ErrorCode::DiskFull
    );
}

fn collect<F, Fut>(run: F) -> impl std::future::Future<Output = Vec<PullEvent>>
where
    F: FnOnce(Box<dyn Fn(PullEvent) -> bool + Send>) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let fut = run(Box::new(move |e| tx.send(e).is_ok()));
    async move {
        fut.await;
        let mut events = Vec::new();
        while let Ok(e) = rx.try_recv() {
            events.push(e);
        }
        events
    }
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn images_push_to_a_registry_with_a_login_and_pull_back() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let registry = testing::registry(&docker, true).await;
    let name = format!("{}/{}:v1", registry.address, testing::unique("push"));
    tag(&docker, testing::IMAGE, &name).await.unwrap();
    let _cleanup = testing::ImageName(docker.clone(), name.clone());
    let login = RegistryAuth {
        address: registry.address.clone(),
        username: testing::REGISTRY_USER.into(),
        secret: testing::REGISTRY_PASSWORD.into(),
    };

    // Without the login the registry refuses.
    let refused = collect(|sink| push(docker.clone(), name.clone(), None, sink)).await;
    assert!(
        matches!(refused.last(), Some(PullEvent::Error { error }) if error.is(homeport_core::error::ErrorCode::RegistryAuthRequired)),
        "{refused:?}"
    );

    let pushed =
        collect(|sink| push(docker.clone(), name.clone(), Some(login.clone()), sink)).await;
    assert_eq!(pushed.last(), Some(&PullEvent::Done), "{pushed:?}");
    assert!(!format!("{pushed:?}").contains(testing::REGISTRY_PASSWORD));

    remove(&docker, &name, false).await.unwrap();
    let pulled = collect(|sink| pull(docker.clone(), name.clone(), Some(login), sink)).await;
    assert_eq!(pulled.last(), Some(&PullEvent::Done), "{pulled:?}");
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
    // Anonymous, as with no login stored.
    pull(testing::docker(), testing::IMAGE.into(), None, move |e| {
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

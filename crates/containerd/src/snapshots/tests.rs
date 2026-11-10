use super::*;

fn parents(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, p)| (k.to_string(), p.to_string()))
        .collect()
}

#[test]
fn an_images_layers_are_its_tops_chain_and_a_container_its_key() {
    let parents = parents(&[
        ("sha256:base", ""),
        ("sha256:mid", "sha256:base"),
        ("sha256:top", "sha256:mid"),
        ("web", "sha256:top"),
        ("orphan", "sha256:base"),
    ]);
    let users = users_of(
        &parents,
        &[("web".into(), "web".into())],
        &[("docker.io/library/alpine:3.22".into(), "sha256:top".into())],
    );
    let image = SnapshotUser::Image {
        name: "docker.io/library/alpine:3.22".into(),
    };
    for layer in ["sha256:base", "sha256:mid", "sha256:top"] {
        assert_eq!(users[layer], std::slice::from_ref(&image), "{layer}");
    }
    assert_eq!(users["web"], [SnapshotUser::Container { id: "web".into() }]);
    assert!(!users.contains_key("orphan"));
}

#[test]
fn a_chain_with_a_loop_ends() {
    let looped = parents(&[("a", "b"), ("b", "a")]);
    assert_eq!(chain("a", &looped), ["a", "b"]);
}

mod live {
    use crate::snapshots::*;
    use crate::testing::{self, NAMESPACE};

    #[tokio::test]
    #[ignore = "needs a local containerd"]
    async fn containerd_live_snapshots_show_users_and_free_orphans() {
        let client = testing::client().await;
        let image = testing::pulled();
        let used = testing::unique("homeport-test-snap");
        let orphan = testing::unique("homeport-test-orphan");
        let _cleanup = (
            testing::Cleanup(used.clone()),
            testing::Cleanup(orphan.clone()),
        );
        testing::ctr(&["containers", "create", &image, &used, "true"]);
        // A container removed with its snapshot kept leaves it behind,
        // marked so containerd's garbage collector doesn't take it.
        testing::ctr(&["containers", "create", &image, &orphan, "true"]);
        crate::containers::remove(&client, NAMESPACE, &orphan, false, true)
            .await
            .unwrap();

        let listed = list(&client, NAMESPACE, None).await.unwrap();
        let find = |key: &str| listed.snapshots.iter().find(|s| s.key == key).cloned();
        let mine = find(&used).expect("the container's snapshot");
        assert_eq!(mine.kind, SnapshotKind::Active);
        assert_eq!(mine.users, [SnapshotUser::Container { id: used.clone() }]);
        let layer = find(&mine.parent).expect("its image's top layer");
        assert_eq!(layer.kind, SnapshotKind::Committed);
        assert!(layer
            .users
            .iter()
            .any(|u| matches!(u, SnapshotUser::Image { name } if *name == image)));
        assert!(layer.size.is_some());
        assert!(listed.total >= layer.size.unwrap());

        let unused = find(&orphan).expect("the orphan's snapshot");
        assert!(unused.users.is_empty());
        let in_use = remove(&client, NAMESPACE, &listed.snapshotter, &layer.key)
            .await
            .unwrap_err();
        assert_eq!(in_use.code, ErrorCode::InUse);
        remove(&client, NAMESPACE, &listed.snapshotter, &orphan)
            .await
            .unwrap();
        assert!(!testing::ctr(&["snapshots", "ls"]).contains(&orphan));
    }
}

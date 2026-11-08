use super::*;

fn info() -> ServerInfo {
    ServerInfo {
        os: "linux".into(),
        arch: "arm64".into(),
        variant: "v8".into(),
        snapshotters: vec!["overlayfs".into()],
    }
}

fn index() -> Manifest {
    serde_json::from_value(serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.oci.image.index.v1+json",
        "manifests": [
            { "mediaType": "application/vnd.oci.image.manifest.v1+json", "digest": "sha256:a", "size": 500,
              "platform": { "os": "linux", "architecture": "amd64" } },
            { "mediaType": "application/vnd.oci.image.manifest.v1+json", "digest": "sha256:b", "size": 501,
              "platform": { "os": "linux", "architecture": "arm64", "variant": "v8" } },
            { "mediaType": "application/vnd.oci.image.manifest.v1+json", "digest": "sha256:c", "size": 300,
              "platform": { "os": "unknown", "architecture": "unknown" } }
        ]
    }))
    .unwrap()
}

#[test]
fn an_index_is_sized_for_the_hosts_platform() {
    let index = index();
    assert_eq!(chosen(&index, &info()).unwrap().digest, "sha256:b");
    let other = ServerInfo {
        arch: "s390x".into(),
        ..info()
    };
    assert_eq!(chosen(&index, &other).unwrap().digest, "sha256:a");
    assert_eq!(platforms(&index), ["linux/amd64", "linux/arm64/v8"]);
}

#[test]
fn a_manifest_lists_its_config_and_layers() {
    let manifest: Manifest = serde_json::from_value(serde_json::json!({
        "config": { "digest": "sha256:cfg", "size": 1000 },
        "layers": [{ "digest": "sha256:l1", "size": 3_000_000 }, { "digest": "sha256:l2", "size": 20 }]
    }))
    .unwrap();
    assert!(chosen(&manifest, &info()).is_none());
    assert_eq!(
        manifest.layers.iter().map(|l| l.size).sum::<i64>(),
        3_000_020
    );
}

mod live {
    use std::sync::Mutex;

    use super::super::*;
    use crate::testing::{self, NAMESPACE};

    #[tokio::test]
    #[ignore = "needs a local containerd"]
    async fn containerd_live_images_pull_list_and_refuse_removal_in_use() {
        let client = testing::client().await;
        if !client.at_least(crate::PULL_VERSION) {
            eprintln!(
                "containerd {} can't pull through Transfer; skipping",
                client.version()
            );
            return;
        }
        let reference = std::env::var("HOMEPORT_CONTAINERD_PULL_IMAGE")
            .unwrap_or_else(|_| "mirror.gcr.io/library/busybox:1.37".into());
        let events = Mutex::new(Vec::new());
        let collected = std::sync::Arc::new(events);
        let sink = collected.clone();
        pull(
            client.clone(),
            NAMESPACE.into(),
            reference.clone(),
            None,
            move |e| {
                sink.lock().unwrap().push(e);
                true
            },
        )
        .await;
        let events = collected.lock().unwrap().clone();
        assert!(
            matches!(events.last(), Some(homeport_core::pull::PullEvent::Done)),
            "{events:?}"
        );
        assert!(testing::ctr(&["images", "ls", "-q"])
            .lines()
            .any(|l| l == reference));

        let listed = list(&client, NAMESPACE).await.unwrap();
        let image = listed.iter().find(|i| i.name == reference).expect("listed");
        assert!(image.size.is_some_and(|s| s > 100_000), "{image:?}");
        assert!(!image.platforms.is_empty());
        assert!(image.digest.starts_with("sha256:"));

        // Unpacked for the default snapshotter: a container runs from it.
        let id = testing::unique("homeport-test-image");
        let _cleanup = testing::Cleanup(id.clone());
        testing::ctr(&["containers", "create", &reference, &id, "true"]);
        let refused = remove(&client, NAMESPACE, &reference).await.unwrap_err();
        assert_eq!(refused.code, ErrorCode::InUse);
        assert!(refused.message.contains(&id), "{refused:?}");
        testing::ctr(&["containers", "rm", &id]);
        remove(&client, NAMESPACE, &reference).await.unwrap();
        assert!(!testing::ctr(&["images", "ls", "-q"])
            .lines()
            .any(|l| l == reference));

        // A tag the registry doesn't have.
        let missing = std::sync::Arc::new(Mutex::new(None));
        let sink = missing.clone();
        let nope = format!("{}-homeport-nope", reference);
        pull(client, NAMESPACE.into(), nope, None, move |e| {
            if let homeport_core::pull::PullEvent::Error { error } = e {
                *sink.lock().unwrap() = Some(error);
            }
            true
        })
        .await;
        let error = missing.lock().unwrap().clone().expect("the pull failed");
        assert_eq!(error.code, ErrorCode::ImageNotFound, "{error:?}");
    }
}

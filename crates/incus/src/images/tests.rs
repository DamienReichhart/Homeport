use super::*;

#[test]
fn images_read_as_incus_lists_them() {
    // An image of GET /1.0/images?recursion=1 on Incus 6.0.
    let raw: RawImage = serde_json::from_value(serde_json::json!({
        "auto_update": true,
        "properties": {
            "architecture": "amd64",
            "description": "Alpine 3.21 amd64 (20261108_13:00)",
            "os": "Alpine",
            "release": "3.21",
            "variant": "default"
        },
        "public": false,
        "aliases": [{ "name": "alpine", "description": "" }],
        "architecture": "x86_64",
        "cached": true,
        "fingerprint": "4495673b594449305e13d8119977a010a524b1d0dd558cb8db46c6e4c8e89f15",
        "size": 3367792,
        "update_source": {
            "alias": "alpine/3.21",
            "protocol": "simplestreams",
            "server": "https://images.linuxcontainers.org"
        },
        "type": "container",
        "last_used_at": "0001-01-01T00:00:00Z",
        "uploaded_at": "2026-11-08T22:35:36.072782797Z"
    }))
    .unwrap();
    let image = raw.into_image();
    assert_eq!(image.aliases, ["alpine"]);
    assert_eq!(image.kind, InstanceType::Container);
    assert_eq!(image.os, "Alpine");
    assert_eq!(image.last_used_at, None);
    assert_eq!(image.source.unwrap().alias, "alpine/3.21");
}

#[test]
fn a_copy_pulls_from_simplestreams_and_can_keep_its_alias() {
    let copy = ImageCopy {
        server: "https://images.linuxcontainers.org".into(),
        alias: "debian/12".into(),
        kind: InstanceType::VirtualMachine,
        auto_update: true,
        add_alias: true,
    };
    assert_eq!(
        copy_body(&copy),
        serde_json::json!({
            "source": {
                "type": "image",
                "mode": "pull",
                "server": "https://images.linuxcontainers.org",
                "protocol": "simplestreams",
                "alias": "debian/12",
                "image_type": "virtual-machine"
            },
            "auto_update": true,
            "aliases": [{ "name": "debian/12" }]
        })
    );
}

#[test]
fn progress_reads_its_percentage() {
    let op: Operation = serde_json::from_value(serde_json::json!({
        "status_code": 103,
        "metadata": { "download_progress": "rootfs: 42% (12.30MB/s)" }
    }))
    .unwrap();
    assert_eq!(
        progress_event(&op),
        Some(PullEvent::Progress {
            id: None,
            status: "rootfs: 42% (12.30MB/s)".into(),
            current: Some(42),
            total: Some(100),
        })
    );
}

#[tokio::test]
#[ignore = "needs a local Incus"]
async fn incus_live_images_are_copied_aliased_and_deleted() {
    let incus = crate::instances::tests::live().await;
    let alias = std::env::var("HOMEPORT_INCUS_ALIAS").unwrap_or_else(|_| "alpine/3.21".into());
    let mut events = Vec::new();
    let copy_of = ImageCopy {
        server: crate::simplestreams::DEFAULT_SERVER.into(),
        alias: alias.clone(),
        kind: InstanceType::Container,
        auto_update: false,
        add_alias: false,
    };
    copy(&incus, "default", &copy_of, |e| {
        events.push(e);
        true
    })
    .await;
    assert_eq!(events.last(), Some(&PullEvent::Done), "{events:?}");
    let images = list(&incus, "default").await.unwrap();
    let image = images
        .iter()
        .find(|i| i.source.as_ref().is_some_and(|s| s.alias == alias))
        .expect("copied");
    let name = format!("homeport-test-{}", std::process::id());
    add_alias(&incus, "default", &image.fingerprint, &name)
        .await
        .unwrap();
    let again = add_alias(&incus, "default", &image.fingerprint, &name)
        .await
        .unwrap_err();
    assert_eq!(again.code, ErrorCode::NameInUse, "{again:?}");
    remove_alias(&incus, "default", &name).await.unwrap();
    // This is the image the other live tests make Instances from:
    // Instances made keep their own copy, so deleting it must work, but
    // not while one is being created from it, which would lose its file.
    let _deleting = crate::instances::tests::LIVE_IMAGE.write().await;
    delete(&incus, "default", &image.fingerprint).await.unwrap();
}

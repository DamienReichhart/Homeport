use super::*;
use crate::testing;

#[test]
fn clones_leave_the_stack() {
    let labels = HashMap::from([
        ("com.docker.compose.project".to_owned(), "shop".to_owned()),
        ("com.docker.compose.volume".to_owned(), "db".to_owned()),
        ("backup".to_owned(), "nightly".to_owned()),
    ]);
    assert_eq!(
        own_labels(labels),
        HashMap::from([("backup".to_owned(), "nightly".to_owned())])
    );
}

#[test]
fn volumes_whose_options_name_their_storage_share_it_with_a_clone() {
    let opts = |pairs: &[(&str, &str)]| -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect()
    };
    let bind = opts(&[("type", "none"), ("o", "bind"), ("device", "/srv/db")]);
    let nfs = opts(&[
        ("type", "nfs"),
        ("o", "addr=10.0.0.2"),
        ("device", ":/export"),
    ]);
    assert!(shares_storage("local", &bind));
    assert!(shares_storage("local", &nfs));
    assert!(!shares_storage("local", &HashMap::new()));
    assert!(!shares_storage("local", &opts(&[("size", "1G")])));
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and busybox:1.37"]
async fn a_volume_on_a_folder_of_the_host_is_never_cloned_onto_itself() {
    let _serial = testing::HELPERS.lock().await;
    let docker = testing::docker();
    let folder = tempfile::tempdir().unwrap();
    std::fs::write(folder.path().join("data.txt"), "precious").unwrap();
    let source = testing::VolumeGuard {
        docker: docker.clone(),
        name: testing::unique("bind"),
    };
    docker
        .create_volume(VolumeCreateRequest {
            name: Some(source.name.clone()),
            driver: Some("local".into()),
            driver_opts: Some(HashMap::from([
                ("type".to_owned(), "none".to_owned()),
                ("o".to_owned(), "bind".to_owned()),
                (
                    "device".to_owned(),
                    folder.path().to_string_lossy().into_owned(),
                ),
            ])),
            ..Default::default()
        })
        .await
        .unwrap();
    let target = testing::VolumeGuard {
        docker: docker.clone(),
        name: testing::unique("bind-copy"),
    };

    let refused = clone(&docker, &source.name, &target.name, false)
        .await
        .unwrap_err();
    assert!(refused.is(ErrorCode::Validation), "{refused:?}");
    assert_eq!(refused.key, "errors.volumes.cloneSharedStorage");
    assert!(
        docker.inspect_volume(&target.name).await.is_err(),
        "nothing made"
    );
    assert_eq!(
        std::fs::read_to_string(folder.path().join("data.txt")).unwrap(),
        "precious"
    );
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and busybox:1.37"]
async fn a_clone_has_the_same_files_owners_and_modes() {
    let _serial = testing::HELPERS.lock().await;
    let docker = testing::docker();
    let source = testing::filled_volume(&docker).await;
    let copy = testing::VolumeGuard {
        docker: docker.clone(),
        name: testing::unique("clone"),
    };

    let made = clone(&docker, &source.name, &copy.name, false)
        .await
        .unwrap();
    assert_eq!(made, copy.name);
    let stat = "stat -c '%u:%g %a %s' /data/etc/note.txt";
    assert_eq!(
        testing::output_with_volume(&docker, &copy.name, stat).await,
        testing::output_with_volume(&docker, &source.name, stat).await,
    );

    let again = clone(&docker, &source.name, &copy.name, false).await;
    assert_eq!(again.unwrap_err().code, ErrorCode::NameInUse);
    let missing = clone(
        &docker,
        &testing::unique("nope"),
        &testing::unique("x"),
        false,
    )
    .await;
    assert!(missing.is_err());
}

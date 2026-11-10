use homeport_core::error::ErrorCode;

use super::*;
use crate::{containers, testing};

fn spec(reference: &str, changes: &[&str]) -> CommitSpec {
    CommitSpec {
        reference: reference.into(),
        changes: changes.iter().map(|c| (*c).to_owned()).collect(),
        ..Default::default()
    }
}

#[test]
fn names_and_changes_are_checked() {
    let (repo, tag, changes) = validate(&spec(
        "registry.home:5000/web-backup",
        &["env MODE=prod", "", "# a note", "  EXPOSE 8080  "],
    ))
    .unwrap();
    assert_eq!(
        (repo.as_str(), tag.as_str()),
        ("registry.home:5000/web-backup", "latest")
    );
    assert_eq!(changes, ["ENV MODE=prod", "EXPOSE 8080"]);

    for bad in [
        spec(" ", &[]),
        spec("web@sha256:abc", &[]),
        spec("web:1", &["RUN apt-get update"]),
        spec("web:1", &["ENV"]),
    ] {
        assert!(
            matches!(validate(&bad), Err(ref e) if e.is(ErrorCode::Validation)),
            "{bad:?}"
        );
    }
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn commits_a_containers_files_and_config_changes() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let guard = testing::container(
        &docker,
        &testing::unique("commit"),
        &["sh", "-c", "echo kept > /committed.txt && sleep 60"],
    )
    .await;
    containers::act(&docker, &guard.id, containers::ContainerAction::Start)
        .await
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    let image = testing::ImageName(
        docker.clone(),
        format!("{}:1", testing::unique("committed")),
    );
    commit(
        &docker,
        &guard.id,
        CommitSpec {
            reference: image.1.clone(),
            comment: Some("Before the upgrade".into()),
            pause: true,
            changes: vec!["ENV GREETING=hello".into()],
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let inspected = docker.inspect_image(&image.1).await.unwrap();
    let env = inspected.config.unwrap().env.unwrap();
    assert!(env.contains(&"GREETING=hello".to_owned()), "{env:?}");
    let check = containers::run::run(
        &docker,
        containers::run::RunSpec {
            image: image.1.clone(),
            name: Some(testing::unique("commit-check")),
            command: Some(
                "sh -c 'grep -q kept /committed.txt && test \"$GREETING\" = hello'".into(),
            ),
            ..Default::default()
        },
        None,
    )
    .await
    .unwrap();
    let exit = loop {
        let state = docker
            .inspect_container(&check, None)
            .await
            .unwrap()
            .state
            .unwrap();
        if state.running != Some(true) {
            break state.exit_code;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    };
    containers::remove(&docker, &check, true, true)
        .await
        .unwrap();
    assert_eq!(exit, Some(0));
    let running = docker.inspect_container(&guard.id, None).await.unwrap();
    assert_eq!(
        running.state.unwrap().running,
        Some(true),
        "the container keeps running"
    );
}

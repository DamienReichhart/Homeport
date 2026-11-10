use super::*;
use crate::testing;

fn spec(context: &Path) -> BuildSpec {
    BuildSpec {
        context: context.to_string_lossy().into_owned(),
        dockerfile: String::new(),
        build_args: vec![],
        target: None,
        tags: vec![],
        no_cache: false,
        pull: false,
    }
}

#[test]
fn names_and_arguments_are_checked() {
    let dir = Path::new("/tmp");
    assert!(validate(&BuildSpec {
        tags: vec!["app:1".into(), "registry.local:5000/app".into()],
        ..spec(dir)
    })
    .is_ok());
    let bad = validate(&BuildSpec {
        tags: vec!["my app".into()],
        ..spec(dir)
    })
    .unwrap_err();
    assert!(bad.message.contains("my app"));
    assert!(validate(&BuildSpec {
        build_args: vec![BuildArg {
            name: "A=B".into(),
            value: String::new()
        }],
        ..spec(dir)
    })
    .is_err());
}

#[test]
fn failed_steps_are_named_and_keep_the_output() {
    let tail: VecDeque<String> = ["Step 2/3 : RUN false".to_owned(), "oops".to_owned()].into();
    let e = failed(
        "The command '/bin/sh -c false' returned a non-zero code: 1",
        Some("Step 2/3 : RUN false"),
        &tail,
    );
    assert_eq!(e.code, ErrorCode::BuildFailed);
    assert_eq!(e.message, "The build failed at step 2/3 (RUN false).");
    let detail = e.detail.unwrap();
    assert!(detail.starts_with("The command '/bin/sh -c false'"));
    assert!(detail.ends_with("Step 2/3 : RUN false\noops"));

    let private = failed(
            "pull access denied for private/base, repository does not exist or may require 'docker login'",
            Some("Step 1/3 : FROM private/base"),
            &tail,
        );
    assert_eq!(private.code, ErrorCode::RegistryAuthRequired);
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn builds_with_arguments_a_target_and_several_tags() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
            dir.path().join("Dockerfile"),
            format!(
                "FROM {} AS base\nARG GREETING=none\nRUN echo \"$GREETING\" > /greeting\nCOPY kept.txt /\n\nFROM base AS extra\nRUN touch /extra\n",
                testing::IMAGE
            ),
        )
        .unwrap();
    std::fs::write(dir.path().join(".dockerignore"), "ignored.txt\n").unwrap();
    std::fs::write(dir.path().join("kept.txt"), "kept").unwrap();
    std::fs::write(dir.path().join("ignored.txt"), "ignored").unwrap();

    let name = testing::unique("build");
    let tags = vec![format!("{name}:one"), format!("{name}:two")];
    let _one = testing::ImageName(docker.clone(), tags[0].clone());
    let _two = testing::ImageName(docker.clone(), tags[1].clone());
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    build(
        docker.clone(),
        BuildSpec {
            build_args: vec![BuildArg {
                name: "GREETING".into(),
                value: "hello".into(),
            }],
            target: Some("base".into()),
            tags: tags.clone(),
            ..spec(dir.path())
        },
        vec![],
        move |e| tx.send(e).is_ok(),
    )
    .await;
    let mut events = Vec::new();
    while let Ok(e) = rx.try_recv() {
        events.push(e);
    }
    assert!(
        matches!(
            events.last(),
            Some(BuildEvent::Done {
                image_id: Some(_),
                ..
            })
        ),
        "{events:?}"
    );
    let image = docker.inspect_image(&tags[1]).await.unwrap();
    assert!(image.repo_tags.unwrap().contains(&tags[0]));

    // The target stopped before `extra`, the argument reached RUN, and
    // the ignored file stayed on this machine.
    let check = crate::containers::run::run(
            &docker,
            crate::containers::run::RunSpec {
                image: tags[0].clone(),
                name: Some(testing::unique("build-check")),
                command: Some(
                    "sh -c 'test ! -e /extra && grep -q hello /greeting && test -e /kept.txt && test ! -e /ignored.txt'"
                        .into(),
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
    crate::containers::remove(&docker, &check, true, true)
        .await
        .unwrap();
    assert_eq!(exit, Some(0));
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn failed_builds_name_their_step() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Dockerfile"),
        format!("FROM {}\nRUN false\n", testing::IMAGE),
    )
    .unwrap();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    build(docker, spec(dir.path()), vec![], move |e| {
        tx.send(e).is_ok()
    })
    .await;
    let mut last = None;
    while let Ok(e) = rx.try_recv() {
        last = Some(e);
    }
    match last {
        Some(BuildEvent::Error { error }) => {
            assert_eq!(error.code, ErrorCode::BuildFailed, "{error:?}");
            assert!(error.message.contains("RUN false"), "{error:?}");
        }
        other => panic!("{other:?}"),
    }
}

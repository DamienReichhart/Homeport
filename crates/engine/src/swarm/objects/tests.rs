use bollard::models::{ServiceSpec, TaskSpec, TaskSpecContainerSpec, TaskSpecContainerSpecSecrets};

use super::*;

fn input(name: &str, text: &str) -> ObjectInput {
    ObjectInput {
        name: name.into(),
        value: ObjectValue::Text { text: text.into() },
        labels: HashMap::new(),
    }
}

#[test]
fn debug_never_shows_a_value() {
    let shown = format!("{:?}", input("db_password", "hunter2"));
    assert!(!shown.contains("hunter2"), "{shown}");
    assert!(shown.contains("7 bytes"), "{shown}");
}

#[test]
fn names_and_values_are_checked_before_sending() {
    assert!(prepare(&input("db_password", "x")).is_ok());
    for bad in ["", "-dash", "has space", "slash/no"] {
        assert!(prepare(&input(bad, "x")).is_err(), "{bad}");
    }
    assert!(prepare(&input("empty", "")).is_err());
    assert!(prepare(&input("big", &"x".repeat(MAX_SIZE + 1))).is_err());
    assert_eq!(prepare(&input("a", "hi")).unwrap(), "aGk=");
}

#[test]
fn values_can_come_from_a_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("key.pem");
    std::fs::write(&path, b"\x00binary").unwrap();
    let file = ObjectInput {
        name: "key".into(),
        value: ObjectValue::File {
            path: path.to_string_lossy().into_owned(),
        },
        labels: HashMap::new(),
    };
    assert_eq!(
        STANDARD.decode(prepare(&file).unwrap()).unwrap(),
        b"\x00binary"
    );
}

#[test]
fn configs_decode_as_text_or_size() {
    assert_eq!(
        decode("aGk="),
        ConfigContent {
            text: Some("hi".into()),
            size: 2
        }
    );
    assert_eq!(decode(&STANDARD.encode([0xff, 0xfe])).text, None);
}

#[test]
fn finds_the_services_using_an_object() {
    let service = |name: &str, secret: &str| Service {
        spec: Some(ServiceSpec {
            name: Some(name.into()),
            task_template: Some(TaskSpec {
                container_spec: Some(TaskSpecContainerSpec {
                    secrets: Some(vec![TaskSpecContainerSpecSecrets {
                        secret_id: Some(secret.into()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    let used = users(&[service("web", "s1"), service("api", "s1")], Kind::Secret);
    assert_eq!(used["s1"], ["web", "api"]);
    assert!(users(&[service("web", "s1")], Kind::Config).is_empty());
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn secrets_and_configs_round_trip() {
    let docker = crate::testing::docker();
    let _swarm = crate::testing::swarm(&docker).await;
    let secret_name = crate::testing::unique("secret");
    let id = create_secret(&docker, input(&secret_name, "hunter2"))
        .await
        .unwrap();
    let listed = secrets(&docker).await.unwrap();
    assert!(listed.iter().any(|s| s.id == id && s.name == secret_name));
    let again = create_secret(&docker, input(&secret_name, "x")).await;
    assert!(
        matches!(&again, Err(e) if e.is(ErrorCode::NameInUse)),
        "{again:?}"
    );

    // A service using it keeps it, and the refusal names the service.
    let service_name = crate::testing::unique("secret-user");
    let service = docker
        .create_service(
            ServiceSpec {
                name: Some(service_name.clone()),
                task_template: Some(TaskSpec {
                    container_spec: Some(TaskSpecContainerSpec {
                        image: Some(crate::testing::IMAGE.into()),
                        command: Some(vec!["sleep".into(), "300".into()]),
                        secrets: Some(vec![TaskSpecContainerSpecSecrets {
                            secret_id: Some(id.clone()),
                            secret_name: Some(secret_name.clone()),
                            file: Some(bollard::models::TaskSpecContainerSpecFile {
                                name: Some("password".into()),
                                uid: Some("0".into()),
                                gid: Some("0".into()),
                                mode: Some(0o400),
                            }),
                        }]),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap()
        .id
        .unwrap();
    let refused = remove_secret(&docker, &id).await.unwrap_err();
    assert!(refused.is(ErrorCode::InUse), "{refused:?}");
    assert!(refused.hint.unwrap().text.contains(&service_name));
    docker.delete_service(&service).await.unwrap();
    remove_secret(&docker, &id).await.unwrap();

    let config_name = crate::testing::unique("config");
    let id = create_config(&docker, input(&config_name, "port: 80\n"))
        .await
        .unwrap();
    assert_eq!(
        config_content(&docker, &id).await.unwrap().text.as_deref(),
        Some("port: 80\n")
    );
    remove_config(&docker, &id).await.unwrap();
    assert!(configs(&docker).await.unwrap().iter().all(|c| c.id != id));
}

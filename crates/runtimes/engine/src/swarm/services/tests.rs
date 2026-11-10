use bollard::models::{
    ObjectVersion, ServiceServiceStatus, ServiceSpec, ServiceUpdateStatus,
    ServiceUpdateStatusStateEnum, TaskSpec, TaskSpecContainerSpec,
};

use super::*;

fn service(mode: ServiceSpecMode) -> Service {
    Service {
        id: Some("s1".into()),
        version: Some(ObjectVersion { index: Some(9) }),
        spec: Some(ServiceSpec {
            name: Some("shop_web".into()),
            labels: Some([(STACK_NAMESPACE.to_string(), "shop".to_string())].into()),
            mode: Some(mode),
            task_template: Some(TaskSpec {
                container_spec: Some(TaskSpecContainerSpec {
                    image: Some("nginx:1.27@sha256:abc".into()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }),
        endpoint: Some(bollard::models::ServiceEndpoint {
            ports: Some(vec![EndpointPortConfig {
                target_port: Some(80),
                published_port: Some(8080),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        service_status: Some(ServiceServiceStatus {
            running_tasks: Some(1),
            desired_tasks: Some(3),
            completed_tasks: None,
        }),
        update_status: Some(ServiceUpdateStatus {
            state: Some(ServiceUpdateStatusStateEnum::UPDATING),
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[test]
fn maps_a_replicated_service_of_a_stack() {
    let v = view(service(ServiceSpecMode {
        replicated: Some(ServiceSpecModeReplicated { replicas: Some(3) }),
        ..Default::default()
    }));
    assert_eq!(v.name, "shop_web");
    assert_eq!(v.version, 9);
    assert_eq!(v.image, "nginx:1.27");
    assert_eq!(v.image_digest.as_deref(), Some("sha256:abc"));
    assert_eq!((v.mode, v.replicas), (ServiceMode::Replicated, Some(3)));
    assert_eq!((v.running, v.desired), (1, 3));
    assert_eq!(v.stack.as_deref(), Some("shop"));
    assert_eq!(v.update_state.as_deref(), Some("updating"));
    assert_eq!(
        v.ports,
        [ServicePort {
            target: 80,
            published: Some(8080),
            protocol: "tcp".into(),
            mode: "ingress".into()
        }]
    );
}

#[test]
fn global_services_have_no_replica_count() {
    let v = view(service(ServiceSpecMode {
        global: Some(HashMap::new()),
        ..Default::default()
    }));
    assert_eq!((v.mode, v.replicas), (ServiceMode::Global, None));
    assert_eq!(split_digest("redis"), ("redis".into(), None));
}

#[test]
fn replicas_have_a_sane_limit() {
    assert!(check_replicas(0).is_ok());
    assert!(check_replicas(10_001).is_err());
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn a_service_scales_rolls_back_logs_and_goes() {
    use std::sync::{Arc, Mutex};

    use bollard::models::ServiceSpec;

    let docker = crate::testing::docker();
    let _swarm = crate::testing::swarm(&docker).await;
    crate::testing::ensure_image(&docker).await;
    let name = crate::testing::unique("svc");
    let created = docker
        .create_service(
            ServiceSpec {
                name: Some(name.clone()),
                task_template: Some(TaskSpec {
                    container_spec: Some(TaskSpecContainerSpec {
                        image: Some(crate::testing::IMAGE.into()),
                        command: Some(vec![
                            "sh".into(),
                            "-c".into(),
                            "echo hello-from-task; sleep 300".into(),
                        ]),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
    let id = created.id.unwrap();
    let find = || async {
        list(&docker)
            .await
            .unwrap()
            .into_iter()
            .find(|s| s.id == id)
            .unwrap()
    };
    let first = find().await;
    assert_eq!(first.replicas, Some(1));

    scale(&docker, &id, first.version, 2).await.unwrap();
    let mut scaled = find().await;
    for _ in 0..100 {
        if scaled.running == 2 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
        scaled = find().await;
    }
    assert_eq!((scaled.replicas, scaled.running), (Some(2), 2));

    rollback(&docker, &id).await.unwrap();
    assert_eq!(find().await.replicas, Some(1));

    let lines = Arc::new(Mutex::new(Vec::new()));
    let sink = {
        let lines = lines.clone();
        move |e| {
            if let LogEvent::Lines { lines: l } = e {
                lines.lock().unwrap().extend(l);
            }
            true
        }
    };
    let options = LogOptions {
        tail: None,
        timestamps: false,
        since: None,
        stdout: true,
        stderr: true,
        follow: false,
    };
    stream_logs(docker.clone(), id.clone(), options, sink).await;
    let source = lines
        .lock()
        .unwrap()
        .iter()
        .find(|l| l.text == "hello-from-task")
        .and_then(|l| l.source.clone())
        .unwrap();
    assert!(source.starts_with(&format!("{name}.")), "{source}");

    remove(&docker, &id).await.unwrap();
    assert!(list(&docker).await.unwrap().iter().all(|s| s.id != id));
}

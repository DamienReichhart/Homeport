use super::*;

fn input() -> ServiceSpecInput {
    ServiceSpecInput {
        name: "web".into(),
        image: "nginx:1.27".into(),
        mode: ModeInput::Replicated { replicas: 2 },
        env: vec!["A=1".into()],
        command: Some("nginx -g 'daemon off;'".into()),
        ports: vec![PortInput {
            target: 80,
            published: Some(8080),
            protocol: "tcp".into(),
            host_mode: false,
        }],
        networks: vec!["backend".into()],
        mounts: vec![
            MountInput {
                source: "web-data".into(),
                target: "/data".into(),
                read_only: false,
            },
            MountInput {
                source: "/etc/ssl".into(),
                target: "/ssl".into(),
                read_only: true,
            },
        ],
        secrets: vec![ObjectRef {
            name: "db_password".into(),
            target: None,
        }],
        configs: vec![ObjectRef {
            name: "nginx_conf".into(),
            target: Some("/etc/nginx/nginx.conf".into()),
        }],
        restart: Some(RestartCondition::OnFailure),
    }
}

fn ids() -> ObjectIds {
    ObjectIds {
        secrets: [("db_password".to_string(), "s1".to_string())].into(),
        configs: [("nginx_conf".to_string(), "c1".to_string())].into(),
    }
}

#[test]
fn a_form_becomes_a_service_spec() {
    let spec = spec_body(&input(), &ids()).unwrap();
    assert_eq!(spec.name.as_deref(), Some("web"));
    assert_eq!(
        mode_of(spec.mode.as_ref()),
        (ServiceMode::Replicated, Some(2))
    );
    let task = spec.task_template.unwrap();
    let c = task.container_spec.unwrap();
    assert_eq!(c.image.as_deref(), Some("nginx:1.27"));
    assert_eq!(c.args.unwrap(), ["nginx", "-g", "daemon off;"]);
    let mounts = c.mounts.unwrap();
    assert_eq!(mounts[0].typ, Some(MountType::VOLUME));
    assert_eq!(mounts[1].typ, Some(MountType::BIND));
    assert_eq!(mounts[1].read_only, Some(true));
    let secret = &c.secrets.unwrap()[0];
    assert_eq!(secret.secret_id.as_deref(), Some("s1"));
    assert_eq!(
        secret.file.as_ref().unwrap().name.as_deref(),
        Some("db_password")
    );
    let config = &c.configs.unwrap()[0];
    assert_eq!(
        config.file.as_ref().unwrap().name.as_deref(),
        Some("/etc/nginx/nginx.conf")
    );
    assert_eq!(task.networks.unwrap()[0].target.as_deref(), Some("backend"));
    assert_eq!(
        task.restart_policy.unwrap().condition,
        Some(TaskSpecRestartPolicyConditionEnum::ON_FAILURE)
    );
    let port = &spec.endpoint_spec.unwrap().ports.unwrap()[0];
    assert_eq!(
        (port.target_port, port.published_port),
        (Some(80), Some(8080))
    );
    assert_eq!(
        port.publish_mode,
        Some(EndpointPortConfigPublishModeEnum::INGRESS)
    );
}

#[test]
fn bad_forms_are_refused_before_sending() {
    let refused = |change: fn(&mut ServiceSpecInput)| {
        let mut i = input();
        change(&mut i);
        spec_body(&i, &ids()).unwrap_err()
    };
    assert!(refused(|i| i.name = "-web".into()).is(ErrorCode::Validation));
    assert!(refused(|i| i.name = "has space".into()).is(ErrorCode::Validation));
    assert!(refused(|i| i.image = " ".into()).is(ErrorCode::Validation));
    assert!(refused(|i| i.env = vec!["NOVALUE".into()]).is(ErrorCode::Validation));
    assert!(refused(|i| i.ports.push(i.ports[0].clone())).is(ErrorCode::Validation));
    assert!(refused(|i| i.ports[0].protocol = "http".into()).is(ErrorCode::Validation));
    assert!(refused(|i| {
        i.ports[0].published = None;
        i.ports[0].host_mode = true;
    })
    .is(ErrorCode::Validation));
    assert!(refused(|i| i.mounts[0].target = "data".into()).is(ErrorCode::Validation));
    assert!(refused(|i| i.secrets[0].name = "nope".into()).is(ErrorCode::NotFound));
    // The same port on another protocol is fine.
    let mut both = input();
    both.ports.push(PortInput {
        protocol: "udp".into(),
        ..both.ports[0].clone()
    });
    assert!(spec_body(&both, &ids()).is_ok());
    let mut global = input();
    global.mode = ModeInput::Global;
    assert_eq!(
        mode_of(spec_body(&global, &ids()).unwrap().mode.as_ref()).0,
        ServiceMode::Global
    );
}

#[test]
fn an_update_changes_only_its_three_fields() {
    let mut current = spec_body(&input(), &ids()).unwrap();
    current
        .task_template
        .as_mut()
        .unwrap()
        .container_spec
        .as_mut()
        .unwrap()
        .image = Some("nginx:1.27@sha256:abc".into());
    let changed = merge(
        current.clone(),
        &ServiceChange {
            image: Some("nginx:1.28".into()),
            env: None,
            replicas: None,
            redeploy: false,
        },
    )
    .unwrap();
    let image = |s: &ServiceSpec| {
        s.task_template
            .as_ref()
            .unwrap()
            .container_spec
            .as_ref()
            .unwrap()
            .image
            .clone()
    };
    assert_eq!(image(&changed).as_deref(), Some("nginx:1.28"));
    // Only the image differs.
    let mut back = changed.clone();
    back.task_template
        .as_mut()
        .unwrap()
        .container_spec
        .as_mut()
        .unwrap()
        .image = image(&current);
    assert_eq!(back, current);

    let env_only = merge(
        current.clone(),
        &ServiceChange {
            image: Some("nginx:1.27".into()),
            env: Some(vec!["B=2".into()]),
            replicas: Some(5),
            redeploy: false,
        },
    )
    .unwrap();
    assert_eq!(
        image(&env_only),
        image(&current),
        "an unchanged tag keeps its digest"
    );
    let c = env_only
        .task_template
        .as_ref()
        .unwrap()
        .container_spec
        .as_ref()
        .unwrap();
    assert_eq!(c.env.as_deref(), Some(&["B=2".to_string()][..]));
    assert_eq!(
        c.mounts,
        current
            .task_template
            .as_ref()
            .unwrap()
            .container_spec
            .as_ref()
            .unwrap()
            .mounts
    );
    assert_eq!(env_only.endpoint_spec, current.endpoint_spec);
    assert_eq!(mode_of(env_only.mode.as_ref()).1, Some(5));
}

#[test]
fn redeploying_bumps_force_update_and_pulls_the_tag_again() {
    let mut current = spec_body(&input(), &ids()).unwrap();
    current
        .task_template
        .as_mut()
        .unwrap()
        .container_spec
        .as_mut()
        .unwrap()
        .image = Some("nginx:1.27@sha256:abc".into());
    let redeploy = ServiceChange {
        redeploy: true,
        ..Default::default()
    };
    let once = merge(current, &redeploy).unwrap();
    let task = once.task_template.clone().unwrap();
    assert_eq!(task.force_update, Some(1));
    assert_eq!(
        task.container_spec.unwrap().image.as_deref(),
        Some("nginx:1.27")
    );
    assert_eq!(
        merge(once, &redeploy)
            .unwrap()
            .task_template
            .unwrap()
            .force_update,
        Some(2)
    );
}

#[test]
fn replicas_of_a_global_service_are_refused() {
    let mut global = input();
    global.mode = ModeInput::Global;
    let spec = spec_body(&global, &ids()).unwrap();
    let change = ServiceChange {
        replicas: Some(2),
        ..Default::default()
    };
    assert!(merge(spec, &change).is_err());
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn a_created_service_updates_its_image_and_env() {
    let docker = crate::testing::docker();
    let _swarm = crate::testing::swarm(&docker).await;
    crate::testing::ensure_image(&docker).await;
    let name = crate::testing::unique("create");
    let input = ServiceSpecInput {
        name: name.clone(),
        image: crate::testing::IMAGE.into(),
        mode: ModeInput::Replicated { replicas: 1 },
        env: vec!["GREETING=hello".into()],
        command: Some("sleep 300".into()),
        ports: vec![],
        networks: vec![],
        mounts: vec![],
        secrets: vec![],
        configs: vec![],
        restart: None,
    };
    let id = create(&docker, &input).await.unwrap();
    let again = create(&docker, &input).await;
    assert!(
        matches!(&again, Err(e) if e.is(ErrorCode::NameInUse)),
        "{again:?}"
    );

    let version = |docker: Docker, id: String| async move {
        docker
            .inspect_service(&id, None)
            .await
            .unwrap()
            .version
            .unwrap()
            .index
            .unwrap()
    };
    let v1 = version(docker.clone(), id.clone()).await;
    let change = ServiceChange {
        image: Some("busybox:1.37".into()),
        env: Some(vec!["GREETING=bonjour".into()]),
        replicas: None,
        redeploy: false,
    };
    update(&docker, &id, v1, &change).await.unwrap();
    // The same edit from the stale version is a conflict.
    let stale = update(&docker, &id, v1, &change).await;
    assert!(
        matches!(&stale, Err(e) if e.is(ErrorCode::Conflict)),
        "{stale:?}"
    );

    let spec = docker
        .inspect_service(&id, None)
        .await
        .unwrap()
        .spec
        .unwrap();
    let c = spec.task_template.unwrap().container_spec.unwrap();
    assert!(c.image.unwrap().starts_with("busybox:1.37"));
    assert_eq!(c.env.unwrap(), ["GREETING=bonjour"]);
    assert_eq!(c.args.unwrap(), ["sleep", "300"]);
    docker.delete_service(&id).await.unwrap();
}

use super::*;

fn service(name: &str, stack: Option<&str>, running: u64, desired: u64) -> ServiceView {
    ServiceView {
        id: format!("{name}-id"),
        version: 1,
        name: name.into(),
        image: "nginx".into(),
        image_digest: None,
        mode: homeport_engine::swarm::services::ServiceMode::Replicated,
        replicas: Some(desired),
        running,
        desired,
        ports: vec![],
        update_state: None,
        update_message: None,
        stack: stack.map(str::to_owned),
        labels: HashMap::new(),
        created: None,
        updated: None,
    }
}

#[test]
fn services_group_into_stacks_by_namespace() {
    let stacks = derive_swarm(
        &[
            service("shop_web", Some("shop"), 2, 2),
            service("shop_db", Some("shop"), 0, 1),
            service("blog_web", Some("blog"), 1, 1),
            service("idle_app", Some("idle"), 0, 0),
            service("lonely", None, 1, 1),
        ],
        &[
            StackRecord::managed("m1", "blog"),
            StackRecord::managed("m2", "fresh"),
        ],
    );
    let names: Vec<&str> = stacks.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(names, ["blog", "fresh", "idle", "shop"]);
    let shop = &stacks[3];
    assert_eq!(shop.kind, StackKind::External);
    assert_eq!(shop.state, StackState::Partial);
    assert_eq!((shop.running, shop.total), (2, 3));
    let services: Vec<&str> = shop.services.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(services, ["web", "db"]);
    assert_eq!(shop.services[1].desired, Some(1));
    assert_eq!(shop.services[1].service_id.as_deref(), Some("shop_db-id"));
    assert_eq!(stacks[0].kind, StackKind::Managed);
    assert_eq!(stacks[0].state, StackState::Running);
    assert_eq!(stacks[1].state, StackState::NotDeployed);
    assert_eq!(stacks[2].state, StackState::Stopped);
}

#[test]
fn a_folder_links_only_when_its_services_match() {
    let running = [
        service("shop_web", Some("shop"), 1, 1),
        service("shop_db", Some("shop"), 1, 1),
        service("blog_web", Some("blog"), 1, 1),
    ];
    let both = "services:\n  web:\n    image: nginx\n  db:\n    image: postgres\n";
    assert!(check_link("shop", &running, &[both]).is_ok());
    let one = "services:\n  web:\n    image: nginx\n";
    let refused = check_link("shop", &running, &[one]).unwrap_err();
    assert!(refused.message.contains("db, web"), "{}", refused.message);
    // Merged like compose: an override's service counts.
    let db = "services:\n  db:\n    image: postgres\n";
    assert!(check_link("shop", &running, &[one, db]).is_ok());
}

#[test]
fn removal_follows_docker_stack_rm() {
    assert_eq!(
        REMOVAL_ORDER,
        [Part::Service, Part::Network, Part::Secret, Part::Config]
    );
    assert_eq!(short_name("shop", "shop_web"), "web");
    assert_eq!(short_name("shop", "other_web"), "other_web");
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn a_stack_deployed_from_a_terminal_is_found_and_removed() {
    use homeport_engine::{swarm::services, testing};

    let docker = testing::docker();
    let _swarm = testing::swarm(&docker).await;
    testing::ensure_image(&docker).await;
    let name = testing::unique("stack").replace('-', "");
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.conf"), "port: 80\n").unwrap();
    let compose = format!(
            "services:\n  app:\n    image: {}\n    command: [\"sleep\", \"300\"]\n    networks: [back]\n    configs:\n      - source: conf\n        target: /app.conf\nnetworks:\n  back: {{}}\nconfigs:\n  conf:\n    file: ./app.conf\n",
            testing::IMAGE
        );
    std::fs::write(dir.path().join("compose.yaml"), compose).unwrap();
    let out = std::process::Command::new("docker")
        .args(["stack", "deploy", "--compose-file", "compose.yaml", &name])
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let listed = services::list(&docker).await.unwrap();
    let stacks = derive_swarm(&listed, &[]);
    let stack = stacks.iter().find(|s| s.name == name).unwrap();
    assert_eq!(stack.kind, StackKind::External);
    assert_eq!(stack.services[0].name, "app");
    let found = parts(&docker, &name).await.unwrap();
    assert_eq!(found.networks, [format!("{name}_back")]);
    assert_eq!(found.configs, [format!("{name}_conf")]);

    let report = remove(&docker, &name).await.unwrap();
    assert!(report.kept.is_empty(), "{report:?}");
    assert_eq!(
        parts(&docker, &name).await.unwrap(),
        SwarmStackParts::default()
    );
    let left = services::list(&docker).await.unwrap();
    assert!(left
        .iter()
        .all(|s| s.stack.as_deref() != Some(name.as_str())));
}

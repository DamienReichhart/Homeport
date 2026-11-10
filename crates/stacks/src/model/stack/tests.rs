use std::collections::HashMap;

use super::*;

fn container(id: &str, stack: Option<&str>, service: &str, state: &str) -> Container {
    Container {
        id: id.into(),
        name: format!("{}-{service}-1", stack.unwrap_or("solo")),
        image: "img".into(),
        image_id: "sha".into(),
        command: String::new(),
        created: 0,
        state: state.into(),
        status: String::new(),
        health: None,
        ports: vec![],
        labels: HashMap::new(),
        stack: stack.map(str::to_owned),
        service: Some(service.into()),
        networks: vec![],
    }
}

#[test]
fn containers_group_into_stacks_by_project() {
    let containers = [
        container("1", Some("shop"), "web", "running"),
        container("2", Some("shop"), "web", "running"),
        container("3", Some("shop"), "db", "exited"),
        container("4", Some("blog"), "app", "exited"),
        container("5", None, "lonely", "running"),
    ];
    let stacks = derive(&containers, &[StackRecord::managed("m1", "shop")]);
    assert_eq!(stacks.len(), 2);

    let blog = &stacks[0];
    assert_eq!(
        (blog.name.as_str(), blog.kind, blog.state),
        ("blog", StackKind::External, StackState::Stopped)
    );

    let shop = &stacks[1];
    assert_eq!(shop.kind, StackKind::Managed);
    assert_eq!(shop.id.as_deref(), Some("m1"));
    assert_eq!(
        (shop.running, shop.total, shop.state),
        (2, 3, StackState::Partial)
    );
    let services: Vec<_> = shop
        .services
        .iter()
        .map(|s| (s.name.as_str(), s.container_ids.len(), s.running))
        .collect();
    assert_eq!(services, [("db", 1, 0), ("web", 2, 2)]);
}

#[test]
fn managed_stacks_without_containers_are_not_deployed() {
    let stacks = derive(&[], &[StackRecord::managed("m1", "new")]);
    assert_eq!(stacks[0].state, StackState::NotDeployed);
    assert_eq!(stacks[0].kind, StackKind::Managed);
}

#[test]
fn all_three_kinds_come_from_containers_and_records() {
    let containers = [
        container("1", Some("shop"), "web", "running"),
        container("2", Some("blog"), "app", "running"),
        container("3", Some("misc"), "x", "running"),
    ];
    let records = [
        StackRecord::managed("m1", "shop"),
        StackRecord::linked("l1", "blog", "/srv/blog"),
        StackRecord::linked("l2", "draft", "/srv/draft"),
    ];
    let stacks = derive(&containers, &records);
    let kinds: Vec<_> = stacks
        .iter()
        .map(|s| (s.name.as_str(), s.kind, s.id.as_deref(), s.state))
        .collect();
    assert_eq!(
        kinds,
        [
            ("blog", StackKind::Linked, Some("l1"), StackState::Running),
            (
                "draft",
                StackKind::Linked,
                Some("l2"),
                StackState::NotDeployed
            ),
            ("misc", StackKind::External, None, StackState::Running),
            ("shop", StackKind::Managed, Some("m1"), StackState::Running),
        ]
    );
    assert_eq!(stacks[0].working_dir.as_deref(), Some("/srv/blog"));
    assert_eq!(stacks[1].working_dir.as_deref(), Some("/srv/draft"));
}

#[test]
fn synced_stacks_come_from_their_records_deployed_or_not() {
    let containers = [
        container("1", Some("shop"), "web", "running"),
        container("2", Some("blog"), "app", "running"),
        container("3", Some("site"), "x", "exited"),
    ];
    let records = [
        StackRecord::managed("m1", "shop"),
        StackRecord::linked("l1", "blog", "/srv/blog"),
        StackRecord::synced("y1", "site"),
        StackRecord::synced("y2", "docs"),
    ];
    let kinds: Vec<_> = derive(&containers, &records)
        .iter()
        .map(|s| (s.name.clone(), s.kind, s.state))
        .collect();
    assert_eq!(
        kinds,
        [
            ("blog".into(), StackKind::Linked, StackState::Running),
            ("docs".into(), StackKind::Synced, StackState::NotDeployed),
            ("shop".into(), StackKind::Managed, StackState::Running),
            ("site".into(), StackKind::Synced, StackState::Stopped),
        ]
    );
    let external = derive(&containers, &[]);
    assert!(external.iter().all(|s| s.kind == StackKind::External));
}

#[test]
fn linked_stacks_without_their_files_show_as_external_with_why() {
    let records = [
        StackRecord::linked("l1", "blog", "/gone").with_files_missing(true),
        StackRecord::linked("l2", "draft", "/gone").with_files_missing(true),
    ];
    let stacks = derive(&[container("1", Some("blog"), "app", "running")], &records);
    for s in &stacks {
        assert_eq!(s.kind, StackKind::External);
        assert_eq!(s.problem, Some(StackProblem::FilesMissing));
        assert!(s.id.is_some(), "it can still be forgotten");
    }
    assert_eq!(stacks[1].state, StackState::NotDeployed);
}

#[test]
fn fully_running_stacks_are_running() {
    let stacks = derive(&[container("1", Some("a"), "x", "running")], &[]);
    assert_eq!(stacks[0].state, StackState::Running);
}

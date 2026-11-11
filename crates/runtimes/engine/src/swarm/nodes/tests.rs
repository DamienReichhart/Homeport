use bollard::models::{ManagerStatus as Bm, NodeDescription, NodeStatus, ObjectVersion};

use super::*;

fn node(id: &str, role: SwarmRole, availability: Availability, state: &str) -> NodeView {
    NodeView {
        id: id.into(),
        version: 1,
        hostname: id.into(),
        role,
        availability,
        state: state.into(),
        message: None,
        manager_status: (role == SwarmRole::Manager).then_some(ManagerStatus::Reachable),
        engine_version: None,
        address: None,
        os: None,
        arch: None,
        labels: HashMap::new(),
        created: None,
    }
}

#[test]
fn maps_a_leader_node() {
    let v = view(Node {
        id: Some("n1".into()),
        version: Some(ObjectVersion { index: Some(42) }),
        spec: Some(NodeSpec {
            role: Some(NodeSpecRoleEnum::MANAGER),
            availability: Some(NodeSpecAvailabilityEnum::DRAIN),
            labels: Some([("zone".to_string(), "a".to_string())].into()),
            ..Default::default()
        }),
        description: Some(NodeDescription {
            hostname: Some("alpha".into()),
            ..Default::default()
        }),
        status: Some(NodeStatus {
            state: Some(NodeState::READY),
            addr: Some("10.0.0.1".into()),
            ..Default::default()
        }),
        manager_status: Some(Bm {
            leader: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    });
    assert_eq!(v.version, 42);
    assert_eq!(v.role, SwarmRole::Manager);
    assert_eq!(v.availability, Availability::Drain);
    assert_eq!(v.state, "ready");
    assert_eq!(v.manager_status, Some(ManagerStatus::Leader));
    assert_eq!(v.labels["zone"], "a");
    let worker = view(Node::default());
    assert_eq!(worker.role, SwarmRole::Worker);
    assert_eq!(worker.manager_status, None);
    assert_eq!(worker.state, "unknown");
}

#[test]
fn the_last_reachable_manager_cant_be_demoted() {
    use SwarmRole::{Manager, Worker};
    let demote = NodeChange {
        role: Some(NodeRole::Worker),
        ..Default::default()
    };
    let mut nodes = vec![
        node("m1", Manager, Availability::Active, "ready"),
        node("w1", Worker, Availability::Active, "ready"),
    ];
    assert!(matches!(check(&nodes, "m1", &demote), Err(e) if e.is(ErrorCode::Validation)));
    nodes.push(node("m2", Manager, Availability::Active, "ready"));
    assert!(check(&nodes, "m1", &demote).is_ok());
    nodes[2].manager_status = Some(ManagerStatus::Unreachable);
    assert!(check(&nodes, "m1", &demote).is_err());
}

#[test]
fn draining_every_node_is_refused() {
    use SwarmRole::{Manager, Worker};
    let drain = NodeChange {
        availability: Some(Availability::Drain),
        ..Default::default()
    };
    let nodes = vec![
        node("m1", Manager, Availability::Active, "ready"),
        node("w1", Worker, Availability::Drain, "ready"),
        node("w2", Worker, Availability::Active, "down"),
    ];
    assert!(check(&nodes, "m1", &drain).is_err());
    assert!(check(&nodes, "w1", &drain).is_ok(), "already drained");
    assert!(check(&nodes, "w2", &drain).is_ok());
    assert!(check(&nodes, "gone", &drain).is_err());
}

#[test]
fn labels_need_a_key() {
    let nodes = vec![node(
        "m1",
        SwarmRole::Manager,
        Availability::Active,
        "ready",
    )];
    let change = NodeChange {
        labels: Some([(" ".to_string(), "x".to_string())].into()),
        ..Default::default()
    };
    assert!(check(&nodes, "m1", &change).is_err());
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn the_single_node_drains_and_comes_back() {
    let docker = crate::testing::docker();
    let _swarm = crate::testing::swarm(&docker).await;
    let node = list(&docker).await.unwrap().remove(0);
    // Pausing the only node is allowed; draining it wouldn't be.
    let drain = NodeChange {
        availability: Some(Availability::Pause),
        labels: Some([("homeport.test".to_string(), "1".to_string())].into()),
        ..Default::default()
    };
    check(std::slice::from_ref(&node), &node.id, &drain).unwrap();
    update(&docker, &node.id, node.version, drain)
        .await
        .unwrap();
    let paused = list(&docker).await.unwrap().remove(0);
    assert_eq!(paused.availability, Availability::Pause);
    assert_eq!(paused.labels["homeport.test"], "1");
    // A stale version is a conflict.
    let stale = update(&docker, &node.id, node.version, NodeChange::default()).await;
    assert!(
        matches!(&stale, Err(e) if e.is(ErrorCode::Conflict)),
        "{stale:?}"
    );
    let back = NodeChange {
        availability: Some(Availability::Active),
        labels: Some(HashMap::new()),
        ..Default::default()
    };
    update(&docker, &node.id, paused.version, back)
        .await
        .unwrap();
    assert_eq!(
        list(&docker).await.unwrap()[0].availability,
        Availability::Active
    );
}

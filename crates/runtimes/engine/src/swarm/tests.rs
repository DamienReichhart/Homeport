use bollard::models::{ClusterInfo, PeerNode, SwarmInfo};

use super::*;

fn info(swarm: SwarmInfo) -> SystemInfo {
    SystemInfo {
        swarm: Some(swarm),
        ..Default::default()
    }
}

#[test]
fn hosts_outside_a_swarm_have_no_membership() {
    assert_eq!(membership_from(&SystemInfo::default()), None);
    for state in [LocalNodeState::INACTIVE, LocalNodeState::EMPTY] {
        let i = info(SwarmInfo {
            local_node_state: Some(state),
            ..Default::default()
        });
        assert_eq!(membership_from(&i), None);
    }
}

#[test]
fn managers_know_their_swarm() {
    let m = membership_from(&info(SwarmInfo {
        node_id: Some("n1".into()),
        node_addr: Some("10.0.0.1".into()),
        local_node_state: Some(LocalNodeState::ACTIVE),
        control_available: Some(true),
        cluster: Some(ClusterInfo {
            id: Some("swarm1".into()),
            ..Default::default()
        }),
        remote_managers: Some(vec![PeerNode {
            node_id: Some("n1".into()),
            addr: Some("10.0.0.1:2377".into()),
        }]),
        ..Default::default()
    }))
    .unwrap();
    assert_eq!(m.swarm_id.as_deref(), Some("swarm1"));
    assert_eq!(m.role, SwarmRole::Manager);
    assert_eq!(m.state, SwarmNodeState::Active);
    assert_eq!(m.managers[0].addr, "10.0.0.1:2377");
}

#[test]
fn workers_only_know_their_managers() {
    let m = membership_from(&info(SwarmInfo {
        node_id: Some("n2".into()),
        local_node_state: Some(LocalNodeState::ACTIVE),
        control_available: Some(false),
        remote_managers: Some(vec![PeerNode {
            node_id: Some("n1".into()),
            addr: Some("10.0.0.1:2377".into()),
        }]),
        ..Default::default()
    }))
    .unwrap();
    assert_eq!(m.swarm_id, None);
    assert_eq!(m.role, SwarmRole::Worker);
    assert_eq!(m.managers[0].node_id, "n1");
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn a_swarm_manager_knows_its_swarm() {
    let docker = crate::testing::docker();
    let guard = crate::testing::swarm(&docker).await;
    let m = membership(&docker).await.unwrap().unwrap();
    assert_eq!(m.swarm_id.as_deref(), Some(guard.swarm_id.as_str()));
    assert_eq!(m.role, SwarmRole::Manager);
    let summary = summary(&docker).await.unwrap();
    assert_eq!(summary.id, guard.swarm_id);
    assert!(summary.nodes >= 1 && summary.managers >= 1);
    assert!(node_ids(&docker).await.unwrap().contains(&m.node_id));
}

#[test]
fn locked_and_failing_nodes_say_so() {
    let locked = membership_from(&info(SwarmInfo {
        node_id: Some("n1".into()),
        local_node_state: Some(LocalNodeState::LOCKED),
        ..Default::default()
    }))
    .unwrap();
    assert_eq!(locked.state, SwarmNodeState::Locked);
    assert_eq!(locked.swarm_id, None);
    let failing = membership_from(&info(SwarmInfo {
        local_node_state: Some(LocalNodeState::ERROR),
        error: Some("manager stopped".into()),
        ..Default::default()
    }))
    .unwrap();
    assert_eq!(failing.state, SwarmNodeState::Error);
    assert_eq!(failing.error.as_deref(), Some("manager stopped"));
}

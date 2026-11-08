use super::*;

/// `GET /1.0/instances?recursion=2` as Incus 6.0 answers it, with one
/// running Alpine container.
const LIST: &str = include_str!("../testdata/instances.json");

fn recorded() -> Vec<RawInstance> {
    let envelope: serde_json::Value = serde_json::from_str(LIST).unwrap();
    serde_json::from_value(envelope["metadata"].clone()).unwrap()
}

#[test]
fn a_running_container_reads_with_its_state() {
    let instances = recorded();
    let t1 = instances[0].summary();
    assert_eq!(t1.name, "t1");
    assert_eq!(t1.kind, InstanceType::Container);
    assert_eq!(t1.status, InstanceStatus::Running);
    assert!(t1.image.starts_with("Alpine 3.21"), "{}", t1.image);
    assert_eq!(t1.profiles, ["default"]);
    assert_eq!(t1.addresses.len(), 1, "{:?}", t1.addresses);
    assert!(t1.addresses[0].starts_with("10."));
    assert!(t1.memory_bytes.is_some_and(|m| m > 0));
    assert!(t1.cpu_usage_ns.is_some());
    assert_eq!(t1.snapshots, 0);
}

#[test]
fn a_stopped_virtual_machine_has_no_usage() {
    let raw: RawInstance = serde_json::from_value(serde_json::json!({
        "name": "vm1",
        "type": "virtual-machine",
        "status": "Stopped",
        "status_code": 102,
        "architecture": "x86_64",
        "profiles": ["default", "vm"],
        "config": { "image.os": "Debian", "image.release": "12" },
        "created_at": "2026-11-08T10:00:00Z",
        "last_used_at": "0001-01-01T00:00:00Z",
        "snapshots": [{ "name": "snap0" }, { "name": "snap1" }],
        "state": { "status_code": 102, "cpu": { "usage": 0 }, "memory": { "usage": 0 }, "processes": -1 }
    }))
    .unwrap();
    let vm = raw.summary();
    assert_eq!(vm.kind, InstanceType::VirtualMachine);
    assert_eq!(vm.status, InstanceStatus::Stopped);
    assert_eq!(vm.image, "Debian 12");
    assert_eq!(vm.last_used_at, None);
    assert_eq!(vm.cpu_usage_ns, None);
    assert_eq!(vm.processes, None);
    assert_eq!(vm.snapshots, 2);
}

#[test]
fn status_codes_map_to_states() {
    for (code, status) in [
        (103, InstanceStatus::Running),
        (102, InstanceStatus::Stopped),
        (110, InstanceStatus::Frozen),
        (106, InstanceStatus::Starting),
        (112, InstanceStatus::Error),
        (999, InstanceStatus::Unknown),
    ] {
        assert_eq!(InstanceStatus::from_code(code), status, "{code}");
    }
}

#[test]
fn graceful_stops_carry_a_timeout_and_forced_ones_dont() {
    assert_eq!(
        state_body(InstanceAction::Stop, false, Some(30)),
        serde_json::json!({ "action": "stop", "force": false, "timeout": 30 })
    );
    assert_eq!(
        state_body(InstanceAction::Freeze, false, None),
        serde_json::json!({ "action": "freeze", "force": false })
    );
}

#[test]
fn instance_types_are_incus_names() {
    assert_eq!(
        serde_json::to_value(InstanceType::VirtualMachine).unwrap(),
        "virtual-machine"
    );
    assert_eq!(path("my vm"), "/1.0/instances/my%20vm");
}

/// A client for the local Incus, for the live tests.
pub(crate) async fn live() -> Incus {
    let socket = std::env::var("HOMEPORT_INCUS_SOCKET")
        .unwrap_or_else(|_| "/var/lib/incus/unix.socket".into());
    let dialer = std::sync::Arc::new(homeport_transport::LocalDialer::for_engine(
        &socket,
        homeport_core::host::EngineKind::Incus,
    ));
    Incus::connect("local", dialer).await.unwrap()
}

/// The image the live tests make Instances from, small and quick to fetch.
pub(crate) fn live_image() -> String {
    std::env::var("HOMEPORT_INCUS_IMAGE").unwrap_or_else(|_| "images:alpine/3.21".into())
}

/// Runs the `incus` CLI on this machine, for the live tests' setup.
pub(crate) fn incus_cli(args: &[&str]) -> std::process::Output {
    std::process::Command::new("incus")
        .args(args)
        .output()
        .expect("the incus CLI runs")
}

#[tokio::test]
#[ignore = "needs a local Incus"]
async fn incus_live_instances_start_freeze_stop_and_delete() {
    let incus = live().await;
    let name = format!("homeport-test-{}", std::process::id());
    let made = incus_cli(&["init", &live_image(), &name]);
    assert!(made.status.success(), "{made:?}");

    act(&incus, "default", &name, InstanceAction::Start, false)
        .await
        .unwrap();
    let listed = list(&incus, "default").await.unwrap();
    let found = listed.iter().find(|i| i.name == name).unwrap();
    assert_eq!(found.status, InstanceStatus::Running);

    act(&incus, "default", &name, InstanceAction::Freeze, false)
        .await
        .unwrap();
    assert_eq!(
        get(&incus, "default", &name).await.unwrap().instance.status,
        InstanceStatus::Frozen
    );
    let again = act(&incus, "default", &name, InstanceAction::Freeze, false)
        .await
        .unwrap_err();
    assert_eq!(again.code, ErrorCode::AlreadyDone, "{again:?}");
    act(&incus, "default", &name, InstanceAction::Unfreeze, false)
        .await
        .unwrap();

    let refused = delete(&incus, "default", &name, false).await.unwrap_err();
    assert_eq!(refused.code, ErrorCode::ContainerRunning, "{refused:?}");
    delete(&incus, "default", &name, true).await.unwrap();
    let gone = get(&incus, "default", &name).await.unwrap_err();
    assert_eq!(gone.code, ErrorCode::NotFound);
}

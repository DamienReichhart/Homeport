use homeport_core::pull::PullEvent;

use super::create::create_body;
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

fn spec(source: InstanceSource) -> InstanceSpec {
    InstanceSpec {
        name: "web".into(),
        kind: InstanceType::Container,
        source,
        profiles: vec!["default".into()],
        cpu: Some(2),
        memory: Some("2GiB".into()),
        disk: Some("20GiB".into()),
        start: true,
    }
}

#[test]
fn a_new_instance_names_its_image_profiles_and_limits() {
    let remote = spec(InstanceSource::Remote {
        server: "https://images.linuxcontainers.org".into(),
        alias: "debian/12".into(),
    });
    assert_eq!(
        create_body(&remote, Some("default"), true),
        serde_json::json!({
            "name": "web",
            "type": "container",
            "profiles": ["default"],
            "config": { "limits.cpu": "2", "limits.memory": "2GiB" },
            "devices": { "root": { "type": "disk", "path": "/", "pool": "default", "size": "20GiB" } },
            "source": {
                "type": "image",
                "mode": "pull",
                "server": "https://images.linuxcontainers.org",
                "protocol": "simplestreams",
                "alias": "debian/12"
            },
            "start": true
        })
    );
    let mut local = spec(InstanceSource::Local {
        fingerprint: "4495673b".into(),
    });
    local.kind = InstanceType::VirtualMachine;
    local.cpu = None;
    local.memory = None;
    local.disk = None;
    let body = create_body(&local, None, false);
    assert_eq!(body["type"], "virtual-machine");
    assert_eq!(
        body["source"],
        serde_json::json!({ "type": "image", "fingerprint": "4495673b" })
    );
    assert_eq!(body["config"], serde_json::json!({}));
    assert_eq!(body["devices"], serde_json::json!({}));
    assert!(body.get("start").is_none());
}

#[test]
fn instance_names_are_hostname_labels() {
    for good in ["web", "db-1", "a"] {
        assert!(validate_name(good).is_ok(), "{good}");
    }
    for bad in [
        "",
        "1web",
        "-web",
        "web-",
        "my web",
        "web.lab",
        &"a".repeat(64),
    ] {
        assert!(validate_name(bad).is_err(), "{bad}");
    }
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

/// Held shared by every live test while it creates an Instance from the
/// live image, and alone by the images test while it deletes that image.
/// A creation unpacks Incus's cached copy of the image, so deleting it
/// mid-creation fails the creation with "Image is missing a rootfs" or "no
/// such file or directory": the images test must wait for creations in
/// flight, and creations started after it pull the image again.
pub(crate) static LIVE_IMAGE: tokio::sync::RwLock<()> = tokio::sync::RwLock::const_new(());

/// Makes a small container Instance named `name` from the live image,
/// stopped, through Homeport's own create.
pub(crate) async fn live_instance(incus: &Incus, name: &str) {
    let alias = live_image();
    let alias = alias.strip_prefix("images:").unwrap_or(&alias).to_owned();
    let spec = InstanceSpec {
        name: name.to_owned(),
        kind: InstanceType::Container,
        source: InstanceSource::Remote {
            server: crate::images::simplestreams::DEFAULT_SERVER.into(),
            alias,
        },
        profiles: vec!["default".into()],
        cpu: None,
        memory: None,
        disk: None,
        start: false,
    };
    let mut events = Vec::new();
    {
        let _creating = LIVE_IMAGE.read().await;
        create(incus, "default", &spec, |e| {
            events.push(e);
            true
        })
        .await;
    }
    assert_eq!(events.last(), Some(&PullEvent::Done), "{events:?}");
}

#[tokio::test]
#[ignore = "needs a local Incus"]
async fn incus_live_instances_start_freeze_stop_and_delete() {
    let incus = live().await;
    let name = format!("homeport-test-{}", std::process::id());
    live_instance(&incus, &name).await;
    // Its name is taken now.
    let alias = live_image();
    let mut taken = spec(InstanceSource::Remote {
        server: crate::images::simplestreams::DEFAULT_SERVER.into(),
        alias: alias.strip_prefix("images:").unwrap_or(&alias).to_owned(),
    });
    taken.name = name.clone();
    taken.disk = None;
    let mut events = Vec::new();
    {
        let _creating = LIVE_IMAGE.read().await;
        create(&incus, "default", &taken, |e| {
            events.push(e);
            true
        })
        .await;
    }
    assert!(
        matches!(events.last(), Some(PullEvent::Error { error }) if error.code == ErrorCode::NameInUse),
        "{events:?}"
    );

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

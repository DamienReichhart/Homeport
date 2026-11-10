use super::*;
use crate::error::ErrorCode;

fn ssh(host: &str, user: &str, port: u16, socket_path: &str) -> HostConnection {
    HostConnection::Ssh {
        host: host.into(),
        port,
        user: user.into(),
        socket_path: socket_path.into(),
        auth: SshAuth::Generated,
        public_key: None,
        bastions: vec![],
    }
}

fn bastion(host: &str, port: u16, user: &str, auth: BastionAuth) -> Bastion {
    Bastion {
        host: host.into(),
        port,
        user: user.into(),
        auth,
        public_key: None,
        host_key: None,
    }
}

fn behind(bastions: Vec<Bastion>) -> HostConnection {
    let HostConnection::Ssh {
        host,
        port,
        user,
        socket_path,
        auth,
        public_key,
        ..
    } = ssh("server", "deploy", 22, "/var/run/docker.sock")
    else {
        unreachable!()
    };
    HostConnection::Ssh {
        host,
        port,
        user,
        socket_path,
        auth,
        public_key,
        bastions,
    }
}

#[test]
fn accepts_well_formed_connections() {
    for c in [
        HostConnection::Local {
            socket_path: "/var/run/docker.sock".into(),
        },
        HostConnection::Tcp {
            address: "10.0.0.2:2375".into(),
        },
        HostConnection::Tls {
            address: "docker.example.com:2376".into(),
        },
        ssh("server", "deploy", 22, "/var/run/docker.sock"),
    ] {
        assert!(c.validate().is_ok(), "{c:?}");
    }
}

#[test]
fn rejects_unusable_connections() {
    for c in [
        HostConnection::Local {
            socket_path: " ".into(),
        },
        HostConnection::Tcp {
            address: "10.0.0.2".into(),
        },
        HostConnection::Tls {
            address: "host:notaport".into(),
        },
        ssh("", "deploy", 22, "/var/run/docker.sock"),
        ssh("server", "", 22, "/var/run/docker.sock"),
        ssh("server", "deploy", 0, "/var/run/docker.sock"),
        ssh("server", "deploy", 22, "docker.sock"),
    ] {
        assert!(
            matches!(c.validate(), Err(ref e) if e.is(ErrorCode::Validation)),
            "{c:?}"
        );
    }
}

#[test]
fn bastions_need_a_server_a_user_a_port_and_their_own_address() {
    let ok = behind(vec![
        bastion("jump", 22, "me", BastionAuth::SameAsHost),
        bastion("inner", 2222, "me", BastionAuth::Agent),
    ]);
    assert!(ok.validate().is_ok());
    for (bastions, n) in [
        (vec![bastion(" ", 22, "me", BastionAuth::Agent)], "1"),
        (
            vec![
                bastion("jump", 22, "me", BastionAuth::Agent),
                bastion("inner", 22, "", BastionAuth::Agent),
            ],
            "2",
        ),
        (vec![bastion("jump", 0, "me", BastionAuth::Agent)], "1"),
    ] {
        let err = behind(bastions).validate().unwrap_err();
        assert!(err.is(ErrorCode::Validation));
        assert_eq!(err.params.get("n").map(|p| p.to_string()), Some(n.into()));
    }
    // The same server twice, or the Host itself as a bastion.
    for bastions in [
        vec![
            bastion("jump", 22, "a", BastionAuth::Agent),
            bastion("jump", 22, "b", BastionAuth::Agent),
        ],
        vec![bastion("server", 22, "a", BastionAuth::Agent)],
    ] {
        assert!(behind(bastions).validate().is_err());
    }
}

#[test]
fn bastions_with_their_own_key_need_secrets_even_with_ssh_agent() {
    let HostConnection::Ssh {
        host,
        port,
        user,
        socket_path,
        public_key,
        ..
    } = ssh("server", "deploy", 22, "/var/run/docker.sock")
    else {
        unreachable!()
    };
    let agent = |bastions| HostConnection::Ssh {
        host: host.clone(),
        port,
        user: user.clone(),
        socket_path: socket_path.clone(),
        auth: SshAuth::Agent,
        public_key: public_key.clone(),
        bastions,
    };
    assert!(!agent(vec![]).needs_secrets());
    assert!(!agent(vec![bastion("jump", 22, "me", BastionAuth::SameAsHost)]).needs_secrets());
    assert!(agent(vec![bastion("jump", 22, "me", BastionAuth::Imported)]).needs_secrets());
}

#[test]
fn secrets_stored_before_bastions_read_with_none() {
    let stored = r#"{"kind":"ssh","private_key":"KEY","passphrase":null}"#;
    let s: HostSecrets = serde_json::from_str(stored).unwrap();
    assert!(s == HostSecrets::ssh_key("KEY".into(), None));
}

#[test]
fn ipv6_addresses_are_bracketed() {
    assert_eq!(ssh_address("10.0.0.1", 22), "10.0.0.1:22");
    assert_eq!(ssh_address("fd00::1", 2222), "[fd00::1]:2222");
}

#[test]
fn engine_kinds_are_lowercase_names() {
    for (kind, name) in [
        (EngineKind::Docker, "docker"),
        (EngineKind::Containerd, "containerd"),
        (EngineKind::Incus, "incus"),
    ] {
        assert_eq!(serde_json::to_value(kind).unwrap(), serde_json::json!(name));
        assert_eq!(
            serde_json::from_value::<EngineKind>(name.into()).unwrap(),
            kind
        );
        assert_eq!(EngineKind::parse(name), Some(kind));
        assert_eq!(kind.as_str(), name);
    }
    assert_eq!(EngineKind::parse("lxd"), None);
    assert_eq!(EngineKind::default(), EngineKind::Docker);
}

#[test]
fn every_engine_is_accepted_over_the_connections_it_answers_on() {
    let local = HostConnection::Local {
        socket_path: "/run/containerd/containerd.sock".into(),
    };
    let tcp = HostConnection::Tcp {
        address: "10.0.0.2:2375".into(),
    };
    let remote = ssh("server", "deploy", 22, "/run/containerd/containerd.sock");
    for c in [&local, &tcp, &remote] {
        assert!(EngineKind::Docker.validate(c).is_ok());
    }
    assert!(EngineKind::Containerd.validate(&local).is_ok());
    assert!(EngineKind::Containerd.validate(&remote).is_ok());
    let err = EngineKind::Containerd.validate(&tcp).unwrap_err();
    assert!(err.is(ErrorCode::Validation));
    assert!(err.message.contains("socket"), "{err:?}");
    let incus = HostConnection::Local {
        socket_path: "/var/lib/incus/unix.socket".into(),
    };
    assert!(EngineKind::Incus.validate(&incus).is_ok());
    assert!(EngineKind::Incus.validate(&remote).is_ok());
    let https = HostConnection::Tls {
        address: "10.0.0.5:8443".into(),
    };
    assert!(EngineKind::Incus.validate(&https).is_ok());
    assert!(EngineKind::Containerd.validate(&https).is_err());
    let err = EngineKind::Incus.validate(&tcp).unwrap_err();
    assert!(err.is(ErrorCode::Validation));
    assert!(err.message.contains("Incus"), "{err:?}");
}

#[test]
fn json_shape_is_tagged_by_kind() {
    let json = serde_json::to_value(HostConnection::Tcp {
        address: "a:1".into(),
    })
    .unwrap();
    assert_eq!(json, serde_json::json!({ "kind": "tcp", "address": "a:1" }));
}

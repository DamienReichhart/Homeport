use super::super::auth::Credentials;
use super::*;

fn target() -> SshTarget {
    SshTarget {
        label: "prod (deploy@prod.example:22)".into(),
        host: "prod.example".into(),
        port: 22,
        user: "deploy".into(),
        socket_path: "/var/run/docker.sock".into(),
        credentials: Credentials::Agent,
        trusted_host_key: None,
        bastions: vec![],
        engine: EngineKind::Docker,
    }
}

fn refused() -> russh::Error {
    russh::Error::ChannelOpenFailure(russh::ChannelOpenFailure::ConnectFailed)
}

#[test]
fn the_probe_tells_why_the_remote_socket_cant_be_opened() {
    for (probe, code, command) in [
        (
            Some(RemoteSocket::Missing),
            ErrorCode::EngineNotInstalled,
            "get.docker.com",
        ),
        (
            Some(RemoteSocket::Stopped),
            ErrorCode::EngineNotRunning,
            "systemctl enable --now docker",
        ),
        (
            Some(RemoteSocket::Denied),
            ErrorCode::PermissionDenied,
            "usermod -aG docker deploy",
        ),
        (
            Some(RemoteSocket::Usable),
            ErrorCode::RemoteEngineUnavailable,
            "AllowStreamLocalForwarding yes",
        ),
        (
            None,
            ErrorCode::RemoteEngineUnavailable,
            "usermod -aG docker deploy",
        ),
    ] {
        let e = socket_unavailable(&target(), probe, &refused());
        assert_eq!(e.code, code, "{probe:?}");
        assert!(e.message.contains("prod.example"), "{e:?}");
        let commands = e.hint.unwrap().commands.join("\n");
        assert!(commands.contains(command), "{probe:?}: {commands}");
    }
}

#[test]
fn a_prohibited_forward_means_sshd_forbids_it() {
    let e = socket_unavailable(
        &target(),
        None,
        &russh::Error::ChannelOpenFailure(russh::ChannelOpenFailure::AdministrativelyProhibited),
    );
    assert!(e.hint.unwrap().commands[0].contains("AllowStreamLocalForwarding yes"));
}

#[test]
fn the_probe_quotes_the_socket_path_and_reads_its_answer() {
    let script = probe_script("/tmp/it's.sock", EngineKind::Docker);
    assert!(script.contains(r"'/tmp/it'\''s.sock'"), "{script}");
    assert_eq!(RemoteSocket::parse("denied\n"), Some(RemoteSocket::Denied));
    assert_eq!(RemoteSocket::parse("bash: syntax error"), None);
}

#[test]
fn containerd_hosts_are_probed_and_explained_as_containerd() {
    let script = probe_script("/run/containerd/containerd.sock", EngineKind::Containerd);
    assert!(script.contains("command -v containerd"), "{script}");
    assert!(!script.contains("dockerd"), "{script}");
    let mut t = target();
    t.engine = EngineKind::Containerd;
    t.socket_path = "/run/containerd/containerd.sock".into();
    for (probe, code, command) in [
        (
            Some(RemoteSocket::Stopped),
            ErrorCode::EngineNotRunning,
            "systemctl enable --now containerd",
        ),
        (
            Some(RemoteSocket::Denied),
            ErrorCode::PermissionDenied,
            "usermod -aG containerd deploy",
        ),
        (
            None,
            ErrorCode::PermissionDenied,
            "usermod -aG containerd deploy",
        ),
    ] {
        let e = socket_unavailable(&t, probe, &refused());
        assert_eq!(e.code, code, "{probe:?}");
        let hint = e.hint.unwrap();
        assert!(!hint.text.contains("docker group"), "{hint:?}");
        assert!(
            hint.commands.join("\n").contains(command),
            "{probe:?}: {hint:?}"
        );
    }
    let missing = socket_unavailable(&t, Some(RemoteSocket::Missing), &refused());
    assert_eq!(missing.code, ErrorCode::EngineNotInstalled);
    assert!(missing.message.contains("containerd"), "{missing:?}");
}

#[test]
fn incus_hosts_are_probed_and_explained_as_incus() {
    let script = probe_script("/var/lib/incus/unix.socket", EngineKind::Incus);
    assert!(script.contains("command -v incusd"), "{script}");
    assert!(!script.contains("dockerd"), "{script}");
    let mut t = target();
    t.engine = EngineKind::Incus;
    t.socket_path = "/var/lib/incus/unix.socket".into();
    for (probe, code, command) in [
        (
            Some(RemoteSocket::Stopped),
            ErrorCode::EngineNotRunning,
            "systemctl enable --now incus.socket",
        ),
        (
            Some(RemoteSocket::Denied),
            ErrorCode::PermissionDenied,
            "usermod -aG incus-admin deploy",
        ),
    ] {
        let e = socket_unavailable(&t, probe, &refused());
        assert_eq!(e.code, code, "{probe:?}");
        let hint = e.hint.unwrap();
        assert!(!hint.text.contains("docker group"), "{hint:?}");
        assert!(hint.commands.join("\n").contains(command), "{hint:?}");
    }
    t.socket_path = "/var/lib/incus/unix.socket.user".into();
    let e = socket_unavailable(&t, Some(RemoteSocket::Denied), &refused());
    assert!(e.hint.unwrap().commands[0].contains("-aG incus deploy"));
    let missing = socket_unavailable(&t, Some(RemoteSocket::Missing), &refused());
    assert_eq!(missing.code, ErrorCode::EngineNotInstalled);
    assert!(missing.message.contains("Incus"), "{missing:?}");
}

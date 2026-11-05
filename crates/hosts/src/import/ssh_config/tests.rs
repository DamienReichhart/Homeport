use super::*;

/// A throwaway key made for these tests; its passphrase is "secret".
const ENCRYPTED: &str = include_str!("../../../../transport/src/ssh/testdata/encrypted_ed25519");

struct Home {
    dir: tempfile::TempDir,
    env: Env,
}

impl Home {
    fn new(config: &str) -> Self {
        let dir = tempfile::tempdir().unwrap();
        let ssh = dir.path().join(".ssh");
        std::fs::create_dir_all(&ssh).unwrap();
        std::fs::write(ssh.join("config"), config).unwrap();
        let env = Env {
            home: dir.path().to_path_buf(),
            local_user: "local".into(),
            docker_config: None,
        };
        Self { dir, env }
    }

    fn key(&self, name: &str, text: &str) {
        std::fs::write(self.dir.path().join(".ssh").join(name), text).unwrap();
    }

    fn config(&self) -> SshConfig {
        read(&self.env).unwrap()
    }

    fn plan(&self, alias: &str) -> AppResult<Plan> {
        plan(&self.config(), &self.env, alias)
    }
}

fn ssh_parts(c: &HostConnection) -> (&str, u16, &str, SshAuth, &[Bastion]) {
    match c {
        HostConnection::Ssh {
            host,
            port,
            user,
            auth,
            bastions,
            ..
        } => (host, *port, user, *auth, bastions),
        _ => panic!("expected SSH"),
    }
}

#[test]
fn an_alias_becomes_an_ssh_host_with_its_key() {
    let home = Home::new(
        "Host prod\n  HostName 10.0.0.5\n  User deploy\n  Port 2222\n  IdentityFile ~/.ssh/prod_%r\n",
    );
    let key = transport::ssh::generate_key("prod").private_key;
    home.key("prod_deploy", &key);
    let plan = home.plan("prod").unwrap();
    assert_eq!(
        ssh_parts(&plan.connection),
        ("10.0.0.5", 2222, "deploy", SshAuth::Imported, &[][..])
    );
    let file = plan.key.unwrap();
    assert_eq!(file.text, key);
    assert!(!file.encrypted);
    assert!(plan.warnings.is_empty());
}

#[test]
fn defaults_are_the_alias_the_local_user_port_22_and_the_agent() {
    let home = Home::new("Host box\n");
    let plan = home.plan("box").unwrap();
    assert_eq!(
        ssh_parts(&plan.connection),
        ("box", 22, "local", SshAuth::Agent, &[][..])
    );
    assert!(plan.key.is_none());
    match plan.connection {
        HostConnection::Ssh { socket_path, .. } => assert_eq!(socket_path, DEFAULT_SOCKET),
        _ => unreachable!(),
    }
}

#[test]
fn an_unreadable_key_falls_back_to_the_agent_with_a_warning() {
    let home = Home::new("Host box\n  IdentityFile ~/.ssh/missing\n");
    let plan = home.plan("box").unwrap();
    assert_eq!(ssh_parts(&plan.connection).3, SshAuth::Agent);
    assert_eq!(plan.warnings.len(), 1);
    assert!(
        plan.warnings[0].message.contains("missing"),
        "{:?}",
        plan.warnings
    );
}

#[test]
fn the_first_readable_identity_file_is_used() {
    let home = Home::new("Host box\n  IdentityFile ~/.ssh/missing\n  IdentityFile ~/.ssh/second\n");
    home.key("second", &transport::ssh::generate_key("s").private_key);
    let plan = home.plan("box").unwrap();
    assert!(plan.key.unwrap().path.ends_with("second"));
    assert!(plan.warnings.is_empty());
}

#[test]
fn encrypted_keys_are_flagged_for_a_passphrase() {
    let home = Home::new("Host box\n  IdentityFile ~/.ssh/enc\n");
    home.key("enc", ENCRYPTED);
    let drafts = drafts(&home.config(), &home.env, &[]);
    assert!(drafts[0].needs_passphrase);
    assert_eq!(drafts[0].files.len(), 1);
}

#[test]
fn proxy_jump_becomes_bastions_in_order_resolved_through_the_config() {
    let home = Home::new(
        "Host app\n  HostName 10.1.0.9\n  ProxyJump edge,core:2200\n  IdentityFile ~/.ssh/app\n\
         Host edge\n  HostName edge.example.com\n  User jump\n  IdentityFile ~/.ssh/edge\n\
         Host core\n  HostName core.internal\n  IdentityFile ~/.ssh/app\n",
    );
    home.key("app", &transport::ssh::generate_key("app").private_key);
    home.key("edge", &transport::ssh::generate_key("edge").private_key);
    let plan = home.plan("app").unwrap();
    let (_, _, _, auth, bastions) = ssh_parts(&plan.connection);
    assert_eq!(auth, SshAuth::Imported);
    let hops: Vec<_> = bastions
        .iter()
        .map(|b| (b.host.as_str(), b.port, b.user.as_str(), b.auth))
        .collect();
    assert_eq!(
        hops,
        [
            ("edge.example.com", 22, "jump", BastionAuth::Imported),
            ("core.internal", 2200, "local", BastionAuth::SameAsHost),
        ]
    );
    assert_eq!(plan.bastion_keys.len(), 1);
    assert_eq!(plan.bastion_keys[0].0, 0);
}

#[test]
fn a_jump_hosts_own_proxy_jump_comes_first() {
    let home = Home::new(
        "Host app\n  ProxyJump inner\nHost inner\n  ProxyJump outer\nHost outer\n  User me\n",
    );
    let plan = home.plan("app").unwrap();
    let hosts: Vec<_> = ssh_parts(&plan.connection)
        .4
        .iter()
        .map(|b| (b.host.clone(), b.auth))
        .collect();
    assert_eq!(
        hosts,
        [
            ("outer".to_owned(), BastionAuth::SameAsHost),
            ("inner".to_owned(), BastionAuth::SameAsHost)
        ]
    );
}

#[test]
fn proxy_jump_loops_are_reported_not_followed() {
    let home = Home::new("Host a\n  ProxyJump b\nHost b\n  ProxyJump a\n");
    let err = home.plan("a").unwrap_err();
    assert!(err.message.contains("loop"), "{err:?}");
    // The draft is offered, with the reason it can't be imported.
    let drafts = drafts(&home.config(), &home.env, &[]);
    assert!(drafts
        .iter()
        .all(|d| d.problem.is_some() && d.connection.is_none()));
}

#[test]
fn jump_specs_take_users_ports_and_ipv6() {
    let parse = |s| JumpSpec::parse(s).unwrap();
    assert_eq!(
        parse("me@jump:2222"),
        JumpSpec {
            user: Some("me".into()),
            host: "jump".into(),
            port: Some(2222)
        }
    );
    assert_eq!(parse("ssh://jump").host, "jump");
    assert_eq!(parse("[fd00::1]:22").host, "fd00::1");
    assert_eq!(parse("fd00::1").host, "fd00::1");
    assert!(JumpSpec::parse("jump:notaport").is_err());
}

#[test]
fn proxy_command_is_warned_about() {
    let home = Home::new("Host box\n  ProxyCommand nc -X 5 -x proxy:1080 %h %p\n");
    let plan = home.plan("box").unwrap();
    assert_eq!(plan.warnings.len(), 1);
    assert!(plan.warnings[0].message.contains("ProxyCommand"));
}

#[test]
fn wildcard_entries_are_not_offered_and_taken_names_are_flagged() {
    let home = Home::new("Host *.example.com\n  User x\nHost prod staging\nHost *\n  User y\n");
    let drafts = drafts(&home.config(), &home.env, &["PROD".into()]);
    let names: Vec<_> = drafts
        .iter()
        .map(|d| (d.source.as_str(), d.name_taken))
        .collect();
    assert_eq!(names, [("prod", true), ("staging", false)]);
}

#[test]
fn a_missing_config_says_so() {
    let dir = tempfile::tempdir().unwrap();
    let env = Env {
        home: dir.path().into(),
        local_user: "u".into(),
        docker_config: None,
    };
    let err = read(&env).unwrap_err();
    assert_eq!(err.code, ErrorCode::NotFound);
}

use super::*;

fn env_of(command: &Command) -> Vec<(String, Option<String>)> {
    command
        .as_std()
        .get_envs()
        .map(|(k, v)| {
            (
                k.to_string_lossy().into_owned(),
                v.map(|v| v.to_string_lossy().into_owned()),
            )
        })
        .collect()
}

fn value(env: &[(String, Option<String>)], key: &str) -> Option<String> {
    env.iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| v.clone())
}

#[tokio::test]
async fn tokens_go_in_a_header_through_the_environment_only() {
    let secret = Secret::Token {
        username: "bot".into(),
        token: "hunter2".into(),
    };
    let session = Session::remote(&Remote {
        url: "https://example.com/r.git",
        secret: &secret,
        host_key: None,
    })
    .await
    .unwrap();
    let mut command = Command::new("git");
    session.apply(&mut command);
    let env = env_of(&command);
    let n: usize = value(&env, "GIT_CONFIG_COUNT").unwrap().parse().unwrap();
    let config: Vec<(String, String)> = (0..n)
        .map(|i| {
            (
                value(&env, &format!("GIT_CONFIG_KEY_{i}")).unwrap(),
                value(&env, &format!("GIT_CONFIG_VALUE_{i}")).unwrap(),
            )
        })
        .collect();
    assert!(config.contains(&("credential.helper".into(), String::new())));
    assert!(config.contains(&("core.autocrlf".into(), "false".into())));
    let header = config
        .iter()
        .find(|(k, _)| k == "http.extraHeader")
        .unwrap();
    // base64("bot:hunter2")
    assert_eq!(header.1, "Authorization: Basic Ym90Omh1bnRlcjI=");
    assert_eq!(value(&env, "GIT_TERMINAL_PROMPT").as_deref(), Some("0"));
    assert!(command
        .as_std()
        .get_args()
        .all(|a| !a.to_string_lossy().contains("hunter2")));
    // The user's agent is never offered.
    assert!(env.contains(&("SSH_AUTH_SOCK".into(), None)));
    // Nothing secret is on disk.
    for entry in std::fs::read_dir(session.dir.path()).unwrap() {
        let text = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        assert!(!text.contains("hunter2") && !text.contains("Ym90"));
    }
}

#[test]
fn ssh_trusts_the_first_key_only_when_none_is_stored() {
    let path = Path::new("/tmp/my dir/known_hosts");
    let first = ssh_command(path, true);
    assert!(first.contains("StrictHostKeyChecking=accept-new"));
    assert!(first.contains("'UserKnownHostsFile=/tmp/my dir/known_hosts'"));
    assert!(first.contains("IdentityFile=none") && first.contains("-F none"));
    assert!(ssh_command(path, false).contains("StrictHostKeyChecking=yes"));
    assert_eq!(shell_quote("it's"), r"'it'\''s'");
}

#[cfg(unix)]
#[tokio::test]
async fn deploy_keys_are_served_by_an_agent_and_a_stored_host_key_is_written() {
    let key = homeport_transport::ssh::generate_key("t");
    let secret = Secret::SshKey {
        private_key: key.private_key.clone(),
    };
    let line = "github.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl";
    let session = Session::remote(&Remote {
        url: "git@github.com:o/r.git",
        secret: &secret,
        host_key: Some(line),
    })
    .await
    .unwrap();
    let mut command = Command::new("git");
    session.apply(&mut command);
    let env = env_of(&command);
    let socket = value(&env, "SSH_AUTH_SOCK").unwrap();
    assert!(Path::new(&socket).exists());
    assert!(value(&env, "GIT_SSH_COMMAND")
        .unwrap()
        .contains("StrictHostKeyChecking=yes"));
    assert_eq!(
        std::fs::read_to_string(session.known_hosts())
            .unwrap()
            .trim(),
        line
    );
    assert_eq!(session.learned_host_key(), None);
    for entry in std::fs::read_dir(session.dir.path()).unwrap() {
        let text = std::fs::read_to_string(entry.unwrap().path()).unwrap_or_default();
        assert!(!text.contains("PRIVATE KEY"));
    }
}

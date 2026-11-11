use super::*;

#[test]
fn lines_split_into_keyword_and_arguments() {
    assert_eq!(
        split_line("  HostName=example.com "),
        Some(("hostname".into(), vec!["example.com".into()]))
    );
    assert_eq!(
        split_line(r#"IdentityFile "~/My Keys/id_ed25519""#),
        Some(("identityfile".into(), vec!["~/My Keys/id_ed25519".into()]))
    );
    assert_eq!(
        split_line("Host a b"),
        Some(("host".into(), vec!["a".into(), "b".into()]))
    );
    assert_eq!(split_line("# comment"), None);
    assert_eq!(split_line("   "), None);
}

#[test]
fn globs_match_like_host_patterns() {
    assert!(glob_matches("*", "anything"));
    assert!(glob_matches("*.example.com", "db.EXAMPLE.com"));
    assert!(glob_matches("web-?", "web-1"));
    assert!(!glob_matches("web-?", "web-10"));
    assert!(!glob_matches("prod", "production"));
}

#[test]
fn the_first_value_wins_and_defaults_come_last() {
    let config = SshConfig::parse(
        "User everyone\n\
             Host prod\n  HostName 10.0.0.5\n  Port 2222\n  IdentityFile ~/.ssh/prod\n\
             Host *.internal !secret.internal\n  User internal\n\
             Host *\n  User fallback\n  Port 22\n  IdentityFile ~/.ssh/id_ed25519\n",
    );
    assert_eq!(config.aliases(), ["prod"]);
    assert_eq!(
        config.resolve("prod"),
        Resolved {
            host_name: Some("10.0.0.5".into()),
            user: Some("everyone".into()),
            port: Some("2222".into()),
            identity_files: vec!["~/.ssh/prod".into(), "~/.ssh/id_ed25519".into()],
            ..Default::default()
        }
    );
    assert_eq!(
        config.resolve("db.internal").user.as_deref(),
        Some("everyone")
    );
}

#[test]
fn negated_patterns_exclude_a_block() {
    let config = SshConfig::parse(
        "Host *.internal !secret.internal\n  User internal\nHost *\n  User other\n",
    );
    assert_eq!(
        config.resolve("db.internal").user.as_deref(),
        Some("internal")
    );
    assert_eq!(
        config.resolve("secret.internal").user.as_deref(),
        Some("other")
    );
}

#[test]
fn match_blocks_are_skipped() {
    let config = SshConfig::parse(
        "Host a\n  User one\nMatch host a exec true\n  User two\nHost b\n  User three\n",
    );
    assert_eq!(config.resolve("a").user.as_deref(), Some("one"));
    assert_eq!(config.resolve("b").user.as_deref(), Some("three"));
}

#[test]
fn includes_are_followed_in_place_with_wildcards() {
    let dir = tempfile::tempdir().unwrap();
    let ssh = dir.path().join(".ssh");
    std::fs::create_dir_all(ssh.join("config.d")).unwrap();
    std::fs::write(ssh.join("config.d/10-work"), "Host work\n  User me\n").unwrap();
    std::fs::write(ssh.join("config.d/20-home"), "Host home\n  User me\n").unwrap();
    std::fs::write(ssh.join("abs"), "Host abs\n  HostName 1.2.3.4\n").unwrap();
    // Includes itself: must stop rather than loop forever.
    std::fs::write(
        ssh.join("config"),
        format!(
            "Include config.d/*\nInclude {}\nInclude ~/.ssh/config\nHost last\n",
            ssh.join("abs").display()
        ),
    )
    .unwrap();
    let config = SshConfig::read(&ssh.join("config"), &ssh, dir.path()).unwrap();
    let aliases = config.aliases();
    assert_eq!(&aliases[..4], ["work", "home", "abs", "last"]);
    assert_eq!(config.resolve("abs").host_name.as_deref(), Some("1.2.3.4"));
}

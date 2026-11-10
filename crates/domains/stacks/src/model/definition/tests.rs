use super::*;

fn check_managed(yaml: &str) -> ComposeCheck {
    check(&[yaml], SourceKind::Managed)
}

#[test]
fn stack_names_follow_compose_rules() {
    assert!(validate_name("shop-2_prod").is_ok());
    for bad in ["", "Shop", "-shop", "my shop", "café"] {
        assert!(validate_name(bad).is_err(), "{bad}");
    }
}

#[test]
fn compose_file_names_are_plain_and_unique() {
    let f = |n: &str| ComposeFile::new(n, "");
    assert!(validate_files(&[f("compose.yaml"), f("compose.prod.yml")]).is_ok());
    assert!(validate_files(&[]).is_err());
    for bad in [
        "",
        "../x.yaml",
        "dir/x.yaml",
        "x.txt",
        ".hidden.yaml",
        "C:x.yaml",
    ] {
        assert!(validate_files(&[f(bad)]).is_err(), "{bad}");
    }
    assert!(validate_files(&[f("a.yaml"), f("a.yaml")]).is_err());
}

#[test]
fn a_swarm_refuses_local_builds_without_an_image() {
    let built = "services:\n  app:\n    build: .\n";
    let pushed = "services:\n  app:\n    build: .\n    image: registry.example.com/app:1\n";
    for kind in [SourceKind::Managed, SourceKind::Linked, SourceKind::Synced] {
        let refused = check_for(&[built], kind, true);
        assert!(!refused.is_deployable(), "{kind:?}");
        assert!(
            refused.errors[0].contains("pull their images"),
            "{refused:?}"
        );
    }
    // With an image, a Linked Stack builds here and the nodes pull it.
    let linked = check_for(&[pushed], SourceKind::Linked, true);
    assert!(linked.is_deployable(), "{linked:?}");
    assert!(linked.warnings.iter().any(|w| w.contains("push the image")));
    // On a Host, a Linked Stack builds without an image, as before.
    assert!(check_for(&[built], SourceKind::Linked, false).is_deployable());
}

#[test]
fn a_swarm_warns_about_what_stack_deploy_ignores() {
    let yaml = "services:\n  web:\n    image: nginx\n    container_name: web\n    restart: always\n    depends_on:\n      db:\n        condition: service_healthy\n    volumes:\n      - ./site:/usr/share/nginx/html\n      - /srv/data:/data\n  db:\n    image: postgres\n";
    let checked = check_for(&[yaml], SourceKind::Linked, true);
    assert!(checked.is_deployable());
    let has = |needle: &str| checked.warnings.iter().any(|w| w.contains(needle));
    assert!(has("container_name:"));
    assert!(has("restart:"));
    assert!(has("depends_on:"));
    assert!(has("\"./site\", a path on whichever node"));
    assert!(has("\"/srv/data\", a path on whichever node"));
    // None of that is said for a Host.
    assert!(check_for(&[yaml], SourceKind::Linked, false)
        .warnings
        .is_empty());
}

#[test]
fn file_configs_need_a_folder_on_a_swarm() {
    let yaml = "services:\n  web:\n    image: nginx\n    configs: [conf]\nconfigs:\n  conf:\n    file: ./nginx.conf\n";
    assert!(!check_for(&[yaml], SourceKind::Managed, true).is_deployable());
    assert!(check_for(&[yaml], SourceKind::Managed, false).is_deployable());
    assert!(check_for(&[yaml], SourceKind::Linked, true).is_deployable());
    assert!(check_for(&[yaml], SourceKind::Synced, true).is_deployable());
}

#[test]
fn profiles_are_the_union_of_every_service_s() {
    let base = "services:\n  web:\n    image: nginx\n  debug:\n    image: busybox\n    profiles: [debug, tools]\n";
    let extra = "services:\n  worker:\n    image: app\n    profiles: [workers, debug]\n";
    assert_eq!(
        declared_profiles(&[base, extra]),
        ["debug", "tools", "workers"]
    );
    assert_eq!(
        check(&[base], SourceKind::Managed).profiles,
        ["debug", "tools"]
    );
    assert!(declared_profiles(&["services:\n  web:\n    image: x\n"]).is_empty());
    assert!(validate_profiles(&["debug".into(), "a.b-c_1".into()]).is_ok());
    assert!(validate_profiles(&["-x".into()]).is_err());
    assert_eq!(
        services_turned_off(&[base, extra], &[]),
        ["debug", "worker"]
    );
    assert_eq!(
        services_turned_off(&[base, extra], &["tools".into()]),
        ["worker"]
    );
    assert!(validate_profiles(&["a b".into()]).is_err());
}

#[test]
fn plain_image_stacks_pass() {
    let c = check_managed(
            "services:\n  web:\n    image: nginx\n    volumes:\n      - data:/data\n      - /srv/site:/usr/share/nginx/html:ro\nvolumes:\n  data: {}\n",
        );
    assert_eq!(c, ComposeCheck::default());
}

#[test]
fn local_builds_are_refused() {
    let c = check_managed("services:\n  app:\n    build: .\n");
    assert!(!c.is_deployable());
    assert!(c.errors[0].contains("build:"));
}

#[test]
fn paths_read_on_this_machine_are_flagged() {
    let c = check_managed(
            "services:\n  app:\n    image: app\n    env_file: .env\n    volumes:\n      - ./data:/data\n      - type: bind\n        source: ~/conf\n        target: /conf\nsecrets:\n  key:\n    file: ./key.pem\n",
        );
    assert!(c.is_deployable());
    assert_eq!(c.warnings.len(), 4, "{:?}", c.warnings);
}

#[test]
fn malformed_definitions_are_explained() {
    assert!(!check_managed("services: [").is_deployable());
    assert!(!check_managed("version: '3'\n").is_deployable());
    assert!(!check_managed("services:\n  app:\n    ports: []\n").is_deployable());
}

#[test]
fn linked_stacks_may_build_and_use_paths_in_their_folder() {
    let yaml = "services:\n  app:\n    build: .\n    env_file: .env\n    volumes:\n      - ./data:/data\nsecrets:\n  key:\n    file: ./key.pem\n";
    assert_eq!(check(&[yaml], SourceKind::Linked), ComposeCheck::default());
    // The structure is still checked.
    assert!(!check(&["services:\n  app:\n    ports: []\n"], SourceKind::Linked).is_deployable());
    assert!(!check(&["version: '3'\n"], SourceKind::Linked).is_deployable());
}

#[test]
fn several_files_are_checked_merged() {
    let base = "services:\n  web:\n    image: nginx\n";
    let prod = "services:\n  web:\n    ports: [\"80:80\"]\n";
    assert!(check(&[prod], SourceKind::Managed).errors[0].contains("needs an image"));
    assert_eq!(
        check(&[base, prod], SourceKind::Managed),
        ComposeCheck::default()
    );
    let extra = "services:\n  worker:\n    command: [\"run\"]\n";
    let c = check(&[base, extra], SourceKind::Managed);
    assert_eq!(c.errors.len(), 1, "{c:?}");
    assert!(c.errors[0].contains("worker"));
    // An override file alone needs no services: section of its own.
    assert_eq!(
        check(&[base, "volumes:\n  data: {}\n"], SourceKind::Managed),
        ComposeCheck::default()
    );
}

#[test]
fn relative_binds_are_listed_by_service() {
    let base = "services:\n  app:\n    image: x\n    volumes:\n      - ./data:/data\n      - /srv/abs:/abs\n      - named:/named\n";
    assert_eq!(
        relative_binds(&[base]),
        [("app".to_owned(), "./data".to_owned())]
    );
}

#[test]
fn env_files_parse_like_compose() {
    let vars = parse_env(
        "# comment\nexport A=1\nB = two words # note\nC=\"quoted # kept\"\nD='single'\nE=\n",
    )
    .unwrap();
    assert_eq!(
        vars,
        [
            ("A".into(), "1".into()),
            ("B".into(), "two words".into()),
            ("C".into(), "quoted # kept".into()),
            ("D".into(), "single".into()),
            ("E".into(), String::new()),
        ]
    );
    assert!(parse_env("not a pair").is_err());
    assert!(parse_env("BAD-NAME=1").is_err());
}

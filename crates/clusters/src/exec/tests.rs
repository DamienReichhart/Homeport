use super::*;
use crate::classify::needs_user;

const TOKEN: &str = r#"{"apiVersion":"client.authentication.k8s.io/v1","kind":"ExecCredential","status":{"token":"k8s-aws-v1.abc","expirationTimestamp":"2099-01-01T00:00:00Z"}}"#;

#[test]
fn a_token_and_its_expiry_are_read() {
    let issued = parse(TOKEN.as_bytes()).unwrap();
    assert_eq!(issued.credentials.token.as_deref(), Some("k8s-aws-v1.abc"));
    let expires = issued.expires.unwrap();
    assert!(expires > SystemTime::now());
    assert!(!issued.is_due(SystemTime::now()));
    assert!(
        issued.is_due(expires - Duration::from_secs(60)),
        "renewed early"
    );
}

#[test]
fn a_client_certificate_is_read_and_never_expires_without_a_date() {
    let issued = parse(
        br#"{"kind":"ExecCredential","status":{"clientCertificateData":"CERT","clientKeyData":"KEY"}}"#,
    )
    .unwrap();
    assert_eq!(issued.credentials.client_key.as_deref(), Some("KEY"));
    assert_eq!(issued.expires, None);
    assert!(!issued.is_due(SystemTime::now()));
}

#[test]
fn output_without_credentials_is_refused() {
    assert!(parse(b"not json").is_err());
    assert!(parse(br#"{"kind":"ExecCredential"}"#).is_err());
    assert!(parse(br#"{"kind":"ExecCredential","status":{"token":""}}"#).is_err());
    assert!(parse(
        br#"{"kind":"ExecCredential","status":{"token":"t","expirationTimestamp":"tomorrow"}}"#
    )
    .is_err());
}

#[test]
fn cluster_info_is_given_only_when_asked() {
    let mut exec = ExecConfig {
        api_version: "client.authentication.k8s.io/v1".into(),
        ..Default::default()
    };
    let cluster = ClusterInfo {
        server: "https://k8s.example.com".into(),
        ..Default::default()
    };
    let info: serde_json::Value = serde_json::from_str(&exec_info(&exec, &cluster)).unwrap();
    assert_eq!(info["spec"]["interactive"], false);
    assert!(info["spec"].get("cluster").is_none());
    exec.provide_cluster_info = true;
    let info: serde_json::Value = serde_json::from_str(&exec_info(&exec, &cluster)).unwrap();
    assert_eq!(info["spec"]["cluster"]["server"], "https://k8s.example.com");
}

#[test]
fn sign_in_prompts_are_told_apart_from_failures() {
    assert!(needs_terminal(
        "To sign in, use a web browser to open the page https://microsoft.com/devicelogin"
    ));
    assert!(needs_terminal(
        "Error loading SSO Token: Token for prod does not exist. Run aws sso login"
    ));
    assert!(!needs_terminal(
        "error: cluster prod not found in region eu-west-1"
    ));
}

#[tokio::test]
async fn a_missing_program_names_it_and_how_to_install_it() {
    let exec = ExecConfig {
        command: "homeport-no-such-plugin".into(),
        install_hint: Some("brew install no-such-plugin".into()),
        ..Default::default()
    };
    let e = run("prod", Some("prod"), &exec, &ClusterInfo::default())
        .await
        .unwrap_err();
    assert!(e.is(ErrorCode::ExecPluginMissing));
    assert!(needs_user(&e));
    assert!(e.message.contains("homeport-no-such-plugin"));
    assert!(e.detail.unwrap().contains("brew install"));

    let aws = ExecConfig {
        command: "/nonexistent/aws".into(),
        ..Default::default()
    };
    let e = run("prod", None, &aws, &ClusterInfo::default())
        .await
        .unwrap_err();
    assert_eq!(e.hint.unwrap().key, "errors.clusters.exec.missing.hintAws");
}

#[tokio::test]
async fn a_plugin_that_always_wants_a_terminal_isnt_run() {
    let exec = ExecConfig {
        command: "kubelogin".into(),
        interactive_mode: Some("Always".into()),
        ..Default::default()
    };
    let e = run("aks", Some("aks-prod"), &exec, &ClusterInfo::default())
        .await
        .unwrap_err();
    assert!(e.is(ErrorCode::ExecPluginInteractive));
    assert_eq!(
        e.hint.unwrap().commands,
        ["kubectl --context aks-prod get namespaces"]
    );
}

#[cfg(unix)]
mod scripts {
    use std::os::unix::fs::PermissionsExt;

    use super::*;

    /// A plugin: a shell script with `body`, in a folder of its own.
    fn plugin(dir: &Path, body: &str) -> String {
        let path = dir.join("plugin");
        std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path.display().to_string()
    }

    fn config(command: String) -> ExecConfig {
        ExecConfig {
            api_version: "client.authentication.k8s.io/v1beta1".into(),
            command,
            args: vec!["--region".into(), "eu-west-1".into()],
            env: vec![("PLUGIN_PROFILE".into(), "prod".into())],
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn a_plugin_gets_its_args_env_and_exec_info() {
        let dir = tempfile::tempdir().unwrap();
        let script = plugin(
            dir.path(),
            r#"case "$KUBERNETES_EXEC_INFO" in *v1beta1*) ;; *) exit 3 ;; esac
[ "$1 $2" = "--region eu-west-1" ] || exit 4
[ "$PLUGIN_PROFILE" = prod ] || exit 5
read -r line && exit 6
echo '{"kind":"ExecCredential","status":{"token":"from-plugin"}}'"#,
        );
        let issued = run("prod", None, &config(script), &ClusterInfo::default())
            .await
            .unwrap();
        assert_eq!(issued.credentials.token.as_deref(), Some("from-plugin"));
    }

    #[tokio::test]
    async fn a_failing_plugin_keeps_its_stderr_in_details_and_is_retried() {
        let dir = tempfile::tempdir().unwrap();
        let script = plugin(dir.path(), "echo 'cluster prod not found' >&2; exit 1");
        let e = run("prod", None, &config(script), &ClusterInfo::default())
            .await
            .unwrap_err();
        assert!(e.is(ErrorCode::ExecPluginFailed));
        assert!(!needs_user(&e));
        assert_eq!(e.detail.as_deref(), Some("cluster prod not found"));
    }

    #[tokio::test]
    async fn a_plugin_asking_to_sign_in_waits_for_the_user() {
        let dir = tempfile::tempdir().unwrap();
        let script = plugin(
            dir.path(),
            "echo 'To sign in, use a web browser to open https://microsoft.com/devicelogin' >&2; exit 1",
        );
        let e = run("aks", None, &config(script), &ClusterInfo::default())
            .await
            .unwrap_err();
        assert!(e.is(ErrorCode::ExecPluginInteractive));
        assert!(needs_user(&e));
    }
}

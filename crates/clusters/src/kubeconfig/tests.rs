use base64::Engine as _;

use super::*;
use crate::model::ClusterConnection;

const CA: &str = "-----BEGIN CERTIFICATE-----\nMIIB\n-----END CERTIFICATE-----\n";

fn b64(s: &str) -> String {
    base64::engine::general_purpose::STANDARD.encode(s)
}

fn typical() -> String {
    format!(
        r#"apiVersion: v1
kind: Config
current-context: kind-dev
clusters:
- name: kind-dev
  cluster:
    server: https://127.0.0.1:6443
    certificate-authority-data: {ca}
- name: prod
  cluster:
    server: https://k8s.example.com
    tls-server-name: api.example.com
- name: eks
  cluster:
    server: https://ABC.gr7.eu-west-1.eks.amazonaws.com
contexts:
- name: kind-dev
  context: {{ cluster: kind-dev, user: kind-dev }}
- name: prod
  context: {{ cluster: prod, user: deployer, namespace: shop }}
- name: sso
  context: {{ cluster: prod, user: oidc-user }}
- name: eks
  context: {{ cluster: eks, user: eks-user }}
- name: old-gke
  context: {{ cluster: prod, user: gcp-user }}
- name: broken
  context: {{ cluster: missing, user: deployer }}
users:
- name: kind-dev
  user:
    client-certificate-data: {cert}
    client-key-data: {key}
- name: deployer
  user:
    token: secret-token
- name: oidc-user
  user:
    auth-provider:
      name: oidc
      config:
        id-token: eyJ.id.token
        refresh-token: refresh-me
        idp-issuer-url: https://login.example.com
        client-id: kubectl
- name: eks-user
  user:
    exec:
      apiVersion: client.authentication.k8s.io/v1beta1
      command: aws
      args: [eks, get-token, --cluster-name, prod]
      env:
      - name: AWS_PROFILE
        value: prod
      installHint: Install the AWS CLI.
- name: gcp-user
  user:
    auth-provider:
      name: gcp
"#,
        ca = b64(CA),
        cert = b64("-----BEGIN CERTIFICATE-----\nCLIENT\n-----END CERTIFICATE-----\n"),
        key = b64("-----BEGIN EC PRIVATE KEY-----\nKEY\n-----END EC PRIVATE KEY-----\n"),
    )
}

fn source() -> Source {
    Source::pasted(&typical()).unwrap()
}

#[test]
fn a_certificate_user_keeps_only_the_server_in_the_clear() {
    let (connection, secrets) = resolve(&source(), "kind-dev").unwrap();
    assert_eq!(connection.server, "https://127.0.0.1:6443");
    assert_eq!(connection.auth, AuthMethod::ClientCertificate);
    assert_eq!(secrets.certificate_authority.as_deref(), Some(CA));
    assert!(secrets.client_certificate.unwrap().contains("CLIENT"));
    assert!(secrets.client_key.unwrap().contains("KEY"));
    let json = serde_json::to_string(&connection).unwrap();
    assert!(
        !json.contains("CERTIFICATE") && !json.contains("KEY"),
        "{json}"
    );
    assert_eq!(
        connection.source,
        Some(crate::model::KubeconfigSource {
            context: "kind-dev".into(),
            pasted: true,
        })
    );
}

#[test]
fn a_token_user_keeps_its_namespace_and_server_name() {
    let (connection, secrets) = resolve(&source(), "prod").unwrap();
    assert_eq!(connection.auth, AuthMethod::Token);
    assert_eq!(connection.namespace.as_deref(), Some("shop"));
    assert_eq!(
        connection.tls_server_name.as_deref(),
        Some("api.example.com")
    );
    assert_eq!(secrets.token.as_deref(), Some("secret-token"));
    assert!(!serde_json::to_string(&connection)
        .unwrap()
        .contains("secret-token"));
}

#[test]
fn an_oidc_user_seals_its_provider() {
    let (connection, secrets) = resolve(&source(), "sso").unwrap();
    assert_eq!(connection.auth, AuthMethod::Oidc);
    let oidc = secrets.oidc.unwrap();
    assert_eq!(oidc["refresh-token"], "refresh-me");
}

#[test]
fn an_exec_user_seals_its_plugin_and_environment() {
    let (connection, secrets) = resolve(&source(), "eks").unwrap();
    assert_eq!(
        connection.auth,
        AuthMethod::Exec {
            command: "aws".into()
        }
    );
    let exec = secrets.exec.unwrap();
    assert_eq!(exec.api_version, "client.authentication.k8s.io/v1beta1");
    assert_eq!(exec.args, ["eks", "get-token", "--cluster-name", "prod"]);
    assert_eq!(exec.env, [("AWS_PROFILE".to_owned(), "prod".to_owned())]);
    assert_eq!(exec.install_hint.as_deref(), Some("Install the AWS CLI."));
    assert!(!serde_json::to_string(&connection)
        .unwrap()
        .contains("AWS_PROFILE"));
}

#[test]
fn removed_auth_providers_say_how_to_switch() {
    let e = resolve(&source(), "old-gke").unwrap_err();
    assert!(e.is(ErrorCode::KubeconfigAuthProviderUnsupported));
    let hint = e.hint.unwrap();
    assert!(hint
        .commands
        .iter()
        .any(|c| c.contains("gke-gcloud-auth-plugin")));
}

#[test]
fn contexts_list_every_one_with_its_problem() {
    let clusters = [Cluster {
        id: "c1".into(),
        name: "Prod shop".into(),
        kind: crate::ClusterKind::Kubernetes,
        connection: ClusterConnection {
            source: Some(crate::model::KubeconfigSource {
                context: "prod".into(),
                pasted: false,
            }),
            ..Default::default()
        },
        enabled: true,
        created_at: String::new(),
    }];
    let listed = contexts(&source(), &clusters);
    assert!(listed.paths.is_empty(), "pasted");
    let by_name = |n: &str| listed.contexts.iter().find(|c| c.name == n).unwrap();
    assert!(by_name("kind-dev").current);
    assert_eq!(by_name("kind-dev").added, None);
    assert_eq!(
        by_name("prod").added.as_ref().map(|a| a.name.as_str()),
        Some("Prod shop")
    );
    assert_eq!(by_name("prod").user.as_deref(), Some("deployer"));
    assert!(by_name("broken").problem.is_some());
    assert!(by_name("old-gke").problem.is_some());
    assert!(by_name("eks").problem.is_none());
}

#[test]
fn files_are_read_and_inlined() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("ca.crt"), CA).unwrap();
    std::fs::write(dir.path().join("token"), "file-token\n").unwrap();
    std::fs::write(
        dir.path().join("config"),
        r#"clusters:
- name: local
  cluster: { server: "https://127.0.0.1:6443", certificate-authority: ca.crt }
contexts:
- name: local
  context: { cluster: local, user: me }
users:
- name: me
  user: { tokenFile: token }
"#,
    )
    .unwrap();
    let source = Source {
        config: Kubeconfig::read_from(dir.path().join("config")).unwrap(),
        pasted: false,
        paths: vec![dir.path().join("config")],
    };
    let (_, secrets) = resolve(&source, "local").unwrap();
    assert_eq!(secrets.certificate_authority.as_deref(), Some(CA));
    assert_eq!(secrets.token.as_deref(), Some("file-token"));
}

#[test]
fn a_pasted_kubeconfig_cant_refer_to_relative_files() {
    let text = r#"clusters:
- name: local
  cluster: { server: "https://127.0.0.1:6443", certificate-authority: ca.crt }
contexts:
- name: local
  context: { cluster: local }
"#;
    let e = resolve(&Source::pasted(text).unwrap(), "local").unwrap_err();
    assert!(e.message.contains("ca.crt"));
    assert!(e.hint.unwrap().commands[0].contains("--flatten"));
}

#[test]
fn kubeconfig_lists_paths_with_the_os_separator() {
    let joined = std::env::join_paths(["/a/config", "/b/config"]).unwrap();
    assert_eq!(
        local_paths(Some(joined), None).unwrap(),
        [PathBuf::from("/a/config"), PathBuf::from("/b/config")]
    );
    assert_eq!(
        local_paths(None, Some(Path::new("/home/me"))).unwrap(),
        [PathBuf::from("/home/me/.kube/config")]
    );
    assert_eq!(
        local_paths(Some("".into()), Some(Path::new("/home/me"))).unwrap(),
        [PathBuf::from("/home/me/.kube/config")]
    );
}

#[test]
fn merged_files_keep_the_first_definition() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a");
    let b = dir.path().join("b");
    std::fs::write(&a, "current-context: one\nclusters:\n- name: x\n  cluster: { server: 'https://first' }\ncontexts:\n- name: one\n  context: { cluster: x }\n").unwrap();
    std::fs::write(&b, "current-context: two\nclusters:\n- name: x\n  cluster: { server: 'https://second' }\n- name: y\n  cluster: { server: 'https://y' }\ncontexts:\n- name: two\n  context: { cluster: y }\n").unwrap();
    let mut config = Kubeconfig::default();
    for p in [&a, &b] {
        config = config.merge(Kubeconfig::read_from(p).unwrap()).unwrap();
    }
    let source = Source {
        config,
        pasted: false,
        paths: vec![a, b],
    };
    let listed = contexts(&source, &[]);
    assert_eq!(listed.contexts.len(), 2);
    assert!(listed.contexts[0].current);
    assert_eq!(listed.contexts[0].server.as_deref(), Some("https://first"));
}

#[test]
fn text_that_isnt_a_kubeconfig_is_refused() {
    assert!(Source::pasted("{{{").is_err());
    assert!(Source::pasted("kind: Config\n").is_err());
}

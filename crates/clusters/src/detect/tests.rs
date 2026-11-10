use std::collections::HashMap;

use super::*;

#[test]
fn local_tools_are_recognised_by_context_and_server() {
    for (name, server, tool) in [
        (
            "docker-desktop",
            "https://kubernetes.docker.internal:6443",
            LocalTool::DockerDesktop,
        ),
        (
            "docker-desktop",
            "https://127.0.0.1:6443",
            LocalTool::DockerDesktop,
        ),
        ("orbstack", "https://127.0.0.1:26443", LocalTool::Orbstack),
        (
            "rancher-desktop",
            "https://127.0.0.1:6443",
            LocalTool::RancherDesktop,
        ),
        (
            "rancher-desktop",
            "https://192.168.5.15:6443",
            LocalTool::RancherDesktop,
        ),
        ("kind-dev", "https://127.0.0.1:41235", LocalTool::Kind),
        ("k3d-shop", "https://0.0.0.0:6550", LocalTool::K3d),
        ("minikube", "https://192.168.49.2:8443", LocalTool::Minikube),
        ("kind-ipv6", "https://[::1]:6443", LocalTool::Kind),
    ] {
        assert_eq!(
            recognise(name, server, false),
            Some(tool),
            "{name} {server}"
        );
    }
    assert_eq!(
        recognise("staging", "https://192.168.59.100:8443", true),
        Some(LocalTool::Minikube),
        "a minikube profile, by its extension"
    );
}

#[test]
fn remote_contexts_are_never_local() {
    for (name, server) in [
        ("kind-dev", "https://k8s.example.com:6443"),
        ("k3d-prod", "https://10.0.0.5:6443"),
        ("docker-desktop", "https://203.0.113.7:6443"),
        ("minikube", "https://k8s.example.com"),
        ("prod", "https://127.0.0.1:6443"),
        ("kind-", "https://127.0.0.1:6443"),
        ("eks", "https://ABC.gr7.eu-west-1.eks.amazonaws.com"),
    ] {
        assert_eq!(recognise(name, server, false), None, "{name} {server}");
    }
}

#[test]
fn minikube_marks_its_entries() {
    let config = Kubeconfig::from_yaml(
        r#"clusters:
- name: work
  cluster:
    server: https://192.168.58.2:8443
    extensions:
    - name: cluster_info
      extension: { provider: minikube.sigs.k8s.io, version: v1.34.0 }
- name: prod
  cluster: { server: "https://192.168.1.10:6443" }
contexts:
- name: work
  context: { cluster: work }
- name: prod
  context: { cluster: prod }
"#,
    )
    .unwrap();
    assert_eq!(
        candidates(&config),
        [(
            LocalTool::Minikube,
            "work".to_owned(),
            "https://192.168.58.2:8443".to_owned()
        )]
    );
}

#[test]
fn running_kind_and_k3d_clusters_missing_from_the_kubeconfig_say_how_to_add_them() {
    let label = |k: &str, v: &str| HashMap::from([(k.to_owned(), v.to_owned())]);
    let labels = [
        label("io.x-k8s.kind.cluster", "dev"),
        label("io.x-k8s.kind.cluster", "dev"),
        label("io.x-k8s.kind.cluster", "known"),
        label("k3d.cluster", "shop"),
        label("com.docker.compose.project", "web"),
    ];
    let missing = missing(&labels, &["kind-known".to_owned()]);
    assert_eq!(
        missing,
        [
            MissingLocalCluster {
                tool: LocalTool::Kind,
                name: "dev".into(),
                command: "kind export kubeconfig --name dev".into(),
            },
            MissingLocalCluster {
                tool: LocalTool::K3d,
                name: "shop".into(),
                command: "k3d kubeconfig merge shop --kubeconfig-merge-default".into(),
            },
        ]
    );
}

//! Against a real kind cluster: `kind create cluster --name homeport`, then
//! `cargo test -p homeport-clusters --test kind -- --ignored`. The context
//! defaults to `kind-homeport`; set `HOMEPORT_TEST_CONTEXT` for another.
//! Without that context in the kubeconfig (as in CI, which runs the
//! ignored Engine tests), it says so and passes.

use homeport_clusters::{
    client,
    kubeconfig::{self, Source},
    resources::{On, WorkloadKind},
    Cluster, ClusterKind,
};

fn context() -> String {
    std::env::var("HOMEPORT_TEST_CONTEXT").unwrap_or_else(|_| "kind-homeport".into())
}

#[tokio::test]
#[ignore = "needs a kind cluster"]
async fn a_kind_cluster_is_browsed() {
    let Some((connection, secrets)) = Source::local()
        .ok()
        .and_then(|source| kubeconfig::resolve(&source, &context()).ok())
    else {
        eprintln!("skipped: no kubeconfig context {}", context());
        return;
    };
    let cluster = Cluster {
        id: "test".into(),
        name: context(),
        kind: ClusterKind::Kubernetes,
        connection,
        enabled: true,
        created_at: String::new(),
    };
    let on = On {
        client: client::client(&cluster, &secrets, None).unwrap(),
        cluster,
    };
    let namespaces = on.namespaces().await.unwrap();
    assert!(namespaces.iter().any(|n| n.name == "kube-system"));
    let system = on.pods(Some("kube-system"), None, None).await.unwrap();
    assert!(system.iter().any(|p| p.name.starts_with("kube-apiserver")));
    let coredns = on
        .workloads(WorkloadKind::Deployment, Some("kube-system"))
        .await
        .unwrap();
    let coredns = coredns.iter().find(|d| d.name == "coredns").unwrap();
    assert!(coredns.desired >= 1);
    let detail = on
        .workload(WorkloadKind::Deployment, "kube-system", "coredns")
        .await
        .unwrap();
    let pods = on
        .pods(Some("kube-system"), Some(&detail.selector), None)
        .await
        .unwrap();
    assert_eq!(pods.len() as u32, coredns.desired);
    let yaml = on
        .yaml(
            homeport_clusters::resources::ResourceKind::Deployment,
            Some("kube-system"),
            "coredns",
        )
        .await
        .unwrap();
    assert!(!yaml.contains("managedFields"));
}

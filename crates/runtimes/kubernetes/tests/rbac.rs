//! RBAC against a real Kubernetes cluster whose context may do anything
//! (kind's, k3s's): `cargo test -p homeport-clusters --test rbac --
//! --ignored kubernetes_live`. The context defaults to `kind-homeport`;
//! set `HOMEPORT_TEST_CONTEXT` (and `KUBECONFIG`) for another. Without it
//! in the kubeconfig, as in CI, each test says so and passes.
//!
//! Each test makes its own Namespaces and ServiceAccount and deletes them.

use homeport_kubernetes::{
    api::client,
    kubeconfig::{self, Source},
    rbac::{AccessCheck, Subject, SubjectKind},
    resources::On,
    KubeCluster, KubernetesConnection, KubernetesSecrets,
};
use k8s_openapi::api::{
    authentication::v1::TokenRequest,
    core::v1::{Namespace, ServiceAccount},
    rbac::v1::RoleBinding,
};
use kube::api::{Api, DeleteParams, PostParams};

fn context() -> String {
    std::env::var("HOMEPORT_TEST_CONTEXT").unwrap_or_else(|_| "kind-homeport".into())
}

fn cluster(connection: KubernetesConnection) -> KubeCluster {
    KubeCluster {
        id: "test".into(),
        name: context(),
        connection,
    }
}

/// The test cluster with the context's credentials, and those credentials;
/// `None` (and why) when the kubeconfig doesn't reach it.
fn connect() -> Option<(On, KubernetesSecrets)> {
    let Some((connection, secrets)) = Source::local()
        .ok()
        .and_then(|source| kubeconfig::resolve(&source, &context()).ok())
    else {
        eprintln!("skipped: no kubeconfig context {}", context());
        return None;
    };
    let cluster = cluster(connection);
    let client = client::client(&cluster, &secrets, None).unwrap();
    Some((On { client, cluster }, secrets))
}

/// Two Namespaces, a ServiceAccount in the first bound to `edit` there,
/// deleted again when dropped.
struct Fixture {
    admin: On,
    secrets: KubernetesSecrets,
    prod: String,
    other: String,
}

const SA: &str = "deployer";

impl Fixture {
    async fn new(admin: On, secrets: KubernetesSecrets, tag: &str) -> Self {
        let id = std::process::id();
        let fixture = Self {
            prod: format!("homeport-rbac-{tag}-prod-{id}"),
            other: format!("homeport-rbac-{tag}-other-{id}"),
            admin,
            secrets,
        };
        let c = fixture.admin.client.clone();
        let namespaces: Api<Namespace> = Api::all(c.clone());
        for ns in [&fixture.prod, &fixture.other] {
            let n: Namespace =
                serde_json::from_value(serde_json::json!({ "metadata": { "name": ns } })).unwrap();
            namespaces.create(&PostParams::default(), &n).await.unwrap();
        }
        let sa: ServiceAccount =
            serde_json::from_value(serde_json::json!({ "metadata": { "name": SA } })).unwrap();
        Api::<ServiceAccount>::namespaced(c.clone(), &fixture.prod)
            .create(&PostParams::default(), &sa)
            .await
            .unwrap();
        let binding: RoleBinding = serde_json::from_value(serde_json::json!({
            "metadata": { "name": "deployer-edit" },
            "roleRef": { "apiGroup": "rbac.authorization.k8s.io", "kind": "ClusterRole", "name": "edit" },
            // No Namespace: a RoleBinding's own, as Kubernetes reads it.
            "subjects": [{ "kind": "ServiceAccount", "name": SA }],
        }))
        .unwrap();
        Api::<RoleBinding>::namespaced(c, &fixture.prod)
            .create(&PostParams::default(), &binding)
            .await
            .unwrap();
        fixture
    }

    fn subject(&self) -> Subject {
        Subject {
            kind: SubjectKind::ServiceAccount,
            name: SA.into(),
            namespace: Some(self.prod.clone()),
        }
    }

    /// The cluster as the ServiceAccount, through a short-lived token.
    async fn as_service_account(&self) -> On {
        let request: TokenRequest = serde_json::from_value(serde_json::json!({
            "spec": { "audiences": [], "expirationSeconds": 600 }
        }))
        .unwrap();
        let token = Api::<ServiceAccount>::namespaced(self.admin.client.clone(), &self.prod)
            .create_token_request(SA, &PostParams::default(), &request)
            .await
            .unwrap()
            .status
            .unwrap()
            .token;
        let secrets = KubernetesSecrets {
            certificate_authority: self.secrets.certificate_authority.clone(),
            token: Some(token),
            ..Default::default()
        };
        let cluster = self.admin.cluster.clone();
        let client = client::client(&cluster, &secrets, None).unwrap();
        On { client, cluster }
    }

    async fn clean_up(self) {
        let namespaces: Api<Namespace> = Api::all(self.admin.client.clone());
        for ns in [&self.prod, &self.other] {
            let _ = namespaces.delete(ns, &DeleteParams::default()).await;
        }
    }
}

#[tokio::test]
#[ignore = "needs a Kubernetes cluster"]
async fn kubernetes_live_rbac_a_service_account_bound_to_edit_may_edit_its_namespace_only() {
    let Some((admin, secrets)) = connect() else {
        return;
    };
    let fixture = Fixture::new(admin, secrets, "edit").await;

    let index = fixture.admin.rbac_index(None).await.unwrap();
    assert!(index.refused.is_empty());
    let edit = index
        .roles
        .iter()
        .find(|r| r.name == "edit" && r.namespace.is_none())
        .unwrap();
    // `edit` is aggregated: its rules come from the ClusterRoles it selects.
    assert!(edit
        .aggregated_from
        .contains(&"system:aggregate-to-edit".to_owned()));
    let subject = fixture.subject();
    assert!(index.subjects.iter().any(|s| s.subject == subject));

    let permissions = fixture.admin.rbac_subject(&subject, None).await.unwrap();
    let scope = |ns: Option<&str>| {
        permissions
            .scopes
            .iter()
            .find(|s| s.namespace.as_deref() == ns)
    };
    let prod = scope(Some(&fixture.prod)).expect("grants in its Namespace");
    let secrets = prod
        .grants
        .resources
        .iter()
        .find(|g| g.api_group.is_empty() && g.resource == "secrets")
        .unwrap();
    for verb in [
        "get",
        "list",
        "watch",
        "create",
        "update",
        "patch",
        "delete",
        "deletecollection",
    ] {
        assert!(secrets.verbs.iter().any(|v| v == verb), "{verb} on secrets");
    }
    // Through the RoleBinding only: nothing in the other Namespace, and
    // cluster-wide no more than what every signed-in ServiceAccount gets.
    assert!(scope(Some(&fixture.other)).is_none());
    if let Some(cluster_wide) = scope(None) {
        assert!(!cluster_wide
            .grants
            .resources
            .iter()
            .any(|g| g.resource == "secrets"));
    }
    let direct: Vec<_> = permissions
        .bindings
        .iter()
        .filter(|b| b.through_group.is_none())
        .map(|b| b.binding.name.as_str())
        .collect();
    assert_eq!(direct, ["deployer-edit"]);

    // The same Namespace alone gives the same answer for it.
    let only = fixture
        .admin
        .rbac_subject(&subject, Some(&fixture.prod))
        .await
        .unwrap();
    assert!(only
        .scopes
        .iter()
        .any(|s| s.namespace.as_deref() == Some(&fixture.prod)));
    fixture.clean_up().await;
}

#[tokio::test]
#[ignore = "needs a Kubernetes cluster"]
async fn kubernetes_live_rbac_can_i_answers_as_the_api_server_does() {
    let Some((admin, secrets)) = connect() else {
        return;
    };
    let fixture = Fixture::new(admin, secrets, "can-i").await;
    let deployer = fixture.as_service_account().await;
    let check = |verb: &str, resource: &str, ns: Option<&str>| AccessCheck {
        verb: verb.into(),
        resource: resource.into(),
        namespace: ns.map(str::to_owned),
        ..Default::default()
    };
    let prod = Some(fixture.prod.as_str());
    let other = Some(fixture.other.as_str());

    let answer = deployer
        .can_i(&check("delete", "secrets", prod))
        .await
        .unwrap();
    assert!(answer.allowed && answer.known);
    assert_eq!(answer.resource, "secrets");
    assert_eq!(
        answer.command,
        format!(
            "kubectl auth can-i delete secrets --namespace {}",
            fixture.prod
        )
    );
    assert!(
        !deployer
            .can_i(&check("delete", "secrets", other))
            .await
            .unwrap()
            .allowed
    );
    assert!(
        !deployer
            .can_i(&check("delete", "secrets", None))
            .await
            .unwrap()
            .allowed
    );
    // `edit` can't touch RBAC, and a short name resolves as kubectl's does.
    assert!(
        !deployer
            .can_i(&check("create", "rolebindings", prod))
            .await
            .unwrap()
            .allowed
    );
    let deploy = deployer
        .can_i(&check("create", "deploy", prod))
        .await
        .unwrap();
    assert!(deploy.allowed);
    assert_eq!(deploy.resource, "deployments.apps");
    let scale = AccessCheck {
        subresource: Some("scale".into()),
        ..check("update", "deployments.apps", prod)
    };
    assert!(deployer.can_i(&scale).await.unwrap().allowed);
    // The cluster's own credentials may do anything.
    assert!(
        fixture
            .admin
            .can_i(&check("delete", "secrets", other))
            .await
            .unwrap()
            .allowed
    );

    // What it may do here, as the API server lists it.
    let own = deployer.own_permissions(prod).await.unwrap();
    assert_eq!(own.namespace, fixture.prod);
    let user = own.user.expect("Kubernetes 1.28 and later say who");
    assert_eq!(
        user.username,
        format!("system:serviceaccount:{}:{SA}", fixture.prod)
    );
    assert!(user.groups.contains(&"system:serviceaccounts".to_owned()));
    assert!(own
        .grants
        .resources
        .iter()
        .any(|g| g.resource == "secrets" && g.verbs.iter().any(|v| v == "delete")));
    let elsewhere = deployer.own_permissions(other).await.unwrap();
    assert!(!elsewhere
        .grants
        .resources
        .iter()
        .any(|g| g.resource == "secrets"));

    // It may not list RBAC objects: the index says so instead of failing halfway.
    let refused = deployer.rbac_index(prod).await.unwrap_err();
    assert_eq!(refused.code, homeport_core::error::ErrorCode::Forbidden);
    fixture.clean_up().await;
}

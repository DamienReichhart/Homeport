use homeport_core::error::ErrorCode;
use homeport_kubernetes::helm;

use super::super::tests::{failed, gone, unknown};
use super::*;
use crate::app::testing::app;

fn install() -> InstallRequest {
    InstallRequest {
        name: "web".into(),
        namespace: "default".into(),
        chart: "bitnami/nginx".into(),
        version: None,
        values: String::new(),
        create_namespace: false,
        wait: false,
        timeout_seconds: None,
    }
}

fn upgrade() -> UpgradeRequest {
    UpgradeRequest {
        name: "web".into(),
        namespace: "default".into(),
        chart: "bitnami/nginx".into(),
        version: None,
        values: String::new(),
        wait: false,
        timeout_seconds: None,
    }
}

fn unheard(_: CliEvent) -> bool {
    panic!("helm ran against an unknown Cluster")
}

#[tokio::test]
async fn an_unknown_cluster_has_no_release() {
    let app = app();
    let s = &app.state;
    let (ns, name) = ("default".to_owned(), "web".to_owned());
    assert!(gone(clusters_helm_releases(s, unknown(), None).await));
    assert!(gone(
        clusters_helm_history(s, unknown(), ns.clone(), name.clone()).await
    ));
    assert!(gone(
        clusters_helm_release_details(s, unknown(), ns.clone(), name.clone(), None).await
    ));
    assert!(gone(
        clusters_helm_preview_install(s, unknown(), install()).await
    ));
    assert!(gone(
        clusters_helm_preview_upgrade(s, unknown(), upgrade()).await
    ));
    assert!(gone(
        clusters_helm_install_subscribe(s, unknown(), install(), unheard).await
    ));
    assert!(gone(
        clusters_helm_upgrade_subscribe(s, unknown(), upgrade(), unheard).await
    ));
    let rollback = RollbackRequest {
        name: name.clone(),
        namespace: ns.clone(),
        revision: 1,
        wait: false,
        timeout_seconds: None,
    };
    assert!(gone(
        clusters_helm_rollback_subscribe(s, unknown(), rollback, unheard).await
    ));
    let uninstall = UninstallRequest {
        name,
        namespace: ns,
        keep_history: false,
        wait: false,
        timeout_seconds: None,
    };
    assert!(gone(
        clusters_helm_uninstall_subscribe(s, unknown(), uninstall, unheard).await
    ));
}

/// With helm installed, only what can't touch this machine's helm
/// configuration is run: a refused name, and a repository that can't exist.
#[tokio::test]
async fn chart_repositories_need_helm_on_this_machine() {
    let app = app();
    let nowhere = || "homeport-test-no-such-repository".to_owned();
    let chart = || format!("{}/chart", nowhere());
    let url = || "https://example.com".to_owned();
    if helm::program().is_ok() {
        let invalid = helm_repos_add("not a name".into(), url()).await;
        assert!(failed(invalid, ErrorCode::Validation));
        // Nothing else is run here: the rest reads or refreshes this
        // machine's own repositories, some of it over the network.
        assert!(helm_repos_remove(nowhere()).await.is_err());
        return;
    }
    let missing = ErrorCode::HelmMissing;
    assert!(failed(helm_repos_list().await, missing));
    assert!(failed(helm_repos_add(nowhere(), url()).await, missing));
    assert!(failed(helm_repos_remove(nowhere()).await, missing));
    assert!(failed(helm_charts_search(nowhere(), false).await, missing));
    assert!(failed(helm_chart_values(chart(), None).await, missing));
    assert!(failed(helm_chart_versions(chart()).await, missing));
    assert!(failed(
        helm_chart_candidates(chart(), "1.0.0".into()).await,
        missing
    ));
    let update = helm_repos_update_subscribe(&app.state, vec![], |_| true).await;
    assert!(failed(update, missing));
}

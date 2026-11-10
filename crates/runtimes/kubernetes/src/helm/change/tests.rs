use super::*;
use crate::helm::ChartVersion;

fn upgrade() -> UpgradeRequest {
    UpgradeRequest {
        name: "web".into(),
        namespace: "shop".into(),
        chart: "bitnami/nginx".into(),
        version: Some("18.2.5".into()),
        values: "replicaCount: 3\n".into(),
        wait: true,
        timeout_seconds: None,
    }
}

#[test]
fn an_upgrade_sends_its_values_on_stdin() {
    assert_eq!(
        upgrade_args(&upgrade(), false),
        [
            "upgrade",
            "web",
            "bitnami/nginx",
            "--namespace",
            "shop",
            "--values",
            "-",
            "--version",
            "18.2.5",
            "--wait"
        ]
    );
    assert_eq!(
        upgrade_args(&upgrade(), true)[7..],
        [
            "--version",
            "18.2.5",
            "--dry-run=server",
            "--output",
            "json"
        ]
    );
}

#[test]
fn rollbacks_and_uninstalls_name_their_release() {
    assert_eq!(
        rollback_args(&RollbackRequest {
            name: "web".into(),
            namespace: "shop".into(),
            revision: 11,
            wait: true,
            timeout_seconds: Some(120),
        }),
        [
            "rollback",
            "web",
            "11",
            "--namespace",
            "shop",
            "--wait",
            "--timeout",
            "120s"
        ]
    );
    assert_eq!(
        uninstall_args(&UninstallRequest {
            name: "web".into(),
            namespace: "shop".into(),
            keep_history: true,
            wait: false,
            timeout_seconds: Some(120),
        }),
        ["uninstall", "web", "--namespace", "shop", "--keep-history"]
    );
}

#[test]
fn the_release_chart_is_found_in_the_repositories() {
    let found: Vec<ChartVersion> = [
        ("bitnami/nginx", "18.2.4"),
        ("bitnami/nginx", "18.2.5"),
        ("mirror/nginx", "18.2.5"),
        ("other/nginx", "18.2.4"),
        ("bitnami/nginx-ingress-controller", "11.0.0"),
    ]
    .map(|(name, version)| ChartVersion {
        name: name.into(),
        version: version.into(),
        app_version: String::new(),
        description: String::new(),
    })
    .to_vec();
    assert_eq!(
        candidates(&found, "nginx", "18.2.4"),
        ["bitnami/nginx", "other/nginx", "mirror/nginx"]
    );
    assert!(candidates(&found, "redis", "1.0.0").is_empty());
}

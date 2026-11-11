use homeport_core::error::ErrorCode;

use super::*;

fn request(chart: &str) -> InstallRequest {
    InstallRequest {
        name: "web".into(),
        namespace: "shop".into(),
        chart: chart.into(),
        version: Some("18.2.4".into()),
        values: "replicaCount: 2\n".into(),
        create_namespace: true,
        wait: true,
        timeout_seconds: Some(600),
    }
}

#[test]
fn a_repository_chart_installs_with_values_on_stdin() {
    assert_eq!(
        install_args(&request("bitnami/nginx"), false),
        [
            "install",
            "web",
            "bitnami/nginx",
            "--namespace",
            "shop",
            "--values",
            "-",
            "--version",
            "18.2.4",
            "--create-namespace",
            "--wait",
            "--timeout",
            "600s"
        ]
    );
}

#[test]
fn an_oci_chart_installs_without_a_repository_and_previews_as_json() {
    let mut r = request("oci://registry-1.docker.io/bitnamicharts/nginx");
    r.version = None;
    r.create_namespace = false;
    assert_eq!(
        install_args(&r, true),
        [
            "install",
            "web",
            "oci://registry-1.docker.io/bitnamicharts/nginx",
            "--namespace",
            "shop",
            "--values",
            "-",
            "--dry-run=server",
            "--output",
            "json"
        ]
    );
}

#[test]
fn release_names_are_checked_like_helm() {
    for ok in ["web", "my-app.v2", "a", &"a".repeat(53)] {
        assert!(validate_name(ok).is_ok(), "{ok}");
    }
    for bad in ["", "Web", "-web", "web-", "my_app", "a..b", &"a".repeat(54)] {
        assert!(
            validate_name(bad).unwrap_err().is(ErrorCode::Validation),
            "{bad}"
        );
    }
    assert!(validate_namespace("shop-1").is_ok());
    assert!(validate_namespace("shop.eu").is_err());
}

#[test]
fn only_repository_and_oci_charts_are_installed() {
    assert!(validate_chart("bitnami/nginx").is_ok());
    assert!(validate_chart("oci://ghcr.io/org/charts/app").is_ok());
    for bad in [
        "nginx",
        "./chart",
        "/tmp/app.tgz",
        "https://x/app.tgz",
        "a/b/c",
        "oci://x",
    ] {
        assert!(validate_chart(bad).is_err(), "{bad}");
    }
}

#[test]
fn values_must_be_a_yaml_mapping() {
    assert!(validate_values("").is_ok());
    assert!(validate_values("# nothing\n").is_ok());
    assert!(validate_values("image:\n  tag: 1.27\n").is_ok());
    let e = validate_values("image:\n  tag: 1.27\n bad: [\n").unwrap_err();
    assert!(e.is(ErrorCode::Validation));
    assert!(e.params.contains_key("line"));
    assert!(validate_values("- a\n- b\n").is_err());
}

#[test]
fn a_dry_run_gives_the_manifest_with_its_hooks_and_notes() {
    let run: DryRun = serde_json::from_str(
        r#"{"name":"web","info":{"status":"pending-install","notes":"Visit http://web"},"manifest":"---\nkind: Service\n","hooks":[{"name":"web-test","path":"nginx/templates/tests/test.yaml","manifest":"kind: Pod\n"}],"version":1}"#,
    )
    .unwrap();
    let preview = ReleasePreview::from(run);
    assert_eq!(
        preview.manifest,
        "---\nkind: Service\n---\n# Hook: nginx/templates/tests/test.yaml\nkind: Pod\n"
    );
    assert_eq!(preview.notes, "Visit http://web");
}

use super::*;

/// `helm list --all-namespaces --output json`, from Helm 3.16.
const LIST: &str = r#"[{"name":"cert-manager","namespace":"cert-manager","revision":"3","updated":"2026-10-02 09:14:27.554152 +0200 CEST","status":"deployed","chart":"cert-manager-v1.16.1","app_version":"v1.16.1"},{"name":"web","namespace":"shop","revision":"12","updated":"2026-11-01 18:02:03.1 +0000 UTC","status":"pending-upgrade","chart":"nginx-18.2.4","app_version":"1.27.2"},{"name":"db","namespace":"shop","revision":"1","updated":"2026-11-01 18:00:00.000000001 -0500 EST","status":"failed","chart":"my-db-chart-0.1.0-rc.1","app_version":""}]"#;

/// `helm history web --output json`.
const HISTORY: &str = r#"[{"revision":11,"updated":"2026-10-30T12:00:00.123456789Z","status":"superseded","chart":"nginx-18.2.3","app_version":"1.27.2","description":"Upgrade complete"},{"revision":12,"updated":"2026-11-01T18:02:03.1+01:00","status":"pending-upgrade","chart":"nginx-18.2.4","app_version":"1.27.2","description":"Preparing upgrade"}]"#;

const MANIFEST: &str = r#"---
# Source: nginx/templates/serviceaccount.yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: web-nginx
  namespace: "shop"
---
# Source: nginx/templates/svc.yaml
apiVersion: v1
kind: Service
metadata:
  name: web-nginx
spec:
  type: ClusterIP
---
# Source: nginx/templates/clusterrole.yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: web-reader
---
# Source: nginx/templates/empty.yaml
"#;

#[test]
fn helm_list_is_read() {
    let raw: Vec<RawRelease> = serde_json::from_str(LIST).unwrap();
    let releases: Vec<ReleaseSummary> = raw.into_iter().map(Into::into).collect();
    let cert = &releases[0];
    assert_eq!(cert.revision, 3);
    assert_eq!(cert.chart_name, "cert-manager");
    assert_eq!(cert.chart_version, "v1.16.1");
    assert_eq!(cert.updated.as_deref(), Some("2026-10-02T07:14:27.554152Z"));
    let web = &releases[1];
    assert_eq!(web.status, "pending-upgrade");
    assert_eq!(web.updated.as_deref(), Some("2026-11-01T18:02:03.1Z"));
    let db = &releases[2];
    assert_eq!(db.chart_name, "my-db-chart");
    assert_eq!(db.chart_version, "0.1.0-rc.1");
    assert_eq!(
        db.updated.as_deref(),
        Some("2026-11-01T23:00:00.000000001Z")
    );
}

#[test]
fn helm_history_is_read() {
    let raw: Vec<RawRevision> = serde_json::from_str(HISTORY).unwrap();
    let history: Vec<ReleaseRevision> = raw.into_iter().map(Into::into).collect();
    assert_eq!(history[0].revision, 11);
    assert_eq!(history[0].chart_version, "18.2.3");
    assert_eq!(history[0].description, "Upgrade complete");
    assert_eq!(
        history[1].updated.as_deref(),
        Some("2026-11-01T17:02:03.1Z")
    );
}

#[test]
fn chart_labels_split_at_the_version() {
    assert_eq!(
        split_chart("nginx-18.2.4"),
        ("nginx".into(), "18.2.4".into())
    );
    assert_eq!(
        split_chart("kube-prometheus-stack-65.1.0"),
        ("kube-prometheus-stack".into(), "65.1.0".into())
    );
    assert_eq!(
        split_chart("no-version"),
        ("no-version".into(), String::new())
    );
}

#[test]
fn a_manifest_names_the_objects_it_creates() {
    let found = resources(MANIFEST);
    assert_eq!(
        found,
        vec![
            ReleaseResource {
                api_version: "v1".into(),
                kind: "ServiceAccount".into(),
                name: "web-nginx".into(),
                namespace: Some("shop".into()),
            },
            ReleaseResource {
                api_version: "v1".into(),
                kind: "Service".into(),
                name: "web-nginx".into(),
                namespace: None,
            },
            ReleaseResource {
                api_version: "rbac.authorization.k8s.io/v1".into(),
                kind: "ClusterRole".into(),
                name: "web-reader".into(),
                namespace: None,
            },
        ]
    );
}

#[test]
fn empty_values_and_notes_headers_are_dropped() {
    assert_eq!(values_yaml("null\n".into()), "");
    assert_eq!(values_yaml("replicaCount: 2\n".into()), "replicaCount: 2\n");
    assert_eq!(
        notes_text("NOTES:\nVisit http://web\n"),
        "Visit http://web\n"
    );
}

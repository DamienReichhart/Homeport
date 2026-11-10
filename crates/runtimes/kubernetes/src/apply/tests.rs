use serde_json::json;

use super::*;

#[test]
fn several_documents_are_split_and_empty_ones_skipped() {
    let docs = documents(
        "---\napiVersion: v1\nkind: ConfigMap\nmetadata: { name: a }\n---\n---\napiVersion: apps/v1\nkind: Deployment\nmetadata: { name: b }\n",
    )
    .unwrap();
    assert_eq!(docs.len(), 2);
    assert_eq!(docs[1]["kind"], "Deployment");
}

#[test]
fn invalid_yaml_names_its_line() {
    let e = documents("apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: [a\n").unwrap_err();
    assert_eq!(e.code, ErrorCode::Validation);
    assert_eq!(e.key, "errors.clusters.apply.yamlInvalid");
    assert!(e.message.contains("line"), "{}", e.message);
    assert_eq!(
        documents("---\n").unwrap_err().key,
        "errors.clusters.apply.yamlEmpty"
    );
}

#[test]
fn a_document_without_a_name_is_explained() {
    let e = identity(&json!({ "apiVersion": "v1", "kind": "ConfigMap" }))
        .err()
        .unwrap();
    assert_eq!(e.key, "errors.clusters.apply.documentIncomplete");
    let id = identity(&json!({
        "apiVersion": "v1", "kind": "ConfigMap",
        "metadata": { "name": "a", "namespace": "shop" }
    }))
    .unwrap();
    assert_eq!(
        (id.name.as_str(), id.namespace.as_deref()),
        ("a", Some("shop"))
    );
}

#[test]
fn what_the_server_manages_is_left_out_of_the_diff() {
    let mut live = json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": "api",
            "resourceVersion": "41",
            "generation": 3,
            "managedFields": [{ "manager": "kubectl" }],
            "annotations": { "kubectl.kubernetes.io/last-applied-configuration": "{}" },
        },
        "spec": { "replicas": 2 },
        "status": { "readyReplicas": 2 },
    });
    normalize(&mut live);
    assert_eq!(
        live,
        json!({
            "apiVersion": "apps/v1",
            "kind": "Deployment",
            "metadata": { "name": "api" },
            "spec": { "replicas": 2 },
        })
    );
}

#[test]
fn applying_the_same_object_again_changes_nothing() {
    let before = json!({
        "kind": "ConfigMap", "metadata": { "name": "a", "resourceVersion": "1" }, "data": { "k": "v" }
    });
    let after = json!({
        "kind": "ConfigMap", "metadata": { "name": "a", "resourceVersion": "2" }, "data": { "k": "v" }
    });
    assert!(same(&before, &after));
    let changed = json!({ "kind": "ConfigMap", "metadata": { "name": "a" }, "data": { "k": "w" } });
    assert!(!same(&before, &changed));
}

#[test]
fn a_secrets_values_count_as_changes_but_stay_hidden() {
    let before = json!({ "kind": "Secret", "metadata": { "name": "s" }, "data": { "k": "YQ==" } });
    let after = json!({ "kind": "Secret", "metadata": { "name": "s" }, "data": { "k": "Yg==" } });
    assert!(!same(&before, &after));
    let shown = shown_normalized(&after);
    assert!(
        shown.contains("(hidden)") && !shown.contains("Yg=="),
        "{shown}"
    );
}

#[test]
fn an_edited_object_drops_what_the_server_manages_its_version_included() {
    let value = editable(json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": "a", "uid": "u", "resourceVersion": "7",
            "creationTimestamp": "2026-11-01T00:00:00Z", "managedFields": [],
        },
        "data": { "k": "v" },
    }));
    assert_eq!(value["metadata"], json!({ "name": "a" }));
}

#[test]
fn field_conflicts_are_read_from_the_answer() {
    let causes = vec![
        StatusCause {
            reason: "FieldManagerConflict".into(),
            message: "conflict with \"kubectl-client-side-apply\" using apps/v1".into(),
            field: ".spec.replicas".into(),
        },
        StatusCause {
            reason: "Other".into(),
            message: String::new(),
            field: String::new(),
        },
    ];
    let found = conflicts(&causes);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].field, ".spec.replicas");
    assert!(found[0].message.contains("kubectl-client-side-apply"));
}

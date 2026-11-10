use super::testing::{agent, ok};
use super::*;

#[test]
fn path_segments_are_escaped() {
    assert_eq!(segment("web"), "web");
    assert_eq!(
        segment("backup/periodic-1700000000"),
        "backup%2Fperiodic-1700000000"
    );
    assert_eq!(segment("a b?"), "a%20b%3F");
}

#[test]
fn the_config_never_prints_its_secrets() {
    let config = NomadConfig {
        name: "prod".into(),
        token: Some("secret-token".into()),
        client_key: Some("PRIVATE KEY".into()),
        ..Default::default()
    };
    let printed = format!("{config:?}");
    assert!(!printed.contains("secret-token") && !printed.contains("PRIVATE"));
}

#[tokio::test]
async fn the_token_goes_in_a_header_and_the_region_in_the_query() {
    let fake = agent(vec![ok("GET /v1/regions", r#"["eu"]"#)]).await;
    let nomad = Nomad::new(&NomadConfig {
        name: "prod".into(),
        address: fake.address.clone(),
        region: Some("eu".into()),
        token: Some("secret-token".into()),
        ..Default::default()
    })
    .unwrap();
    let regions: Vec<String> = nomad.get("/v1/regions", &[]).await.unwrap();
    assert_eq!(regions, ["eu"]);
    let sent = &fake.received()[0];
    assert_eq!((sent.method.as_str(), sent.body.as_str()), ("GET", ""));
    assert_eq!(sent.headers["x-nomad-token"], "secret-token");
    assert!(sent.target.contains("region=eu"), "{}", sent.target);
    assert!(!sent.target.contains("secret-token"));
}

#[tokio::test]
async fn no_token_sends_no_header() {
    let fake = agent(vec![ok("GET /v1/regions", "[]")]).await;
    let _: Vec<String> = fake
        .client(Some("  "))
        .get("/v1/regions", &[])
        .await
        .unwrap();
    assert!(!fake.received()[0].headers.contains_key("x-nomad-token"));
}

#[tokio::test]
async fn error_answers_are_classified() {
    let fake = agent(vec![(
        "GET /v1/jobs",
        403,
        vec![],
        "ACL token not found".into(),
    )])
    .await;
    let e = fake
        .client(Some("t"))
        .get::<serde_json::Value>("/v1/jobs", &[])
        .await
        .unwrap_err();
    assert!(e.is(ErrorCode::NomadTokenInvalid));
}

#[tokio::test]
async fn an_answer_that_isnt_json_is_unreadable() {
    let fake = agent(vec![ok("GET /v1/jobs", "<html>")]).await;
    let e = fake
        .client(None)
        .get::<serde_json::Value>("/v1/jobs", &[])
        .await
        .unwrap_err();
    assert_eq!(e.key, "errors.nomad.unreadable");
}

#[tokio::test]
async fn a_blocking_query_sends_its_index_and_reads_the_next() {
    let fake = agent(vec![(
        "GET /v1/jobs",
        200,
        vec![("X-Nomad-Index", "42".into())],
        "[]".into(),
    )])
    .await;
    let nomad = fake.client(None);
    let first: Indexed<Vec<serde_json::Value>> = nomad
        .get_indexed("/v1/jobs", &[], 0, Duration::from_secs(30))
        .await
        .unwrap();
    assert_eq!(first.index, 42);
    nomad
        .get_indexed::<Vec<serde_json::Value>>("/v1/jobs", &[], 42, Duration::from_secs(30))
        .await
        .unwrap();
    let sent = fake.received();
    assert!(!sent[0].target.contains("index="));
    assert!(sent[1].target.contains("index=42") && sent[1].target.contains("wait=30s"));
}

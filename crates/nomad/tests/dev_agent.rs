//! Against a real Nomad agent: `nomad agent -dev`, reached at `NOMAD_ADDR`
//! (http://127.0.0.1:4646 by default) with `NOMAD_TOKEN` when its ACLs are
//! on. Run with `cargo test -p homeport-nomad -- --ignored nomad_live`, as
//! CI's nomad-tests job does.

use homeport_nomad::{normalize_address, Nomad, NomadConfig};

fn dev_agent() -> Nomad {
    let address = std::env::var("NOMAD_ADDR").unwrap_or_else(|_| "http://127.0.0.1:4646".into());
    Nomad::new(&NomadConfig {
        name: "nomad-dev".into(),
        address: normalize_address(&address, false).unwrap(),
        token: std::env::var("NOMAD_TOKEN").ok(),
        ..Default::default()
    })
    .unwrap()
}

#[tokio::test]
#[ignore = "needs a local Nomad agent"]
async fn nomad_live_a_dev_agent_reports_its_version_and_region() {
    let agent = dev_agent().agent().await.unwrap();
    assert!(agent.version.is_some_and(|v| !v.is_empty()));
    assert_eq!(agent.region.as_deref(), Some("global"));
    assert!(!agent.leader.is_empty());
}

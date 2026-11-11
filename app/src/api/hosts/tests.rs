use std::collections::HashMap;

use homeport_core::error::ErrorCode;
use homeport_hosts::{
    registry::EngineEvent,
    service::{BundleChoice, HostInput},
    EngineKind, HostConnection, SshAuth,
};

use super::*;
use crate::app::testing::{app, TestApp};

const PASS: &str = "a long enough passphrase";

/// A Host nothing answers for: it is stored and listed, never connected.
fn tcp(name: &str) -> HostInput {
    input(
        name,
        HostConnection::Tcp {
            address: "127.0.0.1:1".into(),
        },
    )
}

/// An SSH Host whose key Homeport generates.
fn ssh(name: &str) -> HostInput {
    input(
        name,
        HostConnection::Ssh {
            host: "127.0.0.1".into(),
            port: 1,
            user: "deploy".into(),
            socket_path: "/var/run/docker.sock".into(),
            auth: SshAuth::Generated,
            public_key: None,
            bastions: Vec::new(),
        },
    )
}

fn input(name: &str, connection: HostConnection) -> HostInput {
    HostInput {
        name: name.into(),
        connection,
        engine: EngineKind::Docker,
        credentials: None,
        bastion_keys: Vec::new(),
    }
}

/// How many times the app said its Hosts changed since the last call.
fn hosts_changed(app: &mut TestApp) -> usize {
    std::iter::from_fn(|| app.events.try_recv().ok())
        .filter(|e| matches!(e, Event::HostsChanged))
        .count()
}

fn public_key(view: &HostView) -> String {
    match &view.host.connection {
        HostConnection::Ssh {
            public_key: Some(key),
            ..
        } => key.clone(),
        other => panic!("no generated key in {other:?}"),
    }
}

fn event(time_nano: i64, action: &str) -> EngineEvent {
    EngineEvent {
        time: time_nano / 1_000_000_000,
        time_nano,
        kind: "container".into(),
        action: action.into(),
        actor_id: "abc".into(),
        attributes: HashMap::new(),
    }
}

#[tokio::test]
async fn a_created_host_is_listed() {
    let app = app();
    assert!(hosts_list(&app.state).unwrap().is_empty());

    let created = hosts_create(&app.state, tcp("nas")).await.unwrap();
    assert_eq!(created.host.name, "nas");
    assert!(created.host.enabled);

    let listed = hosts_list(&app.state).unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].host.id, created.host.id);

    let twice = hosts_create(&app.state, tcp("nas")).await.unwrap_err();
    assert_eq!(twice.code, ErrorCode::NameInUse);
}

#[tokio::test]
async fn updating_a_host_says_hosts_changed() {
    let mut app = app();
    let id = hosts_create(&app.state, tcp("nas")).await.unwrap().host.id;
    hosts_changed(&mut app);

    let updated = hosts_update(&app.state, id.to_string(), tcp("attic"))
        .await
        .unwrap();
    assert_eq!(updated.host.name, "attic");
    assert_eq!(hosts_list(&app.state).unwrap()[0].host.name, "attic");
    assert_eq!(hosts_changed(&mut app), 1);

    let unknown = hosts_update(&app.state, "nope".into(), tcp("cellar"))
        .await
        .unwrap_err();
    assert_eq!(unknown.code, ErrorCode::NotFound);
    assert_eq!(hosts_changed(&mut app), 0);
}

#[tokio::test]
async fn deleting_a_host_says_hosts_changed() {
    let mut app = app();
    let id = hosts_create(&app.state, tcp("nas")).await.unwrap().host.id;
    hosts_changed(&mut app);

    hosts_delete(&app.state, id.to_string()).unwrap();
    assert!(hosts_list(&app.state).unwrap().is_empty());
    assert_eq!(hosts_changed(&mut app), 1);

    let again = hosts_delete(&app.state, id.to_string()).unwrap_err();
    assert_eq!(again.code, ErrorCode::NotFound);
    assert_eq!(hosts_changed(&mut app), 0);
}

#[tokio::test]
async fn a_host_is_disabled_and_enabled_again() {
    let app = app();
    let id = hosts_create(&app.state, tcp("nas")).await.unwrap().host.id;

    let off = hosts_set_enabled(&app.state, id.to_string(), false)
        .await
        .unwrap();
    assert!(!off.host.enabled);

    let on = hosts_set_enabled(&app.state, id.to_string(), true)
        .await
        .unwrap();
    assert!(on.host.enabled);

    let unknown = hosts_set_enabled(&app.state, "nope".into(), false)
        .await
        .unwrap_err();
    assert_eq!(unknown.code, ErrorCode::NotFound);
}

#[tokio::test]
async fn reconnecting_needs_a_host() {
    let app = app();
    let id = hosts_create(&app.state, tcp("nas")).await.unwrap().host.id;

    let view = hosts_reconnect(&app.state, id.to_string()).await.unwrap();
    assert_eq!(view.host.id, id);

    let unknown = hosts_reconnect(&app.state, "nope".into())
        .await
        .unwrap_err();
    assert_eq!(unknown.code, ErrorCode::NotFound);
}

#[tokio::test]
async fn an_ssh_hosts_key_is_shown_and_regenerated() {
    let app = app();
    let created = hosts_create(&app.state, ssh("vps")).await.unwrap();
    let id = created.host.id.to_string();
    let key = public_key(&created);

    let setup = hosts_ssh_setup(&app.state, id.clone()).unwrap();
    assert_eq!(setup.public_key, key);
    assert!(!setup.fingerprint.is_empty());
    assert!(setup.install_command.contains(key.trim()));
    assert_eq!(setup.servers, ["deploy@127.0.0.1"]);

    let regenerated = hosts_regenerate_key(&app.state, id.clone()).await.unwrap();
    assert_ne!(public_key(&regenerated), key);
    assert_eq!(
        hosts_ssh_setup(&app.state, id).unwrap().public_key,
        public_key(&regenerated)
    );
}

#[tokio::test]
async fn keys_are_only_for_ssh_hosts() {
    let app = app();
    let id = hosts_create(&app.state, tcp("nas")).await.unwrap().host.id;

    let setup = hosts_ssh_setup(&app.state, id.to_string()).unwrap_err();
    assert_eq!(setup.code, ErrorCode::Validation);
    let regenerate = hosts_regenerate_key(&app.state, id.to_string())
        .await
        .unwrap_err();
    assert_eq!(regenerate.code, ErrorCode::Validation);
}

#[tokio::test]
async fn a_server_key_is_trusted() {
    let app = app();
    let created = hosts_create(&app.state, ssh("vps")).await.unwrap();
    let id = created.host.id.to_string();
    // Any OpenSSH public key stands for the server's here.
    let key = public_key(&created);

    let trusted = hosts_trust_key(&app.state, id.clone(), key.clone(), None)
        .await
        .unwrap();
    assert_eq!(trusted.host.host_key.as_deref(), Some(key.trim()));

    let garbage = hosts_trust_key(&app.state, id.clone(), "not a key".into(), None)
        .await
        .unwrap_err();
    assert_eq!(garbage.code, ErrorCode::Validation);

    let no_bastion = hosts_trust_key(&app.state, id, key, Some("10.0.0.1:22".into())).await;
    assert!(no_bastion.is_err());
}

#[tokio::test]
async fn kept_engine_events_are_paged_and_cleared() {
    let app = app();
    let id = hosts_create(&app.state, tcp("nas"))
        .await
        .unwrap()
        .host
        .id
        .to_string();
    app.state.hosts.events.record(&id, &event(1_000, "start"));
    app.state.hosts.events.record(&id, &event(2_000, "die"));

    let all = hosts_events(&app.state, id.clone(), None, None)
        .await
        .unwrap();
    let actions: Vec<_> = all.iter().map(|e| e.action.as_str()).collect();
    assert_eq!(actions, ["start", "die"]);

    // A limit of 0 still asks for one event: the latest.
    let latest = hosts_events(&app.state, id.clone(), Some(0), None)
        .await
        .unwrap();
    assert_eq!(latest.len(), 1);
    assert_eq!(latest[0].action, "die");

    let before = EventCursor {
        time_nano: 2_000,
        kind: "container".into(),
        action: "die".into(),
        actor_id: "abc".into(),
    };
    let older = hosts_events(&app.state, id.clone(), None, Some(before))
        .await
        .unwrap();
    assert_eq!(older.len(), 1);
    assert_eq!(older[0].action, "start");

    hosts_events_clear(&app.state, id.clone()).await.unwrap();
    assert!(hosts_events(&app.state, id, None, None)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn exported_hosts_are_previewed_and_imported() {
    let app = app();
    let id = hosts_create(&app.state, ssh("vps")).await.unwrap().host.id;
    let path = app.dir.path().join("hosts.homeport");

    let nothing = hosts_export(&app.state, Vec::new(), path.clone(), PASS.into())
        .await
        .unwrap_err();
    assert_eq!(nothing.code, ErrorCode::Validation);

    let outcome = hosts_export(&app.state, vec![id.to_string()], path.clone(), PASS.into())
        .await
        .unwrap();
    assert_eq!(outcome.exported, 1);
    assert!(outcome.without_secrets.is_empty());

    let wrong = hosts_bundle_preview(&app.state, path.clone(), "another passphrase".into())
        .await
        .unwrap_err();
    assert_eq!(wrong.code, ErrorCode::BundleUnreadable);

    let preview = hosts_bundle_preview(&app.state, path.clone(), PASS.into())
        .await
        .unwrap();
    assert_eq!(preview.len(), 1);
    assert_eq!(preview[0].name, "vps");
    assert!(preview[0].name_taken);
    assert!(!preview[0].secrets_missing);

    let choices = vec![BundleChoice {
        index: preview[0].index,
        name: preview[0].suggested_name.clone(),
    }];
    let imported = hosts_bundle_import(&app.state, path, PASS.into(), choices)
        .await
        .unwrap();
    assert!(imported.failed.is_empty());
    assert_eq!(imported.created.len(), 1);
    assert_eq!(imported.created[0].host.name, preview[0].suggested_name);
    assert_eq!(hosts_list(&app.state).unwrap().len(), 2);
}

/// These read the files of whoever runs the tests, so only what holds for
/// any of them is checked: choosing nothing creates nothing.
#[tokio::test]
async fn importing_no_choice_creates_no_host() {
    let app = app();

    if let Ok(drafts) = hosts_ssh_config_preview(&app.state).await {
        assert!(drafts.iter().all(|d| !d.name_taken));
    }
    if let Ok(outcome) = hosts_ssh_config_import(&app.state, Vec::new()).await {
        assert!(outcome.created.is_empty() && outcome.failed.is_empty());
    }
    if let Ok(drafts) = hosts_docker_contexts_preview(&app.state).await {
        assert!(drafts.iter().all(|d| !d.name_taken));
    }
    if let Ok(outcome) = hosts_docker_contexts_import(&app.state, Vec::new()).await {
        assert!(outcome.created.is_empty() && outcome.failed.is_empty());
    }
    assert!(hosts_list(&app.state).unwrap().is_empty());
}

#[tokio::test]
async fn detected_engines_have_a_socket() {
    let detected = hosts_detect_local().await;
    assert!(detected.iter().all(|d| !d.socket_path.is_empty()));
}

use super::*;
use crate::{runtime::StackSource, store::ComposeFile};

#[test]
fn compose_arguments_become_stack_deploy_ones() {
    let one = StackSource::Managed {
        files: vec![ComposeFile::new(
            "compose.yaml",
            "services:\n  web:\n    image: nginx\n",
        )],
        env: String::new(),
    };
    let prepared = one.prepare_for(ComposeAction::Up, true).unwrap();
    assert_eq!(compose_file_args(&prepared), ["--compose-file", "-"]);

    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("compose.yaml");
    std::fs::write(&file, "services:\n  web:\n    image: nginx\n").unwrap();
    std::fs::write(dir.path().join(".env"), "TAG=1.27\nA=folder\n").unwrap();
    let linked = StackSource::Synced {
        working_dir: dir.path().to_owned(),
        config_files: vec![file.to_string_lossy().into_owned()],
        env: "A=stack\n".into(),
    };
    let prepared = linked.prepare_for(ComposeAction::Up, true).unwrap();
    assert_eq!(
        compose_file_args(&prepared),
        [
            "--compose-file".to_owned(),
            file.to_string_lossy().into_owned()
        ]
    );
    // The folder's .env is read, and the Stack's own variables win.
    let vars = variables(&prepared).unwrap();
    assert_eq!(
        vars,
        [
            ("TAG".to_owned(), "1.27".to_owned()),
            ("A".to_owned(), "folder".to_owned()),
            ("A".to_owned(), "stack".to_owned()),
        ]
    );
}

#[test]
fn a_build_only_service_is_refused_before_anything_runs() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("compose.yaml");
    std::fs::write(&file, "services:\n  app:\n    build: .\n").unwrap();
    let linked = StackSource::Linked {
        working_dir: dir.path().to_owned(),
        config_files: vec![file.to_string_lossy().into_owned()],
    };
    assert!(linked.prepare_for(ComposeAction::Up, false).is_ok());
    let refused = linked.prepare_for(ComposeAction::Up, true).unwrap_err();
    assert!(refused.is(ErrorCode::Validation), "{refused:?}");
}

use std::sync::{Arc, Mutex};

use homeport_core::host::HostConnection;
use homeport_engine::{auth::Logins, swarm::services, testing};
#[cfg(unix)]
use homeport_transport::testing::UnixDialer;
use homeport_transport::Dialer;

use crate::runtime::{self, Reach};

/// Runs `action` for a Managed Stack on the local Engine's throwaway
/// Swarm, collecting what it sends.
async fn run(
    name: &str,
    env: &str,
    action: ComposeAction,
    dialer: Option<Arc<dyn Dialer>>,
    connection: HostConnection,
) -> Vec<ComposeEvent> {
    let compose = format!(
            "services:\n  app:\n    image: {}\n    command: [\"sleep\", \"300\"]\n    environment:\n      GREETING: ${{GREETING}}\n",
            testing::IMAGE
        );
    let job = ComposeJob {
        cli: detect().await.expect("the docker CLI is installed"),
        project: name.into(),
        source: StackSource::Managed {
            files: vec![ComposeFile::new("compose.yaml", compose)],
            env: env.into(),
        },
        target: StackTarget::Swarm {
            swarm_id: "test".into(),
            name: "the test Swarm".into(),
            manager: Reach {
                name: "local".into(),
                connection,
                dialer,
                docker: testing::docker(),
            },
        },
        logins: Logins::none(),
        action,
        profiles: vec![],
        record: None,
        registry_auth: false,
    };
    let events = Arc::new(Mutex::new(Vec::new()));
    let sink = {
        let events = events.clone();
        move |e| {
            events.lock().unwrap().push(e);
            true
        }
    };
    runtime::run(job, sink).await;
    let events = events.lock().unwrap().clone();
    events
}

fn succeeded(events: &[ComposeEvent]) -> bool {
    matches!(
        events.last(),
        Some(ComposeEvent::Exit { success: true, .. })
    )
}

async fn greeting(name: &str) -> Option<String> {
    let docker = testing::docker();
    let service = docker
        .inspect_service(&format!("{name}_app"), None)
        .await
        .ok()?;
    service
        .spec?
        .task_template?
        .container_spec?
        .env?
        .into_iter()
        .find_map(|e| e.strip_prefix("GREETING=").map(str::to_owned))
}

async fn deploys_redeploys_and_removes(
    dialer: Option<Arc<dyn Dialer>>,
    connection: HostConnection,
) {
    let docker = testing::docker();
    let _swarm = testing::swarm(&docker).await;
    testing::ensure_image(&docker).await;
    let name = testing::unique("swarm").replace('-', "");
    let up = |env: &'static str| {
        run(
            &name,
            env,
            ComposeAction::Up,
            dialer.clone(),
            connection.clone(),
        )
    };

    let events = up("GREETING=hello").await;
    assert!(succeeded(&events), "{events:?}");
    assert_eq!(greeting(&name).await.as_deref(), Some("hello"));

    let events = up("GREETING=bonjour").await;
    assert!(succeeded(&events), "{events:?}");
    assert_eq!(greeting(&name).await.as_deref(), Some("bonjour"));

    let listed = services::list(&docker).await.unwrap();
    assert!(listed
        .iter()
        .any(|s| s.stack.as_deref() == Some(name.as_str())));

    let events = run(
        &name,
        "",
        ComposeAction::Down,
        dialer.clone(),
        connection.clone(),
    )
    .await;
    assert!(succeeded(&events), "{events:?}");
    assert_eq!(greeting(&name).await, None);
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and the docker CLI"]
async fn managed_stacks_deploy_to_a_swarm() {
    deploys_redeploys_and_removes(
        None,
        HostConnection::Local {
            socket_path: "/var/run/docker.sock".into(),
        },
    )
    .await;
}

#[cfg(unix)]
#[tokio::test]
#[ignore = "needs a local Docker Engine and the docker CLI"]
async fn managed_stacks_deploy_to_a_swarm_through_the_proxy() {
    deploys_redeploys_and_removes(
        Some(Arc::new(UnixDialer)),
        HostConnection::Tls {
            address: "unused:2376".into(),
        },
    )
    .await;
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and the docker CLI"]
async fn a_swarm_refuses_to_pull() {
    let events = run(
        "nopull",
        "",
        ComposeAction::Pull,
        None,
        HostConnection::Local {
            socket_path: "/var/run/docker.sock".into(),
        },
    )
    .await;
    assert!(
        matches!(events.last(), Some(ComposeEvent::Error { .. })),
        "{events:?}"
    );
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and the docker CLI"]
async fn linked_stacks_deploy_their_file_configs_and_env_to_a_swarm() {
    let docker = testing::docker();
    let _swarm = testing::swarm(&docker).await;
    testing::ensure_image(&docker).await;
    let name = testing::unique("linked").replace('-', "");
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.conf"), "port: 80\n").unwrap();
    let compose = format!(
            "services:\n  app:\n    image: {}\n    command: [\"sleep\", \"300\"]\n    environment:\n      GREETING: ${{GREETING}}\n    configs:\n      - source: conf\n        target: /app.conf\nconfigs:\n  conf:\n    file: ./app.conf\n",
            testing::IMAGE
        );
    let file = dir.path().join("compose.yaml");
    std::fs::write(&file, compose).unwrap();
    let deploy = |action: ComposeAction| {
        let job = ComposeJob {
            cli: ComposeCli {
                program: "docker".into(),
                prefix: vec![],
                version: String::new(),
            },
            project: name.clone(),
            source: StackSource::Linked {
                working_dir: dir.path().to_owned(),
                config_files: vec![file.to_string_lossy().into_owned()],
            },
            target: StackTarget::Swarm {
                swarm_id: "test".into(),
                name: "the test Swarm".into(),
                manager: Reach {
                    name: "local".into(),
                    connection: HostConnection::Local {
                        socket_path: "/var/run/docker.sock".into(),
                    },
                    dialer: None,
                    docker: testing::docker(),
                },
            },
            logins: Logins::none(),
            action,
            profiles: vec![],
            record: None,
            registry_auth: false,
        };
        async move {
            let events = Arc::new(Mutex::new(Vec::new()));
            let sink = {
                let events = events.clone();
                move |e| {
                    events.lock().unwrap().push(e);
                    true
                }
            };
            runtime::run(job, sink).await;
            let events = events.lock().unwrap().clone();
            events
        }
    };

    std::fs::write(dir.path().join(".env"), "GREETING=hello\n").unwrap();
    let events = deploy(ComposeAction::Up).await;
    assert!(succeeded(&events), "{events:?}");
    assert_eq!(greeting(&name).await.as_deref(), Some("hello"));
    let parts = swarm::parts(&docker, &name).await.unwrap();
    assert_eq!(parts.configs, [format!("{name}_conf")]);

    // The folder's .env changed: the redeploy reads it again.
    std::fs::write(dir.path().join(".env"), "GREETING=bonjour\n").unwrap();
    let events = deploy(ComposeAction::Up).await;
    assert!(succeeded(&events), "{events:?}");
    assert_eq!(greeting(&name).await.as_deref(), Some("bonjour"));

    // A changed config file can't be updated in place: the hint says to version it.
    std::fs::write(dir.path().join("app.conf"), "port: 8080\n").unwrap();
    let events = deploy(ComposeAction::Up).await;
    let error = events.iter().find_map(|e| match e {
        ComposeEvent::Exit {
            error: Some(error), ..
        } => Some(error.clone()),
        _ => None,
    });
    assert_eq!(
        error.map(|e| e.key),
        Some("errors.compose.swarmObjectChanged".to_owned()),
        "{events:?}"
    );

    let events = deploy(ComposeAction::Down).await;
    assert!(succeeded(&events), "{events:?}");
}

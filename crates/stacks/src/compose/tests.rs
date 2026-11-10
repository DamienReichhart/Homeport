use super::*;
use std::sync::Arc;

use homeport_engine::{auth::Logins, testing};
#[cfg(unix)]
use homeport_transport::testing::UnixDialer;
use homeport_transport::Dialer;

use crate::{lifecycle, runtime::StackSource, store::ComposeFile};

#[test]
fn direct_hosts_are_passed_to_the_cli() {
    if cfg!(unix) {
        assert_eq!(
            direct_docker_host(&HostConnection::Local {
                socket_path: "/var/run/docker.sock".into()
            }),
            Some("unix:///var/run/docker.sock".into())
        );
    }
    assert_eq!(
        direct_docker_host(&HostConnection::Tcp {
            address: "10.0.0.1:2375".into()
        }),
        Some("tcp://10.0.0.1:2375".into())
    );
    assert_eq!(
        direct_docker_host(&HostConnection::Tls {
            address: "h:2376".into()
        }),
        None
    );
}

fn job(
    source: StackSource,
    connection: HostConnection,
    dialer: Option<Arc<dyn Dialer>>,
    docker: bollard::Docker,
    logins: Logins,
    project: &str,
    action: ComposeAction,
) -> ComposeJob {
    ComposeJob {
        cli: docker_cli(),
        project: project.into(),
        source,
        target: StackTarget::Host(crate::runtime::Reach {
            name: "local".into(),
            connection,
            dialer,
            docker,
        }),
        logins,
        action,
        profiles: vec![],
        record: None,
        registry_auth: false,
    }
}

fn docker_cli() -> ComposeCli {
    ComposeCli {
        program: "docker".into(),
        prefix: vec!["compose".into()],
        version: "2.29.0".into(),
    }
}

/// Every argument and environment variable of `command`, as text.
fn everything(command: &Command) -> Vec<String> {
    let std = command.as_std();
    std.get_envs()
        .filter_map(|(k, v)| Some(format!("{}={}", k.to_str()?, v?.to_str()?)))
        .chain(std.get_args().filter_map(|a| a.to_str().map(str::to_owned)))
        .collect()
}

#[test]
fn the_clis_environment_holds_no_registry_login() {
    let job = job(
        StackSource::Managed {
            files: vec![ComposeFile::new("compose.yaml", "")],
            env: "GREETING=hello".into(),
        },
        HostConnection::Tcp {
            address: "h:2375".into(),
        },
        None,
        testing::offline(),
        Logins::new(
            vec![homeport_engine::auth::RegistryAuth {
                address: "ghcr.io".into(),
                username: "alice".into(),
                secret: "ghp_TOPSECRET".into(),
            }],
            |_| "ghcr.io".into(),
        ),
        "shop",
        ComposeAction::Up,
    );
    let prepared = job.source.prepare(ComposeAction::Down).unwrap();
    let everything = everything(&cli_command(&job, &prepared, "tcp://h:2375", &["up"]));
    assert!(everything.contains(&"DOCKER_HOST=tcp://h:2375".to_owned()));
    assert!(everything.contains(&"GREETING=hello".to_owned()));
    assert!(
        !everything
            .iter()
            .any(|e| e.contains("ghp_TOPSECRET") || e.contains("DOCKER_CONFIG")),
        "{everything:?}"
    );
}

#[test]
fn the_source_names_the_files_and_the_target_the_host() {
    let job = job(
        StackSource::Managed {
            files: vec![ComposeFile::new(
                "compose.yaml",
                "services:\n  web:\n    image: nginx\n",
            )],
            env: String::new(),
        },
        HostConnection::Tcp {
            address: "h:2375".into(),
        },
        None,
        testing::offline(),
        Logins::none(),
        "shop",
        ComposeAction::Up,
    );
    let prepared = job.source.prepare(job.action).unwrap();
    let (host, proxy) = docker_host(&job.target).unwrap();
    assert!(proxy.is_none());
    let command = cli_command(&job, &prepared, &host, job.action.args());
    let args: Vec<_> = command
        .as_std()
        .get_args()
        .filter_map(|a| a.to_str())
        .collect();
    assert_eq!(
        args,
        [
            "compose",
            "--project-name",
            "shop",
            "--file",
            "-",
            "--progress",
            "plain",
            "up",
            "--detach",
            "--remove-orphans"
        ]
    );
    assert_eq!(
        command.as_std().get_current_dir(),
        Some(prepared.workdir.path())
    );
}

const WITH_DEBUG: &str = "services:\n  web:\n    image: nginx\n  debug:\n    image: busybox\n    profiles: [debug]\n  tools:\n    image: busybox\n    profiles: [tools]\n";

fn profile_job(action: ComposeAction, active: &[&str], env: &str) -> ComposeJob {
    let mut job = job(
        StackSource::Managed {
            files: vec![ComposeFile::new("compose.yaml", WITH_DEBUG)],
            env: env.into(),
        },
        HostConnection::Tcp {
            address: "h:2375".into(),
        },
        None,
        testing::offline(),
        Logins::none(),
        "shop",
        action,
    );
    job.profiles = active.iter().map(|p| (*p).to_owned()).collect();
    job
}

#[test]
fn only_the_stack_s_profiles_are_deployed() {
    // COMPOSE_PROFILES in the Stack's environment doesn't decide.
    let job = profile_job(ComposeAction::Up, &["debug"], "COMPOSE_PROFILES=tools\n");
    let prepared = job.source.prepare(job.action).unwrap();
    let command = cli_command(&job, &prepared, "tcp://h:2375", job.action.args());
    let all = everything(&command);
    let args: Vec<_> = command
        .as_std()
        .get_args()
        .filter_map(|a| a.to_str())
        .collect();
    assert_eq!(args.iter().filter(|a| **a == "--profile").count(), 1);
    assert!(args.windows(2).any(|w| w == ["--profile", "debug"]));
    let envs: Vec<_> = command
        .as_std()
        .get_envs()
        .filter(|(k, _)| *k == "COMPOSE_PROFILES")
        .map(|(_, v)| v.and_then(|v| v.to_str()))
        .collect();
    assert_eq!(envs, [Some("debug")], "{all:?}");
}

#[test]
fn taking_down_includes_every_declared_profile() {
    let job = profile_job(ComposeAction::Down, &[], "");
    let prepared = job.source.prepare(job.action).unwrap();
    let args = everything(&cli_command(
        &job,
        &prepared,
        "tcp://h:2375",
        job.action.args(),
    ));
    let joined = args.join(" ");
    assert!(
        joined.contains("--profile debug --profile tools"),
        "{joined}"
    );
    // With none turned on, deploying passes none, and pins it empty.
    let job = profile_job(ComposeAction::Up, &[], "");
    let prepared = job.source.prepare(job.action).unwrap();
    let args = everything(&cli_command(
        &job,
        &prepared,
        "tcp://h:2375",
        job.action.args(),
    ));
    assert!(!args.contains(&"--profile".to_owned()));
    assert!(args.contains(&"COMPOSE_PROFILES=".to_owned()));
}

/// Runs with a stand-in compose CLI, a shell script, so it needs a Unix shell.
#[cfg(unix)]
mod stand_in_cli {
    use std::{
        os::unix::fs::PermissionsExt,
        path::{Path, PathBuf},
    };

    use super::*;

    /// A stand-in compose CLI running `body` as a shell script; `ran` is
    /// the file it may record commands in.
    fn script_cli(dir: &Path, body: &str) -> (ComposeCli, PathBuf) {
        let ran = dir.join("ran");
        let script = dir.join("compose");
        std::fs::write(
            &script,
            format!(
                "#!/bin/sh\n{}",
                body.replace("$RAN", &ran.display().to_string())
            ),
        )
        .unwrap();
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
        (
            ComposeCli {
                program: script.to_string_lossy().into_owned(),
                prefix: vec![],
                version: "2.29.0".into(),
            },
            ran,
        )
    }

    /// Answers `config` with one service, without reading the definition
    /// on its stdin, and records any other command in `ran`.
    fn fake_cli(dir: &Path) -> (ComposeCli, PathBuf) {
        script_cli(
                dir,
                "case \"$*\" in\n  *config*) echo '{\"services\":{\"web\":{\"image\":\"nginx:1.27\"}}}' ;;\n  *) echo \"$*\" >> $RAN ;;\nesac\n",
            )
    }

    /// Runs `job` and collects its events.
    async fn events_of(job: ComposeJob) -> Vec<ComposeEvent> {
        let events = std::sync::Mutex::new(Vec::new());
        run(job, |e| {
            events.lock().unwrap().push(e);
            true
        })
        .await;
        events.into_inner().unwrap()
    }

    #[tokio::test]
    async fn a_cli_that_exits_before_reading_its_input_is_explained_by_its_output() {
        let dir = tempfile::tempdir().unwrap();
        let (cli, _) = script_cli(dir.path(), "echo 'unknown flag: --bogus' >&2\nexit 1\n");
        let mut job = profile_job(ComposeAction::Up, &[], "");
        // Far more than a pipe holds, so writing it always meets the closed pipe.
        let padding = "# padding\n".repeat(64 * 1024);
        job.source = StackSource::Managed {
            files: vec![ComposeFile::new(
                "compose.yaml",
                format!("{WITH_DEBUG}{padding}"),
            )],
            env: String::new(),
        };
        job.cli = cli;
        let events = events_of(job).await;
        let Some(ComposeEvent::Error { error }) = events.last() else {
            panic!("{events:?}");
        };
        assert!(!error.is(ErrorCode::Io), "{error:?}");
        assert!(
            error
                .detail
                .as_deref()
                .unwrap_or("")
                .contains("unknown flag"),
            "{error:?}"
        );
    }

    #[tokio::test]
    async fn an_update_stops_after_a_failed_pull() {
        let dir = tempfile::tempdir().unwrap();
        let (cli, ran) = fake_cli(dir.path());
        let mut job = profile_job(ComposeAction::Update, &[], "");
        job.cli = cli;
        // An Engine that can't be reached fails every pull.
        let events = events_of(job).await;
        assert!(
            matches!(events.last(), Some(ComposeEvent::Error { error }) if error.message.contains("nginx:1.27")),
            "{events:?}"
        );
        assert!(!ran.exists(), "compose up never ran");
    }
}

#[test]
fn an_update_redeploys_only_with_what_it_pulled() {
    assert!(ComposeAction::Update.deploys() && ComposeAction::Update.reads_definition());
    assert_eq!(ComposeAction::Update.args(), ComposeAction::Up.args());
}

#[test]
fn tls_hosts_need_a_connection_for_the_proxy() {
    let target = StackTarget::Host(crate::runtime::Reach {
        name: "prod".into(),
        connection: HostConnection::Tls {
            address: "h:2376".into(),
        },
        dialer: None,
        docker: testing::offline(),
    });
    assert!(matches!(
        docker_host(&target),
        Err(ref e) if e.is(ErrorCode::HostNotConnected)
    ));
}

async fn compose(
    project: &str,
    action: ComposeAction,
    dialer: Option<Arc<dyn Dialer>>,
    connection: HostConnection,
) -> Vec<ComposeEvent> {
    let cli = detect().await.expect("docker compose is installed");
    let events = std::sync::Mutex::new(Vec::new());
    let mut job = job(
                StackSource::Managed {
                    files: vec![ComposeFile::new(
                        "compose.yaml",
                        format!(
                            "services:\n  app:\n    image: {}\n    command: [\"sleep\", \"60\"]\n    environment:\n      GREETING: ${{GREETING}}\n",
                            testing::IMAGE
                        ),
                    )],
                    env: "GREETING=hello".into(),
                },
                connection,
                dialer,
                testing::docker(),
                Logins::none(),
                project,
                action,
            );
    job.cli = cli;
    crate::runtime::run(job, |e| {
        events.lock().unwrap().push(e);
        true
    })
    .await;
    events.into_inner().unwrap()
}

async fn deploys_and_tears_down(dialer: Option<Arc<dyn Dialer>>, connection: HostConnection) {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let project = testing::unique("stack");

    let up = compose(
        &project,
        ComposeAction::Up,
        dialer.clone(),
        connection.clone(),
    )
    .await;
    assert!(
        matches!(up.last(), Some(ComposeEvent::Exit { success: true, .. })),
        "{up:?}"
    );
    let running = lifecycle::containers_of(&docker, &project).await.unwrap();
    assert_eq!(running.len(), 1);
    let env = &testing::docker()
        .inspect_container(&running[0].id, None)
        .await
        .unwrap()
        .config
        .unwrap()
        .env
        .unwrap();
    assert!(env.contains(&"GREETING=hello".to_string()));

    let down = compose(&project, ComposeAction::Down, dialer, connection).await;
    assert!(
        matches!(down.last(), Some(ComposeEvent::Exit { success: true, .. })),
        "{down:?}"
    );
    assert!(lifecycle::containers_of(&docker, &project)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and the compose CLI"]
async fn managed_stacks_deploy_on_local_hosts() {
    deploys_and_tears_down(
        None,
        HostConnection::Local {
            socket_path: "/var/run/docker.sock".into(),
        },
    )
    .await;
}

#[cfg(unix)]
#[tokio::test]
#[ignore = "needs a local Docker Engine and the compose CLI"]
async fn managed_stacks_deploy_through_the_proxy() {
    // TLS hosts always go through the proxy; the dialer stands in for the real one.
    deploys_and_tears_down(
        Some(Arc::new(UnixDialer)),
        HostConnection::Tls {
            address: "unused:2376".into(),
        },
    )
    .await;
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and the compose CLI"]
async fn managed_stack_pulls_go_through_the_engine_api() {
    let events = compose(
        &testing::unique("pull"),
        ComposeAction::Pull,
        None,
        HostConnection::Local {
            socket_path: "/var/run/docker.sock".into(),
        },
    )
    .await;
    let lines: Vec<&str> = events
        .iter()
        .filter_map(|e| match e {
            ComposeEvent::Line { text } => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(
        lines.first().copied(),
        Some(format!("Pulling {} anonymously", testing::IMAGE).as_str()),
        "{events:?}"
    );
    assert!(
        matches!(
            events.last(),
            Some(ComposeEvent::Exit { success: true, .. })
        ),
        "{events:?}"
    );
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and the compose CLI"]
async fn managed_stacks_merge_a_base_and_an_override() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let project = testing::unique("merged");
    let _cleanup = RemoveStack(project.clone());
    let base = format!(
            "services:\n  app:\n    image: {}\n    command: [\"sleep\", \"60\"]\n    environment:\n      GREETING: base\n",
            testing::IMAGE
        );
    let prod = "services:\n  app:\n    environment:\n      GREETING: prod\n";
    let mut job = job(
        StackSource::Managed {
            files: vec![
                ComposeFile::new("compose.yaml", base),
                ComposeFile::new("compose.prod.yaml", prod),
            ],
            env: String::new(),
        },
        HostConnection::Local {
            socket_path: "/var/run/docker.sock".into(),
        },
        None,
        testing::docker(),
        Logins::none(),
        &project,
        ComposeAction::Up,
    );
    job.cli = detect().await.expect("docker compose is installed");
    let events = std::sync::Mutex::new(Vec::new());
    crate::runtime::run(job, |e| {
        events.lock().unwrap().push(e);
        true
    })
    .await;
    let events = events.into_inner().unwrap();
    assert!(
        matches!(
            events.last(),
            Some(ComposeEvent::Exit { success: true, .. })
        ),
        "{events:?}"
    );
    let running = lifecycle::containers_of(&docker, &project).await.unwrap();
    let env = docker
        .inspect_container(&running[0].id, None)
        .await
        .unwrap()
        .config
        .unwrap()
        .env
        .unwrap();
    assert!(env.contains(&"GREETING=prod".to_string()), "{env:?}");
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and the compose CLI"]
async fn profiles_decide_which_services_run() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let project = testing::unique("profiles");
    let _cleanup = RemoveStack(project.clone());
    let compose = format!(
            "services:\n  app:\n    image: {0}\n    command: [\"sleep\", \"60\"]\n  debug:\n    image: {0}\n    command: [\"sleep\", \"60\"]\n    profiles: [debug]\n",
            testing::IMAGE
        );
    let run = |action, active: Vec<String>| {
        let mut job = job(
            StackSource::Managed {
                files: vec![ComposeFile::new("compose.yaml", compose.clone())],
                // Ignored: Homeport's own list wins.
                env: "COMPOSE_PROFILES=debug\n".into(),
            },
            HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            None,
            testing::docker(),
            Logins::none(),
            &project,
            action,
        );
        job.profiles = active;
        async move {
            job.cli = detect().await.expect("docker compose is installed");
            let events = std::sync::Mutex::new(Vec::new());
            crate::runtime::run(job, |e| {
                events.lock().unwrap().push(e);
                true
            })
            .await;
            let events = events.into_inner().unwrap();
            assert!(
                matches!(
                    events.last(),
                    Some(ComposeEvent::Exit { success: true, .. })
                ),
                "{events:?}"
            );
        }
    };
    let services = || async {
        let mut names: Vec<String> = lifecycle::containers_of(&testing::docker(), &project)
            .await
            .unwrap()
            .into_iter()
            .filter_map(|c| c.service)
            .collect();
        names.sort();
        names
    };

    run(ComposeAction::Up, vec![]).await;
    assert_eq!(services().await, ["app"]);
    run(ComposeAction::Up, vec!["debug".into()]).await;
    assert_eq!(services().await, ["app", "debug"]);
    // Turned off again: redeploying removes it as an orphan.
    run(ComposeAction::Up, vec![]).await;
    assert_eq!(services().await, ["app"]);
    run(ComposeAction::Up, vec!["debug".into()]).await;
    run(ComposeAction::Down, vec![]).await;
    assert!(
        services().await.is_empty(),
        "down removes every profile's services"
    );
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and the compose CLI"]
async fn deploys_are_recorded_and_rolled_back() {
    use homeport_hosts::store::{HostDraft, HostStore};
    use homeport_storage::{Db, SecretBox};

    use crate::{history::DeployHistory, runtime::Recorder, store::StackStore};

    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let project = testing::unique("history");
    let _cleanup = RemoveStack(project.clone());
    let db = Arc::new(Db::open_in_memory().unwrap());
    let secrets = Arc::new(SecretBox::from_key(&[4; 32]));
    let host = HostStore::new(db.clone(), secrets.clone())
        .create(HostDraft {
            name: "local".into(),
            connection: HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            engine: homeport_hosts::EngineKind::Docker,
            secrets: None,
        })
        .unwrap()
        .id;
    let stacks = StackStore::new(db.clone(), secrets.clone());
    let history = Arc::new(DeployHistory::new(db, secrets));
    let compose = format!(
            "services:\n  app:\n    image: {}\n    command: [\"sleep\", \"60\"]\n    environment:\n      GREETING: ${{GREETING}}\n",
            testing::IMAGE
        );
    let files = vec![ComposeFile::new("compose.yaml", compose)];
    let stack = stacks
        .create(&host, &project, &files, "GREETING=one")
        .unwrap();

    let deploy = || {
        let stack = stacks.get(&stack.id).unwrap();
        let mut job = job(
            StackSource::Managed {
                files: stack.files,
                env: stacks.env(&stack.id).unwrap(),
            },
            HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            None,
            testing::docker(),
            Logins::none(),
            &project,
            ComposeAction::Up,
        );
        job.record = Some(Recorder {
            history: history.clone(),
            owner: crate::StackOwner::Host(host.clone()),
            stack_id: stack.id.clone(),
            commit: None,
            automatic: false,
        });
        async move {
            job.cli = detect().await.expect("docker compose is installed");
            crate::runtime::run(job, |_| true).await;
        }
    };
    let greeting = || async {
        let running = lifecycle::containers_of(&testing::docker(), &project)
            .await
            .unwrap();
        testing::docker()
            .inspect_container(&running[0].id, None)
            .await
            .unwrap()
            .config
            .unwrap()
            .env
            .unwrap()
            .into_iter()
            .find(|e| e.starts_with("GREETING="))
            .unwrap()
    };

    deploy().await;
    stacks.update(&stack.id, &files, "GREETING=two").unwrap();
    deploy().await;
    assert_eq!(greeting().await, "GREETING=two");
    let deploys = history.list(&stack.id).unwrap();
    assert_eq!(deploys.len(), 2);
    assert!(deploys.iter().all(|d| d.success == Some(true)));

    crate::history::restore(&history, &stacks, &deploys[1].id).unwrap();
    deploy().await;
    assert_eq!(greeting().await, "GREETING=one");
    assert_eq!(history.list(&stack.id).unwrap().len(), 3);
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and the compose CLI"]
async fn updating_a_deployed_stack_pulls_then_redeploys() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let project = testing::unique("update");
    let _cleanup = RemoveStack(project.clone());
    let local = HostConnection::Local {
        socket_path: "/var/run/docker.sock".into(),
    };
    let up = compose(&project, ComposeAction::Up, None, local.clone()).await;
    assert!(
        matches!(up.last(), Some(ComposeEvent::Exit { success: true, .. })),
        "{up:?}"
    );
    let before = lifecycle::containers_of(&docker, &project).await.unwrap();

    let update = compose(&project, ComposeAction::Update, None, local).await;
    let lines: Vec<&str> = update
        .iter()
        .filter_map(|e| match e {
            ComposeEvent::Line { text } => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert!(
        matches!(
            update.last(),
            Some(ComposeEvent::Exit { success: true, .. })
        ),
        "{update:?}"
    );
    let pulled = lines
        .iter()
        .position(|l| l.starts_with("Pulling "))
        .unwrap();
    let between = lines
        .iter()
        .position(|l| l.starts_with("Images pulled."))
        .unwrap();
    assert!(pulled < between, "{lines:?}");
    // The image didn't change, so its container wasn't recreated.
    let after = lifecycle::containers_of(&docker, &project).await.unwrap();
    assert_eq!(before[0].id, after[0].id);
}

/// Deploys a folder with a build, an env_file and a relative bind mount
/// as a Linked Stack, checks all three reached the container, then takes
/// it down.
async fn linked_folder_deploys(dialer: Option<Arc<dyn Dialer>>, connection: HostConnection) {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let project = testing::unique("linked");
    let _cleanup = RemoveStack(project.clone());
    let dir = tempfile::tempdir().unwrap();
    let write = |name: &str, text: String| std::fs::write(dir.path().join(name), text).unwrap();
    write(
        "Dockerfile",
        format!("FROM {}\nCMD [\"sleep\", \"60\"]\n", testing::IMAGE),
    );
    write(".env", "GREETING=hello\n".into());
    std::fs::create_dir(dir.path().join("data")).unwrap();
    write(
        "compose.yaml",
        "services:\n  app:\n    build: .\n    env_file: .env\n    volumes:\n      - ./data:/data\n"
            .into(),
    );
    let source = StackSource::Linked {
        working_dir: dir.path().to_owned(),
        config_files: vec![dir
            .path()
            .join("compose.yaml")
            .to_string_lossy()
            .into_owned()],
    };
    let run = |action| {
        let mut job = job(
            source.clone(),
            connection.clone(),
            dialer.clone(),
            testing::docker(),
            Logins::none(),
            &project,
            action,
        );
        async move {
            job.cli = detect().await.expect("docker compose is installed");
            let events = std::sync::Mutex::new(Vec::new());
            crate::runtime::run(job, |e| {
                events.lock().unwrap().push(e);
                true
            })
            .await;
            events.into_inner().unwrap()
        }
    };

    let up = run(ComposeAction::Up).await;
    assert!(
        matches!(up.last(), Some(ComposeEvent::Exit { success: true, .. })),
        "{up:?}"
    );
    let running = lifecycle::containers_of(&docker, &project).await.unwrap();
    assert_eq!(running.len(), 1);
    let inspect = docker
        .inspect_container(&running[0].id, None)
        .await
        .unwrap();
    let env = inspect.config.unwrap().env.unwrap();
    assert!(env.contains(&"GREETING=hello".to_string()), "{env:?}");
    let mount = inspect.mounts.unwrap()[0].source.clone().unwrap();
    assert_eq!(
        std::fs::canonicalize(mount).unwrap(),
        std::fs::canonicalize(dir.path().join("data")).unwrap()
    );

    let down = run(ComposeAction::Down).await;
    assert!(
        matches!(down.last(), Some(ComposeEvent::Exit { success: true, .. })),
        "{down:?}"
    );
    let _ = docker
        .remove_image(
            &format!("{project}-app"),
            None::<bollard::query_parameters::RemoveImageOptions>,
            None,
        )
        .await;
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and the compose CLI"]
async fn linked_stacks_build_and_deploy_from_their_folder() {
    linked_folder_deploys(
        None,
        HostConnection::Local {
            socket_path: "/var/run/docker.sock".into(),
        },
    )
    .await;
}

#[cfg(unix)]
#[tokio::test]
#[ignore = "needs a local Docker Engine and the compose CLI"]
async fn linked_stacks_build_and_deploy_through_the_proxy() {
    linked_folder_deploys(
        Some(Arc::new(UnixDialer)),
        HostConnection::Tls {
            address: "unused:2376".into(),
        },
    )
    .await;
}

/// Removes a Stack's containers and networks when dropped, even if the test panics.
struct RemoveStack(String);

impl Drop for RemoveStack {
    fn drop(&mut self) {
        let project = self.0.clone();
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    let _ = lifecycle::remove(&testing::docker(), &project).await;
                })
        })
        .join()
        .ok();
    }
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn external_stacks_can_be_stopped_and_removed_without_compose_files() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let project = testing::unique("ext");
    let _cleanup = RemoveStack(project.clone());
    let up = compose(
        &project,
        ComposeAction::Up,
        None,
        HostConnection::Local {
            socket_path: "/var/run/docker.sock".into(),
        },
    )
    .await;
    assert!(matches!(
        up.last(),
        Some(ComposeEvent::Exit { success: true, .. })
    ));

    lifecycle::act(&docker, &project, lifecycle::StackAction::Stop)
        .await
        .unwrap();
    // The Engine may list a container a moment before it reports it exited.
    let mut stopped = Vec::new();
    for _ in 0..50 {
        stopped = lifecycle::containers_of(&docker, &project).await.unwrap();
        if stopped.iter().all(|c| c.state == "exited") {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    let states: Vec<(&str, &str)> = stopped
        .iter()
        .map(|c| (c.name.as_str(), c.state.as_str()))
        .collect();
    assert!(
        !stopped.is_empty() && stopped.iter().all(|c| c.state == "exited"),
        "{states:?}"
    );
    lifecycle::remove(&docker, &project).await.unwrap();
    assert!(lifecycle::containers_of(&docker, &project)
        .await
        .unwrap()
        .is_empty());
}

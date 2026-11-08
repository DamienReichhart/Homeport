//! Deploying a Stack to a Swarm: `docker stack deploy` on this machine,
//! against a connected manager, through the same per-command proxy as
//! compose (ADRs 0001 and 0017). The Engine has no stack endpoint: the CLI
//! turns the compose file into services, networks, secrets and configs.
//! Taking a swarm stack down needs no CLI; it goes through the Engine API.

use std::{collections::VecDeque, process::Stdio};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    programs,
};

use super::{
    docker_host, errors, write_stdin, ComposeAction, ComposeCli, ComposeEvent, STDERR_TAIL,
};
use crate::{
    definition, messages as msg,
    runtime::{ComposeJob, Prepared, Recorder, StackTarget, WorkDir},
    swarm,
};

/// The `docker` CLI on this machine, for `docker stack deploy`.
pub async fn detect() -> AppResult<ComposeCli> {
    let missing = || {
        AppError::new(ErrorCode::DockerCliMissing, msg::DOCKER_CLI_MISSING)
            .with_hint(msg::DOCKER_CLI_MISSING_HINT)
    };
    let program = programs::find("docker").ok_or_else(missing)?;
    let out = Command::new(&program)
        .env("PATH", programs::search_path())
        .args(["version", "--format", "{{.Client.Version}}"])
        .stdin(Stdio::null())
        .output()
        .await
        .map_err(|_| missing())?;
    let version = String::from_utf8_lossy(&out.stdout).trim().to_owned();
    if version.is_empty() {
        return Err(missing());
    }
    Ok(ComposeCli {
        program: program.to_string_lossy().into_owned(),
        prefix: vec![],
        version,
    })
}

/// Whether this CLI's `stack deploy` can wait for the services to converge.
async fn waits_for_services(cli: &ComposeCli) -> bool {
    let out = Command::new(&cli.program)
        .env("PATH", programs::search_path())
        .args(["stack", "deploy", "--help"])
        .stdin(Stdio::null())
        .output()
        .await;
    out.is_ok_and(|o| String::from_utf8_lossy(&o.stdout).contains("--detach"))
}

/// `--compose-file` arguments from the compose CLI's, without its
/// `--project-directory` (the run's working directory is the folder).
pub(crate) fn compose_file_args(prepared: &Prepared) -> Vec<String> {
    let mut args = Vec::new();
    let mut it = prepared.file_args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--file" => {
                if let Some(file) = it.next() {
                    args.extend(["--compose-file".to_owned(), file.clone()]);
                }
            }
            "--project-directory" => {
                it.next();
            }
            _ => {}
        }
    }
    args
}

/// The variables of a run: a folder's `.env` (which `docker stack deploy`,
/// unlike compose, doesn't read), then the Stack's own, which win.
fn variables(prepared: &Prepared) -> AppResult<Vec<(String, String)>> {
    let mut vars = Vec::new();
    if let WorkDir::Folder(folder) = &prepared.workdir {
        if let Ok(text) = std::fs::read_to_string(folder.join(".env")) {
            vars.extend(definition::parse_env(&text)?);
        }
    }
    vars.extend(prepared.vars.iter().cloned());
    Ok(vars)
}

/// The arguments of `docker stack deploy` for `job`, before the Stack's name.
pub(crate) fn deploy_args(job: &ComposeJob, prepared: &Prepared, waits: bool) -> Vec<String> {
    let mut args = vec!["stack".to_owned(), "deploy".to_owned()];
    args.extend(compose_file_args(prepared));
    args.push("--prune".into());
    if waits {
        args.push("--detach=false".into());
    }
    if job.registry_auth {
        args.push("--with-registry-auth".into());
    }
    args.push(job.project.clone());
    args
}

/// Why `docker stack deploy` failed, from the end of its stderr.
fn failure(job: &ComposeJob, code: Option<i32>, tail: &[String]) -> AppError {
    let text = tail.join("\n").to_lowercase();
    if text.contains("only updates to labels are allowed") {
        return AppError::new(ErrorCode::ComposeFailed, msg::SWARM_OBJECT_CHANGED)
            .with_hint(msg::SWARM_OBJECT_CHANGED_HINT)
            .with_detail(tail.join("\n"));
    }
    errors::exit_failure(job.target.name(), "stack deploy", code, tail)
}

/// Runs one job on a Swarm, streaming what the CLI prints to `sink`.
pub(crate) async fn execute(
    job: ComposeJob,
    sink: &impl Fn(ComposeEvent) -> bool,
    recorder: Option<&Recorder>,
    recorded: &mut Option<String>,
) -> AppResult<()> {
    definition::validate_name(&job.project)?;
    let StackTarget::Swarm { manager, .. } = &job.target else {
        unreachable!("only swarm targets come here");
    };
    match job.action {
        ComposeAction::Pull => return Err(AppError::validation(msg::SWARM_NO_PULL)),
        ComposeAction::Down | ComposeAction::DownWithVolumes => {
            let report = swarm::remove(&manager.docker, &job.project).await?;
            for kept in &report.kept {
                if !sink(ComposeEvent::Line {
                    text: format!("Kept {kept}"),
                }) {
                    return Ok(());
                }
            }
            sink(ComposeEvent::Exit {
                success: report.kept.is_empty(),
                code: Some(i32::from(!report.kept.is_empty())),
                error: None,
            });
            return Ok(());
        }
        ComposeAction::Up | ComposeAction::Rebuild | ComposeAction::Update => {}
    }

    let prepared = job.source.prepare_for(job.action, true)?;
    let vars = variables(&prepared)?;
    // Keep the proxy alive until the CLI exits.
    let (docker_host, _proxy) = docker_host(&job.target)?;
    if let Some(recorder) = recorder {
        *recorded = recorder.start(&job, &prepared);
    }
    if !job.profiles.is_empty()
        && !sink(ComposeEvent::Line {
            text:
                "Warning: docker stack deploy ignores Compose profiles; every service is deployed."
                    .into(),
        })
    {
        return Ok(());
    }
    let waits = waits_for_services(&job.cli).await;
    let mut child = Command::new(&job.cli.program)
        .args(deploy_args(&job, &prepared, waits))
        .current_dir(prepared.workdir.path())
        .env_remove("DOCKER_CONTEXT")
        .env("DOCKER_HOST", &docker_host)
        .env("PATH", programs::search_path())
        .envs(vars)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| errors::spawn_failed(&job.cli.program, &e))?;
    write_stdin(&mut child, &prepared).await?;

    let mut stdout = BufReader::new(child.stdout.take().expect("stdout is piped")).lines();
    let mut stderr = BufReader::new(child.stderr.take().expect("stderr is piped")).lines();
    let (mut out_done, mut err_done) = (false, false);
    let mut tail = VecDeque::with_capacity(STDERR_TAIL);
    while !(out_done && err_done) {
        let line = tokio::select! {
            l = stdout.next_line(), if !out_done => l?.or_else(|| { out_done = true; None }),
            l = stderr.next_line(), if !err_done => l?.inspect(|text| {
                if tail.len() == STDERR_TAIL {
                    tail.pop_front();
                }
                tail.push_back(text.clone());
            }).or_else(|| { err_done = true; None }),
        };
        if let Some(text) = line {
            if !sink(ComposeEvent::Line { text }) {
                return Ok(());
            }
        }
    }
    let status = child.wait().await?;
    if let (Some(recorder), Some(id)) = (recorder, recorded.as_ref()) {
        recorder.finish(id, status.success(), status.code());
    }
    let error = (!status.success())
        .then(|| failure(&job, status.code(), tail.make_contiguous()).logged("stack deploy"));
    sink(ComposeEvent::Exit {
        success: status.success(),
        code: status.code(),
        error,
    });
    Ok(())
}

#[cfg(test)]
mod tests {
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
}

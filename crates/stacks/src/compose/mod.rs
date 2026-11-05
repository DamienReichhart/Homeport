//! Running the local `docker compose` CLI for a [`ComposeJob`] (ADR 0001).
//!
//! The job's source decides the compose files, working directory and stdin
//! (a Managed Stack's YAML goes in on stdin and its environment as process
//! variables, so nothing secret is written to disk); its target decides
//! `DOCKER_HOST`. TLS and SSH Hosts are reached through a [`proxy`] that
//! exists only while the command runs. Images are pulled through the Engine
//! API with the stored registry logins first, so the CLI never needs one
//! (ADR 0010).

mod error;
mod images;

use std::{collections::VecDeque, process::Stdio};

use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;
use homeport_core::{host::HostConnection, programs};
use homeport_transport::proxy;

use self::error as errors;
use crate::{
    definition,
    runtime::{ComposeJob, Prepared, StackTarget},
};

/// Lines of stderr kept to explain a failed run.
const STDERR_TAIL: usize = 40;

/// How to invoke Compose on this machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ComposeCli {
    pub program: String,
    /// `["compose"]` for the Docker CLI plugin, empty for standalone `docker-compose`.
    pub prefix: Vec<String>,
    pub version: String,
}

/// Finds `docker compose`, falling back to a standalone `docker-compose` v2,
/// and says what to install when neither is usable.
pub async fn detect() -> AppResult<ComposeCli> {
    let docker = programs::find("docker");
    if let Some(docker) = &docker {
        if let Some(version) = version_of(docker, &["compose"]).await {
            return Ok(ComposeCli {
                program: docker.to_string_lossy().into_owned(),
                prefix: vec!["compose".into()],
                version,
            });
        }
    }
    let standalone = programs::find("docker-compose");
    let version = match &standalone {
        Some(program) => version_of(program, &[]).await,
        None => None,
    };
    match (standalone, version) {
        (Some(program), Some(version)) if !is_v1(&version) => Ok(ComposeCli {
            program: program.to_string_lossy().into_owned(),
            prefix: vec![],
            version,
        }),
        (_, old) => Err(errors::missing(docker.is_some(), old.as_deref())),
    }
}

fn is_v1(version: &str) -> bool {
    version.trim_start_matches('v').starts_with("1.")
}

/// What `program … version --short` prints, if it runs and succeeds.
async fn version_of(program: &std::path::Path, prefix: &[&str]) -> Option<String> {
    let out = Command::new(program)
        .env("PATH", programs::search_path())
        .args(prefix)
        .args(["version", "--short"])
        .stdin(Stdio::null())
        .output()
        .await
        .ok()?;
    let version = String::from_utf8_lossy(&out.stdout).trim().to_owned();
    (out.status.success() && !version.is_empty()).then_some(version)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ComposeAction {
    /// `up -d --remove-orphans`: deploy or redeploy, after pulling the
    /// images the Host lacks through the Engine API.
    Up,
    /// Pulls every service's image through the Engine API; compose isn't run.
    Pull,
    /// `down --remove-orphans`, keeping volumes.
    Down,
    /// `down -v --remove-orphans`: also deletes the Stack's volumes.
    DownWithVolumes,
}

impl ComposeAction {
    /// Whether the action needs a definition that can be deployed; taking a
    /// Stack down only needs its project name and files.
    pub fn reads_definition(self) -> bool {
        matches!(self, Self::Up | Self::Pull)
    }

    /// The compose subcommand, for messages.
    fn verb(self) -> &'static str {
        self.args()[0]
    }

    fn args(self) -> &'static [&'static str] {
        match self {
            Self::Up => &["up", "--detach", "--remove-orphans"],
            Self::Pull => &["pull"],
            Self::Down => &["down", "--remove-orphans"],
            Self::DownWithVolumes => &["down", "--volumes", "--remove-orphans"],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ComposeEvent {
    Line {
        text: String,
    },
    /// `error` explains a failed exit, from the end of stderr.
    Exit {
        success: bool,
        code: Option<i32>,
        error: Option<AppError>,
    },
    Error {
        error: AppError,
    },
}

/// The `DOCKER_HOST` for hosts the CLI can reach directly; `None` for TLS
/// and SSH, which need the proxy.
pub fn direct_docker_host(connection: &HostConnection) -> Option<String> {
    match connection {
        HostConnection::Local { socket_path } if cfg!(windows) => {
            Some(format!("npipe://{}", socket_path.replace('\\', "/")))
        }
        HostConnection::Local { socket_path } => Some(format!("unix://{socket_path}")),
        HostConnection::Tcp { address } => Some(format!("tcp://{address}")),
        HostConnection::Tls { .. } | HostConnection::Ssh { .. } => None,
    }
}

/// Runs one compose command, streaming its output lines to `sink`.
pub(crate) async fn run(job: ComposeJob, sink: impl Fn(ComposeEvent) -> bool) {
    if let Err(e) = execute(job, &sink).await {
        sink(ComposeEvent::Error {
            error: e.logged("compose run"),
        });
    }
}

/// The `DOCKER_HOST` the CLI uses for `target`, and the proxy that must stay
/// open until it exits, for TLS and SSH Hosts.
fn docker_host(target: &StackTarget) -> AppResult<(String, Option<proxy::Proxy>)> {
    let StackTarget::Host {
        name,
        connection,
        dialer,
        ..
    } = target;
    if let Some(host) = direct_docker_host(connection) {
        return Ok((host, None));
    }
    let dialer = dialer.clone().ok_or_else(|| {
        AppError::new(
            ErrorCode::HostNotConnected,
            msg::HOST_NOT_CONNECTED.with("name", name),
        )
        .with_hint(msg::HOST_NOT_CONNECTED_HINT)
    })?;
    let proxy = proxy::open(dialer).map_err(|e| {
        AppError::new(ErrorCode::Io, msg::PROXY_FAILED.with("name", name)).with_cause(&e)
    })?;
    Ok((proxy.docker_host.clone(), Some(proxy)))
}

async fn execute(job: ComposeJob, sink: &impl Fn(ComposeEvent) -> bool) -> AppResult<()> {
    definition::validate_name(&job.project)?;
    let prepared = job.source.prepare(job.action)?;
    // Keep the proxy alive until the CLI exits.
    let (docker_host, _proxy) = docker_host(&job.target)?;
    let cli = |args: &[&str]| cli_command(&job, &prepared, &docker_host, args);
    let docker = job.target.docker();

    let mut args: Vec<&str> = job.action.args().to_vec();
    if job.action.reads_definition() {
        let config = config_json(&job, &prepared, cli(&["config", "--format", "json"])).await?;
        let wanted = images::needed(docker, images::from_config(&config)?, job.action).await;
        if !images::pull(docker, &wanted, &job.logins, sink).await? {
            return Ok(());
        }
        if job.action == ComposeAction::Pull {
            sink(ComposeEvent::Exit {
                success: true,
                code: Some(0),
                error: None,
            });
            return Ok(());
        }
        // Pulled above; compose mustn't pull them again with no login.
        if wanted.iter().any(|i| i.always) {
            args.extend(["--pull", "never"]);
        }
    }

    let mut child = cli(&args)
        .spawn()
        .map_err(|e| errors::spawn_failed(&job.cli.program, &e))?;
    write_stdin(&mut child, &prepared).await?;

    let mut stdout = BufReader::new(child.stdout.take().expect("stdout is piped")).lines();
    let mut stderr = BufReader::new(child.stderr.take().expect("stderr is piped")).lines();
    let (mut out_done, mut err_done) = (false, false);
    // The end of stderr explains a failure.
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
    let error = (!status.success()).then(|| {
        errors::exit_failure(
            job.target.name(),
            job.action.verb(),
            status.code(),
            tail.make_contiguous(),
        )
        .logged("compose run")
    });
    sink(ComposeEvent::Exit {
        success: status.success(),
        code: status.code(),
        error,
    });
    Ok(())
}

/// Hands the source's stdin to the CLI, if it has one, and closes it.
async fn write_stdin(child: &mut tokio::process::Child, prepared: &Prepared) -> AppResult<()> {
    let mut stdin = child.stdin.take().expect("stdin is piped");
    if let Some(text) = &prepared.stdin {
        stdin.write_all(text.as_bytes()).await?;
    }
    drop(stdin);
    Ok(())
}

/// One run of the CLI for `job`. Its environment is the Host's address, the
/// `PATH` and the Stack's own variables: never a registry login.
fn cli_command(job: &ComposeJob, prepared: &Prepared, docker_host: &str, args: &[&str]) -> Command {
    let mut command = Command::new(&job.cli.program);
    command
        .args(&job.cli.prefix)
        .args(["--project-name", &job.project])
        .args(&prepared.file_args)
        .args(["--progress", "plain"])
        .args(args)
        .current_dir(prepared.workdir.path())
        .env_remove("DOCKER_CONTEXT")
        .env_remove("COMPOSE_FILE")
        .env_remove("COMPOSE_PROJECT_NAME")
        .env("DOCKER_HOST", docker_host)
        // Lets the CLI find its plugins and credential helpers from a GUI launch.
        .env("PATH", programs::search_path())
        .envs(prepared.vars.iter().cloned())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    command
}

/// The Stack's resolved model from `config --format json`: variables
/// substituted, defaults filled in. Runs on this machine only.
async fn config_json(
    job: &ComposeJob,
    prepared: &Prepared,
    mut command: Command,
) -> AppResult<String> {
    let mut child = command
        .spawn()
        .map_err(|e| errors::spawn_failed(&job.cli.program, &e))?;
    write_stdin(&mut child, prepared).await?;
    let out = child.wait_with_output().await?;
    if !out.status.success() {
        let stderr: Vec<String> = String::from_utf8_lossy(&out.stderr)
            .lines()
            .map(str::to_owned)
            .collect();
        let tail = &stderr[stderr.len().saturating_sub(STDERR_TAIL)..];
        return Err(errors::exit_failure(
            job.target.name(),
            "config",
            out.status.code(),
            tail,
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use homeport_engine::{auth::Logins, testing};
    #[cfg(unix)]
    use homeport_transport::testing::UnixDialer;
    use homeport_transport::Dialer;

    use crate::{lifecycle, runtime::StackSource};

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
            target: StackTarget::Host {
                name: "local".into(),
                connection,
                dialer,
                docker,
            },
            logins,
            action,
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
                compose: String::new(),
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
                compose: "services:\n  web:\n    image: nginx\n".into(),
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

    #[test]
    fn tls_hosts_need_a_connection_for_the_proxy() {
        let target = StackTarget::Host {
            name: "prod".into(),
            connection: HostConnection::Tls {
                address: "h:2376".into(),
            },
            dialer: None,
            docker: testing::offline(),
        };
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
                    compose: format!(
                        "services:\n  app:\n    image: {}\n    command: [\"sleep\", \"60\"]\n    environment:\n      GREETING: ${{GREETING}}\n",
                        testing::IMAGE
                    ),
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
}

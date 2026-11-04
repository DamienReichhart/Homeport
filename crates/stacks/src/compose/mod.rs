//! Deploying Managed Stacks with the local `docker compose` CLI (ADR 0001).
//!
//! The YAML goes in on stdin and the environment as process variables, so
//! nothing secret is written to disk. TLS and SSH Hosts are reached through
//! a [`proxy`] that exists only while the command runs. Images are pulled
//! through the Engine API with the stored registry logins first, so the CLI
//! never needs one (ADR 0010).

mod error;
mod images;

use std::{collections::VecDeque, process::Stdio, sync::Arc};

use bollard::Docker;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_core::{host::HostConnection, programs};
use homeport_engine::auth::Logins;
use homeport_transport::{proxy, Dialer};

use self::error as errors;
use crate::definition;

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

pub struct ComposeRun {
    pub cli: ComposeCli,
    /// The Host's name, for messages.
    pub host_name: String,
    pub connection: HostConnection,
    pub dialer: Option<Arc<dyn Dialer>>,
    /// The Host's Engine, for pulling images with `logins`.
    pub docker: Docker,
    pub logins: Logins,
    pub project: String,
    pub compose: String,
    pub env: String,
    pub action: ComposeAction,
}

/// Runs one compose command, streaming its output lines to `sink`.
pub async fn run(job: ComposeRun, sink: impl Fn(ComposeEvent) -> bool) {
    if let Err(e) = execute(job, &sink).await {
        sink(ComposeEvent::Error {
            error: e.logged("compose run"),
        });
    }
}

async fn execute(job: ComposeRun, sink: &impl Fn(ComposeEvent) -> bool) -> AppResult<()> {
    definition::validate_name(&job.project).map_err(AppError::validation)?;
    if job.action != ComposeAction::Down && job.action != ComposeAction::DownWithVolumes {
        let check = definition::check(&job.compose);
        if !check.is_deployable() {
            return Err(AppError::validation(check.errors.join(" ")));
        }
    }
    let vars = definition::parse_env(&job.env).map_err(AppError::validation)?;

    // Keep the proxy alive until the CLI exits.
    let (docker_host, _proxy) = match direct_docker_host(&job.connection) {
        Some(host) => (host, None),
        None => {
            let dialer = job.dialer.clone().ok_or_else(|| {
                AppError::new(
                    ErrorCode::HostNotConnected,
                    format!("{} isn't connected.", job.host_name),
                )
                .with_hint("Wait for it to connect, then try again.")
            })?;
            let proxy = proxy::open(dialer).map_err(|e| {
                AppError::new(
                    ErrorCode::Io,
                    format!(
                        "Homeport couldn't open the private connection compose uses to reach {}.",
                        job.host_name
                    ),
                )
                .with_cause(&e)
            })?;
            (proxy.docker_host.clone(), Some(proxy))
        }
    };
    // An empty working directory so nothing on this machine is picked up as project files.
    let workdir = tempfile::tempdir()?;
    let cli = |args: &[&str]| cli_command(&job, &vars, &docker_host, workdir.path(), args);

    let mut args: Vec<&str> = job.action.args().to_vec();
    if matches!(job.action, ComposeAction::Up | ComposeAction::Pull) {
        let config = config_json(&job, cli(&["config", "--format", "json"])).await?;
        let wanted = images::needed(&job.docker, images::from_config(&config)?, job.action).await;
        if !images::pull(&job.docker, &wanted, &job.logins, sink).await? {
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

    let mut stdin = child.stdin.take().expect("stdin is piped");
    stdin.write_all(job.compose.as_bytes()).await?;
    drop(stdin);

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
            &job.host_name,
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

/// One run of the CLI for `job`. Its environment is the Host's address, the
/// `PATH` and the Stack's own variables: never a registry login.
fn cli_command(
    job: &ComposeRun,
    vars: &[(String, String)],
    docker_host: &str,
    workdir: &std::path::Path,
    args: &[&str],
) -> Command {
    let mut command = Command::new(&job.cli.program);
    command
        .args(&job.cli.prefix)
        .args([
            "--project-name",
            &job.project,
            "--file",
            "-",
            "--progress",
            "plain",
        ])
        .args(args)
        .current_dir(workdir)
        .env_remove("DOCKER_CONTEXT")
        .env_remove("COMPOSE_FILE")
        .env_remove("COMPOSE_PROJECT_NAME")
        .env("DOCKER_HOST", docker_host)
        // Lets the CLI find its plugins and credential helpers from a GUI launch.
        .env("PATH", programs::search_path())
        .envs(vars.iter().cloned())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    command
}

/// The Stack's resolved model from `config --format json`: variables
/// substituted, defaults filled in. Runs on this machine only.
async fn config_json(job: &ComposeRun, mut command: Command) -> AppResult<String> {
    let mut child = command
        .spawn()
        .map_err(|e| errors::spawn_failed(&job.cli.program, &e))?;
    let mut stdin = child.stdin.take().expect("stdin is piped");
    stdin.write_all(job.compose.as_bytes()).await?;
    drop(stdin);
    let out = child.wait_with_output().await?;
    if !out.status.success() {
        let stderr: Vec<String> = String::from_utf8_lossy(&out.stderr)
            .lines()
            .map(str::to_owned)
            .collect();
        let tail = &stderr[stderr.len().saturating_sub(STDERR_TAIL)..];
        return Err(errors::exit_failure(
            &job.host_name,
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
    use std::path::Path;

    use homeport_engine::testing;
    use homeport_transport::testing::UnixDialer;

    use crate::lifecycle;

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

    #[test]
    fn the_clis_environment_holds_no_registry_login() {
        let job = ComposeRun {
            cli: ComposeCli {
                program: "docker".into(),
                prefix: vec!["compose".into()],
                version: "2.29.0".into(),
            },
            host_name: "local".into(),
            connection: HostConnection::Tcp {
                address: "h:2375".into(),
            },
            dialer: None,
            docker: testing::docker(),
            logins: Logins::new(
                vec![homeport_engine::auth::RegistryAuth {
                    address: "ghcr.io".into(),
                    username: "alice".into(),
                    secret: "ghp_TOPSECRET".into(),
                }],
                |_| "ghcr.io".into(),
            ),
            project: "shop".into(),
            compose: String::new(),
            env: String::new(),
            action: ComposeAction::Up,
        };
        let vars = vec![("GREETING".to_owned(), "hello".to_owned())];
        let command = cli_command(&job, &vars, "tcp://h:2375", Path::new("/tmp"), &["up"]);
        let std = command.as_std();
        let everything: Vec<String> = std
            .get_envs()
            .filter_map(|(k, v)| Some(format!("{}={}", k.to_str()?, v?.to_str()?)))
            .chain(std.get_args().filter_map(|a| a.to_str().map(str::to_owned)))
            .collect();
        assert!(everything.contains(&"DOCKER_HOST=tcp://h:2375".to_owned()));
        assert!(everything.contains(&"GREETING=hello".to_owned()));
        assert!(
            !everything
                .iter()
                .any(|e| e.contains("ghp_TOPSECRET") || e.contains("DOCKER_CONFIG")),
            "{everything:?}"
        );
    }

    async fn compose(
        project: &str,
        action: ComposeAction,
        dialer: Option<Arc<dyn Dialer>>,
        connection: HostConnection,
    ) -> Vec<ComposeEvent> {
        let cli = detect().await.expect("docker compose is installed");
        let events = std::sync::Mutex::new(Vec::new());
        run(
            ComposeRun {
                cli,
                host_name: "local".into(),
                connection,
                dialer,
                docker: testing::docker(),
                logins: Logins::none(),
                project: project.into(),
                compose: format!(
                    "services:\n  app:\n    image: {}\n    command: [\"sleep\", \"60\"]\n    environment:\n      GREETING: ${{GREETING}}\n",
                    testing::IMAGE
                ),
                env: "GREETING=hello".into(),
                action,
            },
            |e| {
                events.lock().unwrap().push(e);
                true
            },
        )
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
            matches!(events.last(), Some(ComposeEvent::Exit { success: true, .. })),
            "{events:?}"
        );
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn external_stacks_can_be_stopped_and_removed_without_compose_files() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let project = testing::unique("ext");
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
        let stopped = lifecycle::containers_of(&docker, &project).await.unwrap();
        assert!(stopped.iter().all(|c| c.state == "exited"));
        lifecycle::remove(&docker, &project).await.unwrap();
        assert!(lifecycle::containers_of(&docker, &project)
            .await
            .unwrap()
            .is_empty());
    }
}

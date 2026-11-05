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
    definition, lifecycle,
    runtime::{ComposeJob, Prepared, Recorder, StackTarget, WorkDir},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ComposeAction {
    /// `up -d --remove-orphans`: deploy or redeploy, after pulling the
    /// images the Host lacks through the Engine API. Builds what has no
    /// image yet.
    Up,
    /// `up -d --remove-orphans --build`: also rebuilds every service with
    /// `build:` first.
    Rebuild,
    /// Pulls every service's image through the Engine API, then `up` with
    /// what was pulled, so only services whose image changed are recreated.
    /// Stops after a failed pull, leaving the running containers alone.
    Update,
    /// Pulls every service's image through the Engine API; compose isn't run.
    Pull,
    /// `down --remove-orphans`, keeping volumes.
    Down,
    /// `down -v --remove-orphans`: also deletes the Stack's volumes.
    DownWithVolumes,
}

impl ComposeAction {
    /// Whether the action deploys, so it's recorded in the Stack's history.
    pub fn deploys(self) -> bool {
        matches!(self, Self::Up | Self::Rebuild | Self::Update)
    }

    /// Whether the action needs a definition that can be deployed; taking a
    /// Stack down only needs its project name and files.
    pub fn reads_definition(self) -> bool {
        matches!(self, Self::Up | Self::Rebuild | Self::Update | Self::Pull)
    }

    /// The compose subcommand, for messages.
    fn verb(self) -> &'static str {
        self.args()[0]
    }

    fn args(self) -> &'static [&'static str] {
        match self {
            Self::Up | Self::Update => &["up", "--detach", "--remove-orphans"],
            Self::Rebuild => &["up", "--detach", "--remove-orphans", "--build"],
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

/// Runs one compose command, streaming its output lines to `sink`. A deploy
/// with a recorder is recorded when it starts and finished when it ends; a
/// run nobody listens to any more stays unfinished.
pub(crate) async fn run(mut job: ComposeJob, sink: impl Fn(ComposeEvent) -> bool) {
    let recorder = job.record.take();
    let mut recorded = None;
    let result = execute(job, &sink, recorder.as_ref(), &mut recorded).await;
    if let Err(e) = result {
        if let (Some(recorder), Some(id)) = (&recorder, &recorded) {
            recorder.finish(id, false, None);
        }
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

async fn execute(
    job: ComposeJob,
    sink: &impl Fn(ComposeEvent) -> bool,
    recorder: Option<&Recorder>,
    recorded: &mut Option<String>,
) -> AppResult<()> {
    definition::validate_name(&job.project)?;
    let prepared = job.source.prepare(job.action)?;
    // Keep the proxy alive until the CLI exits.
    let (docker_host, _proxy) = docker_host(&job.target)?;
    if let Some(recorder) = recorder.filter(|_| job.action.deploys()) {
        *recorded = recorder.start(&job, &prepared);
    }
    let cli = |args: &[&str]| cli_command(&job, &prepared, &docker_host, args);
    let docker = job.target.docker();
    if !job.target.is_local() {
        for warning in remote_path_warnings(&prepared, &job) {
            if !sink(ComposeEvent::Line { text: warning }) {
                return Ok(());
            }
        }
    }

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
        if job.action == ComposeAction::Update
            && !sink(ComposeEvent::Line {
                text: "Images pulled. Recreating the services whose image changed.".into(),
            })
        {
            return Ok(());
        }
        // Pulled above; compose mustn't pull them again with no login.
        if job.action == ComposeAction::Update || wanted.iter().any(|i| i.always) {
            args.extend(["--pull", "never"]);
        }
    }

    if job.action.reads_definition()
        && !remove_turned_off(&job, &prepared, &docker_host, sink).await?
    {
        return Ok(());
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
    if let (Some(recorder), Some(id)) = (recorder, recorded.as_ref()) {
        recorder.finish(id, status.success(), status.code());
    }
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

/// For a Linked Stack deployed to another machine: the relative bind mounts,
/// which resolve to this machine's folder, a path the Host may not have.
fn remote_path_warnings(prepared: &Prepared, job: &ComposeJob) -> Vec<String> {
    let WorkDir::Folder(folder) = &prepared.workdir else {
        return vec![];
    };
    let yaml: Vec<&str> = prepared
        .definition
        .iter()
        .map(|(_, t)| t.as_str())
        .collect();
    definition::relative_binds(&yaml)
        .into_iter()
        .filter(|(_, source)| source.starts_with('.'))
        .map(|(service, source)| {
            let resolved = folder.join(source.trim_start_matches("./"));
            format!(
                "Warning: service \"{service}\" mounts \"{source}\", which is {} on this computer. {} may not have that path.",
                resolved.display(),
                job.target.name()
            )
        })
        .collect()
}

/// Removes the containers of services whose profiles are all turned off:
/// compose doesn't count them as orphans, so `up --remove-orphans` would
/// leave them running. `Ok(false)` when the viewer went away.
async fn remove_turned_off(
    job: &ComposeJob,
    prepared: &Prepared,
    docker_host: &str,
    sink: &impl Fn(ComposeEvent) -> bool,
) -> AppResult<bool> {
    let yaml: Vec<&str> = prepared
        .definition
        .iter()
        .map(|(_, t)| t.as_str())
        .collect();
    let off = definition::services_turned_off(&yaml, &job.profiles);
    if off.is_empty() {
        return Ok(true);
    }
    let containers = lifecycle::containers_of(job.target.docker(), &job.project).await?;
    let running: Vec<&String> = off
        .iter()
        .filter(|s| {
            containers
                .iter()
                .any(|c| c.service.as_deref() == Some(s.as_str()))
        })
        .collect();
    if running.is_empty() {
        return Ok(true);
    }
    let names: Vec<&str> = running.iter().map(|s| s.as_str()).collect();
    if !sink(ComposeEvent::Line {
        text: format!(
            "Removing {}: their profiles are turned off",
            names.join(", ")
        ),
    }) {
        return Ok(false);
    }
    let mut args = vec!["rm", "--stop", "--force"];
    args.extend(&names);
    // Every declared profile, so compose knows the services it removes.
    let all = definition::declared_profiles(&yaml);
    let mut command = profiled_command(job, prepared, docker_host, &args, &all);
    let out = output_of(job, prepared, &mut command).await?;
    if !out.status.success() {
        return Err(failure(job, "rm", &out));
    }
    Ok(true)
}

/// Hands the source's stdin to the CLI, if it has one, and closes it.
///
/// A CLI that exits before reading all of it (a bad flag, a crash) closes
/// the pipe; that isn't the failure to report, since its exit status and
/// stderr, read next, say what went wrong.
async fn write_stdin(child: &mut tokio::process::Child, prepared: &Prepared) -> AppResult<()> {
    let mut stdin = child.stdin.take().expect("stdin is piped");
    if let Some(text) = &prepared.stdin {
        match stdin.write_all(text.as_bytes()).await {
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                log::debug!("compose stopped reading its definition: {e}");
            }
            result => result?,
        }
    }
    drop(stdin);
    Ok(())
}

/// The profiles a run passes: the Stack's own, and for taking it down every
/// profile its files declare, so services of a profile turned off since go too.
fn profiles(job: &ComposeJob, prepared: &Prepared) -> Vec<String> {
    let mut profiles = job.profiles.clone();
    if !job.action.reads_definition() {
        let yaml: Vec<&str> = prepared
            .definition
            .iter()
            .map(|(_, t)| t.as_str())
            .collect();
        profiles.extend(definition::declared_profiles(&yaml));
        profiles.sort();
        profiles.dedup();
    }
    profiles
}

/// One run of the CLI for `job`. Its environment is the Host's address, the
/// `PATH` and the Stack's own variables: never a registry login.
fn cli_command(job: &ComposeJob, prepared: &Prepared, docker_host: &str, args: &[&str]) -> Command {
    profiled_command(job, prepared, docker_host, args, &profiles(job, prepared))
}

/// [`cli_command`] with the given `profiles` turned on.
fn profiled_command(
    job: &ComposeJob,
    prepared: &Prepared,
    docker_host: &str,
    args: &[&str],
    profiles: &[String],
) -> Command {
    let mut command = Command::new(&job.cli.program);
    command
        .args(&job.cli.prefix)
        .args(["--project-name", &job.project])
        .args(&prepared.file_args)
        .args(profiles.iter().flat_map(|p| ["--profile", p.as_str()]))
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
        // Set rather than removed, so neither the user's shell, the Stack's
        // environment nor a folder's .env changes which profiles run.
        .env("COMPOSE_PROFILES", profiles.join(","))
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
    let out = output_of(job, prepared, &mut command).await?;
    if !out.status.success() {
        return Err(failure(job, "config", &out));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Runs a short CLI command to the end and collects what it printed.
async fn output_of(
    job: &ComposeJob,
    prepared: &Prepared,
    command: &mut Command,
) -> AppResult<std::process::Output> {
    let mut child = command
        .spawn()
        .map_err(|e| errors::spawn_failed(&job.cli.program, &e))?;
    write_stdin(&mut child, prepared).await?;
    Ok(child.wait_with_output().await?)
}

/// Why the CLI's `verb` failed, from the end of its stderr.
fn failure(job: &ComposeJob, verb: &str, out: &std::process::Output) -> AppError {
    let stderr: Vec<String> = String::from_utf8_lossy(&out.stderr)
        .lines()
        .map(str::to_owned)
        .collect();
    let tail = &stderr[stderr.len().saturating_sub(STDERR_TAIL)..];
    errors::exit_failure(job.target.name(), verb, out.status.code(), tail)
}

#[cfg(test)]
mod tests {
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
            target: StackTarget::Host {
                name: "local".into(),
                connection,
                dialer,
                docker,
            },
            logins,
            action,
            profiles: vec![],
            record: None,
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
                host_id: host.clone(),
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
            "services:\n  app:\n    build: .\n    env_file: .env\n    volumes:\n      - ./data:/data\n".into(),
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
}

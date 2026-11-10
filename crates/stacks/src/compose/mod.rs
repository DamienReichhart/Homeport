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
pub mod stack;

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
    model::definition,
    runtime::{lifecycle, ComposeJob, Prepared, Reach, Recorder, StackTarget, WorkDir},
};

/// Lines of stderr kept to explain a failed run.
pub(crate) const STDERR_TAIL: usize = 40;

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

/// Runs one compose command, streaming its output lines to `sink`: `docker
/// compose` on a Host, `docker stack deploy` on a Swarm. A deploy with a
/// recorder is recorded when it starts and finished when it ends; a run
/// nobody listens to any more stays unfinished.
pub(crate) async fn run(mut job: ComposeJob, sink: impl Fn(ComposeEvent) -> bool) {
    let recorder = job.record.take();
    let mut recorded = None;
    let result = if job.target.is_swarm() {
        stack::execute(job, &sink, recorder.as_ref(), &mut recorded).await
    } else {
        execute(job, &sink, recorder.as_ref(), &mut recorded).await
    };
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
    let Reach {
        name,
        connection,
        dialer,
        ..
    } = target.reach();
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
mod tests;

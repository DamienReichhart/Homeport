//! Running a CLI on this machine, against a Cluster through a [`ClusterProxy`]
//! when it needs one: its output collected, or streamed line by line.

use std::{collections::VecDeque, io, path::PathBuf, process::Stdio};

use serde::Serialize;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
};

use homeport_core::{
    error::{AppError, AppResult},
    programs,
};

use super::{open, ClientSource, ClusterProxy};
use crate::model::Cluster;

/// Lines of stderr kept to explain a failed run.
const STDERR_TAIL: usize = 40;

/// One run of a program.
#[derive(Debug, Clone, Default)]
pub struct CliCommand {
    pub program: PathBuf,
    pub args: Vec<String>,
    /// Written to the program's stdin, then closed; values never go in a file.
    pub stdin: Option<String>,
    pub env: Vec<(String, String)>,
    /// Variables of Homeport's own environment the program mustn't see,
    /// such as ones that would point it at another cluster.
    pub env_remove: Vec<String>,
}

/// Why a run failed, for the caller to explain in its own words.
#[derive(Debug)]
pub enum Failure<'a> {
    /// The program couldn't be started.
    Spawn(&'a io::Error),
    /// It exited with `code` (`None` when killed), saying `stderr`.
    Exit { code: Option<i32>, stderr: &'a str },
}

/// What a streamed run reports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CliEvent {
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

/// The Cluster a command runs against, and where its client comes from.
#[derive(Clone)]
pub struct ClusterAccess {
    pub cluster: Cluster,
    pub clients: ClientSource,
}

impl CliCommand {
    fn command(&self, proxy: Option<&ClusterProxy>) -> Command {
        let mut command = Command::new(&self.program);
        command
            .args(&self.args)
            .env("PATH", programs::search_path())
            .stdin(if self.stdin.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        for name in &self.env_remove {
            command.env_remove(name);
        }
        command.envs(self.env.iter().map(|(k, v)| (k, v)));
        if let Some(proxy) = proxy {
            command
                .arg("--kubeconfig")
                .arg(proxy.kubeconfig())
                .env("KUBECONFIG", proxy.kubeconfig());
        }
        command
    }

    fn spawn(&self, proxy: Option<&ClusterProxy>) -> io::Result<Child> {
        let mut child = self.command(proxy).spawn()?;
        if let (Some(text), Some(mut stdin)) = (self.stdin.clone(), child.stdin.take()) {
            tokio::spawn(async move {
                if let Err(e) = stdin.write_all(text.as_bytes()).await {
                    log::debug!("a CLI stopped reading its stdin: {e}");
                }
            });
        }
        Ok(child)
    }
}

async fn proxy_for(access: Option<&ClusterAccess>) -> AppResult<Option<ClusterProxy>> {
    match access {
        Some(access) => Ok(Some(open(&access.cluster, access.clients.clone()).await?)),
        None => Ok(None),
    }
}

/// Runs `command`, through a proxy to `access`'s Cluster if given, and
/// returns what it printed on stdout; a failure is explained by `fail`.
pub async fn output(
    command: &CliCommand,
    access: Option<&ClusterAccess>,
    fail: impl Fn(Failure<'_>) -> AppError,
) -> AppResult<String> {
    // Kept open until the program exits.
    let proxy = proxy_for(access).await?;
    let mut child = command
        .spawn(proxy.as_ref())
        .map_err(|e| fail(Failure::Spawn(&e)))?;
    let mut stdout = child.stdout.take().expect("stdout is piped");
    let mut stderr = child.stderr.take().expect("stderr is piped");
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let (read_out, read_err) =
        tokio::join!(stdout.read_to_end(&mut out), stderr.read_to_end(&mut err));
    read_out?;
    read_err?;
    let status = child.wait().await?;
    if !status.success() {
        let stderr = String::from_utf8_lossy(&err);
        return Err(fail(Failure::Exit {
            code: status.code(),
            stderr: stderr.trim(),
        }));
    }
    Ok(String::from_utf8_lossy(&out).into_owned())
}

/// Runs `command`, through a proxy to `access`'s Cluster if given, and
/// streams its stdout and stderr lines to `sink`, then how it ended; a
/// failure is explained by `fail`. Stops, killing the program, once `sink`
/// returns `false`.
pub async fn run_cli(
    command: CliCommand,
    access: Option<ClusterAccess>,
    fail: impl Fn(Failure<'_>) -> AppError,
    sink: impl Fn(CliEvent) -> bool,
) {
    if let Err(error) = stream(&command, access.as_ref(), &fail, &sink).await {
        sink(CliEvent::Error {
            error: error.logged("cli run"),
        });
    }
}

async fn stream(
    command: &CliCommand,
    access: Option<&ClusterAccess>,
    fail: &impl Fn(Failure<'_>) -> AppError,
    sink: &impl Fn(CliEvent) -> bool,
) -> AppResult<()> {
    let proxy = proxy_for(access).await?;
    let mut child = command
        .spawn(proxy.as_ref())
        .map_err(|e| fail(Failure::Spawn(&e)))?;
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
            if !sink(CliEvent::Line { text }) {
                return Ok(());
            }
        }
    }
    let status = child.wait().await?;
    let error = (!status.success()).then(|| {
        let stderr = Vec::from(tail).join("\n");
        fail(Failure::Exit {
            code: status.code(),
            stderr: stderr.trim(),
        })
        .logged("cli run")
    });
    sink(CliEvent::Exit {
        success: status.success(),
        code: status.code(),
        error,
    });
    drop(proxy);
    Ok(())
}

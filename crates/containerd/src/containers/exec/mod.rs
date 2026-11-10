//! Terminals in a containerd container's task (ADR 0020). containerd takes
//! a process's stdio as paths on its machine, so a short script there makes
//! two FIFOs in a private folder and copies the session through them with
//! `cat`; containerd starts the process on them with a terminal.

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use serde::Deserialize;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    task::AbortHandle,
};

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    terminal::{ExecEvent, Utf8Decoder, DEFAULT_SHELL},
};
use homeport_transport::{shell_quote, BoxStream};

use crate::{
    containers,
    grpc::{
        api::containerd::{
            services::tasks::v1::{
                DeleteProcessRequest, ExecProcessRequest, GetRequest, KillRequest,
                ResizePtyRequest, StartRequest, WaitRequest,
            },
            v1::types::Status,
        },
        client::{call, stream},
        error,
    },
    messages as msg, Containerd,
};

/// The type containerd reads a process spec's JSON as.
const PROCESS_SPEC: &str = "types.containerd.io/opencontainers/runtime-spec/1/Process";

/// How long the machine may take to make the FIFOs.
const SETUP_TIMEOUT: Duration = Duration::from_secs(10);

/// Makes the FIFOs, says where, then copies its stdin into one and the
/// other back to its stdout until the process closes it. Background jobs'
/// stdin is /dev/null unless redirected, hence fd 3.
const SCRIPT: &str = r#"d=$(mktemp -d "${TMPDIR:-/tmp}/homeport-exec.XXXXXX") || exit 1
trap 'kill "$p" 2>/dev/null; rm -rf "$d"' EXIT
mkfifo -m 600 "$d/in" "$d/out" || exit 1
echo "$d"
exec 3<&0
cat <&3 >"$d/in" &
p=$!
cat "$d/out""#;

#[derive(Debug, Clone, Default, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContainerdExecOptions {
    /// The command to run, parsed with shell quoting; the default picks a shell.
    pub command: Option<String>,
    /// A numeric `uid[:gid]`; the container's user otherwise.
    pub user: Option<String>,
    pub working_dir: Option<String>,
    pub cols: u16,
    pub rows: u16,
}

/// The process spec of an exec: the container's own process, with this
/// command, a terminal of this size, and the user and folder asked for.
fn process_spec(
    container_spec: &serde_json::Value,
    options: &ContainerdExecOptions,
) -> AppResult<serde_json::Value> {
    let args: Vec<String> = match options.command.as_deref().map(str::trim) {
        Some(c) if !c.is_empty() => {
            shell_words::split(c).map_err(|_| AppError::validation(msg::EXEC_UNCLOSED_QUOTE))?
        }
        _ => vec!["sh".into(), "-c".into(), DEFAULT_SHELL.into()],
    };
    let mut process = container_spec
        .get("process")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({ "cwd": "/", "user": { "uid": 0, "gid": 0 } }));
    let object = process
        .as_object_mut()
        .ok_or_else(|| AppError::new(ErrorCode::EngineError, msg::EXEC_NO_SPEC))?;
    object.insert("args".into(), serde_json::json!(args));
    object.insert("terminal".into(), serde_json::json!(true));
    object.insert(
        "consoleSize".into(),
        serde_json::json!({ "height": options.rows.max(1), "width": options.cols.max(1) }),
    );
    let mut env: Vec<String> = object
        .get("env")
        .and_then(|e| e.as_array())
        .map(|e| {
            e.iter()
                .filter_map(|v| v.as_str())
                .filter(|v| !v.starts_with("TERM="))
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    env.push("TERM=xterm-256color".into());
    object.insert("env".into(), serde_json::json!(env));
    if let Some(dir) = options
        .working_dir
        .as_deref()
        .map(str::trim)
        .filter(|d| !d.is_empty())
    {
        object.insert("cwd".into(), serde_json::json!(dir));
    }
    if let Some(user) = options
        .user
        .as_deref()
        .map(str::trim)
        .filter(|u| !u.is_empty())
    {
        let (uid, gid) = match user.split_once(':') {
            Some((uid, gid)) => (uid, Some(gid)),
            None => (user, None),
        };
        let invalid = || AppError::validation(msg::EXEC_USER_NOT_NUMERIC.with("user", user));
        let uid: u32 = uid.parse().map_err(|_| invalid())?;
        let gid: u32 = match gid {
            Some(gid) => gid.parse().map_err(|_| invalid())?,
            None => uid,
        };
        object.insert("user".into(), serde_json::json!({ "uid": uid, "gid": gid }));
    }
    Ok(process)
}

/// An open terminal in a container's task.
pub struct ContainerdExec {
    client: Containerd,
    namespace: String,
    container_id: String,
    exec_id: String,
    input: tokio::sync::Mutex<WriteHalf<BoxStream>>,
}

impl ContainerdExec {
    pub async fn write(&self, data: &[u8]) -> AppResult<()> {
        let mut input = self.input.lock().await;
        input.write_all(data).await?;
        input.flush().await?;
        Ok(())
    }

    pub async fn resize(&self, cols: u16, rows: u16) -> AppResult<()> {
        self.client
            .tasks()
            .resize_pty(call(
                &self.namespace,
                ResizePtyRequest {
                    container_id: self.container_id.clone(),
                    exec_id: self.exec_id.clone(),
                    width: cols.max(1).into(),
                    height: rows.max(1).into(),
                },
            )?)
            .await
            .map_err(|s| error::status(&s))?;
        Ok(())
    }

    /// Kills the process and deletes it from the task; closing the stream
    /// ends the script, which removes the FIFOs.
    pub async fn close(&self) {
        let tasks = || self.client.tasks();
        if let Ok(request) = call(
            &self.namespace,
            KillRequest {
                container_id: self.container_id.clone(),
                exec_id: self.exec_id.clone(),
                signal: 9,
                all: false,
            },
        ) {
            tasks().kill(request).await.ok();
        }
        if let Ok(request) = stream(
            &self.namespace,
            WaitRequest {
                container_id: self.container_id.clone(),
                exec_id: self.exec_id.clone(),
            },
        ) {
            tokio::time::timeout(Duration::from_secs(5), tasks().wait(request))
                .await
                .ok();
        }
        self.delete().await;
    }

    async fn delete(&self) {
        delete_process(
            &self.client,
            &self.namespace,
            &self.container_id,
            &self.exec_id,
        )
        .await;
    }

    async fn exit_code(&self) -> Option<i64> {
        let request = stream(
            &self.namespace,
            WaitRequest {
                container_id: self.container_id.clone(),
                exec_id: self.exec_id.clone(),
            },
        )
        .ok()?;
        let waited =
            tokio::time::timeout(Duration::from_secs(5), self.client.tasks().wait(request))
                .await
                .ok()?
                .ok()?;
        Some(waited.into_inner().exit_status.into())
    }
}

/// Removes an exec process from its task, once it has exited or never started.
async fn delete_process(client: &Containerd, namespace: &str, container_id: &str, exec_id: &str) {
    if let Ok(request) = call(
        namespace,
        DeleteProcessRequest {
            container_id: container_id.to_owned(),
            exec_id: exec_id.to_owned(),
        },
    ) {
        client.tasks().delete_process(request).await.ok();
    }
}

/// Opens the FIFOs' blocked ends and closes them, so a script whose process
/// never started finishes and removes its folder.
async fn unblock(client: &Containerd, folder: &str) {
    let (fifo_in, fifo_out) = (
        shell_quote(&format!("{folder}/in")),
        shell_quote(&format!("{folder}/out")),
    );
    let command = format!(
        "exec 3<>{fifo_out} 4<>{fifo_in}; exec 3>&- 4>&-; rm -rf {}",
        shell_quote(folder)
    );
    client
        .dialer()
        .run(command, Duration::from_secs(5))
        .await
        .ok();
}

/// Reads the script's first line, the FIFOs' folder, a byte at a time so
/// none of the terminal's output is taken with it.
async fn first_line(io: &mut BoxStream) -> std::io::Result<String> {
    let mut line = Vec::new();
    let mut byte = [0; 1];
    loop {
        if io.read(&mut byte).await? == 0 {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }
        if byte[0] == b'\n' {
            return Ok(String::from_utf8_lossy(&line).into_owned());
        }
        line.push(byte[0]);
    }
}

/// A process the runtime couldn't start because its program isn't there.
fn no_shell(status: &tonic::Status) -> bool {
    let text = status.message();
    text.contains("executable file not found") || text.contains("no such file or directory")
}

/// Starts a terminal in `container_id`'s running task.
pub async fn start(
    client: &Containerd,
    namespace: &str,
    container_id: &str,
    options: ContainerdExecOptions,
) -> AppResult<(Arc<ContainerdExec>, ReadHalf<BoxStream>)> {
    let dialer = client.dialer();
    if !dialer.runs_commands() {
        return Err(AppError::new(
            ErrorCode::ContainerdNeedsShell,
            msg::EXEC_NEEDS_SHELL.with("name", client.name()),
        )
        .with_hint(msg::NEEDS_SHELL_HINT));
    }
    let container = containers::get(client, namespace, container_id).await?;
    let running = client
        .tasks()
        .get(call(
            namespace,
            GetRequest {
                container_id: container_id.to_owned(),
                exec_id: String::new(),
            },
        )?)
        .await
        .ok()
        .and_then(|r| r.into_inner().process)
        .is_some_and(|p| p.status() == Status::Running);
    if !running {
        return Err(AppError::new(
            ErrorCode::ContainerNotRunning,
            msg::TASK_NOT_RUNNING.with("id", container_id),
        ));
    }
    let container_spec: serde_json::Value = container
        .spec
        .as_ref()
        .and_then(|any| serde_json::from_slice(&any.value).ok())
        .unwrap_or_default();
    let spec = process_spec(&container_spec, &options)?;

    let spawned = dialer.spawn(SCRIPT.to_owned()).await.map_err(|e| {
        AppError::new(ErrorCode::ContainerdNeedsShell, msg::EXEC_SETUP_FAILED)
            .with_hint(msg::EXEC_SETUP_FAILED_HINT)
            .with_cause(&e)
    })?;
    let mut io = spawned.io;
    let folder = match tokio::time::timeout(SETUP_TIMEOUT, first_line(&mut io)).await {
        Ok(Ok(folder)) if folder.starts_with('/') => folder,
        other => {
            drop(io);
            let exit = spawned.exit.await;
            let detail = match other {
                Err(_) => "the machine didn't make the FIFOs in time".to_owned(),
                _ => exit.stderr.trim().to_owned(),
            };
            return Err(
                AppError::new(ErrorCode::ContainerdNeedsShell, msg::EXEC_SETUP_FAILED)
                    .with_hint(msg::EXEC_SETUP_FAILED_HINT)
                    .with_detail(detail),
            );
        }
    };
    let exec_id = format!(
        "homeport-{}",
        &uuid::Uuid::new_v4().simple().to_string()[..12]
    );
    let started = async {
        client
            .tasks()
            .exec(call(
                namespace,
                ExecProcessRequest {
                    container_id: container_id.to_owned(),
                    stdin: format!("{folder}/in"),
                    stdout: format!("{folder}/out"),
                    stderr: String::new(),
                    terminal: true,
                    spec: Some(prost_types::Any {
                        type_url: PROCESS_SPEC.into(),
                        value: serde_json::to_vec(&spec)?,
                    }),
                    exec_id: exec_id.clone(),
                },
            )?)
            .await
            .map_err(|s| error::status(&s))?;
        client
            .tasks()
            .start(call(
                namespace,
                StartRequest {
                    container_id: container_id.to_owned(),
                    exec_id: exec_id.clone(),
                },
            )?)
            .await
            .map_err(|s| {
                if no_shell(&s) {
                    AppError::new(ErrorCode::NoShell, msg::EXEC_NO_SHELL)
                        .with_hint(msg::EXEC_NO_SHELL_HINT)
                        .with_detail(error::detail(&s))
                } else {
                    error::status(&s)
                }
            })?;
        Ok::<(), AppError>(())
    };
    if let Err(e) = started.await {
        delete_process(client, namespace, container_id, &exec_id).await;
        drop(io);
        unblock(client, &folder).await;
        return Err(e);
    }
    let (output, input) = tokio::io::split(io);
    let session = ContainerdExec {
        client: client.clone(),
        namespace: namespace.to_owned(),
        container_id: container_id.to_owned(),
        exec_id,
        input: tokio::sync::Mutex::new(input),
    };
    session.resize(options.cols, options.rows).await.ok();
    Ok((Arc::new(session), output))
}

/// Forwards the terminal's output until the process exits, then reports
/// its exit code.
pub async fn pump(
    session: Arc<ContainerdExec>,
    mut output: ReadHalf<BoxStream>,
    sink: impl Fn(ExecEvent) -> bool,
) {
    let mut decoder = Utf8Decoder::default();
    let mut buf = vec![0; 16 * 1024];
    loop {
        match output.read(&mut buf).await {
            Ok(0) => break,
            Ok(n) => {
                let data = decoder.push(&buf[..n]);
                if !data.is_empty() && !sink(ExecEvent::Output { data }) {
                    return;
                }
            }
            Err(e) => {
                sink(ExecEvent::Error {
                    error: AppError::from(e).logged("containerd exec session"),
                });
                return;
            }
        }
    }
    let code = session.exit_code().await;
    session.delete().await;
    sink(ExecEvent::Exit {
        code,
        error: matches!(code, Some(126 | 127)).then(|| {
            AppError::new(ErrorCode::NoShell, msg::EXEC_NO_SHELL).with_hint(msg::EXEC_NO_SHELL_HINT)
        }),
    });
}

pub type SessionId = u32;

/// The open terminals, by id, each with the task forwarding its output.
#[derive(Default)]
pub struct ContainerdExecSessions {
    next: AtomicU32,
    open: Mutex<HashMap<SessionId, (Arc<ContainerdExec>, AbortHandle)>>,
}

impl ContainerdExecSessions {
    pub fn open(
        &self,
        session: Arc<ContainerdExec>,
        output: ReadHalf<BoxStream>,
        sink: impl Fn(ExecEvent) -> bool + Send + 'static,
    ) -> SessionId {
        let id = self.next.fetch_add(1, Ordering::Relaxed) + 1;
        let pumped = tokio::spawn(pump(session.clone(), output, sink));
        self.lock().insert(id, (session, pumped.abort_handle()));
        id
    }

    pub fn get(&self, id: SessionId) -> AppResult<Arc<ContainerdExec>> {
        self.lock()
            .get(&id)
            .map(|(session, _)| session.clone())
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, msg::EXEC_ENDED))
    }

    /// Ends a terminal: stops forwarding, kills the process, closes the FIFOs.
    pub fn close(&self, id: SessionId) {
        if let Some((session, pumped)) = self.lock().remove(&id) {
            pumped.abort();
            tokio::spawn(async move {
                session.close().await;
                drop(session);
            });
        }
    }

    fn lock(
        &self,
    ) -> std::sync::MutexGuard<'_, HashMap<SessionId, (Arc<ContainerdExec>, AbortHandle)>> {
        self.open.lock().expect("exec sessions lock poisoned")
    }
}

#[cfg(test)]
mod tests;

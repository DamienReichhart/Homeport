//! Terminals on an Instance through Incus's websockets: Exec runs a
//! command (a shell by default) inside a running Instance, Console attaches
//! to its text console. Both start an operation whose metadata holds the
//! secrets of a data websocket and a control one (resizes, signals).

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::Deserialize;
use tokio::task::AbortHandle;
use tokio_tungstenite::tungstenite::Message;

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    terminal::{ExecEvent, Utf8Decoder, DEFAULT_SHELL},
};
use homeport_transport::http::{self, Request};

use crate::{
    client::{scoped, Incus},
    instances, messages as msg, operations,
    websocket::{self, Socket},
};

/// What the Exec tab asks for.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusExecOptions {
    /// A command line, split like a shell would; a shell when empty.
    pub command: Option<String>,
    /// A numeric user ID; root when empty.
    pub user: Option<u32>,
    pub cwd: Option<String>,
    pub cols: u16,
    pub rows: u16,
}

/// Exec or Console.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Exec,
    Console,
}

/// An open terminal on an Instance.
pub struct IncusTerminal {
    incus: Incus,
    project: String,
    operation: String,
    kind: Kind,
    data: tokio::sync::Mutex<SplitSink<Socket, Message>>,
    control: tokio::sync::Mutex<SplitSink<Socket, Message>>,
}

/// The body of an exec request.
pub(crate) fn exec_body(options: &IncusExecOptions) -> AppResult<serde_json::Value> {
    let command: Vec<String> = match options.command.as_deref().map(str::trim) {
        Some(line) if !line.is_empty() => {
            shell_words::split(line).map_err(|_| AppError::validation(msg::EXEC_UNCLOSED_QUOTE))?
        }
        _ => vec!["/bin/sh".into(), "-c".into(), DEFAULT_SHELL.into()],
    };
    let mut body = serde_json::json!({
        "command": command,
        "environment": { "TERM": "xterm-256color" },
        "interactive": true,
        "wait-for-websocket": true,
        "width": options.cols.max(1),
        "height": options.rows.max(1),
    });
    if let Some(user) = options.user {
        body["user"] = user.into();
        body["group"] = user.into();
    }
    if let Some(cwd) = options
        .cwd
        .as_deref()
        .map(str::trim)
        .filter(|c| !c.is_empty())
    {
        body["cwd"] = cwd.into();
    }
    Ok(body)
}

/// A control message that resizes the terminal; Incus wants text numbers.
pub(crate) fn resize_message(cols: u16, rows: u16) -> String {
    serde_json::json!({
        "command": "window-resize",
        "args": { "width": cols.max(1).to_string(), "height": rows.max(1).to_string() },
    })
    .to_string()
}

/// A control message that sends `signal` to the command.
pub(crate) fn signal_message(signal: i32) -> String {
    serde_json::json!({ "command": "signal", "signal": signal }).to_string()
}

#[derive(Deserialize)]
struct Fds {
    fds: HashMap<String, String>,
}

/// Opens the websockets of a started exec or console `operation`.
async fn attach(
    incus: &Incus,
    project: &str,
    operation: &operations::Operation,
) -> AppResult<(Socket, Socket)> {
    let fds: Fds = crate::client::metadata(operation.metadata.clone().unwrap_or_default())?;
    let path = |secret: &str| {
        scoped(
            &format!(
                "/1.0/operations/{}/websocket?secret={}",
                http::encode(&operation.id),
                http::encode(secret)
            ),
            project,
        )
    };
    let secret = |fd: &str| {
        fds.fds
            .get(fd)
            .cloned()
            .ok_or_else(|| AppError::new(ErrorCode::EngineError, msg::UNREADABLE))
    };
    let control = websocket::open(incus, &path(&secret("control")?)).await?;
    let data = websocket::open(incus, &path(&secret("0")?)).await?;
    Ok((data, control))
}

/// A terminal refused because the Instance doesn't run, named.
fn not_running(error: AppError, name: &str) -> AppError {
    if error.is(ErrorCode::ContainerNotRunning) {
        AppError::new(
            ErrorCode::ContainerNotRunning,
            msg::TERMINAL_NOT_RUNNING.with("name", name),
        )
        .with_hint(msg::INSTANCE_NOT_RUNNING_HINT)
        .with_detail(error.detail.unwrap_or(error.message))
    } else {
        error
    }
}

impl IncusTerminal {
    /// Starts `options`' command in the running Instance `name`.
    pub async fn exec(
        incus: &Incus,
        project: &str,
        name: &str,
        options: &IncusExecOptions,
    ) -> AppResult<(Arc<Self>, SplitStream<Socket>)> {
        let body = exec_body(options)?;
        let path = scoped(&format!("{}/exec", instances::path(name)), project);
        Self::open(
            incus,
            project,
            name,
            Request::post(path).json(&body)?,
            Kind::Exec,
        )
        .await
    }

    /// Attaches to the text console of the running Instance `name`.
    pub async fn console(
        incus: &Incus,
        project: &str,
        name: &str,
        cols: u16,
        rows: u16,
    ) -> AppResult<(Arc<Self>, SplitStream<Socket>)> {
        let body = serde_json::json!({
            "type": "console",
            "width": cols.max(1),
            "height": rows.max(1),
        });
        let path = scoped(&format!("{}/console", instances::path(name)), project);
        Self::open(
            incus,
            project,
            name,
            Request::post(path).json(&body)?,
            Kind::Console,
        )
        .await
    }

    async fn open(
        incus: &Incus,
        project: &str,
        name: &str,
        request: Request,
        kind: Kind,
    ) -> AppResult<(Arc<Self>, SplitStream<Socket>)> {
        let operation = incus
            .start(request)
            .await
            .map_err(|e| not_running(e, name))?
            .ok_or_else(|| AppError::new(ErrorCode::EngineError, msg::UNREADABLE))?;
        let (data, control) = attach(incus, project, &operation).await?;
        let (data_sink, data_stream) = data.split();
        // Incus sends nothing on the control websocket that a client needs.
        let (control_sink, control_stream) = control.split();
        tokio::spawn(control_stream.for_each(|_| async {}));
        Ok((
            Arc::new(Self {
                incus: incus.clone(),
                project: project.to_owned(),
                operation: operation.id,
                kind,
                data: tokio::sync::Mutex::new(data_sink),
                control: tokio::sync::Mutex::new(control_sink),
            }),
            data_stream,
        ))
    }

    pub async fn write(&self, data: &[u8]) -> AppResult<()> {
        self.data
            .lock()
            .await
            .send(Message::binary(data.to_vec()))
            .await
            .map_err(|e| ended().with_cause(&e))
    }

    pub async fn resize(&self, cols: u16, rows: u16) -> AppResult<()> {
        self.control
            .lock()
            .await
            .send(Message::text(resize_message(cols, rows)))
            .await
            .map_err(|e| ended().with_cause(&e))
    }

    /// Ends the terminal. Exec hangs up its command (SIGHUP, then SIGKILL)
    /// so nothing it started from the terminal is left running; Console
    /// only detaches, leaving the Instance as it was.
    pub async fn close(&self) {
        if self.kind == Kind::Exec {
            let mut control = self.control.lock().await;
            for signal in [1, 9] {
                if control
                    .send(Message::text(signal_message(signal)))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
        self.hang_up().await;
    }

    /// Closes both websockets: Incus ends an exec's operation only then.
    async fn hang_up(&self) {
        let _ = self.data.lock().await.close().await;
        let _ = self.control.lock().await.close().await;
    }

    /// How the command ended, once its operation has: its exit code, or
    /// why it couldn't run.
    async fn outcome(&self) -> (Option<i64>, Option<AppError>) {
        let ended = async {
            let running = operations::get(&self.incus, &self.project, &self.operation).await?;
            operations::wait(
                &self.incus,
                &self.project,
                running,
                std::time::Duration::from_secs(5),
                |_| {},
            )
            .await
        };
        match ended.await {
            Ok(done) => {
                let code = done
                    .metadata
                    .as_ref()
                    .and_then(|m| m.get("return"))
                    .and_then(serde_json::Value::as_i64);
                (code, matches!(code, Some(126 | 127)).then(no_program))
            }
            // Incus fails the operation of a command it can't start.
            Err(e) if missing_program(&e) => (
                Some(127),
                Some(no_program().with_detail(e.detail.unwrap_or(e.message))),
            ),
            Err(e) => (None, Some(e)),
        }
    }
}

fn ended() -> AppError {
    AppError::new(ErrorCode::NotFound, msg::TERMINAL_ENDED)
}

/// Forwards the terminal's output until it closes, then reports how it ended.
pub async fn pump(
    session: Arc<IncusTerminal>,
    mut output: SplitStream<Socket>,
    sink: impl Fn(ExecEvent) -> bool,
) {
    let mut decoder = Utf8Decoder::default();
    while let Some(message) = output.next().await {
        let bytes = match message {
            Ok(Message::Binary(bytes)) => bytes.to_vec(),
            Ok(Message::Text(text)) => text.as_bytes().to_vec(),
            Ok(Message::Close(_)) => break,
            Ok(_) => continue,
            Err(e) => {
                sink(ExecEvent::Error {
                    error: AppError::new(ErrorCode::ConnectionLost, msg::TERMINAL_ENDED)
                        .with_cause(&e)
                        .logged("incus terminal"),
                });
                return;
            }
        };
        // Incus ends a stream with an empty message.
        if bytes.is_empty() {
            break;
        }
        let data = decoder.push(&bytes);
        if !data.is_empty() && !sink(ExecEvent::Output { data }) {
            return;
        }
    }
    if session.kind == Kind::Console {
        sink(ExecEvent::Exit {
            code: None,
            error: None,
        });
        return;
    }
    // The operation records the exit code once the command has ended and
    // the websockets are closed.
    session.hang_up().await;
    let (code, error) = session.outcome().await;
    sink(ExecEvent::Exit { code, error });
}

fn no_program() -> AppError {
    AppError::new(ErrorCode::NoShell, msg::EXEC_NO_SHELL).with_hint(msg::EXEC_NO_SHELL_HINT)
}

/// Whether an exec failed because its program doesn't exist.
fn missing_program(error: &AppError) -> bool {
    let text = error
        .detail
        .as_deref()
        .unwrap_or(&error.message)
        .to_lowercase();
    ["no such file", "not found", "executable file"]
        .iter()
        .any(|t| text.contains(t))
}

/// What the Instance printed on its console before attaching; empty when
/// Incus keeps none.
pub async fn console_log(incus: &Incus, project: &str, name: &str) -> String {
    let path = scoped(&format!("{}/console", instances::path(name)), project);
    match http::send(&**incus.dialer(), Request::get(path)).await {
        Ok(response) if response.status.is_success() => {
            String::from_utf8_lossy(&response.body).into_owned()
        }
        _ => String::new(),
    }
}

pub type SessionId = u32;

/// The open terminals, by id, each with the task forwarding its output.
#[derive(Default)]
pub struct IncusTerminals {
    next: AtomicU32,
    open: Mutex<HashMap<SessionId, (Arc<IncusTerminal>, AbortHandle)>>,
}

impl IncusTerminals {
    pub fn open(
        &self,
        session: Arc<IncusTerminal>,
        output: SplitStream<Socket>,
        sink: impl Fn(ExecEvent) -> bool + Send + 'static,
    ) -> SessionId {
        let id = self.next.fetch_add(1, Ordering::Relaxed) + 1;
        let pumped = tokio::spawn(pump(session.clone(), output, sink));
        self.lock().insert(id, (session, pumped.abort_handle()));
        id
    }

    pub fn get(&self, id: SessionId) -> AppResult<Arc<IncusTerminal>> {
        self.lock()
            .get(&id)
            .map(|(session, _)| session.clone())
            .ok_or_else(ended)
    }

    /// Ends a terminal: stops forwarding, hangs up an exec, detaches a console.
    pub fn close(&self, id: SessionId) {
        if let Some((session, pumped)) = self.lock().remove(&id) {
            pumped.abort();
            tokio::spawn(async move { session.close().await });
        }
    }

    fn lock(
        &self,
    ) -> std::sync::MutexGuard<'_, HashMap<SessionId, (Arc<IncusTerminal>, AbortHandle)>> {
        self.open.lock().expect("terminal sessions lock poisoned")
    }
}

#[cfg(test)]
mod tests;

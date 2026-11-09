//! Interactive terminals in a task of an allocation, through Nomad's exec
//! WebSocket (`/v1/client/allocation/:id/exec`), which the servers forward
//! to the client node: no SSH to the node, nothing installed on it. The
//! token goes in the `X-Nomad-Token` header of the upgrade, never in the
//! URL. Frames are JSON: Homeport sends `stdin` and `tty_size`, Nomad
//! sends `stdout`, `stderr` and finally `exited` with the exit code.

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use base64::Engine as _;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{net::TcpStream, task::AbortHandle};
use tokio_tungstenite::{
    client_async,
    tungstenite::{self, client::IntoClientRequest, protocol::CloseFrame, Message},
    WebSocketStream,
};

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    terminal::{ExecEvent, Utf8Decoder},
};
use homeport_transport::BoxStream;

use crate::{
    client::{segment, CONNECT_TIMEOUT, TOKEN_HEADER},
    error::{answer, mentions, unreachable},
    messages as msg, Nomad,
};

type Socket = WebSocketStream<BoxStream>;

/// The command a terminal runs when none is given.
pub const DEFAULT_COMMAND: &str = "/bin/sh";

/// What the Shell tab asks for.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NomadExecOptions {
    pub task: String,
    /// A command line, split like a shell would; `/bin/sh` when empty.
    pub command: Option<String>,
    pub cols: u16,
    pub rows: u16,
}

/// The command `options` asks for, as the JSON array Nomad takes.
pub fn command(options: &NomadExecOptions) -> AppResult<Vec<String>> {
    match options.command.as_deref().map(str::trim) {
        Some(line) if !line.is_empty() => {
            shell_words::split(line).map_err(|_| AppError::validation(msg::COMMAND_UNCLOSED_QUOTE))
        }
        _ => Ok(vec![DEFAULT_COMMAND.into()]),
    }
}

/// `value` escaped for a URL's query.
fn escape(value: &str) -> String {
    segment(value)
}

/// The URL of the exec WebSocket for `options` in allocation `alloc`.
pub(crate) fn exec_url(
    address: &str,
    alloc: &str,
    region: Option<&str>,
    options: &NomadExecOptions,
) -> AppResult<String> {
    let base = address
        .strip_prefix("https://")
        .map(|rest| format!("wss://{rest}"))
        .or_else(|| {
            address
                .strip_prefix("http://")
                .map(|rest| format!("ws://{rest}"))
        })
        .unwrap_or_else(|| address.to_owned());
    let command = serde_json::to_string(&command(options)?)?;
    let mut url = format!(
        "{base}/v1/client/allocation/{}/exec?task={}&tty=true&command={}",
        segment(alloc),
        escape(&options.task),
        escape(&command)
    );
    if let Some(region) = region {
        url.push_str(&format!("&region={}", escape(region)));
    }
    Ok(url)
}

/// A frame sending `data` to the command's input.
pub(crate) fn stdin_frame(data: &[u8]) -> String {
    json!({ "stdin": { "data": base64::engine::general_purpose::STANDARD.encode(data) } })
        .to_string()
}

/// A frame telling the command the terminal's size.
pub(crate) fn size_frame(cols: u16, rows: u16) -> String {
    json!({ "tty_size": { "width": cols.max(1), "height": rows.max(1) } }).to_string()
}

/// What a frame from Nomad says.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Received {
    Output(Vec<u8>),
    Exited(Option<i64>),
    Nothing,
}

#[derive(Deserialize, Serialize, Default)]
struct Stream {
    data: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct ExitResult {
    exit_code: Option<i64>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Frame {
    stdout: Option<Stream>,
    stderr: Option<Stream>,
    exited: bool,
    result: Option<ExitResult>,
}

/// Reads one of Nomad's exec frames.
pub(crate) fn read_frame(text: &str) -> Received {
    let Ok(frame) = serde_json::from_str::<Frame>(text) else {
        return Received::Nothing;
    };
    if frame.exited {
        return Received::Exited(frame.result.and_then(|r| r.exit_code));
    }
    let data = frame
        .stdout
        .and_then(|s| s.data)
        .or_else(|| frame.stderr.and_then(|s| s.data));
    match data.and_then(|d| base64::engine::general_purpose::STANDARD.decode(d).ok()) {
        Some(bytes) if !bytes.is_empty() => Received::Output(bytes),
        _ => Received::Nothing,
    }
}

/// An open terminal in a task.
pub struct NomadTerminal {
    sink: tokio::sync::Mutex<SplitSink<Socket, Message>>,
    command: String,
}

impl NomadTerminal {
    /// Opens a terminal running `options`' command in task `options.task`
    /// of allocation `alloc`, at the size given.
    pub async fn open(
        nomad: &Nomad,
        alloc: &str,
        options: &NomadExecOptions,
    ) -> AppResult<(Arc<Self>, SplitStream<Socket>)> {
        let url = exec_url(nomad.address(), alloc, nomad.region(), options)?;
        let failed = |e: AppError| {
            nomad.allocation_error(
                e,
                alloc,
                Some(&options.task),
                msg::EXEC_DENIED,
                msg::EXEC_DENIED_HINT,
            )
        };
        let socket = tokio::time::timeout(
            CONNECT_TIMEOUT + Duration::from_secs(5),
            connect(nomad, &url),
        )
        .await
        .map_err(|_| {
            AppError::new(ErrorCode::Timeout, msg::TIMEOUT.with("name", nomad.name()))
                .with_hint(msg::TIMEOUT_HINT)
        })?
        .map_err(failed)?;
        let (sink, stream) = socket.split();
        let terminal = Arc::new(Self {
            sink: tokio::sync::Mutex::new(sink),
            command: command(options)?.join(" "),
        });
        terminal.resize(options.cols, options.rows).await?;
        Ok((terminal, stream))
    }

    pub async fn write(&self, data: &[u8]) -> AppResult<()> {
        self.send(stdin_frame(data)).await
    }

    pub async fn resize(&self, cols: u16, rows: u16) -> AppResult<()> {
        self.send(size_frame(cols, rows)).await
    }

    async fn send(&self, frame: String) -> AppResult<()> {
        self.sink
            .lock()
            .await
            .send(Message::text(frame))
            .await
            .map_err(|e| ended().with_cause(&e))
    }

    /// Ends the terminal: closes its input, then the WebSocket, which ends
    /// the command on the node.
    pub async fn close(&self) {
        let mut sink = self.sink.lock().await;
        let _ = sink
            .send(Message::text(
                json!({ "stdin": { "close": true } }).to_string(),
            ))
            .await;
        let _ = sink.close().await;
    }
}

/// Opens the WebSocket at `url`, over TLS for `wss://`, with the token.
async fn connect(nomad: &Nomad, url: &str) -> AppResult<Socket> {
    let mut request = url.into_client_request().map_err(|e| {
        AppError::validation(msg::ADDRESS_INVALID.with("address", nomad.address())).with_cause(&e)
    })?;
    if let Some(token) = nomad.token() {
        request.headers_mut().insert(
            TOKEN_HEADER,
            token
                .parse()
                .map_err(|_| crate::error::token_invalid(nomad.name()))?,
        );
    }
    let uri = request.uri().clone();
    let host = uri
        .host()
        .unwrap_or_default()
        .trim_matches(['[', ']'])
        .to_owned();
    let port = uri.port_u16().unwrap_or(crate::DEFAULT_PORT);
    let tcp = TcpStream::connect((host.as_str(), port))
        .await
        .map_err(|e| unreachable(nomad.name(), nomad.address(), &e))?;
    tcp.set_nodelay(true)
        .map_err(|e| unreachable(nomad.name(), nomad.address(), &e))?;
    let stream: BoxStream = if uri.scheme_str() == Some("wss") {
        let name = rustls::pki_types::ServerName::try_from(host.clone()).map_err(|e| {
            AppError::validation(msg::ADDRESS_INVALID.with("address", nomad.address()))
                .with_cause(&e)
        })?;
        let tls = tokio_rustls::TlsConnector::from(nomad.tls_config())
            .connect(name, tcp)
            .await
            .map_err(|e| unreachable(nomad.name(), nomad.address(), &e))?;
        Box::new(tls)
    } else {
        Box::new(tcp)
    };
    let (socket, _) = client_async(request, stream)
        .await
        .map_err(|e| handshake_error(nomad, e))?;
    Ok(socket)
}

/// A refused upgrade carries Nomad's answer in its body.
fn handshake_error(nomad: &Nomad, e: tungstenite::Error) -> AppError {
    match e {
        tungstenite::Error::Http(response) => {
            let body = response
                .body()
                .as_deref()
                .map(String::from_utf8_lossy)
                .unwrap_or_default();
            answer(nomad.name(), response.status().as_u16(), &body)
        }
        tungstenite::Error::Io(io) => unreachable(nomad.name(), nomad.address(), &io),
        other => ended().with_cause(&other),
    }
}

fn ended() -> AppError {
    AppError::new(ErrorCode::ConnectionLost, msg::TERMINAL_ENDED)
}

/// Why Nomad closed the WebSocket before the command exited.
fn closed_early(nomad: &Nomad, frame: Option<&CloseFrame>, command: &str) -> Option<AppError> {
    let frame = frame?;
    let reason = frame.reason.to_string();
    if u16::from(frame.code) == 1000 && reason.is_empty() {
        return None;
    }
    if mentions(
        &reason,
        &[
            "no such file",
            "executable file not found",
            "not found in $path",
        ],
    ) {
        return Some(
            AppError::new(
                ErrorCode::NoShell,
                msg::EXEC_NO_COMMAND.with("command", command),
            )
            .with_hint(msg::EXEC_NO_COMMAND_HINT)
            .with_detail(&reason),
        );
    }
    let status = if mentions(&reason, &["permission denied"]) {
        403
    } else {
        500
    };
    Some(answer(nomad.name(), status, &reason))
}

/// Forwards the terminal's output until the command exits or the socket
/// closes, then reports how it ended.
pub async fn pump(
    nomad: Nomad,
    alloc: String,
    task: String,
    session: Arc<NomadTerminal>,
    mut output: SplitStream<Socket>,
    sink: impl Fn(ExecEvent) -> bool,
) {
    let mut decoder = Utf8Decoder::default();
    while let Some(message) = output.next().await {
        let received = match message {
            Ok(Message::Text(text)) => read_frame(text.as_str()),
            Ok(Message::Binary(bytes)) => read_frame(&String::from_utf8_lossy(&bytes)),
            Ok(Message::Close(frame)) => {
                if let Some(error) = closed_early(&nomad, frame.as_ref(), &session.command) {
                    let error = nomad.allocation_error(
                        error,
                        &alloc,
                        Some(&task),
                        msg::EXEC_DENIED,
                        msg::EXEC_DENIED_HINT,
                    );
                    sink(ExecEvent::Error { error });
                    return;
                }
                break;
            }
            Ok(_) => continue,
            Err(e) => {
                sink(ExecEvent::Error {
                    error: ended().with_cause(&e).logged("nomad terminal"),
                });
                return;
            }
        };
        match received {
            Received::Output(bytes) => {
                let data = decoder.push(&bytes);
                if !data.is_empty() && !sink(ExecEvent::Output { data }) {
                    return;
                }
            }
            Received::Exited(code) => {
                let error = matches!(code, Some(126 | 127)).then(|| {
                    AppError::new(
                        ErrorCode::NoShell,
                        msg::EXEC_NO_COMMAND.with("command", &session.command),
                    )
                    .with_hint(msg::EXEC_NO_COMMAND_HINT)
                });
                sink(ExecEvent::Exit { code, error });
                return;
            }
            Received::Nothing => {}
        }
    }
    sink(ExecEvent::Exit {
        code: None,
        error: None,
    });
}

pub type TerminalId = u32;

/// The open terminals, by id, each with the task forwarding its output.
#[derive(Default)]
pub struct NomadTerminals {
    next: AtomicU32,
    open: Mutex<HashMap<TerminalId, (Arc<NomadTerminal>, AbortHandle)>>,
}

impl NomadTerminals {
    pub fn open(
        &self,
        nomad: Nomad,
        alloc: String,
        task: String,
        session: Arc<NomadTerminal>,
        output: SplitStream<Socket>,
        sink: impl Fn(ExecEvent) -> bool + Send + 'static,
    ) -> TerminalId {
        let id = self.next.fetch_add(1, Ordering::Relaxed) + 1;
        let pumped = tokio::spawn(pump(nomad, alloc, task, session.clone(), output, sink));
        self.lock().insert(id, (session, pumped.abort_handle()));
        id
    }

    pub fn get(&self, id: TerminalId) -> AppResult<Arc<NomadTerminal>> {
        self.lock()
            .get(&id)
            .map(|(session, _)| session.clone())
            .ok_or_else(ended)
    }

    /// Ends a terminal: stops forwarding and closes the WebSocket.
    pub fn close(&self, id: TerminalId) {
        if let Some((session, pumped)) = self.lock().remove(&id) {
            pumped.abort();
            tokio::spawn(async move { session.close().await });
        }
    }

    fn lock(
        &self,
    ) -> std::sync::MutexGuard<'_, HashMap<TerminalId, (Arc<NomadTerminal>, AbortHandle)>> {
        self.open.lock().expect("nomad terminals lock poisoned")
    }
}

/// The JSON of a frame Nomad sends, for tests.
#[cfg(test)]
pub(crate) fn output_frame(stream: &str, text: &str) -> String {
    let data = base64::engine::general_purpose::STANDARD.encode(text);
    let mut frame = serde_json::Map::new();
    frame.insert(stream.into(), json!({ "data": data }));
    serde_json::Value::Object(frame).to_string()
}

#[cfg(test)]
mod tests;

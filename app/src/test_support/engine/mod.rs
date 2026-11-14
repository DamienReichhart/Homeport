//! An Engine for tests: a socket that speaks enough of the Engine API
//! for a Host to connect to it and for a page's streams and transfers to
//! go through the real use cases, the real crates and the real client.
//! It counts what is open on it, so a test can hold a page to letting go
//! of a stream: a stream is counted from its request to the moment its
//! connection closes.
//!
//! ```ignore
//! let engine = Engine::start();              // inside a runtime
//! let host_id = engine.host(&state).await;   // a connected Host of `state`
//! assert_eq!(engine.open(Stream::Logs), 0);
//! engine.log("hello");                       // to every open log stream
//! ```
//!
//! What it answers: `_ping`, `version`, `info`, `events` (held open),
//! a container's inspect document (`json`, when it started as told),
//! `logs` and `stats` (streamed), its `archive` (stat, download, upload)
//! and its `export`, a terminal (`exec`: created, started on the
//! connection, resized, inspected), and what Run, Edit and Commit ask
//! (`create`: an image's inspect, a container created, started, stopped,
//! renamed and removed, its changes, a commit). Anything else is `404`.

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

mod create;
mod exec;
mod transfer;
mod wire;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
    sync::{broadcast, watch},
};

use homeport_hosts::{service::HostInput, EngineKind, HostConnection};

use self::wire::{inspected, Held};
pub use self::{create::Created, transfer::EXPORTED, wire::Stall};
use crate::{api::hosts::hosts_create, app::AppState};

/// A stream the fake keeps open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stream {
    Logs,
    Stats,
    /// A download or an upload that has not been answered.
    Transfer,
    /// A terminal in the container.
    Exec,
}

/// What is sent to the streams that are open.
#[derive(Debug, Clone)]
enum Push {
    Log(Vec<u8>),
    EndLogs,
    Sample(String),
    /// What a terminal's program prints, and its end.
    Print(Vec<u8>),
    Exit,
}

#[derive(Default)]
struct Shared {
    open: Mutex<HashMap<Stream, usize>>,
    /// Every request, as `GET /containers/c1/logs?follow=true`.
    asked: Mutex<Vec<String>>,
    /// What a path of the container holds.
    files: Mutex<HashMap<String, Held>>,
    /// The folders uploaded to, each with the names its archive held.
    uploaded: Mutex<Vec<(String, Vec<String>)>>,
    /// An upload whose archive holds this name is refused.
    refused: Mutex<Option<String>>,
    /// How many transfers were let go of before their answer.
    abandoned: AtomicUsize,
    /// A download's answer breaks off half way.
    cut: std::sync::atomic::AtomicBool,
    /// Where a download's answer waits for [`Engine::release`].
    stall: Mutex<Option<Stall>>,
    /// When the container last started, if it was said.
    started: Mutex<Option<String>>,
    /// What each exec was created with, and what its terminals were sent.
    execs: Mutex<Vec<String>>,
    typed: Mutex<Vec<u8>>,
    /// What reached the terminals, in order: `input …` and `resize h=…&w=…`.
    terminal: Mutex<Vec<String>>,
    /// What an exec's inspect says it exited with.
    exit_code: Mutex<i64>,
    /// The containers created, and the commits asked for (their query).
    created: Mutex<Vec<Created>>,
    commits: Mutex<Vec<String>>,
    /// What a container's inspect says, when a test said it.
    inspect: Mutex<Option<String>>,
    /// How many paths `docker diff` lists.
    changes: AtomicUsize,
    /// Why a create is refused, when it is.
    refused_create: Mutex<Option<String>>,
}

/// Counts a stream as open until dropped.
struct Open(Arc<Shared>, Stream);

impl Open {
    fn new(shared: &Arc<Shared>, stream: Stream) -> Self {
        *shared.open.lock().unwrap().entry(stream).or_default() += 1;
        Self(shared.clone(), stream)
    }
}

impl Drop for Open {
    fn drop(&mut self) {
        if let Some(count) = self.0.open.lock().unwrap().get_mut(&self.1) {
            *count = count.saturating_sub(1);
        }
    }
}

/// The fake Engine. Dropping it closes its socket.
pub struct Engine {
    pub socket: PathBuf,
    shared: Arc<Shared>,
    pushes: broadcast::Sender<Push>,
    /// Whether transfers wait before they are answered.
    held: watch::Sender<bool>,
    _dir: tempfile::TempDir,
    serving: tokio::task::JoinHandle<()>,
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.serving.abort();
    }
}

impl Engine {
    /// Starts listening on a socket of its own. Inside a runtime.
    pub fn start() -> Self {
        let dir = tempfile::tempdir().expect("a temporary folder");
        let socket = dir.path().join("engine.sock");
        let listener = UnixListener::bind(&socket).expect("a socket to listen on");
        let shared = Arc::new(Shared::default());
        let (pushes, _) = broadcast::channel(1024);
        let (held, _) = watch::channel(false);
        let serving = tokio::spawn(serve(
            listener,
            shared.clone(),
            pushes.clone(),
            held.clone(),
        ));
        Self {
            socket,
            shared,
            pushes,
            held,
            _dir: dir,
            serving,
        }
    }

    /// This Engine as a connected Host of `state`: its id.
    pub async fn host(&self, state: &Arc<AppState>) -> String {
        let input = HostInput {
            name: "fake".to_owned(),
            connection: HostConnection::Local {
                socket_path: self.socket.display().to_string(),
            },
            engine: EngineKind::Docker,
            credentials: None,
            bastion_keys: Vec::new(),
        };
        let id = hosts_create(state, input)
            .await
            .expect("a Host is created")
            .host
            .id;
        for _ in 0..200 {
            if state.hosts.registry.docker(&id).is_ok() {
                return id;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
        let status = state.hosts.registry.status(&id);
        panic!("the Host never connected to the fake Engine: {status:?}");
    }

    /// How many streams of this kind are open now.
    pub fn open(&self, stream: Stream) -> usize {
        let open = self.shared.open.lock().unwrap();
        open.get(&stream).copied().unwrap_or(0)
    }

    /// Waits, for two seconds at most, until `count` streams are open.
    pub async fn settles_at(&self, stream: Stream, count: usize) -> bool {
        for _ in 0..200 {
            if self.open(stream) == count {
                return true;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        false
    }

    /// The requests that named `part`, in order.
    pub fn asked(&self, part: &str) -> Vec<String> {
        let asked = self.shared.asked.lock().unwrap();
        asked.iter().filter(|r| r.contains(part)).cloned().collect()
    }

    /// Writes a line to the stdout of every open log stream.
    pub fn log(&self, text: &str) {
        let mut frame = vec![1, 0, 0, 0];
        let line = format!("{text}\n");
        frame.extend(u32::try_from(line.len()).unwrap_or(0).to_be_bytes());
        frame.extend(line.as_bytes());
        let _ = self.pushes.send(Push::Log(frame));
    }

    /// Ends every open log stream, as a container that stopped.
    pub fn end_logs(&self) {
        let _ = self.pushes.send(Push::EndLogs);
    }

    /// Prints `text` on every open terminal.
    pub fn print(&self, text: &str) {
        let _ = self.pushes.send(Push::Print(text.as_bytes().to_vec()));
    }

    /// Ends every open terminal, as a shell that exits.
    pub fn exit(&self) {
        let _ = self.pushes.send(Push::Exit);
    }

    /// The next terminals exit with `code`, as their inspect says.
    pub fn exit_with(&self, code: i64) {
        *self.shared.exit_code.lock().unwrap() = code;
    }

    /// What reached the terminals, in the order it came: `input <text>`
    /// for what was typed, `resize h=<rows>&w=<cols>` for a new size.
    pub fn terminal(&self) -> Vec<String> {
        self.shared.terminal.lock().unwrap().clone()
    }

    /// What the terminals were sent, all together.
    pub fn typed(&self) -> String {
        String::from_utf8_lossy(&self.shared.typed.lock().unwrap()).into_owned()
    }

    /// What each exec was created with: the Engine API's JSON.
    pub fn execs(&self) -> Vec<String> {
        self.shared.execs.lock().unwrap().clone()
    }

    /// The container started (again) at `at`, as its inspect says.
    pub fn started_at(&self, at: &str) {
        *self.shared.started.lock().unwrap() = Some(at.to_owned());
    }

    /// Sends a stats report to every open stats stream.
    pub fn sample(&self, memory: u64, received: u64) {
        let report = format!(
            r#"{{"memory_stats":{{"usage":{memory},"limit":1073741824}},"pids_stats":{{"current":3}},"networks":{{"eth0":{{"rx_bytes":{received},"tx_bytes":0}}}}}}"#
        );
        let _ = self.pushes.send(Push::Sample(format!("{report}\n")));
    }

    /// The container holds a file of `bytes` at `path`.
    pub fn file(&self, path: &str, bytes: &[u8]) {
        self.hold(path, Held::File(bytes.to_vec()));
    }

    /// The container holds a folder at `path`.
    pub fn folder(&self, path: &str) {
        self.hold(path, Held::Folder);
    }

    /// The container holds a symbolic link at `path` to `target`.
    pub fn link(&self, path: &str, target: &str) {
        self.hold(path, Held::Link(target.to_owned()));
    }

    fn hold(&self, path: &str, held: Held) {
        let mut files = self.shared.files.lock().unwrap();
        files.insert(path.to_owned(), held);
    }

    /// The folders uploaded to, each with the names its archive held.
    pub fn uploaded(&self) -> Vec<(String, Vec<String>)> {
        self.shared.uploaded.lock().unwrap().clone()
    }

    /// An upload that holds `name` is refused from now on.
    pub fn refuse_upload_of(&self, name: &str) {
        *self.shared.refused.lock().unwrap() = Some(name.to_owned());
    }

    /// Transfers wait for [`Engine::release`] before they are answered.
    pub fn hold_transfers(&self) {
        let _ = self.held.send(true);
    }

    pub fn release(&self) {
        let _ = self.held.send(false);
    }

    /// A download's answer breaks off half way from now on, as a
    /// connection that drops.
    pub fn cut_downloads(&self) {
        self.shared.cut.store(true, Ordering::SeqCst);
    }

    /// A download's answer stops at `at` until [`Engine::release`].
    pub fn stall(&self, at: Stall) {
        *self.shared.stall.lock().unwrap() = Some(at);
        self.hold_transfers();
    }

    /// How many transfers were let go of before they were answered.
    pub fn abandoned(&self) -> usize {
        self.shared.abandoned.load(Ordering::SeqCst)
    }
}

async fn serve(
    listener: UnixListener,
    shared: Arc<Shared>,
    pushes: broadcast::Sender<Push>,
    held: watch::Sender<bool>,
) {
    while let Ok((socket, _)) = listener.accept().await {
        let connection = Connection {
            socket,
            shared: shared.clone(),
            pushes: pushes.clone(),
            held: held.subscribe(),
            buffer: Vec::new(),
        };
        tokio::spawn(connection.run());
    }
}

struct Request {
    method: String,
    /// The path without its API version, and its query.
    path: String,
    query: String,
    body: Vec<u8>,
}

struct Connection {
    socket: UnixStream,
    shared: Arc<Shared>,
    pushes: broadcast::Sender<Push>,
    held: watch::Receiver<bool>,
    buffer: Vec<u8>,
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

impl Connection {
    /// Reads more of the connection; `false` when the client closed it.
    async fn more(&mut self) -> bool {
        let mut chunk = [0_u8; 8192];
        match self.socket.read(&mut chunk).await {
            Ok(read) if read > 0 => {
                self.buffer.extend_from_slice(&chunk[..read]);
                true
            }
            _ => false,
        }
    }

    async fn request(&mut self) -> Option<Request> {
        let end = loop {
            if let Some(end) = find(&self.buffer, b"\r\n\r\n") {
                break end;
            }
            if !self.more().await {
                return None;
            }
        };
        let head = String::from_utf8_lossy(&self.buffer[..end]).into_owned();
        self.buffer.drain(..end + 4);
        let mut lines = head.split("\r\n");
        let mut first = lines.next()?.split(' ');
        let (method, target) = (first.next()?.to_owned(), first.next()?.to_owned());
        let headers: HashMap<String, String> = lines
            .filter_map(|line| line.split_once(':'))
            .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_owned()))
            .collect();
        let body = if headers
            .get("transfer-encoding")
            .is_some_and(|v| v.contains("chunked"))
        {
            self.chunked().await?
        } else {
            let length = headers.get("content-length").and_then(|v| v.parse().ok());
            self.exactly(length.unwrap_or(0)).await?
        };
        let (path, query) = target.split_once('?').unwrap_or((&target, ""));
        // `/v1.47/containers/..` and `/containers/..` are the same.
        let path = match path.strip_prefix("/v") {
            Some(rest) if rest.starts_with(|c: char| c.is_ascii_digit()) => {
                rest.find('/').map_or("/", |at| &rest[at..])
            }
            _ => path,
        };
        Some(Request {
            method,
            path: path.to_owned(),
            query: query.to_owned(),
            body,
        })
    }

    async fn exactly(&mut self, length: usize) -> Option<Vec<u8>> {
        while self.buffer.len() < length {
            if !self.more().await {
                return None;
            }
        }
        Some(self.buffer.drain(..length).collect())
    }

    async fn chunked(&mut self) -> Option<Vec<u8>> {
        let mut body = vec![];
        loop {
            let end = loop {
                if let Some(end) = find(&self.buffer, b"\r\n") {
                    break end;
                }
                if !self.more().await {
                    return None;
                }
            };
            let size = String::from_utf8_lossy(&self.buffer[..end]).into_owned();
            let size = usize::from_str_radix(size.split(';').next()?.trim(), 16).ok()?;
            self.buffer.drain(..end + 2);
            let chunk = self.exactly(size + 2).await?;
            if size == 0 {
                return Some(body);
            }
            body.extend_from_slice(&chunk[..size]);
        }
    }

    async fn answer(&mut self, status: &str, headers: &[(&str, String)], body: &[u8]) -> bool {
        let mut head = format!("HTTP/1.1 {status}\r\nContent-Length: {}\r\n", body.len());
        for (name, value) in headers {
            head.push_str(&format!("{name}: {value}\r\n"));
        }
        head.push_str("\r\n");
        let wrote = self.socket.write_all(head.as_bytes()).await.is_ok();
        wrote && self.socket.write_all(body).await.is_ok()
    }

    async fn json(&mut self, status: &str, body: &str) -> bool {
        let json = [("Content-Type", "application/json".to_owned())];
        self.answer(status, &json, body.as_bytes()).await
    }

    /// A response that goes on until the client closes the connection
    /// or the source ends: `frames` are what [`Push`]es it writes.
    async fn stream(&mut self, kind: Option<Stream>, content: &str) {
        let _open = kind.map(|kind| Open::new(&self.shared, kind));
        let mut pushes = self.pushes.subscribe();
        let head = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {content}\r\nTransfer-Encoding: chunked\r\n\r\n"
        );
        if self.socket.write_all(head.as_bytes()).await.is_err() {
            return;
        }
        let mut closed = [0_u8; 64];
        loop {
            let bytes = tokio::select! {
                // Nothing more is sent on a stream: a read that ends is
                // the client letting go of it.
                read = self.socket.read(&mut closed) => match read {
                    Ok(read) if read > 0 => continue,
                    _ => return,
                },
                push = pushes.recv() => match (push, kind) {
                    (Ok(Push::Log(frame)), Some(Stream::Logs)) => frame,
                    (Ok(Push::Sample(report)), Some(Stream::Stats)) => report.into_bytes(),
                    (Ok(Push::EndLogs), Some(Stream::Logs)) => {
                        let _ = self.socket.write_all(b"0\r\n\r\n").await;
                        return;
                    }
                    (Err(broadcast::error::RecvError::Closed), _) => return,
                    _ => continue,
                },
            };
            let mut chunk = format!("{:x}\r\n", bytes.len()).into_bytes();
            chunk.extend(bytes);
            chunk.extend(b"\r\n");
            if self.socket.write_all(&chunk).await.is_err() {
                return;
            }
        }
    }

    async fn run(mut self) {
        while let Some(request) = self.request().await {
            let line = format!("{} {}?{}", request.method, request.path, request.query);
            self.shared.asked.lock().unwrap().push(line);
            let parts: Vec<&str> = request.path.split('/').skip(1).collect();
            let goes_on = match (request.method.as_str(), parts.as_slice()) {
                (_, ["_ping"]) => {
                    let version = [("API-Version", "1.47".to_owned())];
                    self.answer("200 OK", &version, b"OK").await
                }
                ("GET", ["version"]) => self.json("200 OK", VERSION).await,
                ("GET", ["info"]) => self.json("200 OK", "{}").await,
                ("GET", ["events"]) => {
                    self.stream(None, "application/json").await;
                    false
                }
                ("GET", ["containers", _, "logs"]) => {
                    let multiplexed = "application/vnd.docker.multiplexed-stream";
                    self.stream(Some(Stream::Logs), multiplexed).await;
                    false
                }
                ("GET", ["containers", _, "stats"]) => {
                    self.stream(Some(Stream::Stats), "application/json").await;
                    false
                }
                ("GET", ["containers", _, "json"]) => {
                    let said = self.shared.inspect.lock().unwrap().clone();
                    let started = self.shared.started.lock().unwrap().clone();
                    let document = said.unwrap_or_else(|| inspected(started.as_deref()));
                    self.json("200 OK", &document).await
                }
                ("GET", ["images", .., "json"]) => self.inspect_image().await,
                ("POST", ["containers", "create"]) => self.create_container(&request).await,
                ("POST", ["containers", _, "start" | "stop" | "rename"])
                | ("DELETE", ["containers", _]) => self.done().await,
                ("GET", ["containers", _, "changes"]) => self.changes().await,
                ("POST", ["commit"]) => self.commit(&request).await,
                (_, ["containers", _, "archive"]) => self.archive(&request).await,
                ("POST", ["containers", _, "exec"]) => self.create_exec(&request).await,
                ("POST", ["exec", _, "start"]) => {
                    self.start_exec().await;
                    false
                }
                ("POST", ["exec", _, "resize"]) => self.resize_exec(&request).await,
                ("GET", ["exec", _, "json"]) => self.inspect_exec().await,
                ("GET", ["containers", _, "export"]) => self.export().await,
                _ => {
                    self.json("404 Not Found", r#"{"message":"not found"}"#)
                        .await
                }
            };
            if !goes_on {
                return;
            }
        }
    }
}

const VERSION: &str = r#"{"Version":"27.0.0","ApiVersion":"1.47","MinAPIVersion":"1.24","Os":"linux","Arch":"amd64"}"#;

#[cfg(test)]
mod tests;

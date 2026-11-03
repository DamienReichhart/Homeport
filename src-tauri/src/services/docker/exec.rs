//! Interactive exec sessions: a TTY process in a container whose output is
//! streamed to the frontend and whose input and size the frontend controls.

use std::{
    collections::HashMap,
    pin::Pin,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
};

use bollard::{
    container::LogOutput,
    exec::{ResizeExecOptions, StartExecOptions, StartExecResults},
    models::ExecConfig,
    Docker,
};
use futures_util::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncWrite, AsyncWriteExt},
    task::AbortHandle,
};

use crate::error::{AppError, AppResult, ErrorCode};

/// Starts bash when the image has it, sh otherwise.
const DEFAULT_SHELL: &str = "if [ -x /bin/bash ]; then exec /bin/bash; else exec /bin/sh; fi";

#[derive(Debug, Clone, Default, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ExecOptions {
    /// The command to run, parsed with shell quoting; the default picks a shell.
    pub command: Option<String>,
    pub user: Option<String>,
    pub working_dir: Option<String>,
    pub cols: u16,
    pub rows: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ExecEvent {
    Output {
        data: String,
    },
    /// `error` explains an exit that means the command couldn't run.
    Exit {
        code: Option<i64>,
        error: Option<AppError>,
    },
    Error {
        error: AppError,
    },
}

/// Decodes a byte stream as UTF-8, holding back a character split across chunks.
#[derive(Default)]
pub struct Utf8Decoder {
    pending: Vec<u8>,
}

impl Utf8Decoder {
    pub fn push(&mut self, bytes: &[u8]) -> String {
        self.pending.extend_from_slice(bytes);
        let mut out = String::new();
        loop {
            match std::str::from_utf8(&self.pending) {
                Ok(s) => {
                    out.push_str(s);
                    self.pending.clear();
                    return out;
                }
                Err(e) => {
                    let valid = e.valid_up_to();
                    out.push_str(std::str::from_utf8(&self.pending[..valid]).expect("validated"));
                    match e.error_len() {
                        // An incomplete character at the end: wait for more bytes.
                        None => {
                            self.pending.drain(..valid);
                            return out;
                        }
                        Some(bad) => {
                            out.push(char::REPLACEMENT_CHARACTER);
                            self.pending.drain(..valid + bad);
                        }
                    }
                }
            }
        }
    }
}

type Output = Pin<Box<dyn Stream<Item = Result<LogOutput, bollard::errors::Error>> + Send>>;

pub struct ExecSession {
    docker: Docker,
    exec_id: String,
    input: tokio::sync::Mutex<Pin<Box<dyn AsyncWrite + Send>>>,
}

impl ExecSession {
    pub async fn write(&self, data: &[u8]) -> AppResult<()> {
        let mut input = self.input.lock().await;
        input.write_all(data).await?;
        input.flush().await?;
        Ok(())
    }

    pub async fn resize(&self, cols: u16, rows: u16) -> AppResult<()> {
        let size = ResizeExecOptions {
            width: cols.max(1),
            height: rows.max(1),
        };
        Ok(self.docker.resize_exec(&self.exec_id, size).await?)
    }
}

/// Creates and attaches to a TTY exec in `container_id`.
pub async fn start(
    docker: &Docker,
    container_id: &str,
    options: ExecOptions,
) -> AppResult<(Arc<ExecSession>, Output)> {
    let cmd = match options.command.as_deref().map(str::trim) {
        Some(c) if !c.is_empty() => shell_words::split(c)
            .map_err(|_| AppError::validation("The command has an unclosed quote."))?,
        _ => vec!["sh".into(), "-c".into(), DEFAULT_SHELL.into()],
    };
    let non_empty = |s: Option<String>| s.filter(|s| !s.trim().is_empty());
    let config = ExecConfig {
        attach_stdin: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        tty: Some(true),
        console_size: Some(vec![options.rows.max(1).into(), options.cols.max(1).into()]),
        env: Some(vec!["TERM=xterm-256color".into()]),
        cmd: Some(cmd),
        user: non_empty(options.user),
        working_dir: non_empty(options.working_dir),
        ..Default::default()
    };
    let exec_id = docker.create_exec(container_id, config).await?.id;
    let started = docker
        .start_exec(
            &exec_id,
            Some(StartExecOptions {
                detach: false,
                tty: true,
                output_capacity: None,
            }),
        )
        .await?;
    let StartExecResults::Attached { output, input } = started else {
        return Err(AppError::new(
            ErrorCode::EngineError,
            "The Docker Engine didn't open the terminal session.",
        ));
    };
    Ok((
        Arc::new(ExecSession {
            docker: docker.clone(),
            exec_id,
            input: tokio::sync::Mutex::new(input),
        }),
        output,
    ))
}

/// Forwards output until the process exits, then reports its exit code.
pub async fn pump(session: Arc<ExecSession>, mut output: Output, sink: impl Fn(ExecEvent) -> bool) {
    let mut decoder = Utf8Decoder::default();
    while let Some(item) = output.next().await {
        match item {
            Ok(chunk) => {
                let data = decoder.push(&chunk.into_bytes());
                if !data.is_empty() && !sink(ExecEvent::Output { data }) {
                    return;
                }
            }
            Err(e) => {
                sink(ExecEvent::Error {
                    error: AppError::from(e).logged("exec session"),
                });
                return;
            }
        }
    }
    let code = session
        .docker
        .inspect_exec(&session.exec_id)
        .await
        .ok()
        .and_then(|i| i.exit_code);
    sink(ExecEvent::Exit {
        code,
        error: exit_error(code),
    });
}

/// Shells answer 127 for a command they can't find and 126 for one they
/// can't run; in a container that usually means a minimal image.
fn exit_error(code: Option<i64>) -> Option<AppError> {
    matches!(code, Some(126 | 127)).then(crate::error::engine::command_not_found)
}

pub type SessionId = u32;

type OpenSessions = HashMap<SessionId, (Arc<ExecSession>, AbortHandle)>;

/// Open exec sessions by id; closing one stops forwarding its output.
#[derive(Default)]
pub struct ExecSessions {
    next: AtomicU32,
    open: Arc<Mutex<OpenSessions>>,
}

impl ExecSessions {
    pub fn open(
        &self,
        session: Arc<ExecSession>,
        output: Output,
        sink: impl Fn(ExecEvent) -> bool + Send + 'static,
    ) -> SessionId {
        let id = self.next.fetch_add(1, Ordering::Relaxed) + 1;
        let open = self.open.clone();
        let pumped = session.clone();
        // Hold the lock across spawn so a session that ends at once can't be
        // removed before it is inserted.
        let mut guard = self.open.lock().expect("exec lock poisoned");
        let task = tokio::spawn(async move {
            pump(pumped, output, sink).await;
            open.lock().expect("exec lock poisoned").remove(&id);
        });
        guard.insert(id, (session, task.abort_handle()));
        id
    }

    pub fn get(&self, id: SessionId) -> AppResult<Arc<ExecSession>> {
        self.open
            .lock()
            .expect("exec lock poisoned")
            .get(&id)
            .map(|(s, _)| s.clone())
            .ok_or_else(|| AppError::not_found("This terminal session has ended."))
    }

    pub fn close(&self, id: SessionId) {
        if let Some((_, task)) = self.open.lock().expect("exec lock poisoned").remove(&id) {
            task.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::services::docker::{containers, testing};

    #[test]
    fn exits_that_mean_the_command_couldnt_run_are_explained() {
        use crate::error::ErrorCode;
        assert_eq!(exit_error(Some(127)).unwrap().code, ErrorCode::NoShell);
        assert_eq!(exit_error(Some(126)).unwrap().code, ErrorCode::NoShell);
        assert!(exit_error(Some(1)).is_none());
        assert!(exit_error(None).is_none());
    }

    #[test]
    fn utf8_split_across_chunks_is_reassembled() {
        let mut d = Utf8Decoder::default();
        let bytes = "héllo ✓".as_bytes();
        let mut out = String::new();
        for b in bytes {
            out.push_str(&d.push(&[*b]));
        }
        assert_eq!(out, "héllo ✓");
        assert_eq!(d.push(&[b'a', 0xFF, b'b']), "a\u{FFFD}b");
    }

    async fn run_shell(docker: Docker) {
        let c = testing::container(
            &testing::docker(),
            &testing::unique("exec"),
            &["sleep", "60"],
        )
        .await;
        containers::act(&docker, &c.id, containers::ContainerAction::Start)
            .await
            .unwrap();
        let (session, output) = start(
            &docker,
            &c.id,
            ExecOptions {
                cols: 80,
                rows: 24,
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let sessions = ExecSessions::default();
        let id = sessions.open(session, output, move |e| tx.send(e).is_ok());

        let s = sessions.get(id).unwrap();
        s.resize(100, 30).await.unwrap();
        s.write(b"echo hel''lo; exit 3\n").await.unwrap();

        let mut output = String::new();
        let exit = tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                match rx.recv().await.unwrap() {
                    ExecEvent::Output { data } => output.push_str(&data),
                    ExecEvent::Exit { code, .. } => return code,
                    ExecEvent::Error { error } => panic!("{error}"),
                }
            }
        })
        .await
        .unwrap();
        assert!(output.contains("hello"), "{output}");
        assert_eq!(exit, Some(3));
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn shells_run_interactively() {
        run_shell(testing::docker()).await;
    }

    #[cfg(unix)]
    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn shells_run_interactively_over_dialed_streams() {
        run_shell(testing::dialed_docker()).await;
    }
}

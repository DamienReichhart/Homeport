//! Interactive terminals in a Pod's container, through the Kubernetes exec
//! API: a TTY process whose output is streamed to the frontend and whose
//! input and size the frontend controls, like `kubectl exec -it`.

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
};

use futures_util::{Sink, SinkExt};
use k8s_openapi::{api::core::v1::Pod, apimachinery::pkg::apis::meta::v1::Status};
use kube::api::{Api, AttachParams, AttachedProcess, TerminalSize};
use serde::Deserialize;
use tokio::{
    io::{AsyncReadExt, AsyncWrite, AsyncWriteExt},
    task::AbortHandle,
};

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    terminal::{ExecEvent, Utf8Decoder, DEFAULT_SHELL},
};

use crate::{
    messages as msg,
    resources::{forbidden_on, ApiType, On},
};

#[derive(Debug, Clone, Default, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PodExecOptions {
    /// The container; the Pod's only one (or default) without.
    pub container: Option<String>,
    /// The command to run, parsed with shell quoting; the default picks a shell.
    pub command: Option<String>,
    pub cols: u16,
    pub rows: u16,
}

/// The command `options` asks for: its own, or bash when the image has it, sh otherwise.
pub fn command(options: &PodExecOptions) -> AppResult<Vec<String>> {
    match options.command.as_deref().map(str::trim) {
        Some(c) if !c.is_empty() => {
            shell_words::split(c).map_err(|_| AppError::validation(msg::COMMAND_UNCLOSED_QUOTE))
        }
        _ => Ok(vec!["sh".into(), "-c".into(), DEFAULT_SHELL.into()]),
    }
}

/// A terminal open in a Pod: where its input goes and how it's resized.
pub struct PodTerminal {
    input: tokio::sync::Mutex<Box<dyn AsyncWrite + Send + Unpin>>,
    size: tokio::sync::Mutex<Box<dyn Sink<TerminalSize, Error = ()> + Send + Unpin>>,
}

impl PodTerminal {
    pub async fn write(&self, data: &[u8]) -> AppResult<()> {
        let mut input = self.input.lock().await;
        input.write_all(data).await?;
        input.flush().await?;
        Ok(())
    }

    pub async fn resize(&self, cols: u16, rows: u16) -> AppResult<()> {
        // A closed channel means the process ended; its exit says so.
        let size = TerminalSize {
            width: cols.max(1),
            height: rows.max(1),
        };
        let _ = self.size.lock().await.send(size).await;
        Ok(())
    }
}

/// The `pods/exec` subresource, as RBAC names it.
fn pod_exec() -> ApiType {
    ApiType {
        group: String::new(),
        version: "v1".into(),
        kind: "Pod".into(),
        plural: "pods/exec".into(),
        namespaced: true,
    }
}

impl On {
    /// Opens a TTY process in `pod`'s container.
    pub async fn exec(
        &self,
        namespace: &str,
        pod: &str,
        options: PodExecOptions,
    ) -> AppResult<(Arc<PodTerminal>, AttachedProcess)> {
        let command = command(&options)?;
        let mut params = AttachParams::interactive_tty();
        params.container = options.container.clone().filter(|c| !c.is_empty());
        let api: Api<Pod> = Api::namespaced(self.client.clone(), namespace);
        let mut process = api
            .exec(pod, command, &params)
            .await
            .map_err(|e| self.exec_failed(namespace, pod, &options, &e))?;
        let (Some(input), Some(size)) = (process.stdin(), process.terminal_size()) else {
            return Err(AppError::new(
                ErrorCode::ClusterApiError,
                msg::TERMINAL_NOT_OPENED.with("pod", pod),
            ));
        };
        let terminal = Arc::new(PodTerminal {
            input: tokio::sync::Mutex::new(Box::new(input)),
            size: tokio::sync::Mutex::new(Box::new(size.sink_map_err(drop))),
        });
        terminal.resize(options.cols, options.rows).await?;
        Ok((terminal, process))
    }

    fn exec_failed(
        &self,
        namespace: &str,
        pod: &str,
        options: &PodExecOptions,
        error: &kube::Error,
    ) -> AppError {
        let container = options.container.as_deref().unwrap_or(pod);
        match error {
            kube::Error::Api(status) if status.code == 403 => {
                forbidden_on("create", &pod_exec(), Some(namespace)).with_detail(&status.message)
            }
            kube::Error::Api(status) if status.code == 400 => AppError::new(
                ErrorCode::ContainerNotRunning,
                msg::EXEC_NOT_RUNNING
                    .with("container", container)
                    .with("pod", pod),
            )
            .with_hint(msg::EXEC_NOT_RUNNING_HINT)
            .with_detail(&status.message),
            e => self.failed_on("create", &pod_exec(), Some(namespace), Some(pod), e),
        }
    }
}

/// How a process ended, from the Status the API server sends last: its exit
/// code, and why it means the command couldn't run, when it does.
pub fn exit(status: Option<&Status>) -> (Option<i64>, Option<AppError>) {
    let Some(status) = status else {
        return (None, None);
    };
    if status.status.as_deref() == Some("Success") {
        return (Some(0), None);
    }
    let code = status
        .details
        .as_ref()
        .and_then(|d| d.causes.as_ref())
        .into_iter()
        .flatten()
        .find(|c| c.reason.as_deref() == Some("ExitCode"))
        .and_then(|c| c.message.as_deref()?.parse().ok());
    let message = status.message.as_deref().unwrap_or_default();
    let missing = message.contains("executable file not found")
        || message.contains("no such file or directory");
    if missing || matches!(code, Some(126 | 127)) {
        let error = AppError::new(ErrorCode::NoShell, msg::NO_SHELL)
            .with_hint(msg::NO_SHELL_HINT)
            .with_command("kubectl debug -it <pod> --image=busybox --target=<container>");
        let error = if message.is_empty() {
            error
        } else {
            error.with_detail(message)
        };
        return (code, Some(error));
    }
    if code.is_none() && !message.is_empty() {
        let error = AppError::new(ErrorCode::ClusterApiError, msg::EXEC_ENDED_WITH_ERROR)
            .with_detail(message);
        return (None, Some(error));
    }
    (code, None)
}

/// Forwards output until the process exits, then reports how.
pub async fn pump(mut process: AttachedProcess, sink: impl Fn(ExecEvent) -> bool) {
    let status = process.take_status();
    if let Some(mut output) = process.stdout() {
        let mut decoder = Utf8Decoder::default();
        let mut buf = vec![0; 8 * 1024];
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
                        error: AppError::from(e).logged("pod terminal"),
                    });
                    return;
                }
            }
        }
    }
    let status = match status {
        Some(status) => status.await,
        None => None,
    };
    let (code, error) = exit(status.as_ref());
    sink(ExecEvent::Exit { code, error });
}

pub type TerminalId = u32;

type Open = HashMap<TerminalId, (Arc<PodTerminal>, AbortHandle)>;

/// Open Pod terminals by id; closing one ends its process.
#[derive(Default)]
pub struct PodTerminals {
    next: AtomicU32,
    open: Arc<Mutex<Open>>,
}

impl PodTerminals {
    pub fn open(
        &self,
        terminal: Arc<PodTerminal>,
        process: AttachedProcess,
        sink: impl Fn(ExecEvent) -> bool + Send + 'static,
    ) -> TerminalId {
        let id = self.next.fetch_add(1, Ordering::Relaxed) + 1;
        let open = self.open.clone();
        // Held across spawn so a process that ends at once can't be removed before it's added.
        let mut guard = self.open.lock().expect("pod terminals lock poisoned");
        let task = tokio::spawn(async move {
            pump(process, sink).await;
            open.lock()
                .expect("pod terminals lock poisoned")
                .remove(&id);
        });
        guard.insert(id, (terminal, task.abort_handle()));
        id
    }

    pub fn get(&self, id: TerminalId) -> AppResult<Arc<PodTerminal>> {
        self.open
            .lock()
            .expect("pod terminals lock poisoned")
            .get(&id)
            .map(|(t, _)| t.clone())
            .ok_or_else(|| AppError::not_found(msg::TERMINAL_ENDED))
    }

    /// Ends a terminal: dropping its process closes the connection, which
    /// ends the command in the container.
    pub fn close(&self, id: TerminalId) {
        if let Some((_, task)) = self
            .open
            .lock()
            .expect("pod terminals lock poisoned")
            .remove(&id)
        {
            task.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::{StatusCause, StatusDetails};

    use super::*;

    fn options(command: Option<&str>) -> PodExecOptions {
        PodExecOptions {
            command: command.map(Into::into),
            ..Default::default()
        }
    }

    #[test]
    fn the_default_command_picks_a_shell() {
        assert_eq!(
            command(&options(None)).unwrap(),
            ["sh", "-c", DEFAULT_SHELL]
        );
        assert_eq!(command(&options(Some("  "))).unwrap()[0], "sh");
    }

    #[test]
    fn a_typed_command_is_split_like_a_shell_would() {
        assert_eq!(
            command(&options(Some("psql -U 'app user' shop"))).unwrap(),
            ["psql", "-U", "app user", "shop"]
        );
        assert!(command(&options(Some("echo 'unclosed"))).is_err());
    }

    fn failure(message: &str, code: Option<&str>) -> Status {
        Status {
            status: Some("Failure".into()),
            message: Some(message.into()),
            reason: Some("NonZeroExitCode".into()),
            details: code.map(|c| StatusDetails {
                causes: Some(vec![StatusCause {
                    reason: Some("ExitCode".into()),
                    message: Some(c.into()),
                    field: None,
                }]),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn the_status_channel_gives_the_exit_code() {
        let success = Status {
            status: Some("Success".into()),
            ..Default::default()
        };
        assert_eq!(exit(Some(&success)), (Some(0), None));
        let (code, error) = exit(Some(&failure(
            "command terminated with non-zero exit code",
            Some("3"),
        )));
        assert_eq!(code, Some(3));
        assert!(error.is_none());
        assert_eq!(exit(None), (None, None));
    }

    #[test]
    fn a_missing_shell_is_explained() {
        let (_, error) = exit(Some(&failure(
            "OCI runtime exec failed: exec: \"sh\": executable file not found in $PATH",
            None,
        )));
        assert_eq!(error.unwrap().code, ErrorCode::NoShell);
        let (code, error) = exit(Some(&failure("exit 127", Some("127"))));
        assert_eq!(code, Some(127));
        assert_eq!(error.unwrap().code, ErrorCode::NoShell);
    }
}

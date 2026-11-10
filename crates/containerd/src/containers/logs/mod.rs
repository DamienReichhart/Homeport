//! A containerd container's logs, where the tool that made it wrote them:
//! nerdctl's JSON lines, Kubernetes' CRI lines, or nowhere (ADR 0020).
//! containerd keeps none itself, so they're read from the Host's machine
//! with `tail`, on the SSH session or on this computer.

use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    logs::{LineSplitter, LogEvent, LogLine, LogOptions, BATCH_EVERY, BATCH_LINES},
};
use homeport_transport::shell_quote;

use crate::{
    api::containerd::services::containers::v1::Container, containers, messages as msg, Containerd,
};

/// Where nerdctl keeps a container's files, and how it logs.
const NERDCTL_STATE_DIR: &str = "nerdctl/state-dir";
const NERDCTL_LOG_CONFIG: &str = "nerdctl/log-config";
/// CRI's record of a container, which names its log file.
const CRI_METADATA: &str = "io.cri-containerd.container.metadata";
const KUBERNETES_POD: &str = "io.kubernetes.pod.name";
const KUBERNETES_POD_NAMESPACE: &str = "io.kubernetes.pod.namespace";
const KUBERNETES_POD_UID: &str = "io.kubernetes.pod.uid";
const KUBERNETES_CONTAINER: &str = "io.kubernetes.container.name";

/// How a log file's lines are written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum ContainerdLogFormat {
    /// Docker's json-file lines, as nerdctl writes them.
    Json,
    /// `<time> <stream> <F|P> <text>`, as the kubelet's CRI writes them.
    Cri,
}

/// Where a container's output was kept, if anywhere.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ContainerdLogSource {
    /// A file on the Host's machine.
    File {
        path: String,
        format: ContainerdLogFormat,
    },
    /// The newest `*.log` of a CRI container's folder.
    Folder { path: String },
    /// A nerdctl log driver Homeport can't read, such as journald or fluentd.
    Driver { driver: String },
    /// Nothing kept it: `ctr run` and most other clients.
    None,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CriRecord {
    metadata: CriMetadata,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CriMetadata {
    #[serde(default)]
    log_path: String,
}

#[derive(Deserialize)]
struct LogConfig {
    #[serde(default)]
    driver: String,
}

/// Where `container`'s output went, from what its creator recorded.
pub(crate) fn source(container: &Container) -> ContainerdLogSource {
    let labels = &container.labels;
    if let Some(any) = container.extensions.get(CRI_METADATA) {
        if let Ok(record) = serde_json::from_slice::<CriRecord>(&any.value) {
            if !record.metadata.log_path.is_empty() {
                return ContainerdLogSource::File {
                    path: record.metadata.log_path,
                    format: ContainerdLogFormat::Cri,
                };
            }
        }
    }
    if let (Some(ns), Some(pod), Some(uid), Some(name)) = (
        labels.get(KUBERNETES_POD_NAMESPACE),
        labels.get(KUBERNETES_POD),
        labels.get(KUBERNETES_POD_UID),
        labels.get(KUBERNETES_CONTAINER),
    ) {
        return ContainerdLogSource::Folder {
            path: format!("/var/log/pods/{ns}_{pod}_{uid}/{name}"),
        };
    }
    if let Some(state_dir) = labels.get(NERDCTL_STATE_DIR) {
        let driver = labels
            .get(NERDCTL_LOG_CONFIG)
            .and_then(|c| serde_json::from_str::<LogConfig>(c).ok())
            .map(|c| c.driver)
            .filter(|d| !d.is_empty())
            .unwrap_or_else(|| "json-file".into());
        if driver != "json-file" {
            return ContainerdLogSource::Driver { driver };
        }
        return ContainerdLogSource::File {
            path: format!(
                "{}/{}-json.log",
                state_dir.trim_end_matches('/'),
                container.id
            ),
            format: ContainerdLogFormat::Json,
        };
    }
    ContainerdLogSource::None
}

/// Where a container's logs are, for the page to say before it reads them.
pub async fn locate(
    client: &Containerd,
    namespace: &str,
    id: &str,
) -> AppResult<ContainerdLogSource> {
    Ok(source(&containers::get(client, namespace, id).await?))
}

/// One line of a log file: its stream, time and text.
#[derive(Debug, PartialEq, Eq)]
struct Parsed {
    stream: String,
    time: String,
    text: String,
    /// A CRI line the next one continues.
    partial: bool,
}

#[derive(Deserialize)]
struct JsonLine {
    #[serde(default)]
    log: String,
    #[serde(default)]
    stream: String,
    #[serde(default)]
    time: String,
}

fn parse(format: ContainerdLogFormat, line: &str) -> Option<Parsed> {
    match format {
        ContainerdLogFormat::Json => {
            let json: JsonLine = serde_json::from_str(line).ok()?;
            Some(Parsed {
                stream: json.stream,
                time: json.time,
                text: json
                    .log
                    .trim_end_matches('\n')
                    .trim_end_matches('\r')
                    .to_owned(),
                partial: false,
            })
        }
        ContainerdLogFormat::Cri => {
            let mut parts = line.splitn(4, ' ');
            let time = parts.next()?.to_owned();
            let stream = parts.next()?.to_owned();
            let tag = parts.next()?;
            let text = parts.next().unwrap_or_default().to_owned();
            Some(Parsed {
                stream,
                time,
                text,
                partial: tag.split(':').next() == Some("P"),
            })
        }
    }
}

/// The `tail` that reads `source`, `None` for a source with no file.
fn command(source: &ContainerdLogSource, options: &LogOptions) -> Option<String> {
    let lines = options
        .tail
        .map_or_else(|| "+1".to_owned(), |n| n.to_string());
    let follow = if options.follow { " -F" } else { "" };
    match source {
        ContainerdLogSource::File { path, .. } => Some(format!(
            "exec tail -n {lines}{follow} -- {}",
            shell_quote(path)
        )),
        ContainerdLogSource::Folder { path } => Some(format!(
            "f=$(ls -1t {}/*.log 2>/dev/null | head -n 1); [ -n \"$f\" ] || {{ echo 'no log file' >&2; exit 2; }}; exec tail -n {lines}{follow} -- \"$f\"",
            shell_quote(path)
        )),
        ContainerdLogSource::Driver { .. } | ContainerdLogSource::None => None,
    }
}

/// Why there's nothing to show for `source`.
fn unavailable(source: &ContainerdLogSource) -> AppError {
    match source {
        ContainerdLogSource::Driver { driver } => AppError::new(
            ErrorCode::ContainerdNoLogs,
            msg::LOGS_DRIVER.with("driver", driver),
        )
        .with_hint(msg::LOGS_DRIVER_HINT.with("driver", driver)),
        _ => AppError::new(ErrorCode::ContainerdNoLogs, msg::LOGS_NOT_RECORDED)
            .with_hint(msg::LOGS_NOT_RECORDED_HINT),
    }
}

/// Unix seconds of an RFC 3339 time, `None` if it can't be read.
fn seconds(time: &str) -> Option<i64> {
    time.parse::<jiff::Timestamp>().ok().map(|t| t.as_second())
}

/// Streams a container's logs to `sink` until the file ends (without
/// `follow`), the sink is gone (returns `false`) or the task is cancelled.
pub async fn stream(
    client: Containerd,
    namespace: String,
    id: String,
    options: LogOptions,
    sink: impl Fn(LogEvent) -> bool,
) {
    let fail = |error: AppError| {
        sink(LogEvent::Error {
            error: error.logged("containerd log stream"),
        });
    };
    let source = match locate(&client, &namespace, &id).await {
        Ok(source) => source,
        Err(e) => return fail(e),
    };
    let (Some(command), format) = (
        command(&source, &options),
        match &source {
            ContainerdLogSource::File { format, .. } => *format,
            _ => ContainerdLogFormat::Cri,
        },
    ) else {
        return fail(unavailable(&source));
    };
    let dialer = client.dialer();
    if !dialer.runs_commands() {
        return fail(
            AppError::new(
                ErrorCode::ContainerdNeedsShell,
                msg::LOGS_NEED_SHELL.with("name", client.name()),
            )
            .with_hint(msg::NEEDS_SHELL_HINT),
        );
    }
    let mut spawned = match dialer.spawn(command).await {
        Ok(spawned) => spawned,
        Err(e) => {
            return fail(
                AppError::new(ErrorCode::ContainerdNeedsShell, msg::LOGS_UNREADABLE)
                    .with_hint(msg::LOGS_UNREADABLE_HINT)
                    .with_cause(&e),
            )
        }
    };
    let mut splitter = LineSplitter::default();
    let mut batch: Vec<LogLine> = Vec::new();
    let mut partial = String::new();
    let mut ticker = tokio::time::interval(BATCH_EVERY);
    let mut buf = vec![0; 32 * 1024];
    let mut push = |batch: &mut Vec<LogLine>, line: String| {
        let Some(parsed) = parse(format, &line) else {
            return;
        };
        if parsed.partial {
            partial.push_str(&parsed.text);
            return;
        }
        let text = std::mem::take(&mut partial) + &parsed.text;
        let wanted = match parsed.stream.as_str() {
            "stderr" => options.stderr,
            _ => options.stdout,
        };
        let recent = options
            .since
            .is_none_or(|since| seconds(&parsed.time).is_none_or(|t| t >= i64::from(since)));
        if wanted && recent {
            batch.push(LogLine {
                source: None,
                stream: parsed.stream,
                text: if options.timestamps {
                    format!("{} {text}", parsed.time)
                } else {
                    text
                },
            });
        }
    };
    loop {
        tokio::select! {
            read = spawned.io.read(&mut buf) => match read {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    for line in splitter.push(&buf[..n]) {
                        push(&mut batch, line);
                    }
                    if batch.len() >= BATCH_LINES
                        && !sink(LogEvent::Lines { lines: std::mem::take(&mut batch) })
                    {
                        return;
                    }
                }
            },
            _ = ticker.tick() => {
                if !batch.is_empty() && !sink(LogEvent::Lines { lines: std::mem::take(&mut batch) }) {
                    return;
                }
            }
        }
    }
    if let Some(line) = splitter.finish() {
        push(&mut batch, line);
    }
    if !batch.is_empty() && !sink(LogEvent::Lines { lines: batch }) {
        return;
    }
    drop(spawned.io);
    let exit = spawned.exit.await;
    if !exit.success() && exit.status.is_some() {
        return fail(
            AppError::new(ErrorCode::ContainerdNoLogs, msg::LOGS_UNREADABLE)
                .with_hint(msg::LOGS_UNREADABLE_HINT)
                .with_detail(exit.stderr.trim()),
        );
    }
    sink(LogEvent::End);
}

#[cfg(test)]
mod tests;

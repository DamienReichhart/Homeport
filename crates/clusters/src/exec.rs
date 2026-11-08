//! Exec credential plugins (`aws eks get-token`, `gke-gcloud-auth-plugin`,
//! `kubelogin`…): run on this machine per the `client.authentication.k8s.io`
//! ExecCredential protocol, v1 and v1beta1. What they print stays in memory.

use std::{
    path::{Path, PathBuf},
    process::Stdio,
    time::{Duration, SystemTime},
};

use base64::Engine as _;
use serde::Deserialize;
use tokio::process::Command;

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    programs,
};

use crate::{client::IssuedCredentials, messages as msg, model::ExecConfig};

/// How long a plugin may take; one waiting on a browser sign-in never ends.
pub const TIMEOUT: Duration = Duration::from_secs(60);
/// Credentials are renewed this long before they expire.
pub const RENEW_MARGIN: Duration = Duration::from_secs(120);
/// The last lines of a failing plugin's stderr kept for Details.
const STDERR_LINES: usize = 20;

/// What a plugin issued, and until when.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Issued {
    pub credentials: IssuedCredentials,
    /// `None` when the plugin gave no expiry: valid until the API refuses it.
    pub expires: Option<SystemTime>,
}

impl Issued {
    /// Whether these credentials should be renewed before being used at `now`.
    pub fn is_due(&self, now: SystemTime) -> bool {
        self.expires
            .is_some_and(|expires| expires <= now + RENEW_MARGIN)
    }
}

/// The Cluster a plugin is asked credentials for, when it wants to know
/// (`provideClusterInfo`).
#[derive(Debug, Clone, Default)]
pub struct ClusterInfo {
    pub server: String,
    pub tls_server_name: Option<String>,
    pub insecure_skip_tls_verify: bool,
    pub certificate_authority: Option<String>,
}

/// Runs the plugin of the Cluster `name`, imported from the kubeconfig
/// context `context`, and reads the credentials it prints.
pub async fn run(
    name: &str,
    context: Option<&str>,
    exec: &ExecConfig,
    cluster: &ClusterInfo,
) -> AppResult<Issued> {
    if exec.interactive_mode.as_deref() == Some("Always") {
        return Err(interactive(&exec.command, context, None));
    }
    let program = locate(&exec.command).ok_or_else(|| missing(exec))?;
    let mut command = Command::new(&program);
    command
        .args(&exec.args)
        .env("PATH", programs::search_path())
        .env("KUBERNETES_EXEC_INFO", exec_info(exec, cluster))
        .envs(exec.env.iter().map(|(k, v)| (k, v)))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    let output = match tokio::time::timeout(TIMEOUT, command.output()).await {
        Ok(Ok(output)) => output,
        Ok(Err(e)) if e.kind() == std::io::ErrorKind::NotFound => return Err(missing(exec)),
        Ok(Err(e)) => return Err(failed(name, &exec.command, None).with_cause(&e)),
        Err(_) => return Err(interactive(&exec.command, context, Some("timed out"))),
    };
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        if needs_terminal(&stderr) {
            return Err(interactive(&exec.command, context, Some(&tail(&stderr))));
        }
        return Err(failed(name, &exec.command, Some(&tail(&stderr))));
    }
    parse(&output.stdout).map_err(|why| failed(name, &exec.command, Some(&why)))
}

/// Where `command` is: as given when it's a path, else on PATH or in a usual
/// install location (a desktop launcher's PATH has neither `aws` nor `gcloud`).
fn locate(command: &str) -> Option<PathBuf> {
    if command.contains('/') || command.contains('\\') {
        let path = Path::new(command);
        return path.is_file().then(|| path.to_owned());
    }
    programs::find(command)
}

/// The `KUBERNETES_EXEC_INFO` a plugin reads: never interactive, since
/// Homeport has no terminal to give it.
fn exec_info(exec: &ExecConfig, cluster: &ClusterInfo) -> String {
    let mut spec = serde_json::json!({ "interactive": false });
    if exec.provide_cluster_info {
        let mut info = serde_json::json!({
            "server": cluster.server,
            "insecure-skip-tls-verify": cluster.insecure_skip_tls_verify,
        });
        if let Some(name) = &cluster.tls_server_name {
            info["tls-server-name"] = name.clone().into();
        }
        if let Some(ca) = &cluster.certificate_authority {
            info["certificate-authority-data"] =
                base64::engine::general_purpose::STANDARD.encode(ca).into();
        }
        spec["cluster"] = info;
    }
    serde_json::json!({
        "apiVersion": exec.api_version,
        "kind": "ExecCredential",
        "spec": spec,
    })
    .to_string()
}

#[derive(Deserialize)]
struct ExecCredential {
    status: Option<ExecStatus>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExecStatus {
    token: Option<String>,
    client_certificate_data: Option<String>,
    client_key_data: Option<String>,
    expiration_timestamp: Option<String>,
}

/// The credentials in a plugin's output, or why there are none.
fn parse(stdout: &[u8]) -> Result<Issued, String> {
    let credential: ExecCredential =
        serde_json::from_slice(stdout).map_err(|e| format!("not an ExecCredential: {e}"))?;
    let status = credential
        .status
        .ok_or_else(|| "the ExecCredential has no status".to_owned())?;
    let credentials = IssuedCredentials {
        token: status.token.filter(|t| !t.is_empty()),
        client_certificate: status.client_certificate_data,
        client_key: status.client_key_data,
    };
    let has_cert = credentials.client_certificate.is_some() && credentials.client_key.is_some();
    if credentials.token.is_none() && !has_cert {
        return Err("the ExecCredential holds neither a token nor a client certificate".into());
    }
    let expires = status
        .expiration_timestamp
        .map(|ts| {
            ts.parse::<jiff::Timestamp>()
                .map(SystemTime::from)
                .map_err(|e| format!("its expirationTimestamp {ts} isn't RFC 3339: {e}"))
        })
        .transpose()?;
    Ok(Issued {
        credentials,
        expires,
    })
}

/// Whether a failing plugin's stderr says it wanted to ask the user something.
fn needs_terminal(stderr: &str) -> bool {
    let lower = stderr.to_lowercase();
    [
        "devicelogin",
        "device code",
        "to sign in, use a web browser",
        "interactive",
        "no tty",
        "not a terminal",
        "inappropriate ioctl",
        "please log in",
        "please run 'gcloud auth login'",
        "aws sso login",
        "token has expired and refresh failed",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

/// The last lines of `stderr`.
fn tail(stderr: &str) -> String {
    let lines: Vec<&str> = stderr.trim_end().lines().collect();
    lines[lines.len().saturating_sub(STDERR_LINES)..].join("\n")
}

fn missing(exec: &ExecConfig) -> AppError {
    let error = AppError::new(
        ErrorCode::ExecPluginMissing,
        msg::EXEC_MISSING.with("command", &exec.command),
    );
    let program = Path::new(&exec.command)
        .file_name()
        .map_or(exec.command.as_str(), |n| n.to_str().unwrap_or_default());
    match (&exec.install_hint, program) {
        (Some(hint), _) => error
            .with_hint(msg::EXEC_MISSING_HINT_KUBECONFIG)
            .with_detail(hint.trim()),
        (None, "aws") => error
            .with_hint(msg::EXEC_MISSING_HINT_AWS)
            .with_command("aws --version"),
        (None, "gke-gcloud-auth-plugin") => error
            .with_hint(msg::EXEC_MISSING_HINT_GKE)
            .with_command("gcloud components install gke-gcloud-auth-plugin"),
        (None, "kubelogin") => error
            .with_hint(msg::EXEC_MISSING_HINT_KUBELOGIN)
            .with_command("az aks install-cli"),
        (None, "az") => error.with_hint(msg::EXEC_MISSING_HINT_AZ),
        (None, _) => error.with_hint(msg::EXEC_MISSING_HINT),
    }
}

fn failed(name: &str, command: &str, detail: Option<&str>) -> AppError {
    let error = AppError::new(
        ErrorCode::ExecPluginFailed,
        msg::EXEC_FAILED.with("command", command).with("name", name),
    )
    .with_hint(msg::EXEC_FAILED_HINT);
    match detail {
        Some(detail) if !detail.is_empty() => error.with_detail(detail),
        _ => error,
    }
}

fn interactive(command: &str, context: Option<&str>, detail: Option<&str>) -> AppError {
    let kubectl = match context {
        Some(context) => format!("kubectl --context {context} get namespaces"),
        None => "kubectl get namespaces".to_owned(),
    };
    let error = AppError::new(
        ErrorCode::ExecPluginInteractive,
        msg::EXEC_INTERACTIVE.with("command", command),
    )
    .with_hint(msg::EXEC_INTERACTIVE_HINT)
    .with_command(kubectl);
    match detail {
        Some(detail) => error.with_detail(detail),
        None => error,
    }
}

#[cfg(test)]
mod tests;

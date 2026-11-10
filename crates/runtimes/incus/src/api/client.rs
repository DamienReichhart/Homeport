//! A connection to one Incus: requests over the Host's dialer, the envelope
//! every answer comes in, and the server it describes itself as.

use std::{sync::Arc, time::Duration};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::{
    http::{self, HeaderName, HeaderValue, Request},
    Dialer, CONNECT_TIMEOUT,
};

use crate::{
    api::{error, operations::Operation},
    messages as msg,
};

/// The API extensions every Incus Homeport connects to has: Incus 6.0
/// and later have them all.
pub const REQUIRED_EXTENSIONS: [&str; 2] = ["instances", "projects"];

/// The project every Incus has, which holds everything until others are made.
pub const DEFAULT_PROJECT: &str = "default";

/// What `GET /1.0` says about the server.
#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct ServerAnswer {
    #[serde(default)]
    pub(crate) api_extensions: Vec<String>,
    #[serde(default)]
    pub(crate) auth: String,
    #[serde(default)]
    pub(crate) environment: Environment,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct Environment {
    #[serde(default)]
    pub(crate) server: String,
    #[serde(default)]
    pub(crate) server_version: String,
    #[serde(default)]
    pub(crate) server_name: String,
    #[serde(default)]
    pub(crate) server_clustered: bool,
    #[serde(default)]
    pub(crate) os_name: String,
    #[serde(default)]
    pub(crate) os_version: String,
    #[serde(default)]
    pub(crate) kernel_architecture: String,
    #[serde(default)]
    pub(crate) architectures: Vec<String>,
    #[serde(default)]
    pub(crate) driver: String,
    #[serde(default)]
    pub(crate) storage: String,
    #[serde(default)]
    pub(crate) project: String,
}

/// The server a connected Incus is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusServer {
    pub version: String,
    /// Its name in a cluster, or the machine's.
    pub server_name: String,
    pub clustered: bool,
    /// "Debian 12", as the server says.
    pub os: String,
    /// The kernel's architecture: "x86_64", "aarch64".
    pub arch: String,
    /// Every architecture Instances can run on.
    pub architectures: Vec<String>,
    /// The instance drivers: "lxc | qemu".
    pub driver: String,
    /// The storage drivers: "zfs | dir".
    pub storage: String,
    /// The project this connection is bound to: the user's own on the
    /// restricted socket, `default` otherwise.
    pub project: String,
    /// The API extensions it has, for features that need one.
    #[serde(skip)]
    pub extensions: Vec<String>,
}

impl IncusServer {
    pub(crate) fn from_answer(answer: &ServerAnswer) -> Self {
        let env = &answer.environment;
        let os = format!("{} {}", env.os_name, env.os_version)
            .trim()
            .to_owned();
        Self {
            version: env.server_version.clone(),
            server_name: env.server_name.clone(),
            clustered: env.server_clustered,
            os,
            arch: env.kernel_architecture.clone(),
            architectures: env.architectures.clone(),
            driver: env.driver.clone(),
            storage: env.storage.clone(),
            project: if env.project.is_empty() {
                DEFAULT_PROJECT.to_owned()
            } else {
                env.project.clone()
            },
            extensions: answer.api_extensions.clone(),
        }
    }

    /// Whether the server has the API extension `name`.
    pub fn has(&self, name: &str) -> bool {
        self.extensions.iter().any(|e| e == name)
    }
}

/// Every answer of Incus's API.
#[derive(Debug, Deserialize)]
pub(crate) struct Envelope {
    #[serde(rename = "type", default)]
    pub(crate) kind: String,
    #[serde(default)]
    pub(crate) error: String,
    #[serde(default)]
    pub(crate) error_code: u16,
    #[serde(default)]
    pub(crate) metadata: serde_json::Value,
}

/// A live connection to a Host's Incus.
#[derive(Clone)]
pub struct Incus {
    dialer: Arc<dyn Dialer>,
    /// The Host's name, for messages.
    name: String,
    server: Arc<IncusServer>,
}

impl std::fmt::Debug for Incus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Incus")
            .field("name", &self.name)
            .field("version", &self.server.version)
            .finish_non_exhaustive()
    }
}

impl Incus {
    /// Asks the Incus at the other end of `dialer` to describe itself, and
    /// refuses what isn't a trusting Incus with the extensions Homeport
    /// needs. `name` is the Host's.
    pub async fn connect(name: &str, dialer: Arc<dyn Dialer>) -> AppResult<Self> {
        let answer = tokio::time::timeout(CONNECT_TIMEOUT, describe(&*dialer))
            .await
            .map_err(|_| AppError::new(ErrorCode::Timeout, msg::TIMED_OUT))?
            .map_err(|e| match e.code {
                ErrorCode::EngineError | ErrorCode::NotFound | ErrorCode::BadRequest => {
                    not_incus(name).with_detail(e.detail.unwrap_or(e.message))
                }
                _ => e,
            })?;
        if answer.environment.server != "incus" {
            return Err(
                not_incus(name).with_detail(format!("server: {:?}", answer.environment.server))
            );
        }
        if answer.auth != "trusted" {
            return Err(AppError::new(
                ErrorCode::TlsClientRejected,
                msg::UNTRUSTED.with("name", name),
            )
            .with_hint(msg::UNTRUSTED_HINT));
        }
        let server = IncusServer::from_answer(&answer);
        if let Some(missing) = REQUIRED_EXTENSIONS.iter().find(|e| !server.has(e)) {
            return Err(extension_missing(name, missing));
        }
        Ok(Self {
            dialer,
            name: name.to_owned(),
            server: Arc::new(server),
        })
    }

    /// The Host's name, for messages.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The server, as it described itself on connecting.
    pub fn server(&self) -> &IncusServer {
        &self.server
    }

    /// What reaches the Incus API.
    pub fn dialer(&self) -> &Arc<dyn Dialer> {
        &self.dialer
    }

    /// Fails with `IncusExtensionMissing` when the server lacks `extension`.
    pub fn require(&self, extension: &str) -> AppResult<()> {
        if self.server.has(extension) {
            Ok(())
        } else {
            Err(extension_missing(&self.name, extension))
        }
    }

    /// Answers quickly while the connection works.
    pub async fn ping(&self) -> AppResult<()> {
        tokio::time::timeout(CONNECT_TIMEOUT, describe(&*self.dialer))
            .await
            .map_err(|_| AppError::new(ErrorCode::Timeout, msg::TIMED_OUT))?
            .map(drop)
    }

    /// Sends `request` and reads the envelope of the answer, failing with
    /// Incus's classified error.
    pub(crate) async fn envelope(&self, request: Request) -> AppResult<(Envelope, http::Response)> {
        let response = http::send(&*self.dialer, request).await?;
        let envelope = read_envelope(&response)?;
        Ok((envelope, response))
    }

    /// Sends `request` and reads a synchronous answer's metadata as `T`.
    pub(crate) async fn sync<T: DeserializeOwned>(&self, request: Request) -> AppResult<T> {
        let (envelope, _) = self.envelope(request).await?;
        metadata(envelope.metadata)
    }

    /// Like [`Self::sync`], with the answer's `ETag` beside it.
    pub(crate) async fn sync_tagged<T: DeserializeOwned>(
        &self,
        request: Request,
    ) -> AppResult<(T, Option<String>)> {
        let (envelope, response) = self.envelope(request).await?;
        let etag = response.header("etag").map(str::to_owned);
        Ok((metadata(envelope.metadata)?, etag))
    }

    /// Sends `request`, which Incus may answer with an operation, and
    /// waits for it to end; a synchronous answer counts as done.
    pub(crate) async fn run(
        &self,
        project: &str,
        request: Request,
        timeout: Duration,
    ) -> AppResult<Operation> {
        self.run_with(project, request, timeout, |_| {}).await
    }

    /// Like [`Self::run`], calling `progress` with each state of the
    /// operation while it runs.
    pub(crate) async fn run_with(
        &self,
        project: &str,
        request: Request,
        timeout: Duration,
        progress: impl FnMut(&Operation),
    ) -> AppResult<Operation> {
        match self.start(request).await? {
            Some(operation) => {
                crate::api::operations::wait(self, project, operation, timeout, progress).await
            }
            None => Ok(Operation::done()),
        }
    }

    /// Sends `request` and hands back the operation it started; `None`
    /// for a synchronous answer.
    pub(crate) async fn start(&self, request: Request) -> AppResult<Option<Operation>> {
        let (envelope, _) = self.envelope(request).await?;
        if envelope.kind == "async" {
            Ok(Some(metadata(envelope.metadata)?))
        } else {
            Ok(None)
        }
    }
}

/// `GET /1.0`, read as Incus's server description.
async fn describe(dialer: &dyn Dialer) -> AppResult<ServerAnswer> {
    let response = http::send(dialer, Request::get("/1.0")).await?;
    let envelope = read_envelope(&response)?;
    metadata(envelope.metadata)
}

/// The envelope of `response`, or the error it reports.
pub(crate) fn read_envelope(response: &http::Response) -> AppResult<Envelope> {
    let envelope: Envelope = serde_json::from_slice(&response.body).map_err(|e| {
        let text = String::from_utf8_lossy(&response.body);
        if response.status.is_success() {
            unreadable(&e)
        } else {
            error::answer(response.status.as_u16(), text.trim())
        }
    })?;
    if envelope.kind == "error" || !response.status.is_success() {
        let status = if envelope.error_code != 0 {
            envelope.error_code
        } else {
            response.status.as_u16()
        };
        return Err(error::answer(status, &envelope.error));
    }
    Ok(envelope)
}

/// `value` read as `T`, or why it can't be.
pub(crate) fn metadata<T: DeserializeOwned>(value: serde_json::Value) -> AppResult<T> {
    serde_json::from_value(value).map_err(|e| unreadable(&e))
}

fn unreadable(error: &(dyn std::error::Error + 'static)) -> AppError {
    AppError::new(ErrorCode::EngineError, msg::UNREADABLE)
        .with_hint(msg::DETAILS_HINT)
        .with_cause(error)
}

fn not_incus(name: &str) -> AppError {
    AppError::new(
        ErrorCode::RemoteEngineUnavailable,
        msg::NOT_INCUS.with("name", name),
    )
    .with_hint(msg::NOT_INCUS_HINT)
}

fn extension_missing(name: &str, extension: &str) -> AppError {
    AppError::new(
        ErrorCode::IncusExtensionMissing,
        msg::EXTENSION_MISSING
            .with("name", name)
            .with("extension", extension),
    )
    .with_hint(msg::EXTENSION_MISSING_HINT)
}

/// `path` with `project=` in its query, for project-scoped resources.
pub(crate) fn scoped(path: &str, project: &str) -> String {
    let separator = if path.contains('?') { '&' } else { '?' };
    format!("{path}{separator}project={}", http::encode(project))
}

/// A path segment, percent-encoded.
pub(crate) fn segment(value: &str) -> String {
    http::encode(value)
}

/// `request` with `If-Match: etag`, so Incus refuses it if the object
/// changed since it was read.
pub(crate) fn if_match(request: Request, etag: Option<&str>) -> Request {
    match etag.and_then(|e| HeaderValue::from_str(e).ok()) {
        Some(value) => request.header(HeaderName::from_static("if-match"), value),
        None => request,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response(status: u16, body: &str) -> http::Response {
        http::Response {
            status: http::StatusCode::from_u16(status).unwrap(),
            headers: Default::default(),
            body: body.to_owned().into(),
        }
    }

    #[test]
    fn errors_come_in_the_envelope() {
        let e = read_envelope(&response(
            404,
            r#"{"type":"error","status":"","status_code":0,"operation":"","error_code":404,"error":"Instance not found","metadata":null}"#,
        ))
        .unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
        assert!(e.detail.unwrap().contains("Instance not found"));

        let ok = read_envelope(&response(
            200,
            r#"{"type":"sync","status":"Success","status_code":200,"operation":"","error_code":0,"error":"","metadata":["/1.0/instances/web"]}"#,
        ))
        .unwrap();
        assert_eq!(ok.kind, "sync");
        let names: Vec<String> = metadata(ok.metadata).unwrap();
        assert_eq!(names, ["/1.0/instances/web"]);
    }

    #[test]
    fn a_server_reads_from_get_1_0() {
        // Trimmed from Incus 6.0's GET /1.0 on the admin socket.
        let answer: ServerAnswer = serde_json::from_value(serde_json::json!({
            "api_extensions": ["instances", "projects", "network_leases"],
            "api_status": "stable",
            "api_version": "1.0",
            "auth": "trusted",
            "environment": {
                "architectures": ["x86_64", "i686"],
                "driver": "lxc | qemu",
                "kernel_architecture": "x86_64",
                "os_name": "Debian GNU/Linux",
                "os_version": "12",
                "project": "default",
                "server": "incus",
                "server_clustered": false,
                "server_name": "lab",
                "server_version": "6.0.4",
                "storage": "zfs"
            }
        }))
        .unwrap();
        let server = IncusServer::from_answer(&answer);
        assert_eq!(server.version, "6.0.4");
        assert_eq!(server.os, "Debian GNU/Linux 12");
        assert_eq!(server.project, "default");
        assert!(server.has("network_leases"));
        assert!(!server.has("console"));
    }

    #[test]
    fn project_scoped_paths_name_their_project() {
        assert_eq!(
            scoped("/1.0/instances?recursion=2", "my project"),
            "/1.0/instances?recursion=2&project=my%20project"
        );
        assert_eq!(
            scoped("/1.0/profiles", "default"),
            "/1.0/profiles?project=default"
        );
    }
}

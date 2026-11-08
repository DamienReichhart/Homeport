//! Plain HTTP/1.1 over a stream from a [`Dialer`], for the APIs bollard
//! doesn't speak: Podman's own (libpod) on the Engine socket, and later
//! Incus's REST API. Each request gets its own connection, as bollard's do,
//! and failures are classified like the Docker Engine's answers.

use std::{error::Error as StdError, pin::Pin, task::Poll, time::Duration};

use bytes::{Bytes, BytesMut};
use futures_util::Stream;
use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Body, Incoming},
    client::conn::http1::SendRequest,
    header::{self, HeaderMap, HeaderName, HeaderValue},
    upgrade::Upgraded,
    Method,
};
use hyper_util::rt::TokioIo;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use homeport_core::{
    error::{engine::from_message, AppError, AppResult, ErrorCode},
    messages as core_msg,
};

use crate::Dialer;

pub use hyper::StatusCode;

/// How long a request may take before its answer starts.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);

/// Opens a connection through `dialer` and runs its HTTP/1.1 handshake. The
/// connection is driven by a task of its own, with upgrades, so it can
/// carry the streams of exec and attach.
pub(crate) async fn handshake<B>(
    dialer: &dyn Dialer,
) -> Result<SendRequest<B>, bollard::errors::Error>
where
    B: Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    let stream = dialer.dial().await?;
    let (sender, conn) = hyper::client::conn::http1::handshake(TokioIo::new(stream)).await?;
    tokio::spawn(async move {
        if let Err(e) = conn.with_upgrades().await {
            log::debug!("engine connection closed: {e}");
        }
    });
    Ok(sender)
}

/// Sends `req` in origin-form with an explicit Host header, as Go's servers
/// (the Docker Engine, Podman, Incus) require.
pub(crate) fn origin_form<B>(req: &mut hyper::Request<B>) {
    let path = req
        .uri()
        .path_and_query()
        .map(|p| p.as_str().to_owned())
        .unwrap_or_else(|| "/".into());
    *req.uri_mut() = path.parse().expect("a path is a valid URI");
    req.headers_mut()
        .insert(header::HOST, HeaderValue::from_static("docker"));
}

/// One request: a method, a path with its query, headers and a body.
#[derive(Debug, Clone)]
pub struct Request {
    method: Method,
    path: String,
    headers: Vec<(HeaderName, HeaderValue)>,
    body: Bytes,
}

impl Request {
    pub fn new(method: Method, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            headers: Vec::new(),
            body: Bytes::new(),
        }
    }

    pub fn get(path: impl Into<String>) -> Self {
        Self::new(Method::GET, path)
    }

    pub fn post(path: impl Into<String>) -> Self {
        Self::new(Method::POST, path)
    }

    pub fn delete(path: impl Into<String>) -> Self {
        Self::new(Method::DELETE, path)
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn body_bytes(&self) -> &Bytes {
        &self.body
    }

    pub fn header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.headers.push((name, value));
        self
    }

    /// `body` with its content type.
    pub fn body(self, content_type: &'static str, body: impl Into<Bytes>) -> Self {
        let mut req = self.header(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
        req.body = body.into();
        req
    }

    /// `value` as a JSON body.
    pub fn json(self, value: &impl Serialize) -> AppResult<Self> {
        Ok(self.body("application/json", serde_json::to_vec(value)?))
    }

    fn build(self) -> hyper::Request<Full<Bytes>> {
        let mut req = hyper::Request::new(Full::new(self.body));
        *req.method_mut() = self.method;
        *req.uri_mut() = self
            .path
            .parse()
            .unwrap_or_else(|_| hyper::Uri::from_static("/"));
        for (name, value) in self.headers {
            req.headers_mut().append(name, value);
        }
        origin_form(&mut req);
        req
    }
}

/// An answer read whole.
#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Bytes,
}

impl Response {
    /// A header's value, when it is text.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name)?.to_str().ok()
    }

    /// The answer, or the classified failure it reports for a status that
    /// isn't a success.
    pub fn ok(self) -> AppResult<Self> {
        if self.status.is_success() {
            Ok(self)
        } else {
            Err(status_error(self.status, &self.body))
        }
    }

    /// The body read as JSON.
    pub fn json<T: DeserializeOwned>(&self) -> AppResult<T> {
        serde_json::from_slice(&self.body).map_err(|e| unreadable(&e))
    }
}

/// What Docker and Podman put in an error's body: `message`, and Podman's
/// `cause` beside it.
#[derive(Deserialize)]
struct ErrorBody {
    message: Option<String>,
    cause: Option<String>,
}

/// The failure an Engine reports with `status`, classified by its message
/// like bollard's are; its own words stay in the detail.
pub fn status_error(status: StatusCode, body: &[u8]) -> AppError {
    let message = serde_json::from_slice::<ErrorBody>(body)
        .ok()
        .and_then(|b| b.message.or(b.cause))
        .unwrap_or_else(|| String::from_utf8_lossy(body).trim().to_owned());
    from_message(Some(status.as_u16()), &message)
        .with_detail(format!("HTTP {}: {message}", status.as_u16()))
}

/// An answer that isn't the JSON expected.
fn unreadable(error: &(dyn StdError + 'static)) -> AppError {
    AppError::new(ErrorCode::EngineError, core_msg::ENGINE_UNREADABLE)
        .with_hint(core_msg::ENGINE_UNREADABLE_HINT)
        .with_cause(error)
}

/// Sends `request` and waits for the head of its answer.
async fn start(dialer: &dyn Dialer, request: Request) -> AppResult<hyper::Response<Incoming>> {
    let send = async {
        let mut sender = handshake::<Full<Bytes>>(dialer).await?;
        Ok::<_, bollard::errors::Error>(sender.send_request(request.build()).await?)
    };
    match tokio::time::timeout(REQUEST_TIMEOUT, send).await {
        Ok(result) => result.map_err(AppError::from),
        Err(_) => Err(AppError::from(bollard::errors::Error::RequestTimeoutError)),
    }
}

/// Sends `request` and reads its whole answer, whatever its status.
pub async fn send(dialer: &dyn Dialer, request: Request) -> AppResult<Response> {
    let answer = start(dialer, request).await?;
    let (head, body) = answer.into_parts();
    let body = tokio::time::timeout(REQUEST_TIMEOUT, body.collect())
        .await
        .map_err(|_| AppError::from(bollard::errors::Error::RequestTimeoutError))?
        .map_err(|e| AppError::from(bollard::errors::Error::from(e)))?
        .to_bytes();
    Ok(Response {
        status: head.status,
        headers: head.headers,
        body,
    })
}

/// Sends `request` and reads its successful answer as JSON.
pub async fn json<T: DeserializeOwned>(dialer: &dyn Dialer, request: Request) -> AppResult<T> {
    send(dialer, request).await?.ok()?.json()
}

/// The lines of a streamed answer, such as Podman's line-delimited JSON.
pub struct Lines {
    body: Incoming,
    buffer: BytesMut,
    done: bool,
}

impl Stream for Lines {
    type Item = AppResult<Bytes>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(end) = self.buffer.iter().position(|&b| b == b'\n') {
                let line = self.buffer.split_to(end + 1);
                let line = line.strip_suffix(b"\n").unwrap_or(&line);
                let line = line.strip_suffix(b"\r").unwrap_or(line);
                if line.is_empty() {
                    continue;
                }
                return Poll::Ready(Some(Ok(Bytes::copy_from_slice(line))));
            }
            if self.done {
                if self.buffer.is_empty() {
                    return Poll::Ready(None);
                }
                let rest = self.buffer.split().freeze();
                return Poll::Ready(Some(Ok(rest)));
            }
            match Pin::new(&mut self.body).poll_frame(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) => self.done = true,
                Poll::Ready(Some(Err(e))) => {
                    self.done = true;
                    self.buffer.clear();
                    return Poll::Ready(Some(Err(AppError::from(bollard::errors::Error::from(e)))));
                }
                Poll::Ready(Some(Ok(frame))) => {
                    if let Ok(data) = frame.into_data() {
                        self.buffer.extend_from_slice(&data);
                    }
                }
            }
        }
    }
}

/// Sends `request` and streams its successful answer line by line.
pub async fn lines(dialer: &dyn Dialer, request: Request) -> AppResult<Lines> {
    let answer = start(dialer, request).await?;
    if !answer.status().is_success() {
        let status = answer.status();
        let body = answer
            .into_body()
            .collect()
            .await
            .map(|b| b.to_bytes())
            .unwrap_or_default();
        return Err(status_error(status, &body));
    }
    Ok(Lines {
        body: answer.into_body(),
        buffer: BytesMut::new(),
        done: false,
    })
}

/// Sends `request` asking to switch protocols (`Upgrade: <protocol>`) and
/// hands back the head of the answer and the connection, kept open for
/// whatever speaks next on it (a websocket, a raw stream).
pub async fn upgrade(
    dialer: &dyn Dialer,
    request: Request,
    protocol: &'static str,
) -> AppResult<(StatusCode, HeaderMap, TokioIo<Upgraded>)> {
    let request = request
        .header(header::CONNECTION, HeaderValue::from_static("Upgrade"))
        .header(header::UPGRADE, HeaderValue::from_static(protocol));
    let answer = start(dialer, request).await?;
    let status = answer.status();
    if status != StatusCode::SWITCHING_PROTOCOLS && !status.is_success() {
        let body = answer
            .into_body()
            .collect()
            .await
            .map(|b| b.to_bytes())
            .unwrap_or_default();
        return Err(status_error(status, &body));
    }
    let headers = answer.headers().clone();
    let upgraded = hyper::upgrade::on(answer)
        .await
        .map_err(|e| AppError::from(bollard::errors::Error::from(e)))?;
    Ok((status, headers, TokioIo::new(upgraded)))
}

/// `value` percent-encoded for a path segment or a query value.
pub fn encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

#[cfg(all(test, unix))]
mod tests;

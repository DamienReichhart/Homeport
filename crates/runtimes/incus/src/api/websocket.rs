//! Incus's websockets (events, and the data and control channels of exec
//! and console), opened over a fresh stream from the Host's dialer.

use std::time::Duration;

use tokio_tungstenite::{
    client_async,
    tungstenite::{self, http::Uri},
    WebSocketStream,
};

use homeport_core::error::{reach::classified_cause, AppError, AppResult, ErrorCode};
use homeport_transport::{BoxStream, CONNECT_TIMEOUT};

use crate::{
    api::{client::Incus, error},
    messages as msg,
};

pub(crate) type Socket = WebSocketStream<BoxStream>;

/// Opens the websocket at `path` (with its query) on `incus`.
pub(crate) async fn open(incus: &Incus, path: &str) -> AppResult<Socket> {
    let uri: Uri = format!("ws://incus{path}")
        .parse()
        .map_err(|e| AppError::validation(msg::UNREADABLE).with_detail(e))?;
    let connect = async {
        let stream = incus.dialer().dial().await.map_err(io_error)?;
        client_async(uri, stream).await.map_err(handshake_error)
    };
    let (socket, _) = tokio::time::timeout(CONNECT_TIMEOUT + Duration::from_secs(5), connect)
        .await
        .map_err(|_| AppError::new(ErrorCode::Timeout, msg::TIMED_OUT))??;
    Ok(socket)
}

fn io_error(e: std::io::Error) -> AppError {
    match classified_cause(&e) {
        Some(classified) => classified.clone(),
        None => AppError::from(e),
    }
}

/// A refused upgrade carries Incus's answer in its body.
fn handshake_error(e: tungstenite::Error) -> AppError {
    match e {
        tungstenite::Error::Http(response) => {
            let status = response.status().as_u16();
            let body = response
                .body()
                .as_deref()
                .map(String::from_utf8_lossy)
                .unwrap_or_default();
            let message = serde_json::from_str::<serde_json::Value>(&body)
                .ok()
                .and_then(|v| v.get("error")?.as_str().map(str::to_owned))
                .unwrap_or_else(|| body.trim().to_owned());
            error::answer(status, &message)
        }
        tungstenite::Error::Io(io) => io_error(io),
        other => AppError::new(ErrorCode::ConnectionLost, msg::FAILED)
            .with_hint(msg::DETAILS_HINT)
            .with_cause(&other),
    }
}

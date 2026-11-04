//! One SSH session per Host, and a `direct-streamlocal` channel to the remote
//! Engine socket per Engine connection.

use std::{io, sync::Arc, time::Duration};

use russh::{
    client::{self, Handle},
    keys::PrivateKeyWithHashAlg,
};
use tokio::sync::Mutex;

use homeport_core::error::{AppError, ErrorCode};

use super::{
    auth::{auth_refused, authenticate_with_agent, Credentials},
    host_key::{host_key_error, ServerKeyCheck},
    socket::{probe_socket, socket_unavailable},
};
use crate::{messages as msg, reach, BoxFuture, BoxStream, ConnectError, Dialer, CONNECT_TIMEOUT};

pub struct SshTarget {
    /// The Host's name and SSH address, for messages.
    pub label: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub socket_path: String,
    pub credentials: Credentials,
    pub trusted_host_key: Option<String>,
}

/// Keeps one SSH session per Host and opens a channel per Engine connection.
pub struct SshDialer {
    target: SshTarget,
    session: Mutex<Option<Handle<ServerKeyCheck>>>,
}

impl SshDialer {
    pub fn new(target: SshTarget) -> Self {
        Self {
            target,
            session: Mutex::new(None),
        }
    }

    async fn open_session(&self) -> Result<Handle<ServerKeyCheck>, ConnectError> {
        let t = &self.target;
        let config = Arc::new(client::Config {
            keepalive_interval: Some(Duration::from_secs(15)),
            keepalive_max: 3,
            ..Default::default()
        });
        let presented = Arc::new(std::sync::Mutex::new(None));
        let handler = ServerKeyCheck {
            trusted: t.trusted_host_key.clone(),
            presented: presented.clone(),
        };
        let connecting = client::connect(config, (t.host.as_str(), t.port), handler);
        let result = tokio::time::timeout(CONNECT_TIMEOUT, connecting)
            .await
            .map_err(|_| {
                ConnectError::Unreachable(AppError::new(
                    ErrorCode::Timeout,
                    msg::NETWORK_TIMEOUT.with("host", &t.label),
                ))
            })?;

        let mut session = match result {
            Ok(session) => session,
            Err(e) => {
                let presented = presented.lock().expect("lock poisoned").take();
                return Err(match presented {
                    Some(key) => host_key_error(t.trusted_host_key.as_deref(), &key),
                    None => ConnectError::from(reach::ssh(&t.label, &e)),
                });
            }
        };

        let authenticated = match &t.credentials {
            Credentials::Key(key) => {
                let hash = session
                    .best_supported_rsa_hash()
                    .await
                    .map_err(|e| ConnectError::from(reach::ssh(&t.label, &e)))?
                    .flatten();
                session
                    .authenticate_publickey(&t.user, PrivateKeyWithHashAlg::new(key.clone(), hash))
                    .await
                    .map_err(|e| ConnectError::from(reach::ssh(&t.label, &e)))?
                    .success()
            }
            Credentials::Agent => authenticate_with_agent(&mut session, &t.user, &t.label).await?,
        };
        if !authenticated {
            return Err(ConnectError::Unauthorized(auth_refused(t)));
        }
        Ok(session)
    }

    async fn open_channel(&self) -> Result<BoxStream, ConnectError> {
        let mut session = self.session.lock().await;
        if session.as_ref().is_none_or(|s| s.is_closed()) {
            *session = Some(self.open_session().await?);
        }
        let session = session.as_ref().expect("session was just opened");
        match session
            .channel_open_direct_streamlocal(self.target.socket_path.as_str())
            .await
        {
            Ok(channel) => Ok(Box::new(channel.into_stream())),
            Err(e) => {
                // The refusal doesn't say why, so ask the server.
                let probe = probe_socket(session, &self.target.socket_path).await;
                Err(ConnectError::from(socket_unavailable(
                    &self.target,
                    probe,
                    &e,
                )))
            }
        }
    }
}

impl Dialer for SshDialer {
    fn connect(&self) -> BoxFuture<'_, Result<(), ConnectError>> {
        Box::pin(async move { self.open_channel().await.map(drop) })
    }

    fn dial(&self) -> BoxFuture<'_, io::Result<BoxStream>> {
        Box::pin(async move {
            self.open_channel()
                .await
                // Keep the classification for whoever reads the failed request.
                .map_err(|e| io::Error::new(io::ErrorKind::ConnectionRefused, AppError::from(e)))
        })
    }
}

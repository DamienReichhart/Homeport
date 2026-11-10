//! One SSH session per Host, chained through its bastions (ADR 0012), and a
//! `direct-streamlocal` channel to the remote Engine socket per Engine
//! connection.

use std::{io, sync::Arc, time::Duration};

use russh::{
    client::{self, Handle},
    keys::PrivateKeyWithHashAlg,
};
use tokio::{
    io::AsyncWriteExt,
    sync::{oneshot, Mutex},
};

use homeport_core::{
    error::{AppError, ErrorCode},
    host::EngineKind,
};

use super::{
    auth::{auth_refused, authenticate_with_agent, Credentials},
    host_key::{host_key_error, ServerKeyCheck},
    socket::{probe_socket, socket_unavailable},
};
use crate::{
    dialer::Guarded, messages as msg, reach, BoxFuture, BoxStream, CommandOutput, ConnectError,
    Dialer, Spawned, CONNECT_TIMEOUT,
};

pub struct SshTarget {
    /// The Host's name and SSH address, for messages.
    pub label: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub socket_path: String,
    pub credentials: Credentials,
    pub trusted_host_key: Option<String>,
    /// The servers to go through first, in order.
    pub bastions: Vec<SshHop>,
    /// The kind of Engine behind the socket, for explaining a refusal.
    pub engine: EngineKind,
}

/// A bastion on the way to a Host.
pub struct SshHop {
    /// The bastion and the Host it leads to, for messages.
    pub label: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub credentials: Credentials,
    pub trusted_host_key: Option<String>,
}

/// One server of the chain, as the dialer walks it.
struct Hop<'a> {
    label: &'a str,
    host: &'a str,
    port: u16,
    user: &'a str,
    credentials: &'a Credentials,
    trusted_host_key: Option<&'a str>,
    /// The bastion's address, `None` for the Host.
    bastion: Option<String>,
}

impl<'a> From<&'a SshHop> for Hop<'a> {
    fn from(h: &'a SshHop) -> Self {
        Self {
            label: &h.label,
            host: &h.host,
            port: h.port,
            user: &h.user,
            credentials: &h.credentials,
            trusted_host_key: h.trusted_host_key.as_deref(),
            bastion: Some(homeport_core::host::ssh_address(&h.host, h.port)),
        }
    }
}

impl SshTarget {
    /// Every server Homeport logs in to, from the first bastion to the Host.
    fn hops(&self) -> Vec<Hop<'_>> {
        let host = Hop {
            label: &self.label,
            host: &self.host,
            port: self.port,
            user: &self.user,
            credentials: &self.credentials,
            trusted_host_key: self.trusted_host_key.as_deref(),
            bastion: None,
        };
        self.bastions
            .iter()
            .map(Hop::from)
            .chain(std::iter::once(host))
            .collect()
    }
}

/// The sessions of a chain; the bastions' are kept open for the Host's.
struct Chain {
    sessions: Vec<Handle<ServerKeyCheck>>,
}

impl Chain {
    fn host(&self) -> &Handle<ServerKeyCheck> {
        self.sessions.last().expect("a chain ends with the Host")
    }

    fn is_closed(&self) -> bool {
        self.sessions.iter().any(Handle::is_closed)
    }
}

/// Keeps one SSH session per Host and opens a channel per Engine connection.
pub struct SshDialer {
    target: SshTarget,
    chain: Mutex<Option<Chain>>,
}

impl SshDialer {
    pub fn new(target: SshTarget) -> Self {
        Self {
            target,
            chain: Mutex::new(None),
        }
    }

    /// Logs in to each server in turn, each next one through a `direct-tcpip`
    /// channel of the previous session.
    async fn open_chain(&self) -> Result<Chain, ConnectError> {
        let hops = self.target.hops();
        let mut sessions: Vec<Handle<ServerKeyCheck>> = Vec::with_capacity(hops.len());
        for (i, hop) in hops.iter().enumerate() {
            let through = i.checked_sub(1).map(|p| (&hops[p], &sessions[p]));
            sessions.push(open_session(hop, through).await?);
        }
        Ok(Chain { sessions })
    }

    async fn open_channel(&self) -> Result<BoxStream, ConnectError> {
        let mut chain = self.chain.lock().await;
        if chain.as_ref().is_none_or(Chain::is_closed) {
            *chain = Some(self.open_chain().await?);
        }
        let session = chain.as_ref().expect("the chain was just opened").host();
        match session
            .channel_open_direct_streamlocal(self.target.socket_path.as_str())
            .await
        {
            Ok(channel) => Ok(Box::new(channel.into_stream())),
            Err(e) => {
                // The refusal doesn't say why, so ask the server.
                let probe =
                    probe_socket(session, &self.target.socket_path, self.target.engine).await;
                Err(ConnectError::from(socket_unavailable(
                    &self.target,
                    probe,
                    &e,
                )))
            }
        }
    }
}

fn config() -> Arc<client::Config> {
    Arc::new(client::Config {
        keepalive_interval: Some(Duration::from_secs(15)),
        keepalive_max: 3,
        ..Default::default()
    })
}

/// Opens and authenticates a session to `hop`: over TCP from this computer,
/// or through the session of the previous server.
async fn open_session(
    hop: &Hop<'_>,
    through: Option<(&Hop<'_>, &Handle<ServerKeyCheck>)>,
) -> Result<Handle<ServerKeyCheck>, ConnectError> {
    let presented = Arc::new(std::sync::Mutex::new(None));
    let handler = ServerKeyCheck {
        trusted: hop.trusted_host_key.map(str::to_owned),
        presented: presented.clone(),
    };
    let connecting = async {
        match through {
            None => client::connect(config(), (hop.host, hop.port), handler).await,
            Some((previous, session)) => {
                let channel = session
                    .channel_open_direct_tcpip(hop.host, u32::from(hop.port), "127.0.0.1", 0)
                    .await
                    .map_err(|e| Err(forward_failed(previous, hop, &e)))?;
                client::connect_stream(config(), channel.into_stream(), handler).await
            }
        }
        .map_err(Ok)
    };
    let result = tokio::time::timeout(CONNECT_TIMEOUT, connecting)
        .await
        .map_err(|_| {
            ConnectError::Unreachable(AppError::new(
                ErrorCode::Timeout,
                msg::NETWORK_TIMEOUT.with("host", hop.label),
            ))
        })?;

    let mut session = match result {
        Ok(session) => session,
        Err(Err(classified)) => return Err(classified),
        Err(Ok(e)) => {
            let presented = presented.lock().expect("lock poisoned").take();
            return Err(match presented {
                Some(key) => host_key_error(hop.trusted_host_key, &key, hop.bastion.as_deref()),
                None => ConnectError::from(reach::ssh(hop.label, &e)),
            });
        }
    };

    let authenticated = match hop.credentials {
        Credentials::Key(key) => {
            let hash = session
                .best_supported_rsa_hash()
                .await
                .map_err(|e| ConnectError::from(reach::ssh(hop.label, &e)))?
                .flatten();
            session
                .authenticate_publickey(hop.user, PrivateKeyWithHashAlg::new(key.clone(), hash))
                .await
                .map_err(|e| ConnectError::from(reach::ssh(hop.label, &e)))?
                .success()
        }
        Credentials::Agent => authenticate_with_agent(&mut session, hop.user, hop.label).await?,
    };
    if !authenticated {
        return Err(ConnectError::Unauthorized(auth_refused(
            hop.user,
            hop.host,
            hop.credentials,
        )));
    }
    Ok(session)
}

/// `bastion` logged Homeport in but wouldn't open a connection to `next`.
fn forward_failed(bastion: &Hop<'_>, next: &Hop<'_>, e: &russh::Error) -> ConnectError {
    let next = homeport_core::host::ssh_address(next.host, next.port);
    let bastion = homeport_core::host::ssh_address(bastion.host, bastion.port);
    ConnectError::from(
        AppError::new(
            ErrorCode::SshFailed,
            msg::SSH_BASTION_FORWARD_FAILED
                .with("bastion", &bastion)
                .with("next", &next),
        )
        .with_hint(
            msg::SSH_BASTION_FORWARD_FAILED_HINT
                .with("bastion", &bastion)
                .with("next", &next),
        )
        .with_cause(e),
    )
}

impl SshDialer {
    /// Opens a session channel on the Host's SSH session, holding the
    /// session's lock only to open it.
    async fn session_channel(&self) -> io::Result<russh::Channel<client::Msg>> {
        let mut chain = self.chain.lock().await;
        if chain.as_ref().is_none_or(Chain::is_closed) {
            let opened = self
                .open_chain()
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::ConnectionRefused, AppError::from(e)))?;
            *chain = Some(opened);
        }
        let session = chain.as_ref().expect("the chain was just opened").host();
        session
            .channel_open_session()
            .await
            .map_err(io::Error::other)
    }

    /// Starts `command` on a session channel, its data as the stream; the
    /// channel closes when the stream is dropped.
    async fn spawn_command(&self, command: String) -> io::Result<Spawned> {
        let channel = self.session_channel().await?;
        channel
            .exec(true, command)
            .await
            .map_err(io::Error::other)?;
        let (mut read, write) = channel.split();
        let writer = write.make_writer();
        let (mut sink, source) = tokio::io::duplex(64 * 1024);
        let (guard, mut dropped) = oneshot::channel::<()>();
        let (done, exited) = oneshot::channel();
        tokio::spawn(async move {
            let (mut stderr, mut status) = (Vec::new(), None);
            loop {
                tokio::select! {
                    message = read.wait() => match message {
                        Some(russh::ChannelMsg::Data { data }) => {
                            if sink.write_all(&data).await.is_err() {
                                break;
                            }
                        }
                        Some(russh::ChannelMsg::ExtendedData { data, ext: 1 }) => {
                            stderr.extend_from_slice(&data);
                        }
                        Some(russh::ChannelMsg::ExitStatus { exit_status }) => {
                            status = Some(exit_status);
                        }
                        Some(russh::ChannelMsg::Eof) => {
                            sink.shutdown().await.ok();
                        }
                        Some(russh::ChannelMsg::Close) | None => break,
                        Some(_) => {}
                    },
                    _ = &mut dropped => break,
                }
            }
            write.close().await.ok();
            drop(sink);
            done.send(CommandOutput {
                status,
                stdout: String::new(),
                stderr: String::from_utf8_lossy(&stderr).into_owned(),
            })
            .ok();
        });
        Ok(Spawned {
            io: Box::new(Guarded {
                inner: tokio::io::join(source, writer),
                _guard: guard,
            }),
            exit: Box::pin(async move {
                exited.await.unwrap_or(CommandOutput {
                    status: None,
                    stdout: String::new(),
                    stderr: String::new(),
                })
            }),
        })
    }

    /// Runs `command` on a session channel of the Host's SSH session.
    async fn run_command(&self, command: String) -> io::Result<CommandOutput> {
        let mut channel = self.session_channel().await?;
        channel
            .exec(true, command)
            .await
            .map_err(io::Error::other)?;
        let (mut stdout, mut stderr, mut status) = (Vec::new(), Vec::new(), None);
        while let Some(message) = channel.wait().await {
            match message {
                russh::ChannelMsg::Data { data } => stdout.extend_from_slice(&data),
                russh::ChannelMsg::ExtendedData { data, ext: 1 } => stderr.extend_from_slice(&data),
                russh::ChannelMsg::ExitStatus { exit_status } => status = Some(exit_status),
                russh::ChannelMsg::Close => break,
                _ => {}
            }
        }
        Ok(CommandOutput {
            status,
            stdout: String::from_utf8_lossy(&stdout).into_owned(),
            stderr: String::from_utf8_lossy(&stderr).into_owned(),
        })
    }
}

impl Dialer for SshDialer {
    fn connect(&self) -> BoxFuture<'_, Result<(), ConnectError>> {
        Box::pin(async move { self.open_channel().await.map(drop) })
    }

    fn runs_commands(&self) -> bool {
        true
    }

    fn run(&self, command: String, timeout: Duration) -> BoxFuture<'_, io::Result<CommandOutput>> {
        Box::pin(async move {
            tokio::time::timeout(timeout, self.run_command(command))
                .await
                .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "the command timed out"))?
        })
    }

    fn spawn(&self, command: String) -> BoxFuture<'_, io::Result<Spawned>> {
        Box::pin(self.spawn_command(command))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn hop(host: &str, label: &str) -> SshHop {
        SshHop {
            label: label.into(),
            host: host.into(),
            port: 22,
            user: "me".into(),
            credentials: Credentials::Agent,
            trusted_host_key: None,
        }
    }

    fn target(bastions: Vec<SshHop>) -> SshTarget {
        SshTarget {
            label: "prod (deploy@prod:22)".into(),
            host: "prod".into(),
            port: 22,
            user: "deploy".into(),
            socket_path: "/var/run/docker.sock".into(),
            credentials: Credentials::Agent,
            trusted_host_key: Some("ssh-ed25519 PROD".into()),
            bastions,
            engine: EngineKind::Docker,
        }
    }

    #[test]
    fn the_chain_goes_through_each_bastion_then_the_host() {
        let t = target(vec![hop("jump", "jump:22"), hop("inner", "inner:22")]);
        let hops = t.hops();
        let order: Vec<_> = hops.iter().map(|h| (h.host, h.bastion.clone())).collect();
        assert_eq!(
            order,
            [
                ("jump", Some("jump:22".into())),
                ("inner", Some("inner:22".into())),
                ("prod", None)
            ]
        );
        assert_eq!(hops[2].trusted_host_key, Some("ssh-ed25519 PROD"));
        assert_eq!(target(vec![]).hops().len(), 1);
    }

    #[test]
    fn a_refused_forward_names_the_bastion_and_the_next_server() {
        let t = target(vec![hop("jump", "jump:22")]);
        let hops = t.hops();
        let e = forward_failed(
            &hops[0],
            &hops[1],
            &russh::Error::ChannelOpenFailure(
                russh::ChannelOpenFailure::AdministrativelyProhibited,
            ),
        );
        let ConnectError::Unreachable(e) = e else {
            panic!("expected a retryable failure, got {e:?}");
        };
        assert_eq!(e.code, ErrorCode::SshFailed);
        assert_eq!(
            e.message,
            "Bastion jump:22 couldn't open a connection to prod:22."
        );
        assert!(e.hint.unwrap().text.contains("AllowTcpForwarding"));
    }

    #[tokio::test]
    async fn an_unreachable_bastion_is_reported_as_the_bastion() {
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let mut bastion = hop("127.0.0.1", "127.0.0.1 (bastion 1 of prod)");
        bastion.port = port;
        let dialer = SshDialer::new(target(vec![bastion]));
        let err = dialer.connect().await.unwrap_err();
        let ConnectError::Unreachable(e) = err else {
            panic!("expected unreachable, got {err:?}");
        };
        assert_eq!(e.code, ErrorCode::ConnectionRefused, "{e:?}");
        assert!(e.message.contains("bastion 1 of prod"), "{e:?}");
    }
}

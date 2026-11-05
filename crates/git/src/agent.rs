//! A private SSH agent holding one deploy key for one git command, so `ssh`
//! can authenticate without the key ever being written to disk (ADR 0014).
//!
//! It listens on a Unix socket with 0600 permissions inside a folder only
//! this user can enter, or a named pipe that refuses remote clients on
//! Windows, and stops when the [`Agent`] is dropped. Once the key is loaded
//! it refuses to add, remove or lock keys, so a client can only list it and
//! sign with it.

use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use russh::keys::{
    agent::server::{self, MessageType},
    decode_secret_key, PrivateKey,
};
use tokio::task::JoinHandle;

/// Refuses every change once the key is in.
#[derive(Clone)]
struct OneKey {
    sealed: Arc<AtomicBool>,
}

impl server::Agent for OneKey {
    async fn confirm_request(&self, msg: MessageType) -> bool {
        match msg {
            MessageType::RequestKeys | MessageType::Sign => true,
            MessageType::AddKeys => !self.sealed.load(Ordering::SeqCst),
            _ => false,
        }
    }
}

pub(crate) struct Agent {
    /// The value of `SSH_AUTH_SOCK` for the command.
    pub(crate) socket: String,
    serve: JoinHandle<()>,
    #[cfg(unix)]
    _dir: tempfile::TempDir,
}

impl Drop for Agent {
    fn drop(&mut self) {
        self.serve.abort();
    }
}

fn key_of(private_key: &str) -> io::Result<PrivateKey> {
    decode_secret_key(private_key, None).map_err(io::Error::other)
}

#[cfg(unix)]
pub(crate) async fn start(private_key: &str) -> io::Result<Agent> {
    use std::os::unix::fs::PermissionsExt;

    let key = key_of(private_key)?;
    let base = std::env::var_os("XDG_RUNTIME_DIR")
        .map(std::path::PathBuf::from)
        .filter(|p| p.is_dir())
        .unwrap_or_else(std::env::temp_dir);
    let dir = tempfile::Builder::new()
        .prefix("homeport-git-")
        .tempdir_in(base)?;
    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700))?;
    let path = dir.path().join("agent.sock");
    let listener = tokio::net::UnixListener::bind(&path)?;
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    let connections = futures_util::stream::unfold(listener, |listener| async move {
        let next = listener.accept().await.map(|(stream, _)| stream);
        Some((next, listener))
    });
    let sealed = Arc::new(AtomicBool::new(false));
    let agent = OneKey {
        sealed: sealed.clone(),
    };
    let serve = tokio::spawn(async move {
        if let Err(e) = server::serve(Box::pin(connections), agent).await {
            log::warn!("git's SSH agent stopped: {e}");
        }
    });
    let mut client = russh::keys::agent::client::AgentClient::connect_uds(&path)
        .await
        .map_err(io::Error::other)?;
    client
        .add_identity(&key, &[])
        .await
        .map_err(io::Error::other)?;
    sealed.store(true, Ordering::SeqCst);
    Ok(Agent {
        socket: path.to_string_lossy().into_owned(),
        serve,
        _dir: dir,
    })
}

#[cfg(windows)]
pub(crate) async fn start(private_key: &str) -> io::Result<Agent> {
    use tokio::net::windows::named_pipe::{ClientOptions, ServerOptions};

    let key = key_of(private_key)?;
    let name = format!(r"\\.\pipe\homeport-git-{}", uuid::Uuid::new_v4().simple());
    let first = ServerOptions::new()
        .first_pipe_instance(true)
        .reject_remote_clients(true)
        .create(&name)?;
    let pipe = name.clone();
    // Each connection takes the waiting instance and leaves a new one.
    let connections = futures_util::stream::unfold(Some(first), move |server| {
        let pipe = pipe.clone();
        async move {
            let server = server?;
            if let Err(e) = server.connect().await {
                return Some((Err(e), None));
            }
            match ServerOptions::new()
                .reject_remote_clients(true)
                .create(&pipe)
            {
                Ok(next) => Some((Ok(server), Some(next))),
                Err(e) => {
                    log::warn!("git's SSH agent couldn't open its next pipe instance: {e}");
                    Some((Ok(server), None))
                }
            }
        }
    });
    let sealed = Arc::new(AtomicBool::new(false));
    let agent = OneKey {
        sealed: sealed.clone(),
    };
    let serve = tokio::spawn(async move {
        if let Err(e) = server::serve(Box::pin(connections), agent).await {
            log::warn!("git's SSH agent stopped: {e}");
        }
    });
    let stream = ClientOptions::new().open(&name)?;
    let mut client = russh::keys::agent::client::AgentClient::connect(stream);
    client
        .add_identity(&key, &[])
        .await
        .map_err(io::Error::other)?;
    sealed.store(true, Ordering::SeqCst);
    Ok(Agent {
        socket: name,
        serve,
    })
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn the_agent_holds_only_its_key_and_refuses_changes() {
        let key = homeport_transport::ssh::generate_key("test");
        let agent = start(&key.private_key).await.unwrap();
        let mode = std::os::unix::fs::PermissionsExt::mode(
            &std::fs::metadata(&agent.socket).unwrap().permissions(),
        );
        assert_eq!(mode & 0o777, 0o600);

        let mut client = russh::keys::agent::client::AgentClient::connect_uds(&agent.socket)
            .await
            .unwrap();
        let keys = client.request_identities().await.unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(
            keys[0].public_key().to_openssh().unwrap(),
            key.public_key
                .split(' ')
                .take(2)
                .collect::<Vec<_>>()
                .join(" ")
        );
        let other = key_of(&homeport_transport::ssh::generate_key("x").private_key).unwrap();
        assert!(client.add_identity(&other, &[]).await.is_err());
        let mut client = russh::keys::agent::client::AgentClient::connect_uds(&agent.socket)
            .await
            .unwrap();
        assert!(client.remove_all_identities().await.is_err());
        let mut client = russh::keys::agent::client::AgentClient::connect_uds(&agent.socket)
            .await
            .unwrap();
        assert_eq!(client.request_identities().await.unwrap().len(), 1);

        let socket = agent.socket.clone();
        drop(agent);
        assert!(!std::path::Path::new(&socket).exists());
    }
}

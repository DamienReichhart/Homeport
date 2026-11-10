//! The client of a connected Host, of the kind its Engine has, and why there is
//! none when there isn't.

use std::sync::Arc;

use bollard::Docker;

use homeport_containerd::Containerd;
use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_engine::podman::Podman;
use homeport_incus::Incus;
use homeport_transport::{Connected, Dialer};

use super::{Client, Entry, HostRegistry, HostStatus};
use crate::messages as msg;

impl HostRegistry {
    /// The Docker Engine API client of a connected Host;
    /// `EngineUnsupported` for a Host whose Engine doesn't speak it.
    pub fn docker(&self, id: &str) -> AppResult<Docker> {
        self.docker_connection(id).map(|c| c.docker)
    }

    /// What opens streams to a connected Host's Engine socket, for the
    /// compose proxy and for the APIs bollard doesn't speak.
    pub fn dialer(&self, id: &str) -> AppResult<Arc<dyn Dialer>> {
        match self.connected(id)? {
            Client::Docker(connected) => Ok(connected.dialer),
            Client::Containerd(containerd) => Ok(containerd.dialer().clone()),
            Client::Incus(incus) => Ok(incus.dialer().clone()),
        }
    }

    /// containerd's API on a connected containerd Host; `EngineUnsupported`
    /// for any other Engine.
    pub fn containerd(&self, id: &str) -> AppResult<Containerd> {
        match self.connected(id)? {
            Client::Containerd(containerd) => Ok(containerd),
            Client::Docker(_) | Client::Incus(_) => Err(self.not_containerd(id)),
        }
    }

    /// Incus's API on a connected Incus Host; `EngineUnsupported` for any
    /// other Engine.
    pub fn incus(&self, id: &str) -> AppResult<Incus> {
        match self.connected(id)? {
            Client::Incus(incus) => Ok(incus),
            Client::Docker(_) | Client::Containerd(_) => Err(self.not_incus(id)),
        }
    }

    /// Why a Host can't do what only Incus does.
    pub(super) fn not_incus(&self, id: &str) -> AppError {
        let error = match self.store.get(id) {
            Ok(host) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::NOT_INCUS.with("name", host.name),
            ),
            Err(_) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::ENGINE_UNSUPPORTED_UNKNOWN,
            ),
        };
        error.with_hint(msg::NOT_INCUS_HINT)
    }

    /// Why a Host can't do what only containerd does.
    pub(super) fn not_containerd(&self, id: &str) -> AppError {
        let error = match self.store.get(id) {
            Ok(host) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::NOT_CONTAINERD.with("name", host.name),
            ),
            Err(_) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::ENGINE_UNSUPPORTED_UNKNOWN,
            ),
        };
        error.with_hint(msg::NOT_CONTAINERD_HINT)
    }

    /// Podman's own API on a connected Podman Host; `EngineUnsupported`
    /// for any other Engine.
    pub fn podman(&self, id: &str) -> AppResult<Podman> {
        let connected = self.docker_connection(id)?;
        match self.status(id).podman_version() {
            Some(version) => Ok(Podman::new(connected.dialer, version)),
            None => Err(self.not_podman(id)),
        }
    }

    /// Why a Host can't do what only Podman does.
    pub(super) fn not_podman(&self, id: &str) -> AppError {
        let error = match self.store.get(id) {
            Ok(host) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::NOT_PODMAN.with("name", host.name),
            ),
            Err(_) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::ENGINE_UNSUPPORTED_UNKNOWN,
            ),
        };
        error.with_hint(msg::NOT_PODMAN_HINT)
    }

    pub(super) fn docker_connection(&self, id: &str) -> AppResult<Connected> {
        match self.connected(id)? {
            Client::Docker(connected) => Ok(connected),
            Client::Containerd(_) | Client::Incus(_) => Err(self.unsupported(id)),
        }
    }

    /// Why a Host's Engine can't do what was asked: it is of another kind.
    pub fn unsupported(&self, id: &str) -> AppError {
        match self.store.get(id) {
            Ok(host) => unsupported(&host.name, host.engine.label()),
            Err(_) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::ENGINE_UNSUPPORTED_UNKNOWN,
            ),
        }
    }

    pub(super) fn connected(&self, id: &str) -> AppResult<Client> {
        let status = {
            let entries = self.lock();
            match entries.get(id) {
                Some(Entry {
                    client: Some(client),
                    ..
                }) => return Ok(client.clone()),
                Some(entry) => entry.status.clone(),
                None => HostStatus::Connecting,
            }
        };
        Err(self.not_connected(id, &status))
    }

    /// Why a command can't reach the Host right now, carrying the cause from its status.
    pub(super) fn not_connected(&self, id: &str, status: &HostStatus) -> AppError {
        let message = match self.store.get(id) {
            Ok(host) => msg::NOT_CONNECTED.with("name", host.name),
            Err(_) => msg::NOT_CONNECTED_UNKNOWN.into(),
        };
        let error = AppError::new(ErrorCode::HostNotConnected, message);
        if let Some(cause) = status.error() {
            return AppError {
                hint: cause.hint.clone(),
                detail: Some(match &cause.detail {
                    Some(detail) => format!("{}\n{detail}", cause.message),
                    None => cause.message.clone(),
                }),
                ..error
            };
        }
        match status {
            HostStatus::Disabled => error.with_hint(msg::NOT_CONNECTED_DISABLED_HINT),
            HostStatus::Connecting => error.with_hint(msg::NOT_CONNECTED_CONNECTING_HINT),
            _ => error.with_hint(msg::NOT_CONNECTED_HINT),
        }
    }
}

/// `name`'s Engine, described by `engine`, doesn't offer what was asked.
pub(crate) fn unsupported(name: &str, engine: &str) -> AppError {
    AppError::new(
        ErrorCode::EngineUnsupported,
        msg::ENGINE_UNSUPPORTED
            .with("name", name)
            .with("engine", engine),
    )
    .with_hint(msg::ENGINE_UNSUPPORTED_HINT)
}

//! A connection to one containerd: HTTP/2 over the Host's dialer, and the
//! requests every service call is built from.

use std::{sync::Arc, time::Duration};

use hyper_util::rt::TokioIo;
use tonic::{
    metadata::MetadataValue,
    transport::{Channel, Endpoint, Uri},
    Request,
};

use homeport_core::error::{AppError, ErrorCode};
use homeport_transport::{Dialer, CONNECT_TIMEOUT};

use crate::{
    api::containerd::services::{
        containers::v1::containers_client::ContainersClient,
        events::v1::events_client::EventsClient, images::v1::images_client::ImagesClient,
        introspection::v1::introspection_client::IntrospectionClient,
        namespaces::v1::namespaces_client::NamespacesClient,
        snapshots::v1::snapshots_client::SnapshotsClient, tasks::v1::tasks_client::TasksClient,
        version::v1::version_client::VersionClient,
    },
    error, messages as msg,
};

/// The oldest containerd Homeport connects to, the oldest still supported
/// upstream (ADR 0020).
pub const MIN_VERSION: &str = "1.7";
/// The oldest containerd Homeport pulls images on, through Transfer.
pub const PULL_VERSION: &str = "2.0";

/// How long a call that isn't a stream may take.
const CALL_TIMEOUT: Duration = Duration::from_secs(120);

/// A live connection to a Host's containerd.
#[derive(Clone)]
pub struct Containerd {
    channel: Channel,
    dialer: Arc<dyn Dialer>,
    /// The Host's name, for messages.
    name: String,
    version: String,
    revision: String,
}

impl std::fmt::Debug for Containerd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Containerd")
            .field("name", &self.name)
            .field("version", &self.version)
            .finish_non_exhaustive()
    }
}

/// A channel whose every HTTP/2 connection is a fresh stream from `dialer`.
fn channel(dialer: Arc<dyn Dialer>) -> Channel {
    Endpoint::from_static("http://containerd")
        .connect_timeout(CONNECT_TIMEOUT)
        .http2_keep_alive_interval(Duration::from_secs(15))
        .keep_alive_timeout(CONNECT_TIMEOUT)
        .connect_with_connector_lazy(tower::service_fn(move |_: Uri| {
            let dialer = dialer.clone();
            async move { dialer.dial().await.map(TokioIo::new) }
        }))
}

impl Containerd {
    /// Asks containerd at the other end of `dialer` for its version, and
    /// refuses one older than [`MIN_VERSION`]. `name` is the Host's.
    pub async fn connect(name: &str, dialer: Arc<dyn Dialer>) -> Result<Self, AppError> {
        let channel = channel(dialer.clone());
        let response = tokio::time::timeout(
            CONNECT_TIMEOUT,
            VersionClient::new(channel.clone()).version(()),
        )
        .await
        .map_err(|_| AppError::new(ErrorCode::Timeout, msg::CONNECT_TIMEOUT.with("name", name)))?
        .map_err(|status| error::not_containerd(name, &status))?
        .into_inner();
        let version = response.version.trim_start_matches('v').to_owned();
        if !at_least(&version, MIN_VERSION) {
            return Err(AppError::new(
                ErrorCode::ContainerdTooOld,
                msg::TOO_OLD_TO_CONNECT
                    .with("name", name)
                    .with("version", &version)
                    .with("minimum", MIN_VERSION),
            )
            .with_hint(msg::TOO_OLD_HINT.with("minimum", MIN_VERSION)));
        }
        Ok(Self {
            channel,
            dialer,
            name: name.to_owned(),
            version,
            revision: response.revision,
        })
    }

    /// The Host's name, for messages.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Its version, without the leading `v`: "2.1.4".
    pub fn version(&self) -> &str {
        &self.version
    }

    /// The commit it was built from.
    pub fn revision(&self) -> &str {
        &self.revision
    }

    /// What reaches the machine containerd runs on, for logs and exec.
    pub fn dialer(&self) -> &Arc<dyn Dialer> {
        &self.dialer
    }

    /// Whether this containerd is `minimum` or later.
    pub fn at_least(&self, minimum: &str) -> bool {
        at_least(&self.version, minimum)
    }

    /// Answers `version` quickly while the connection works.
    pub async fn ping(&self) -> Result<(), AppError> {
        tokio::time::timeout(CONNECT_TIMEOUT, self.version_client().version(()))
            .await
            .map_err(|_| {
                AppError::new(
                    ErrorCode::Timeout,
                    msg::CONNECT_TIMEOUT.with("name", &self.name),
                )
            })?
            .map(drop)
            .map_err(|status| error::status(&status))
    }

    pub(crate) fn version_client(&self) -> VersionClient<Channel> {
        VersionClient::new(self.channel.clone())
    }
    pub(crate) fn events(&self) -> EventsClient<Channel> {
        EventsClient::new(self.channel.clone())
    }
    pub(crate) fn namespaces(&self) -> NamespacesClient<Channel> {
        NamespacesClient::new(self.channel.clone())
    }
    pub(crate) fn containers(&self) -> ContainersClient<Channel> {
        ContainersClient::new(self.channel.clone())
    }
    pub(crate) fn tasks(&self) -> TasksClient<Channel> {
        TasksClient::new(self.channel.clone())
    }
    pub(crate) fn images(&self) -> ImagesClient<Channel> {
        ImagesClient::new(self.channel.clone())
    }
    pub(crate) fn snapshots(&self) -> SnapshotsClient<Channel> {
        SnapshotsClient::new(self.channel.clone())
    }
    pub(crate) fn introspection(&self) -> IntrospectionClient<Channel> {
        IntrospectionClient::new(self.channel.clone())
    }
}

/// `message` for a call in `namespace`, which goes in containerd's
/// `containerd-namespace` metadata, with a deadline.
pub(crate) fn call<T>(namespace: &str, message: T) -> Result<Request<T>, AppError> {
    let mut request = stream(namespace, message)?;
    request.set_timeout(CALL_TIMEOUT);
    Ok(request)
}

/// `message` for a streaming or long call in `namespace`, without a deadline.
pub(crate) fn stream<T>(namespace: &str, message: T) -> Result<Request<T>, AppError> {
    crate::namespaces::validate(namespace)?;
    let value: MetadataValue<_> = namespace
        .parse()
        .map_err(|_| AppError::validation(msg::NAMESPACE_INVALID.with("name", namespace)))?;
    let mut request = Request::new(message);
    request.metadata_mut().insert("containerd-namespace", value);
    Ok(request)
}

/// `version`'s leading numbers, so "2.0.0-rc.1" reads as [2, 0, 0].
fn numbers(version: &str) -> Vec<u64> {
    version
        .trim()
        .trim_start_matches('v')
        .split(|c: char| !c.is_ascii_digit() && c != '.')
        .next()
        .unwrap_or_default()
        .split('.')
        .filter_map(|n| n.parse().ok())
        .collect()
}

/// Whether `version` is `minimum` or later.
pub(crate) fn at_least(version: &str, minimum: &str) -> bool {
    let (have, need) = (numbers(version), numbers(minimum));
    for i in 0..have.len().max(need.len()) {
        let (a, b) = (
            have.get(i).copied().unwrap_or(0),
            need.get(i).copied().unwrap_or(0),
        );
        if a != b {
            return a > b;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_compare_by_their_numbers() {
        assert!(at_least("2.1.4", "1.7"));
        assert!(at_least("1.7.0", "1.7"));
        assert!(at_least("v2.0.0-rc.1", "2.0"));
        assert!(!at_least("1.6.36", "1.7"));
        assert!(!at_least("1.7.27", "2.0"));
        assert!(at_least("10.0", "2.0"));
    }

    #[test]
    fn calls_carry_their_namespace() {
        let request = call("k8s.io", ()).unwrap();
        assert_eq!(
            request.metadata().get("containerd-namespace").unwrap(),
            "k8s.io"
        );
        assert!(call("bad namespace", ()).is_err());
    }
}

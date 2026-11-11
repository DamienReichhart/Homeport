//! What the backend's crates tell the UI as it happens, and the notifiers
//! that carry it: each crate calls a trait, this one puts an [`Event`] on a
//! channel the UI listens to.
//!
//! The channel only carries the news. What the Tauri shell did on hearing
//! it is the listener's to do: the window removes leftover helper
//! containers (`api::app::lifecycle`), the tray redraws on every Host
//! status (#250), and the OS notifications of Host and container events,
//! with their settings and their once-per-outage rule, are #251's.

use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;

use homeport_clusters::registry::{ClusterNotifier, ClusterStatus};
use homeport_core::error::AppError;
use homeport_hosts::registry::{EngineEvent, HostStatus, Notifier};
use homeport_kubernetes::streams::forward::ForwardEvent;
use homeport_stacks::StackOwner;

#[derive(Debug, Clone)]
pub enum Event {
    HostStatus {
        host_id: String,
        status: Arc<HostStatus>,
    },
    ClusterStatus {
        cluster_id: String,
        status: Arc<ClusterStatus>,
    },
    PortForward(Arc<ForwardEvent>),
    Engine {
        host_id: String,
        event: Arc<EngineEvent>,
    },
    /// A Host was renamed, otherwise changed, or removed: the tray and every
    /// list of Hosts are out of date. A Host that is added, imported,
    /// enabled or disabled is only told through its [`Event::HostStatus`],
    /// so a list of Hosts follows both.
    HostsChanged,
    /// The text the OS draws (tray menu, notifications) was given in another
    /// language.
    NativeTextChanged,
    /// A Synced Stack's poll found a new commit that changes its compose
    /// file's folder.
    SyncedUpdateAvailable {
        /// The Host or Swarm it's deployed to.
        owner: StackOwner,
        stack_id: String,
        name: String,
    },
    /// A Synced Stack's automatic deploy ended; `error` says why it failed.
    SyncedAutoDeployed {
        owner: StackOwner,
        stack_id: String,
        name: String,
        commit: String,
        error: Option<Box<AppError>>,
    },
}

/// The sending half, cloned into each registry.
#[derive(Clone)]
pub struct Events(UnboundedSender<Event>);

impl Events {
    pub fn new(sender: UnboundedSender<Event>) -> Self {
        Self(sender)
    }

    /// Tells the UI of `event`.
    pub fn send(&self, event: Event) {
        // The receiver only goes away when the app is closing.
        let _ = self.0.send(event);
    }
}

impl Notifier for Events {
    fn host_status(&self, host_id: &str, status: &HostStatus) {
        self.send(Event::HostStatus {
            host_id: host_id.to_owned(),
            status: Arc::new(status.clone()),
        });
    }

    fn engine_event(&self, host_id: &str, event: &EngineEvent) {
        self.send(Event::Engine {
            host_id: host_id.to_owned(),
            event: Arc::new(event.clone()),
        });
    }
}

impl ClusterNotifier for Events {
    fn cluster_status(&self, cluster_id: &str, status: &ClusterStatus) {
        self.send(Event::ClusterStatus {
            cluster_id: cluster_id.to_owned(),
            status: Arc::new(status.clone()),
        });
    }

    fn port_forward(&self, event: &ForwardEvent) {
        self.send(Event::PortForward(Arc::new(event.clone())));
    }
}

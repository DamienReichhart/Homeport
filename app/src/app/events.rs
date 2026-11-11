//! What the backend's crates tell the UI as it happens, and the notifiers
//! that carry it: each crate calls a trait, this one puts an [`Event`] on a
//! channel the UI listens to.

use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;

use homeport_clusters::registry::{ClusterNotifier, ClusterStatus};
use homeport_hosts::registry::{EngineEvent, HostStatus, Notifier};
use homeport_kubernetes::streams::forward::ForwardEvent;

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
}

/// The sending half, cloned into each registry.
#[derive(Clone)]
pub struct Events(UnboundedSender<Event>);

impl Events {
    pub fn new(sender: UnboundedSender<Event>) -> Self {
        Self(sender)
    }

    fn send(&self, event: Event) {
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

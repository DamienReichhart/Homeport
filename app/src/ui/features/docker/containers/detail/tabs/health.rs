//! The Health tab (`HealthTab`): a container's healthcheck, its status,
//! its configuration and its last results. Read again when the status
//! the list shows changes, and every check interval while the tab shows.

use std::{collections::HashSet, time::Duration};

use homeport_engine::containers::health::Health;

use crate::ui::widgets::feedback::error_message;

/// The Engine's interval when the healthcheck sets none.
const DEFAULT_INTERVAL: u32 = 30;

/// How often the tab reads the healthcheck again: once per check.
pub fn every(health: Option<&Health>) -> Duration {
    let seconds = health
        .and_then(|health| health.config.interval_s)
        .unwrap_or(DEFAULT_INTERVAL);
    Duration::from_secs(u64::from(seconds.max(1)))
}

#[derive(Debug, Clone)]
pub enum Message {
    /// "Show more" or "Show less" under the output of the run that
    /// started then.
    Fold(String),
    /// It is time to read the healthcheck again.
    Poll,
    /// What was asked of the failure shown in place of the tab.
    Failure(error_message::Event),
}

#[derive(Debug, Default)]
pub struct Tab {
    /// The runs whose whole output shows, by when they started.
    pub open: HashSet<String>,
}

impl Tab {
    pub fn fold(&mut self, start: String) {
        if !self.open.remove(&start) {
            self.open.insert(start);
        }
    }
}

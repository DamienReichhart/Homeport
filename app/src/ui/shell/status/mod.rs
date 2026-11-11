//! How a Host's or a Cluster's connection reads: a short label, a colour
//! and the reason (`src/core/host/model`, `src/core/cluster/model`), what
//! a Host's Engine offers, and a Swarm's name.

pub mod cluster;
pub mod engine;
pub mod host;
pub mod swarm;
#[cfg(test)]
mod tests;

use crate::ui::theme::tokens::Tone;

/// A status in one line: its label, its colour and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    pub label: String,
    pub tone: Tone,
    pub detail: Option<String>,
}

impl Summary {
    fn new(label: String, tone: Tone) -> Self {
        Self {
            label,
            tone,
            detail: None,
        }
    }

    fn because(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }

    /// The label and the reason in one line, as shown when hovering a Host
    /// in a list (`summaryTitle`).
    pub fn title(&self) -> String {
        match &self.detail {
            Some(detail) => {
                crate::t!(
                    "hosts.statusTitle",
                    label = self.label.as_str(),
                    detail = detail.as_str()
                )
            }
            None => self.label.clone(),
        }
    }
}

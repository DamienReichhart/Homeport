//! The samples an open page has read, kept in memory for its charts and
//! dropped with it: never on disk.

use std::collections::VecDeque;

use super::{ContainerHistory, ResourceUsage, UsageSample};

/// How long an open page keeps its samples, in seconds: 15 minutes.
pub const KEPT: i64 = 15 * 60;

/// The samples of each series (a Pod's containers, or a Node), in the order
/// they were first seen, none older than [`KEPT`] before the newest.
#[derive(Debug, Default)]
pub struct History {
    series: Vec<(String, VecDeque<UsageSample>)>,
}

impl History {
    /// Adds what `key` used at `time` (Unix seconds). A reading no newer
    /// than the last one is left out: metrics-server hasn't read the
    /// kubelet again since, so it's the same sample.
    pub fn push(&mut self, key: &str, time: i64, usage: ResourceUsage) {
        let index = match self.series.iter().position(|(k, _)| k == key) {
            Some(index) => index,
            None => {
                self.series.push((key.to_owned(), VecDeque::new()));
                self.series.len() - 1
            }
        };
        let samples = &mut self.series[index].1;
        if samples.back().is_some_and(|last| last.time >= time) {
            return;
        }
        samples.push_back(UsageSample { time, usage });
        self.trim(time);
    }

    /// Drops the samples older than [`KEPT`] before `newest`, and the series
    /// left empty (a container that's gone).
    fn trim(&mut self, newest: i64) {
        let oldest = newest - KEPT;
        for (_, samples) in &mut self.series {
            while samples.front().is_some_and(|s| s.time < oldest) {
                samples.pop_front();
            }
        }
        self.series.retain(|(_, samples)| !samples.is_empty());
    }

    /// Every series, as a container's.
    pub fn containers(&self) -> Vec<ContainerHistory> {
        self.series
            .iter()
            .map(|(name, samples)| ContainerHistory {
                name: name.clone(),
                samples: samples.iter().copied().collect(),
            })
            .collect()
    }

    /// The samples of `key`, oldest first.
    pub fn samples(&self, key: &str) -> Vec<UsageSample> {
        self.series
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, samples)| samples.iter().copied().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn usage(cpu: f64) -> ResourceUsage {
        ResourceUsage { cpu, memory: 1 }
    }

    #[test]
    fn samples_older_than_fifteen_minutes_before_the_newest_are_dropped() {
        let mut history = History::default();
        for i in 0..100 {
            history.push("web", 1_000 + i * 15, usage(i as f64));
        }
        let samples = history.samples("web");
        // 15 minutes at one sample every 15 s, both ends kept.
        assert_eq!(samples.len(), 61);
        let newest = 1_000 + 99 * 15;
        assert_eq!(samples.last().unwrap().time, newest);
        assert_eq!(samples[0].time, newest - KEPT);
        assert!(samples.windows(2).all(|w| w[0].time < w[1].time));
    }

    #[test]
    fn the_same_reading_twice_is_kept_once() {
        let mut history = History::default();
        history.push("web", 100, usage(1.0));
        history.push("web", 100, usage(2.0));
        history.push("web", 90, usage(3.0));
        history.push("web", 115, usage(4.0));
        let cpu: Vec<f64> = history.samples("web").iter().map(|s| s.usage.cpu).collect();
        assert_eq!(cpu, [1.0, 4.0]);
    }

    #[test]
    fn a_container_no_longer_reported_ages_out() {
        let mut history = History::default();
        history.push("web", 0, usage(1.0));
        history.push("sidecar", 0, usage(1.0));
        for t in 1..=61 {
            history.push("web", t * 15, usage(1.0));
        }
        let names: Vec<String> = history.containers().into_iter().map(|c| c.name).collect();
        assert_eq!(names, ["web"]);
        assert!(history.samples("sidecar").is_empty());
    }

    #[test]
    fn containers_keep_the_order_they_were_first_seen_in() {
        let mut history = History::default();
        history.push("web", 0, usage(1.0));
        history.push("proxy", 0, usage(1.0));
        history.push("web", 15, usage(1.0));
        let names: Vec<String> = history.containers().into_iter().map(|c| c.name).collect();
        assert_eq!(names, ["web", "proxy"]);
    }
}

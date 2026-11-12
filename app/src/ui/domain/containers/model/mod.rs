//! The pure parts of a container's row (`containerTone`, `portLabels`,
//! `matches` and `stackOf` of the React feature's `model/`).

use std::collections::HashMap;

use homeport_engine::containers::{stats::ContainerUsage, Container};

use crate::{data::streams::Update, ui::theme::Tone};

/// The tone of an Engine container state (`running`, `exited`…) and its
/// health, if it has a check.
pub fn tone(state: &str, health: Option<&str>) -> Tone {
    if health == Some("unhealthy") {
        return Tone::Danger;
    }
    match state {
        "running" if health == Some("starting") => Tone::Warning,
        "running" => Tone::Success,
        "paused" | "restarting" => Tone::Warning,
        "dead" => Tone::Danger,
        _ => Tone::Muted,
    }
}

/// A port as a list shows it, and where it is published, if it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortLabel {
    pub label: String,
    pub public_port: Option<u16>,
}

/// `8080:80/tcp`-style labels, published ports first.
pub fn port_labels(container: &Container) -> Vec<PortLabel> {
    let mut ports: Vec<_> = container.ports.iter().collect();
    // Stable: the published ones keep their order, and so do the others.
    ports.sort_by_key(|port| port.public_port.is_none());
    ports
        .into_iter()
        .map(|port| PortLabel {
            label: match port.public_port.filter(|public| *public != 0) {
                Some(public) => format!("{public}:{}/{}", port.private_port, port.protocol),
                None => format!("{}/{}", port.private_port, port.protocol),
            },
            public_port: port.public_port,
        })
        .collect()
}

/// The label `docker stack deploy` puts on a swarm stack's containers.
pub const STACK_NAMESPACE: &str = "com.docker.stack.namespace";

/// The Stack a container belongs to, and whether it is a swarm stack's
/// task on this Host rather than a Compose project's container.
pub fn stack_of(container: &Container) -> Option<(&str, bool)> {
    if let Some(stack) = container.stack.as_deref().filter(|stack| !stack.is_empty()) {
        return Some((stack, false));
    }
    container
        .labels
        .get(STACK_NAMESPACE)
        .filter(|namespace| !namespace.is_empty())
        .map(|namespace| (namespace.as_str(), true))
}

/// Whether a container matches the search box: its name, image, Stack or
/// service, whatever the case, or the start of its id.
pub fn matches(container: &Container, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return true;
    }
    let holds = |text: &str| text.to_lowercase().contains(&query);
    holds(&container.name)
        || holds(&container.image)
        || container.stack.as_deref().is_some_and(holds)
        || container.service.as_deref().is_some_and(holds)
        || container.id.starts_with(&query)
}

/// What a container's status says, for a list that shows the health as a
/// badge beside it: `Up 3 hours (healthy)` is `Up 3 hours`.
pub fn status_without_health(container: &Container) -> String {
    let status = if container.status.is_empty() {
        &container.state
    } else {
        &container.status
    };
    let found = ["(healthy)", "(unhealthy)", "(health: starting)"]
        .iter()
        .filter_map(|health| status.find(health).map(|at| (at, health.len())))
        .min();
    match found {
        // Taken out with the space before it.
        Some((at, len)) => format!("{}{}", status[..at].trim_end(), &status[at + len..]),
        None => status.clone(),
    }
}

/// The CPU and memory of a Host's running containers, by container id.
pub type Usage = HashMap<String, ContainerUsage>;

/// What a batch of the usage stream leaves to show: its last report, which
/// stands for every running container. `None` for an update that reports
/// nothing (the stream started, or ended).
pub fn latest_usage(update: Update<Vec<ContainerUsage>>) -> Option<Usage> {
    let Update::Batch { mut items, .. } = update else {
        return None;
    };
    let last = items.pop()?;
    Some(
        last.into_iter()
            .map(|usage| (usage.id.clone(), usage))
            .collect(),
    )
}

#[cfg(test)]
mod tests;

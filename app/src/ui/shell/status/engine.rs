//! What a Host's Engine offers (`src/core/host/model/engine.ts`).

use homeport_core::host::EngineKind;
use homeport_hosts::service::HostView;

use crate::{t, ui::shell::registry::Capability};

/// The name of a kind of Engine, as the Host form and pages show it.
pub fn label(engine: EngineKind) -> String {
    match engine {
        EngineKind::Docker => t!("core.engine.docker"),
        EngineKind::Containerd => t!("core.engine.containerd"),
        EngineKind::Incus => t!("core.engine.incus"),
    }
}

/// Whether `host` offers `capability`. The Docker Engine API is known from
/// the stored Engine kind; Podman's own only once the Host is connected,
/// because the same socket can change product.
pub fn supports(host: Option<&HostView>, capability: Capability) -> bool {
    let Some(host) = host else {
        return false;
    };
    let docker = host.host.engine == EngineKind::Docker;
    match capability {
        Capability::Docker => docker,
        Capability::Podman => docker && host.status.podman_version().is_some(),
        Capability::Containerd => host.host.engine == EngineKind::Containerd,
        Capability::Incus => host.host.engine == EngineKind::Incus,
    }
}

/// Whether what `host` offers is known yet: its Engine kind always is, and
/// Podman once it's connected. Until then a page can't say it's unsupported.
pub fn capability_known(host: &HostView, capability: Capability) -> bool {
    capability != Capability::Podman || super::host::is_connected(&host.status)
}

/// `version`'s leading numbers, so "5.8.1-dev" reads as [5, 8, 1].
fn numbers(version: &str) -> Vec<u64> {
    let version = version.trim();
    let version = version.strip_prefix('v').unwrap_or(version);
    let digits: String = version
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    if !digits.starts_with(|c: char| c.is_ascii_digit()) {
        return vec![];
    }
    digits
        .split('.')
        .map_while(|part| part.parse().ok())
        .collect()
}

/// Whether `version` is `minimum` or later, comparing their numbers in order.
pub fn version_at_least(version: &str, minimum: &str) -> bool {
    let (have, need) = (numbers(version), numbers(minimum));
    if have.is_empty() {
        return false;
    }
    for index in 0..have.len().max(need.len()) {
        let a = have.get(index).copied().unwrap_or(0);
        let b = need.get(index).copied().unwrap_or(0);
        if a != b {
            return a > b;
        }
    }
    true
}

/// Whether a connected Host runs Podman `minimum` or later.
pub fn podman_at_least(host: Option<&HostView>, minimum: &str) -> bool {
    host.and_then(|host| host.status.podman_version())
        .is_some_and(|version| version_at_least(version, minimum))
}

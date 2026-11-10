//! Registry addresses as `docker login` understands them.

/// The address Docker Hub's logins are stored under.
pub const DOCKER_HUB: &str = "docker.io";

/// Names Docker Hub goes by, after the scheme and path are stripped.
const DOCKER_HUB_ALIASES: &[&str] = &[
    "docker.io",
    "index.docker.io",
    "registry-1.docker.io",
    "registry.hub.docker.com",
    "hub.docker.com",
];

/// The canonical form of a registry address: no scheme, no path such as
/// `/v1/`, lower case, and every Docker Hub alias as [`DOCKER_HUB`].
/// `None` when nothing is left.
pub fn normalise(address: &str) -> Option<String> {
    let address = address.trim();
    let address = address
        .strip_prefix("https://")
        .or_else(|| address.strip_prefix("http://"))
        .unwrap_or(address);
    let host = address.split('/').next().unwrap_or_default().to_lowercase();
    if host.is_empty() || host.contains(char::is_whitespace) {
        return None;
    }
    if DOCKER_HUB_ALIASES.contains(&host.as_str()) {
        return Some(DOCKER_HUB.to_owned());
    }
    Some(host)
}

/// The registry an image reference is pulled from: its first path component
/// when that looks like a host (`ghcr.io/org/app:1`, `localhost:5000/app`),
/// Docker Hub otherwise (`nginx`, `library/nginx`, `bitnami/redis`).
pub fn registry_of(reference: &str) -> String {
    let reference = reference.trim();
    match reference.split_once('/') {
        Some((first, _)) if is_host(first) => {
            normalise(first).unwrap_or_else(|| DOCKER_HUB.to_owned())
        }
        _ => DOCKER_HUB.to_owned(),
    }
}

/// Docker's own rule: a first component with a dot or a port, or `localhost`.
fn is_host(component: &str) -> bool {
    component.contains('.') || component.contains(':') || component == "localhost"
}

/// The host Homeport talks to for a registry's HTTP API.
pub(crate) fn api_host(address: &str) -> &str {
    if address == DOCKER_HUB {
        "registry-1.docker.io"
    } else {
        address
    }
}

/// Registries on this machine, which Docker lets speak plain HTTP.
pub(crate) fn is_loopback(address: &str) -> bool {
    let host = address
        .rsplit_once(':')
        .filter(|(_, port)| port.chars().all(|c| c.is_ascii_digit()))
        .map_or(address, |(host, _)| host);
    matches!(host, "localhost" | "[::1]" | "::1") || host.starts_with("127.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docker_hub_has_one_address() {
        for alias in [
            "docker.io",
            "index.docker.io",
            "https://index.docker.io/v1/",
            "registry-1.docker.io",
            "Docker.IO",
        ] {
            assert_eq!(normalise(alias).as_deref(), Some(DOCKER_HUB), "{alias}");
        }
    }

    #[test]
    fn addresses_lose_their_scheme_path_and_case() {
        assert_eq!(normalise("https://GHCR.io/").as_deref(), Some("ghcr.io"));
        assert_eq!(
            normalise(" http://registry.local:5000/v2/ ").as_deref(),
            Some("registry.local:5000")
        );
        assert_eq!(normalise(""), None);
        assert_eq!(normalise("https://"), None);
        assert_eq!(normalise("bad host"), None);
    }

    #[test]
    fn references_name_their_registry() {
        assert_eq!(registry_of("nginx"), DOCKER_HUB);
        assert_eq!(registry_of("nginx:1.27"), DOCKER_HUB);
        assert_eq!(registry_of("library/nginx"), DOCKER_HUB);
        assert_eq!(registry_of("bitnami/redis:7"), DOCKER_HUB);
        assert_eq!(registry_of("docker.io/library/nginx"), DOCKER_HUB);
        assert_eq!(registry_of("ghcr.io/org/app:1"), "ghcr.io");
        assert_eq!(registry_of("localhost:5000/app"), "localhost:5000");
        assert_eq!(registry_of("localhost/app"), "localhost");
        assert_eq!(
            registry_of("registry.local:5000/team/app@sha256:abc"),
            "registry.local:5000"
        );
    }

    #[test]
    fn loopback_registries_are_recognised() {
        assert!(is_loopback("localhost:5000"));
        assert!(is_loopback("127.0.0.1:5000"));
        assert!(is_loopback("localhost"));
        assert!(!is_loopback("ghcr.io"));
        assert!(!is_loopback("registry.local:5000"));
    }
}

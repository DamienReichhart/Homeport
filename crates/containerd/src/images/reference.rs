//! Image references as containerd wants them: fully qualified.

/// `alpine` → `docker.io/library/alpine:latest`; `ghcr.io/x/y` →
/// `ghcr.io/x/y:latest`. containerd, unlike Docker, doesn't fill these in.
pub fn qualify(reference: &str) -> String {
    let reference = reference.trim();
    let (first, rest) = match reference.split_once('/') {
        Some((first, rest)) => (Some(first), rest),
        None => (None, reference),
    };
    let registry = first.filter(|f| f.contains('.') || f.contains(':') || *f == "localhost");
    let path = match (registry, first) {
        (Some(_), _) => rest.to_owned(),
        (None, Some(first)) => format!("{first}/{rest}"),
        (None, None) => format!("library/{rest}"),
    };
    let registry = registry.unwrap_or("docker.io");
    let name = path.rsplit('/').next().unwrap_or_default();
    let tagged = path.contains('@') || name.contains(':');
    if tagged {
        format!("{registry}/{path}")
    } else {
        format!("{registry}/{path}:latest")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn references_are_qualified_as_containerd_needs() {
        for (short, full) in [
            ("alpine", "docker.io/library/alpine:latest"),
            ("nginx:1.27", "docker.io/library/nginx:1.27"),
            ("bitnami/redis", "docker.io/bitnami/redis:latest"),
            ("ghcr.io/home/app", "ghcr.io/home/app:latest"),
            ("localhost:5000/app:dev", "localhost:5000/app:dev"),
            ("localhost/app", "localhost/app:latest"),
            (
                "alpine@sha256:49340b346798875b5592e5d420241f2f9cfba5f47e62ebb1446a1b34734e898f",
                "docker.io/library/alpine@sha256:49340b346798875b5592e5d420241f2f9cfba5f47e62ebb1446a1b34734e898f",
            ),
            ("docker.io/library/alpine:3.22", "docker.io/library/alpine:3.22"),
        ] {
            assert_eq!(qualify(short), full, "{short}");
        }
    }
}

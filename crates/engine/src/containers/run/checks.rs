//! Validation of the Run dialog's free-text options, each failure explained in
//! terms of what to type instead.

use std::net::IpAddr;

use homeport_core::error::{AppError, AppResult, Message, Text};

use crate::messages as msg;

/// Linux capabilities without their `CAP_` prefix, as `docker run --cap-add` takes them.
pub const CAPABILITIES: &[&str] = &[
    "AUDIT_CONTROL",
    "AUDIT_READ",
    "AUDIT_WRITE",
    "BLOCK_SUSPEND",
    "BPF",
    "CHECKPOINT_RESTORE",
    "CHOWN",
    "DAC_OVERRIDE",
    "DAC_READ_SEARCH",
    "FOWNER",
    "FSETID",
    "IPC_LOCK",
    "IPC_OWNER",
    "KILL",
    "LEASE",
    "LINUX_IMMUTABLE",
    "MAC_ADMIN",
    "MAC_OVERRIDE",
    "MKNOD",
    "NET_ADMIN",
    "NET_BIND_SERVICE",
    "NET_BROADCAST",
    "NET_RAW",
    "PERFMON",
    "SETFCAP",
    "SETGID",
    "SETPCAP",
    "SETUID",
    "SYS_ADMIN",
    "SYS_BOOT",
    "SYS_CHROOT",
    "SYS_MODULE",
    "SYS_NICE",
    "SYS_PACCT",
    "SYS_PTRACE",
    "SYS_RAWIO",
    "SYS_RESOURCE",
    "SYS_TIME",
    "SYS_TTY_CONFIG",
    "SYSLOG",
    "WAKE_ALARM",
];

/// Label keys Docker Compose owns; a container carrying them joins a Stack.
pub(super) const COMPOSE_LABEL_PREFIX: &str = "com.docker.compose.";

/// The smallest memory limit the Engine accepts.
pub(super) const MIN_MEMORY: i64 = 6 * 1024 * 1024;

fn invalid(message: impl Into<Text>) -> AppError {
    AppError::validation(message)
}

/// `NET_ADMIN`, `cap_net_admin` or `CAP_NET_ADMIN` as the Engine's
/// `CAP_NET_ADMIN`; `ALL` stays `ALL`.
pub(super) fn capability(name: &str) -> AppResult<String> {
    let upper = name.trim().to_ascii_uppercase();
    let bare = upper.strip_prefix("CAP_").unwrap_or(&upper);
    if bare == "ALL" {
        return Ok("ALL".to_owned());
    }
    if CAPABILITIES.contains(&bare) {
        Ok(format!("CAP_{bare}"))
    } else {
        Err(invalid(msg::RUN_CAPABILITY.with("name", name.trim())))
    }
}

/// `KEY=value` with a non-empty key; `not_key_value` says which list it's
/// in, with the entry as `{entry}`.
pub(super) fn key_value(entry: &str, not_key_value: Message) -> AppResult<(&str, &str)> {
    match entry.split_once('=') {
        Some((key, value)) if !key.trim().is_empty() => Ok((key.trim(), value)),
        _ => Err(invalid(not_key_value.with("entry", entry))),
    }
}

/// A user label, refusing Compose's own keys.
pub(super) fn label(entry: &str) -> AppResult<(&str, &str)> {
    let (key, value) = key_value(entry, msg::RUN_LABEL)?;
    if key.starts_with(COMPOSE_LABEL_PREFIX) {
        return Err(invalid(
            msg::RUN_COMPOSE_LABEL.with("prefix", COMPOSE_LABEL_PREFIX),
        ));
    }
    Ok((key, value))
}

/// A host or domain name: dot-separated labels of letters, digits and
/// hyphens. `not_valid` says which field it's in, with the name as `{name}`.
pub(super) fn host_name(name: &str, not_valid: Message) -> AppResult<()> {
    if is_host_name(name) {
        Ok(())
    } else {
        Err(invalid(not_valid.with("name", name)))
    }
}

fn is_host_name(name: &str) -> bool {
    let valid_label = |l: &str| {
        (1..=63).contains(&l.len())
            && l.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
            && !l.starts_with('-')
            && !l.ends_with('-')
    };
    name.len() <= 253 && name.split('.').all(valid_label)
}

/// A DNS server's IP address.
pub(super) fn dns_server(address: &str) -> AppResult<()> {
    address
        .parse::<IpAddr>()
        .map(|_| ())
        .map_err(|_| invalid(msg::RUN_DNS_SERVER.with("address", address)))
}

/// `name:ip` for `/etc/hosts`, where `ip` may be IPv6 or `host-gateway`.
pub(super) fn extra_host(entry: &str) -> AppResult<String> {
    let bad = || invalid(msg::RUN_EXTRA_HOST.with("entry", entry));
    let (name, address) = entry.split_once(':').ok_or_else(bad)?;
    let address = address.trim_start_matches('[').trim_end_matches(']');
    if !is_host_name(name) {
        return Err(bad());
    }
    if address != "host-gateway" && address.parse::<IpAddr>().is_err() {
        return Err(bad());
    }
    Ok(format!("{name}:{address}"))
}

/// Device cgroup permissions: some of `r`, `w` and `m`, each once.
pub(super) fn device_permissions(permissions: &str) -> AppResult<()> {
    let mut seen = [false; 3];
    for c in permissions.chars() {
        let i = "rwm".find(c).ok_or_else(bad_permissions)?;
        if std::mem::replace(&mut seen[i], true) {
            return Err(bad_permissions());
        }
    }
    if permissions.is_empty() {
        return Err(bad_permissions());
    }
    Ok(())
}

fn bad_permissions() -> AppError {
    invalid(msg::RUN_DEVICE_PERMISSIONS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capabilities_are_normalised() {
        assert_eq!(capability("net_admin").unwrap(), "CAP_NET_ADMIN");
        assert_eq!(capability("CAP_SYS_TIME").unwrap(), "CAP_SYS_TIME");
        assert_eq!(capability(" all ").unwrap(), "ALL");
        assert!(capability("NET_ADMINS").is_err());
    }

    #[test]
    fn labels_keep_compose_keys_out() {
        assert_eq!(
            label("traefik.enable=true").unwrap(),
            ("traefik.enable", "true")
        );
        assert_eq!(label("empty=").unwrap(), ("empty", ""));
        assert!(label("=value").is_err());
        assert!(label("novalue").is_err());
        assert!(label("com.docker.compose.project=shop").is_err());
    }

    #[test]
    fn host_names_are_dns_names() {
        assert!(host_name("web", msg::RUN_HOSTNAME).is_ok());
        assert!(host_name("web-1.home.arpa", msg::RUN_HOSTNAME).is_ok());
        for bad in ["", "-web", "web-", "we_b", "a..b", &"a".repeat(64)] {
            assert!(host_name(bad, msg::RUN_HOSTNAME).is_err(), "{bad}");
        }
    }

    #[test]
    fn extra_hosts_take_ipv4_ipv6_and_the_host_gateway() {
        assert_eq!(extra_host("nas:192.168.1.10").unwrap(), "nas:192.168.1.10");
        assert_eq!(extra_host("v6:[fd00::1]").unwrap(), "v6:fd00::1");
        assert_eq!(extra_host("v6:fd00::1").unwrap(), "v6:fd00::1");
        assert!(extra_host("host.docker.internal:host-gateway").is_ok());
        for bad in ["nas", "nas:", ":1.2.3.4", "nas:999.1.1.1", "n as:1.2.3.4"] {
            assert!(extra_host(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn dns_servers_are_addresses() {
        assert!(dns_server("1.1.1.1").is_ok());
        assert!(dns_server("2606:4700::1111").is_ok());
        assert!(dns_server("dns.example").is_err());
    }

    #[test]
    fn device_permissions_are_a_subset_of_rwm() {
        for good in ["r", "rw", "rwm", "mr"] {
            assert!(device_permissions(good).is_ok(), "{good}");
        }
        for bad in ["", "x", "rr", "rwmx"] {
            assert!(device_permissions(bad).is_err(), "{bad}");
        }
    }
}

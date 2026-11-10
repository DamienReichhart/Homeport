//! A [`RunSpec`] read back from an existing container, for Duplicate and
//! Edit: what the Run dialog would have needed to create it.

use bollard::models::{
    ContainerConfig, ContainerInspectResponse, HostConfig, ImageInspect, RestartPolicyNameEnum,
};

use super::{
    checks::COMPOSE_LABEL_PREFIX, DeviceSpec, GpuSpec, HealthcheckSpec, LogSpec, MountSpec,
    PortSpec, RestartPolicy, RunSpec,
};

/// Reads a container back into the Run dialog's terms. With its `image`,
/// values the container only inherited from the image (environment, labels,
/// command, healthcheck) are left out, so the form shows the overrides.
/// Stack labels, anonymous volumes, a generated hostname and the original's
/// name are never copied.
pub fn spec_from_inspect(
    container: &ContainerInspectResponse,
    image: Option<&ImageInspect>,
) -> RunSpec {
    let config = container.config.clone().unwrap_or_default();
    let host = container.host_config.clone().unwrap_or_default();
    let image_config = image.and_then(|i| i.config.clone()).unwrap_or_default();
    let id = container.id.as_deref().unwrap_or_default();

    let env = own(config.env.clone(), image_config.env.as_ref());
    let mut labels: Vec<String> = config
        .labels
        .clone()
        .unwrap_or_default()
        .into_iter()
        .filter(|(k, v)| {
            !k.starts_with(COMPOSE_LABEL_PREFIX)
                && image_config.labels.as_ref().and_then(|l| l.get(k)) != Some(v)
        })
        .map(|(k, v)| format!("{k}={v}"))
        .collect();
    labels.sort();
    let command = config
        .cmd
        .clone()
        .filter(|cmd| image_config.cmd.as_ref() != Some(cmd) && !cmd.is_empty())
        .map(|cmd| shell_words::join(&cmd));
    let healthcheck = config
        .healthcheck
        .clone()
        .filter(|h| image_config.healthcheck.as_ref() != Some(h))
        .and_then(|h| healthcheck(&h));
    let generated_hostname = config
        .hostname
        .as_deref()
        .is_some_and(|h| id.starts_with(h) && !h.is_empty());

    RunSpec {
        image: config.image.clone().unwrap_or_default(),
        name: None,
        command,
        env,
        ports: ports(&host),
        mounts: mounts(container, &host),
        network: host
            .network_mode
            .clone()
            .filter(|n| !matches!(n.as_str(), "" | "default" | "bridge")),
        restart: restart(&host),
        auto_remove: host.auto_remove.unwrap_or(false),
        cpus: host.nano_cpus.filter(|n| *n > 0).map(|n| n as f64 / 1e9),
        cpu_shares: host.cpu_shares.filter(|s| *s > 0),
        memory: host.memory.filter(|m| *m > 0),
        memory_reservation: host.memory_reservation.filter(|m| *m > 0),
        memory_swap: host.memory_swap.filter(|s| *s != 0),
        labels,
        cap_add: capabilities(host.cap_add.as_ref()),
        cap_drop: capabilities(host.cap_drop.as_ref()),
        privileged: host.privileged.unwrap_or(false),
        devices: devices(&host),
        gpus: gpus(&host),
        healthcheck,
        hostname: config
            .hostname
            .clone()
            .filter(|h| !h.is_empty() && !generated_hostname),
        domainname: config.domainname.clone().filter(|d| !d.is_empty()),
        dns: host.dns.clone().unwrap_or_default(),
        dns_search: host.dns_search.clone().unwrap_or_default(),
        extra_hosts: host.extra_hosts.clone().unwrap_or_default(),
        log: host.log_config.as_ref().and_then(|l| {
            let driver = l.typ.clone().filter(|t| !t.is_empty())?;
            let mut options: Vec<String> = l
                .config
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect();
            options.sort();
            Some(LogSpec { driver, options })
        }),
        based_on: container.id.clone(),
    }
}

/// The entries of `values` the image doesn't already set.
fn own(values: Option<Vec<String>>, image: Option<&Vec<String>>) -> Vec<String> {
    values
        .unwrap_or_default()
        .into_iter()
        .filter(|v| !image.is_some_and(|i| i.contains(v)))
        .collect()
}

fn ports(host: &HostConfig) -> Vec<PortSpec> {
    let mut ports: Vec<PortSpec> = host
        .port_bindings
        .clone()
        .unwrap_or_default()
        .into_iter()
        .flat_map(|(key, bindings)| {
            let (port, protocol) = key.split_once('/').unwrap_or((&key, "tcp"));
            let container_port: u16 = port.parse().unwrap_or_default();
            let protocol = protocol.to_owned();
            bindings
                .unwrap_or_default()
                .into_iter()
                .map(move |b| PortSpec {
                    host_ip: b.host_ip.filter(|ip| !ip.is_empty()),
                    host_port: b.host_port.and_then(|p| p.parse().ok()),
                    container_port,
                    protocol: protocol.clone(),
                })
                .collect::<Vec<_>>()
        })
        .filter(|p| p.container_port != 0)
        .collect();
    ports.sort_by_key(|p| (p.container_port, p.protocol.clone(), p.host_port));
    ports
}

/// Whether volume `name` is anonymous: named by a 64-digit hex ID, and not
/// one the user gave by that name.
fn is_anonymous(name: &str, host: &HostConfig) -> bool {
    let given_by_user = host
        .binds
        .iter()
        .flatten()
        .any(|b| b.split(':').next() == Some(name))
        || host
            .mounts
            .iter()
            .flatten()
            .any(|m| m.source.as_deref() == Some(name));
    name.len() == 64 && name.bytes().all(|b| b.is_ascii_hexdigit()) && !given_by_user
}

/// Where the container's anonymous volumes are mounted.
pub(crate) fn anonymous_volumes(container: &ContainerInspectResponse) -> Vec<String> {
    let host = container.host_config.clone().unwrap_or_default();
    container
        .mounts
        .iter()
        .flatten()
        .filter(|m| m.typ.as_deref() == Some("volume"))
        .filter(|m| m.name.as_deref().is_some_and(|n| is_anonymous(n, &host)))
        .filter_map(|m| m.destination.clone())
        .collect()
}

/// Bind mounts and named volumes; anonymous volumes are left for a fresh
/// one to be made.
fn mounts(container: &ContainerInspectResponse, host: &HostConfig) -> Vec<MountSpec> {
    container
        .mounts
        .iter()
        .flatten()
        .filter_map(|m| {
            let source = match m.typ.as_deref() {
                Some("bind") => m.source.clone()?,
                Some("volume") => {
                    let name = m.name.clone()?;
                    if is_anonymous(&name, host) {
                        return None;
                    }
                    name
                }
                _ => return None,
            };
            Some(MountSpec {
                source,
                target: m.destination.clone()?,
                read_only: m.rw == Some(false),
            })
        })
        .collect()
}

fn restart(host: &HostConfig) -> RestartPolicy {
    match host.restart_policy.as_ref().and_then(|r| r.name) {
        Some(RestartPolicyNameEnum::ALWAYS) => RestartPolicy::Always,
        Some(RestartPolicyNameEnum::UNLESS_STOPPED) => RestartPolicy::UnlessStopped,
        Some(RestartPolicyNameEnum::ON_FAILURE) => RestartPolicy::OnFailure,
        _ => RestartPolicy::No,
    }
}

/// `CAP_NET_ADMIN` as the dialog shows it, `NET_ADMIN`.
fn capabilities(caps: Option<&Vec<String>>) -> Vec<String> {
    caps.into_iter()
        .flatten()
        .map(|c| c.trim_start_matches("CAP_").to_owned())
        .collect()
}

fn devices(host: &HostConfig) -> Vec<DeviceSpec> {
    host.devices
        .iter()
        .flatten()
        .filter_map(|d| {
            let host_path = d.path_on_host.clone()?;
            let container_path = d.path_in_container.clone().filter(|p| *p != host_path);
            Some(DeviceSpec {
                host_path,
                container_path,
                permissions: d.cgroup_permissions.clone().unwrap_or_else(|| "rwm".into()),
            })
        })
        .collect()
}

fn gpus(host: &HostConfig) -> Option<GpuSpec> {
    let request = host.device_requests.iter().flatten().find(|r| {
        r.capabilities
            .iter()
            .flatten()
            .any(|set| set.iter().any(|c| c == "gpu"))
    })?;
    match (&request.device_ids, request.count) {
        (Some(ids), _) if !ids.is_empty() => Some(GpuSpec::Ids { ids: ids.clone() }),
        (_, Some(count)) if count > 0 => Some(GpuSpec::Count {
            count: u32::try_from(count).unwrap_or(1),
        }),
        _ => Some(GpuSpec::All),
    }
}

fn healthcheck(h: &bollard::models::HealthConfig) -> Option<HealthcheckSpec> {
    let test = h.test.as_ref()?;
    let seconds = |nanos: Option<i64>| {
        nanos
            .filter(|n| *n > 0)
            .and_then(|n| u32::try_from(n / 1_000_000_000).ok())
            .filter(|s| *s > 0)
    };
    let command = match test.first().map(String::as_str) {
        Some("NONE") => {
            return Some(HealthcheckSpec {
                disabled: true,
                ..Default::default()
            })
        }
        Some("CMD-SHELL") => test.get(1).cloned().unwrap_or_default(),
        Some("CMD") => shell_words::join(&test[1..]),
        _ => return None,
    };
    Some(HealthcheckSpec {
        command,
        interval_s: seconds(h.interval),
        timeout_s: seconds(h.timeout),
        retries: h
            .retries
            .and_then(|r| u32::try_from(r).ok())
            .filter(|r| *r > 0),
        start_period_s: seconds(h.start_period),
        disabled: false,
    })
}

/// The parts of a container's configuration the Run dialog doesn't show,
/// minus what its image already sets, so a new image brings its own.
pub(super) fn unshown(config: &ContainerConfig, image: Option<&ImageInspect>) -> ContainerConfig {
    let image = image.and_then(|i| i.config.clone()).unwrap_or_default();
    let differs = |value: &Option<Vec<String>>, base: &Option<Vec<String>>| {
        value
            .as_ref()
            .filter(|v| Some(*v) != base.as_ref() && !v.is_empty())
            .cloned()
    };
    ContainerConfig {
        user: config
            .user
            .clone()
            .filter(|u| Some(u) != image.user.as_ref() && !u.is_empty()),
        working_dir: config
            .working_dir
            .clone()
            .filter(|w| Some(w) != image.working_dir.as_ref() && !w.is_empty()),
        entrypoint: differs(&config.entrypoint, &image.entrypoint),
        shell: differs(&config.shell, &image.shell),
        stop_signal: config
            .stop_signal
            .clone()
            .filter(|s| Some(s) != image.stop_signal.as_ref()),
        volumes: config.volumes.as_ref().map(|v| {
            v.iter()
                .filter(|path| !image.volumes.iter().flatten().any(|i| i == *path))
                .cloned()
                .collect()
        }),
        tty: config.tty,
        open_stdin: config.open_stdin,
        stdin_once: config.stdin_once,
        attach_stdin: config.attach_stdin,
        attach_stdout: config.attach_stdout,
        attach_stderr: config.attach_stderr,
        stop_timeout: config.stop_timeout,
        network_disabled: config.network_disabled,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests;

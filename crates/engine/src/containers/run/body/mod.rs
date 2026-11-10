//! A [`RunSpec`] as the Engine's create request, validated on the way.

use std::collections::HashMap;

use bollard::models::{
    ContainerCreateBody, ContainerInspectResponse, DeviceMapping, DeviceRequest, EndpointSettings,
    HealthConfig, HostConfig, HostConfigLogConfig, ImageInspect, MountType, NetworkingConfig,
    PortBinding, RestartPolicy as EnginePolicy, RestartPolicyNameEnum,
};

use homeport_core::error::{AppError, AppResult, Message, Text};

use crate::messages as msg;

use super::{checks, from_inspect, GpuSpec, HealthcheckSpec, LogSpec, RestartPolicy, RunSpec};
use crate::COMPOSE_SERVICE;

const NANOS_PER_SECOND: i64 = 1_000_000_000;

fn invalid(message: impl Into<Text>) -> AppError {
    AppError::validation(message)
}

/// The container a spec is based on, for Duplicate and Edit
/// ([`RunSpec::based_on`]).
pub struct Base<'a> {
    pub inspect: &'a ContainerInspectResponse,
    /// Its image, to tell the container's own settings from inherited ones.
    pub image: Option<&'a ImageInspect>,
    /// The new container takes this one's place (Edit): it keeps its Stack
    /// labels and the network aliases that name it. A copy drops them.
    pub same_identity: bool,
}

impl Base<'_> {
    fn id(&self) -> &str {
        self.inspect.id.as_deref().unwrap_or_default()
    }

    fn name(&self) -> &str {
        self.inspect
            .name
            .as_deref()
            .unwrap_or_default()
            .trim_start_matches('/')
    }

    fn primary_network(&self) -> &str {
        let host = self.inspect.host_config.as_ref();
        match host.and_then(|h| h.network_mode.as_deref()) {
            None | Some("" | "default") => "bridge",
            Some(mode) => mode,
        }
    }

    /// Aliases on `network` worth keeping: never the Engine's own (the short
    /// ID), and for a copy not those naming the original or its service.
    fn aliases(&self, network: &str) -> Vec<String> {
        let id = self.id();
        let service = self
            .inspect
            .config
            .as_ref()
            .and_then(|c| c.labels.as_ref())
            .and_then(|l| l.get(COMPOSE_SERVICE));
        self.inspect
            .network_settings
            .as_ref()
            .and_then(|n| n.networks.as_ref())
            .and_then(|n| n.get(network))
            .and_then(|e| e.aliases.clone())
            .unwrap_or_default()
            .into_iter()
            .filter(|a| !a.is_empty() && !id.starts_with(a.as_str()))
            .filter(|a| self.same_identity || (a != self.name() && Some(a) != service))
            .collect()
    }

    /// The original's networks besides its primary one and the spec's, with
    /// their aliases, to connect the new container to after creating it.
    pub(crate) fn other_networks(&self, spec: &RunSpec) -> Vec<(String, Vec<String>)> {
        let primary = optional(&spec.network).unwrap_or("bridge");
        if !user_defined(primary) && primary != "bridge" {
            return vec![];
        }
        let mut networks: Vec<(String, Vec<String>)> = self
            .inspect
            .network_settings
            .as_ref()
            .and_then(|n| n.networks.as_ref())
            .into_iter()
            .flatten()
            .map(|(name, _)| name)
            .filter(|name| *name != primary && *name != self.primary_network())
            .map(|name| (name.clone(), self.aliases(name)))
            .collect();
        networks.sort();
        networks
    }
}

/// Networks a container can join by name, with aliases; not the default
/// bridge, `host`, `none` or another container's stack.
fn user_defined(network: &str) -> bool {
    !matches!(network, "bridge" | "host" | "none" | "default") && !network.starts_with("container:")
}

/// Starts from the base container's settings the dialog doesn't show.
fn from_base(base: &Base) -> (ContainerCreateBody, HostConfig) {
    let config =
        from_inspect::unshown(&base.inspect.config.clone().unwrap_or_default(), base.image);
    let body = ContainerCreateBody {
        user: config.user,
        working_dir: config.working_dir,
        entrypoint: config.entrypoint,
        shell: config.shell,
        stop_signal: config.stop_signal,
        stop_timeout: config.stop_timeout,
        volumes: config.volumes.filter(|v| !v.is_empty()),
        tty: config.tty,
        open_stdin: config.open_stdin,
        stdin_once: config.stdin_once,
        attach_stdin: config.attach_stdin,
        attach_stdout: config.attach_stdout,
        attach_stderr: config.attach_stderr,
        network_disabled: config.network_disabled,
        ..Default::default()
    };
    let mut host = base.inspect.host_config.clone().unwrap_or_default();
    // Binds and volumes come back from the spec; only tmpfs mounts stay.
    host.mounts = host
        .mounts
        .map(|m| {
            m.into_iter()
                .filter(|m| m.typ == Some(MountType::TMPFS))
                .collect::<Vec<_>>()
        })
        .filter(|m| !m.is_empty());
    host.container_id_file = None;
    (body, host)
}

/// Translates a [`RunSpec`] into the Engine's create request, on top of
/// `base`'s settings when it has one.
pub fn create_body(spec: &RunSpec, base: Option<&Base>) -> AppResult<ContainerCreateBody> {
    let image = spec.image.trim();
    if image.is_empty() {
        return Err(invalid(msg::RUN_IMAGE));
    }
    let cmd = match spec.command.as_deref().map(str::trim) {
        Some(c) if !c.is_empty() => {
            Some(shell_words::split(c).map_err(|_| invalid(msg::COMMAND_UNCLOSED_QUOTE))?)
        }
        _ => None,
    };
    for entry in &spec.env {
        if !entry.contains('=') || entry.starts_with('=') {
            return Err(invalid(msg::RUN_ENV.with("entry", entry)));
        }
    }
    let (exposed, bindings) = ports(spec)?;
    let binds = binds(spec)?;
    let labels = spec
        .labels
        .iter()
        .map(|l| checks::label(l).map(|(k, v)| (k.to_owned(), v.to_owned())))
        .collect::<AppResult<HashMap<_, _>>>()?;
    let hostname = optional(&spec.hostname);
    if let Some(name) = hostname {
        checks::host_name(name, msg::RUN_HOSTNAME)?;
    }
    let domainname = optional(&spec.domainname);
    if let Some(name) = domainname {
        checks::host_name(name, msg::RUN_DOMAIN_NAME)?;
    }

    let restart = match spec.restart {
        RestartPolicy::No => RestartPolicyNameEnum::NO,
        RestartPolicy::Always => RestartPolicyNameEnum::ALWAYS,
        RestartPolicy::UnlessStopped => RestartPolicyNameEnum::UNLESS_STOPPED,
        RestartPolicy::OnFailure => RestartPolicyNameEnum::ON_FAILURE,
    };
    if spec.auto_remove && spec.restart != RestartPolicy::No {
        return Err(invalid(msg::RUN_AUTO_REMOVE_RESTART));
    }

    let (mut body, mut host) = base.map(from_base).unwrap_or_default();
    host.port_bindings = (!bindings.is_empty()).then_some(bindings);
    host.binds = (!binds.is_empty()).then_some(binds);
    host.network_mode = optional(&spec.network).map(str::to_owned);
    host.restart_policy = Some(EnginePolicy {
        name: Some(restart),
        maximum_retry_count: None,
    });
    host.auto_remove = Some(spec.auto_remove);
    resources(spec, &mut host)?;
    security(spec, &mut host)?;
    network_identity(spec, &mut host)?;
    host.log_config = spec.log.as_ref().map(log_config).transpose()?;

    let mut labels = labels;
    if let Some(base) = base.filter(|b| b.same_identity) {
        let stack = base.inspect.config.as_ref().and_then(|c| c.labels.as_ref());
        for (k, v) in stack.into_iter().flatten() {
            if k.starts_with(checks::COMPOSE_LABEL_PREFIX) {
                labels.insert(k.clone(), v.clone());
            }
        }
    }
    if let Some(base) = base {
        let network = optional(&spec.network).unwrap_or("bridge");
        let aliases = base.aliases(base.primary_network());
        if user_defined(network) && network == base.primary_network() && !aliases.is_empty() {
            body.networking_config = Some(NetworkingConfig {
                endpoints_config: Some(HashMap::from([(
                    network.to_owned(),
                    EndpointSettings {
                        aliases: Some(aliases),
                        ..Default::default()
                    },
                )])),
            });
        }
    }

    body.image = Some(image.to_owned());
    body.cmd = cmd;
    body.env = (!spec.env.is_empty()).then(|| spec.env.clone());
    body.exposed_ports = (!exposed.is_empty()).then_some(exposed);
    body.labels = (!labels.is_empty()).then_some(labels);
    body.healthcheck = spec.healthcheck.as_ref().map(healthcheck).transpose()?;
    body.hostname = hostname.map(str::to_owned);
    body.domainname = domainname.map(str::to_owned);
    body.host_config = Some(host);
    Ok(body)
}

fn optional(value: &Option<String>) -> Option<&str> {
    value.as_deref().map(str::trim).filter(|v| !v.is_empty())
}

type Bindings = HashMap<String, Option<Vec<PortBinding>>>;

fn ports(spec: &RunSpec) -> AppResult<(Vec<String>, Bindings)> {
    let mut exposed = Vec::new();
    let mut bindings = Bindings::new();
    for p in &spec.ports {
        let protocol = match p.protocol.as_str() {
            "tcp" | "udp" | "sctp" => p.protocol.as_str(),
            _ => return Err(invalid(msg::RUN_PORT_PROTOCOL)),
        };
        let key = format!("{}/{protocol}", p.container_port);
        if !exposed.contains(&key) {
            exposed.push(key.clone());
        }
        bindings
            .entry(key)
            .or_insert_with(|| Some(vec![]))
            .get_or_insert_with(Vec::new)
            .push(PortBinding {
                host_ip: p.host_ip.clone().filter(|ip| !ip.is_empty()),
                host_port: Some(p.host_port.map(|h| h.to_string()).unwrap_or_default()),
            });
    }
    Ok((exposed, bindings))
}

fn binds(spec: &RunSpec) -> AppResult<Vec<String>> {
    spec.mounts
        .iter()
        .map(|m| {
            if m.source.trim().is_empty() || !m.target.starts_with('/') {
                return Err(invalid(msg::RUN_MOUNT));
            }
            let mode = if m.read_only { ":ro" } else { "" };
            Ok(format!("{}:{}{mode}", m.source.trim(), m.target))
        })
        .collect()
}

/// CPU and memory limits (`--cpus`, `--cpu-shares`, `--memory…`).
fn resources(spec: &RunSpec, host: &mut HostConfig) -> AppResult<()> {
    if let Some(cpus) = spec.cpus {
        if !cpus.is_finite() || cpus <= 0.0 {
            return Err(invalid(msg::RUN_CPUS));
        }
        host.nano_cpus = Some((cpus * NANOS_PER_SECOND as f64).round() as i64);
    }
    if let Some(shares) = spec.cpu_shares {
        if shares < 2 {
            return Err(invalid(msg::RUN_CPU_SHARES));
        }
        host.cpu_shares = Some(shares);
    }
    let positive = |value: Option<i64>, not_positive: Message| match value {
        Some(v) if v <= 0 => Err(invalid(not_positive)),
        other => Ok(other),
    };
    let memory = positive(spec.memory, msg::RUN_MEMORY_ABOVE_ZERO)?;
    let reservation = positive(
        spec.memory_reservation,
        msg::RUN_MEMORY_RESERVATION_ABOVE_ZERO,
    )?;
    if memory.is_some_and(|m| m < checks::MIN_MEMORY) {
        return Err(invalid(msg::RUN_MEMORY_MINIMUM));
    }
    if let (Some(memory), Some(reservation)) = (memory, reservation) {
        if reservation > memory {
            return Err(invalid(msg::RUN_RESERVATION_ABOVE_LIMIT));
        }
    }
    match (spec.memory_swap, memory) {
        (None | Some(-1), _) => {}
        (Some(_), None) => {
            return Err(invalid(msg::RUN_SWAP_WITHOUT_MEMORY));
        }
        (Some(swap), Some(memory)) if swap < memory => {
            return Err(invalid(msg::RUN_SWAP_BELOW_MEMORY));
        }
        _ => {}
    }
    host.memory = memory;
    host.memory_reservation = reservation;
    host.memory_swap = spec.memory_swap;
    Ok(())
}

/// Capabilities, privileged mode, devices and GPUs.
fn security(spec: &RunSpec, host: &mut HostConfig) -> AppResult<()> {
    let caps = |names: &[String]| {
        names
            .iter()
            .filter(|n| !n.trim().is_empty())
            .map(|n| checks::capability(n))
            .collect::<AppResult<Vec<_>>>()
    };
    let cap_add = caps(&spec.cap_add)?;
    let cap_drop = caps(&spec.cap_drop)?;
    host.cap_add = (!cap_add.is_empty()).then_some(cap_add);
    host.cap_drop = (!cap_drop.is_empty()).then_some(cap_drop);
    host.privileged = spec.privileged.then_some(true);

    let devices = spec
        .devices
        .iter()
        .map(|d| {
            let host_path = d.host_path.trim();
            let container_path = optional(&d.container_path).unwrap_or(host_path);
            if !host_path.starts_with('/') || !container_path.starts_with('/') {
                return Err(invalid(msg::RUN_DEVICE_PATH));
            }
            let permissions = d.permissions.trim();
            checks::device_permissions(permissions)?;
            Ok(DeviceMapping {
                path_on_host: Some(host_path.to_owned()),
                path_in_container: Some(container_path.to_owned()),
                cgroup_permissions: Some(permissions.to_owned()),
            })
        })
        .collect::<AppResult<Vec<_>>>()?;
    host.devices = (!devices.is_empty()).then_some(devices);

    // As `docker run --gpus` does: no driver, the `gpu` capability.
    host.device_requests = match &spec.gpus {
        None => None,
        Some(gpus) => {
            let (count, device_ids) = match gpus {
                GpuSpec::All => (Some(-1), None),
                GpuSpec::Count { count: 0 } => {
                    return Err(invalid(msg::RUN_GPU_COUNT));
                }
                GpuSpec::Count { count } => (Some(i64::from(*count)), None),
                GpuSpec::Ids { ids } => {
                    let ids: Vec<String> = ids
                        .iter()
                        .map(|i| i.trim().to_owned())
                        .filter(|i| !i.is_empty())
                        .collect();
                    if ids.is_empty() {
                        return Err(invalid(msg::RUN_GPU_IDS));
                    }
                    (None, Some(ids))
                }
            };
            Some(vec![DeviceRequest {
                count,
                device_ids,
                capabilities: Some(vec![vec!["gpu".to_owned()]]),
                ..Default::default()
            }])
        }
    };
    Ok(())
}

/// DNS servers, search domains and `/etc/hosts` entries.
fn network_identity(spec: &RunSpec, host: &mut HostConfig) -> AppResult<()> {
    let trimmed = |list: &[String]| -> Vec<String> {
        list.iter()
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect()
    };
    let dns = trimmed(&spec.dns);
    for server in &dns {
        checks::dns_server(server)?;
    }
    let search = trimmed(&spec.dns_search);
    for domain in &search {
        checks::host_name(domain, msg::RUN_SEARCH_DOMAIN)?;
    }
    let extra_hosts = trimmed(&spec.extra_hosts)
        .iter()
        .map(|h| checks::extra_host(h))
        .collect::<AppResult<Vec<_>>>()?;
    host.dns = (!dns.is_empty()).then_some(dns);
    host.dns_search = (!search.is_empty()).then_some(search);
    host.extra_hosts = (!extra_hosts.is_empty()).then_some(extra_hosts);
    Ok(())
}

fn healthcheck(spec: &HealthcheckSpec) -> AppResult<HealthConfig> {
    if spec.disabled {
        return Ok(HealthConfig {
            test: Some(vec!["NONE".to_owned()]),
            ..Default::default()
        });
    }
    let command = spec.command.trim();
    if command.is_empty() {
        return Err(invalid(msg::RUN_HEALTHCHECK_COMMAND));
    }
    let nanos = |seconds: Option<u32>, zero: Message| match seconds {
        Some(0) => Err(invalid(zero)),
        other => Ok(other.map(|s| i64::from(s) * NANOS_PER_SECOND)),
    };
    Ok(HealthConfig {
        test: Some(vec!["CMD-SHELL".to_owned(), command.to_owned()]),
        interval: nanos(spec.interval_s, msg::RUN_HEALTHCHECK_INTERVAL)?,
        timeout: nanos(spec.timeout_s, msg::RUN_HEALTHCHECK_TIMEOUT)?,
        retries: spec.retries.map(i64::from),
        // A start period of 0 is the Engine's default, so it's allowed.
        start_period: spec.start_period_s.map(|s| i64::from(s) * NANOS_PER_SECOND),
        start_interval: None,
    })
}

fn log_config(spec: &LogSpec) -> AppResult<HostConfigLogConfig> {
    let driver = spec.driver.trim();
    if driver.is_empty() || driver.contains(char::is_whitespace) {
        return Err(invalid(msg::RUN_LOG_DRIVER));
    }
    let options = spec
        .options
        .iter()
        .filter(|o| !o.trim().is_empty())
        .map(|o| {
            checks::key_value(o.trim(), msg::RUN_LOG_OPTION)
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
        })
        .collect::<AppResult<HashMap<_, _>>>()?;
    Ok(HostConfigLogConfig {
        typ: Some(driver.to_owned()),
        config: (!options.is_empty()).then_some(options),
    })
}

#[cfg(test)]
mod tests;

//! A [`RunSpec`] as the Engine's create request, validated on the way.

use std::collections::HashMap;

use bollard::models::{
    ContainerCreateBody, ContainerInspectResponse, DeviceMapping, DeviceRequest, EndpointSettings,
    HealthConfig, HostConfig, HostConfigLogConfig, ImageInspect, MountType, NetworkingConfig,
    PortBinding, RestartPolicy as EnginePolicy, RestartPolicyNameEnum,
};

use homeport_core::error::{AppError, AppResult};

use super::{checks, from_inspect, GpuSpec, HealthcheckSpec, LogSpec, RestartPolicy, RunSpec};
use crate::COMPOSE_SERVICE;

const NANOS_PER_SECOND: i64 = 1_000_000_000;

fn invalid(message: impl Into<String>) -> AppError {
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
        return Err(invalid("Choose an image."));
    }
    let cmd = match spec.command.as_deref().map(str::trim) {
        Some(c) if !c.is_empty() => {
            Some(shell_words::split(c).map_err(|_| invalid("The command has an unclosed quote."))?)
        }
        _ => None,
    };
    for entry in &spec.env {
        if !entry.contains('=') || entry.starts_with('=') {
            return Err(AppError::validation(format!(
                "\"{entry}\" isn't a KEY=value environment variable."
            )));
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
        checks::host_name(name, "hostname")?;
    }
    let domainname = optional(&spec.domainname);
    if let Some(name) = domainname {
        checks::host_name(name, "domain name")?;
    }

    let restart = match spec.restart {
        RestartPolicy::No => RestartPolicyNameEnum::NO,
        RestartPolicy::Always => RestartPolicyNameEnum::ALWAYS,
        RestartPolicy::UnlessStopped => RestartPolicyNameEnum::UNLESS_STOPPED,
        RestartPolicy::OnFailure => RestartPolicyNameEnum::ON_FAILURE,
    };
    if spec.auto_remove && spec.restart != RestartPolicy::No {
        return Err(invalid("Containers removed on exit can't also restart."));
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
            _ => return Err(invalid("Port protocols are tcp, udp or sctp.")),
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
                return Err(invalid(
                    "Mounts need a source and an absolute container path.",
                ));
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
            return Err(invalid(
                "The CPU limit must be a number above 0, like 0.5 or 2.",
            ));
        }
        host.nano_cpus = Some((cpus * NANOS_PER_SECOND as f64).round() as i64);
    }
    if let Some(shares) = spec.cpu_shares {
        if shares < 2 {
            return Err(invalid(
                "CPU shares must be 2 or more; 1024 is the default weight.",
            ));
        }
        host.cpu_shares = Some(shares);
    }
    let positive = |value: Option<i64>, what: &str| match value {
        Some(v) if v <= 0 => Err(invalid(format!("The {what} must be above 0."))),
        other => Ok(other),
    };
    let memory = positive(spec.memory, "memory limit")?;
    let reservation = positive(spec.memory_reservation, "memory reservation")?;
    if memory.is_some_and(|m| m < checks::MIN_MEMORY) {
        return Err(invalid("The memory limit must be at least 6 MB."));
    }
    if let (Some(memory), Some(reservation)) = (memory, reservation) {
        if reservation > memory {
            return Err(invalid(
                "The memory reservation can't be above the memory limit.",
            ));
        }
    }
    match (spec.memory_swap, memory) {
        (None | Some(-1), _) => {}
        (Some(_), None) => {
            return Err(invalid("Set a memory limit before limiting swap."));
        }
        (Some(swap), Some(memory)) if swap < memory => {
            return Err(invalid(
                "The memory and swap limit counts memory too, so it can't be below the memory limit. Use -1 for unlimited swap.",
            ));
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
                return Err(invalid(
                    "Device paths must be absolute, like /dev/dri or /dev/ttyUSB0.",
                ));
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
                    return Err(invalid("Request at least one GPU."));
                }
                GpuSpec::Count { count } => (Some(i64::from(*count)), None),
                GpuSpec::Ids { ids } => {
                    let ids: Vec<String> = ids
                        .iter()
                        .map(|i| i.trim().to_owned())
                        .filter(|i| !i.is_empty())
                        .collect();
                    if ids.is_empty() {
                        return Err(invalid("List the GPUs to use, by index or UUID."));
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
        checks::host_name(domain, "search domain")?;
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
        return Err(invalid(
            "Enter the healthcheck's command, or turn off the image's healthcheck.",
        ));
    }
    let nanos = |seconds: Option<u32>, what: &str| match seconds {
        Some(0) => Err(invalid(format!(
            "The healthcheck's {what} must be above 0."
        ))),
        other => Ok(other.map(|s| i64::from(s) * NANOS_PER_SECOND)),
    };
    Ok(HealthConfig {
        test: Some(vec!["CMD-SHELL".to_owned(), command.to_owned()]),
        interval: nanos(spec.interval_s, "interval")?,
        timeout: nanos(spec.timeout_s, "timeout")?,
        retries: spec.retries.map(i64::from),
        // A start period of 0 is the Engine's default, so it's allowed.
        start_period: spec.start_period_s.map(|s| i64::from(s) * NANOS_PER_SECOND),
        start_interval: None,
    })
}

fn log_config(spec: &LogSpec) -> AppResult<HostConfigLogConfig> {
    let driver = spec.driver.trim();
    if driver.is_empty() || driver.contains(char::is_whitespace) {
        return Err(invalid("Choose a log driver, like json-file or local."));
    }
    let options = spec
        .options
        .iter()
        .filter(|o| !o.trim().is_empty())
        .map(|o| {
            checks::key_value(o.trim(), "log option").map(|(k, v)| (k.to_owned(), v.to_owned()))
        })
        .collect::<AppResult<HashMap<_, _>>>()?;
    Ok(HostConfigLogConfig {
        typ: Some(driver.to_owned()),
        config: (!options.is_empty()).then_some(options),
    })
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::containers::run::{DeviceSpec, MountSpec, PortSpec};

    fn spec() -> RunSpec {
        RunSpec {
            image: "nginx:1.27".into(),
            ..Default::default()
        }
    }

    fn host(spec: RunSpec) -> HostConfig {
        create_body(&spec, None).unwrap().host_config.unwrap()
    }

    #[test]
    fn run_options_map_to_the_engine_request() {
        let body = create_body(
            &RunSpec {
                command: Some(r#"sh -c "echo 'hi there'""#.into()),
                env: vec!["MODE=prod".into()],
                ports: vec![
                    PortSpec {
                        host_ip: None,
                        host_port: Some(8080),
                        container_port: 80,
                        protocol: "tcp".into(),
                    },
                    PortSpec {
                        host_ip: Some("127.0.0.1".into()),
                        host_port: None,
                        container_port: 53,
                        protocol: "udp".into(),
                    },
                ],
                mounts: vec![
                    MountSpec {
                        source: "/srv/site".into(),
                        target: "/usr/share/nginx/html".into(),
                        read_only: true,
                    },
                    MountSpec {
                        source: "cache".into(),
                        target: "/cache".into(),
                        read_only: false,
                    },
                ],
                restart: RestartPolicy::UnlessStopped,
                ..spec()
            },
            None,
        )
        .unwrap();
        assert_eq!(body.cmd.unwrap(), ["sh", "-c", "echo 'hi there'"]);
        assert_eq!(body.env.unwrap(), ["MODE=prod"]);
        let host = body.host_config.unwrap();
        let ports = host.port_bindings.unwrap();
        assert_eq!(
            ports["80/tcp"].as_ref().unwrap()[0].host_port.as_deref(),
            Some("8080")
        );
        let dns = &ports["53/udp"].as_ref().unwrap()[0];
        assert_eq!(
            (dns.host_ip.as_deref(), dns.host_port.as_deref()),
            (Some("127.0.0.1"), Some(""))
        );
        assert_eq!(
            host.binds.unwrap(),
            ["/srv/site:/usr/share/nginx/html:ro", "cache:/cache"]
        );
        assert_eq!(
            host.restart_policy.unwrap().name,
            Some(RestartPolicyNameEnum::UNLESS_STOPPED)
        );
    }

    #[test]
    fn limits_and_labels_map_to_the_engine_request() {
        let body = create_body(
            &RunSpec {
                cpus: Some(0.5),
                cpu_shares: Some(512),
                memory: Some(512 * 1024 * 1024),
                memory_reservation: Some(256 * 1024 * 1024),
                memory_swap: Some(-1),
                labels: vec!["traefik.enable=true".into(), "team=web".into()],
                ..spec()
            },
            None,
        )
        .unwrap();
        let labels = body.labels.unwrap();
        assert_eq!(labels["traefik.enable"], "true");
        assert_eq!(labels["team"], "web");
        let host = body.host_config.unwrap();
        assert_eq!(host.nano_cpus, Some(500_000_000));
        assert_eq!(host.cpu_shares, Some(512));
        assert_eq!(host.memory, Some(512 * 1024 * 1024));
        assert_eq!(host.memory_reservation, Some(256 * 1024 * 1024));
        assert_eq!(host.memory_swap, Some(-1));
    }

    #[test]
    fn security_options_map_to_the_engine_request() {
        let host = host(RunSpec {
            cap_add: vec!["net_admin".into()],
            cap_drop: vec!["ALL".into()],
            privileged: true,
            devices: vec![
                DeviceSpec {
                    host_path: "/dev/dri".into(),
                    container_path: None,
                    permissions: "rwm".into(),
                },
                DeviceSpec {
                    host_path: "/dev/ttyUSB0".into(),
                    container_path: Some("/dev/zigbee".into()),
                    permissions: "rw".into(),
                },
            ],
            gpus: Some(GpuSpec::All),
            ..spec()
        });
        assert_eq!(host.cap_add.unwrap(), ["CAP_NET_ADMIN"]);
        assert_eq!(host.cap_drop.unwrap(), ["ALL"]);
        assert_eq!(host.privileged, Some(true));
        let devices = host.devices.unwrap();
        assert_eq!(devices[0].path_in_container.as_deref(), Some("/dev/dri"));
        assert_eq!(devices[1].path_in_container.as_deref(), Some("/dev/zigbee"));
        assert_eq!(devices[1].cgroup_permissions.as_deref(), Some("rw"));
        let gpu = &host.device_requests.unwrap()[0];
        assert_eq!(gpu.count, Some(-1));
        assert_eq!(gpu.driver, None);
        assert_eq!(gpu.capabilities, Some(vec![vec!["gpu".to_owned()]]));
    }

    #[test]
    fn gpus_can_be_counted_or_named() {
        let requests = |gpus| {
            host(RunSpec {
                gpus: Some(gpus),
                ..spec()
            })
            .device_requests
            .unwrap()
        };
        assert_eq!(requests(GpuSpec::Count { count: 2 })[0].count, Some(2));
        let named = requests(GpuSpec::Ids {
            ids: vec!["0".into(), " GPU-abc ".into()],
        });
        assert_eq!(named[0].count, None);
        assert_eq!(
            named[0].device_ids.as_deref(),
            Some(&["0".to_owned(), "GPU-abc".to_owned()][..])
        );
    }

    #[test]
    fn healthchecks_network_identity_and_logging_map_to_the_engine_request() {
        let body = create_body(
            &RunSpec {
                healthcheck: Some(HealthcheckSpec {
                    command: "wget -qO- localhost || exit 1".into(),
                    interval_s: Some(30),
                    timeout_s: Some(5),
                    retries: Some(3),
                    start_period_s: Some(90),
                    disabled: false,
                }),
                hostname: Some("web".into()),
                domainname: Some("home.arpa".into()),
                dns: vec!["1.1.1.1".into(), "".into()],
                dns_search: vec!["home.arpa".into()],
                extra_hosts: vec!["nas:192.168.1.10".into(), "gw:host-gateway".into()],
                log: Some(LogSpec {
                    driver: "json-file".into(),
                    options: vec!["max-size=10m".into(), "max-file=3".into()],
                }),
                ..spec()
            },
            None,
        )
        .unwrap();
        let health = body.healthcheck.unwrap();
        assert_eq!(
            health.test.unwrap(),
            ["CMD-SHELL", "wget -qO- localhost || exit 1"]
        );
        assert_eq!(health.interval, Some(30_000_000_000));
        assert_eq!(health.timeout, Some(5_000_000_000));
        assert_eq!(health.retries, Some(3));
        assert_eq!(health.start_period, Some(90_000_000_000));
        assert_eq!(body.hostname.as_deref(), Some("web"));
        assert_eq!(body.domainname.as_deref(), Some("home.arpa"));
        let host = body.host_config.unwrap();
        assert_eq!(host.dns.unwrap(), ["1.1.1.1"]);
        assert_eq!(host.dns_search.unwrap(), ["home.arpa"]);
        assert_eq!(
            host.extra_hosts.unwrap(),
            ["nas:192.168.1.10", "gw:host-gateway"]
        );
        let log = host.log_config.unwrap();
        assert_eq!(log.typ.as_deref(), Some("json-file"));
        assert_eq!(log.config.unwrap()["max-size"], "10m");
    }

    #[test]
    fn disabling_the_healthcheck_sends_none() {
        let body = create_body(
            &RunSpec {
                healthcheck: Some(HealthcheckSpec {
                    disabled: true,
                    ..Default::default()
                }),
                ..spec()
            },
            None,
        )
        .unwrap();
        assert_eq!(body.healthcheck.unwrap().test.unwrap(), ["NONE"]);
    }

    #[test]
    fn unset_options_leave_the_engine_defaults() {
        let body = create_body(&spec(), None).unwrap();
        assert_eq!(body.labels, None);
        assert_eq!(body.healthcheck, None);
        let host = body.host_config.unwrap();
        assert_eq!(
            (host.nano_cpus, host.memory, host.cap_add, host.privileged),
            (None, None, None, None)
        );
        assert_eq!(
            (host.devices, host.device_requests, host.log_config),
            (None, None, None)
        );
    }

    #[test]
    fn invalid_run_options_are_explained() {
        let mb = 1024 * 1024;
        let device = |host_path: &str, permissions: &str| DeviceSpec {
            host_path: host_path.into(),
            container_path: None,
            permissions: permissions.into(),
        };
        let bad = [
            RunSpec {
                image: " ".into(),
                ..spec()
            },
            RunSpec {
                env: vec!["NOVALUE".into()],
                ..spec()
            },
            RunSpec {
                command: Some("echo 'oops".into()),
                ..spec()
            },
            RunSpec {
                auto_remove: true,
                restart: RestartPolicy::Always,
                ..spec()
            },
            RunSpec {
                mounts: vec![MountSpec {
                    source: "data".into(),
                    target: "relative".into(),
                    read_only: false,
                }],
                ..spec()
            },
            RunSpec {
                cpus: Some(0.0),
                ..spec()
            },
            RunSpec {
                cpus: Some(f64::NAN),
                ..spec()
            },
            RunSpec {
                cpu_shares: Some(1),
                ..spec()
            },
            RunSpec {
                memory: Some(mb),
                ..spec()
            },
            RunSpec {
                memory: Some(-5),
                ..spec()
            },
            RunSpec {
                memory: Some(64 * mb),
                memory_reservation: Some(128 * mb),
                ..spec()
            },
            RunSpec {
                memory_swap: Some(128 * mb),
                ..spec()
            },
            RunSpec {
                memory: Some(128 * mb),
                memory_swap: Some(64 * mb),
                ..spec()
            },
            RunSpec {
                labels: vec!["novalue".into()],
                ..spec()
            },
            RunSpec {
                labels: vec!["com.docker.compose.project=shop".into()],
                ..spec()
            },
            RunSpec {
                cap_add: vec!["NOT_A_CAP".into()],
                ..spec()
            },
            RunSpec {
                devices: vec![device("dev/dri", "rwm")],
                ..spec()
            },
            RunSpec {
                devices: vec![device("/dev/dri", "rwx")],
                ..spec()
            },
            RunSpec {
                gpus: Some(GpuSpec::Count { count: 0 }),
                ..spec()
            },
            RunSpec {
                gpus: Some(GpuSpec::Ids {
                    ids: vec![" ".into()],
                }),
                ..spec()
            },
            RunSpec {
                healthcheck: Some(HealthcheckSpec {
                    command: " ".into(),
                    ..Default::default()
                }),
                ..spec()
            },
            RunSpec {
                healthcheck: Some(HealthcheckSpec {
                    command: "true".into(),
                    interval_s: Some(0),
                    ..Default::default()
                }),
                ..spec()
            },
            RunSpec {
                hostname: Some("not_valid".into()),
                ..spec()
            },
            RunSpec {
                dns: vec!["dns.example".into()],
                ..spec()
            },
            RunSpec {
                extra_hosts: vec!["nas".into()],
                ..spec()
            },
            RunSpec {
                log: Some(LogSpec {
                    driver: "json-file".into(),
                    options: vec!["max-size".into()],
                }),
                ..spec()
            },
            RunSpec {
                log: Some(LogSpec {
                    driver: " ".into(),
                    options: vec![],
                }),
                ..spec()
            },
        ];
        for s in bad {
            assert!(
                matches!(create_body(&s, None), Err(ref e) if e.is(ErrorCode::Validation)),
                "{s:?}"
            );
        }
    }
}

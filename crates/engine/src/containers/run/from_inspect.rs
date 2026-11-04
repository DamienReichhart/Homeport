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

/// Bind mounts and named volumes. Anonymous volumes (named by a 64-digit
/// hex ID and not given by the user) are left for a fresh one to be made.
fn mounts(container: &ContainerInspectResponse, host: &HostConfig) -> Vec<MountSpec> {
    let given_by_user = |name: &str| {
        host.binds
            .iter()
            .flatten()
            .any(|b| b.split(':').next() == Some(name))
            || host
                .mounts
                .iter()
                .flatten()
                .any(|m| m.source.as_deref() == Some(name))
    };
    let anonymous = |name: &str| {
        name.len() == 64 && name.bytes().all(|b| b.is_ascii_hexdigit()) && !given_by_user(name)
    };
    container
        .mounts
        .iter()
        .flatten()
        .filter_map(|m| {
            let source = match m.typ.as_deref() {
                Some("bind") => m.source.clone()?,
                Some("volume") => {
                    let name = m.name.clone()?;
                    if anonymous(&name) {
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
mod tests {
    use serde_json::json;

    use super::*;
    use crate::containers::run::{create_body, Base};

    const ID: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const ANONYMOUS: &str = "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210";

    fn image() -> ImageInspect {
        serde_json::from_value(json!({
            "Config": {
                "Env": ["PATH=/usr/local/bin:/usr/bin", "NGINX_VERSION=1.27"],
                "Cmd": ["nginx", "-g", "daemon off;"],
                "Entrypoint": ["/docker-entrypoint.sh"],
                "Labels": { "maintainer": "NGINX" },
                "Volumes": { "/var/cache/nginx": {} },
                "StopSignal": "SIGQUIT"
            }
        }))
        .unwrap()
    }

    /// A container as `docker run -d --name web -p 8080:80 -v site:/srv nginx:1.27` leaves it.
    fn plain() -> serde_json::Value {
        json!({
            "Id": ID,
            "Name": "/web",
            "Image": "sha256:img",
            "Config": {
                "Hostname": "0123456789ab",
                "Env": ["PATH=/usr/local/bin:/usr/bin", "NGINX_VERSION=1.27", "MODE=prod"],
                "Cmd": ["nginx", "-g", "daemon off;"],
                "Entrypoint": ["/docker-entrypoint.sh"],
                "Image": "nginx:1.27",
                "Labels": { "maintainer": "NGINX" },
                "Volumes": { "/var/cache/nginx": {} },
                "StopSignal": "SIGQUIT"
            },
            "HostConfig": {
                "Binds": ["site:/srv"],
                "NetworkMode": "bridge",
                "PortBindings": { "80/tcp": [{ "HostIp": "", "HostPort": "8080" }] },
                "RestartPolicy": { "Name": "no", "MaximumRetryCount": 0 },
                "AutoRemove": false,
                "LogConfig": { "Type": "json-file", "Config": {} },
                "NanoCpus": 0,
                "Memory": 0,
                "CpuShares": 0,
                "MemorySwap": 0,
                "Ulimits": [{ "Name": "nofile", "Soft": 1024, "Hard": 2048 }]
            },
            "Mounts": [
                { "Type": "volume", "Name": "site", "Source": "/var/lib/docker/volumes/site/_data", "Destination": "/srv", "RW": true },
                { "Type": "volume", "Name": ANONYMOUS, "Source": "/x", "Destination": "/var/cache/nginx", "RW": true }
            ],
            "NetworkSettings": { "Networks": { "bridge": { "Aliases": null } } }
        })
    }

    /// The `web` service of the `shop` Stack, on its network and another.
    fn stack() -> serde_json::Value {
        let mut c = plain();
        c["Name"] = json!("/shop-web-1");
        c["Config"]["Labels"] = json!({
            "maintainer": "NGINX",
            "com.docker.compose.project": "shop",
            "com.docker.compose.service": "web",
            "traefik.enable": "true"
        });
        c["HostConfig"]["NetworkMode"] = json!("shop_default");
        c["NetworkSettings"]["Networks"] = json!({
            "shop_default": { "Aliases": ["shop-web-1", "web", "0123456789ab"] },
            "proxy": { "Aliases": ["shop-web-1", "web", "www"] }
        });
        c
    }

    /// Every option the Run dialog has, set.
    fn everything() -> serde_json::Value {
        let mut c = plain();
        let host = c["HostConfig"].as_object_mut().unwrap();
        host.insert("NanoCpus".into(), json!(1_500_000_000i64));
        host.insert("CpuShares".into(), json!(512));
        host.insert("Memory".into(), json!(536_870_912i64));
        host.insert("MemoryReservation".into(), json!(268_435_456i64));
        host.insert("MemorySwap".into(), json!(-1));
        host.insert("CapAdd".into(), json!(["CAP_NET_ADMIN"]));
        host.insert("CapDrop".into(), json!(["ALL"]));
        host.insert("Privileged".into(), json!(true));
        host.insert("Devices".into(), json!([
            { "PathOnHost": "/dev/dri", "PathInContainer": "/dev/dri", "CgroupPermissions": "rwm" },
            { "PathOnHost": "/dev/ttyUSB0", "PathInContainer": "/dev/zigbee", "CgroupPermissions": "rw" }
        ]));
        host.insert(
            "DeviceRequests".into(),
            json!([
                { "Driver": "", "Count": -1, "DeviceIDs": null, "Capabilities": [["gpu"]] }
            ]),
        );
        host.insert("Dns".into(), json!(["1.1.1.1"]));
        host.insert("DnsSearch".into(), json!(["home.arpa"]));
        host.insert("ExtraHosts".into(), json!(["nas:192.168.1.10"]));
        host.insert(
            "LogConfig".into(),
            json!({ "Type": "local", "Config": { "max-size": "10m" } }),
        );
        host.insert(
            "RestartPolicy".into(),
            json!({ "Name": "unless-stopped", "MaximumRetryCount": 0 }),
        );
        c["Config"]["Hostname"] = json!("web");
        c["Config"]["Domainname"] = json!("home.arpa");
        c["Config"]["Cmd"] = json!(["nginx", "-c", "/etc/nginx/alt conf"]);
        c["Config"]["Healthcheck"] = json!({
            "Test": ["CMD-SHELL", "curl -f localhost || exit 1"],
            "Interval": 30_000_000_000i64,
            "Timeout": 5_000_000_000i64,
            "Retries": 3,
            "StartPeriod": 0
        });
        c["Config"]["Labels"]["team"] = json!("web");
        c
    }

    fn inspect(value: serde_json::Value) -> ContainerInspectResponse {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn a_plain_container_reads_back_its_own_settings_only() {
        let spec = spec_from_inspect(&inspect(plain()), Some(&image()));
        assert_eq!(spec.image, "nginx:1.27");
        assert_eq!(spec.name, None);
        assert_eq!(spec.command, None, "the image's command");
        assert_eq!(spec.env, ["MODE=prod"]);
        assert!(spec.labels.is_empty(), "{:?}", spec.labels);
        assert_eq!(spec.ports.len(), 1);
        assert_eq!(
            (spec.ports[0].host_port, spec.ports[0].container_port),
            (Some(8080), 80)
        );
        assert_eq!(spec.mounts.len(), 1, "the anonymous volume is left out");
        assert_eq!(spec.mounts[0].source, "site");
        assert_eq!(spec.network, None);
        assert_eq!(spec.hostname, None, "the generated hostname is left out");
        assert_eq!(spec.based_on.as_deref(), Some(ID));
        assert_eq!(spec.log.unwrap().driver, "json-file");
        assert_eq!(
            (spec.cpus, spec.memory, spec.memory_swap),
            (None, None, None)
        );
    }

    #[test]
    fn a_stack_container_reads_back_without_its_stack_labels() {
        let spec = spec_from_inspect(&inspect(stack()), Some(&image()));
        assert_eq!(spec.labels, ["traefik.enable=true"]);
        assert_eq!(spec.network.as_deref(), Some("shop_default"));
    }

    #[test]
    fn every_option_reads_back() {
        let spec = spec_from_inspect(&inspect(everything()), Some(&image()));
        assert_eq!(
            spec.command.as_deref(),
            Some("nginx -c '/etc/nginx/alt conf'")
        );
        assert_eq!(spec.cpus, Some(1.5));
        assert_eq!(spec.cpu_shares, Some(512));
        assert_eq!(spec.memory, Some(536_870_912));
        assert_eq!(spec.memory_reservation, Some(268_435_456));
        assert_eq!(spec.memory_swap, Some(-1));
        assert_eq!(spec.labels, ["team=web"]);
        assert_eq!(spec.cap_add, ["NET_ADMIN"]);
        assert_eq!(spec.cap_drop, ["ALL"]);
        assert!(spec.privileged);
        assert_eq!(spec.devices[0].container_path, None, "same path");
        assert_eq!(
            spec.devices[1].container_path.as_deref(),
            Some("/dev/zigbee")
        );
        assert_eq!(spec.gpus, Some(GpuSpec::All));
        let health = spec.healthcheck.unwrap();
        assert_eq!(health.command, "curl -f localhost || exit 1");
        assert_eq!(
            (health.interval_s, health.timeout_s, health.retries),
            (Some(30), Some(5), Some(3))
        );
        assert_eq!(health.start_period_s, None);
        assert_eq!(spec.hostname.as_deref(), Some("web"));
        assert_eq!(spec.domainname.as_deref(), Some("home.arpa"));
        assert_eq!(spec.dns, ["1.1.1.1"]);
        assert_eq!(spec.dns_search, ["home.arpa"]);
        assert_eq!(spec.extra_hosts, ["nas:192.168.1.10"]);
        assert_eq!(spec.log.unwrap().options, ["max-size=10m"]);
        assert_eq!(spec.restart, RestartPolicy::UnlessStopped);
    }

    #[test]
    fn a_disabled_healthcheck_reads_back_as_disabled() {
        let mut c = plain();
        c["Config"]["Healthcheck"] = json!({ "Test": ["NONE"] });
        let spec = spec_from_inspect(&inspect(c), Some(&image()));
        assert!(spec.healthcheck.unwrap().disabled);
    }

    #[test]
    fn a_spec_read_back_creates_the_same_container() {
        let original = inspect(everything());
        let image = image();
        let spec = spec_from_inspect(&original, Some(&image));
        let base = Base {
            inspect: &original,
            image: Some(&image),
            same_identity: false,
        };
        let body = create_body(&spec, Some(&base)).unwrap();
        let host = body.host_config.clone().unwrap();
        assert_eq!(host.nano_cpus, Some(1_500_000_000));
        assert_eq!(host.cap_add.unwrap(), ["CAP_NET_ADMIN"]);
        assert_eq!(
            host.ulimits.unwrap()[0].name.as_deref(),
            Some("nofile"),
            "kept from the base"
        );
        assert_eq!(host.binds.unwrap(), ["site:/srv"]);
        assert_eq!(body.entrypoint, None, "the image's entrypoint isn't pinned");
        assert_eq!(body.stop_signal, None, "nor its stop signal");
        assert_eq!(body.volumes, None, "the image's volumes come with it");
        assert_eq!(body.hostname.as_deref(), Some("web"));
    }

    #[test]
    fn a_copy_leaves_the_stack_and_an_edit_stays_in_it() {
        let original = inspect(stack());
        let image = image();
        let spec = spec_from_inspect(&original, Some(&image));
        let body = |same_identity| {
            let base = Base {
                inspect: &original,
                image: Some(&image),
                same_identity,
            };
            (
                create_body(&spec, Some(&base)).unwrap(),
                base.other_networks(&spec),
            )
        };

        let (copy, copy_networks) = body(false);
        let labels = copy.labels.unwrap();
        assert!(
            !labels.keys().any(|k| k.starts_with("com.docker.compose.")),
            "{labels:?}"
        );
        assert_eq!(labels["traefik.enable"], "true");
        assert_eq!(
            copy.networking_config, None,
            "no alias of the original's on its network"
        );
        assert_eq!(
            copy_networks,
            [("proxy".to_owned(), vec!["www".to_owned()])]
        );

        let (edit, edit_networks) = body(true);
        let labels = edit.labels.unwrap();
        assert_eq!(labels["com.docker.compose.project"], "shop");
        let endpoints = edit.networking_config.unwrap().endpoints_config.unwrap();
        assert_eq!(
            endpoints["shop_default"].aliases.as_deref(),
            Some(&["shop-web-1".to_owned(), "web".to_owned()][..])
        );
        assert_eq!(
            edit_networks,
            [(
                "proxy".to_owned(),
                vec!["shop-web-1".to_owned(), "web".to_owned(), "www".to_owned()]
            )]
        );
    }
}

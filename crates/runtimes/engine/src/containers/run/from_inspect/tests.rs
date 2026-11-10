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

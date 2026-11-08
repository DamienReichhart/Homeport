//! Running a container into a pod. The Docker-compatible create request has
//! no pod, so the Run dialog's spec goes to `POST /libpod/containers/create`
//! as Podman's `SpecGenerator`. It is first built and checked as the Docker
//! request, so both dialogs refuse the same mistakes the same way.

use std::collections::BTreeMap;

use bollard::{
    models::{ContainerCreateBody, RestartPolicyNameEnum},
    Docker,
};
use serde::Deserialize;
use serde_json::{json, Map, Value};

use homeport_core::error::{AppError, AppResult};
use homeport_transport::http::Request;

use super::Podman;
use crate::{
    auth::RegistryAuth,
    containers::{
        self,
        run::{self, RunSpec},
    },
    messages as msg,
};

/// Docker's CPU period, in microseconds, which `--cpus` is a share of.
const CPU_PERIOD: i64 = 100_000;

/// What a container in a pod can't set, because the pod owns it.
fn check_for_pod(spec: &RunSpec) -> AppResult<()> {
    let set = |v: &Option<String>| v.as_deref().is_some_and(|v| !v.trim().is_empty());
    if spec.based_on.is_some() {
        return Err(AppError::validation(msg::POD_CONTAINER_BASED_ON));
    }
    if !spec.ports.is_empty() {
        return Err(
            AppError::validation(msg::POD_CONTAINER_PORTS).with_hint(msg::POD_CONTAINER_PORTS_HINT)
        );
    }
    if set(&spec.network)
        || set(&spec.hostname)
        || set(&spec.domainname)
        || !spec.dns.is_empty()
        || !spec.dns_search.is_empty()
        || !spec.extra_hosts.is_empty()
    {
        return Err(AppError::validation(msg::POD_CONTAINER_SHARED));
    }
    if spec.gpus.is_some() {
        return Err(AppError::validation(msg::POD_CONTAINER_GPUS));
    }
    Ok(())
}

/// The body of `POST /libpod/containers/create` for `spec`, in `pod`.
pub fn create_body(spec: &RunSpec, pod: &str) -> AppResult<Value> {
    check_for_pod(spec)?;
    let docker = run::create_body(spec, None)?;
    let mut body = translate(&docker);
    body.insert("pod".into(), pod.into());
    if let Some(name) = spec
        .name
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty())
    {
        body.insert("name".into(), name.into());
    }
    Ok(Value::Object(body))
}

/// The Docker create request as Podman's `SpecGenerator`, field by field.
fn translate(docker: &ContainerCreateBody) -> Map<String, Value> {
    let mut body = Map::new();
    let mut put = |key: &str, value: Value| {
        body.insert(key.into(), value);
    };
    put("image", docker.image.clone().unwrap_or_default().into());
    if let Some(cmd) = &docker.cmd {
        put("command", json!(cmd));
    }
    if let Some(env) = &docker.env {
        let env: BTreeMap<&str, &str> = env.iter().filter_map(|e| e.split_once('=')).collect();
        put("env", json!(env));
    }
    if let Some(labels) = &docker.labels {
        let labels: BTreeMap<_, _> = labels.iter().collect();
        put("labels", json!(labels));
    }
    if let Some(health) = &docker.healthcheck {
        let mut config = Map::new();
        if let Some(test) = &health.test {
            config.insert("Test".into(), json!(test));
        }
        for (key, value) in [
            ("Interval", health.interval),
            ("Timeout", health.timeout),
            ("Retries", health.retries),
            ("StartPeriod", health.start_period),
        ] {
            if let Some(v) = value {
                config.insert(key.into(), v.into());
            }
        }
        put("healthconfig", Value::Object(config));
    }
    let Some(host) = &docker.host_config else {
        return body;
    };

    let (mut mounts, mut volumes) = (Vec::new(), Vec::new());
    for bind in host.binds.iter().flatten() {
        let (rest, read_only) = match bind.strip_suffix(":ro") {
            Some(rest) => (rest, true),
            None => (bind.as_str(), false),
        };
        let Some((source, target)) = rest.split_once(':') else {
            continue;
        };
        if source.starts_with('/') {
            let mut options = vec!["rbind"];
            if read_only {
                options.push("ro");
            }
            mounts.push(json!({
                "destination": target,
                "type": "bind",
                "source": source,
                "options": options,
            }));
        } else {
            let options: Vec<&str> = if read_only { vec!["ro"] } else { vec![] };
            volumes.push(json!({ "Name": source, "Dest": target, "Options": options }));
        }
    }
    if !mounts.is_empty() {
        put("mounts", json!(mounts));
    }
    if !volumes.is_empty() {
        put("volumes", json!(volumes));
    }

    let restart = host.restart_policy.as_ref().and_then(|p| p.name);
    match restart {
        Some(RestartPolicyNameEnum::ALWAYS) => put("restart_policy", "always".into()),
        Some(RestartPolicyNameEnum::UNLESS_STOPPED) => {
            put("restart_policy", "unless-stopped".into())
        }
        Some(RestartPolicyNameEnum::ON_FAILURE) => put("restart_policy", "on-failure".into()),
        _ => {}
    }
    if host.auto_remove == Some(true) {
        put("remove", true.into());
    }

    let mut cpu = Map::new();
    if let Some(nano) = host.nano_cpus {
        cpu.insert("quota".into(), (nano * CPU_PERIOD / 1_000_000_000).into());
        cpu.insert("period".into(), CPU_PERIOD.into());
    }
    if let Some(shares) = host.cpu_shares {
        cpu.insert("shares".into(), shares.into());
    }
    let mut memory = Map::new();
    for (key, value) in [
        ("limit", host.memory),
        ("reservation", host.memory_reservation),
        ("swap", host.memory_swap),
    ] {
        if let Some(v) = value {
            memory.insert(key.into(), v.into());
        }
    }
    let mut resources = Map::new();
    if !cpu.is_empty() {
        resources.insert("cpu".into(), Value::Object(cpu));
    }
    if !memory.is_empty() {
        resources.insert("memory".into(), Value::Object(memory));
    }
    if !resources.is_empty() {
        put("resource_limits", Value::Object(resources));
    }

    if let Some(caps) = host.cap_add.as_ref().filter(|c| !c.is_empty()) {
        put("cap_add", json!(caps));
    }
    if let Some(caps) = host.cap_drop.as_ref().filter(|c| !c.is_empty()) {
        put("cap_drop", json!(caps));
    }
    if host.privileged == Some(true) {
        put("privileged", true.into());
    }
    if let Some(devices) = host.devices.as_ref().filter(|d| !d.is_empty()) {
        // Podman reads each as `--device` does: host[:container[:permissions]].
        let devices: Vec<Value> = devices
            .iter()
            .map(|d| {
                let host_path = d.path_on_host.clone().unwrap_or_default();
                let container = d
                    .path_in_container
                    .clone()
                    .unwrap_or_else(|| host_path.clone());
                let permissions = d.cgroup_permissions.clone().unwrap_or_else(|| "rwm".into());
                json!({ "path": format!("{host_path}:{container}:{permissions}") })
            })
            .collect();
        put("devices", json!(devices));
    }
    if let Some(log) = &host.log_config {
        let mut config = Map::new();
        if let Some(driver) = &log.typ {
            config.insert("driver".into(), driver.clone().into());
        }
        if let Some(options) = log.config.as_ref().filter(|o| !o.is_empty()) {
            let options: BTreeMap<_, _> = options.iter().collect();
            config.insert("options".into(), json!(options));
        }
        put("log_configuration", Value::Object(config));
    }
    body
}

#[derive(Deserialize)]
struct Created {
    #[serde(rename = "Id")]
    id: String,
}

/// Creates and starts a container in `pod`, pulling its image first (with
/// `auth` when given) if the Host doesn't have it; returns its id. A
/// container that can't start is removed again, as the Run dialog does.
pub async fn run_in_pod(
    docker: &Docker,
    podman: &Podman,
    spec: RunSpec,
    pod: &str,
    auth: Option<RegistryAuth>,
) -> AppResult<String> {
    let body = create_body(&spec, pod)?;
    let image = spec.image.trim();
    if docker.inspect_image(image).await.is_err() {
        run::pull(docker, image, auth).await?;
    }
    let path = podman.path("/containers/create");
    let created: Created = podman
        .json(Request::post(path).json(&body)?)
        .await
        .map_err(|e| pod_missing(e, pod))?;
    if let Err(e) = containers::act(docker, &created.id, containers::ContainerAction::Start).await {
        let _ = containers::remove(docker, &created.id, true, true).await;
        return Err(e);
    }
    Ok(created.id)
}

/// Podman refuses a container for a pod that's gone with a 404 like a
/// missing image's; say which.
fn pod_missing(e: AppError, pod: &str) -> AppError {
    let detail = e.detail.clone().unwrap_or_default().to_lowercase();
    if detail.contains("no such pod") || detail.contains(&format!("pod {}", pod.to_lowercase())) {
        AppError::not_found(msg::POD_GONE)
            .with_hint(msg::POD_GONE_HINT)
            .with_detail(detail)
    } else {
        e
    }
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::containers::run::{
        DeviceSpec, GpuSpec, HealthcheckSpec, LogSpec, MountSpec, PortSpec, RestartPolicy,
    };

    fn spec() -> RunSpec {
        RunSpec {
            image: " docker.io/library/nginx:1.29 ".into(),
            name: Some(" web-app ".into()),
            ..Default::default()
        }
    }

    #[test]
    fn a_bare_spec_names_its_image_and_pod() {
        assert_eq!(
            create_body(&spec(), "web").unwrap(),
            json!({
                "image": "docker.io/library/nginx:1.29",
                "name": "web-app",
                "pod": "web",
            })
        );
    }

    #[test]
    fn the_dialogs_settings_become_podmans() {
        let full = RunSpec {
            command: Some("nginx -g 'daemon off;'".into()),
            env: vec!["A=1".into(), "B=x=y".into()],
            labels: vec!["app=web".into()],
            mounts: vec![
                MountSpec {
                    source: "/srv/www".into(),
                    target: "/usr/share/nginx/html".into(),
                    read_only: true,
                },
                MountSpec {
                    source: "cache".into(),
                    target: "/var/cache/nginx".into(),
                    read_only: false,
                },
            ],
            restart: RestartPolicy::UnlessStopped,
            cpus: Some(0.5),
            cpu_shares: Some(512),
            memory: Some(64 * 1024 * 1024),
            cap_add: vec!["NET_ADMIN".into()],
            cap_drop: vec!["MKNOD".into()],
            devices: vec![DeviceSpec {
                host_path: "/dev/dri".into(),
                container_path: None,
                permissions: "rw".into(),
            }],
            healthcheck: Some(HealthcheckSpec {
                command: "curl -f localhost".into(),
                interval_s: Some(30),
                retries: Some(3),
                ..Default::default()
            }),
            log: Some(LogSpec {
                driver: "journald".into(),
                options: vec!["tag=web".into()],
            }),
            ..spec()
        };
        let body = create_body(&full, "web").unwrap();
        assert_eq!(body["command"], json!(["nginx", "-g", "daemon off;"]));
        assert_eq!(body["env"], json!({ "A": "1", "B": "x=y" }));
        assert_eq!(body["labels"], json!({ "app": "web" }));
        assert_eq!(
            body["mounts"],
            json!([{
                "destination": "/usr/share/nginx/html",
                "type": "bind",
                "source": "/srv/www",
                "options": ["rbind", "ro"],
            }])
        );
        assert_eq!(
            body["volumes"],
            json!([{ "Name": "cache", "Dest": "/var/cache/nginx", "Options": [] }])
        );
        assert_eq!(body["restart_policy"], "unless-stopped");
        assert_eq!(
            body["resource_limits"],
            json!({
                "cpu": { "quota": 50_000, "period": 100_000, "shares": 512 },
                "memory": { "limit": 64 * 1024 * 1024 },
            })
        );
        assert_eq!(body["cap_add"], json!(["CAP_NET_ADMIN"]));
        assert_eq!(body["cap_drop"], json!(["CAP_MKNOD"]));
        assert_eq!(body["devices"], json!([{ "path": "/dev/dri:/dev/dri:rw" }]));
        assert_eq!(
            body["healthconfig"]["Test"],
            json!(["CMD-SHELL", "curl -f localhost"])
        );
        assert_eq!(body["healthconfig"]["Interval"], 30_000_000_000_i64);
        assert_eq!(body["healthconfig"]["Retries"], 3);
        assert_eq!(
            body["log_configuration"],
            json!({ "driver": "journald", "options": { "tag": "web" } })
        );
        assert!(body.get("remove").is_none() && body.get("privileged").is_none());
    }

    #[test]
    fn removal_on_exit_and_privileges_carry_over() {
        let body = create_body(
            &RunSpec {
                auto_remove: true,
                privileged: true,
                ..spec()
            },
            "web",
        )
        .unwrap();
        assert_eq!(body["remove"], true);
        assert_eq!(body["privileged"], true);
        assert!(body.get("restart_policy").is_none());
    }

    #[test]
    fn what_the_pod_owns_is_refused() {
        let port = PortSpec {
            host_ip: None,
            host_port: Some(8080),
            container_port: 80,
            protocol: "tcp".into(),
        };
        let cases = [
            RunSpec {
                ports: vec![port],
                ..spec()
            },
            RunSpec {
                network: Some("podman".into()),
                ..spec()
            },
            RunSpec {
                hostname: Some("web".into()),
                ..spec()
            },
            RunSpec {
                dns: vec!["1.1.1.1".into()],
                ..spec()
            },
            RunSpec {
                extra_hosts: vec!["nas:10.0.0.2".into()],
                ..spec()
            },
            RunSpec {
                gpus: Some(GpuSpec::All),
                ..spec()
            },
            RunSpec {
                based_on: Some("abc".into()),
                ..spec()
            },
        ];
        for case in cases {
            let err = create_body(&case, "web").unwrap_err();
            assert!(err.is(ErrorCode::Validation), "{case:?}");
        }
        let ports = create_body(
            &RunSpec {
                ports: vec![PortSpec {
                    host_ip: None,
                    host_port: None,
                    container_port: 80,
                    protocol: "tcp".into(),
                }],
                ..spec()
            },
            "web",
        )
        .unwrap_err();
        assert_eq!(ports.key, "errors.pods.containerPorts");
        assert!(ports.hint.is_some());
    }

    #[test]
    fn the_docker_checks_still_apply() {
        let bad = RunSpec {
            env: vec!["NOEQUALS".into()],
            ..spec()
        };
        assert_eq!(
            create_body(&bad, "web").unwrap_err().key,
            "errors.run.envNotKeyValue"
        );
        let none = RunSpec {
            image: " ".into(),
            ..spec()
        };
        assert!(create_body(&none, "web").is_err());
    }
}

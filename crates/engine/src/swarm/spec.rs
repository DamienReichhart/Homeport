//! Creating a service from a form, and updating an existing service's
//! image, environment and replicas, through a manager. The Engine rolls the
//! tasks over to the new spec itself (its rolling update).

use std::collections::HashMap;

use bollard::{
    models::{
        EndpointPortConfig, EndpointPortConfigProtocolEnum, EndpointPortConfigPublishModeEnum,
        EndpointSpec, Mount, MountType, NetworkAttachmentConfig, ServiceSpec, ServiceSpecMode,
        ServiceSpecModeReplicated, TaskSpec, TaskSpecContainerSpec, TaskSpecContainerSpecConfigs,
        TaskSpecContainerSpecFile, TaskSpecContainerSpecFile1, TaskSpecContainerSpecSecrets,
        TaskSpecRestartPolicy, TaskSpecRestartPolicyConditionEnum,
    },
    query_parameters::{ListConfigsOptions, ListSecretsOptions},
    Docker,
};
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::services::{at_version, check_replicas, mode_of, update_error, ServiceMode};
use crate::messages as msg;

/// Replicated with a number of replicas, or one task per node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ModeInput {
    Replicated { replicas: u64 },
    Global,
}

/// A published port. `host_mode` publishes on the node running the task
/// only, instead of on every node through the routing mesh.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PortInput {
    pub target: u16,
    /// `None` lets the Swarm pick one (ingress only).
    pub published: Option<u16>,
    /// `tcp`, `udp` or `sctp`.
    pub protocol: String,
    pub host_mode: bool,
}

/// A volume (by name) or a bind mount (an absolute path on each node).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MountInput {
    pub source: String,
    pub target: String,
    pub read_only: bool,
}

/// A secret or config given to the service, by name, as a file at `target`
/// (`/run/secrets/<name>` or `/<name>` when empty).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ObjectRef {
    pub name: String,
    pub target: Option<String>,
}

/// When a task's container is restarted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "kebab-case")]
pub enum RestartCondition {
    None,
    OnFailure,
    Any,
}

/// What the Create service dialog asks for.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSpecInput {
    pub name: String,
    pub image: String,
    pub mode: ModeInput,
    /// `KEY=value` entries.
    pub env: Vec<String>,
    /// Replaces the image's command, split like a shell would.
    pub command: Option<String>,
    pub ports: Vec<PortInput>,
    /// Overlay networks, by name.
    pub networks: Vec<String>,
    pub mounts: Vec<MountInput>,
    pub secrets: Vec<ObjectRef>,
    pub configs: Vec<ObjectRef>,
    pub restart: Option<RestartCondition>,
}

/// The IDs of the Swarm's secrets and configs by name, which a spec refers to.
#[derive(Debug, Clone, Default)]
pub struct ObjectIds {
    pub secrets: HashMap<String, String>,
    pub configs: HashMap<String, String>,
}

fn invalid(text: impl Into<homeport_core::error::Text>) -> AppError {
    AppError::validation(text)
}

/// Whether `name` is a service name the Swarm takes (a DNS label).
pub fn valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    !name.is_empty()
        && name.len() <= 63
        && bytes[0].is_ascii_alphanumeric()
        && bytes[bytes.len() - 1].is_ascii_alphanumeric()
        && bytes
            .iter()
            .all(|b| b.is_ascii_alphanumeric() || *b == b'-' || *b == b'_')
}

fn check_env(env: &[String]) -> AppResult<()> {
    for entry in env {
        if !entry.contains('=') || entry.starts_with('=') {
            return Err(invalid(msg::RUN_ENV.with("entry", entry)));
        }
    }
    Ok(())
}

fn protocol(name: &str) -> AppResult<EndpointPortConfigProtocolEnum> {
    match name {
        "" | "tcp" => Ok(EndpointPortConfigProtocolEnum::TCP),
        "udp" => Ok(EndpointPortConfigProtocolEnum::UDP),
        "sctp" => Ok(EndpointPortConfigProtocolEnum::SCTP),
        _ => Err(invalid(msg::RUN_PORT_PROTOCOL)),
    }
}

/// The bollard `ServiceSpec` for `input`, checked first; secrets and configs
/// are resolved to their IDs through `ids`.
pub fn spec_body(input: &ServiceSpecInput, ids: &ObjectIds) -> AppResult<ServiceSpec> {
    let name = input.name.trim();
    if !valid_name(name) {
        return Err(invalid(msg::SERVICE_NAME));
    }
    let image = input.image.trim();
    if image.is_empty() {
        return Err(invalid(msg::RUN_IMAGE));
    }
    check_env(&input.env)?;
    let command = match input.command.as_deref().map(str::trim) {
        Some(c) if !c.is_empty() => {
            Some(shell_words::split(c).map_err(|_| invalid(msg::COMMAND_UNCLOSED_QUOTE))?)
        }
        _ => None,
    };
    let mode = match input.mode {
        ModeInput::Replicated { replicas } => {
            check_replicas(replicas)?;
            ServiceSpecMode {
                replicated: Some(ServiceSpecModeReplicated {
                    replicas: Some(i64::try_from(replicas).unwrap_or(i64::MAX)),
                }),
                ..Default::default()
            }
        }
        ModeInput::Global => ServiceSpecMode {
            global: Some(HashMap::new()),
            ..Default::default()
        },
    };

    let mut ports = Vec::new();
    let mut published = Vec::new();
    for p in &input.ports {
        if p.target == 0 {
            return Err(invalid(msg::SERVICE_PORT_TARGET));
        }
        let protocol = protocol(p.protocol.trim())?;
        if let Some(port) = p.published.filter(|p| *p > 0) {
            if published.contains(&(port, protocol)) {
                return Err(invalid(msg::SERVICE_PORT_TWICE.with("port", port)));
            }
            published.push((port, protocol));
        } else if p.host_mode {
            return Err(invalid(msg::SERVICE_HOST_PORT_NEEDED));
        }
        ports.push(EndpointPortConfig {
            protocol: Some(protocol),
            target_port: Some(p.target.into()),
            published_port: p.published.filter(|p| *p > 0).map(i64::from),
            publish_mode: Some(if p.host_mode {
                EndpointPortConfigPublishModeEnum::HOST
            } else {
                EndpointPortConfigPublishModeEnum::INGRESS
            }),
            ..Default::default()
        });
    }

    let mut mounts = Vec::new();
    for m in &input.mounts {
        let (source, target) = (m.source.trim(), m.target.trim());
        if source.is_empty() || !target.starts_with('/') {
            return Err(invalid(msg::RUN_MOUNT));
        }
        let bind = source.starts_with('/') || source.contains(':') && source.contains('\\');
        mounts.push(Mount {
            source: Some(source.to_owned()),
            target: Some(target.to_owned()),
            typ: Some(if bind {
                MountType::BIND
            } else {
                MountType::VOLUME
            }),
            read_only: Some(m.read_only),
            ..Default::default()
        });
    }

    let id_of = |map: &HashMap<String, String>, name: &str| {
        map.get(name)
            .cloned()
            .ok_or_else(|| AppError::not_found(msg::OBJECT_MISSING.with("name", name)))
    };
    let target = |r: &ObjectRef| {
        r.target
            .as_deref()
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .unwrap_or(&r.name)
            .to_owned()
    };
    let secrets = input
        .secrets
        .iter()
        .map(|r| {
            Ok(TaskSpecContainerSpecSecrets {
                secret_id: Some(id_of(&ids.secrets, &r.name)?),
                secret_name: Some(r.name.clone()),
                file: Some(TaskSpecContainerSpecFile {
                    name: Some(target(r)),
                    uid: Some("0".into()),
                    gid: Some("0".into()),
                    mode: Some(0o444),
                }),
            })
        })
        .collect::<AppResult<Vec<_>>>()?;
    let configs = input
        .configs
        .iter()
        .map(|r| {
            Ok(TaskSpecContainerSpecConfigs {
                config_id: Some(id_of(&ids.configs, &r.name)?),
                config_name: Some(r.name.clone()),
                file: Some(TaskSpecContainerSpecFile1 {
                    name: Some(target(r)),
                    uid: Some("0".into()),
                    gid: Some("0".into()),
                    mode: Some(0o444),
                }),
                ..Default::default()
            })
        })
        .collect::<AppResult<Vec<_>>>()?;

    let networks: Vec<NetworkAttachmentConfig> = input
        .networks
        .iter()
        .map(|n| n.trim())
        .filter(|n| !n.is_empty())
        .map(|n| NetworkAttachmentConfig {
            target: Some(n.to_owned()),
            ..Default::default()
        })
        .collect();

    Ok(ServiceSpec {
        name: Some(name.to_owned()),
        mode: Some(mode),
        task_template: Some(TaskSpec {
            container_spec: Some(TaskSpecContainerSpec {
                image: Some(image.to_owned()),
                env: (!input.env.is_empty()).then(|| input.env.clone()),
                args: command,
                mounts: (!mounts.is_empty()).then_some(mounts),
                secrets: (!secrets.is_empty()).then_some(secrets),
                configs: (!configs.is_empty()).then_some(configs),
                ..Default::default()
            }),
            restart_policy: input.restart.map(|r| TaskSpecRestartPolicy {
                condition: Some(match r {
                    RestartCondition::None => TaskSpecRestartPolicyConditionEnum::NONE,
                    RestartCondition::OnFailure => TaskSpecRestartPolicyConditionEnum::ON_FAILURE,
                    RestartCondition::Any => TaskSpecRestartPolicyConditionEnum::ANY,
                }),
                ..Default::default()
            }),
            networks: (!networks.is_empty()).then_some(networks),
            ..Default::default()
        }),
        endpoint_spec: (!ports.is_empty()).then(|| EndpointSpec {
            ports: Some(ports),
            ..Default::default()
        }),
        ..Default::default()
    })
}

/// The Swarm's secret and config IDs by name.
pub async fn object_ids(docker: &Docker) -> AppResult<ObjectIds> {
    Ok(ObjectIds {
        secrets: docker
            .list_secrets(None::<ListSecretsOptions>)
            .await?
            .into_iter()
            .filter_map(|s| Some((s.spec?.name?, s.id?)))
            .collect(),
        configs: docker
            .list_configs(None::<ListConfigsOptions>)
            .await?
            .into_iter()
            .filter_map(|c| Some((c.spec?.name?, c.id?)))
            .collect(),
    })
}

/// Creates a service from `input`, returning its ID.
pub async fn create(docker: &Docker, input: &ServiceSpecInput) -> AppResult<String> {
    let needs_ids = !input.secrets.is_empty() || !input.configs.is_empty();
    let ids = if needs_ids {
        object_ids(docker).await?
    } else {
        ObjectIds::default()
    };
    let spec = spec_body(input, &ids)?;
    let created = docker
        .create_service(spec, None)
        .await
        .map_err(|e| create_error(&e))?;
    Ok(created.id.unwrap_or_default())
}

fn create_error(e: &bollard::errors::Error) -> AppError {
    let text = e.to_string().to_lowercase();
    if text.contains("name conflicts with an existing object") || text.contains("already exists") {
        return AppError::new(ErrorCode::NameInUse, msg::SERVICE_NAME_IN_USE)
            .with_hint(msg::SERVICE_NAME_IN_USE_HINT)
            .with_cause(e);
    }
    update_error(e)
}

/// What the Edit dialog changes on a service; `None` keeps that part.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ServiceChange {
    pub image: Option<String>,
    /// Replaces every `KEY=value` entry.
    pub env: Option<Vec<String>>,
    pub replicas: Option<u64>,
    /// Restart every task even when nothing changed, as `--force` does:
    /// pulls the tag again when the image is unchanged.
    pub redeploy: bool,
}

/// `spec` with only `change`'s fields changed; everything else is kept.
pub fn merge(mut spec: ServiceSpec, change: &ServiceChange) -> AppResult<ServiceSpec> {
    let mut task = spec.task_template.take().unwrap_or_default();
    let mut container = task.container_spec.take().unwrap_or_default();
    let old_image = container.image.clone().unwrap_or_default();
    if let Some(image) = change.image.as_deref().map(str::trim) {
        if image.is_empty() {
            return Err(invalid(msg::RUN_IMAGE));
        }
        // An unchanged reference keeps the digest the Swarm pinned.
        if image != super::services::split_digest(&old_image).0 && image != old_image {
            container.image = Some(image.to_owned());
        }
    }
    if let Some(env) = &change.env {
        check_env(env)?;
        container.env = Some(env.clone());
    }
    if let Some(replicas) = change.replicas {
        check_replicas(replicas)?;
        if mode_of(spec.mode.as_ref()).0 != ServiceMode::Replicated {
            return Err(invalid(msg::SERVICE_NOT_REPLICATED));
        }
        spec.mode = Some(ServiceSpecMode {
            replicated: Some(ServiceSpecModeReplicated {
                replicas: Some(i64::try_from(replicas).unwrap_or(i64::MAX)),
            }),
            ..Default::default()
        });
    }
    if change.redeploy {
        task.force_update = Some(task.force_update.unwrap_or(0) + 1);
        // Redeploying the same tag pulls it again rather than reusing the pinned digest.
        container.image = Some(super::services::split_digest(&old_image).0);
        if let Some(image) = change.image.as_deref().map(str::trim) {
            container.image = Some(image.to_owned());
        }
    }
    task.container_spec = Some(container);
    spec.task_template = Some(task);
    Ok(spec)
}

/// Applies `change` to the service `id` as read at `version`: a service
/// updated since fails with a conflict rather than being overwritten.
pub async fn update(
    docker: &Docker,
    id: &str,
    version: u64,
    change: &ServiceChange,
) -> AppResult<()> {
    let current = docker.inspect_service(id, None).await?;
    let spec = merge(current.spec.unwrap_or_default(), change)?;
    docker
        .update_service(id, spec, at_version(Some(version)).build(), None)
        .await
        .map(|_| ())
        .map_err(|e| update_error(&e))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ServiceSpecInput {
        ServiceSpecInput {
            name: "web".into(),
            image: "nginx:1.27".into(),
            mode: ModeInput::Replicated { replicas: 2 },
            env: vec!["A=1".into()],
            command: Some("nginx -g 'daemon off;'".into()),
            ports: vec![PortInput {
                target: 80,
                published: Some(8080),
                protocol: "tcp".into(),
                host_mode: false,
            }],
            networks: vec!["backend".into()],
            mounts: vec![
                MountInput {
                    source: "web-data".into(),
                    target: "/data".into(),
                    read_only: false,
                },
                MountInput {
                    source: "/etc/ssl".into(),
                    target: "/ssl".into(),
                    read_only: true,
                },
            ],
            secrets: vec![ObjectRef {
                name: "db_password".into(),
                target: None,
            }],
            configs: vec![ObjectRef {
                name: "nginx_conf".into(),
                target: Some("/etc/nginx/nginx.conf".into()),
            }],
            restart: Some(RestartCondition::OnFailure),
        }
    }

    fn ids() -> ObjectIds {
        ObjectIds {
            secrets: [("db_password".to_string(), "s1".to_string())].into(),
            configs: [("nginx_conf".to_string(), "c1".to_string())].into(),
        }
    }

    #[test]
    fn a_form_becomes_a_service_spec() {
        let spec = spec_body(&input(), &ids()).unwrap();
        assert_eq!(spec.name.as_deref(), Some("web"));
        assert_eq!(
            mode_of(spec.mode.as_ref()),
            (ServiceMode::Replicated, Some(2))
        );
        let task = spec.task_template.unwrap();
        let c = task.container_spec.unwrap();
        assert_eq!(c.image.as_deref(), Some("nginx:1.27"));
        assert_eq!(c.args.unwrap(), ["nginx", "-g", "daemon off;"]);
        let mounts = c.mounts.unwrap();
        assert_eq!(mounts[0].typ, Some(MountType::VOLUME));
        assert_eq!(mounts[1].typ, Some(MountType::BIND));
        assert_eq!(mounts[1].read_only, Some(true));
        let secret = &c.secrets.unwrap()[0];
        assert_eq!(secret.secret_id.as_deref(), Some("s1"));
        assert_eq!(
            secret.file.as_ref().unwrap().name.as_deref(),
            Some("db_password")
        );
        let config = &c.configs.unwrap()[0];
        assert_eq!(
            config.file.as_ref().unwrap().name.as_deref(),
            Some("/etc/nginx/nginx.conf")
        );
        assert_eq!(task.networks.unwrap()[0].target.as_deref(), Some("backend"));
        assert_eq!(
            task.restart_policy.unwrap().condition,
            Some(TaskSpecRestartPolicyConditionEnum::ON_FAILURE)
        );
        let port = &spec.endpoint_spec.unwrap().ports.unwrap()[0];
        assert_eq!(
            (port.target_port, port.published_port),
            (Some(80), Some(8080))
        );
        assert_eq!(
            port.publish_mode,
            Some(EndpointPortConfigPublishModeEnum::INGRESS)
        );
    }

    #[test]
    fn bad_forms_are_refused_before_sending() {
        let refused = |change: fn(&mut ServiceSpecInput)| {
            let mut i = input();
            change(&mut i);
            spec_body(&i, &ids()).unwrap_err()
        };
        assert!(refused(|i| i.name = "-web".into()).is(ErrorCode::Validation));
        assert!(refused(|i| i.name = "has space".into()).is(ErrorCode::Validation));
        assert!(refused(|i| i.image = " ".into()).is(ErrorCode::Validation));
        assert!(refused(|i| i.env = vec!["NOVALUE".into()]).is(ErrorCode::Validation));
        assert!(refused(|i| i.ports.push(i.ports[0].clone())).is(ErrorCode::Validation));
        assert!(refused(|i| i.ports[0].protocol = "http".into()).is(ErrorCode::Validation));
        assert!(refused(|i| {
            i.ports[0].published = None;
            i.ports[0].host_mode = true;
        })
        .is(ErrorCode::Validation));
        assert!(refused(|i| i.mounts[0].target = "data".into()).is(ErrorCode::Validation));
        assert!(refused(|i| i.secrets[0].name = "nope".into()).is(ErrorCode::NotFound));
        // The same port on another protocol is fine.
        let mut both = input();
        both.ports.push(PortInput {
            protocol: "udp".into(),
            ..both.ports[0].clone()
        });
        assert!(spec_body(&both, &ids()).is_ok());
        let mut global = input();
        global.mode = ModeInput::Global;
        assert_eq!(
            mode_of(spec_body(&global, &ids()).unwrap().mode.as_ref()).0,
            ServiceMode::Global
        );
    }

    #[test]
    fn an_update_changes_only_its_three_fields() {
        let mut current = spec_body(&input(), &ids()).unwrap();
        current
            .task_template
            .as_mut()
            .unwrap()
            .container_spec
            .as_mut()
            .unwrap()
            .image = Some("nginx:1.27@sha256:abc".into());
        let changed = merge(
            current.clone(),
            &ServiceChange {
                image: Some("nginx:1.28".into()),
                env: None,
                replicas: None,
                redeploy: false,
            },
        )
        .unwrap();
        let image = |s: &ServiceSpec| {
            s.task_template
                .as_ref()
                .unwrap()
                .container_spec
                .as_ref()
                .unwrap()
                .image
                .clone()
        };
        assert_eq!(image(&changed).as_deref(), Some("nginx:1.28"));
        // Only the image differs.
        let mut back = changed.clone();
        back.task_template
            .as_mut()
            .unwrap()
            .container_spec
            .as_mut()
            .unwrap()
            .image = image(&current);
        assert_eq!(back, current);

        let env_only = merge(
            current.clone(),
            &ServiceChange {
                image: Some("nginx:1.27".into()),
                env: Some(vec!["B=2".into()]),
                replicas: Some(5),
                redeploy: false,
            },
        )
        .unwrap();
        assert_eq!(
            image(&env_only),
            image(&current),
            "an unchanged tag keeps its digest"
        );
        let c = env_only
            .task_template
            .as_ref()
            .unwrap()
            .container_spec
            .as_ref()
            .unwrap();
        assert_eq!(c.env.as_deref(), Some(&["B=2".to_string()][..]));
        assert_eq!(
            c.mounts,
            current
                .task_template
                .as_ref()
                .unwrap()
                .container_spec
                .as_ref()
                .unwrap()
                .mounts
        );
        assert_eq!(env_only.endpoint_spec, current.endpoint_spec);
        assert_eq!(mode_of(env_only.mode.as_ref()).1, Some(5));
    }

    #[test]
    fn redeploying_bumps_force_update_and_pulls_the_tag_again() {
        let mut current = spec_body(&input(), &ids()).unwrap();
        current
            .task_template
            .as_mut()
            .unwrap()
            .container_spec
            .as_mut()
            .unwrap()
            .image = Some("nginx:1.27@sha256:abc".into());
        let redeploy = ServiceChange {
            redeploy: true,
            ..Default::default()
        };
        let once = merge(current, &redeploy).unwrap();
        let task = once.task_template.clone().unwrap();
        assert_eq!(task.force_update, Some(1));
        assert_eq!(
            task.container_spec.unwrap().image.as_deref(),
            Some("nginx:1.27")
        );
        assert_eq!(
            merge(once, &redeploy)
                .unwrap()
                .task_template
                .unwrap()
                .force_update,
            Some(2)
        );
    }

    #[test]
    fn replicas_of_a_global_service_are_refused() {
        let mut global = input();
        global.mode = ModeInput::Global;
        let spec = spec_body(&global, &ids()).unwrap();
        let change = ServiceChange {
            replicas: Some(2),
            ..Default::default()
        };
        assert!(merge(spec, &change).is_err());
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn a_created_service_updates_its_image_and_env() {
        let docker = crate::testing::docker();
        let _swarm = crate::testing::swarm(&docker).await;
        crate::testing::ensure_image(&docker).await;
        let name = crate::testing::unique("create");
        let input = ServiceSpecInput {
            name: name.clone(),
            image: crate::testing::IMAGE.into(),
            mode: ModeInput::Replicated { replicas: 1 },
            env: vec!["GREETING=hello".into()],
            command: Some("sleep 300".into()),
            ports: vec![],
            networks: vec![],
            mounts: vec![],
            secrets: vec![],
            configs: vec![],
            restart: None,
        };
        let id = create(&docker, &input).await.unwrap();
        let again = create(&docker, &input).await;
        assert!(
            matches!(&again, Err(e) if e.is(ErrorCode::NameInUse)),
            "{again:?}"
        );

        let version = |docker: Docker, id: String| async move {
            docker
                .inspect_service(&id, None)
                .await
                .unwrap()
                .version
                .unwrap()
                .index
                .unwrap()
        };
        let v1 = version(docker.clone(), id.clone()).await;
        let change = ServiceChange {
            image: Some("busybox:1.37".into()),
            env: Some(vec!["GREETING=bonjour".into()]),
            replicas: None,
            redeploy: false,
        };
        update(&docker, &id, v1, &change).await.unwrap();
        // The same edit from the stale version is a conflict.
        let stale = update(&docker, &id, v1, &change).await;
        assert!(
            matches!(&stale, Err(e) if e.is(ErrorCode::Conflict)),
            "{stale:?}"
        );

        let spec = docker
            .inspect_service(&id, None)
            .await
            .unwrap()
            .spec
            .unwrap();
        let c = spec.task_template.unwrap().container_spec.unwrap();
        assert!(c.image.unwrap().starts_with("busybox:1.37"));
        assert_eq!(c.env.unwrap(), ["GREETING=bonjour"]);
        assert_eq!(c.args.unwrap(), ["sleep", "300"]);
        docker.delete_service(&id).await.unwrap();
    }
}

//! The Run dialog: create and start a container from an image.

use std::collections::HashMap;

use bollard::{
    models::{
        ContainerCreateBody, HostConfig, PortBinding, RestartPolicy as EnginePolicy,
        RestartPolicyNameEnum,
    },
    query_parameters::CreateContainerOptionsBuilder,
    Docker,
};
use serde::Deserialize;

use super::{containers, images};
use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RunSpec {
    pub image: String,
    pub name: Option<String>,
    /// Overrides the image command; parsed with shell quoting rules.
    pub command: Option<String>,
    /// `KEY=value` entries.
    pub env: Vec<String>,
    pub ports: Vec<PortSpec>,
    pub mounts: Vec<MountSpec>,
    pub network: Option<String>,
    pub restart: RestartPolicy,
    /// Remove the container when it exits (`--rm`).
    pub auto_remove: bool,
}

#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PortSpec {
    pub host_ip: Option<String>,
    /// `None` lets the Engine pick a free port.
    pub host_port: Option<u16>,
    pub container_port: u16,
    /// `tcp` or `udp`.
    pub protocol: String,
}

#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MountSpec {
    /// An absolute path on the Host (bind mount) or a volume name.
    pub source: String,
    pub target: String,
    pub read_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "kebab-case")]
pub enum RestartPolicy {
    No,
    Always,
    UnlessStopped,
    OnFailure,
}

/// Translates a [`RunSpec`] into the Engine's create request.
pub fn create_body(spec: &RunSpec) -> AppResult<ContainerCreateBody> {
    let invalid = |m: &str| AppError::Validation(m.to_owned());
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
            return Err(AppError::Validation(format!(
                "\"{entry}\" isn't a KEY=value environment variable."
            )));
        }
    }

    let mut exposed = Vec::new();
    let mut bindings: HashMap<String, Option<Vec<PortBinding>>> = HashMap::new();
    for p in &spec.ports {
        let protocol = match p.protocol.as_str() {
            "tcp" | "udp" | "sctp" => p.protocol.as_str(),
            _ => return Err(invalid("Port protocols are tcp, udp or sctp.")),
        };
        let key = format!("{}/{protocol}", p.container_port);
        exposed.push(key.clone());
        bindings
            .entry(key)
            .or_insert_with(|| Some(vec![]))
            .as_mut()
            .expect("just set")
            .push(PortBinding {
                host_ip: p.host_ip.clone().filter(|ip| !ip.is_empty()),
                host_port: Some(p.host_port.map(|h| h.to_string()).unwrap_or_default()),
            });
    }

    let binds = spec
        .mounts
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
        .collect::<AppResult<Vec<_>>>()?;

    let restart = match spec.restart {
        RestartPolicy::No => RestartPolicyNameEnum::NO,
        RestartPolicy::Always => RestartPolicyNameEnum::ALWAYS,
        RestartPolicy::UnlessStopped => RestartPolicyNameEnum::UNLESS_STOPPED,
        RestartPolicy::OnFailure => RestartPolicyNameEnum::ON_FAILURE,
    };
    if spec.auto_remove && spec.restart != RestartPolicy::No {
        return Err(invalid("Containers removed on exit can't also restart."));
    }

    Ok(ContainerCreateBody {
        image: Some(image.to_owned()),
        cmd,
        env: (!spec.env.is_empty()).then(|| spec.env.clone()),
        exposed_ports: (!exposed.is_empty()).then_some(exposed),
        host_config: Some(HostConfig {
            port_bindings: (!bindings.is_empty()).then_some(bindings),
            binds: (!binds.is_empty()).then_some(binds),
            network_mode: spec.network.clone().filter(|n| !n.trim().is_empty()),
            restart_policy: Some(EnginePolicy {
                name: Some(restart),
                maximum_retry_count: None,
            }),
            auto_remove: Some(spec.auto_remove),
            ..Default::default()
        }),
        ..Default::default()
    })
}

/// Pulls the image if it's missing, then creates and starts the container.
pub async fn run(docker: &Docker, spec: RunSpec) -> AppResult<String> {
    let body = create_body(&spec)?;
    if docker.inspect_image(spec.image.trim()).await.is_err() {
        let failure = std::sync::Mutex::new(None);
        images::pull(docker.clone(), spec.image.trim().to_owned(), |e| {
            if let images::PullEvent::Error { message } = e {
                *failure.lock().expect("lock poisoned") = Some(message);
            }
            true
        })
        .await;
        if let Some(message) = failure.into_inner().expect("lock poisoned") {
            return Err(AppError::Docker(message));
        }
    }
    let mut options = CreateContainerOptionsBuilder::default();
    if let Some(name) = spec
        .name
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty())
    {
        options = options.name(name);
    }
    let id = docker
        .create_container(Some(options.build()), body)
        .await?
        .id;
    containers::act(docker, &id, containers::ContainerAction::Start).await?;
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::docker::testing;

    fn spec() -> RunSpec {
        RunSpec {
            image: "nginx:1.27".into(),
            name: None,
            command: None,
            env: vec![],
            ports: vec![],
            mounts: vec![],
            network: None,
            restart: RestartPolicy::No,
            auto_remove: false,
        }
    }

    #[test]
    fn run_options_map_to_the_engine_request() {
        let body = create_body(&RunSpec {
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
        })
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
    fn invalid_run_options_are_explained() {
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
        ];
        for s in bad {
            assert!(
                matches!(create_body(&s), Err(AppError::Validation(_))),
                "{s:?}"
            );
        }
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn runs_a_container_from_an_image() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let name = testing::unique("run");
        let id = run(
            &docker,
            RunSpec {
                image: testing::IMAGE.into(),
                name: Some(name.clone()),
                command: Some("sleep 30".into()),
                env: vec!["A=1".into()],
                ..spec()
            },
        )
        .await
        .unwrap();
        let listed = containers::list(&docker).await.unwrap();
        let c = listed.iter().find(|c| c.id == id).unwrap();
        assert_eq!(
            (c.name.as_str(), c.state.as_str()),
            (name.as_str(), "running")
        );
        containers::remove(&docker, &id, true, true).await.unwrap();
    }
}

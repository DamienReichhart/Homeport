//! Helpers for integration tests against the local Engine. Tests only touch
//! resources they create, named `homeport-test-*`, and remove them after.

use bollard::{
    models::ContainerCreateBody,
    query_parameters::{
        CreateContainerOptionsBuilder, CreateImageOptionsBuilder, RemoveContainerOptionsBuilder,
    },
    Docker,
};
use futures_util::TryStreamExt;

pub const IMAGE: &str = "alpine:3.22";

pub fn docker() -> Docker {
    Docker::connect_with_local_defaults().expect("local Engine socket")
}

pub fn unique(prefix: &str) -> String {
    format!(
        "homeport-test-{prefix}-{}",
        &uuid::Uuid::new_v4().simple().to_string()[..8]
    )
}

pub async fn ensure_image(docker: &Docker) {
    if docker.inspect_image(IMAGE).await.is_ok() {
        return;
    }
    docker
        .create_image(
            Some(
                CreateImageOptionsBuilder::default()
                    .from_image(IMAGE)
                    .build(),
            ),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .expect("pull test image");
}

/// Creates (not starts) a container running `cmd`, removed when the guard drops.
pub async fn container(docker: &Docker, name: &str, cmd: &[&str]) -> Guard {
    ensure_image(docker).await;
    let created = docker
        .create_container(
            Some(CreateContainerOptionsBuilder::default().name(name).build()),
            ContainerCreateBody {
                image: Some(IMAGE.into()),
                cmd: Some(cmd.iter().map(|s| s.to_string()).collect()),
                labels: Some([("homeport.test".to_string(), "1".to_string())].into()),
                ..Default::default()
            },
        )
        .await
        .expect("create test container");
    Guard {
        docker: docker.clone(),
        id: created.id,
    }
}

pub struct Guard {
    docker: Docker,
    pub id: String,
}

impl Drop for Guard {
    fn drop(&mut self) {
        let docker = self.docker.clone();
        let id = self.id.clone();
        // Best effort: the test runtime may already be shutting down.
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    let _ = docker
                        .remove_container(
                            &id,
                            Some(
                                RemoveContainerOptionsBuilder::default()
                                    .force(true)
                                    .v(true)
                                    .build(),
                            ),
                        )
                        .await;
                })
        })
        .join()
        .ok();
    }
}

/// A user and password the [`registry`] with auth accepts.
pub const REGISTRY_USER: &str = "homeport";
pub const REGISTRY_PASSWORD: &str = "homeport-secret";
/// `htpasswd -B` of the pair above, for `registry:2`.
const REGISTRY_HTPASSWD: &str =
    "homeport:$2b$05$bZ8KDfTcHueCesWIPtuofemWq4exHM9KDBfgMwy4u8LTl7v2XQo4m";
const REGISTRY_IMAGE: &str = "registry:2";

/// A throwaway `registry:2` on a free port of this machine, removed with the
/// guard. With `auth`, it asks for [`REGISTRY_USER`] and [`REGISTRY_PASSWORD`].
pub struct Registry {
    /// `localhost:<port>`, which the Engine lets speak plain HTTP.
    pub address: String,
    _guard: Guard,
}

pub async fn registry(docker: &Docker, auth: bool) -> Registry {
    use bollard::models::{HostConfig, PortBinding};

    if docker.inspect_image(REGISTRY_IMAGE).await.is_err() {
        docker
            .create_image(
                Some(
                    CreateImageOptionsBuilder::default()
                        .from_image(REGISTRY_IMAGE)
                        .build(),
                ),
                None,
                None,
            )
            .try_collect::<Vec<_>>()
            .await
            .expect("pull registry:2");
    }
    let mut env = Vec::new();
    let mut entrypoint = None;
    let mut cmd = None;
    if auth {
        env.extend([
            "REGISTRY_AUTH=htpasswd".to_owned(),
            "REGISTRY_AUTH_HTPASSWD_REALM=homeport-test".to_owned(),
            "REGISTRY_AUTH_HTPASSWD_PATH=/auth/htpasswd".to_owned(),
        ]);
        entrypoint = Some(vec!["/bin/sh".to_owned(), "-c".to_owned()]);
        cmd = Some(vec![format!(
            "mkdir -p /auth && echo '{REGISTRY_HTPASSWD}' > /auth/htpasswd && exec registry serve /etc/docker/registry/config.yml"
        )]);
    }
    let created = docker
        .create_container(
            Some(
                CreateContainerOptionsBuilder::default()
                    .name(&unique("registry"))
                    .build(),
            ),
            ContainerCreateBody {
                image: Some(REGISTRY_IMAGE.into()),
                env: Some(env),
                entrypoint,
                cmd,
                labels: Some([("homeport.test".to_string(), "1".to_string())].into()),
                exposed_ports: Some(vec!["5000/tcp".to_owned()]),
                host_config: Some(HostConfig {
                    port_bindings: Some(
                        [(
                            "5000/tcp".to_owned(),
                            Some(vec![PortBinding {
                                host_ip: Some("127.0.0.1".into()),
                                host_port: Some("0".into()),
                            }]),
                        )]
                        .into(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await
        .expect("create test registry");
    let guard = Guard {
        docker: docker.clone(),
        id: created.id,
    };
    docker
        .start_container(&guard.id, None)
        .await
        .expect("start test registry");
    let port = docker
        .inspect_container(&guard.id, None)
        .await
        .expect("inspect test registry")
        .network_settings
        .and_then(|n| n.ports)
        .and_then(|p| p.get("5000/tcp").cloned().flatten())
        .and_then(|b| b.into_iter().next())
        .and_then(|b| b.host_port)
        .expect("test registry port");
    let address = format!("localhost:{port}");
    // Wait until it answers.
    for _ in 0..50 {
        if tokio::net::TcpStream::connect(("127.0.0.1", port.parse::<u16>().unwrap()))
            .await
            .is_ok()
        {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    Registry {
        address,
        _guard: guard,
    }
}

/// Removes the image name `0` when dropped, even if the test panics first.
pub struct ImageName(pub Docker, pub String);

impl Drop for ImageName {
    fn drop(&mut self) {
        let docker = self.0.clone();
        let name = self.1.clone();
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    let _ = docker
                        .remove_image(
                            &name,
                            None::<bollard::query_parameters::RemoveImageOptions>,
                            None,
                        )
                        .await;
                })
        })
        .join()
        .ok();
    }
}

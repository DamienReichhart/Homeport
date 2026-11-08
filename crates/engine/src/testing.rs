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

/// Held by every test that creates helper containers: one of them removes
/// this computer's leftovers, which would end another test's helper.
pub static HELPERS: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

pub fn docker() -> Docker {
    Docker::connect_with_local_defaults().expect("local Engine socket")
}

/// A client of the local Engine speaking API `major.minor`, to see how an
/// older Engine (or Podman) answers.
pub fn docker_at_api(major: usize, minor: usize) -> Docker {
    let version = bollard::ClientVersion {
        major_version: major,
        minor_version: minor,
    };
    Docker::connect_with_local(&local_address(), 120, &version).expect("local Engine socket")
}

fn local_address() -> String {
    std::env::var("DOCKER_HOST").unwrap_or_else(|_| {
        if cfg!(windows) {
            "npipe:////./pipe/docker_engine".into()
        } else {
            "unix:///var/run/docker.sock".into()
        }
    })
}

/// A client for tests that must fail before reaching an Engine: nothing
/// listens on the discard port, so a request that slips through fails too.
/// Unlike [`docker`], it works on machines without an Engine (CI on macOS
/// and Windows).
pub fn offline() -> Docker {
    Docker::connect_with_http("tcp://127.0.0.1:9", 5, bollard::API_DEFAULT_VERSION)
        .expect("an HTTP client always constructs")
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

/// Runs `sh -c script` in a throwaway container with `volume` mounted at
/// `/data`, waits for it to exit, and removes it.
pub async fn run_with_volume(docker: &Docker, volume: &str, script: &str) {
    output_with_volume(docker, volume, script).await;
}

/// [`run_with_volume`], returning what the script wrote to stdout.
pub async fn output_with_volume(docker: &Docker, volume: &str, script: &str) -> String {
    use bollard::query_parameters::{LogsOptionsBuilder, WaitContainerOptions};

    let guard = container_with_volume_running(docker, volume, &["sh", "-c", script]).await;
    docker
        .wait_container(&guard.id, None::<WaitContainerOptions>)
        .try_collect::<Vec<_>>()
        .await
        .expect("test script exits with 0");
    docker
        .logs(
            &guard.id,
            Some(LogsOptionsBuilder::default().stdout(true).build()),
        )
        .try_collect::<Vec<_>>()
        .await
        .expect("test script output")
        .into_iter()
        .map(|line| String::from_utf8_lossy(&line.into_bytes()).into_owned())
        .collect()
}

/// A stopped container mounting `volume` at `/data`, removed with the guard.
pub async fn container_with_volume(docker: &Docker, volume: &str) -> Guard {
    create_with_volume(docker, volume, &["sleep", "60"]).await
}

async fn container_with_volume_running(docker: &Docker, volume: &str, cmd: &[&str]) -> Guard {
    let guard = create_with_volume(docker, volume, cmd).await;
    docker
        .start_container(&guard.id, None)
        .await
        .expect("start test container");
    guard
}

async fn create_with_volume(docker: &Docker, volume: &str, cmd: &[&str]) -> Guard {
    use bollard::models::HostConfig;

    ensure_image(docker).await;
    let created = docker
        .create_container(
            Some(
                CreateContainerOptionsBuilder::default()
                    .name(&unique("user"))
                    .build(),
            ),
            ContainerCreateBody {
                image: Some(IMAGE.into()),
                cmd: Some(cmd.iter().map(|s| s.to_string()).collect()),
                labels: Some([("homeport.test".to_string(), "1".to_string())].into()),
                host_config: Some(HostConfig {
                    binds: Some(vec![format!("{volume}:/data")]),
                    ..Default::default()
                }),
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

/// A volume holding `etc/note.txt` ("hello\n", owned by 1234:5678, mode
/// 640), removed with the guard.
pub async fn filled_volume(docker: &Docker) -> VolumeGuard {
    let name = unique("vol");
    docker
        .create_volume(bollard::models::VolumeCreateRequest {
            name: Some(name.clone()),
            ..Default::default()
        })
        .await
        .expect("create test volume");
    let guard = VolumeGuard {
        docker: docker.clone(),
        name,
    };
    run_with_volume(
        docker,
        &guard.name,
        "mkdir /data/etc && echo hello > /data/etc/note.txt \
         && chown 1234:5678 /data/etc/note.txt && chmod 640 /data/etc/note.txt",
    )
    .await;
    guard
}

/// Removes the volume `name` when dropped, with whatever is in it.
pub struct VolumeGuard {
    pub docker: Docker,
    pub name: String,
}

impl Drop for VolumeGuard {
    fn drop(&mut self) {
        let docker = self.docker.clone();
        let name = self.name.clone();
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    let _ = docker
                        .remove_volume(
                            &name,
                            Some(
                                bollard::query_parameters::RemoveVolumeOptionsBuilder::default()
                                    .force(true)
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

/// Removes the image name `0` when dropped, even if the test panics first,
/// and the digest references of its repository that a push or pull left
/// (the classic image store lists them as `repo:<none>`).
pub struct ImageName(pub Docker, pub String);

impl Drop for ImageName {
    fn drop(&mut self) {
        let docker = self.0.clone();
        let name = self.1.clone();
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    let (repo, _) = crate::images::split_reference(&name);
                    let digests: Vec<String> = docker
                        .list_images(None::<bollard::query_parameters::ListImagesOptions>)
                        .await
                        .unwrap_or_default()
                        .into_iter()
                        .flat_map(|i| i.repo_digests)
                        .filter(|d| d.split_once('@').is_some_and(|(r, _)| r == repo))
                        .collect();
                    for reference in std::iter::once(name).chain(digests) {
                        let _ = docker
                            .remove_image(
                                &reference,
                                None::<bollard::query_parameters::RemoveImageOptions>,
                                None,
                            )
                            .await;
                    }
                })
        })
        .join()
        .ok();
    }
}

/// Held by every test that needs the local Engine in Swarm mode, so one
/// test's Swarm isn't left under another's feet.
static SWARM: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

/// The local Engine as the single manager of a throwaway Swarm. An Engine
/// already in a Swarm is used as it is and left in it; otherwise the Swarm
/// is created for the test and left (with force) when the guard drops.
pub struct SwarmGuard {
    pub swarm_id: String,
    created: bool,
    _lock: tokio::sync::MutexGuard<'static, ()>,
}

pub async fn swarm(docker: &Docker) -> SwarmGuard {
    use bollard::models::SwarmInitRequest;

    let lock = SWARM.lock().await;
    let info = docker.info().await.expect("engine info");
    let active = info
        .swarm
        .as_ref()
        .and_then(|s| s.local_node_state)
        .is_some_and(|s| s == bollard::models::LocalNodeState::ACTIVE);
    if !active {
        docker
            .init_swarm(SwarmInitRequest {
                listen_addr: Some("127.0.0.1:2377".into()),
                advertise_addr: Some("127.0.0.1".into()),
                ..Default::default()
            })
            .await
            .expect("init a throwaway swarm");
    }
    let swarm_id = docker
        .inspect_swarm()
        .await
        .expect("inspect the swarm")
        .id
        .unwrap_or_default();
    SwarmGuard {
        swarm_id,
        created: !active,
        _lock: lock,
    }
}

impl SwarmGuard {
    /// Whether the test created this Swarm, so it may leave and recreate it.
    pub fn created(&self) -> bool {
        self.created
    }
}

impl Drop for SwarmGuard {
    fn drop(&mut self) {
        if !self.created {
            return;
        }
        // Drop can't await: leave from a runtime of its own.
        std::thread::spawn(|| {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("runtime");
            runtime.block_on(async {
                let options = bollard::query_parameters::LeaveSwarmOptionsBuilder::default()
                    .force(true)
                    .build();
                if let Err(e) = docker().leave_swarm(Some(options)).await {
                    eprintln!("couldn't leave the test swarm: {e}");
                }
            });
        })
        .join()
        .ok();
    }
}

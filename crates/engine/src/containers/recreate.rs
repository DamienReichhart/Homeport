//! Edit: recreate a container with a changed configuration, keeping its
//! name, networks and named volumes, and putting the original back if any
//! step fails.

use bollard::Docker;
use serde::Serialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    auth::RegistryAuth,
    containers::{
        self,
        run::{self, Original, RunSpec},
        ContainerAction,
    },
    COMPOSE_PROJECT,
};

/// Something the recreated container won't have, for the confirmation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Loss {
    /// An anonymous volume: the new container gets an empty one at `path`.
    AnonymousVolume { path: String },
    /// Files changed inside the container since it was created.
    FilesystemChanges { count: u32 },
    /// It runs now, so it stops for as long as the swap takes.
    Downtime,
    /// It belongs to a Stack, whose next deploy recreates it from compose.
    StackRedeploy { stack: String },
}

/// What recreating `original` loses: pure, from its inspect document and the
/// number of paths `docker diff` reports.
pub fn what_is_lost(
    original: &bollard::models::ContainerInspectResponse,
    changed_paths: usize,
) -> Vec<Loss> {
    let mut losses: Vec<Loss> = run::anonymous_volumes(original)
        .into_iter()
        .map(|path| Loss::AnonymousVolume { path })
        .collect();
    if changed_paths > 0 {
        losses.push(Loss::FilesystemChanges {
            count: u32::try_from(changed_paths).unwrap_or(u32::MAX),
        });
    }
    if original
        .state
        .as_ref()
        .and_then(|s| s.running)
        .unwrap_or(false)
    {
        losses.push(Loss::Downtime);
    }
    let stack = original
        .config
        .as_ref()
        .and_then(|c| c.labels.as_ref())
        .and_then(|l| l.get(COMPOSE_PROJECT));
    if let Some(stack) = stack {
        losses.push(Loss::StackRedeploy {
            stack: stack.clone(),
        });
    }
    losses
}

/// [`what_is_lost`] for a container on the Engine.
pub async fn losses(docker: &Docker, id: &str) -> AppResult<Vec<Loss>> {
    let original = docker.inspect_container(id, None).await?;
    let changes = docker.container_changes(id).await?.unwrap_or_default();
    Ok(what_is_lost(&original, changes.len()))
}

/// The Engine operations a recreate needs, so the sequence and its rollback
/// can be tested without one.
trait Steps {
    /// Creates the replacement under `name`, not started; returns its ID.
    async fn create(&self, name: &str) -> AppResult<String>;
    async fn stop(&self, id: &str) -> AppResult<()>;
    async fn start(&self, id: &str) -> AppResult<()>;
    async fn rename(&self, id: &str, name: &str) -> AppResult<()>;
    async fn remove(&self, id: &str) -> AppResult<()>;
}

struct Engine<'a> {
    docker: &'a Docker,
    spec: &'a RunSpec,
    original: &'a Original,
    auth: Option<RegistryAuth>,
}

impl Steps for Engine<'_> {
    async fn create(&self, name: &str) -> AppResult<String> {
        let base = self.original.base(true);
        run::create(
            self.docker,
            self.spec,
            Some(&base),
            Some(name),
            self.auth.clone(),
        )
        .await
    }

    async fn stop(&self, id: &str) -> AppResult<()> {
        containers::act(self.docker, id, ContainerAction::Stop).await
    }

    async fn start(&self, id: &str) -> AppResult<()> {
        containers::act(self.docker, id, ContainerAction::Start).await
    }

    async fn rename(&self, id: &str, name: &str) -> AppResult<()> {
        containers::rename(self.docker, id, name).await
    }

    async fn remove(&self, id: &str) -> AppResult<()> {
        // Anonymous volumes are kept: they become unused volumes, not lost data.
        containers::remove(self.docker, id, true, false).await
    }
}

/// The container being replaced.
struct Old<'a> {
    id: &'a str,
    name: &'a str,
    running: bool,
}

/// Recreates container `id` from `spec` under the same name: the new one is
/// created aside, the old one stopped and renamed, the new one renamed and
/// started (if the old one ran), and only then the old one removed. On any
/// failure the original is put back as it was and the failure returned.
pub async fn recreate(
    docker: &Docker,
    id: &str,
    spec: RunSpec,
    auth: Option<RegistryAuth>,
) -> AppResult<String> {
    let original = Original::inspect(docker, id).await?;
    let name = original
        .container
        .name
        .as_deref()
        .unwrap_or_default()
        .trim_start_matches('/')
        .to_owned();
    let old = Old {
        id: original.container.id.as_deref().unwrap_or(id),
        name: &name,
        running: original
            .container
            .state
            .as_ref()
            .and_then(|s| s.running)
            .unwrap_or(false),
    };
    let steps = Engine {
        docker,
        spec: &spec,
        original: &original,
        auth,
    };
    sequence(&steps, &old, &suffix()).await
}

/// A short suffix for the names the two containers wear during the swap.
fn suffix() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or_default();
    format!("{nanos:08x}")
}

async fn sequence(steps: &impl Steps, old: &Old<'_>, suffix: &str) -> AppResult<String> {
    let new_name = format!("{}-new-{suffix}", old.name);
    let old_name = format!("{}-old-{suffix}", old.name);
    // Nothing has changed yet if creating fails (a bad option, a missing image).
    let new = steps.create(&new_name).await?;

    // The error, and whether the original was already renamed aside.
    let result = async {
        if old.running {
            steps.stop(old.id).await.map_err(|e| (e, false))?;
        }
        steps
            .rename(old.id, &old_name)
            .await
            .map_err(|e| (e, false))?;
        steps.rename(&new, old.name).await.map_err(|e| (e, true))?;
        if old.running {
            steps.start(&new).await.map_err(|e| (e, true))?;
        }
        Ok::<_, (AppError, bool)>(())
    }
    .await;

    let Err((error, aside)) = result else {
        if let Err(e) = steps.remove(old.id).await {
            log::warn!(
                "recreated {} but couldn't remove the original, now {old_name}: {}",
                old.name,
                e.message
            );
        }
        return Ok(new);
    };

    // Put the original back: drop the new one, restore its name, restart it.
    let rollback = async {
        steps.remove(&new).await?;
        if aside {
            steps.rename(old.id, old.name).await?;
        }
        if old.running {
            steps.start(old.id).await?;
        }
        AppResult::Ok(())
    }
    .await;
    match rollback {
        Ok(()) => Err(error),
        Err(rollback_error) => Err(AppError::new(
            ErrorCode::RecreateRolledBack,
            format!(
                "Recreating {} failed, and Homeport couldn't put the original back.",
                old.name
            ),
        )
        .with_hint(format!(
            "The original container may still be named {old_name}{}. Rename it back to {} and start it, or remove it once you've checked nothing in it is needed.",
            if old.running { ", stopped" } else { "" },
            old.name
        ))
        .with_detail(format!(
            "{}; then, putting it back: {}",
            error.message, rollback_error.message
        ))),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    /// Records the steps and fails the one named in `fail`.
    struct Fake {
        log: Mutex<Vec<String>>,
        fail: Vec<&'static str>,
    }

    impl Fake {
        fn failing(fail: &[&'static str]) -> Self {
            Self {
                log: Mutex::new(vec![]),
                fail: fail.to_vec(),
            }
        }

        fn step(&self, step: String) -> AppResult<()> {
            let failed = self.fail.iter().any(|f| step.starts_with(f));
            self.log.lock().unwrap().push(step);
            if failed {
                Err(AppError::new(
                    ErrorCode::PortInUse,
                    "Port 8080 is already in use.",
                ))
            } else {
                Ok(())
            }
        }

        fn log(&self) -> Vec<String> {
            self.log.lock().unwrap().clone()
        }
    }

    impl Steps for Fake {
        async fn create(&self, name: &str) -> AppResult<String> {
            self.step(format!("create {name}"))
                .map(|_| "new".to_owned())
        }
        async fn stop(&self, id: &str) -> AppResult<()> {
            self.step(format!("stop {id}"))
        }
        async fn start(&self, id: &str) -> AppResult<()> {
            self.step(format!("start {id}"))
        }
        async fn rename(&self, id: &str, name: &str) -> AppResult<()> {
            self.step(format!("rename {id} {name}"))
        }
        async fn remove(&self, id: &str) -> AppResult<()> {
            self.step(format!("remove {id}"))
        }
    }

    const RUNNING: Old = Old {
        id: "old",
        name: "web",
        running: true,
    };

    #[tokio::test]
    async fn a_recreate_swaps_the_containers_and_removes_the_original_last() {
        let fake = Fake::failing(&[]);
        assert_eq!(sequence(&fake, &RUNNING, "x").await.unwrap(), "new");
        assert_eq!(
            fake.log(),
            [
                "create web-new-x",
                "stop old",
                "rename old web-old-x",
                "rename new web",
                "start new",
                "remove old",
            ]
        );
    }

    #[tokio::test]
    async fn a_stopped_container_stays_stopped() {
        let fake = Fake::failing(&[]);
        let stopped = Old {
            running: false,
            ..RUNNING
        };
        sequence(&fake, &stopped, "x").await.unwrap();
        assert!(
            !fake.log().iter().any(|s| s.starts_with("st")),
            "{:?}",
            fake.log()
        );
    }

    #[tokio::test]
    async fn a_failed_create_changes_nothing() {
        let fake = Fake::failing(&["create"]);
        let e = sequence(&fake, &RUNNING, "x").await.unwrap_err();
        assert_eq!(e.code, ErrorCode::PortInUse);
        assert_eq!(fake.log(), ["create web-new-x"]);
    }

    #[tokio::test]
    async fn a_failed_start_puts_the_original_back_running() {
        let fake = Fake::failing(&["start new"]);
        let e = sequence(&fake, &RUNNING, "x").await.unwrap_err();
        assert_eq!(e.code, ErrorCode::PortInUse, "the original failure");
        assert_eq!(
            fake.log(),
            [
                "create web-new-x",
                "stop old",
                "rename old web-old-x",
                "rename new web",
                "start new",
                "remove new",
                "rename old web",
                "start old",
            ]
        );
    }

    #[tokio::test]
    async fn a_failed_rename_aside_restarts_the_original() {
        let fake = Fake::failing(&["rename old web-old"]);
        sequence(&fake, &RUNNING, "x").await.unwrap_err();
        let log = fake.log();
        assert_eq!(&log[log.len() - 2..], ["remove new", "start old"]);
    }

    #[tokio::test]
    async fn a_failed_rollback_names_the_leftover_container() {
        let fake = Fake::failing(&["start"]);
        let e = sequence(&fake, &RUNNING, "x").await.unwrap_err();
        assert_eq!(e.code, ErrorCode::RecreateRolledBack);
        assert!(e.hint.unwrap().text.contains("web-old-x, stopped"));
        assert!(e
            .detail
            .unwrap()
            .starts_with("Port 8080 is already in use."));
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn editing_keeps_the_name_volume_and_networks() {
        use bollard::models::NetworkCreateRequest;

        use crate::testing;

        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let name = testing::unique("edit");
        let network = format!("{name}-net");
        docker
            .create_network(NetworkCreateRequest {
                name: network.clone(),
                ..Default::default()
            })
            .await
            .unwrap();
        let original = run::run(
            &docker,
            RunSpec {
                image: testing::IMAGE.into(),
                name: Some(name.clone()),
                command: Some("sleep 120".into()),
                env: vec!["A=1".into()],
                mounts: vec![run::MountSpec {
                    source: format!("{name}-data"),
                    target: "/data".into(),
                    read_only: false,
                }],
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
        docker
            .connect_network(
                &network,
                bollard::models::NetworkConnectRequest {
                    container: original.clone(),
                    endpoint_config: Some(bollard::models::EndpointSettings {
                        aliases: Some(vec!["db".into()]),
                        ..Default::default()
                    }),
                },
            )
            .await
            .unwrap();

        let mut spec = run::spec_for(&docker, &original).await.unwrap();
        spec.env = vec!["A=2".into()];
        let edited = recreate(&docker, &original, spec, None).await.unwrap();

        let inspected = containers::inspect(&docker, &edited).await.unwrap();
        assert_eq!(inspected["Name"], format!("/{name}"));
        assert_eq!(inspected["State"]["Running"], true);
        let env = inspected["Config"]["Env"].as_array().unwrap();
        assert!(env.iter().any(|e| e == "A=2"), "{env:?}");
        assert_eq!(inspected["Mounts"][0]["Name"], format!("{name}-data"));
        let aliases = &inspected["NetworkSettings"]["Networks"][&network]["Aliases"];
        assert!(
            aliases.as_array().unwrap().iter().any(|a| a == "db"),
            "{aliases}"
        );
        assert!(
            docker.inspect_container(&original, None).await.is_err(),
            "the original is gone"
        );

        containers::remove(&docker, &edited, true, true)
            .await
            .unwrap();
        docker.remove_network(&network).await.unwrap();
        docker
            .remove_volume(
                &format!("{name}-data"),
                None::<bollard::query_parameters::RemoveVolumeOptions>,
            )
            .await
            .unwrap();
    }

    #[test]
    fn losses_name_anonymous_volumes_changes_downtime_and_the_stack() {
        let original: bollard::models::ContainerInspectResponse =
            serde_json::from_value(serde_json::json!({
                "State": { "Running": true },
                "Config": { "Labels": { "com.docker.compose.project": "shop" } },
                "HostConfig": { "Binds": ["site:/srv"] },
                "Mounts": [
                    { "Type": "volume", "Name": "site", "Destination": "/srv" },
                    { "Type": "volume", "Name": "a".repeat(64), "Destination": "/var/cache" }
                ]
            }))
            .unwrap();
        assert_eq!(
            what_is_lost(&original, 3),
            [
                Loss::AnonymousVolume {
                    path: "/var/cache".into()
                },
                Loss::FilesystemChanges { count: 3 },
                Loss::Downtime,
                Loss::StackRedeploy {
                    stack: "shop".into()
                },
            ]
        );
        assert!(what_is_lost(&Default::default(), 0).is_empty());
    }
}

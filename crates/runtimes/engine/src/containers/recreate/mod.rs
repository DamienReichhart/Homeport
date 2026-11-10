//! Edit: recreate a container with a changed configuration, keeping its
//! name, networks and named volumes, and putting the original back if any
//! step fails.

use bollard::Docker;
use serde::Serialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;

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
            msg::RECREATE_ROLLBACK_FAILED.with("name", old.name),
        )
        .with_hint(
            if old.running {
                msg::RECREATE_ROLLBACK_FAILED_STOPPED_HINT
            } else {
                msg::RECREATE_ROLLBACK_FAILED_HINT
            }
            .with("asideName", old_name.as_str())
            .with("name", old.name),
        )
        .with_detail(format!(
            "{}; then, putting it back: {}",
            error.message, rollback_error.message
        ))),
    }
}

#[cfg(test)]
mod tests;

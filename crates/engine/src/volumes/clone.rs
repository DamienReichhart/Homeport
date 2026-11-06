//! Copying a volume into a new one on the same Host, through a helper
//! container (ADR 0015): the Engine API has no call for it.

use std::collections::HashMap;

use bollard::{models::VolumeCreateRequest, query_parameters::RemoveVolumeOptionsBuilder, Docker};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::helper::{self, Mount};
use crate::messages as msg;

/// The prefix of the labels that make a volume part of a Compose project;
/// a clone leaves them out so it doesn't join the source's Stack.
const COMPOSE_LABELS: &str = "com.docker.compose.";

/// Copies `source` into a new volume `target` with the source's driver,
/// driver options and labels (Compose's aside), keeping files' permissions
/// and owners. The source is mounted read-only. A failed copy removes the
/// half-made target and says so. Returns the new volume's name.
pub async fn clone(
    docker: &Docker,
    source: &str,
    target: &str,
    pulled_image: bool,
) -> AppResult<String> {
    let result = copy(docker, source, target).await;
    if pulled_image {
        helper::remove_image_if_unused(docker).await;
    }
    result
}

async fn copy(docker: &Docker, source: &str, target: &str) -> AppResult<String> {
    let target = target.trim();
    if target.is_empty() {
        return Err(AppError::validation(msg::VOLUME_NAME));
    }
    if docker.inspect_volume(target).await.is_ok() {
        return Err(AppError::new(
            ErrorCode::NameInUse,
            msg::VOLUME_EXISTS.with("volume", target),
        )
        .with_hint(msg::VOLUME_EXISTS_CLONE_HINT));
    }
    let original = docker.inspect_volume(source).await?;
    docker
        .create_volume(VolumeCreateRequest {
            name: Some(target.to_owned()),
            driver: Some(original.driver),
            driver_opts: Some(original.options),
            labels: Some(own_labels(original.labels)),
            ..Default::default()
        })
        .await?;

    let mounts = [
        Mount {
            volume: source,
            target: "/from",
            read_only: true,
        },
        Mount {
            volume: target,
            target: "/to",
            read_only: false,
        },
    ];
    let failure = match helper::run(docker, &mounts, &["cp", "-a", "/from/.", "/to/"]).await {
        Ok((0, _)) => return Ok(target.to_owned()),
        Ok((_, stderr)) => AppError::new(
            ErrorCode::EngineError,
            msg::CLONE_FAILED.with("source", source),
        )
        .with_detail(stderr.trim()),
        Err(e) => e,
    };
    let options = RemoveVolumeOptionsBuilder::default().force(true).build();
    match docker.remove_volume(target, Some(options)).await {
        Ok(()) => Err(failure.with_hint(msg::CLONE_REMOVED_HINT.with("target", target))),
        Err(e) => {
            log::warn!("couldn't remove the incomplete clone {target}: {e}");
            Err(failure.with_hint(msg::CLONE_LEFT_HINT.with("target", target)))
        }
    }
}

/// `labels` without Compose's.
fn own_labels(labels: HashMap<String, String>) -> HashMap<String, String> {
    labels
        .into_iter()
        .filter(|(key, _)| !key.starts_with(COMPOSE_LABELS))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing;

    #[test]
    fn clones_leave_the_stack() {
        let labels = HashMap::from([
            ("com.docker.compose.project".to_owned(), "shop".to_owned()),
            ("com.docker.compose.volume".to_owned(), "db".to_owned()),
            ("backup".to_owned(), "nightly".to_owned()),
        ]);
        assert_eq!(
            own_labels(labels),
            HashMap::from([("backup".to_owned(), "nightly".to_owned())])
        );
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine and busybox:1.37"]
    async fn a_clone_has_the_same_files_owners_and_modes() {
        let _serial = testing::HELPERS.lock().await;
        let docker = testing::docker();
        let source = testing::filled_volume(&docker).await;
        let copy = testing::VolumeGuard {
            docker: docker.clone(),
            name: testing::unique("clone"),
        };

        let made = clone(&docker, &source.name, &copy.name, false)
            .await
            .unwrap();
        assert_eq!(made, copy.name);
        let stat = "stat -c '%u:%g %a %s' /data/etc/note.txt";
        assert_eq!(
            testing::output_with_volume(&docker, &copy.name, stat).await,
            testing::output_with_volume(&docker, &source.name, stat).await,
        );

        let again = clone(&docker, &source.name, &copy.name, false).await;
        assert_eq!(again.unwrap_err().code, ErrorCode::NameInUse);
        let missing = clone(
            &docker,
            &testing::unique("nope"),
            &testing::unique("x"),
            false,
        )
        .await;
        assert!(missing.is_err());
    }
}

//! The helper container: the one thing Homeport runs on a Host (ADR 0015).
//!
//! The Engine API can't read or write a volume, so browsing, backing up,
//! restoring and cloning one go through a throwaway `busybox` container that
//! mounts it. A helper has no network and a read-only root filesystem, mounts
//! a volume read-only unless it must write to it, and is removed afterwards:
//! by Homeport when it's done, by the Engine when a running helper's hour is
//! up ([`LIFETIME_SECS`]), or by Homeport when this computer next connects to
//! the Host ([`remove_leftovers`]).

use std::collections::HashMap;

use bollard::{
    models::{ContainerCreateBody, HostConfig},
    query_parameters::{
        CreateContainerOptionsBuilder, ListContainersOptionsBuilder, LogsOptionsBuilder,
        RemoveContainerOptionsBuilder, RemoveImageOptions, WaitContainerOptions,
    },
    Docker,
};
use futures_util::StreamExt;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;

/// The helper's image: multi-arch, about 2 MB, with `find`, `stat`, `cp`, `sleep`.
pub const IMAGE: &str = "busybox:1.37";
/// What the confirmation before pulling [`IMAGE`] says it weighs.
pub const IMAGE_SIZE: u64 = 2_200_000;
/// Marks a helper, so Homeport hides it and can find it again.
pub const LABEL: &str = homeport_core::host::HELPER_LABEL;
/// The computer that created a helper; only its own leftovers are removed,
/// so one computer never ends another's session on a shared Host.
pub const OWNER_LABEL: &str = "com.homeport.helper.owner";
/// Where a browsed volume is mounted in its helper.
pub const ROOT: &str = "/volume";
/// How long a running helper lives at most, even if Homeport goes away.
pub const LIFETIME_SECS: u32 = 3600;
/// How long a request waits for the Engine's answer when that answer only
/// comes once a copy is done (a restore's upload, a clone's `cp`): the
/// client's usual two minutes would end a large copy over a slow link.
/// Cancelling still stops it at once.
pub const COPY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(24 * 3600);

/// A volume mounted in a helper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mount<'a> {
    pub volume: &'a str,
    pub target: &'a str,
    pub read_only: bool,
}

impl Mount<'_> {
    fn bind(&self) -> String {
        let mode = if self.read_only { "ro" } else { "rw" };
        format!("{}:{}:{mode}", self.volume, self.target)
    }
}

/// Whether `labels` are a helper's.
pub fn is_helper(labels: &HashMap<String, String>) -> bool {
    labels.contains_key(LABEL)
}

/// The helper's image, and whether the Host has it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HelperImage {
    pub image: String,
    pub present: bool,
    /// About how much a pull downloads, in bytes.
    pub size: u64,
}

pub async fn image(docker: &Docker) -> AppResult<HelperImage> {
    Ok(HelperImage {
        image: IMAGE.into(),
        present: image_present(docker).await?,
        size: IMAGE_SIZE,
    })
}

/// Whether the helper's image is on the Host.
pub async fn image_present(docker: &Docker) -> AppResult<bool> {
    match docker.inspect_image(IMAGE).await {
        Ok(_) => Ok(true),
        Err(e) => match AppError::from(e) {
            e if e.is(ErrorCode::ImageNotFound) || e.is(ErrorCode::NotFound) => Ok(false),
            e => Err(e),
        },
    }
}

/// Starts a helper with `mounts` that sleeps for [`LIFETIME_SECS`], then is
/// removed by the Engine. For browsing, which runs commands in it.
pub async fn start(docker: &Docker, mounts: &[Mount<'_>]) -> AppResult<String> {
    let id = create(docker, mounts, &["sleep", &LIFETIME_SECS.to_string()], true).await?;
    if let Err(e) = docker.start_container(&id, None).await {
        remove(docker, &id).await;
        return Err(e.into());
    }
    Ok(id)
}

/// Creates a helper with `mounts` without starting it, for the archive API,
/// which reads and writes a stopped container's volumes. Remove it with
/// [`remove`] once done.
pub async fn create_stopped(docker: &Docker, mounts: &[Mount<'_>]) -> AppResult<String> {
    create(docker, mounts, &["true"], false).await
}

/// Runs `cmd` in a helper with `mounts` until it exits, then removes it.
/// Returns its exit code and what it wrote to stderr.
pub async fn run(docker: &Docker, mounts: &[Mount<'_>], cmd: &[&str]) -> AppResult<(i64, String)> {
    let id = create(docker, mounts, cmd, false).await?;
    let result = run_to_end(docker, &id).await;
    remove(docker, &id).await;
    result
}

async fn run_to_end(docker: &Docker, id: &str) -> AppResult<(i64, String)> {
    docker.start_container(id, None).await?;
    let mut code = 0;
    let patient = docker.clone().with_timeout(COPY_TIMEOUT);
    let mut waits = patient.wait_container(id, None::<WaitContainerOptions>);
    while let Some(wait) = waits.next().await {
        match wait {
            Ok(response) => code = response.status_code,
            // bollard reports a non-zero exit as an error carrying the code.
            Err(bollard::errors::Error::DockerContainerWaitError { code: c, .. }) => code = c,
            Err(e) => return Err(e.into()),
        }
    }
    let mut stderr = String::new();
    let mut logs = docker.logs(
        id,
        Some(
            LogsOptionsBuilder::default()
                .stdout(false)
                .stderr(true)
                .build(),
        ),
    );
    while let Some(Ok(line)) = logs.next().await {
        stderr.push_str(&String::from_utf8_lossy(&line.into_bytes()));
    }
    Ok((code, stderr))
}

async fn create(
    docker: &Docker,
    mounts: &[Mount<'_>],
    cmd: &[&str],
    auto_remove: bool,
) -> AppResult<String> {
    if docker.info().await?.os_type.as_deref() == Some("windows") {
        return Err(AppError::validation(msg::HELPER_WINDOWS));
    }
    if !image_present(docker).await? {
        return Err(image_unavailable(None));
    }
    let name = format!(
        "homeport-helper-{}",
        &uuid::Uuid::new_v4().simple().to_string()[..12]
    );
    let created = docker
        .create_container(
            Some(CreateContainerOptionsBuilder::default().name(&name).build()),
            ContainerCreateBody {
                image: Some(IMAGE.into()),
                cmd: Some(cmd.iter().map(|s| s.to_string()).collect()),
                labels: Some(HashMap::from([
                    (LABEL.to_owned(), "true".to_owned()),
                    (OWNER_LABEL.to_owned(), owner()),
                ])),
                network_disabled: Some(true),
                host_config: Some(HostConfig {
                    binds: Some(mounts.iter().map(Mount::bind).collect()),
                    auto_remove: Some(auto_remove),
                    network_mode: Some("none".into()),
                    readonly_rootfs: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await?;
    Ok(created.id)
}

/// Removes a helper, running or not. Never fails: a helper that's gone
/// already is fine, and one that can't be removed now is removed as a
/// leftover later.
pub async fn remove(docker: &Docker, id: &str) {
    let options = RemoveContainerOptionsBuilder::default()
        .force(true)
        .v(false)
        .build();
    if let Err(e) = docker.remove_container(id, Some(options)).await {
        let e = AppError::from(e);
        if !e.is(ErrorCode::NotFound) && !e.is(ErrorCode::Conflict) {
            log::warn!("couldn't remove the helper container {id}: {e}");
        }
    }
}

/// Removes the helpers this computer left on a Host (Homeport quit or
/// crashed during a session), except those in `keep`, which are still in
/// use. Returns how many it removed.
pub async fn remove_leftovers(docker: &Docker, keep: &[String]) -> AppResult<usize> {
    let ids: Vec<String> = helpers(docker, true)
        .await?
        .into_iter()
        .filter(|id| !keep.contains(id))
        .collect();
    for id in &ids {
        remove(docker, id).await;
    }
    Ok(ids.len())
}

/// Removes the helper's image if no helper, from any computer, still uses
/// it; for when Homeport pulled the image itself. Other containers using it
/// keep it, since the Engine refuses to remove it then.
pub async fn remove_image_if_unused(docker: &Docker) {
    match helpers(docker, false).await {
        Ok(ids) if ids.is_empty() => {}
        Ok(_) => return,
        Err(e) => return log::warn!("couldn't list helper containers: {e}"),
    }
    if let Err(e) = docker
        .remove_image(IMAGE, None::<RemoveImageOptions>, None)
        .await
    {
        log::info!("kept the helper image {IMAGE}: {}", AppError::from(e));
    }
}

/// The ids of the helpers on a Host: this computer's only, or everyone's.
async fn helpers(docker: &Docker, own: bool) -> AppResult<Vec<String>> {
    let mut labels = vec![LABEL.to_owned()];
    if own {
        labels.push(format!("{OWNER_LABEL}={}", owner()));
    }
    let filters = HashMap::from([("label".to_owned(), labels)]);
    Ok(docker
        .list_containers(Some(
            ListContainersOptionsBuilder::default()
                .all(true)
                .filters(&filters)
                .build(),
        ))
        .await?
        .into_iter()
        .filter_map(|c| c.id)
        .collect())
}

/// This computer, as helpers' owner label names it.
fn owner() -> String {
    gethostname::gethostname().to_string_lossy().into_owned()
}

/// The helper's image isn't on the Host and couldn't be pulled; `cause` is
/// the pull's error, kept in Details.
pub fn image_unavailable(cause: Option<&AppError>) -> AppError {
    let error = AppError::new(
        ErrorCode::HelperImageUnavailable,
        msg::HELPER_IMAGE_UNAVAILABLE.with("image", IMAGE),
    )
    .with_hint(msg::HELPER_IMAGE_UNAVAILABLE_HINT.with("image", IMAGE))
    .with_command(format!("docker pull {IMAGE}"));
    match cause {
        Some(cause) => error.with_detail(match &cause.detail {
            Some(detail) => format!("{}\n{detail}", cause.message),
            None => cause.message.to_string(),
        }),
        None => error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{containers::files, testing};

    #[test]
    fn mounts_say_whether_they_are_read_only() {
        let ro = Mount {
            volume: "db",
            target: ROOT,
            read_only: true,
        };
        assert_eq!(ro.bind(), "db:/volume:ro");
        let rw = Mount {
            read_only: false,
            ..ro
        };
        assert_eq!(rw.bind(), "db:/volume:rw");
    }

    #[test]
    fn helpers_are_told_apart_by_their_label() {
        assert!(is_helper(&HashMap::from([(LABEL.into(), "true".into())])));
        assert!(!is_helper(&HashMap::new()));
    }

    #[test]
    fn a_missing_image_says_how_to_get_it() {
        let e = image_unavailable(None);
        assert_eq!(e.code, ErrorCode::HelperImageUnavailable);
        assert_eq!(e.hint.unwrap().commands, ["docker pull busybox:1.37"]);
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine and busybox:1.37"]
    async fn a_helper_lists_a_volume_and_leaves_nothing_behind() {
        let _serial = testing::HELPERS.lock().await;
        let docker = testing::docker();
        let volume = testing::filled_volume(&docker).await;
        let id = start(
            &docker,
            &[Mount {
                volume: &volume.name,
                target: ROOT,
                read_only: true,
            }],
        )
        .await
        .unwrap();

        // busybox's find and stat answer in the shape the file browser reads.
        let listed = files::list(&docker, &id, "/volume/etc").await;
        let dir = tempfile::tempdir().unwrap();
        let local = dir.path().join("new.txt");
        std::fs::write(&local, "nope").unwrap();
        let written = files::upload(&docker, &id, "/volume", &[local]).await;
        remove(&docker, &id).await;
        let listed = listed.unwrap();
        assert_eq!(listed.len(), 1, "{listed:?}");
        assert_eq!(listed[0].name, "note.txt");
        assert_eq!(listed[0].mode, "-rw-r-----");
        assert_eq!(listed[0].size, 6);
        assert!(written.is_err(), "the volume is mounted read-only");
        assert!(helpers(&docker, true).await.unwrap().is_empty());
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine and busybox:1.37"]
    async fn leftovers_of_this_computer_are_removed() {
        let _serial = testing::HELPERS.lock().await;
        let docker = testing::docker();
        let volume = testing::filled_volume(&docker).await;
        let mounts = [Mount {
            volume: &volume.name,
            target: ROOT,
            read_only: true,
        }];
        let open = start(&docker, &mounts).await.unwrap();
        start(&docker, &mounts).await.unwrap();
        create_stopped(&docker, &mounts).await.unwrap();
        assert_eq!(
            remove_leftovers(&docker, std::slice::from_ref(&open))
                .await
                .unwrap(),
            2
        );
        assert_eq!(
            helpers(&docker, true).await.unwrap(),
            std::slice::from_ref(&open)
        );
        remove(&docker, &open).await;
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine and busybox:1.37"]
    async fn a_run_reports_its_exit_code_and_stderr() {
        let _serial = testing::HELPERS.lock().await;
        let docker = testing::docker();
        let volume = testing::filled_volume(&docker).await;
        let mounts = [Mount {
            volume: &volume.name,
            target: ROOT,
            read_only: true,
        }];
        let (code, stderr) = run(&docker, &mounts, &["touch", "/volume/x"])
            .await
            .unwrap();
        assert_ne!(code, 0);
        assert!(stderr.contains("Read-only file system"), "{stderr}");
        let (code, _) = run(&docker, &mounts, &["test", "-f", "/volume/etc/note.txt"])
            .await
            .unwrap();
        assert_eq!(code, 0);
        assert!(helpers(&docker, true).await.unwrap().is_empty());
    }
}

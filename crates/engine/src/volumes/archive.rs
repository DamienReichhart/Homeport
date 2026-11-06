//! Backing a volume up to a `.tar` file on this machine, and restoring one
//! from such a file, through a helper container (ADR 0015).
//!
//! The helper is created but never started: the archive API reads and writes
//! a stopped container's volumes. The archive streams both ways, so memory
//! stays flat for volumes of several gigabytes, and its paths are relative to
//! the volume's root (`./etc/note.txt`), the layout of `tar -C <volume> -c .`,
//! so a backup also restores with plain `tar`.

use std::{
    collections::HashMap,
    io::Read,
    path::{Path, PathBuf},
};

use bollard::{
    models::VolumeCreateRequest,
    query_parameters::{
        DownloadFromContainerOptionsBuilder, ListContainersOptionsBuilder,
        UploadToContainerOptionsBuilder,
    },
    Docker,
};

use homeport_core::error::{local_file, AppError, AppResult, ErrorCode};

use super::helper::{self, Mount};
use crate::{
    messages as msg,
    transfer::{self, TransferEvent, REPORT_EVERY},
};

/// Where a restore writes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RestoreSpec {
    pub volume: String,
    /// Create `volume` first; it must not exist.
    pub create: bool,
    /// Empty an existing volume that holds files before extracting.
    /// Without it, such a volume is refused with `volume_not_empty`.
    pub replace: bool,
}

/// The names of the running containers mounting `volume`, helpers aside.
pub async fn running_users(docker: &Docker, volume: &str) -> AppResult<Vec<String>> {
    let filters = HashMap::from([
        ("volume".to_owned(), vec![volume.to_owned()]),
        ("status".to_owned(), vec!["running".to_owned()]),
    ]);
    let mut names: Vec<String> = docker
        .list_containers(Some(
            ListContainersOptionsBuilder::default()
                .filters(&filters)
                .build(),
        ))
        .await?
        .into_iter()
        .filter(|c| !c.labels.as_ref().is_some_and(helper::is_helper))
        .filter_map(|c| c.names.and_then(|n| n.into_iter().next()))
        .map(|n| n.trim_start_matches('/').to_owned())
        .collect();
    names.sort();
    Ok(names)
}

/// A helper that is removed when dropped, also when the transfer is
/// cancelled midway, and the image with it when Homeport pulled it for this.
struct Helper {
    docker: Docker,
    id: Option<String>,
    pulled_image: bool,
}

impl Helper {
    async fn create(docker: &Docker, mount: Mount<'_>, pulled_image: bool) -> AppResult<Self> {
        Ok(Self {
            id: Some(helper::create_stopped(docker, &[mount]).await?),
            docker: docker.clone(),
            pulled_image,
        })
    }

    fn id(&self) -> &str {
        self.id.as_deref().unwrap_or_default()
    }

    async fn remove(mut self) {
        if let Some(id) = self.id.take() {
            remove(self.docker.clone(), id, self.pulled_image).await;
        }
    }
}

impl Drop for Helper {
    fn drop(&mut self) {
        let Some(id) = self.id.take() else { return };
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            runtime.spawn(remove(self.docker.clone(), id, self.pulled_image));
        }
    }
}

async fn remove(docker: Docker, id: String, pulled_image: bool) {
    helper::remove(&docker, &id).await;
    if pulled_image {
        helper::remove_image_if_unused(&docker).await;
    }
}

/// Saves `volume`'s contents to `destination`, streaming progress. The volume
/// is mounted read-only; a failed or cancelled backup leaves no file behind.
pub async fn backup(
    docker: Docker,
    volume: String,
    destination: PathBuf,
    pulled_image: bool,
    sink: impl Fn(TransferEvent) -> bool,
) {
    let result = backup_to(&docker, &volume, &destination, pulled_image, &sink).await;
    transfer::finish(result.map(|()| vec![]), "volume backup", &sink);
}

async fn backup_to(
    docker: &Docker,
    volume: &str,
    destination: &Path,
    pulled_image: bool,
    sink: &impl Fn(TransferEvent) -> bool,
) -> AppResult<()> {
    let mount = Mount {
        volume,
        target: helper::ROOT,
        read_only: true,
    };
    let helper = Helper::create(docker, mount, pulled_image).await?;
    let stream = docker.download_from_container(
        helper.id(),
        Some(
            DownloadFromContainerOptionsBuilder::default()
                .path(&format!("{}/.", helper::ROOT))
                .build(),
        ),
    );
    let result = transfer::write_file(stream, destination, sink).await;
    helper.remove().await;
    result
}

/// Restores the `.tar` at `source` into a volume, streaming progress; the
/// file is read as the Engine takes it, never whole into memory.
pub async fn restore(
    docker: Docker,
    source: PathBuf,
    spec: RestoreSpec,
    pulled_image: bool,
    sink: impl Fn(TransferEvent) -> bool,
) {
    let result = restore_from(&docker, &source, &spec, pulled_image, &sink).await;
    transfer::finish(result.map(|()| vec![]), "volume restore", &sink);
}

async fn restore_from(
    docker: &Docker,
    source: &Path,
    spec: &RestoreSpec,
    pulled_image: bool,
    sink: &impl Fn(TransferEvent) -> bool,
) -> AppResult<()> {
    let volume = spec.volume.trim();
    if volume.is_empty() {
        return Err(AppError::validation(msg::VOLUME_NAME));
    }
    check_tar(source)?;
    if spec.create {
        if docker.inspect_volume(volume).await.is_ok() {
            return Err(AppError::new(
                ErrorCode::NameInUse,
                msg::VOLUME_EXISTS.with("volume", volume),
            )
            .with_hint(msg::VOLUME_EXISTS_HINT));
        }
        docker
            .create_volume(VolumeCreateRequest {
                name: Some(volume.to_owned()),
                ..Default::default()
            })
            .await?;
    } else {
        let running = running_users(docker, volume).await?;
        if !running.is_empty() {
            return Err(in_use(volume, &running));
        }
        if !is_empty(docker, volume).await? {
            if !spec.replace {
                return Err(AppError::new(
                    ErrorCode::VolumeNotEmpty,
                    msg::VOLUME_NOT_EMPTY.with("volume", volume),
                ));
            }
            empty(docker, volume).await?;
        }
    }

    let mount = Mount {
        volume,
        target: helper::ROOT,
        read_only: false,
    };
    let helper = Helper::create(docker, mount, pulled_image).await?;
    let result = upload(docker, helper.id(), source, sink).await;
    helper.remove().await;
    result.map_err(|e| not_a_tar(e, source))
}

/// Streams the file at `source` into the helper's volume, reporting progress.
async fn upload(
    docker: &Docker,
    id: &str,
    source: &Path,
    sink: &impl Fn(TransferEvent) -> bool,
) -> AppResult<()> {
    let (body, total, mut rx) = transfer::file_body(source).await?;
    let options = UploadToContainerOptionsBuilder::default()
        .path(helper::ROOT)
        .build();
    let upload = docker.upload_to_container(id, Some(options), bollard::body_try_stream(body));
    tokio::pin!(upload);
    let (mut sent, mut reported) = (0u64, 0u64);
    loop {
        tokio::select! {
            Some(n) = rx.recv() => {
                sent += n;
                if sent - reported >= REPORT_EVERY {
                    reported = sent;
                    if !sink(TransferEvent::Progress { bytes: sent, total }) {
                        return Ok(());
                    }
                }
            }
            result = &mut upload => {
                result?;
                break;
            }
        }
    }
    sink(TransferEvent::Progress {
        bytes: total.unwrap_or(sent),
        total,
    });
    Ok(())
}

/// Whether `volume` holds nothing, hidden files included.
async fn is_empty(docker: &Docker, volume: &str) -> AppResult<bool> {
    let mount = Mount {
        volume,
        target: helper::ROOT,
        read_only: true,
    };
    let test = format!("[ -z \"$(ls -A {})\" ]", helper::ROOT);
    let (code, _) = helper::run(docker, &[mount], &["sh", "-c", &test]).await?;
    Ok(code == 0)
}

/// Deletes everything in `volume`, keeping the volume itself.
async fn empty(docker: &Docker, volume: &str) -> AppResult<()> {
    let mount = Mount {
        volume,
        target: helper::ROOT,
        read_only: false,
    };
    let cmd = ["find", helper::ROOT, "-mindepth", "1", "-delete"];
    match helper::run(docker, &[mount], &cmd).await? {
        (0, _) => Ok(()),
        (_, stderr) => Err(AppError::new(
            ErrorCode::EngineError,
            msg::VOLUME_NOT_EMPTIED.with("volume", volume),
        )
        .with_detail(stderr.trim())),
    }
}

/// The running containers using a volume stop a restore.
pub fn in_use(volume: &str, containers: &[String]) -> AppError {
    AppError::new(
        ErrorCode::ContainerRunning,
        msg::VOLUME_USED_BY_RUNNING
            .with("volume", volume)
            .with("containers", containers.join(", ")),
    )
    .with_hint(msg::VOLUME_USED_BY_RUNNING_HINT)
}

/// Refuses a file that doesn't start with a tar header (`ustar` at offset
/// 257, in POSIX and GNU archives), before anything is changed.
fn check_tar(source: &Path) -> AppResult<()> {
    let mut header = [0u8; 512];
    let mut file = std::fs::File::open(source).map_err(|e| local_file(source, e))?;
    let read = file.read(&mut header).map_err(|e| local_file(source, e))?;
    if read == header.len() && &header[257..262] == b"ustar" {
        Ok(())
    } else {
        Err(not_a_tar_error(source))
    }
}

fn not_a_tar_error(source: &Path) -> AppError {
    AppError::new(
        ErrorCode::NotATarArchive,
        msg::VOLUME_NOT_A_TAR.with("path", source),
    )
    .with_hint(msg::VOLUME_NOT_A_TAR_HINT)
}

/// The Engine's words for an archive it can't unpack, partway through.
fn not_a_tar(error: AppError, source: &Path) -> AppError {
    let detail = error.detail.as_deref().unwrap_or_default().to_lowercase();
    if detail.contains("unexpected eof")
        || detail.contains("invalid tar header")
        || detail.contains("archive/tar")
    {
        not_a_tar_error(source).with_detail(detail)
    } else {
        error
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::testing;

    fn recorder() -> (
        impl Fn(TransferEvent) -> bool,
        Arc<Mutex<Vec<TransferEvent>>>,
    ) {
        let events = Arc::new(Mutex::new(Vec::new()));
        let sink = {
            let events = events.clone();
            move |e| {
                events.lock().unwrap().push(e);
                true
            }
        };
        (sink, events)
    }

    fn last(events: &Mutex<Vec<TransferEvent>>) -> TransferEvent {
        events.lock().unwrap().last().cloned().unwrap()
    }

    #[test]
    fn only_tar_archives_are_restored() {
        let dir = tempfile::tempdir().unwrap();
        let garbage = dir.path().join("notes.tar");
        std::fs::write(&garbage, "not an archive").unwrap();
        assert_eq!(
            check_tar(&garbage).unwrap_err().code,
            ErrorCode::NotATarArchive
        );

        let archive = dir.path().join("ok.tar");
        let mut builder = tar::Builder::new(std::fs::File::create(&archive).unwrap());
        let mut header = tar::Header::new_ustar();
        header.set_size(2);
        header.set_cksum();
        builder.append_data(&mut header, "./a", &b"hi"[..]).unwrap();
        builder.finish().unwrap();
        check_tar(&archive).unwrap();
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine and busybox:1.37"]
    async fn a_backup_restores_into_a_new_volume_with_owners_and_modes() {
        let _serial = testing::HELPERS.lock().await;
        let docker = testing::docker();
        let original = testing::filled_volume(&docker).await;
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("backup.tar");

        let (sink, events) = recorder();
        backup(
            docker.clone(),
            original.name.clone(),
            file.clone(),
            false,
            sink,
        )
        .await;
        assert_eq!(last(&events), TransferEvent::Done { images: vec![] });
        // The layout of `tar -C <volume> -c .`.
        let mut names: Vec<String> = tar::Archive::new(std::fs::File::open(&file).unwrap())
            .entries()
            .unwrap()
            .map(|e| e.unwrap().path().unwrap().display().to_string())
            .collect();
        names.sort();
        assert_eq!(names, ["./", "./etc/", "./etc/note.txt"]);

        let copy = testing::VolumeGuard {
            docker: docker.clone(),
            name: testing::unique("restored"),
        };
        let (sink, events) = recorder();
        let spec = RestoreSpec {
            volume: copy.name.clone(),
            create: true,
            replace: false,
        };
        restore(docker.clone(), file.clone(), spec.clone(), false, sink).await;
        assert_eq!(last(&events), TransferEvent::Done { images: vec![] });
        let listing = testing::output_with_volume(
            &docker,
            &copy.name,
            "stat -c '%u:%g %a %s' /data/etc/note.txt",
        )
        .await;
        assert_eq!(listing.trim(), "1234:5678 640 6");

        // Into it again: it now holds files, so only with `replace`.
        let (sink, events) = recorder();
        restore(
            docker.clone(),
            file.clone(),
            RestoreSpec {
                create: false,
                ..spec.clone()
            },
            false,
            sink,
        )
        .await;
        let TransferEvent::Error { error } = last(&events) else {
            panic!("restored into a volume holding files");
        };
        assert_eq!(error.code, ErrorCode::VolumeNotEmpty);

        testing::run_with_volume(&docker, &copy.name, "echo stale > /data/stale").await;
        let (sink, events) = recorder();
        let replace = RestoreSpec {
            create: false,
            replace: true,
            ..spec
        };
        restore(docker.clone(), file, replace, false, sink).await;
        assert_eq!(last(&events), TransferEvent::Done { images: vec![] });
        let listing = testing::output_with_volume(&docker, &copy.name, "ls -A /data").await;
        assert_eq!(listing.trim(), "etc");
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine and busybox:1.37"]
    async fn a_running_user_stops_a_restore_but_not_a_backup() {
        let _serial = testing::HELPERS.lock().await;
        let docker = testing::docker();
        let volume = testing::filled_volume(&docker).await;
        let user = testing::container_with_volume(&docker, &volume.name).await;
        docker.start_container(&user.id, None).await.unwrap();
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("backup.tar");

        assert_eq!(running_users(&docker, &volume.name).await.unwrap().len(), 1);
        let (sink, events) = recorder();
        backup(
            docker.clone(),
            volume.name.clone(),
            file.clone(),
            false,
            sink,
        )
        .await;
        assert_eq!(last(&events), TransferEvent::Done { images: vec![] });

        let (sink, events) = recorder();
        let spec = RestoreSpec {
            volume: volume.name.clone(),
            create: false,
            replace: true,
        };
        restore(docker.clone(), file, spec, false, sink).await;
        let TransferEvent::Error { error } = last(&events) else {
            panic!("restored into a volume a running container uses");
        };
        assert_eq!(error.code, ErrorCode::ContainerRunning);
    }
}

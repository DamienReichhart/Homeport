//! Browsing a volume's files in a helper container (ADR 0015).
//!
//! A browse session is a running helper with the volume at [`helper::ROOT`],
//! read-only. Paths are given and shown relative to the volume (`/` is its
//! root); listing and downloading reuse the container file browser.

use std::{collections::HashMap, path::Path, sync::Mutex, time::Duration};

use bollard::Docker;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::helper::{self, Mount};
use crate::{
    containers::files::{self, FileEntry},
    messages as msg,
};

/// How long quitting Homeport waits for its helpers to be removed.
const END_ALL_TIMEOUT: Duration = Duration::from_secs(5);

struct Session {
    docker: Docker,
    /// Homeport pulled the helper's image for this session, so it removes
    /// it afterwards if nothing else uses it.
    pulled_image: bool,
}

/// The open browse sessions, by helper id.
#[derive(Default)]
pub struct BrowseSessions {
    open: Mutex<HashMap<String, Session>>,
}

impl BrowseSessions {
    /// Starts a helper with `volume` read-only and returns the session's id.
    pub async fn start(
        &self,
        docker: &Docker,
        volume: &str,
        pulled_image: bool,
    ) -> AppResult<String> {
        let id = helper::start(
            docker,
            &[Mount {
                volume,
                target: helper::ROOT,
                read_only: true,
            }],
        )
        .await?;
        self.lock().insert(
            id.clone(),
            Session {
                docker: docker.clone(),
                pulled_image,
            },
        );
        Ok(id)
    }

    /// Ends a session: removes its helper, and the image if Homeport pulled it.
    pub async fn end(&self, id: &str) {
        let Some(session) = self.lock().remove(id) else {
            return;
        };
        end(id, session).await;
    }

    /// Ends every session, as Homeport quits; gives up after a few seconds
    /// (an unreachable Host's helpers end on their own within the hour).
    pub async fn end_all(&self) {
        let sessions: Vec<_> = self.lock().drain().collect();
        let ending = futures_util::future::join_all(
            sessions
                .into_iter()
                .map(|(id, session)| async move { end(&id, session).await }),
        );
        if tokio::time::timeout(END_ALL_TIMEOUT, ending).await.is_err() {
            log::warn!("quit before every volume browser's helper was removed");
        }
    }

    /// The helpers of open sessions, which removing leftovers must keep.
    pub fn helper_ids(&self) -> Vec<String> {
        self.lock().keys().cloned().collect()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<String, Session>> {
        self.open.lock().expect("browse sessions lock poisoned")
    }
}

async fn end(id: &str, session: Session) {
    helper::remove(&session.docker, id).await;
    if session.pulled_image {
        helper::remove_image_if_unused(&session.docker).await;
    }
}

/// `path` in the volume as a path in the helper: `/etc` is `/volume/etc`.
fn inside(path: &str) -> AppResult<String> {
    let path = path.trim();
    if !path.starts_with('/') {
        return Err(AppError::validation(msg::FILES_PATH_NOT_ABSOLUTE));
    }
    if path.split('/').any(|segment| segment == "..") {
        return Err(AppError::validation(msg::VOLUME_PATH_OUTSIDE));
    }
    Ok(format!("{}{}", helper::ROOT, path.trim_end_matches('/')))
}

/// A path in the helper as a path in the volume.
fn relative(path: &str) -> String {
    match path.strip_prefix(helper::ROOT) {
        Some("") | None => "/".into(),
        Some(rest) => rest.into(),
    }
}

/// Lists the direct children of `path` in the volume of session `id`.
pub async fn list(docker: &Docker, id: &str, path: &str) -> AppResult<Vec<FileEntry>> {
    let entries = files::list(docker, id, &inside(path)?)
        .await
        .map_err(|e| in_volume(e, path))?;
    Ok(entries
        .into_iter()
        .map(|entry| FileEntry {
            path: relative(&entry.path),
            ..entry
        })
        .collect())
}

/// Saves `path` in the volume to `destination`: a file as itself, a folder
/// as a `.tar`. Returns whether a folder was archived.
pub async fn download(
    docker: &Docker,
    id: &str,
    path: &str,
    destination: &Path,
) -> AppResult<bool> {
    files::download(docker, id, &inside(path)?, destination)
        .await
        .map_err(|e| in_volume(e, path))
}

/// The container browser's words for a missing path name the volume instead.
fn in_volume(error: AppError, path: &str) -> AppError {
    if error.is(ErrorCode::PathNotFound) {
        error.with_message(msg::VOLUME_PATH_MISSING.with("path", path))
    } else {
        error
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing;

    #[test]
    fn paths_are_relative_to_the_volume() {
        assert_eq!(inside("/").unwrap(), "/volume");
        assert_eq!(inside("/etc/").unwrap(), "/volume/etc");
        assert!(inside("etc").is_err());
        assert!(inside("/etc/../../proc").is_err());
        assert_eq!(relative("/volume/etc/note.txt"), "/etc/note.txt");
        assert_eq!(relative("/volume"), "/");
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine and busybox:1.37"]
    async fn a_session_browses_and_downloads_then_leaves_nothing() {
        let _serial = testing::HELPERS.lock().await;
        let docker = testing::docker();
        let volume = testing::filled_volume(&docker).await;
        let sessions = BrowseSessions::default();
        let id = sessions.start(&docker, &volume.name, false).await.unwrap();
        assert_eq!(sessions.helper_ids(), std::slice::from_ref(&id));

        let root = list(&docker, &id, "/").await;
        let missing = list(&docker, &id, "/nope").await;
        let dir = tempfile::tempdir().unwrap();
        let saved = dir.path().join("note.txt");
        let archived = download(&docker, &id, "/etc/note.txt", &saved).await;
        sessions.end_all().await;

        let root = root.unwrap();
        assert_eq!(root.len(), 1);
        assert_eq!(root[0].path, "/etc");
        let missing = missing.unwrap_err();
        assert_eq!(missing.code, ErrorCode::PathNotFound);
        assert_eq!(missing.message, "/nope doesn't exist in the volume.");
        assert!(!archived.unwrap());
        assert_eq!(std::fs::read_to_string(saved).unwrap(), "hello\n");
        assert!(sessions.helper_ids().is_empty());
        assert!(docker.inspect_container(&id, None).await.is_err());
    }
}

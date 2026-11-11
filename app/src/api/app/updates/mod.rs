//! Checks for a new version and, where Homeport can, installs it.
//!
//! What downloads, verifies and replaces Homeport is the updater layer
//! (#255), behind [`Updater`]: it knows the manifest (`updates/latest.json`
//! on the website) and the public key every update must be signed with.
//! What's here is the rest: how this copy gets its updates, what a check
//! answers, when an install is refused and what a failure tells the user.

#[cfg(test)]
mod tests;

use std::{error::Error as StdError, future::Future};

use serde::Serialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_updates::{update_method, Bundle, UpdateMethod};

use crate::messages as msg;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheck {
    pub current_version: String,
    /// The newer version, if there is one.
    pub available: Option<AvailableUpdate>,
    pub method: UpdateMethod,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableUpdate {
    pub version: String,
    /// Markdown release notes.
    pub notes: Option<String>,
    /// RFC 3339 publication date.
    pub date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgress {
    pub downloaded: u64,
    /// Total size, when the server sends it.
    pub total: Option<u64>,
}

/// What went wrong in the updater, as far as the user can act on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateFailureKind {
    /// The update server couldn't be reached, or the download broke.
    Network,
    /// The manifest has no release, or none for this system.
    NonePublished,
    /// The signature is wrong, or signs another version.
    Signature,
    /// The OS asked for an administrator, who refused or cancelled.
    AdminRefused,
    /// Homeport may not write where it's installed.
    NotAllowed,
    /// The download can't be moved over the install: another disk.
    WrongDisk,
    Other,
}

/// A failure of the [`Updater`], with what caused it for the log.
#[derive(Debug)]
pub struct UpdateFailure {
    pub kind: UpdateFailureKind,
    pub cause: Box<dyn StdError + Send + Sync>,
}

/// The updater layer (#255): everything that needs the update server, the
/// signature's public key or the installer.
pub trait Updater {
    /// What the running binary was installed from. A build that isn't a
    /// release bundle (`cargo run`, a debug build) is [`Bundle::Unknown`].
    fn bundle(&self) -> Bundle;

    /// Asks the update server for a newer version, and keeps what it finds
    /// (or that it found nothing) for [`Updater::download_and_install`].
    fn check(&self) -> impl Future<Output = Result<Option<AvailableUpdate>, UpdateFailure>> + Send;

    /// The version the last [`Updater::check`] found, if it found one.
    fn waiting(&self) -> Option<String>;

    /// Downloads the update the last check found, verifies its signature
    /// and installs it. `on_chunk` is given each downloaded chunk's size and
    /// the total size, when the server sends it.
    fn download_and_install(
        &self,
        on_chunk: impl FnMut(usize, Option<u64>) + Send,
    ) -> impl Future<Output = Result<(), UpdateFailure>> + Send;

    /// Restarts Homeport on the version just installed.
    fn restart(&self);
}

/// Checking for, downloading or installing an update failed.
fn update(e: UpdateFailure) -> AppError {
    use UpdateFailureKind as K;
    let error = match e.kind {
        K::Network => AppError::new(ErrorCode::UpdateNetwork, msg::UPDATE_DOWNLOAD)
            .with_hint(msg::UPDATE_DOWNLOAD_HINT),
        K::NonePublished => AppError::new(ErrorCode::UpdateFailed, msg::UPDATE_NONE_PUBLISHED)
            .with_hint(msg::UPDATE_NONE_PUBLISHED_HINT),
        K::Signature => AppError::new(ErrorCode::UpdateSignatureInvalid, msg::UPDATE_SIGNATURE)
            .with_hint(msg::UPDATE_SIGNATURE_HINT),
        K::AdminRefused => {
            AppError::new(ErrorCode::UpdatePermissionDenied, msg::UPDATE_ADMIN_REFUSED)
                .with_hint(msg::UPDATE_ADMIN_REFUSED_HINT)
        }
        K::NotAllowed => AppError::new(ErrorCode::UpdatePermissionDenied, msg::UPDATE_NOT_ALLOWED)
            .with_hint(msg::UPDATE_NOT_ALLOWED_HINT),
        K::WrongDisk => AppError::new(ErrorCode::UpdatePermissionDenied, msg::UPDATE_WRONG_DISK)
            .with_hint(msg::UPDATE_WRONG_DISK_HINT),
        K::Other => AppError::new(ErrorCode::UpdateFailed, msg::UPDATE_FAILED),
    };
    error.with_cause(&*e.cause)
}

fn method(bundle: Bundle) -> UpdateMethod {
    let os_release = std::fs::read_to_string("/etc/os-release").ok();
    let exe = std::env::current_exe().ok();
    update_method(bundle, os_release.as_deref(), exe.as_deref())
}

/// Asks the update server for a newer version and remembers it for [`updates_install`].
pub async fn updates_check(updater: &impl Updater) -> AppResult<UpdateCheck> {
    let available = updater.check().await.map_err(update)?;
    Ok(UpdateCheck {
        current_version: env!("CARGO_PKG_VERSION").to_owned(),
        available,
        method: method(updater.bundle()),
    })
}

/// Downloads the update found by the last [`updates_check`], verifies its
/// signature, installs it and restarts Homeport. Only for installs Homeport can
/// update itself; package-managed installs are refused.
pub async fn updates_install(
    updater: &impl Updater,
    on_progress: impl Fn(UpdateProgress) -> bool + Send + Sync + 'static,
) -> AppResult<()> {
    if method(updater.bundle()) != UpdateMethod::InApp {
        return Err(AppError::validation(msg::UPDATE_BY_PACKAGE_MANAGER));
    }
    let Some(version) = updater.waiting() else {
        return Err(AppError::not_found(msg::UPDATE_NOTHING_WAITING));
    };
    let mut downloaded = 0u64;
    updater
        .download_and_install(|chunk, total| {
            downloaded += chunk as u64;
            // A closed window stops listening; the install carries on regardless.
            let _ = on_progress(UpdateProgress { downloaded, total });
        })
        .await
        .map_err(update)?;
    log::info!("installed Homeport {version}, restarting");
    updater.restart();
    Ok(())
}

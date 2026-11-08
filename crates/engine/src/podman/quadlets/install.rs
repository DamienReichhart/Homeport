//! Installing, editing and removing units through Podman's API (5.8 and
//! later), which writes the files where Podman's user keeps them and
//! reloads systemd itself.

use serde::Deserialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::http::{self, Request};

use super::{gone, too_old, Podman, FILES_SINCE};
use crate::messages as msg;

/// A unit's extensions, each with the section its file must have.
pub const KINDS: [(&str, &str); 8] = [
    ("container", "Container"),
    ("pod", "Pod"),
    ("volume", "Volume"),
    ("network", "Network"),
    ("kube", "Kube"),
    ("image", "Image"),
    ("build", "Build"),
    ("artifact", "Artifact"),
];

/// A file to install: its name, and its text.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct QuadletFile {
    pub name: String,
    pub content: String,
}

/// A unit to install or edit, with the files that go beside it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct QuadletInstall {
    /// The unit file, such as `web.container`.
    pub unit: QuadletFile,
    /// Files installed beside it: a Containerfile, kube YAML, an env file.
    #[serde(default)]
    pub extra: Vec<QuadletFile>,
    /// The application folder to install into. With extra files and none
    /// given, the unit's name without its extension.
    #[serde(default)]
    pub app: Option<String>,
}

/// The section a unit called `name` must have, or why the name won't do.
pub(crate) fn check_name(name: &str) -> AppResult<&'static str> {
    let invalid = || AppError::validation(msg::QUADLET_NAME.with("name", name));
    let (stem, extension) = name.rsplit_once('.').ok_or_else(invalid)?;
    let section = KINDS
        .iter()
        .find(|(ext, _)| *ext == extension)
        .map(|(_, section)| *section)
        .ok_or_else(invalid)?;
    let mut chars = stem.chars();
    let valid = chars.next().is_some_and(|c| c.is_ascii_alphanumeric())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-' | '@'));
    if valid {
        Ok(section)
    } else {
        Err(invalid())
    }
}

/// A file name to put beside a unit: one plain name, no folders.
fn check_extra_name(name: &str) -> AppResult<()> {
    let plain = !name.is_empty()
        && name != "."
        && name != ".."
        && !name.contains(['/', '\\', '\0'])
        && !name.starts_with('-');
    if plain {
        Ok(())
    } else {
        Err(AppError::validation(
            msg::QUADLET_EXTRA_NAME.with("name", name),
        ))
    }
}

/// Checks the files and returns the application folder to install into.
pub fn validate(install: &QuadletInstall) -> AppResult<Option<String>> {
    let unit = &install.unit;
    let section = check_name(&unit.name)?;
    let header = format!("[{section}]");
    if !unit.content.lines().any(|l| l.trim() == header) {
        return Err(AppError::new(
            ErrorCode::QuadletInvalid,
            msg::QUADLET_SECTION_MISSING
                .with("name", unit.name.as_str())
                .with("section", header.as_str()),
        )
        .with_hint(msg::QUADLET_SECTION_MISSING_HINT.with("section", header.as_str())));
    }
    let mut seen = vec![unit.name.as_str()];
    for file in &install.extra {
        check_extra_name(&file.name)?;
        if seen.contains(&file.name.as_str()) {
            return Err(AppError::validation(
                msg::QUADLET_EXTRA_TWICE.with("name", file.name.as_str()),
            ));
        }
        seen.push(&file.name);
    }
    let app = install
        .app
        .as_deref()
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .map(str::to_owned)
        .or_else(|| {
            (!install.extra.is_empty())
                .then(|| unit.name.rsplit_once('.').map(|(s, _)| s.to_owned()))
                .flatten()
        });
    if let Some(app) = &app {
        let valid = !app.starts_with('-')
            && app != "."
            && app != ".."
            && app
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-' | '@'));
        if !valid {
            return Err(AppError::validation(
                msg::QUADLET_APP.with("app", app.as_str()),
            ));
        }
    }
    Ok(app)
}

/// The files as the tar archive Podman's install endpoint reads, the unit first.
pub fn tar_of(install: &QuadletInstall) -> AppResult<Vec<u8>> {
    let mut archive = tar::Builder::new(Vec::new());
    for file in std::iter::once(&install.unit).chain(&install.extra) {
        let mut header = tar::Header::new_gnu();
        header.set_size(file.content.len() as u64);
        header.set_mode(0o644);
        header.set_mtime(0);
        header.set_entry_type(tar::EntryType::Regular);
        archive.append_data(&mut header, &file.name, file.content.as_bytes())?;
    }
    Ok(archive.into_inner()?)
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Installed {
    #[serde(default)]
    installed_quadlets: Option<std::collections::BTreeMap<String, String>>,
}

/// Installs a unit and its files, and has Podman reload systemd. With
/// `replace`, the files already there under those names are overwritten
/// (Edit); others in the application folder stay. Returns where the files went.
pub async fn install(
    podman: &Podman,
    install: &QuadletInstall,
    replace: bool,
) -> AppResult<Vec<String>> {
    if !podman.at_least(FILES_SINCE) {
        return Err(too_old(podman, FILES_SINCE));
    }
    let app = validate(install)?;
    let mut path = format!("/quadlets?replace={replace}&reload-systemd=true");
    if let Some(app) = &app {
        path.push_str(&format!("&application={}", http::encode(app)));
    }
    let request = Request::post(podman.path(&path)).body("application/x-tar", tar_of(install)?);
    let answer = podman.send(request).await?;
    if !answer.status.is_success() {
        return Err(install_error(
            &install.unit.name,
            answer.status,
            &answer.body,
        ));
    }
    let installed: Installed = answer.json()?;
    Ok(installed
        .installed_quadlets
        .unwrap_or_default()
        .into_values()
        .collect())
}

/// Why Podman refused to install `name`, in the words of what to do.
fn install_error(name: &str, status: http::StatusCode, body: &[u8]) -> AppError {
    let raw = http::status_error(status, body);
    let detail = raw.detail.clone().unwrap_or_default();
    let text = detail.to_lowercase();
    if text.contains("already exists") {
        AppError::new(
            ErrorCode::QuadletExists,
            msg::QUADLET_EXISTS.with("name", name),
        )
        .with_hint(msg::QUADLET_EXISTS_HINT)
        .with_detail(detail)
    } else if text.contains("dbus") || text.contains("systemd") {
        AppError::new(ErrorCode::SystemdRefused, msg::QUADLET_NO_SYSTEMD)
            .with_hint(msg::QUADLET_NO_SYSTEMD_HINT)
            .with_detail(detail)
    } else if status.as_u16() == 400 {
        AppError::new(
            ErrorCode::QuadletInvalid,
            msg::QUADLET_REFUSED.with("name", name),
        )
        .with_hint(msg::QUADLET_REFUSED_HINT)
        .with_detail(detail)
    } else {
        raw
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Removed {
    #[serde(default)]
    errors: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

/// Removes a unit's file and has Podman reload systemd; `force` stops its
/// service first.
pub async fn remove(podman: &Podman, name: &str, force: bool) -> AppResult<()> {
    if !podman.at_least(FILES_SINCE) {
        return Err(too_old(podman, FILES_SINCE));
    }
    check_name(name)?;
    let path = podman.path(&format!(
        "/quadlets/{}?force={force}&reload-systemd=true",
        http::encode(name)
    ));
    let answer = podman.send(Request::delete(path)).await?;
    let detail = String::from_utf8_lossy(&answer.body).into_owned();
    match answer.status.as_u16() {
        200..=299 => {
            let removed: Removed = answer.json()?;
            if removed.errors.is_some_and(|e| e.contains_key(name)) {
                return Err(AppError::new(
                    ErrorCode::EngineError,
                    msg::QUADLET_NOT_REMOVED.with("name", name),
                )
                .with_hint(msg::QUADLET_NOT_REMOVED_HINT)
                .with_detail(detail));
            }
            Ok(())
        }
        404 => Err(gone(name).with_detail(detail)),
        400 if detail.contains("is running") => Err(AppError::new(
            ErrorCode::Conflict,
            msg::QUADLET_RUNNING.with("name", name),
        )
        .with_hint(msg::QUADLET_RUNNING_HINT)
        .with_detail(detail)),
        _ => Err(http::status_error(answer.status, &answer.body)),
    }
}

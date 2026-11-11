//! Quadlet units: the files Podman's systemd generator turns into services
//! (ADR 0019). Listed through Podman's API from 5.7, or from their
//! containers' `PODMAN_SYSTEMD_UNIT` label before; read, installed and
//! removed through the API from 5.8; started, stopped and restarted with
//! `systemctl` where the Host's dialer runs commands.

mod act;
mod install;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::http::{self, Request};

use super::Podman;
use crate::{containers::Container, messages as msg};

pub use act::{act, unit_command, UnitAction};
pub use install::{install, remove, tar_of, validate, QuadletFile, QuadletInstall, KINDS};

/// The label Podman puts on every container a Quadlet unit created.
pub const UNIT_LABEL: &str = "PODMAN_SYSTEMD_UNIT";

/// The first Podman that lists Quadlet units through its API.
pub const LIST_SINCE: (u32, u32) = (5, 7);
/// The first Podman that reads, installs and removes them through its API.
pub const FILES_SINCE: (u32, u32) = (5, 8);

/// Whose systemd runs a unit: the system's for rootful Podman, the user's
/// (`systemctl --user`) for rootless.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum UnitScope {
    System,
    User,
}

/// A container a unit created.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct QuadletContainer {
    pub id: String,
    pub name: String,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct QuadletUnit {
    /// The unit file's name, `web.container`; `None` when the unit is only
    /// known from its containers' label.
    pub name: Option<String>,
    /// The systemd service it generates, `web.service`; `None` until
    /// systemd has been reloaded since it was installed.
    pub unit_name: Option<String>,
    /// Where the file is on the Host.
    pub path: Option<String>,
    /// systemd's state (`active/running`, `inactive/dead`, `failed/failed`),
    /// Podman's `Not loaded` or `loaded template`, or for a unit known from
    /// its containers, theirs (`running`, `exited`).
    pub status: String,
    /// The application it was installed with, beside its extra files.
    pub app: Option<String>,
    /// The pod unit a `.container` unit names in `Pod=`.
    pub pod: Option<String>,
    pub scope: UnitScope,
    pub containers: Vec<QuadletContainer>,
}

/// Where a Host's units were read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum QuadletSource {
    /// Podman's API, with every unit and its file's path.
    Api,
    /// Only the label of the containers units created, on Podman before 5.7.
    Labels,
}

/// A Podman Host's Quadlet units, and what it can do with them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct QuadletUnits {
    pub units: Vec<QuadletUnit>,
    pub source: QuadletSource,
    pub podman_version: String,
    /// Podman 5.8 and later read, install, edit and remove unit files.
    pub manages_files: bool,
    /// The Host's connection can run `systemctl`: SSH, or a local Linux socket.
    pub runs_systemctl: bool,
}

/// One unit of `GET /libpod/quadlets/json`.
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Listed {
    name: String,
    #[serde(default)]
    unit_name: String,
    #[serde(default)]
    path: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    app: String,
    #[serde(default)]
    pod: String,
}

#[derive(Deserialize)]
struct Info {
    host: Option<InfoHost>,
}

#[derive(Deserialize)]
struct InfoHost {
    security: Option<Security>,
}

#[derive(Deserialize)]
struct Security {
    #[serde(default)]
    rootless: bool,
}

/// Whether the Host's Podman runs rootless, so its units are the user's.
async fn rootless(podman: &Podman) -> bool {
    let info: AppResult<Info> = podman.json(Request::get(podman.path("/info"))).await;
    match info {
        Ok(info) => info
            .host
            .and_then(|h| h.security)
            .is_some_and(|s| s.rootless),
        Err(e) => {
            log::debug!("couldn't read whether Podman is rootless: {e}");
            false
        }
    }
}

/// Whose unit the file at `path` is, when its folder says; else `rootless`'s.
pub fn scope_of(path: Option<&str>, rootless: bool) -> UnitScope {
    match path {
        Some(p)
            if p.contains("/.config/containers/systemd/")
                || p.contains("/containers/systemd/users/")
                || p.starts_with("/run/user/") =>
        {
            UnitScope::User
        }
        Some(p)
            if p.starts_with("/etc/containers/systemd/")
                || p.starts_with("/usr/share/containers/systemd/")
                || p.starts_with("/run/containers/systemd/") =>
        {
            UnitScope::System
        }
        _ if rootless => UnitScope::User,
        _ => UnitScope::System,
    }
}

/// `containers` grouped by the unit their label names, in name order.
fn by_unit(containers: &[Container]) -> BTreeMap<String, Vec<QuadletContainer>> {
    let mut units: BTreeMap<String, Vec<QuadletContainer>> = BTreeMap::new();
    for c in containers {
        if let Some(unit) = c.labels.get(UNIT_LABEL).filter(|u| !u.is_empty()) {
            units
                .entry(unit.clone())
                .or_default()
                .push(QuadletContainer {
                    id: c.id.clone(),
                    name: c.name.clone(),
                    state: c.state.clone(),
                });
        }
    }
    units
}

/// The state of a unit known only from its containers: running while one runs.
fn state_of(containers: &[QuadletContainer]) -> String {
    if containers.iter().any(|c| c.state == "running") {
        "running".into()
    } else {
        containers
            .first()
            .map_or_else(|| "exited".into(), |c| c.state.clone())
    }
}

/// Puts each unit's containers with it, and keeps units only their
/// containers name (made before the API, or by hand) at the end.
fn merge(listed: Vec<Listed>, containers: &[Container], rootless: bool) -> Vec<QuadletUnit> {
    let mut grouped = by_unit(containers);
    let mut units: Vec<QuadletUnit> = listed
        .into_iter()
        .map(|q| {
            let unit_name = some(q.unit_name);
            let path = some(q.path);
            let containers = unit_name
                .as_ref()
                .and_then(|u| grouped.remove(u))
                .unwrap_or_default();
            QuadletUnit {
                scope: scope_of(path.as_deref(), rootless),
                name: some(q.name),
                unit_name,
                path,
                status: q.status,
                app: some(q.app),
                pod: some(q.pod),
                containers,
            }
        })
        .collect();
    units.sort_by(|a, b| a.name.cmp(&b.name));
    units.extend(grouped.into_iter().map(|(unit, containers)| QuadletUnit {
        name: None,
        status: state_of(&containers),
        unit_name: Some(unit),
        path: None,
        app: None,
        pod: None,
        scope: scope_of(None, rootless),
        containers,
    }));
    units
}

fn some(s: String) -> Option<String> {
    (!s.is_empty()).then_some(s)
}

/// The Host's units: from Podman's API on 5.7 and later, else from the
/// containers they created. `containers` is the Host's container list.
pub async fn list(podman: &Podman, containers: &[Container]) -> AppResult<QuadletUnits> {
    let rootless = rootless(podman).await;
    let (listed, source) = if podman.at_least(LIST_SINCE) {
        let listed: Option<Vec<Listed>> = podman
            .json(Request::get(podman.path("/quadlets/json")))
            .await?;
        (listed.unwrap_or_default(), QuadletSource::Api)
    } else {
        (vec![], QuadletSource::Labels)
    };
    Ok(QuadletUnits {
        units: merge(listed, containers, rootless),
        source,
        podman_version: podman.version().to_owned(),
        manages_files: podman.at_least(FILES_SINCE),
        runs_systemctl: podman.runs_commands(),
    })
}

/// What a Podman too old for `needed` can't do, and which version can.
pub fn too_old(podman: &Podman, needed: (u32, u32)) -> AppError {
    let needed = format!("{}.{}", needed.0, needed.1);
    AppError::new(
        ErrorCode::PodmanTooOld,
        msg::PODMAN_TOO_OLD
            .with("needed", needed.as_str())
            .with("version", podman.version()),
    )
    .with_hint(msg::PODMAN_TOO_OLD_HINT.with("needed", needed.as_str()))
}

fn gone(name: &str) -> AppError {
    AppError::not_found(msg::QUADLET_GONE.with("name", name)).with_hint(msg::QUADLET_GONE_HINT)
}

/// The unit file `name`, as Podman reads it from the Host (5.8 and later).
pub async fn file(podman: &Podman, name: &str) -> AppResult<String> {
    if !podman.at_least(FILES_SINCE) {
        return Err(too_old(podman, FILES_SINCE));
    }
    install::check_name(name)?;
    let path = podman.path(&format!("/quadlets/{}/file", http::encode(name)));
    let answer = podman.send(Request::get(path)).await?;
    if answer.status.as_u16() == 404 {
        return Err(gone(name).with_detail(String::from_utf8_lossy(&answer.body)));
    }
    let answer = answer.ok()?;
    Ok(String::from_utf8_lossy(&answer.body).into_owned())
}

#[cfg(all(test, unix))]
mod tests;

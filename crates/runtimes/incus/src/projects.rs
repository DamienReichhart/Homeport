//! Incus's projects, which scope Instances, images, profiles, volumes and
//! networks. The frontend remembers the one each Host shows.

use serde::{Deserialize, Serialize};

use homeport_core::error::AppResult;
use homeport_transport::http::Request;

use crate::api::client::Incus;

/// A project, with how many Instances and images it holds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusProject {
    pub name: String,
    pub description: String,
    pub instances: u32,
    pub images: u32,
}

#[derive(Deserialize)]
struct RawProject {
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    used_by: Option<Vec<String>>,
}

/// How many of `used_by` are under `/1.0/<collection>/`.
pub(crate) fn count(used_by: &[String], collection: &str) -> u32 {
    let prefix = format!("/1.0/{collection}/");
    let n = used_by
        .iter()
        .filter(|path| {
            path.strip_prefix(&prefix)
                .is_some_and(|rest| !rest.contains("/snapshots/"))
        })
        .count();
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// The projects the connection may use: every one on the admin socket, the
/// user's own on the restricted socket.
pub async fn list(incus: &Incus) -> AppResult<Vec<IncusProject>> {
    let raw: Vec<RawProject> = incus
        .sync(Request::get("/1.0/projects?recursion=1"))
        .await?;
    let mut projects: Vec<IncusProject> = raw
        .into_iter()
        .map(|p| {
            let used_by = p.used_by.unwrap_or_default();
            IncusProject {
                instances: count(&used_by, "instances"),
                images: count(&used_by, "images"),
                name: p.name,
                description: p.description,
            }
        })
        .collect();
    projects.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(projects)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn used_by_counts_objects_not_their_snapshots() {
        let used_by: Vec<String> = [
            "/1.0/instances/web",
            "/1.0/instances/db?project=lab",
            "/1.0/instances/web/snapshots/snap0",
            "/1.0/images/2c5e",
            "/1.0/profiles/default",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(count(&used_by, "instances"), 2);
        assert_eq!(count(&used_by, "images"), 1);
        assert_eq!(count(&used_by, "networks"), 0);
    }
}

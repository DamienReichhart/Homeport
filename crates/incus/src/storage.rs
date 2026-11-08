//! Incus's storage pools, with how full they are, and their volumes;
//! custom volumes can be created and deleted, everything else is read.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::http::Request;

use crate::{
    client::{scoped, segment, Incus},
    messages as msg,
    usage::{self, IncusUser},
};

/// One storage pool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusStoragePool {
    pub name: String,
    pub description: String,
    /// `dir`, `zfs`, `btrfs`, `lvm`, `ceph`…
    pub driver: String,
    /// `Created`, or what went wrong with it.
    pub status: String,
    /// Bytes used and in all, when the driver can tell.
    pub used: Option<i64>,
    pub total: Option<i64>,
    pub used_by: Vec<IncusUser>,
}

/// One volume of a pool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusVolume {
    pub name: String,
    /// `container`, `virtual-machine`, `image` or `custom`.
    #[serde(rename = "type")]
    pub kind: String,
    /// `filesystem`, `block` or `iso`.
    pub content_type: String,
    pub description: String,
    /// Its size limit, as configured: "10GiB".
    pub size: Option<String>,
    pub created_at: Option<String>,
    pub used_by: Vec<IncusUser>,
}

#[derive(Deserialize)]
struct RawPool {
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    driver: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    used_by: Option<Vec<String>>,
}

#[derive(Deserialize, Default)]
struct Resources {
    #[serde(default)]
    space: Space,
}

#[derive(Deserialize, Default)]
struct Space {
    #[serde(default)]
    used: Option<i64>,
    #[serde(default)]
    total: Option<i64>,
}

#[derive(Deserialize)]
struct RawVolume {
    name: String,
    #[serde(rename = "type", default)]
    kind: String,
    #[serde(default)]
    content_type: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    config: BTreeMap<String, String>,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    used_by: Option<Vec<String>>,
}

impl RawVolume {
    fn into_volume(self) -> IncusVolume {
        IncusVolume {
            used_by: usage::users(&self.used_by.unwrap_or_default()),
            size: self.config.get("size").cloned(),
            created_at: (!self.created_at.is_empty() && !self.created_at.starts_with("0001-"))
                .then_some(self.created_at),
            name: self.name,
            kind: self.kind,
            content_type: self.content_type,
            description: self.description,
        }
    }
}

fn pool_path(pool: &str) -> String {
    format!("/1.0/storage-pools/{}", segment(pool))
}

/// Every storage pool, with its space when the driver can tell.
pub async fn pools(incus: &Incus, project: &str) -> AppResult<Vec<IncusStoragePool>> {
    let raw: Vec<RawPool> = incus
        .sync(Request::get(scoped(
            "/1.0/storage-pools?recursion=1",
            project,
        )))
        .await?;
    let mut pools = Vec::with_capacity(raw.len());
    for pool in raw {
        let resources: Resources = incus
            .sync(Request::get(format!("{}/resources", pool_path(&pool.name))))
            .await
            .unwrap_or_default();
        pools.push(IncusStoragePool {
            used_by: usage::users(&pool.used_by.unwrap_or_default()),
            used: resources.space.used,
            total: resources.space.total.filter(|t| *t > 0),
            name: pool.name,
            description: pool.description,
            driver: pool.driver,
            status: pool.status,
        });
    }
    pools.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(pools)
}

/// The volumes of `pool` in `project`: Instances' and images' own, and custom ones.
pub async fn volumes(incus: &Incus, project: &str, pool: &str) -> AppResult<Vec<IncusVolume>> {
    let raw: Vec<RawVolume> = incus
        .sync(Request::get(scoped(
            &format!("{}/volumes?recursion=1", pool_path(pool)),
            project,
        )))
        .await?;
    let mut volumes: Vec<IncusVolume> = raw.into_iter().map(RawVolume::into_volume).collect();
    volumes.sort_by(|a, b| (&a.kind, &a.name).cmp(&(&b.kind, &b.name)));
    Ok(volumes)
}

/// What the New volume dialog asks for.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct VolumeRequest {
    pub name: String,
    /// "10GiB"; the pool's default when `None`.
    pub size: Option<String>,
    /// `filesystem` or `block`.
    pub content_type: String,
}

/// The body of `POST …/volumes/custom` for `request`.
pub(crate) fn volume_body(request: &VolumeRequest) -> serde_json::Value {
    let mut config = serde_json::Map::new();
    if let Some(size) = request
        .size
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        config.insert("size".into(), size.replace(' ', "").into());
    }
    serde_json::json!({
        "name": request.name.trim(),
        "content_type": request.content_type,
        "config": config,
    })
}

/// Makes a custom volume in `pool`.
pub async fn create_volume(
    incus: &Incus,
    project: &str,
    pool: &str,
    request: &VolumeRequest,
) -> AppResult<()> {
    let name = request.name.trim();
    if name.is_empty() || name.contains('/') {
        return Err(AppError::validation(
            msg::VOLUME_NAME_INVALID.with("name", name),
        ));
    }
    let path = scoped(&format!("{}/volumes/custom", pool_path(pool)), project);
    incus
        .run(
            project,
            Request::post(path).json(&volume_body(request))?,
            crate::operations::LONG,
        )
        .await
        .map(drop)
}

/// Deletes the custom volume `name` of `pool`; one in use is refused,
/// naming its users.
pub async fn delete_volume(incus: &Incus, project: &str, pool: &str, name: &str) -> AppResult<()> {
    let path = scoped(
        &format!("{}/volumes/custom/{}", pool_path(pool), segment(name)),
        project,
    );
    let volume: RawVolume = incus.sync(Request::get(path.clone())).await?;
    let users = usage::users(&volume.used_by.unwrap_or_default());
    if !users.is_empty() {
        return Err(AppError::new(
            ErrorCode::InUse,
            msg::IN_USE
                .with("name", name)
                .with("users", usage::names(&users)),
        )
        .with_hint(msg::VOLUME_IN_USE_HINT));
    }
    incus
        .run(project, Request::delete(path), crate::operations::LONG)
        .await
        .map(drop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volumes_read_with_their_size_and_users() {
        let raw: RawVolume = serde_json::from_value(serde_json::json!({
            "name": "data",
            "type": "custom",
            "content_type": "filesystem",
            "config": { "size": "10GiB" },
            "created_at": "2026-11-08T23:01:22Z",
            "used_by": ["/1.0/instances/web"]
        }))
        .unwrap();
        let volume = raw.into_volume();
        assert_eq!(volume.size.as_deref(), Some("10GiB"));
        assert_eq!(volume.used_by[0].name, "web");
    }

    #[test]
    fn a_new_volume_names_its_content_and_size() {
        let request = VolumeRequest {
            name: " data ".into(),
            size: Some("10 GiB".into()),
            content_type: "block".into(),
        };
        assert_eq!(
            volume_body(&request),
            serde_json::json!({ "name": "data", "content_type": "block", "config": { "size": "10GiB" } })
        );
    }

    #[tokio::test]
    #[ignore = "needs a local Incus"]
    async fn incus_live_custom_volumes_are_created_and_deleted() {
        let incus = crate::instances::tests::live().await;
        let pools = pools(&incus, "default").await.unwrap();
        let pool = &pools[0];
        assert!(pool.total.is_some_and(|t| t > 0), "{pool:?}");
        let name = format!("homeport-test-{}", std::process::id());
        let request = VolumeRequest {
            name: name.clone(),
            size: None,
            content_type: "filesystem".into(),
        };
        create_volume(&incus, "default", &pool.name, &request)
            .await
            .unwrap();
        let again = create_volume(&incus, "default", &pool.name, &request)
            .await
            .unwrap_err();
        assert_eq!(again.code, ErrorCode::NameInUse, "{again:?}");
        let listed = volumes(&incus, "default", &pool.name).await.unwrap();
        assert!(listed.iter().any(|v| v.name == name && v.kind == "custom"));
        delete_volume(&incus, "default", &pool.name, &name)
            .await
            .unwrap();
    }
}

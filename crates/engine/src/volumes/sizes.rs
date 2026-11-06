//! How much disk each volume uses, as `docker system df -v` reports it.
//!
//! The Engine walks every volume to answer, which takes seconds on a large
//! Host, so this is asked for apart from the volume list.

use std::collections::HashMap;

use bollard::{
    models::Volume,
    query_parameters::{DataUsageOptions, ListVolumesOptions},
    BollardRequest, Docker,
};

use homeport_core::error::AppResult;

/// The first API version answering `/system/df` with `VolumeUsage.Items`
/// (Docker 29). Older Engines and Podman answer with a top-level `Volumes`.
const ITEMS_SINCE: (usize, usize) = (1, 52);

/// Each volume's size in bytes by name; `None` when its driver doesn't
/// report one (every driver but `local`).
pub async fn sizes(docker: &Docker) -> AppResult<HashMap<String, Option<u64>>> {
    let version = docker.client_version();
    let version = (version.major_version, version.minor_version);
    // The request is rewritten: bollard can't encode the `type` list, and it
    // leaves the API version out of the path, so the Engine would answer in
    // the shape of its own latest version rather than the one read here.
    let volumes_only = docker
        .clone()
        .with_request_modifier(move |req| to_df(req, version));
    if version >= ITEMS_SINCE {
        let items = volumes_only
            .df(None::<DataUsageOptions>)
            .await?
            .volume_usage
            .and_then(|u| u.items)
            .unwrap_or_default();
        return Ok(from_items(items));
    }
    // bollard reads only the newer shape and drops the older `Volumes` array,
    // which has the volume list's shape: ask for the list, sent to df instead.
    let volumes = volumes_only
        .list_volumes(None::<ListVolumesOptions>)
        .await?
        .volumes
        .unwrap_or_default();
    Ok(from_volumes(volumes))
}

/// Turns a disk usage or volume list request into a disk usage request for
/// volumes only, with each volume listed.
fn to_df(mut req: BollardRequest, version: (usize, usize)) -> BollardRequest {
    if let Ok(uri) = df_uri(&req.uri().to_string(), version).parse() {
        *req.uri_mut() = uri;
    }
    req
}

/// `…/volumes?…` or `…/system/df?…` as
/// `…/v<version>/system/df?type=volume&verbose=1`. Engines before API 1.42
/// ignore `type` and add the other kinds; before 1.52, `verbose` is ignored
/// and every volume is listed anyway.
fn df_uri(uri: &str, (major, minor): (usize, usize)) -> String {
    let path = uri.split_once('?').map_or(uri, |(path, _)| path);
    let Some(base) = path
        .strip_suffix("/volumes")
        .or_else(|| path.strip_suffix("/system/df"))
    else {
        return uri.to_owned();
    };
    // A version bollard put in the path is replaced by the one read here.
    let base = match base.rsplit_once('/') {
        Some((root, last)) if is_version(last) => root,
        _ => base,
    };
    format!("{base}/v{major}.{minor}/system/df?type=volume&verbose=1")
}

fn is_version(segment: &str) -> bool {
    segment.strip_prefix('v').is_some_and(|v| {
        v.split_once('.').is_some_and(|(a, b)| {
            !a.is_empty()
                && !b.is_empty()
                && a.bytes().all(|c| c.is_ascii_digit())
                && b.bytes().all(|c| c.is_ascii_digit())
        })
    })
}

fn from_items(items: Vec<serde_json::Value>) -> HashMap<String, Option<u64>> {
    from_volumes(
        items
            .into_iter()
            .filter_map(|item| serde_json::from_value::<Volume>(item).ok())
            .collect(),
    )
}

fn from_volumes(volumes: Vec<Volume>) -> HashMap<String, Option<u64>> {
    volumes
        .into_iter()
        .map(|v| {
            // `-1` means the driver doesn't say.
            let size = v.usage_data.and_then(|u| u64::try_from(u.size).ok());
            (v.name, size)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use bollard::models::{SystemDataUsageResponse, VolumeListResponse};

    use super::*;
    use crate::testing;

    /// `/system/df?type=volume&verbose=1` from Docker 29 (API 1.52 and later).
    const ITEMS: &str = r#"{"VolumeUsage":{"Items":[
        {"CreatedAt":"2026-11-06T08:42:18Z","Driver":"local","Labels":null,"Mountpoint":"/var/lib/docker/volumes/db/_data","Name":"db","Options":null,"Scope":"local","UsageData":{"RefCount":1,"Size":300000}},
        {"CreatedAt":"2026-11-06T08:42:18Z","Driver":"rclone","Labels":{"a":"b"},"Mountpoint":"","Name":"remote","Options":{},"Scope":"local","UsageData":{"RefCount":0,"Size":-1}}
    ],"Reclaimable":0,"TotalCount":2,"TotalSize":300000}}"#;

    /// `/system/df?type=volume` from Docker before 29, and Podman's compat API.
    const VOLUMES: &str = r#"{"Volumes":[
        {"CreatedAt":"2026-11-06T08:42:18Z","Driver":"local","Labels":null,"Mountpoint":"/var/lib/docker/volumes/db/_data","Name":"db","Options":null,"Scope":"local","UsageData":{"RefCount":1,"Size":300000}},
        {"CreatedAt":"2026-11-06T08:42:18Z","Driver":"rclone","Labels":{},"Mountpoint":"","Name":"remote","Options":{},"Scope":"local","UsageData":{"RefCount":0,"Size":-1}},
        {"Driver":"local","Labels":{},"Mountpoint":"/x","Name":"unmeasured","Options":{},"Scope":"local"}
    ]}"#;

    #[test]
    fn reads_the_newer_shape() {
        let df: SystemDataUsageResponse = serde_json::from_str(ITEMS).unwrap();
        let sizes = from_items(df.volume_usage.unwrap().items.unwrap());
        assert_eq!(
            sizes,
            HashMap::from([("db".into(), Some(300_000)), ("remote".into(), None)])
        );
    }

    #[test]
    fn reads_the_older_shape_through_the_volume_list() {
        let list: VolumeListResponse = serde_json::from_str(VOLUMES).unwrap();
        let sizes = from_volumes(list.volumes.unwrap());
        assert_eq!(
            sizes,
            HashMap::from([
                ("db".into(), Some(300_000)),
                ("remote".into(), None),
                ("unmeasured".into(), None),
            ])
        );
    }

    #[test]
    fn list_requests_become_disk_usage_requests() {
        assert_eq!(
            df_uri("unix://2f72756e/volumes", (1, 41)),
            "unix://2f72756e/v1.41/system/df?type=volume&verbose=1"
        );
        assert_eq!(
            df_uri("http://docker/v1.53/system/df?verbose=false", (1, 52)),
            "http://docker/v1.52/system/df?type=volume&verbose=1"
        );
        assert_eq!(
            df_uri("/volumes", (1, 47)),
            "/v1.47/system/df?type=volume&verbose=1"
        );
        assert_eq!(df_uri("/v1.41/info", (1, 41)), "/v1.41/info");
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn a_volume_reports_at_least_what_it_holds() {
        // Negotiated, as every Host's client is: the request names its version.
        let docker = testing::docker().negotiate_version().await.unwrap();
        let volume = testing::filled_volume(&docker).await;
        let volume = &volume.name;
        testing::run_with_volume(&docker, volume, "head -c 200000 /dev/zero > /data/f").await;

        let size = sizes(&docker).await.unwrap()[volume];
        // And in the older shape, as Podman and Docker before 29 answer.
        let older = sizes(&testing::docker_at_api(1, 47)).await.unwrap()[volume];
        assert!(size.unwrap() >= 200_000, "{size:?}");
        assert!(older.unwrap() >= 200_000, "{older:?}");
    }
}

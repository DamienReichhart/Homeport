//! Creating an Instance from an image: the request Incus takes, and following
//! the operation while the image downloads.

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    pull::PullEvent,
};
use homeport_transport::http::Request;

use super::{
    act,
    model::{InstanceAction, InstanceSource, InstanceSpec},
};
use crate::{
    api::{
        client::{scoped, Incus},
        operations,
    },
    messages as msg,
};

/// Incus's rule for Instance names: a hostname label.
pub fn validate_name(name: &str) -> AppResult<()> {
    let valid = !name.is_empty()
        && name.len() <= 63
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.starts_with(|c: char| c.is_ascii_digit());
    if valid {
        Ok(())
    } else {
        Err(AppError::validation(
            msg::INSTANCE_NAME_INVALID.with("name", name),
        ))
    }
}

/// The body of `POST /1.0/instances` for `spec`; `root_pool` is the pool
/// of the root disk its profiles give it, needed to resize that disk.
pub(crate) fn create_body(
    spec: &InstanceSpec,
    root_pool: Option<&str>,
    start: bool,
) -> serde_json::Value {
    let source = match &spec.source {
        InstanceSource::Local { fingerprint } => serde_json::json!({
            "type": "image",
            "fingerprint": fingerprint,
        }),
        InstanceSource::Remote { server, alias } => serde_json::json!({
            "type": "image",
            "mode": "pull",
            "server": server,
            "protocol": "simplestreams",
            "alias": alias,
        }),
    };
    let mut config = serde_json::Map::new();
    if let Some(cpu) = spec.cpu {
        config.insert("limits.cpu".into(), cpu.to_string().into());
    }
    if let Some(memory) = spec
        .memory
        .as_deref()
        .map(str::trim)
        .filter(|m| !m.is_empty())
    {
        config.insert("limits.memory".into(), memory.into());
    }
    let mut devices = serde_json::Map::new();
    if let (Some(size), Some(pool)) = (
        spec.disk
            .as_deref()
            .map(str::trim)
            .filter(|d| !d.is_empty()),
        root_pool,
    ) {
        devices.insert(
            "root".into(),
            serde_json::json!({ "type": "disk", "path": "/", "pool": pool, "size": size }),
        );
    }
    let mut body = serde_json::json!({
        "name": spec.name,
        "type": crate::images::instance_type(spec.kind),
        "profiles": spec.profiles,
        "config": config,
        "devices": devices,
        "source": source,
    });
    if start {
        body["start"] = true.into();
    }
    body
}

/// The pool of the root disk `profiles` give an Instance, the last one's winning.
async fn root_pool(incus: &Incus, project: &str, profiles: &[String]) -> AppResult<Option<String>> {
    let all = crate::profiles::list(incus, project).await?;
    Ok(profiles
        .iter()
        .filter_map(|name| all.iter().find(|p| &p.name == name))
        .filter_map(|p| {
            p.devices
                .values()
                .find(|d| {
                    d.get("type").map(String::as_str) == Some("disk")
                        && d.get("path").map(String::as_str) == Some("/")
                })
                .and_then(|d| d.get("pool").cloned())
        })
        .next_back())
}

/// Makes an Instance from `spec`, sending the image download's progress to
/// `sink` until it's made (and started, if asked) or failed.
pub async fn create(
    incus: &Incus,
    project: &str,
    spec: &InstanceSpec,
    mut sink: impl FnMut(PullEvent) -> bool,
) {
    let result = async {
        validate_name(&spec.name)?;
        let pool = if spec.disk.as_deref().is_some_and(|d| !d.trim().is_empty()) {
            let pool = root_pool(incus, project, &spec.profiles).await?;
            if pool.is_none() {
                return Err(AppError::validation(msg::ROOT_DISK_UNKNOWN)
                    .with_hint(msg::ROOT_DISK_UNKNOWN_HINT));
            }
            pool
        } else {
            None
        };
        // Incus 6 starts it in the same operation; older ones need a start after.
        let start_with = spec.start && incus.server().has("instance_create_start");
        let body = create_body(spec, pool.as_deref(), start_with);
        let request = Request::post(scoped("/1.0/instances", project)).json(&body)?;
        incus
            .run_with(project, request, operations::LONG, |op| {
                if let Some(event) = crate::images::progress_event(op) {
                    sink(event);
                }
            })
            .await
            .map_err(|e| {
                if e.is(ErrorCode::NameInUse) {
                    AppError::new(
                        ErrorCode::NameInUse,
                        msg::INSTANCE_EXISTS.with("name", &spec.name),
                    )
                    .with_hint(msg::ALREADY_EXISTS_HINT)
                    .with_detail(e.detail.unwrap_or(e.message))
                } else {
                    e
                }
            })?;
        if spec.start && !start_with {
            act(incus, project, &spec.name, InstanceAction::Start, false).await?;
        }
        Ok::<_, AppError>(())
    }
    .await;
    match result {
        Ok(()) => {
            sink(PullEvent::Done);
        }
        Err(error) => {
            sink(PullEvent::Error { error });
        }
    }
}

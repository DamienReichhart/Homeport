//! Pulling a Managed Stack's images through the Engine API, with the stored
//! registry logins, before compose deploys it (ADR 0010). The CLI never sees
//! a login.

use std::collections::BTreeMap;

use bollard::Docker;
use serde::Deserialize;

use homeport_core::error::{AppError, AppResult};

use crate::messages as msg;
use homeport_engine::{
    auth::Logins,
    images::{self, PullEvent},
};

use super::{ComposeAction, ComposeEvent};

/// An image a Stack's services run, as `docker compose config` resolved it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StackImage {
    pub image: String,
    /// The services that run it, for messages.
    pub services: Vec<String>,
    /// A service asks for it to be pulled on every `up` (`pull_policy: always`,
    /// or a refresh period such as `daily`).
    pub always: bool,
}

#[derive(Deserialize)]
struct Config {
    #[serde(default)]
    services: BTreeMap<String, Service>,
}

#[derive(Deserialize)]
struct Service {
    image: Option<String>,
    pull_policy: Option<String>,
}

/// The images to pull, from `docker compose config --format json`. Services
/// with `pull_policy: never` or `build` are left to compose; several services
/// running one image pull it once.
pub(super) fn from_config(json: &str) -> AppResult<Vec<StackImage>> {
    let config: Config = serde_json::from_str(json)?;
    let mut images: Vec<StackImage> = Vec::new();
    for (name, service) in config.services {
        let Some(image) = service.image.filter(|i| !i.trim().is_empty()) else {
            continue;
        };
        let policy = service.pull_policy.as_deref().unwrap_or("missing");
        if matches!(policy, "never" | "build") {
            continue;
        }
        let always = !matches!(policy, "missing" | "if_not_present");
        match images.iter_mut().find(|i| i.image == image) {
            Some(existing) => {
                existing.services.push(name);
                existing.always |= always;
            }
            None => images.push(StackImage {
                image,
                services: vec![name],
                always,
            }),
        }
    }
    Ok(images)
}

/// Which images an action pulls: **Pull** refreshes every one; **Deploy**
/// pulls those the Host lacks and those a service always wants fresh.
pub(super) async fn needed(
    docker: &Docker,
    images: Vec<StackImage>,
    action: ComposeAction,
) -> Vec<StackImage> {
    let mut needed = Vec::new();
    for image in images {
        let wanted = match action {
            ComposeAction::Pull => true,
            ComposeAction::Up => image.always || docker.inspect_image(&image.image).await.is_err(),
            ComposeAction::Down | ComposeAction::DownWithVolumes => false,
        };
        if wanted {
            needed.push(image);
        }
    }
    needed
}

/// Pulls each image in turn, telling the output which login it uses; stops
/// at the first failure. `Ok(false)` when the viewer went away.
pub(super) async fn pull(
    docker: &Docker,
    images: &[StackImage],
    logins: &Logins,
    sink: &impl Fn(ComposeEvent) -> bool,
) -> AppResult<bool> {
    for image in images {
        let auth = logins.for_reference(&image.image).cloned();
        let how = match &auth {
            Some(a) => format!("as {} on {}", a.username, a.address),
            None => "anonymously".to_owned(),
        };
        if !line(sink, format!("Pulling {} {how}", image.image)) {
            return Ok(false);
        }
        let failure = std::sync::Mutex::new(None::<AppError>);
        let open = std::sync::atomic::AtomicBool::new(true);
        images::pull(docker.clone(), image.image.clone(), auth, |event| {
            let text = match event {
                PullEvent::Progress {
                    id, status, total, ..
                } if total.is_none() && !is_layer(id.as_deref()) => status,
                PullEvent::Error { error } => {
                    *failure.lock().expect("lock poisoned") = Some(error);
                    return false;
                }
                _ => return true,
            };
            let ok = line(sink, format!("{}: {text}", image.image));
            open.store(ok, std::sync::atomic::Ordering::Relaxed);
            ok
        })
        .await;
        if let Some(error) = failure.into_inner().expect("lock poisoned") {
            // The reason, already a sentence of its own, moves to the detail.
            let detail = match &error.detail {
                Some(detail) => format!("{}\n{detail}", error.message),
                None => error.message.clone(),
            };
            return Err(error
                .with_message(
                    msg::PULL_FAILED
                        .with("image", &image.image)
                        .with("services", image.services.join(", ")),
                )
                .with_detail(detail));
        }
        if !open.into_inner() {
            return Ok(false);
        }
    }
    Ok(true)
}

fn line(sink: &impl Fn(ComposeEvent) -> bool, text: String) -> bool {
    sink(ComposeEvent::Line { text })
}

/// Layer events carry a 12-digit hex id; image-level ones carry the reference.
fn is_layer(id: Option<&str>) -> bool {
    id.is_some_and(|id| id.len() == 12 && id.chars().all(|c| c.is_ascii_hexdigit()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_config_names_each_image_once() {
        let json = r#"{
            "name": "shop",
            "services": {
                "web": { "image": "nginx:1.27" },
                "worker": { "image": "ghcr.io/org/app:1", "pull_policy": "always" },
                "api": { "image": "ghcr.io/org/app:1" },
                "cache": { "image": "redis:7", "pull_policy": "never" },
                "local": { "image": "built", "pull_policy": "build" },
                "nightly": { "image": "app:nightly", "pull_policy": "daily" },
                "noimage": { "command": ["true"] }
            }
        }"#;
        assert_eq!(
            from_config(json).unwrap(),
            [
                StackImage {
                    image: "ghcr.io/org/app:1".into(),
                    services: vec!["api".into(), "worker".into()],
                    always: true,
                },
                StackImage {
                    image: "app:nightly".into(),
                    services: vec!["nightly".into()],
                    always: true,
                },
                StackImage {
                    image: "nginx:1.27".into(),
                    services: vec!["web".into()],
                    always: false,
                },
            ]
        );
    }

    #[test]
    fn a_config_without_services_pulls_nothing() {
        assert!(from_config(r#"{"name":"x"}"#).unwrap().is_empty());
        assert!(from_config("not json").is_err());
    }

    #[test]
    fn layers_are_told_apart_from_image_lines() {
        assert!(is_layer(Some("0123456789ab")));
        assert!(!is_layer(Some("nginx:1.27")));
        assert!(!is_layer(None));
    }
}

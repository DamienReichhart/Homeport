//! Incus's profiles, the settings many Instances share: listing them with
//! the Instances using each, editing one as YAML (refused if it changed
//! since it was read), creating and deleting them.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::http::Request;

use crate::{
    client::{if_match, scoped, segment, Incus},
    messages as msg,
};

/// The profile every project has, which Incus doesn't let anyone delete.
pub const DEFAULT_PROFILE: &str = "default";

/// One profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusProfile {
    pub name: String,
    pub description: String,
    pub config: BTreeMap<String, String>,
    pub devices: BTreeMap<String, BTreeMap<String, String>>,
    /// The Instances using it, by name.
    pub used_by: Vec<String>,
}

/// A profile with what editing it needs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusProfileDetail {
    pub profile: IncusProfile,
    /// Its description, configuration and devices, as `incus profile edit` shows them.
    pub yaml: String,
    /// What it was when read, so saving refuses changes made since.
    pub etag: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawProfile {
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    config: BTreeMap<String, String>,
    #[serde(default)]
    devices: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(default)]
    used_by: Option<Vec<String>>,
}

/// What a profile's YAML holds: the fields `PUT` takes.
#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ProfileBody {
    #[serde(default)]
    pub(crate) description: String,
    #[serde(default)]
    pub(crate) config: BTreeMap<String, String>,
    #[serde(default)]
    pub(crate) devices: BTreeMap<String, BTreeMap<String, String>>,
}

/// The Instance names among `used_by`'s API paths.
pub(crate) fn instance_names(used_by: &[String]) -> Vec<String> {
    used_by
        .iter()
        .filter_map(|path| {
            let rest = path.strip_prefix("/1.0/instances/")?;
            let name = rest.split('?').next()?;
            (!name.contains('/')).then(|| decode(name))
        })
        .collect()
}

fn decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if let Some(byte) = text
                .get(i + 1..i + 3)
                .and_then(|hex| u8::from_str_radix(hex, 16).ok())
            {
                out.push(byte);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

impl RawProfile {
    fn into_profile(self) -> IncusProfile {
        IncusProfile {
            used_by: instance_names(&self.used_by.unwrap_or_default()),
            name: self.name,
            description: self.description,
            config: self.config,
            devices: self.devices,
        }
    }
}

fn path(name: &str) -> String {
    format!("/1.0/profiles/{}", segment(name))
}

/// The profiles of `project`, by name.
pub async fn list(incus: &Incus, project: &str) -> AppResult<Vec<IncusProfile>> {
    let raw: Vec<RawProfile> = incus
        .sync(Request::get(scoped("/1.0/profiles?recursion=1", project)))
        .await?;
    let mut profiles: Vec<IncusProfile> = raw.into_iter().map(RawProfile::into_profile).collect();
    profiles.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(profiles)
}

/// The profile `name`, with its YAML and the ETag to save it with.
pub async fn get(incus: &Incus, project: &str, name: &str) -> AppResult<IncusProfileDetail> {
    let (raw, etag): (RawProfile, _) = incus
        .sync_tagged(Request::get(scoped(&path(name), project)))
        .await
        .map_err(|e| not_found(e, name))?;
    let body = ProfileBody {
        description: raw.description.clone(),
        config: raw.config.clone(),
        devices: raw.devices.clone(),
    };
    Ok(IncusProfileDetail {
        yaml: serde_yaml_ng::to_string(&body).unwrap_or_default(),
        profile: raw.into_profile(),
        etag,
    })
}

/// `yaml` read as a profile's description, configuration and devices.
pub(crate) fn parse_yaml(yaml: &str) -> AppResult<ProfileBody> {
    if yaml.trim().is_empty() {
        return Ok(ProfileBody::default());
    }
    serde_yaml_ng::from_str(yaml).map_err(|e| {
        AppError::validation(msg::PROFILE_YAML_INVALID)
            .with_hint(msg::PROFILE_YAML_INVALID_HINT)
            .with_detail(e)
    })
}

/// Replaces the profile's description, configuration and devices with
/// `yaml`; with `etag`, Incus refuses if it changed since it was read.
pub async fn update(
    incus: &Incus,
    project: &str,
    name: &str,
    yaml: &str,
    etag: Option<&str>,
) -> AppResult<()> {
    let body = parse_yaml(yaml)?;
    let request = if_match(
        Request::put(scoped(&path(name), project)).json(&body)?,
        etag,
    );
    incus
        .sync::<serde_json::Value>(request)
        .await
        .map(drop)
        .map_err(|e| not_found(e, name))
}

/// Makes an empty profile `name`.
pub async fn create(incus: &Incus, project: &str, name: &str, description: &str) -> AppResult<()> {
    let name = name.trim();
    validate_name(name)?;
    let body = serde_json::json!({
        "name": name,
        "description": description.trim(),
        "config": {},
        "devices": {},
    });
    incus
        .sync::<serde_json::Value>(Request::post(scoped("/1.0/profiles", project)).json(&body)?)
        .await
        .map(drop)
}

/// Deletes the profile `name`. `default` and profiles in use are refused,
/// naming the Instances that use it.
pub async fn delete(incus: &Incus, project: &str, name: &str) -> AppResult<()> {
    if name == DEFAULT_PROFILE {
        return Err(AppError::new(ErrorCode::Conflict, msg::PROFILE_DEFAULT));
    }
    let used_by = get(incus, project, name).await?.profile.used_by;
    if !used_by.is_empty() {
        return Err(in_use(name, &used_by));
    }
    incus
        .sync::<serde_json::Value>(Request::delete(scoped(&path(name), project)))
        .await
        .map(drop)
        .map_err(|e| {
            if e.is(ErrorCode::InUse) {
                in_use(name, &[])
            } else {
                not_found(e, name)
            }
        })
}

fn in_use(name: &str, users: &[String]) -> AppError {
    let message = if users.is_empty() {
        msg::IN_USE_UNNAMED.with("name", name)
    } else {
        msg::IN_USE
            .with("name", name)
            .with("users", users.join(", "))
    };
    AppError::new(ErrorCode::InUse, message).with_hint(msg::PROFILE_IN_USE_HINT)
}

/// Incus's rule for profile names: no slashes, not `.` or `..`, not empty.
fn validate_name(name: &str) -> AppResult<()> {
    if name.is_empty()
        || name == "."
        || name == ".."
        || name.contains('/')
        || name.contains(char::is_whitespace)
    {
        return Err(AppError::validation(
            msg::PROFILE_NAME_INVALID.with("name", name),
        ));
    }
    Ok(())
}

fn not_found(error: AppError, name: &str) -> AppError {
    if error.is(ErrorCode::NotFound) {
        AppError::new(
            ErrorCode::NotFound,
            msg::PROFILE_NOT_FOUND.with("name", name),
        )
        .with_detail(error.detail.unwrap_or(error.message))
    } else {
        error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn used_by_names_the_instances() {
        let used_by: Vec<String> = [
            "/1.0/instances/web",
            "/1.0/instances/my%20db?project=lab",
            "/1.0/instances/web/snapshots/snap0",
            "/1.0/networks/incusbr0",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(instance_names(&used_by), ["web", "my db"]);
    }

    #[test]
    fn yaml_round_trips_what_put_takes() {
        let body = ProfileBody {
            description: "Web servers".into(),
            config: BTreeMap::from([("limits.cpu".into(), "2".into())]),
            devices: BTreeMap::from([(
                "root".into(),
                BTreeMap::from([
                    ("path".into(), "/".into()),
                    ("pool".into(), "default".into()),
                    ("type".into(), "disk".into()),
                ]),
            )]),
        };
        let yaml = serde_yaml_ng::to_string(&body).unwrap();
        assert!(yaml.contains("limits.cpu: '2'"), "{yaml}");
        assert_eq!(parse_yaml(&yaml).unwrap(), body);
        assert_eq!(parse_yaml("").unwrap(), ProfileBody::default());
        let e = parse_yaml("config: [not, a, map]").unwrap_err();
        assert_eq!(e.code, ErrorCode::Validation);
        assert!(e.detail.is_some());
    }

    #[test]
    fn names_follow_incus_rules() {
        assert!(validate_name("web-servers").is_ok());
        for bad in ["", "a/b", "..", "two words"] {
            assert!(validate_name(bad).is_err(), "{bad}");
        }
    }

    #[tokio::test]
    #[ignore = "needs a local Incus"]
    async fn incus_live_profiles_are_created_edited_and_deleted() {
        let incus = crate::instances::tests::live().await;
        let name = format!("homeport-test-{}", std::process::id());
        create(&incus, "default", &name, "made by a test")
            .await
            .unwrap();
        let again = create(&incus, "default", &name, "").await.unwrap_err();
        assert_eq!(again.code, ErrorCode::NameInUse, "{again:?}");

        let read = get(&incus, "default", &name).await.unwrap();
        assert!(read.etag.is_some());
        let yaml = "description: edited\nconfig:\n  limits.cpu: '1'\ndevices: {}\n";
        update(&incus, "default", &name, yaml, read.etag.as_deref())
            .await
            .unwrap();
        // Saving with the ETag of before the edit is refused.
        let stale = update(&incus, "default", &name, yaml, read.etag.as_deref())
            .await
            .unwrap_err();
        assert_eq!(stale.code, ErrorCode::ChangedSinceLoaded, "{stale:?}");
        let edited = get(&incus, "default", &name).await.unwrap();
        assert_eq!(edited.profile.config["limits.cpu"], "1");
        let bad = update(
            &incus,
            "default",
            &name,
            "config:\n  limits.cpuz: '1'\n",
            None,
        )
        .await
        .unwrap_err();
        assert_eq!(bad.code, ErrorCode::BadRequest, "{bad:?}");

        let refused = delete(&incus, "default", DEFAULT_PROFILE)
            .await
            .unwrap_err();
        assert_eq!(refused.code, ErrorCode::Conflict);
        delete(&incus, "default", &name).await.unwrap();
    }
}

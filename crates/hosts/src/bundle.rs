//! Hosts in an export file (ADR 0013): written with their opened secrets and
//! trusted server keys under a passphrase, and read back on any computer,
//! where the secrets are sealed again under its own keychain key.

use serde::{Deserialize, Serialize};

use homeport_core::error::AppResult;
use homeport_storage::{crypto::bundle, db::sqlite_error, SecretBox};

use crate::{EngineKind, HostConnection, HostSecrets};

/// The payload version this Homeport writes and reads. Sections a later
/// version adds (Stacks, settings) sit next to `hosts`, so older readers
/// skip them and a later Homeport still reads this one.
pub const PAYLOAD_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub version: u32,
    /// When it was written, in UTC (RFC 3339).
    pub created_at: String,
    /// The Homeport that wrote it.
    pub app_version: String,
    #[serde(default)]
    pub hosts: Vec<ExportedHost>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportedHost {
    /// Its id, so a backup's Stacks keep their Host on restoring. An import
    /// gives each Host a new one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    /// As stored, bastions and their trusted keys included.
    pub connection: HostConnection,
    /// Files from before 0.18 have none: their Hosts were all Docker-compatible.
    #[serde(default)]
    pub engine: EngineKind,
    pub enabled: bool,
    /// The SSH server key trusted on first use.
    pub host_key: Option<String>,
    /// `None` when the Host needs none, or they couldn't be opened.
    pub secrets: Option<HostSecrets>,
}

impl std::fmt::Debug for ExportedHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExportedHost")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl Payload {
    pub fn new(hosts: Vec<ExportedHost>) -> Self {
        Self {
            version: PAYLOAD_VERSION,
            created_at: now_rfc3339(),
            app_version: env!("CARGO_PKG_VERSION").into(),
            hosts,
        }
    }

    pub fn seal(&self, passphrase: &str) -> AppResult<Vec<u8>> {
        bundle::seal(passphrase, &serde_json::to_vec(self)?)
    }

    pub fn open(passphrase: &str, bytes: &[u8]) -> AppResult<Self> {
        let json = bundle::open(passphrase, bytes)?;
        // Read the version first, so a newer payload is reported as such
        // rather than as unreadable data.
        #[derive(Deserialize)]
        struct Version {
            version: u32,
        }
        let Version { version } = serde_json::from_slice(&json)?;
        if version > PAYLOAD_VERSION {
            return Err(bundle::too_new());
        }
        Ok(serde_json::from_slice(&json)?)
    }
}

/// Writes a backup's Hosts in `tx`, into a database emptied of them, with
/// their ids, so their Stacks find them, and their secrets sealed under
/// this computer's key. A Host without its secrets asks for them again.
pub fn restore(
    tx: &rusqlite::Transaction<'_>,
    secrets: &SecretBox,
    hosts: &[ExportedHost],
) -> AppResult<()> {
    for host in hosts {
        let id = host
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let sealed = match &host.secrets {
            Some(opened) if host.connection.needs_secrets() => Some(secrets.seal_json(opened)?),
            _ => None,
        };
        tx.execute(
            "INSERT INTO hosts (id, name, connection, secrets, host_key, enabled, engine)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &id,
                &host.name,
                serde_json::to_string(&host.connection)?,
                &sealed,
                &host.host_key,
                host.enabled,
                host.engine.as_str(),
            ),
        )
        .map_err(sqlite_error)?;
    }
    Ok(())
}

/// The current time in UTC as `YYYY-MM-DDTHH:MM:SSZ`.
pub fn now_rfc3339() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    rfc3339(secs)
}

/// Seconds since the epoch as RFC 3339, by Howard Hinnant's civil-from-days.
fn rfc3339(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use homeport_core::error::ErrorCode;

    const PASS: &str = "a long enough passphrase";

    #[test]
    fn timestamps_are_rfc3339_in_utc() {
        assert_eq!(rfc3339(0), "1970-01-01T00:00:00Z");
        assert_eq!(rfc3339(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(rfc3339(1_793_876_213), "2026-11-05T10:56:53Z");
    }

    #[test]
    fn payloads_round_trip_and_keep_unknown_sections_out_of_the_way() {
        let payload = Payload::new(vec![ExportedHost {
            id: None,
            name: "prod".into(),
            connection: HostConnection::Tcp {
                address: "h:2375".into(),
            },
            engine: EngineKind::Docker,
            enabled: false,
            host_key: None,
            secrets: None,
        }]);
        let opened = Payload::open(PASS, &payload.seal(PASS).unwrap()).unwrap();
        assert_eq!(opened.hosts[0].name, "prod");
        assert!(!opened.hosts[0].enabled);
        assert_eq!(opened.app_version, env!("CARGO_PKG_VERSION"));

        // A later Homeport's extra section is skipped.
        let later =
            br#"{"version":1,"createdAt":"","appVersion":"1.0.0","hosts":[],"stacks":[{}]}"#;
        let sealed = bundle::seal(PASS, later).unwrap();
        assert!(Payload::open(PASS, &sealed).unwrap().hosts.is_empty());
    }

    #[test]
    fn a_newer_payload_is_reported_as_such() {
        let sealed = bundle::seal(PASS, br#"{"version":2,"hosts":"changed"}"#).unwrap();
        let err = Payload::open(PASS, &sealed).err().unwrap();
        assert_eq!(err.code, ErrorCode::BundleTooNew);
    }
}

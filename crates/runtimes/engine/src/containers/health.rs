//! A container's healthcheck: its configuration, status and last results,
//! typed so the frontend doesn't read Engine JSON.

use bollard::{models::ContainerInspectResponse, Docker};
use serde::Serialize;

use homeport_core::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    /// `starting`, `healthy` or `unhealthy`.
    pub status: String,
    /// Failed checks in a row.
    pub failing_streak: u32,
    pub config: HealthConfig,
    /// The last results the Engine keeps (up to five), newest first.
    pub log: Vec<HealthResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HealthConfig {
    /// The check as it would be typed: the shell command for `CMD-SHELL`, the
    /// arguments quoted for `CMD`.
    pub command: String,
    /// `None` where the Engine's default applies (30 s, 30 s, 3, 0 s).
    pub interval_s: Option<u32>,
    pub timeout_s: Option<u32>,
    pub retries: Option<u32>,
    pub start_period_s: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HealthResult {
    /// RFC 3339 times.
    pub start: String,
    pub end: String,
    pub exit_code: i64,
    pub output: String,
}

/// The healthcheck of an inspected container; `None` without one, or when
/// it's turned off (`NONE`).
pub fn from_inspect(container: &ContainerInspectResponse) -> Option<Health> {
    let check = container.config.as_ref()?.healthcheck.as_ref()?;
    let test = check.test.as_ref()?;
    let command = match test.first().map(String::as_str)? {
        "CMD-SHELL" => test.get(1).cloned().unwrap_or_default(),
        "CMD" => shell_words::join(&test[1..]),
        _ => return None,
    };
    let seconds = |nanos: Option<i64>| {
        nanos
            .filter(|n| *n > 0)
            .and_then(|n| u32::try_from(n / 1_000_000_000).ok())
    };
    let state = container.state.as_ref()?.health.as_ref();
    let mut log: Vec<HealthResult> = state
        .and_then(|h| h.log.clone())
        .unwrap_or_default()
        .into_iter()
        .map(|r| HealthResult {
            start: r.start.unwrap_or_default(),
            end: r.end.unwrap_or_default(),
            exit_code: r.exit_code.unwrap_or_default(),
            output: r.output.unwrap_or_default(),
        })
        .collect();
    log.reverse();
    Some(Health {
        status: state
            .and_then(|h| h.status)
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty() && s != "none")
            .unwrap_or_else(|| "starting".into()),
        failing_streak: state
            .and_then(|h| h.failing_streak)
            .and_then(|s| u32::try_from(s).ok())
            .unwrap_or(0),
        config: HealthConfig {
            command,
            interval_s: seconds(check.interval),
            timeout_s: seconds(check.timeout),
            retries: check
                .retries
                .and_then(|r| u32::try_from(r).ok())
                .filter(|r| *r > 0),
            start_period_s: seconds(check.start_period),
        },
        log,
    })
}

pub async fn health(docker: &Docker, id: &str) -> AppResult<Option<Health>> {
    Ok(from_inspect(&docker.inspect_container(id, None).await?))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn inspect(value: serde_json::Value) -> ContainerInspectResponse {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn an_unhealthy_container_shows_its_failing_output_newest_first() {
        let health = from_inspect(&inspect(json!({
            "Config": { "Healthcheck": {
                "Test": ["CMD-SHELL", "curl -fsS localhost/health || exit 1"],
                "Interval": 10_000_000_000i64,
                "Retries": 3
            }},
            "State": { "Health": {
                "Status": "unhealthy",
                "FailingStreak": 4,
                "Log": [
                    { "Start": "2026-11-04T10:00:00Z", "End": "2026-11-04T10:00:01Z", "ExitCode": 0, "Output": "ok" },
                    { "Start": "2026-11-04T10:00:10Z", "End": "2026-11-04T10:00:11Z", "ExitCode": 1, "Output": "curl: (7) Failed to connect" }
                ]
            }}
        })))
        .unwrap();
        assert_eq!(health.status, "unhealthy");
        assert_eq!(health.failing_streak, 4);
        assert_eq!(
            health.config.command,
            "curl -fsS localhost/health || exit 1"
        );
        assert_eq!(
            (
                health.config.interval_s,
                health.config.timeout_s,
                health.config.retries
            ),
            (Some(10), None, Some(3))
        );
        assert_eq!(health.log[0].exit_code, 1);
        assert_eq!(health.log[0].output, "curl: (7) Failed to connect");
        assert_eq!(health.log[1].output, "ok");
    }

    #[test]
    fn exec_form_checks_read_as_typed() {
        let health = from_inspect(&inspect(json!({
            "Config": { "Healthcheck": { "Test": ["CMD", "pg_isready", "-U", "app user"] } },
            "State": {}
        })))
        .unwrap();
        assert_eq!(health.config.command, "pg_isready -U 'app user'");
        assert_eq!(health.status, "starting");
        assert!(health.log.is_empty());
    }

    #[test]
    fn no_or_disabled_healthchecks_have_no_health() {
        assert_eq!(
            from_inspect(&inspect(json!({ "Config": {}, "State": {} }))),
            None
        );
        assert_eq!(
            from_inspect(&inspect(json!({
                "Config": { "Healthcheck": { "Test": ["NONE"] } },
                "State": {}
            }))),
            None
        );
    }
}

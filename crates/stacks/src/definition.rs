//! Checks on a Stack's compose definition and environment.
//!
//! Compose resolves relative paths and reads `env_file`/`build` contexts on
//! the machine running the CLI, which isn't the Host (ADR 0001). A Managed
//! Stack has no folder on this machine, so those are refused or flagged
//! before deploying; each [`SourceKind`] says which rules apply.

use serde::Serialize;
use serde_yaml_ng::Value;

use homeport_core::error::{AppError, AppResult};

use crate::{messages as msg, runtime::SourceKind};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ComposeCheck {
    /// Problems that block deploying.
    pub errors: Vec<String>,
    /// Things that will probably not do what the author expects.
    pub warnings: Vec<String>,
}

impl ComposeCheck {
    pub fn is_deployable(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Compose project names: lowercase letters, digits, `-` and `_`, starting
/// with a letter or digit.
pub fn validate_name(name: &str) -> AppResult<()> {
    let mut chars = name.chars();
    let valid_start = chars
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    if valid_start
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        Ok(())
    } else {
        Err(AppError::validation(msg::STACK_NAME))
    }
}

/// What can be found wrong with a definition: the problems that block
/// deploying it, as errors, and the warnings as the editor shows them.
#[derive(Default)]
struct Review {
    blockers: Vec<AppError>,
    warnings: Vec<String>,
}

pub fn check(yaml: &str, kind: SourceKind) -> ComposeCheck {
    let review = review(yaml, kind);
    ComposeCheck {
        // The editor lists each blocker with its raw cause, if any.
        errors: review
            .blockers
            .into_iter()
            .map(|e| match e.detail {
                Some(detail) => format!("{} {detail}", e.message),
                None => e.message,
            })
            .collect(),
        warnings: review.warnings,
    }
}

/// The first problem that blocks deploying `yaml`, if any.
pub fn blocker(yaml: &str, kind: SourceKind) -> Option<AppError> {
    review(yaml, kind).blockers.into_iter().next()
}

/// Whether the definition's files on this machine are its project files, so
/// `build:`, `env_file:` and relative paths mean what their author meant.
fn has_project_folder(kind: SourceKind) -> bool {
    match kind {
        SourceKind::Managed => false,
    }
}

fn review(yaml: &str, kind: SourceKind) -> Review {
    let mut check = Review::default();
    let local = has_project_folder(kind);
    let doc: Value = match serde_yaml_ng::from_str(yaml) {
        Ok(doc) => doc,
        Err(e) => {
            check
                .blockers
                .push(AppError::validation(msg::COMPOSE_UNPARSABLE).with_detail(e));
            return check;
        }
    };
    let Some(services) = doc.get("services").and_then(Value::as_mapping) else {
        check
            .blockers
            .push(AppError::validation(msg::COMPOSE_NO_SERVICES_SECTION));
        return check;
    };
    if services.is_empty() {
        check
            .blockers
            .push(AppError::validation(msg::COMPOSE_NO_SERVICE));
    }

    for (name, service) in services {
        let name = name.as_str().unwrap_or("?");
        if !local && service.get("build").is_some() {
            check.blockers.push(AppError::validation(
                msg::COMPOSE_BUILD.with("service", name),
            ));
        }
        if service.get("image").is_none() && service.get("build").is_none() {
            check.blockers.push(AppError::validation(
                msg::COMPOSE_NO_IMAGE.with("service", name),
            ));
        }
        if local {
            continue;
        }
        if service.get("env_file").is_some() {
            check.warnings.push(format!(
                "Service \"{name}\" uses env_file:, which is read on this machine. Put the variables in the Stack's environment instead."
            ));
        }
        for volume in service
            .get("volumes")
            .and_then(Value::as_sequence)
            .into_iter()
            .flatten()
        {
            if let Some(source) = bind_source(volume).filter(|s| is_relative(s)) {
                check.warnings.push(format!(
                    "Service \"{name}\" mounts \"{source}\", a path relative to this machine. Use an absolute path on the Host or a named volume."
                ));
            }
        }
    }

    if local {
        return check;
    }
    for section in ["configs", "secrets"] {
        for (name, entry) in doc
            .get(section)
            .and_then(Value::as_mapping)
            .into_iter()
            .flatten()
        {
            if entry.get("file").is_some() {
                check.warnings.push(format!(
                    "{section} \"{}\" is read from a file on this machine.",
                    name.as_str().unwrap_or("?")
                ));
            }
        }
    }
    check
}

/// The host side of a service volume, when it's a path rather than a named volume.
fn bind_source(volume: &Value) -> Option<String> {
    match volume {
        Value::String(short) => {
            let source = short.split(':').next()?;
            (source.starts_with(['.', '~', '/'])).then(|| source.to_owned())
        }
        Value::Mapping(_) => {
            let kind = volume.get("type").and_then(Value::as_str);
            let source = volume.get("source").and_then(Value::as_str)?;
            (kind == Some("bind")).then(|| source.to_owned())
        }
        _ => None,
    }
}

fn is_relative(path: &str) -> bool {
    !path.starts_with('/')
}

/// Parses `.env` text into variables for the compose process. Supports
/// comments, `export` prefixes and single or double quotes.
pub fn parse_env(text: &str) -> AppResult<Vec<(String, String)>> {
    let mut vars = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line);
        let Some((key, value)) = line.split_once('=') else {
            return Err(AppError::validation(
                msg::ENV_NOT_KEY_VALUE.with("line", n + 1),
            ));
        };
        let key = key.trim();
        if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(AppError::validation(msg::ENV_BAD_NAME.with("line", n + 1)));
        }
        let value = value.trim();
        let value = if let Some(inner) = value.strip_prefix('"').and_then(|v| v.strip_suffix('"')) {
            inner.replace("\\n", "\n").replace("\\\"", "\"")
        } else if let Some(inner) = value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')) {
            inner.to_owned()
        } else {
            // Unquoted values end at an inline comment.
            value.split(" #").next().unwrap_or("").trim_end().to_owned()
        };
        vars.push((key.to_owned(), value));
    }
    Ok(vars)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_managed(yaml: &str) -> ComposeCheck {
        check(yaml, SourceKind::Managed)
    }

    #[test]
    fn stack_names_follow_compose_rules() {
        assert!(validate_name("shop-2_prod").is_ok());
        for bad in ["", "Shop", "-shop", "my shop", "café"] {
            assert!(validate_name(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn plain_image_stacks_pass() {
        let c = check_managed(
            "services:\n  web:\n    image: nginx\n    volumes:\n      - data:/data\n      - /srv/site:/usr/share/nginx/html:ro\nvolumes:\n  data: {}\n",
        );
        assert_eq!(c, ComposeCheck::default());
    }

    #[test]
    fn local_builds_are_refused() {
        let c = check_managed("services:\n  app:\n    build: .\n");
        assert!(!c.is_deployable());
        assert!(c.errors[0].contains("build:"));
    }

    #[test]
    fn paths_read_on_this_machine_are_flagged() {
        let c = check_managed(
            "services:\n  app:\n    image: app\n    env_file: .env\n    volumes:\n      - ./data:/data\n      - type: bind\n        source: ~/conf\n        target: /conf\nsecrets:\n  key:\n    file: ./key.pem\n",
        );
        assert!(c.is_deployable());
        assert_eq!(c.warnings.len(), 4, "{:?}", c.warnings);
    }

    #[test]
    fn malformed_definitions_are_explained() {
        assert!(!check_managed("services: [").is_deployable());
        assert!(!check_managed("version: '3'\n").is_deployable());
        assert!(!check_managed("services:\n  app:\n    ports: []\n").is_deployable());
    }

    #[test]
    fn env_files_parse_like_compose() {
        let vars = parse_env(
            "# comment\nexport A=1\nB = two words # note\nC=\"quoted # kept\"\nD='single'\nE=\n",
        )
        .unwrap();
        assert_eq!(
            vars,
            [
                ("A".into(), "1".into()),
                ("B".into(), "two words".into()),
                ("C".into(), "quoted # kept".into()),
                ("D".into(), "single".into()),
                ("E".into(), String::new()),
            ]
        );
        assert!(parse_env("not a pair").is_err());
        assert!(parse_env("BAD-NAME=1").is_err());
    }
}

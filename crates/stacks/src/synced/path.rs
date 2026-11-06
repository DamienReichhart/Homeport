//! A Synced Stack's compose file path: relative to the repository's root,
//! with `/`, and never outside it.

use homeport_core::error::{AppError, AppResult};

use crate::messages as msg;

/// `path` trimmed and checked: relative, no `..`, no `.git`, no empty part.
pub fn normalise(path: &str) -> AppResult<String> {
    let trimmed = path.trim().replace('\\', "/");
    let trimmed = trimmed.trim_start_matches("./");
    let refuse = || AppError::validation(msg::SYNCED_PATH.with("path", path.trim()));
    if trimmed.is_empty() || trimmed.starts_with('/') || trimmed.contains(':') {
        return Err(refuse());
    }
    let parts: Vec<&str> = trimmed.split('/').collect();
    if parts
        .iter()
        .any(|p| p.is_empty() || *p == "." || *p == ".." || p.eq_ignore_ascii_case(".git"))
    {
        return Err(refuse());
    }
    let name = parts.last().copied().unwrap_or_default();
    if !(name.ends_with(".yaml") || name.ends_with(".yml")) {
        return Err(AppError::validation(
            msg::SYNCED_PATH_NOT_YAML.with("path", trimmed),
        ));
    }
    Ok(parts.join("/"))
}

/// The folder of a normalised path, empty for the repository's root.
pub fn folder_of(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(folder, _)| folder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_stay_inside_the_repository() {
        assert_eq!(normalise(" compose.yaml ").unwrap(), "compose.yaml");
        assert_eq!(
            normalise("./deploy/prod/compose.yml").unwrap(),
            "deploy/prod/compose.yml"
        );
        assert_eq!(
            normalise("apps\\web\\compose.yaml").unwrap(),
            "apps/web/compose.yaml"
        );
        for path in [
            "",
            "/etc/compose.yaml",
            "../compose.yaml",
            "apps/../../compose.yaml",
            "apps//compose.yaml",
            ".git/config.yaml",
            "C:/compose.yaml",
            "apps/",
        ] {
            assert!(normalise(path).is_err(), "{path:?} was accepted");
        }
        assert!(normalise("apps/Dockerfile").is_err());
    }

    #[test]
    fn the_folder_is_the_project_directory() {
        assert_eq!(folder_of("compose.yaml"), "");
        assert_eq!(folder_of("deploy/prod/compose.yaml"), "deploy/prod");
    }
}

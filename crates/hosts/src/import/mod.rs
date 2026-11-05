//! Adding Hosts from what this computer already describes: the servers of
//! `~/.ssh/config` and the Docker CLI's contexts. Each is first previewed as
//! drafts, then the ones the user picks are created through
//! [`HostService::create`](crate::service::HostService::create), which seals
//! their keys and certificates (ADR 0002). Neither source is ever written to.

pub mod docker_context;
pub mod ssh_config;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use homeport_core::error::AppError;

use crate::{service::HostView, HostConnection};

/// One entry a source offers, as the preview shows it.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportDraft {
    /// The alias or context name it comes from; how the import names it.
    pub source: String,
    /// The Host name proposed for it.
    pub name: String,
    /// What would be created; `None` when it can't be imported.
    pub connection: Option<HostConnection>,
    /// The key or certificate files it reads, for the user to recognise.
    pub files: Vec<String>,
    /// Whether one of its keys is encrypted and needs a passphrase.
    pub needs_passphrase: bool,
    /// Why it can't be imported; such a draft can't be picked.
    pub problem: Option<AppError>,
    /// What will differ from the source, or needs a look.
    pub warnings: Vec<AppError>,
    /// A Host with this name already exists, so importing as is would fail.
    pub name_taken: bool,
}

/// A draft the user picked, with what they changed in the preview.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportChoice {
    pub source: String,
    pub name: String,
    /// The Engine socket on an SSH server; `/var/run/docker.sock` when absent.
    pub socket_path: Option<String>,
    /// For the entry's encrypted keys.
    pub passphrase: Option<String>,
}

#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportFailure {
    pub source: String,
    pub error: AppError,
}

/// What an import did: the Hosts it created, and why the others weren't.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportOutcome {
    pub created: Vec<HostView>,
    pub failed: Vec<ImportFailure>,
}

/// Where this user's files are.
#[derive(Debug, Clone)]
pub struct Env {
    pub home: PathBuf,
    /// The default SSH user, as ssh uses it.
    pub local_user: String,
    /// `DOCKER_CONFIG`, when set.
    pub docker_config: Option<PathBuf>,
}

impl Env {
    /// The current user's, or `None` without a home folder.
    pub fn current() -> Option<Self> {
        Some(Self {
            home: homeport_core::programs::home()?,
            local_user: std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .unwrap_or_else(|_| "root".into()),
            docker_config: std::env::var_os("DOCKER_CONFIG").map(PathBuf::from),
        })
    }
}

/// `name`, or `name (2)`, `name (3)`… when taken (ignoring case, as Host names are).
pub fn free_name(name: &str, taken: &[String]) -> String {
    let is_taken = |n: &str| taken.iter().any(|t| t.eq_ignore_ascii_case(n));
    if !is_taken(name) {
        return name.to_owned();
    }
    (2..)
        .map(|i| format!("{name} ({i})"))
        .find(|n| !is_taken(n))
        .expect("an unbounded range finds a free name")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taken_names_get_a_number() {
        let taken = vec!["prod".to_owned(), "Prod (2)".to_owned()];
        assert_eq!(free_name("staging", &taken), "staging");
        assert_eq!(free_name("PROD", &taken), "PROD (3)");
    }
}

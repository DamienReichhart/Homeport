//! Reading kubeconfigs: this machine's (`KUBECONFIG`, else `~/.kube/config`)
//! or one the user pasted, which is parsed in memory and never written to
//! disk. Each context becomes a Cluster: what reaches the API server in its
//! connection, every credential in its sealed secrets, files inlined.

mod resolve;

use std::path::{Path, PathBuf};

use kube::config::Kubeconfig;
use serde::{Deserialize, Serialize};

use homeport_core::{
    error::{local_file, AppError, AppResult, ErrorCode},
    programs,
};

use crate::{messages as msg, model::KubeCluster, AuthMethod};

pub use self::resolve::resolve;

/// A kubeconfig, merged from its files, and where it came from.
pub struct Source {
    pub config: Kubeconfig,
    /// Pasted into Homeport, so relative file paths mean nothing.
    pub pasted: bool,
    /// The files it was read from, in merge order; empty when pasted.
    pub paths: Vec<PathBuf>,
}

impl Source {
    /// This machine's kubeconfig: the files in `KUBECONFIG` that exist,
    /// merged as kubectl does, else `~/.kube/config`.
    pub fn local() -> AppResult<Self> {
        let paths = local_paths(std::env::var_os("KUBECONFIG"), programs::home().as_deref())?;
        let existing: Vec<PathBuf> = paths.iter().filter(|p| p.is_file()).cloned().collect();
        if existing.is_empty() {
            let shown = paths
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(AppError::new(
                ErrorCode::NotFound,
                msg::KUBECONFIG_MISSING.with("path", shown),
            )
            .with_hint(msg::KUBECONFIG_MISSING_HINT));
        }
        let mut config = Kubeconfig::default();
        for path in &existing {
            let next = Kubeconfig::read_from(path).map_err(|e| invalid(&e, Some(path)))?;
            config = config.merge(next).map_err(|e| invalid(&e, Some(path)))?;
        }
        Ok(Self {
            config,
            pasted: false,
            paths: existing,
        })
    }

    /// A kubeconfig the user pasted.
    pub fn pasted(text: &str) -> AppResult<Self> {
        let config = Kubeconfig::from_yaml(text).map_err(|e| invalid(&e, None))?;
        if config.contexts.is_empty() {
            return Err(AppError::validation(msg::KUBECONFIG_NO_CONTEXTS));
        }
        Ok(Self {
            config,
            pasted: true,
            paths: vec![],
        })
    }

    /// The pasted kubeconfig `text`, or this machine's without one.
    pub fn of(text: Option<&str>) -> AppResult<Self> {
        match text {
            Some(text) => Self::pasted(text),
            None => Self::local(),
        }
    }
}

/// The files `KUBECONFIG` lists (its OS's path-list separator), else `~/.kube/config`.
fn local_paths(
    kubeconfig: Option<std::ffi::OsString>,
    home: Option<&Path>,
) -> AppResult<Vec<PathBuf>> {
    if let Some(value) = kubeconfig {
        let paths: Vec<PathBuf> = std::env::split_paths(&value)
            .filter(|p| !p.as_os_str().is_empty())
            .collect();
        if !paths.is_empty() {
            return Ok(paths);
        }
    }
    let home = home.ok_or_else(|| AppError::new(ErrorCode::NotFound, msg::NO_HOME_FOLDER))?;
    Ok(vec![home.join(".kube").join("config")])
}

fn invalid(error: &dyn std::error::Error, path: Option<&Path>) -> AppError {
    let message = match path {
        Some(path) => msg::KUBECONFIG_INVALID_FILE.with("path", path),
        None => msg::KUBECONFIG_INVALID.into(),
    };
    AppError::new(ErrorCode::Validation, message)
        .with_hint(msg::KUBECONFIG_INVALID_HINT)
        .with_detail(error)
}

/// One context of a kubeconfig, as the import lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct KubeContext {
    pub name: String,
    /// The API server, when the context's cluster is in the kubeconfig.
    pub server: Option<String>,
    pub user: Option<String>,
    pub namespace: Option<String>,
    pub auth: Option<AuthMethod>,
    /// The kubeconfig's current context.
    pub current: bool,
    /// The Cluster already made from this context, which importing refreshes.
    pub added: Option<AddedCluster>,
    /// Why it can't be imported, if it can't.
    pub problem: Option<AppError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AddedCluster {
    pub id: String,
    pub name: String,
}

/// A kubeconfig's contexts, as the Add a Cluster page shows them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct KubeContexts {
    /// The files read, in merge order; empty for a pasted kubeconfig.
    pub paths: Vec<String>,
    pub contexts: Vec<KubeContext>,
}

/// One context the user picked, and the name its Cluster gets.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContextChoice {
    pub context: String,
    pub name: String,
}

/// Every context of `source`, each with the Cluster made from it, if any.
pub fn contexts(source: &Source, clusters: &[KubeCluster]) -> KubeContexts {
    let config = &source.config;
    let contexts = config
        .contexts
        .iter()
        .map(|named| {
            let context = named.context.as_ref();
            let cluster = context.and_then(|c| {
                config
                    .clusters
                    .iter()
                    .find(|n| n.name == c.cluster)
                    .and_then(|n| n.cluster.as_ref())
            });
            let resolved = resolve(source, &named.name);
            KubeContext {
                name: named.name.clone(),
                server: cluster.and_then(|c| c.server.clone()),
                user: context.and_then(|c| c.user.clone()),
                namespace: context.and_then(|c| c.namespace.clone()),
                auth: resolved.as_ref().ok().map(|(c, _)| c.auth.clone()),
                current: config.current_context.as_deref() == Some(named.name.as_str()),
                added: added_from(clusters, &named.name),
                problem: resolved.err(),
            }
        })
        .collect();
    KubeContexts {
        paths: source
            .paths
            .iter()
            .map(|p| p.display().to_string())
            .collect(),
        contexts,
    }
}

/// The Cluster made from the context `name`.
fn added_from(clusters: &[KubeCluster], name: &str) -> Option<AddedCluster> {
    clusters
        .iter()
        .find(|c| {
            c.connection
                .source
                .as_ref()
                .is_some_and(|s| s.context == name)
        })
        .map(|c| AddedCluster {
            id: c.id.clone(),
            name: c.name.clone(),
        })
}

/// The contents of a file the kubeconfig refers to, at most 1 MB.
fn read_referenced(source: &Source, path: &str) -> AppResult<String> {
    let path = Path::new(path);
    if source.pasted && !path.is_absolute() {
        return Err(
            AppError::validation(msg::KUBECONFIG_RELATIVE_PATH.with("path", path))
                .with_hint(msg::KUBECONFIG_RELATIVE_PATH_HINT)
                .with_command("kubectl config view --raw --flatten --minify"),
        );
    }
    const MAX: u64 = 1024 * 1024;
    let size = std::fs::metadata(path)
        .map_err(|e| local_file(path, e))?
        .len();
    if size > MAX {
        return Err(AppError::validation(
            msg::KUBECONFIG_FILE_TOO_LARGE.with("path", path),
        ));
    }
    std::fs::read_to_string(path).map_err(|e| local_file(path, e))
}

pub mod detect;
#[cfg(test)]
mod tests;

//! The one way every Stack runs: where its compose definition comes from (a
//! [`StackSource`]) and where it runs (a [`StackTarget`]).
//!
//! Deploying, pulling and taking down go through [`run`], which hands the
//! job to the compose CLI with the arguments its source decides and the
//! `DOCKER_HOST` its target decides. Starting, stopping and removing go
//! through [`act`] and [`remove`], on the Engine API, so they also work for
//! External Stacks, which have no source at all (ADR 0001).
//!
//! A new kind of source or target is a new variant here and its arm below;
//! the commands only resolve a Stack's source and target and hand them in.

mod source;

use std::sync::Arc;

use bollard::Docker;

use homeport_core::{error::AppResult, host::HostConnection};
use homeport_engine::auth::Logins;
use homeport_transport::Dialer;

use crate::{
    compose::{self, ComposeAction, ComposeCli, ComposeEvent},
    history::{DeployHistory, NewDeploy},
    lifecycle::{self, StackAction},
    store::ComposeFile,
};

pub(crate) use self::source::{Prepared, WorkDir};

/// Where a Stack's compose definition comes from. External Stacks have none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackSource {
    /// A definition Homeport holds, with its environment as process
    /// variables so nothing secret is written to disk. A single compose file
    /// goes in on stdin; several are written to the run's private folder,
    /// since compose takes only one file on stdin.
    Managed {
        files: Vec<ComposeFile>,
        env: String,
    },
    /// Compose files in a folder on this machine (ADR 0008): compose runs in
    /// the folder, so `build:`, `env_file:` and relative paths work.
    Linked {
        working_dir: std::path::PathBuf,
        /// Absolute paths, in the order compose merges them.
        config_files: Vec<String>,
    },
}

/// The kinds of source, for the rules a definition is checked against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum SourceKind {
    Managed,
    Linked,
}

impl StackSource {
    pub fn kind(&self) -> SourceKind {
        match self {
            Self::Managed { .. } => SourceKind::Managed,
            Self::Linked { .. } => SourceKind::Linked,
        }
    }
}

/// Where a Stack runs.
#[derive(Clone)]
pub enum StackTarget {
    /// A Compose project on a Host.
    Host {
        /// The Host's name, for messages.
        name: String,
        connection: HostConnection,
        /// Reaches TLS and SSH Hosts, through the per-command proxy.
        dialer: Option<Arc<dyn Dialer>>,
        /// The Host's Engine, for lifecycle and for pulling images.
        docker: Docker,
    },
}

impl StackTarget {
    /// The Engine the target runs on.
    pub fn docker(&self) -> &Docker {
        match self {
            Self::Host { docker, .. } => docker,
        }
    }

    /// Whether the target is this machine, where a path here means the same
    /// file to the Engine.
    pub fn is_local(&self) -> bool {
        match self {
            Self::Host { connection, .. } => matches!(connection, HostConnection::Local { .. }),
        }
    }

    /// The target's name, for messages.
    pub fn name(&self) -> &str {
        match self {
            Self::Host { name, .. } => name,
        }
    }
}

/// One compose command for one Stack.
pub struct ComposeJob {
    pub cli: ComposeCli,
    /// The Compose project name: the Stack's name.
    pub project: String,
    pub source: StackSource,
    pub target: StackTarget,
    /// For pulling images through the Engine API (ADR 0010).
    pub logins: Logins,
    pub action: ComposeAction,
    /// The Compose profiles the Stack deploys, besides its services without
    /// one. They replace any `COMPOSE_PROFILES` from elsewhere.
    pub profiles: Vec<String>,
    /// Where to record a deploy, for the Stack's history.
    pub record: Option<Recorder>,
}

/// Records the deploys of one Stack in its history.
pub struct Recorder {
    pub history: Arc<DeployHistory>,
    pub host_id: String,
    pub stack_id: String,
}

impl Recorder {
    /// Records `job` as started; a failure to record never stops the deploy.
    pub(crate) fn start(&self, job: &ComposeJob, prepared: &Prepared) -> Option<String> {
        let env = match &job.source {
            StackSource::Managed { env, .. } => env.as_str(),
            StackSource::Linked { .. } => "",
        };
        self.history
            .start(&NewDeploy {
                host_id: &self.host_id,
                stack_id: &self.stack_id,
                kind: job.source.kind(),
                name: &job.project,
                action: job.action,
                files: &prepared.definition,
                env,
                profiles: &job.profiles,
            })
            .map_err(|e| e.logged("recording a deploy"))
            .ok()
    }

    pub(crate) fn finish(&self, id: &str, success: bool, code: Option<i32>) {
        if let Err(e) = self.history.finish(id, success, code) {
            e.logged("recording how a deploy ended");
        }
    }
}

/// Runs one compose command, streaming its output to `sink`; `sink` returns
/// false once nobody listens, which stops the run.
pub async fn run(job: ComposeJob, sink: impl Fn(ComposeEvent) -> bool) {
    compose::run(job, sink).await;
}

/// Starts, stops or restarts the Stack named `name` on `target`.
pub async fn act(target: &StackTarget, name: &str, action: StackAction) -> AppResult<()> {
    match target {
        StackTarget::Host { docker, .. } => lifecycle::act(docker, name, action).await,
    }
}

/// Removes the Stack's containers and networks from `target`, keeping volumes.
pub async fn remove(target: &StackTarget, name: &str) -> AppResult<()> {
    match target {
        StackTarget::Host { docker, .. } => lifecycle::remove(docker, name).await,
    }
}

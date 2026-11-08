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

use homeport_core::{
    error::{AppError, AppResult},
    host::HostConnection,
};
use homeport_engine::auth::Logins;
use homeport_transport::Dialer;

use crate::{
    compose::{self, ComposeAction, ComposeCli, ComposeEvent},
    history::{DeployHistory, NewDeploy},
    lifecycle::{self, StackAction},
    messages as msg,
    owner::StackOwner,
    store::ComposeFile,
    swarm,
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
    /// A commit of a git repository checked out on this machine (ADR 0014):
    /// run like a Linked Stack from the compose file's folder, with the
    /// Stack's environment as process variables, as for a Managed one.
    Synced {
        working_dir: std::path::PathBuf,
        /// Absolute paths, in the order compose merges them.
        config_files: Vec<String>,
        env: String,
    },
}

/// The kinds of source, for the rules a definition is checked against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum SourceKind {
    Managed,
    Linked,
    Synced,
}

impl StackSource {
    pub fn kind(&self) -> SourceKind {
        match self {
            Self::Managed { .. } => SourceKind::Managed,
            Self::Linked { .. } => SourceKind::Linked,
            Self::Synced { .. } => SourceKind::Synced,
        }
    }
}

/// How the CLI and the Engine API reach one Host: a Stack's own, or the
/// manager a Swarm is reached through.
#[derive(Clone)]
pub struct Reach {
    /// The Host's name, for messages.
    pub name: String,
    pub connection: HostConnection,
    /// Reaches TLS and SSH Hosts, through the per-command proxy.
    pub dialer: Option<Arc<dyn Dialer>>,
    /// The Host's Engine, for lifecycle and for pulling images.
    pub docker: Docker,
}

/// Where a Stack runs.
#[derive(Clone)]
pub enum StackTarget {
    /// A Compose project on a Host.
    Host(Reach),
    /// A swarm stack on a Swarm, through one of its connected managers
    /// (ADR 0017): `docker stack deploy` runs against it.
    Swarm {
        swarm_id: String,
        /// The Swarm's name, for messages.
        name: String,
        manager: Reach,
    },
}

impl StackTarget {
    /// The Host a command reaches: the Stack's own, or the Swarm's manager.
    pub fn reach(&self) -> &Reach {
        match self {
            Self::Host(reach) | Self::Swarm { manager: reach, .. } => reach,
        }
    }

    /// The Engine the target runs on, or the manager of its Swarm.
    pub fn docker(&self) -> &Docker {
        &self.reach().docker
    }

    /// Whether the target is this machine, where a path here means the same
    /// file to the Engine. Never true for a Swarm, whose tasks run anywhere.
    pub fn is_local(&self) -> bool {
        match self {
            Self::Host(reach) => matches!(reach.connection, HostConnection::Local { .. }),
            Self::Swarm { .. } => false,
        }
    }

    pub fn is_swarm(&self) -> bool {
        matches!(self, Self::Swarm { .. })
    }

    /// The target's name, for messages.
    pub fn name(&self) -> &str {
        match self {
            Self::Host(reach) => &reach.name,
            Self::Swarm { name, .. } => name,
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
    /// On a Swarm: `--with-registry-auth`, sending the docker CLI's own
    /// registry logins to the nodes.
    pub registry_auth: bool,
}

/// Records the deploys of one Stack in its history.
pub struct Recorder {
    pub history: Arc<DeployHistory>,
    /// The Host or Swarm the Stack is deployed to.
    pub owner: StackOwner,
    pub stack_id: String,
    /// The commit a Synced Stack deploys.
    pub commit: Option<String>,
    /// Started by a Synced Stack's auto-deploy.
    pub automatic: bool,
}

impl Recorder {
    /// Records `job` as started; a failure to record never stops the deploy.
    pub(crate) fn start(&self, job: &ComposeJob, prepared: &Prepared) -> Option<String> {
        let env = match &job.source {
            StackSource::Managed { env, .. } | StackSource::Synced { env, .. } => env.as_str(),
            StackSource::Linked { .. } => "",
        };
        self.history
            .start(&NewDeploy {
                owner: &self.owner,
                stack_id: &self.stack_id,
                kind: job.source.kind(),
                name: &job.project,
                action: job.action,
                files: &prepared.definition,
                env,
                profiles: &job.profiles,
                commit: self.commit.as_deref(),
                automatic: self.automatic,
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

/// Starts, stops or restarts the Stack named `name` on `target`. A swarm
/// stack has no such operation: its services are scaled instead.
pub async fn act(target: &StackTarget, name: &str, action: StackAction) -> AppResult<()> {
    match target {
        StackTarget::Host(reach) => lifecycle::act(&reach.docker, name, action).await,
        StackTarget::Swarm { .. } => Err(AppError::validation(msg::SWARM_NO_START_STOP)),
    }
}

/// Removes the Stack from `target`, keeping volumes: its containers and
/// networks on a Host; its services, networks, secrets and configs on a
/// Swarm, as `docker stack rm` does.
pub async fn remove(target: &StackTarget, name: &str) -> AppResult<()> {
    match target {
        StackTarget::Host(reach) => lifecycle::remove(&reach.docker, name).await,
        StackTarget::Swarm { manager, .. } => {
            let report = swarm::remove(&manager.docker, name).await?;
            for kept in report.kept {
                log::warn!("kept {kept} of swarm stack {name}");
            }
            Ok(())
        }
    }
}

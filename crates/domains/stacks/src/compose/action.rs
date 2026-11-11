//! What a compose job does, and what it reports while it runs.

use serde::{Deserialize, Serialize};

use homeport_core::error::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ComposeAction {
    /// `up -d --remove-orphans`: deploy or redeploy, after pulling the
    /// images the Host lacks through the Engine API. Builds what has no
    /// image yet.
    Up,
    /// `up -d --remove-orphans --build`: also rebuilds every service with
    /// `build:` first.
    Rebuild,
    /// Pulls every service's image through the Engine API, then `up` with
    /// what was pulled, so only services whose image changed are recreated.
    /// Stops after a failed pull, leaving the running containers alone.
    Update,
    /// Pulls every service's image through the Engine API; compose isn't run.
    Pull,
    /// `down --remove-orphans`, keeping volumes.
    Down,
    /// `down -v --remove-orphans`: also deletes the Stack's volumes.
    DownWithVolumes,
}

impl ComposeAction {
    /// Whether the action deploys, so it's recorded in the Stack's history.
    pub fn deploys(self) -> bool {
        matches!(self, Self::Up | Self::Rebuild | Self::Update)
    }

    /// Whether the action needs a definition that can be deployed; taking a
    /// Stack down only needs its project name and files.
    pub fn reads_definition(self) -> bool {
        matches!(self, Self::Up | Self::Rebuild | Self::Update | Self::Pull)
    }

    /// The compose subcommand, for messages.
    pub(super) fn verb(self) -> &'static str {
        self.args()[0]
    }

    pub(super) fn args(self) -> &'static [&'static str] {
        match self {
            Self::Up | Self::Update => &["up", "--detach", "--remove-orphans"],
            Self::Rebuild => &["up", "--detach", "--remove-orphans", "--build"],
            Self::Pull => &["pull"],
            Self::Down => &["down", "--remove-orphans"],
            Self::DownWithVolumes => &["down", "--volumes", "--remove-orphans"],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ComposeEvent {
    Line {
        text: String,
    },
    /// `error` explains a failed exit, from the end of stderr.
    Exit {
        success: bool,
        code: Option<i32>,
        error: Option<AppError>,
    },
    Error {
        error: AppError,
    },
}

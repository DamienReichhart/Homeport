//! Running the local `docker compose` CLI for a [`ComposeJob`] (ADR 0001).
//!
//! The job's source decides the compose files, working directory and stdin
//! (a Managed Stack's YAML goes in on stdin and its environment as process
//! variables, so nothing secret is written to disk); its target decides
//! `DOCKER_HOST`. TLS and SSH Hosts are reached through a [`proxy`] that
//! exists only while the command runs. Images are pulled through the Engine
//! API with the stored registry logins first, so the CLI never needs one
//! (ADR 0010).

mod action;
mod cli;
mod error;
mod images;
mod run;
pub mod stack;

pub(crate) use self::run::run;
pub use self::{
    action::{ComposeAction, ComposeEvent},
    cli::{detect, ComposeCli},
    run::direct_docker_host,
};

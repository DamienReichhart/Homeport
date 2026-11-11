//! Git repositories for Synced Stacks (see `GLOSSARY.md` and ADR 0014).
//!
//! Homeport reads repositories with the user's `git` program and hands it a
//! repository's credentials for one command at a time: a private SSH agent
//! holding the deploy key, or an authorisation header set through the
//! environment. Nothing secret is written to disk, and the user's own git
//! configuration, credential helpers and SSH keys are never used.
//!
//! - [`url`] tells SSH and HTTPS repository URLs apart and checks them.
//! - [`store`] keeps the repositories, each secret sealed in SQLite, and
//!   [`Repositories`] their bare clones on this machine.
//! - [`cli`] finds `git` and runs it, isolated from the user's configuration.
//! - [`ops`] are the commands Synced Stacks need: listing a remote's
//!   branches and tags, fetching, reading files at a commit, comparing two
//!   commits and checking one out.

mod access;
mod agent;
pub mod cli;
mod error;
pub mod messages;
pub mod ops;
mod service;
pub mod store;
pub mod url;

pub use self::{
    access::Remote,
    cli::GitCli,
    ops::{Commit, RefKind, RemoteRefs},
    service::Repositories,
    store::{Access, RepoAuth, Repository, RepositoryDraft, RepositoryStore, Secret},
    url::RepoUrl,
};

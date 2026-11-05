//! Stacks: Compose projects on a Host (see `GLOSSARY.md` and ADR 0001).
//!
//! Every Stack is found through the `com.docker.compose.project` label. A
//! Managed Stack also has its compose definition stored here; a Linked Stack
//! has its compose files in a folder on this machine (ADR 0008); a Synced
//! Stack has its compose file in a git repository (ADR 0014); an External
//! Stack only has its running containers until it is Adopted.
//!
//! - [`list`] lists every Stack of a Host, recording the Linked ones it
//!   finds; [`stack`] tells them apart from its containers.
//! - [`definition`] checks a compose definition before it's stored.
//! - [`store`] keeps Managed Stacks, their environment sealed.
//! - [`history`] keeps what each deploy used, for rolling back.
//! - [`diff`] tells what a redeploy would change.
//! - [`linked`] keeps where each Linked Stack's folder and files are.
//! - [`synced`] keeps Synced Stacks, whose compose file comes from a git
//!   repository (ADR 0014), and lays a commit out to deploy it.
//! - [`runtime`] is the one way a Stack runs: its source (where the compose
//!   definition comes from) and its target (where it runs). Every command
//!   that deploys, pulls, takes down, starts, stops or removes goes through it.
//! - [`lifecycle`] starts, stops and removes any Stack through the Engine API.
//! - [`compose`] runs the `docker compose` CLI for a runtime job.

pub mod compose;
pub mod definition;
pub mod diff;
pub mod history;
pub mod lifecycle;
pub mod linked;
mod listing;
pub mod messages;
pub mod runtime;
pub mod stack;
pub mod store;
pub mod synced;

pub use self::{
    listing::list,
    stack::{derive, Stack, StackKind, StackProblem, StackRecord, StackService, StackState},
};

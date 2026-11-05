//! Stacks: Compose projects on a Host (see `GLOSSARY.md` and ADR 0001).
//!
//! Every Stack is found through the `com.docker.compose.project` label. A
//! Managed Stack also has its compose definition stored here; a Linked Stack
//! has its compose files in a folder on this machine (ADR 0008); an External
//! Stack only has its running containers until it is Adopted.
//!
//! - [`stack`] tells the Stacks of a Host apart from its containers.
//! - [`definition`] checks a compose definition before it's stored.
//! - [`store`] keeps Managed Stacks, their environment sealed.
//! - [`history`] keeps what each deploy used, for rolling back.
//! - [`linked`] keeps where each Linked Stack's folder and files are.
//! - [`runtime`] is the one way a Stack runs: its source (where the compose
//!   definition comes from) and its target (where it runs). Every command
//!   that deploys, pulls, takes down, starts, stops or removes goes through it.
//! - [`lifecycle`] starts, stops and removes any Stack through the Engine API.
//! - [`compose`] runs the `docker compose` CLI for a runtime job.

pub mod compose;
pub mod definition;
pub mod history;
pub mod lifecycle;
pub mod linked;
mod listing;
pub mod messages;
pub mod runtime;
pub mod stack;
pub mod store;

pub use self::{
    listing::list,
    stack::{derive, Stack, StackKind, StackProblem, StackRecord, StackService, StackState},
};

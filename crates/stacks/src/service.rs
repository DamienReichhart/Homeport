//! Every store of Stacks in one place, as the app holds them.

use std::sync::Arc;

use crate::{
    history::DeployHistory,
    linked::{FileCache, LinkedStore},
    store::StackStore,
    synced::SyncedStacks,
};

/// The Stacks Homeport knows, of every kind, and what it remembers about
/// them.
pub struct Stacks {
    pub managed: StackStore,
    pub linked: LinkedStore,
    /// The compose files of Linked Stacks, as last read.
    pub linked_files: FileCache,
    pub history: Arc<DeployHistory>,
    pub synced: Arc<SyncedStacks>,
}

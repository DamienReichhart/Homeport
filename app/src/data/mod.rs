//! The live data layer: what TanStack Query was to the React app. Features
//! read Hosts and Clusters through it so none invents its own loading,
//! caching or refreshing.
//!
//! - [`Key`] and [`keys`]: where each resource is filed. Every key of a
//!   Host starts with `keys::host::scope(id)`, of a Cluster with
//!   `keys::cluster::scope(id)`, so one invalidation refreshes all it has.
//! - [`Query`]: how a resource is read (a use case of `api`) and how long
//!   it stays good. [`resources`] has the ones several features share.
//! - [`Data`]: the cache, kept in the window's state. A page `watch`es what
//!   it shows, reads it in its view as a [`Resource`] (`Idle`, `Loading`,
//!   `Loaded`, `Refreshing`, `Failed` with the previous data), and
//!   `unwatch`es it as it closes, which aborts a read nobody waits for.
//! - `live`: [`Data::on_event`] keeps the cache in step with what the
//!   backend pushes, instead of polling.
//! - [`Action`]: something the user asks for, with its pending state, its
//!   error and what it makes out of date.
//! - [`streams`]: logs, stats, progress and terminals as iced streams.
//!
//! Nothing here blocks: reads are tasks on the runtime iced owns, and their
//! answers come back as [`Message`]s for [`Data::update`].

pub mod action;
mod cache;
mod key;
pub mod keys;
pub mod live;
mod query;
mod resource;
pub mod resources;
mod store;
pub mod streams;
#[cfg(test)]
mod tests;

pub use action::Action;
pub use key::Key;
pub use query::{Policy, Query};
pub use resource::Resource;
pub use store::{Data, Loaded, Message, SETTLE};

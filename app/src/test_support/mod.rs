//! What the app's tests stand on, in place of vitest, Testing Library and
//! the IPC mocks: sample data, a way to run what `update` returns, and
//! snapshots of what `view` draws. Only built for tests.
//!
//! A feature is tested in three layers (`docs/agents/iced-migration.md`,
//! Testing a feature):
//!
//! 1. its use cases, against [`crate::app::testing::app`];
//! 2. its `update`, by sending messages and running the tasks it returns
//!    with [`tasks::messages`];
//! 3. its `view`, with [`snapshot::assert_snapshot`], in English and Arabic,
//!    and with [`window::drive`] where the test presses and types.
//!
//! A page's streams and transfers are run against [`engine::Engine`], a
//! socket that speaks the Engine's API and counts what is open on it,
//! with [`following::Following`] standing for the runtime that starts
//! and drops a page's subscriptions.

#[cfg(unix)]
pub mod engine;
pub mod fixtures;
pub mod following;
pub mod snapshot;
pub mod stage;
pub mod tasks;
pub mod window;

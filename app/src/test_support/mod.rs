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
//! 3. its `view`, with [`snapshot::assert_snapshot`], in English and Arabic.

pub mod fixtures;
pub mod snapshot;
pub mod tasks;

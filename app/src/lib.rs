//! Homeport's desktop application, on iced: the use cases (`api`), what they
//! stand on (`app`) and the window (`ui`). See `docs/agents/iced-migration.md`.

// Nothing depends on the app, so `pub` inside it only means "visible to the
// rest of the app"; the library crates keep the lint.
#![allow(unreachable_pub)]
// Pages are ported one at a time; what none reads yet is still to come.
#![allow(dead_code)]

pub mod api;
pub mod app;
pub mod i18n;
pub mod messages;
#[cfg(test)]
pub mod test_support;
pub mod ui;

pub use ui::run;

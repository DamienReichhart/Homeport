//! What the UI stands on: the state every use case is given, built once at
//! startup, and the events the backend's crates push to the UI.

pub mod events;
pub mod setup;
mod state;
#[cfg(test)]
pub mod testing;

pub use self::state::AppState;

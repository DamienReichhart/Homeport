//! The diagnostics bundle a user attaches to a bug report: a summary of
//! versions and of the kinds of Hosts, Engines and Clusters in use
//! ([`summary`]), and the app's logs with secrets removed ([`redact`]).
//!
//! Everything here is a pure function of plain inputs. The shell gathers
//! them from each service and writes the zip; nothing is sent anywhere, and
//! sealed secrets are never read for it.

mod redact;
mod summary;

pub use redact::{redact, Aliases};
pub use summary::{summary, AppFacts, ClusterFacts, HostFacts, Inputs, StackCounts, Summary};

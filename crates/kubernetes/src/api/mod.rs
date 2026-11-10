//! Reaching the Kubernetes API: the kube-rs client built in memory, the
//! credentials an exec plugin issues, what the API server serves, and what its
//! failures mean.

pub mod classify;
pub mod client;
pub mod credentials;
pub mod discovery;

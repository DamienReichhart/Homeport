//! SSH Hosts: key handling and streams to the remote Engine socket over
//! `direct-streamlocal` channels (ADR 0002). Private keys stay in memory.

mod auth;
mod dialer;
mod host_key;
mod keys;
mod socket;

pub use self::{
    auth::Credentials,
    dialer::{SshDialer, SshTarget},
    host_key::{check_host_key, HostKeyCheck},
    keys::{authorized_keys_command, fingerprint, generate_key, public_key_of, GeneratedKey},
};

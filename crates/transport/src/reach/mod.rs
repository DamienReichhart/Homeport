//! Classifies failures to reach an Engine: local sockets and named pipes,
//! containerd's and Incus's sockets, then (for remote Hosts) the network, TLS and SSH layers.

mod containerd;
mod incus;
mod local;
mod network;
mod ssh;
mod tls;

pub(crate) use self::{containerd::containerd_group_commands, incus::incus_group_command};
pub use self::{
    containerd::local_containerd,
    incus::{is_restricted_incus_socket, local_incus},
    local::{local_socket, LocalEngine},
    network::network,
    ssh::ssh,
    tls::tls,
};
pub use homeport_core::error::reach::{classified_cause, connection_lost, io_cause};

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn io_causes_are_found_through_bollard() {
        let e = bollard::errors::Error::IOError {
            err: io::Error::from(io::ErrorKind::ConnectionRefused),
        };
        assert_eq!(
            io_cause(&e).map(io::Error::kind),
            Some(io::ErrorKind::ConnectionRefused)
        );
    }
}

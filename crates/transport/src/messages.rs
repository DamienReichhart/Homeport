//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.
//!
//! `{host}` is a remote Host's name and address ("prod (prod.example:2376)")
//! or an SSH server's host name, `{path}` an Engine socket's path.

homeport_core::messages! {
    // Connecting to a Host.

    /// bollard rejected a TCP Host's address.
    CONNECT_TCP_ADDRESS_INVALID = "errors.transport.connect.tcpAddressInvalid" => "The TCP address isn't valid.";
    /// A TLS Host has no certificates stored.
    CONNECT_TLS_MATERIAL_MISSING = "errors.transport.connect.tlsMaterialMissing" => "The TLS certificates are missing.";
    CONNECT_TLS_MATERIAL_MISSING_HINT = "errors.transport.connect.tlsMaterialMissing.hint" => "Add the CA certificate, client certificate and client key on the host page.";
    /// The Engine took too long to answer the first request.
    CONNECT_ENGINE_TIMEOUT = "errors.transport.connect.engineTimeout" => "The Docker Engine didn't answer in time.";

    // A local socket or named pipe.

    /// The local Engine socket didn't answer in time.
    LOCAL_TIMEOUT = "errors.transport.local.timeout" => "The Docker Engine at {path} didn't answer in time.";
    LOCAL_TIMEOUT_HINT = "errors.transport.local.timeout.hint" => "It may be starting or overloaded. Homeport keeps retrying.";
    /// The local Engine socket failed in a way not told apart.
    LOCAL_UNREACHABLE = "errors.transport.local.unreachable" => "Homeport can't talk to the Docker Engine at {path}.";
    LOCAL_UNREACHABLE_HINT = "errors.transport.local.unreachable.hint" => "Check that the Docker Engine is running, then reconnect.";
    /// Nothing listens on the local Engine socket.
    LOCAL_NOT_RUNNING = "errors.transport.local.notRunning" => "No Docker Engine is running at {path}.";
    LOCAL_NOT_RUNNING_DOCKER_ENGINE_HINT = "errors.transport.local.notRunning.dockerEngine.hint" => "Start the Docker Engine, and have it start with the computer:";
    LOCAL_NOT_RUNNING_DOCKER_ROOTLESS_HINT = "errors.transport.local.notRunning.dockerRootless.hint" => "Start your rootless Docker Engine, and have it start when you log in:";
    LOCAL_NOT_RUNNING_PODMAN_SOCKET_HINT = "errors.transport.local.notRunning.podmanSocket.hint" => "Start Podman's Docker-compatible socket:";
    LOCAL_NOT_RUNNING_PODMAN_MACHINE_HINT = "errors.transport.local.notRunning.podmanMachine.hint" => "Start the Podman machine:";
    LOCAL_NOT_RUNNING_COLIMA_HINT = "errors.transport.local.notRunning.colima.hint" => "Start Colima:";
    LOCAL_NOT_RUNNING_ORBSTACK_HINT = "errors.transport.local.notRunning.orbStack.hint" => "Open OrbStack, or start it from a terminal:";
    LOCAL_NOT_RUNNING_DOCKER_DESKTOP_HINT = "errors.transport.local.notRunning.dockerDesktop.hint" => "Open Docker Desktop and wait until its Engine is running.";
    LOCAL_NOT_RUNNING_RANCHER_DESKTOP_HINT = "errors.transport.local.notRunning.rancherDesktop.hint" => "Open Rancher Desktop and wait until its Engine is running.";
    LOCAL_NOT_RUNNING_UNKNOWN_HINT = "errors.transport.local.notRunning.unknown.hint" => "Start the Docker Engine that provides this socket, or check the socket path on the host page.";
    /// No local Engine socket exists and no Engine program is installed.
    LOCAL_NOT_INSTALLED = "errors.transport.local.notInstalled" => "The Docker Engine doesn't seem to be installed: nothing exists at {path}.";
    LOCAL_NOT_INSTALLED_LINUX_HINT = "errors.transport.local.notInstalled.linux.hint" => "Install Docker Engine (see docs.docker.com/engine/install), for example with Docker's install script, then let your user use it:";
    LOCAL_NOT_INSTALLED_MACOS_HINT = "errors.transport.local.notInstalled.macos.hint" => "Install Docker Desktop, OrbStack or Colima. With Homebrew, Colima is:";
    LOCAL_NOT_INSTALLED_WINDOWS_HINT = "errors.transport.local.notInstalled.windows.hint" => "Install Docker Desktop (see docs.docker.com/desktop), then open it:";
    /// The user may not use the local Engine socket.
    LOCAL_PERMISSION_DENIED = "errors.transport.local.permissionDenied" => "Homeport isn't allowed to use the Docker Engine at {path}.";
    LOCAL_PERMISSION_DENIED_DOCKER_GROUP_HINT = "errors.transport.local.permissionDenied.dockerGroup.hint" => "Add your user to the docker group, then log out and back in (or restart) so it applies:";
    LOCAL_PERMISSION_DENIED_DOCKER_USERS_HINT = "errors.transport.local.permissionDenied.dockerUsers.hint" => "Add your user to the docker-users group in an administrator terminal, then sign out and back in:";
    LOCAL_PERMISSION_DENIED_SOCKET_HINT = "errors.transport.local.permissionDenied.socket.hint" => "Check who may use the socket:";

    // The network to a remote Host.

    /// The Host's name doesn't resolve.
    NETWORK_DNS_FAILED = "errors.transport.network.dnsFailed" => "The name of {host} can't be found.";
    NETWORK_DNS_FAILED_HINT = "errors.transport.network.dnsFailed.hint" => "Check the address for typos, and that this computer's network and DNS work.";
    /// Nothing listens on the Host's port.
    NETWORK_REFUSED = "errors.transport.network.refused" => "{host} refused the connection.";
    NETWORK_REFUSED_HINT = "errors.transport.network.refused.hint" => "Check the port, that the service listens on it, and that no firewall blocks it.";
    /// The Host didn't answer in time.
    NETWORK_TIMEOUT = "errors.transport.network.timeout" => "{host} didn't answer in time.";
    NETWORK_TIMEOUT_HINT = "errors.transport.network.timeout.hint" => "Check the address, that the machine is on, and that no firewall drops the connection.";
    /// No route to the Host.
    NETWORK_UNREACHABLE = "errors.transport.network.unreachable" => "{host} can't be reached from this network.";
    NETWORK_UNREACHABLE_HINT = "errors.transport.network.unreachable.hint" => "Check your network connection, or connect the VPN it's behind.";
    /// The network failed in a way not told apart.
    NETWORK_FAILED = "errors.transport.network.failed" => "Homeport can't reach {host}.";
    NETWORK_FAILED_HINT = "errors.transport.network.failed.hint" => "Check the address and your network, then reconnect.";

    // TLS.

    /// A PEM file of a TLS Host can't be parsed or used.
    TLS_CA_UNUSABLE = "errors.transport.tls.caUnusable" => "The CA certificate can't be used.";
    TLS_CLIENT_CERT_UNUSABLE = "errors.transport.tls.clientCertUnusable" => "The client certificate can't be used.";
    TLS_CLIENT_KEY_UNUSABLE = "errors.transport.tls.clientKeyUnusable" => "The client key can't be used.";
    TLS_CLIENT_PAIR_UNUSABLE = "errors.transport.tls.clientPairUnusable" => "The client certificate and key can't be used.";
    TLS_CONFIG_UNUSABLE = "errors.transport.tls.configUnusable" => "The TLS configuration can't be used.";
    TLS_ADDRESS_UNUSABLE = "errors.transport.tls.addressUnusable" => "The address can't be used.";
    TLS_CA_EMPTY = "errors.transport.tls.caEmpty" => "The CA certificate contains no certificate.";
    TLS_CLIENT_CERT_EMPTY = "errors.transport.tls.clientCertEmpty" => "The client certificate contains no certificate.";
    TLS_MATERIAL_INVALID_HINT = "errors.transport.tls.materialInvalid.hint" => "Paste PEM files (ca.pem, cert.pem, key.pem) made for this Engine on the host page.";
    /// The Engine refused Homeport's client certificate.
    TLS_CLIENT_REJECTED = "errors.transport.tls.clientRejected" => "{host} rejected Homeport's client certificate.";
    TLS_CLIENT_REJECTED_HINT = "errors.transport.tls.clientRejected.hint" => "Use a client certificate (cert.pem and key.pem) signed by the CA the Engine trusts, its --tlscacert.";
    /// The Engine's certificate has expired.
    TLS_CERT_EXPIRED = "errors.transport.tls.certExpired" => "The certificate of {host} has expired.";
    TLS_CERT_EXPIRED_HINT = "errors.transport.tls.certExpired.hint" => "Renew the Docker Engine's TLS certificate, then reconnect.";
    /// The Engine's certificate starts later than this computer's clock.
    TLS_CERT_NOT_YET_VALID = "errors.transport.tls.certNotYetValid" => "The certificate of {host} isn't valid yet.";
    TLS_CERT_NOT_YET_VALID_HINT = "errors.transport.tls.certNotYetValid.hint" => "Check that this computer's date and time are right.";
    /// The Engine's certificate isn't signed by the Host's CA.
    TLS_UNKNOWN_CA = "errors.transport.tls.unknownCa" => "The certificate of {host} isn't signed by the CA certificate on the host page.";
    TLS_UNKNOWN_CA_HINT = "errors.transport.tls.unknownCa.hint" => "Paste the ca.pem that signed the Engine's certificate.";
    /// The Engine's certificate doesn't list the name connected to (`{name}`).
    TLS_NAME_MISMATCH = "errors.transport.tls.nameMismatch" => "The certificate of {host} isn't made for {name}.";
    TLS_NAME_MISMATCH_HINT = "errors.transport.tls.nameMismatch.hint" => "Connect with a name or IP address listed in the certificate, or reissue it to include this one.";
    /// The Engine says Homeport's client certificate has expired.
    TLS_CLIENT_CERT_EXPIRED = "errors.transport.tls.clientCertExpired" => "Homeport's client certificate has expired.";
    TLS_CLIENT_CERT_EXPIRED_HINT = "errors.transport.tls.clientCertExpired.hint" => "Make a new client certificate signed by the Engine's CA and paste it on the host page.";
    /// The address answers, but not with TLS.
    TLS_NOT_SUPPORTED = "errors.transport.tls.notSupported" => "{host} doesn't speak TLS.";
    TLS_NOT_SUPPORTED_HINT = "errors.transport.tls.notSupported.hint" => "Use a TCP connection for this address, or the Engine's TLS port (usually 2376).";
    /// The TLS handshake failed in a way not told apart.
    TLS_HANDSHAKE_FAILED = "errors.transport.tls.handshakeFailed" => "The secure connection to {host} failed.";
    TLS_HANDSHAKE_FAILED_HINT = "errors.transport.tls.handshakeFailed.hint" => "Check the certificates on the host page against the ones the Engine uses.";

    // SSH.

    /// The SSH server's key was never trusted.
    SSH_HOST_KEY_UNTRUSTED = "errors.transport.ssh.hostKeyUntrusted" => "The SSH server's key isn't trusted yet ({fingerprint}).";
    SSH_HOST_KEY_UNTRUSTED_HINT = "errors.transport.ssh.hostKeyUntrusted.hint" => "Check the fingerprint, then trust the key on the host page.";
    /// The SSH server presented a key other than the trusted one.
    SSH_HOST_KEY_CHANGED = "errors.transport.ssh.hostKeyChanged" => "The SSH server's key changed since it was trusted.";
    SSH_HOST_KEY_CHANGED_HINT = "errors.transport.ssh.hostKeyChanged.hint" => "If the machine was reinstalled, trust the new key on the host page. Otherwise someone may be intercepting the connection.";
    /// The SSH session went silent.
    SSH_STOPPED_ANSWERING = "errors.transport.ssh.stoppedAnswering" => "The SSH server at {host} stopped answering.";
    SSH_STOPPED_ANSWERING_HINT = "errors.transport.ssh.stoppedAnswering.hint" => "Check that the machine is on and reachable. Homeport keeps retrying.";
    /// Key exchange found no algorithm both sides support.
    SSH_NO_COMMON_ALGORITHM = "errors.transport.ssh.noCommonAlgorithm" => "Homeport and the SSH server at {host} share no encryption method.";
    SSH_NO_COMMON_ALGORITHM_HINT = "errors.transport.ssh.noCommonAlgorithm.hint" => "Update OpenSSH on that machine, or allow modern algorithms such as ed25519 and curve25519 in its sshd_config.";
    /// The SSH session failed in a way not told apart.
    SSH_FAILED = "errors.transport.ssh.failed" => "The SSH connection to {host} failed.";
    /// An SSH Host using a stored key has none.
    SSH_KEY_MISSING = "errors.transport.ssh.keyMissing" => "The SSH private key is missing.";
    SSH_KEY_MISSING_HINT = "errors.transport.ssh.keyMissing.hint" => "Add a private key on the host page, or generate one.";
    /// The stored or imported private key can't be decoded.
    SSH_KEY_UNREADABLE = "errors.transport.ssh.keyUnreadable" => "The SSH private key can't be read.";
    SSH_KEY_UNREADABLE_HINT = "errors.transport.ssh.keyUnreadable.hint" => "Check that it's an OpenSSH or PEM private key and that the passphrase is right.";
    /// A decoded private key's public half can't be written as OpenSSH.
    SSH_PUBLIC_KEY_UNENCODABLE = "errors.transport.ssh.publicKeyUnencodable" => "The private key's public half can't be encoded.";
    /// The SSH server refused every key offered.
    SSH_AUTH_REFUSED = "errors.transport.ssh.authRefused" => "The SSH server refused to let {user}@{host} log in.";
    SSH_AUTH_REFUSED_KEY_HINT = "errors.transport.ssh.authRefused.key.hint" => "Authorise Homeport's key for {user} by running this on {host}:";
    SSH_AUTH_REFUSED_PUBLIC_KEY_HINT = "errors.transport.ssh.authRefused.publicKey.hint" => "Add this host's public key to authorized_keys on {host}.";
    SSH_AUTH_REFUSED_AGENT_HINT = "errors.transport.ssh.authRefused.agent.hint" => "Load a key the SSH server accepts into your ssh-agent, then reconnect.";
    /// No ssh-agent could be reached.
    SSH_AGENT_UNAVAILABLE = "errors.transport.ssh.agentUnavailable" => "No ssh-agent is running.";
    SSH_AGENT_UNAVAILABLE_WINDOWS_HINT = "errors.transport.ssh.agentUnavailable.windows.hint" => "Start the OpenSSH Authentication Agent service in an administrator PowerShell, then add your key:";
    SSH_AGENT_UNAVAILABLE_UNIX_HINT = "errors.transport.ssh.agentUnavailable.unix.hint" => "Start an agent and add your key in the terminal Homeport is launched from, or pick another authentication method:";
    /// The ssh-agent failed to list or use its keys.
    SSH_AGENT_FAILED = "errors.transport.ssh.agentFailed" => "The ssh-agent failed to sign in.";
    SSH_AGENT_FAILED_HINT = "errors.transport.ssh.agentFailed.hint" => "Check that the agent holds your key with `ssh-add -l`.";
    /// No Engine program is installed on the SSH server.
    SSH_ENGINE_NOT_INSTALLED = "errors.transport.ssh.engineNotInstalled" => "The Docker Engine isn't installed on {host}.";
    SSH_ENGINE_NOT_INSTALLED_HINT = "errors.transport.ssh.engineNotInstalled.hint" => "Install it on {host}, for example with Docker's install script, then let {user} use it:";
    /// An Engine is installed on the SSH server but its socket is missing.
    SSH_ENGINE_NOT_RUNNING = "errors.transport.ssh.engineNotRunning" => "The Docker Engine isn't running on {host}: nothing listens at {path}.";
    SSH_ENGINE_NOT_RUNNING_HINT = "errors.transport.ssh.engineNotRunning.hint" => "Start it on {host}, and have it start with the machine:";
    /// The SSH user may not use the remote Engine socket.
    SSH_ENGINE_PERMISSION_DENIED = "errors.transport.ssh.enginePermissionDenied" => "{user} isn't allowed to use the Docker Engine on {host}.";
    SSH_ENGINE_PERMISSION_DENIED_HINT = "errors.transport.ssh.enginePermissionDenied.hint" => "On {host}, add {user} to the docker group. It applies to new logins, so reconnect afterwards:";
    /// The remote Engine socket couldn't be opened, and the server couldn't say why.
    SSH_SOCKET_UNAVAILABLE = "errors.transport.ssh.socketUnavailable" => "{user} can't open the Docker Engine socket at {path} on {host}.";
    SSH_SOCKET_UNAVAILABLE_HINT = "errors.transport.ssh.socketUnavailable.hint" => "Check that the Docker Engine runs on {host} and that {user} may use it. On {host}, run:";
    /// sshd refuses to forward to the remote Engine socket.
    SSH_FORWARDING_DISABLED = "errors.transport.ssh.forwardingDisabled" => "The SSH server on {host} doesn't allow forwarding to the Docker Engine socket.";
    SSH_FORWARDING_DISABLED_HINT = "errors.transport.ssh.forwardingDisabled.hint" => "Allow it in sshd's configuration on {host}, then restart sshd (the service is \"sshd\" on some systems):";
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use homeport_core::error::arguments;

    #[test]
    fn keys_are_unique_and_arguments_are_plain_names() {
        let mut keys = BTreeSet::new();
        for m in super::ALL {
            assert!(m.key.starts_with("errors.transport."), "{}", m.key);
            assert!(keys.insert(m.key), "{} is declared twice", m.key);
            for arg in arguments(m.en) {
                assert!(
                    !arg.is_empty() && arg.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
                    "{}: {{{arg}}}",
                    m.key
                );
            }
        }
    }
}

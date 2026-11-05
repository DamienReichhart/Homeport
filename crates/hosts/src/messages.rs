//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    /// A Host that was connected stopped answering.
    CONNECTION_LOST = "errors.hosts.connectionLost" => "Homeport lost the connection to {name}.";
    CONNECTION_LOST_HINT = "errors.hosts.connectionLost.hint" => "The Docker Engine or the network may have restarted. Homeport keeps retrying.";
    /// A command for a Host that isn't connected.
    NOT_CONNECTED = "errors.hosts.notConnected" => "{name} isn't connected.";
    /// A command for a Host that isn't connected and is no longer stored.
    NOT_CONNECTED_UNKNOWN = "errors.hosts.notConnectedUnknown" => "This host isn't connected.";
    /// …because it's disabled.
    NOT_CONNECTED_DISABLED_HINT = "errors.hosts.notConnected.hintDisabled" => "Enable the host on its page to use it.";
    /// …because Homeport is still connecting.
    NOT_CONNECTED_CONNECTING_HINT = "errors.hosts.notConnected.hintConnecting" => "Homeport is still connecting; try again in a moment.";
    /// …for another reason, shown on its page.
    NOT_CONNECTED_HINT = "errors.hosts.notConnected.hint" => "Open the host's page to see why and fix it.";
    /// Saving a TLS Host with SSH credentials.
    TLS_NEEDS_CERTIFICATES = "errors.hosts.tlsNeedsCertificates" => "TLS hosts need certificates.";
    /// Saving an SSH Host with TLS credentials.
    SSH_NEEDS_PRIVATE_KEY = "errors.hosts.sshNeedsPrivateKey" => "SSH hosts need a private key.";
    /// Trusting text that isn't an SSH public key.
    NOT_A_PUBLIC_KEY = "errors.hosts.notAPublicKey" => "That isn't an SSH public key.";
    /// Generating a key for a Host that doesn't use SSH.
    KEYS_ONLY_FOR_SSH = "errors.hosts.keysOnlyForSsh" => "Only SSH hosts use keys.";
    /// Asking for the setup of a key Homeport didn't generate.
    NO_MANAGED_KEY = "errors.hosts.noManagedKey" => "This host doesn't use a Homeport-managed SSH key.";
    /// A bastion set to use its own key, without one.
    BASTION_NEEDS_KEY = "errors.hosts.bastionNeedsKey" => "Bastion {n} needs its private key.";
    /// Trusting a key for a bastion the Host no longer goes through.
    NO_SUCH_BASTION = "errors.hosts.noSuchBastion" => "This host doesn't go through {address}.";
    // Importing Hosts.

    /// The user has no home folder to find `~/.ssh` or `~/.docker` in.
    NO_HOME_FOLDER = "errors.hosts.import.noHomeFolder" => "Homeport can't find your home folder.";
    /// There's no `~/.ssh/config` to import from.
    SSH_CONFIG_MISSING = "errors.hosts.import.sshConfigMissing" => "There's no SSH config at {path}.";
    /// An entry's `ProxyCommand` won't be used.
    IMPORT_PROXY_COMMAND = "errors.hosts.import.proxyCommand" => "Its ProxyCommand isn't supported; Homeport will connect directly. Add its jump servers as bastions if it needs them.";
    /// None of an entry's `IdentityFile`s could be read.
    IMPORT_KEY_UNREADABLE = "errors.hosts.import.keyUnreadable" => "The key of {alias} can't be read ({path}); it will use ssh-agent instead.";
    /// An entry's `Port` isn't a port.
    IMPORT_PORT_INVALID = "errors.hosts.import.portInvalid" => "The port of {alias} isn't a number between 1 and 65535 ({port}).";
    /// `ProxyJump` chains that lead back to themselves.
    IMPORT_JUMP_LOOP = "errors.hosts.import.jumpLoop" => "Its ProxyJump chain makes a loop through {alias}.";
    /// A `ProxyJump` entry that isn't `[user@]host[:port]`.
    IMPORT_JUMP_INVALID = "errors.hosts.import.jumpInvalid" => "Its ProxyJump entry \"{jump}\" isn't [user@]host[:port].";
    /// A context with no Docker endpoint.
    IMPORT_NO_DOCKER_ENDPOINT = "errors.hosts.import.noDockerEndpoint" => "It has no Docker endpoint.";
    /// A context endpoint of a kind Homeport doesn't connect to.
    IMPORT_ENDPOINT_UNSUPPORTED = "errors.hosts.import.endpointUnsupported" => "Homeport can't connect to {endpoint}.";
    /// A TLS context that turns certificate checks off.
    IMPORT_SKIP_TLS_VERIFY = "errors.hosts.import.skipTlsVerify" => "It skips TLS verification, and Homeport always verifies the Engine's certificate.";
    /// A TLS context with some of its three files.
    IMPORT_TLS_INCOMPLETE = "errors.hosts.import.tlsIncomplete" => "Its TLS folder doesn't hold all three files (ca.pem, cert.pem and key.pem) Homeport needs.";
    /// A context whose Engine is already a Host.
    IMPORT_ALREADY_A_HOST = "errors.hosts.import.alreadyAHost" => "{name} already connects to this Engine.";
    /// The entry the user picked is no longer in the source.
    IMPORT_SOURCE_GONE = "errors.hosts.import.sourceGone" => "{source} is no longer there; preview again.";
    /// A Host removed while it was in use.
    GONE = "errors.hosts.gone" => "This host no longer exists.";
    /// Saving a connection that needs credentials without them.
    NEEDS_CREDENTIALS = "errors.hosts.needsCredentials" => "This connection needs credentials.";
    /// Saving a Host without a name.
    NAME_EMPTY = "errors.hosts.nameEmpty" => "Give the host a name.";
    /// Saving a Host under a name another one has.
    NAME_IN_USE = "errors.hosts.nameInUse" => "A host named \"{name}\" already exists.";
    NAME_IN_USE_HINT = "errors.hosts.nameInUse.hint" => "Pick another name.";
}

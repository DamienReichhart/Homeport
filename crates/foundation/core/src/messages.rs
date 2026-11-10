//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

crate::messages! {
    // The app itself.

    /// A file or system call failed with nothing more specific to say.
    IO_FAILED = "errors.core.ioFailed" => "A file or system operation failed.";
    /// JSON that Homeport was given couldn't be parsed.
    UNREADABLE_DATA = "errors.core.unreadableData" => "Homeport received data it couldn't read.";
    /// A local Host without a socket path.
    SOCKET_PATH_MISSING = "errors.core.socketPathMissing" => "Enter the socket path.";
    /// A TCP or TLS Host whose address isn't host:port.
    ADDRESS_INVALID = "errors.core.addressInvalid" => "Enter the address as host:port, for example 192.168.1.10:2376.";
    /// An SSH Host without a host name.
    SSH_HOST_MISSING = "errors.core.sshHostMissing" => "Enter the SSH host.";
    /// An SSH Host without a user.
    SSH_USER_MISSING = "errors.core.sshUserMissing" => "Enter the SSH user.";
    /// An SSH Host on port 0.
    SSH_PORT_INVALID = "errors.core.sshPortInvalid" => "The SSH port must be between 1 and 65535.";
    /// An SSH Host whose remote socket path is relative.
    REMOTE_SOCKET_NOT_ABSOLUTE = "errors.core.remoteSocketNotAbsolute" => "The remote socket path must be absolute, like /var/run/docker.sock.";
    /// A bastion, by its place in the list, without a server.
    BASTION_HOST_MISSING = "errors.core.bastionHostMissing" => "Enter the server of bastion {n}.";
    /// A bastion without a user.
    BASTION_USER_MISSING = "errors.core.bastionUserMissing" => "Enter the user of bastion {n}.";
    /// A bastion on port 0.
    BASTION_PORT_INVALID = "errors.core.bastionPortInvalid" => "The port of bastion {n} must be between 1 and 65535.";
    /// A containerd Host saved with TCP or TLS, which containerd doesn't serve.
    CONTAINERD_NEEDS_SOCKET = "errors.core.containerdNeedsSocket" => "containerd only answers on its socket. Connect through its socket on this computer, or over SSH.";
    /// An Incus Host saved with plain TCP, which Incus doesn't serve.
    INCUS_NEEDS_SOCKET = "errors.core.incusNeedsSocket" => "Incus doesn't answer over plain TCP. Connect through its socket on this computer, over SSH, or over HTTPS with a trust token.";
    /// Two hops of one Host at the same address.
    BASTION_REPEATED = "errors.core.bastionRepeated" => "{address} appears twice on the way to this host.";

    // Files on this computer.

    /// A file the user picked doesn't exist.
    FILE_MISSING = "errors.file.missing" => "{path} doesn't exist.";
    /// The OS refused access to a file the user picked.
    FILE_FORBIDDEN = "errors.file.forbidden" => "Homeport isn't allowed to access {path}.";
    FILE_FORBIDDEN_HINT = "errors.file.forbidden.hint" => "Check the file's permissions, or pick another location.";
    /// The disk filled up while writing a file the user picked.
    FILE_DISK_FULL = "errors.file.diskFull" => "There's no space left to write {path}.";
    FILE_DISK_FULL_HINT = "errors.file.diskFull.hint" => "Free some disk space, or pick another drive.";
    /// The user picked a folder where a file was expected.
    FILE_IS_FOLDER = "errors.file.isFolder" => "{path} is a folder, not a file.";
    /// A file read as text holds binary data.
    FILE_NOT_TEXT = "errors.file.notText" => "{path} isn't a text file.";
    /// Any other failure to read or write a file the user picked.
    FILE_INACCESSIBLE = "errors.file.inaccessible" => "Homeport couldn't access {path}.";
    /// A key, certificate or compose file over the size limit.
    FILE_TOO_LARGE = "errors.file.tooLarge" => "{path} is too large to be a key, certificate or compose file.";
    FILE_TOO_LARGE_HINT = "errors.file.tooLarge.hint" => "Pick a file under 1 MB.";
    /// The file manager couldn't show a folder.
    FOLDER_NOT_OPENED = "errors.file.folderNotOpened" => "{path} can't be opened.";
    FOLDER_NOT_OPENED_HINT = "errors.file.folderNotOpened.hint" => "Open it in your file manager instead.";

    // Secrets in the OS keychain.

    /// A stored secret can't be decrypted with the keychain's key.
    CREDENTIALS_LOST = "errors.secrets.credentialsLost" => "Stored credentials can't be decrypted.";
    CREDENTIALS_LOST_HINT = "errors.secrets.credentialsLost.hint" => "The key that protects them is gone from the OS keychain. Re-enter the credentials on the host page, or regenerate the key.";
    /// The OS keychain can't be used, so no secret can be stored.
    KEYCHAIN_UNAVAILABLE = "errors.secrets.keychainUnavailable" => "The OS keychain is unavailable, so secrets can't be stored.";
    KEYCHAIN_UNAVAILABLE_HINT_LINUX = "errors.secrets.keychainUnavailable.hint.linux" => "Install and unlock a Secret Service keyring, such as GNOME Keyring or KWallet, then restart Homeport.";
    KEYCHAIN_UNAVAILABLE_HINT_MACOS = "errors.secrets.keychainUnavailable.hint.macos" => "Unlock the login keychain in Keychain Access, then restart Homeport.";
    KEYCHAIN_UNAVAILABLE_HINT_WINDOWS = "errors.secrets.keychainUnavailable.hint.windows" => "Make sure Windows Credential Manager is available, then restart Homeport.";
    /// The keychain answers, but has no entry for the key the database's
    /// secrets were sealed with: Homeport doesn't make a new one over them.
    KEYCHAIN_ENTRY_MISSING = "errors.secrets.keychainEntryMissing" => "The keychain entry Homeport's secrets were sealed with is missing, so they can't be opened or stored.";
    KEYCHAIN_ENTRY_MISSING_HINT = "errors.secrets.keychainEntryMissing.hint" => "If Homeport is using another keychain than before, switch back to that one and restart Homeport. If the entry is gone for good, start over with a new key in Settings → This machine, then re-enter the credentials or restore a backup.";

    // Reaching an Engine.

    /// The connection to a Host broke midway; `address` is the Host's label.
    CONNECTION_LOST = "errors.reach.connectionLost" => "The connection to {address} was cut.";
    CONNECTION_LOST_HINT = "errors.reach.connectionLost.hint" => "The machine or the network may have restarted. Homeport keeps retrying.";
    /// The connection to the Docker Engine broke midway.
    ENGINE_CONNECTION_LOST = "errors.reach.engineConnectionLost" => "The connection to the Docker Engine was cut.";

    // The Docker Engine's answers.

    /// The Engine didn't answer before the request timed out.
    ENGINE_TIMEOUT = "errors.engine.timeout" => "The Docker Engine didn't answer in time.";
    ENGINE_TIMEOUT_HINT = "errors.engine.timeout.hint" => "It may be busy. Try again in a moment.";
    /// The Engine's answer isn't the JSON Homeport expects.
    ENGINE_UNREADABLE = "errors.engine.unreadableAnswer" => "The Docker Engine sent an answer Homeport can't read.";
    ENGINE_UNREADABLE_HINT = "errors.engine.unreadableAnswer.hint" => "The Engine may be much newer or older than Homeport supports. Updating both usually helps.";
    /// The Engine's API version can't be parsed.
    API_VERSION_UNREADABLE = "errors.engine.apiVersionUnreadable" => "The Docker Engine reported an API version Homeport can't read.";
    /// The Engine's socket disappeared or refused the connection.
    ENGINE_STOPPED = "errors.engine.stopped" => "The Docker Engine stopped answering.";
    ENGINE_STOPPED_HINT = "errors.engine.stopped.hint" => "Check that it's still running. Homeport reconnects on its own.";
    /// A request to the Engine failed for no known reason.
    REQUEST_FAILED = "errors.engine.requestFailed" => "The Docker Engine request failed.";
    /// A container publishes a host port something else uses.
    PORT_IN_USE = "errors.engine.portInUse" => "Port {port} is already in use.";
    PORT_IN_USE_HINT = "errors.engine.portInUse.hint" => "Stop the container or program that uses it, or publish this container on another port.";
    /// A published port is in use, but the Engine didn't say which.
    PUBLISHED_PORT_IN_USE = "errors.engine.publishedPortInUse" => "A published port is already in use.";
    /// The disk the Engine stores its data on is full.
    ENGINE_DISK_FULL = "errors.engine.diskFull" => "The Docker Engine is out of disk space.";
    ENGINE_DISK_FULL_HINT = "errors.engine.diskFull.hint" => "Remove unused images, containers and volumes (Clean up on the Overview page), or free space on the machine it runs on.";
    /// The registry's pull rate limit was reached.
    RATE_LIMITED = "errors.engine.rateLimited" => "The registry's pull limit was reached.";
    RATE_LIMITED_HINT = "errors.engine.rateLimited.hint" => "Docker Hub limits anonymous pulls. Wait for the limit to reset (up to 6 hours), or add a Docker Hub login in Settings → Registries, which raises the limit.";
    /// The registry refused the stored login.
    LOGIN_FAILED = "errors.engine.loginFailed" => "The registry refused the stored user name and password.";
    LOGIN_FAILED_HINT = "errors.engine.loginFailed.hint" => "Check the login for this registry in Settings → Registries.";
    /// The registry denied access: the image is missing or private.
    AUTH_REQUIRED = "errors.engine.authRequired" => "The image doesn't exist, or it's private.";
    AUTH_REQUIRED_HINT = "errors.engine.authRequired.hint" => "Check the image name. If it's private, add a login for its registry in Settings → Registries.";
    /// The registry has no such image or tag.
    IMAGE_OR_TAG_MISSING = "errors.engine.imageOrTagMissing" => "This image or tag doesn't exist.";
    IMAGE_OR_TAG_MISSING_HINT = "errors.engine.imageOrTagMissing.hint" => "Check the image name and tag on the registry.";
    /// An image was removed from the Engine.
    IMAGE_GONE = "errors.engine.imageGone" => "This image no longer exists on the Docker Engine.";
    /// The Engine can't connect to an image registry.
    REGISTRY_UNREACHABLE = "errors.engine.registryUnreachable" => "The Docker Engine can't reach the image registry.";
    REGISTRY_UNREACHABLE_HINT = "errors.engine.registryUnreachable.hint" => "Check the internet access, DNS and proxy settings of the machine it runs on.";
    /// An image reference the Engine can't parse.
    INVALID_IMAGE_NAME = "errors.engine.invalidImageName" => "That image name isn't valid.";
    INVALID_IMAGE_NAME_HINT = "errors.engine.invalidImageName.hint" => "Use a name like nginx:1.27 or registry.example.com/team/app:tag, in lower case.";
    /// Plain HTTP sent to an Engine that only speaks TLS.
    TLS_REQUIRED = "errors.engine.tlsRequired" => "This Docker Engine only accepts TLS connections.";
    TLS_REQUIRED_HINT = "errors.engine.tlsRequired.hint" => "Change the connection to TLS on the host page and add the Engine's certificates.";
    /// The Engine's API is too old or too new for Homeport.
    API_VERSION_UNSUPPORTED = "errors.engine.apiVersionUnsupported" => "This Docker Engine's API version isn't supported.";
    API_VERSION_UNSUPPORTED_HINT = "errors.engine.apiVersionUnsupported.hint" => "Update the Docker Engine.";
    /// A container asks for a GPU the host can't provide.
    GPU_UNAVAILABLE = "errors.engine.gpuUnavailable" => "This host can't give containers a GPU.";
    GPU_UNAVAILABLE_HINT = "errors.engine.gpuUnavailable.hint" => "Install the GPU vendor's container toolkit on the host (for NVIDIA, the NVIDIA Container Toolkit) and restart the Docker Engine, or run the container without a GPU.";
    /// A CPU limit above the host's CPU count, which is more than one.
    CPUS_LIMITED = "errors.engine.cpusLimited" => "This host has only {cpus} CPUs to give a container.";
    CPUS_LIMITED_HINT = "errors.engine.cpusLimited.hint" => "Lower the CPU limit, or leave it empty to use every CPU.";
    /// A CPU limit above the host's single CPU.
    CPUS_LIMITED_ONE = "errors.engine.cpusLimited.one" => "This host has only {cpus} CPU to give a container.";
    /// A CPU limit above the host's CPU count, which the Engine didn't give.
    CPUS_FEWER = "errors.engine.cpusFewer" => "This host has fewer CPUs than the limit asks for.";
    /// Another container already has this name.
    CONTAINER_NAME_IN_USE = "errors.engine.containerNameInUse" => "That container name is already used.";
    CONTAINER_NAME_IN_USE_HINT = "errors.engine.containerNameInUse.hint" => "Pick another name, or remove or rename the other container first.";
    /// Removing a volume a container uses.
    VOLUME_IN_USE = "errors.engine.volumeInUse" => "This volume is used by a container.";
    VOLUME_IN_USE_HINT = "errors.engine.volumeInUse.hint" => "Remove the containers that use it first.";
    /// Removing a network containers are attached to.
    NETWORK_IN_USE = "errors.engine.networkInUse" => "This network still has containers attached.";
    NETWORK_IN_USE_HINT = "errors.engine.networkInUse.hint" => "Disconnect or remove its containers first.";
    /// Removing an image by ID while it has several tags.
    IMAGE_SEVERAL_TAGS = "errors.engine.imageSeveralTags" => "This image has several tags.";
    IMAGE_SEVERAL_TAGS_HINT = "errors.engine.imageSeveralTags.hint" => "Force the removal to delete every tag, or remove the tags one by one.";
    /// Removing an image a container uses.
    IMAGE_IN_USE = "errors.engine.imageInUse" => "This image is used by a container.";
    IMAGE_IN_USE_HINT = "errors.engine.imageInUse.hint" => "Remove the containers that use it, or force the removal.";
    /// Removing a running container without force.
    CONTAINER_RUNNING = "errors.engine.containerRunning" => "The container is running.";
    CONTAINER_RUNNING_HINT = "errors.engine.containerRunning.hint" => "Stop it first, or remove it with force.";
    /// Acting on a stopped container that must be running.
    CONTAINER_NOT_RUNNING = "errors.engine.containerNotRunning" => "The container isn't running.";
    CONTAINER_NOT_RUNNING_HINT = "errors.engine.containerNotRunning.hint" => "Start it first.";
    /// Pausing a paused container, or the like.
    ALREADY_IN_STATE = "errors.engine.alreadyInState" => "The container is already in that state.";
    /// A command run in a container doesn't exist there.
    COMMAND_NOT_FOUND = "errors.engine.commandNotFound" => "That command doesn't exist in the container.";
    COMMAND_NOT_FOUND_HINT = "errors.engine.commandNotFound.hint" => "Minimal images may have no shell or tools. Try /bin/sh, or another command.";
    /// A path in a container that doesn't exist.
    PATH_MISSING = "errors.engine.pathMissing" => "That path doesn't exist in the container.";
    /// A container was removed elsewhere.
    CONTAINER_GONE = "errors.engine.containerGone" => "This container no longer exists.";
    CONTAINER_GONE_HINT = "errors.engine.containerGone.hint" => "It was probably removed elsewhere. The list refreshes on its own.";
    /// A volume was removed elsewhere.
    VOLUME_GONE = "errors.engine.volumeGone" => "This volume no longer exists.";
    /// A network was removed elsewhere.
    NETWORK_GONE = "errors.engine.networkGone" => "This network no longer exists.";
    /// A Swarm object updated by someone else since Homeport read it.
    OUT_OF_SEQUENCE = "errors.engine.outOfSequence" => "This was changed elsewhere since Homeport read it.";
    OUT_OF_SEQUENCE_HINT = "errors.engine.outOfSequence.hint" => "Refresh and try again, so nothing changed elsewhere is overwritten.";
    /// An unrecognised HTTP 400 from the Engine.
    BAD_REQUEST = "errors.engine.badRequest" => "The Docker Engine rejected the request as invalid.";
    /// An unrecognised HTTP 401 or 403 from the Engine.
    FORBIDDEN = "errors.engine.forbidden" => "The Docker Engine refused to do this.";
    FORBIDDEN_HINT = "errors.engine.forbidden.hint" => "An authorisation plugin or a rootless Docker Engine may forbid it. The exact answer is in Details.";
    /// An unrecognised HTTP 404 from the Engine.
    GONE = "errors.engine.gone" => "This no longer exists on the Docker Engine.";
    GONE_HINT = "errors.engine.gone.hint" => "It was probably removed elsewhere. The lists refresh on their own.";
    /// An unrecognised HTTP 409 from the Engine.
    CONFLICT = "errors.engine.conflict" => "This clashes with the current state of the Docker Engine.";
    /// Any other unrecognised answer from the Engine.
    FAILED = "errors.engine.failed" => "The Docker Engine failed to do this.";
    /// The hint of every unrecognised answer the Engine gave in its own words.
    FAILED_HINT = "errors.engine.failed.hint" => "The Docker Engine's exact answer is in Details.";
}

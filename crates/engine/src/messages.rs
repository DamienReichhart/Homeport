//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    // Containers

    /// A terminal or Run dialog command with a quote left open.
    COMMAND_UNCLOSED_QUOTE = "errors.containers.commandUnclosedQuote" => "The command has an unclosed quote.";
    /// The Engine created the terminal's exec but didn't attach to it.
    TERMINAL_NOT_OPENED = "errors.containers.terminalNotOpened" => "The Docker Engine didn't open the terminal session.";
    /// Input or a resize for a terminal session that already closed.
    TERMINAL_ENDED = "errors.containers.terminalEnded" => "This terminal session has ended.";
    /// Renaming a container to an empty name.
    RENAME_EMPTY = "errors.containers.nameEmpty" => "Enter a name.";

    // Container files

    /// A container path that isn't absolute.
    FILES_PATH_NOT_ABSOLUTE = "errors.files.pathNotAbsolute" => "Paths must start with /.";
    /// Listing files in a container without `find` and `stat`.
    FILES_NO_TOOLS = "errors.files.noTools" => "This container has no find and stat tools, so its files can't be listed.";
    FILES_NO_TOOLS_HINT = "errors.files.noTools.hint" => "Minimal images (distroless, scratch) leave them out. Copy files with docker cp from a terminal instead.";
    /// Listing or uploading to a container path that doesn't exist.
    FILES_PATH_MISSING = "errors.files.pathMissing" => "{path} doesn't exist in the container.";
    /// Listing a container path its user may not read.
    FILES_NOT_ALLOWED = "errors.files.notAllowed" => "The container's user isn't allowed to read {path}.";
    FILES_NOT_ALLOWED_HINT = "errors.files.notAllowed.hint" => "Files owned by root can only be listed when the container runs as root.";
    /// Listing a container path failed for another reason, given in Details.
    FILES_NOT_LISTED = "errors.files.notListed" => "{path} can't be listed in the container.";
    /// The Engine created a command's exec but didn't attach to it.
    FILES_SESSION_NOT_OPENED = "errors.files.sessionNotOpened" => "The Docker Engine didn't open the command's session.";
    /// Downloading a file the Engine sent no archive entry for.
    FILES_EMPTY_ARCHIVE = "errors.files.emptyArchive" => "The Docker Engine sent an empty archive for this file.";
    /// Uploading a path without a file name, such as `/`.
    FILES_NOT_A_FILE = "errors.files.notAFile" => "{path} is not a file.";

    // Commit and import changes

    /// Committing a container under a name that isn't `repo:tag`.
    COMMIT_NAME = "errors.commit.name" => "Name the image like repo:tag, such as web-backup:2026-11-04.";
    /// A change line not starting with a supported instruction.
    COMMIT_UNKNOWN_INSTRUCTION = "errors.commit.unknownInstruction" => "\"{line}\" isn't an instruction Homeport can apply to an image. Start each line with one of {instructions}.";
    /// A change line with an instruction and nothing after it.
    COMMIT_NEEDS_VALUE = "errors.commit.needsValue" => "{instruction} needs a value, like {example}.";

    // Recreate

    /// Recreating failed, and so did putting the original back.
    RECREATE_ROLLBACK_FAILED = "errors.recreate.rollbackFailed" => "Recreating {name} failed, and Homeport couldn't put the original back.";
    /// The original container, which wasn't running, may be left renamed.
    RECREATE_ROLLBACK_FAILED_HINT = "errors.recreate.rollbackFailed.hint" => "The original container may still be named {asideName}. Rename it back to {name} and start it, or remove it once you've checked nothing in it is needed.";
    /// The original container, which was running, may be left renamed and stopped.
    RECREATE_ROLLBACK_FAILED_STOPPED_HINT = "errors.recreate.rollbackFailed.hintStopped" => "The original container may still be named {asideName}, stopped. Rename it back to {name} and start it, or remove it once you've checked nothing in it is needed.";

    // Run dialog

    /// Running without an image.
    RUN_IMAGE = "errors.run.image" => "Choose an image.";
    /// An environment entry that isn't `KEY=value`.
    RUN_ENV = "errors.run.envNotKeyValue" => "\"{entry}\" isn't a KEY=value environment variable.";
    /// A label that isn't `key=value`.
    RUN_LABEL = "errors.run.labelNotKeyValue" => "\"{entry}\" isn't a key=value label.";
    /// A log option that isn't `key=value`.
    RUN_LOG_OPTION = "errors.run.logOptionNotKeyValue" => "\"{entry}\" isn't a key=value log option.";
    /// A label in Docker Compose's own namespace.
    RUN_COMPOSE_LABEL = "errors.run.composeLabel" => "Labels starting with {prefix} belong to Docker Compose; Homeport would show the container as part of a Stack. Choose another key.";
    /// Removing on exit together with a restart policy.
    RUN_AUTO_REMOVE_RESTART = "errors.run.autoRemoveRestart" => "Containers removed on exit can't also restart.";
    /// A port protocol other than tcp, udp or sctp.
    RUN_PORT_PROTOCOL = "errors.run.portProtocol" => "Port protocols are tcp, udp or sctp.";
    /// A mount without a source or with a relative container path.
    RUN_MOUNT = "errors.run.mount" => "Mounts need a source and an absolute container path.";
    /// A CPU limit of 0 or less, or not a number.
    RUN_CPUS = "errors.run.cpus" => "The CPU limit must be a number above 0, like 0.5 or 2.";
    /// CPU shares below 2.
    RUN_CPU_SHARES = "errors.run.cpuShares" => "CPU shares must be 2 or more; 1024 is the default weight.";
    /// A memory limit of 0 or less.
    RUN_MEMORY_ABOVE_ZERO = "errors.run.memoryAboveZero" => "The memory limit must be above 0.";
    /// A memory reservation of 0 or less.
    RUN_MEMORY_RESERVATION_ABOVE_ZERO = "errors.run.memoryReservationAboveZero" => "The memory reservation must be above 0.";
    /// A memory limit below the Engine's minimum.
    RUN_MEMORY_MINIMUM = "errors.run.memoryMinimum" => "The memory limit must be at least 6 MB.";
    /// A memory reservation above the memory limit.
    RUN_RESERVATION_ABOVE_LIMIT = "errors.run.reservationAboveLimit" => "The memory reservation can't be above the memory limit.";
    /// A swap limit without a memory limit.
    RUN_SWAP_WITHOUT_MEMORY = "errors.run.swapWithoutMemory" => "Set a memory limit before limiting swap.";
    /// A memory and swap limit below the memory limit.
    RUN_SWAP_BELOW_MEMORY = "errors.run.swapBelowMemory" => "The memory and swap limit counts memory too, so it can't be below the memory limit. Use -1 for unlimited swap.";
    /// A capability name Linux doesn't have.
    RUN_CAPABILITY = "errors.run.capability" => "\"{name}\" isn't a Linux capability. Use a name like NET_ADMIN or SYS_TIME, or ALL.";
    /// A device path that isn't absolute.
    RUN_DEVICE_PATH = "errors.run.devicePath" => "Device paths must be absolute, like /dev/dri or /dev/ttyUSB0.";
    /// Device permissions other than some of r, w and m.
    RUN_DEVICE_PERMISSIONS = "errors.run.devicePermissions" => "Device permissions are some of r (read), w (write) and m (create device files), like rwm.";
    /// Asking for 0 GPUs.
    RUN_GPU_COUNT = "errors.run.gpuCount" => "Request at least one GPU.";
    /// Choosing GPUs by id without listing any.
    RUN_GPU_IDS = "errors.run.gpuIds" => "List the GPUs to use, by index or UUID.";
    /// A hostname that isn't a DNS name.
    RUN_HOSTNAME = "errors.run.hostname" => "\"{name}\" isn't a valid hostname. Use letters, digits and hyphens, with dots between parts.";
    /// A domain name that isn't a DNS name.
    RUN_DOMAIN_NAME = "errors.run.domainName" => "\"{name}\" isn't a valid domain name. Use letters, digits and hyphens, with dots between parts.";
    /// A DNS search domain that isn't a DNS name.
    RUN_SEARCH_DOMAIN = "errors.run.searchDomain" => "\"{name}\" isn't a valid search domain. Use letters, digits and hyphens, with dots between parts.";
    /// A DNS server given by name instead of address.
    RUN_DNS_SERVER = "errors.run.dnsServer" => "\"{address}\" isn't an IP address. DNS servers are given by address, like 1.1.1.1.";
    /// An `/etc/hosts` entry that isn't `name:address`.
    RUN_EXTRA_HOST = "errors.run.extraHost" => "\"{entry}\" isn't a name:address host entry, like nas.home:192.168.1.10 or host.docker.internal:host-gateway.";
    /// A healthcheck without a command.
    RUN_HEALTHCHECK_COMMAND = "errors.run.healthcheckCommand" => "Enter the healthcheck's command, or turn off the image's healthcheck.";
    /// A healthcheck interval of 0.
    RUN_HEALTHCHECK_INTERVAL = "errors.run.healthcheckInterval" => "The healthcheck's interval must be above 0.";
    /// A healthcheck timeout of 0.
    RUN_HEALTHCHECK_TIMEOUT = "errors.run.healthcheckTimeout" => "The healthcheck's timeout must be above 0.";
    /// A log driver that's empty or has spaces.
    RUN_LOG_DRIVER = "errors.run.logDriver" => "Choose a log driver, like json-file or local.";

    // Podman pods

    /// A pod that isn't there any more.
    POD_GONE = "errors.pods.gone" => "This pod doesn't exist any more.";
    POD_GONE_HINT = "errors.pods.gone.hint" => "It may have been removed from a terminal. The list shows the pods there are now.";
    /// Starting a running pod, stopping a stopped one…
    POD_ALREADY = "errors.pods.already" => "The pod is already in that state.";
    /// Some containers of a pod failed to start, stop, restart, pause or resume.
    /// One message per action, so each language words its own verb.
    POD_START_FAILED = "errors.pods.startFailed" => "Podman couldn't start every container of the pod.";
    POD_STOP_FAILED = "errors.pods.stopFailed" => "Podman couldn't stop every container of the pod.";
    POD_RESTART_FAILED = "errors.pods.restartFailed" => "Podman couldn't restart every container of the pod.";
    POD_PAUSE_FAILED = "errors.pods.pauseFailed" => "Podman couldn't pause every container of the pod.";
    POD_UNPAUSE_FAILED = "errors.pods.unpauseFailed" => "Podman couldn't unpause every container of the pod.";
    POD_ACTION_FAILED_HINT = "errors.pods.actionFailed.hint" => "Details names the containers and why. Open them on the Containers page to see their logs.";
    /// Removing a pod with running containers without stopping them.
    POD_RUNNING = "errors.pods.running" => "The pod has running containers.";
    POD_RUNNING_HINT = "errors.pods.running.hint" => "Stop the pod first, or remove it with its running containers.";
    /// A pod name Podman doesn't accept.
    POD_NAME = "errors.pods.name" => "\"{name}\" isn't a valid pod name. Start with a letter or digit, then use letters, digits, _, . or -.";
    /// Creating a pod under a name another pod has.
    POD_NAME_IN_USE = "errors.pods.nameInUse" => "A pod named {name} already exists.";
    POD_NAME_IN_USE_HINT = "errors.pods.nameInUse.hint" => "Choose another name, or remove the other pod first.";
    /// Publishing a port on a container that runs in a pod.
    POD_CONTAINER_PORTS = "errors.pods.containerPorts" => "Containers in a pod can't publish ports: the pod publishes them for all its containers.";
    POD_CONTAINER_PORTS_HINT = "errors.pods.containerPorts.hint" => "Remove the ports, and publish them when you create the pod instead.";
    /// Setting what a pod's containers share (network, hostname, DNS) on one of them.
    POD_CONTAINER_SHARED = "errors.pods.containerShared" => "Containers in a pod share its network, hostname and DNS settings, so they can't set their own.";
    /// GPUs for a container run into a pod.
    POD_CONTAINER_GPUS = "errors.pods.containerGpus" => "Homeport can't give GPUs to a container in a pod yet.";
    /// Running a copy of a container into a pod.
    POD_CONTAINER_BASED_ON = "errors.pods.containerBasedOn" => "Only new containers can be run into a pod, not copies or edits of existing ones.";

    // Quadlet units (ADR 0019)

    /// An action the Host's Podman is too old for.
    PODMAN_TOO_OLD = "errors.quadlets.podmanTooOld" => "This needs Podman {needed} or later, and the host runs Podman {version}.";
    PODMAN_TOO_OLD_HINT = "errors.quadlets.podmanTooOld.hint" => "Update Podman on the host to {needed} or later to do this from Homeport, or use podman quadlet in a terminal there.";
    /// A unit that isn't there any more.
    QUADLET_GONE = "errors.quadlets.gone" => "{name} isn't installed any more.";
    QUADLET_GONE_HINT = "errors.quadlets.gone.hint" => "It may have been removed from a terminal. The list shows the units there are now.";
    /// A unit file name without a Quadlet extension.
    QUADLET_NAME = "errors.quadlets.name" => "\"{name}\" isn't a Quadlet unit name. Use a name like web.container, ending in .container, .pod, .volume, .network, .kube, .image, .build or .artifact.";
    /// An extra file name with a folder in it.
    QUADLET_EXTRA_NAME = "errors.quadlets.extraName" => "\"{name}\" can't be installed beside a unit. Use a plain file name, without folders.";
    /// Two files under one name.
    QUADLET_EXTRA_TWICE = "errors.quadlets.extraTwice" => "{name} is given twice. Each file beside a unit needs its own name.";
    /// An application folder name Podman won't take.
    QUADLET_APP = "errors.quadlets.app" => "\"{app}\" isn't a valid application name. Use letters, digits, _, ., @ or -.";
    /// A unit file without its kind's section.
    QUADLET_SECTION_MISSING = "errors.quadlets.sectionMissing" => "{name} has no {section} section, so Quadlet would skip it.";
    QUADLET_SECTION_MISSING_HINT = "errors.quadlets.sectionMissing.hint" => "Add a {section} line, with the unit's settings under it.";
    /// Installing under a name another unit has.
    QUADLET_EXISTS = "errors.quadlets.exists" => "A unit named {name} is already installed.";
    QUADLET_EXISTS_HINT = "errors.quadlets.exists.hint" => "Choose another name, or edit the installed unit instead.";
    /// Podman refused a unit's files.
    QUADLET_REFUSED = "errors.quadlets.refused" => "Podman refused to install {name}.";
    QUADLET_REFUSED_HINT = "errors.quadlets.refused.hint" => "Details has Podman's reason.";
    /// Podman's Quadlet API without systemd to talk to.
    QUADLET_NO_SYSTEMD = "errors.quadlets.noSystemd" => "Podman couldn't reach systemd on the host.";
    QUADLET_NO_SYSTEMD_HINT = "errors.quadlets.noSystemd.hint" => "Quadlet units need systemd to run the Podman service the host's socket belongs to. Details has Podman's words.";
    /// Removing a running unit without stopping it.
    /// Saving an edit of a unit file that changed on the Host since it was opened.
    QUADLET_CHANGED = "errors.quadlets.changed" => "{name} changed on the Host since you opened it.";
    QUADLET_CHANGED_HINT = "errors.quadlets.changed.hint" => "Close the editor and open it again to see the change, or overwrite it with your version.";
    QUADLET_RUNNING = "errors.quadlets.running" => "{name} is running.";
    QUADLET_RUNNING_HINT = "errors.quadlets.running.hint" => "Choose to stop it as it's removed, or stop it first.";
    /// Podman reported an error removing a unit.
    QUADLET_NOT_REMOVED = "errors.quadlets.notRemoved" => "Podman couldn't remove {name}.";
    QUADLET_NOT_REMOVED_HINT = "errors.quadlets.notRemoved.hint" => "Details has Podman's reason. The list shows whether it's still installed.";
    /// A service name that isn't a systemd service's.
    QUADLET_UNIT_NAME = "errors.quadlets.unitName" => "\"{name}\" isn't a systemd service name.";
    /// Starting a unit on a connection that runs no commands.
    QUADLET_NO_COMMANDS = "errors.quadlets.noCommands" => "Homeport can only start and stop units over SSH, or for a Podman on this computer.";
    QUADLET_NO_COMMANDS_HINT = "errors.quadlets.noCommands.hint" => "Podman's API can't start or stop a unit's service, so Homeport runs systemctl where Podman runs. Connect to this host over SSH to do it from Homeport.";
    /// The user's systemd instance isn't running.
    QUADLET_NO_USER_SYSTEMD = "errors.quadlets.noUserSystemd" => "The user's systemd isn't running on the host, so its units can't be started or stopped.";
    QUADLET_NO_USER_SYSTEMD_HINT = "errors.quadlets.noUserSystemd.hint" => "Let the user's units run without a login session by enabling lingering, as that user on the host:";
    /// systemd refused the user's rights.
    QUADLET_NOT_ALLOWED = "errors.quadlets.notAllowed" => "systemd didn't let this user manage {unit}.";
    QUADLET_NOT_ALLOWED_HINT = "errors.quadlets.notAllowed.hint" => "System units need a user allowed to manage them, such as root; Homeport never uses sudo. Connect as that user, or use rootless Podman's user units.";
    /// systemd doesn't know a unit's service.
    QUADLET_UNIT_NOT_LOADED = "errors.quadlets.unitNotLoaded" => "systemd doesn't know {unit}.";
    QUADLET_UNIT_NOT_LOADED_HINT = "errors.quadlets.unitNotLoaded.hint" => "Quadlet skips files it can't read. Check the unit file, then systemctl daemon-reload on the host shows why.";
    /// systemd failed to start, stop or restart a service.
    /// One message per action, so each language words its own verb.
    QUADLET_START_FAILED = "errors.quadlets.startFailed" => "systemd couldn't start {unit}.";
    QUADLET_STOP_FAILED = "errors.quadlets.stopFailed" => "systemd couldn't stop {unit}.";
    QUADLET_RESTART_FAILED = "errors.quadlets.restartFailed" => "systemd couldn't restart {unit}.";
    QUADLET_ACTION_FAILED_HINT = "errors.quadlets.actionFailed.hint" => "Details has what systemctl said. journalctl -u with the unit's name on the host shows its log.";
    /// systemctl didn't finish in time.
    QUADLET_TIMEOUT = "errors.quadlets.timeout" => "{unit} didn't finish in time.";
    QUADLET_TIMEOUT_HINT = "errors.quadlets.timeout.hint" => "It may still be pulling its image. Refresh the list in a moment to see its state.";
    /// systemctl couldn't be run at all.
    QUADLET_COMMAND_FAILED = "errors.quadlets.commandFailed" => "Homeport couldn't run systemctl for {unit} on the host.";

    // Images

    /// Pushing an image under a name that isn't a repository and tag.
    PUSH_NAME = "errors.images.pushName" => "Enter a name like registry.example.com/team/app:tag.";
    /// The registry refused a push.
    PUSH_REFUSED = "errors.images.pushRefused" => "The registry refused the push.";
    PUSH_REFUSED_HINT = "errors.images.pushRefused.hint" => "Add a login that may push to this repository in Settings → Registries.";
    /// Tagging an image with a name that isn't `repo:tag`.
    TAG_NAME = "errors.images.tagName" => "Enter a name like repo:tag.";
    /// The registry answered a digest check without a digest.
    REGISTRY_NO_DIGEST = "errors.images.noDigest" => "The registry didn't say which image it has.";

    // Volumes and networks

    /// Creating a volume without a name.
    VOLUME_NAME = "errors.volumes.name" => "Enter a volume name.";
    /// Opening a volume on an Engine that runs Windows containers.
    HELPER_WINDOWS = "errors.volumes.helperWindows" => "Homeport can't open volumes on Docker Engines that run Windows containers.";
    /// The helper's image isn't on the Host and pulling it failed.
    HELPER_IMAGE_UNAVAILABLE = "errors.volumes.helperImageUnavailable" => "Homeport couldn't get {image} onto this Host to open the volume.";
    HELPER_IMAGE_UNAVAILABLE_HINT = "errors.volumes.helperImageUnavailable.hint" => "Pull it on the Host, or save it elsewhere with docker save and load it on the Host with docker load:";
    /// A volume path that doesn't exist (any more).
    VOLUME_PATH_MISSING = "errors.volumes.pathMissing" => "{path} doesn't exist in the volume.";
    /// A volume path with `..` in it.
    VOLUME_PATH_OUTSIDE = "errors.volumes.pathOutside" => "Paths can't leave the volume.";
    /// Restoring into a new volume whose name is taken.
    VOLUME_EXISTS = "errors.volumes.exists" => "A volume named {volume} already exists.";
    VOLUME_EXISTS_HINT = "errors.volumes.exists.hint" => "Choose another name, or restore into the existing volume from its row.";
    VOLUME_EXISTS_CLONE_HINT = "errors.volumes.exists.cloneHint" => "Choose another name for the copy.";
    /// The copy into a clone failed; the reason is in Details.
    CLONE_FAILED = "errors.volumes.cloneFailed" => "Homeport couldn't copy {source}.";
    CLONE_REMOVED_HINT = "errors.volumes.cloneFailed.removedHint" => "The incomplete copy, {target}, was removed. The output in Details says what went wrong.";
    CLONE_LEFT_HINT = "errors.volumes.cloneFailed.leftHint" => "The incomplete copy, {target}, couldn't be removed; delete it before using it. The output in Details says what went wrong.";
    /// Restoring into a volume that holds files, without replacing them.
    VOLUME_NOT_EMPTY = "errors.volumes.notEmpty" => "{volume} already holds files.";
    /// Emptying a volume before a restore failed; the reason is in Details.
    VOLUME_NOT_EMPTIED = "errors.volumes.notEmptied" => "Homeport couldn't empty {volume} before restoring into it.";
    /// Restoring into a volume running containers use.
    VOLUME_USED_BY_RUNNING = "errors.volumes.usedByRunning" => "{volume} is used by running containers: {containers}.";
    VOLUME_USED_BY_RUNNING_HINT = "errors.volumes.usedByRunning.hint" => "Stop them first, so nothing writes to the volume while it's restored.";
    /// Restoring a volume from a file that isn't a tar archive.
    VOLUME_NOT_A_TAR = "errors.volumes.notATar" => "{path} isn't a tar archive.";
    VOLUME_NOT_A_TAR_HINT = "errors.volumes.notATar.hint" => "Choose a file made by Back up, or a tar archive of a folder's contents, such as one made by tar -cf backup.tar -C folder .";
    /// Creating a network without a name.
    NETWORK_NAME = "errors.networks.name" => "Enter a network name.";

    // Build

    /// A build folder that isn't a folder.
    BUILD_NOT_A_FOLDER = "errors.build.notAFolder" => "{path} isn't a folder on this computer.";
    /// A Dockerfile path leaving the build folder.
    BUILD_DOCKERFILE_OUTSIDE = "errors.build.dockerfileOutside" => "The Dockerfile must be a path inside the build folder.";
    /// The build folder has no file at the Dockerfile path.
    BUILD_NO_DOCKERFILE = "errors.build.noDockerfile" => "There's no {dockerfile} in {folder}.";
    /// A build tag that isn't an image name.
    BUILD_TAG = "errors.build.tag" => "\"{tag}\" isn't a valid name. Use a name like app:1.0 or registry.example.com/team/app:tag.";
    /// A build argument without a name, or with `=` in it.
    BUILD_ARG_NAME = "errors.build.argName" => "Each build argument needs a name without \"=\".";
    /// The build failed before any step.
    BUILD_FAILED = "errors.build.failed" => "The build failed.";
    /// The build failed at a numbered step with its instruction.
    BUILD_FAILED_AT_STEP = "errors.build.failedAtStep" => "The build failed at step {step} ({instruction}).";
    /// The build failed at a step the output gave no instruction for.
    BUILD_FAILED_AT = "errors.build.failedAt" => "The build failed at step {step}.";
    BUILD_FAILED_HINT = "errors.build.failed.hint" => "The end of the build's output is in Details.";

    // Save, load and import

    /// Saving without choosing an image.
    TRANSFER_NOTHING_TO_SAVE = "errors.transfer.nothingToSave" => "Choose at least one image to save.";
    /// The Engine streamed no bytes for a save or export.
    TRANSFER_EMPTY_ARCHIVE = "errors.transfer.emptyArchive" => "The Docker Engine sent an empty archive.";
    /// Importing a filesystem under a name that isn't `repo:tag`.
    TRANSFER_IMPORT_NAME = "errors.transfer.importName" => "Name the image like repo:tag, such as debian-rootfs:12.";
    /// Importing a file that isn't a tar of a filesystem.
    TRANSFER_NOT_A_FILESYSTEM = "errors.transfer.notAFilesystem" => "{path} isn't a tar archive the Docker Engine can import.";
    TRANSFER_NOT_A_FILESYSTEM_HINT = "errors.transfer.notAFilesystem.hint" => "Choose a .tar of a root filesystem, such as one made by Export filesystem or docker export. Image archives made by Save go through Load instead.";
    /// Loading a file that isn't an image archive.
    TRANSFER_NOT_AN_IMAGE_ARCHIVE = "errors.transfer.notAnImageArchive" => "{path} isn't an image archive the Docker Engine can load.";
    TRANSFER_NOT_AN_IMAGE_ARCHIVE_HINT = "errors.transfer.notAnImageArchive.hint" => "Choose a .tar made by docker save or by Save in Homeport, or a compressed one (.tar.gz).";

    // Swarms

    /// A node that left the Swarm since the list was read.
    NODE_GONE = "errors.swarm.nodeGone" => "This node is no longer in the Swarm.";
    /// Demoting the only manager that takes part in the Swarm.
    NODE_LAST_MANAGER = "errors.swarm.lastManager" => "This is the Swarm's last reachable manager, so it can't be demoted.";
    NODE_LAST_MANAGER_HINT = "errors.swarm.lastManager.hint" => "Promote another node first. A Swarm without a manager can't be managed at all.";
    /// Draining the last node that can run tasks.
    NODE_LAST_ACTIVE = "errors.swarm.lastActive" => "Draining this node would leave no node to run the Swarm's tasks.";
    /// A node label with an empty key.
    NODE_LABEL_KEY = "errors.swarm.labelKey" => "Each label needs a key.";
    /// Removing a manager node.
    NODE_REMOVE_MANAGER = "errors.swarm.removeManager" => "A manager can't be removed. Demote it to a worker first.";
    /// Removing a node that's still up without force.
    NODE_REMOVE_READY = "errors.swarm.removeReady" => "This node is still up.";
    NODE_REMOVE_READY_HINT = "errors.swarm.removeReady.hint" => "Have it leave the Swarm first, or remove it with Force.";
    /// Rolling back a service that was never updated.
    SERVICE_NO_PREVIOUS = "errors.swarm.serviceNoPrevious" => "This service has no earlier version to roll back to.";
    /// Scaling a global service or a job.
    SERVICE_NOT_REPLICATED = "errors.swarm.serviceNotReplicated" => "Only replicated services can be scaled; a global service runs one task on every node.";
    /// An absurd number of replicas.
    SERVICE_REPLICAS = "errors.swarm.serviceReplicas" => "Choose at most 10000 replicas.";
    /// A secret or config name Docker refuses.
    OBJECT_NAME = "errors.swarm.objectName" => "Names start with a letter or digit, and hold up to 64 letters, digits, dots, hyphens and underscores.";
    /// A secret or config without a value.
    OBJECT_EMPTY = "errors.swarm.objectEmpty" => "Enter a value, or load it from a file.";
    /// A secret or config over the Swarm's 500 KB limit.
    OBJECT_TOO_LARGE = "errors.swarm.objectTooLarge" => "Secrets and configs can hold at most 500 KB.";
    /// A secret or config whose name another one has.
    OBJECT_NAME_IN_USE = "errors.swarm.objectNameInUse" => "That name is already used in this Swarm.";
    OBJECT_NAME_IN_USE_HINT = "errors.swarm.objectNameInUse.hint" => "Secrets and configs can't be changed: create one under a new name, such as with a version at the end, and point the services at it.";
    /// Removing a secret or config a service uses.
    OBJECT_IN_USE = "errors.swarm.objectInUse" => "A service still uses this.";
    OBJECT_IN_USE_HINT = "errors.swarm.objectInUse.hint" => "Used by {services}. Remove it from them, or remove them, first.";
    OBJECT_IN_USE_HINT_UNKNOWN = "errors.swarm.objectInUse.hintUnknown" => "Remove it from the services that use it, or remove them, first.";
    /// A service name that isn't a DNS label.
    SERVICE_NAME = "errors.swarm.serviceName" => "Service names hold up to 63 letters, digits, hyphens and underscores, and start and end with a letter or digit.";
    /// Another service has this name.
    SERVICE_NAME_IN_USE = "errors.swarm.serviceNameInUse" => "A service with that name already runs on this Swarm.";
    SERVICE_NAME_IN_USE_HINT = "errors.swarm.serviceNameInUse.hint" => "Pick another name, or edit that service instead.";
    /// A port without a container port.
    SERVICE_PORT_TARGET = "errors.swarm.servicePortTarget" => "Each port needs the container port it goes to.";
    /// The same published port twice in one service.
    SERVICE_PORT_TWICE = "errors.swarm.servicePortTwice" => "Port {port} is published twice.";
    /// A host-mode port without a published port.
    SERVICE_HOST_PORT_NEEDED = "errors.swarm.serviceHostPortNeeded" => "Ports published on the node itself need a port number.";
    /// A secret or config named in a service that the Swarm doesn't have.
    OBJECT_MISSING = "errors.swarm.objectMissing" => "This Swarm has no secret or config named {name}.";
    /// Initialising or joining a Swarm on a Host already in one.
    ALREADY_IN_SWARM = "errors.swarm.alreadyIn" => "This host is already in a Swarm.";
    ALREADY_IN_SWARM_HINT = "errors.swarm.alreadyIn.hint" => "Have it leave that Swarm first.";
    /// Leaving a Swarm on a Host that isn't in one.
    NOT_IN_SWARM = "errors.swarm.notIn" => "This host isn't in a Swarm.";
    /// Leaving as a manager without force.
    LEAVE_NEEDS_FORCE = "errors.swarm.leaveNeedsForce" => "A manager leaves the Swarm only with Force.";
    /// The Engine can't tell which address other nodes reach it at.
    ADVERTISE_NEEDED = "errors.swarm.advertiseNeeded" => "Choose the address other nodes reach this host at.";
    ADVERTISE_NEEDED_HINT = "errors.swarm.advertiseNeeded.hint" => "The machine has several addresses. Enter the one the other nodes can reach, such as its LAN address.";
    /// Joining without a token or an address.
    JOIN_NEEDS_TOKEN = "errors.swarm.joinNeedsToken" => "Enter the join token and a manager's address.";
    /// A join token the managers refuse.
    JOIN_TOKEN_INVALID = "errors.swarm.joinTokenInvalid" => "The managers refused this join token.";
    /// The node couldn't reach a manager.
    JOIN_FAILED = "errors.swarm.joinFailed" => "This host couldn't reach the Swarm's managers.";
    JOIN_FAILED_HINT = "errors.swarm.joinFailed.hint" => "Nodes talk to each other directly: open 2377/tcp (managers), 7946/tcp and 7946/udp, and 4789/udp between every node, in their firewalls and your network's.";
}

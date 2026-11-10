//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    // Connecting.

    /// containerd didn't answer its first call in time.
    CONNECT_TIMEOUT = "errors.containerd.connectTimeout" => "containerd on {name} didn't answer in time.";
    /// Something answered on the socket, but not as containerd.
    NOT_CONTAINERD = "errors.containerd.notContainerd" => "The socket of {name} doesn't answer as containerd.";
    NOT_CONTAINERD_HINT = "errors.containerd.notContainerd.hint" => "Check the socket path on the host page: containerd's is usually /run/containerd/containerd.sock, and k3s's /run/k3s/containerd/containerd.sock.";
    /// A containerd older than Homeport connects to.
    TOO_OLD_TO_CONNECT = "errors.containerd.tooOldToConnect" => "{name} runs containerd {version}; Homeport needs {minimum} or later.";
    TOO_OLD_HINT = "errors.containerd.tooOld.hint" => "Update containerd to {minimum} or later from your distribution or containerd's releases.";
    /// A containerd older than pulling needs.
    TOO_OLD_TO_PULL = "errors.containerd.tooOldToPull" => "{name} runs containerd {version}, which can't pull images for Homeport; pulling needs {minimum} or later.";

    // containerd's answers.

    /// containerd has no such object.
    NOT_FOUND = "errors.containerd.notFound" => "containerd has no such object; it may have just been removed.";
    /// An object with this name exists already.
    ALREADY_EXISTS = "errors.containerd.alreadyExists" => "containerd already has an object with this name.";
    /// containerd refused because of the object's state.
    REFUSED = "errors.containerd.refused" => "containerd refused, because of the state the object is in.";
    /// containerd refused the user.
    PERMISSION_DENIED = "errors.containerd.permissionDenied" => "containerd refused to do this for Homeport.";
    /// The connection broke during a call.
    UNAVAILABLE = "errors.containerd.unavailable" => "The connection to containerd broke.";
    UNAVAILABLE_HINT = "errors.containerd.unavailable.hint" => "containerd may have restarted. Homeport reconnects on its own; try again in a moment.";
    /// A call took too long.
    TIMED_OUT = "errors.containerd.timedOut" => "containerd didn't answer in time.";
    /// containerd doesn't offer a service Homeport called.
    UNIMPLEMENTED = "errors.containerd.unimplemented" => "This containerd doesn't offer what Homeport asked for.";
    UNIMPLEMENTED_HINT = "errors.containerd.unimplemented.hint" => "Its version or configuration leaves the service out. Update containerd, or check that the plugin isn't disabled in its config.toml.";
    /// containerd rejected the request's arguments.
    INVALID = "errors.containerd.invalid" => "containerd rejected what Homeport sent.";
    /// containerd failed in a way not told apart.
    FAILED = "errors.containerd.failed" => "containerd couldn't do this.";
    DETAILS_HINT = "errors.containerd.details.hint" => "Details has containerd's own answer.";

    // Containers and tasks.

    /// A container that isn't in the namespace.
    CONTAINER_NOT_FOUND = "errors.containerd.containerNotFound" => "There's no container {id} in this namespace; it may have just been removed.";
    /// Starting a container whose task already runs.
    TASK_ALREADY_RUNNING = "errors.containerd.taskAlreadyRunning" => "{id} is already running.";
    /// Stopping, pausing or killing a container without a running task.
    TASK_NOT_RUNNING = "errors.containerd.taskNotRunning" => "{id} isn't running.";
    /// Removing a container whose task runs, without forcing.
    TASK_STILL_RUNNING = "errors.containerd.taskStillRunning" => "{id} is running, so it can't be removed.";
    TASK_STILL_RUNNING_HINT = "errors.containerd.taskStillRunning.hint" => "Stop it first, or remove it with Force, which kills it.";
    /// A signal name Linux doesn't have.
    SIGNAL_UNKNOWN = "errors.containerd.signalUnknown" => "{signal} isn't a signal; use a name like SIGHUP or a number from 1 to 64.";
    /// A task still alive after SIGKILL.
    TASK_WONT_STOP = "errors.containerd.taskWontStop" => "{id} didn't stop, even after SIGKILL.";
    TASK_WONT_STOP_HINT = "errors.containerd.taskWontStop.hint" => "Its process may be stuck in the kernel (uninterruptible I/O, a hung mount). Check it on the host with ps, then try again.";

    // Logs and exec, which run commands on the Host's machine (ADR 0020).

    /// A container whose output no tool kept.
    LOGS_NOT_RECORDED = "errors.containerd.logsNotRecorded" => "This container's output isn't recorded.";
    LOGS_NOT_RECORDED_HINT = "errors.containerd.logsNotRecorded.hint" => "containerd keeps no logs: the tool that starts a task decides where its output goes. ctr and most clients send it nowhere; nerdctl and Kubernetes write it to a file Homeport reads.";
    /// nerdctl sent the output to a log driver Homeport can't read.
    LOGS_DRIVER = "errors.containerd.logsDriver" => "nerdctl sends this container's output to {driver}, which Homeport can't read.";
    LOGS_DRIVER_HINT = "errors.containerd.logsDriver.hint" => "Read it where {driver} keeps it, or run the container again with nerdctl's default json-file log driver.";
    /// Logs of a containerd Host whose connection can't run commands.
    LOGS_NEED_SHELL = "errors.containerd.logsNeedShell" => "Homeport can't read the logs of {name}'s containers from this kind of connection.";
    NEEDS_SHELL_HINT = "errors.containerd.needsShell.hint" => "containerd's logs and terminals need its machine's files, which Homeport reaches over SSH, or on this computer for a local Linux socket. Connect to the host over SSH to use them.";
    /// tail failed on the Host's machine.
    LOGS_UNREADABLE = "errors.containerd.logsUnreadable" => "Homeport couldn't read this container's log file.";
    LOGS_UNREADABLE_HINT = "errors.containerd.logsUnreadable.hint" => "Log files usually belong to root. Connect as a user who may read them; Details has what the machine answered.";

    /// A terminal on a containerd Host whose connection can't run commands.
    EXEC_NEEDS_SHELL = "errors.containerd.execNeedsShell" => "Homeport can't open a terminal in {name}'s containers from this kind of connection.";
    /// The script that makes the FIFOs failed on the Host's machine.
    EXEC_SETUP_FAILED = "errors.containerd.execSetupFailed" => "Homeport couldn't prepare the terminal on the host.";
    EXEC_SETUP_FAILED_HINT = "errors.containerd.execSetupFailed.hint" => "It runs mktemp, mkfifo and cat there, in the temporary folder. Details has what the machine answered.";
    /// A command with an unclosed quote.
    EXEC_UNCLOSED_QUOTE = "errors.containerd.execUnclosedQuote" => "The command has an unclosed quote.";
    /// A user that isn't a number.
    EXEC_USER_NOT_NUMERIC = "errors.containerd.execUserNotNumeric" => "\"{user}\" isn't a user ID: give a number, or uid:gid.";
    /// A container spec without a process to start from.
    EXEC_NO_SPEC = "errors.containerd.execNoSpec" => "This container's spec has no process to start a terminal from.";
    /// The container has no shell, or not the program asked for.
    EXEC_NO_SHELL = "errors.containerd.execNoShell" => "The container has no such program.";
    EXEC_NO_SHELL_HINT = "errors.containerd.execNoShell.hint" => "Images without a shell (distroless, scratch) can't open one; run a program the image has instead.";
    /// Input or a resize for a terminal that already closed.
    EXEC_ENDED = "errors.containerd.execEnded" => "This terminal session has ended.";

    // Images.

    /// Removing an image a container was made from.
    IMAGE_IN_USE = "errors.containerd.imageInUse" => "{name} is the image of {containers}, so it can't be removed.";
    IMAGE_IN_USE_HINT = "errors.containerd.imageInUse.hint" => "Remove those containers first. containerd only frees the image's content once nothing holds it.";
    /// Removing an image that's gone.
    IMAGE_NOT_FOUND = "errors.containerd.imageNotFound" => "There's no image {name} in this namespace.";
    /// A pull of an image or tag the registry doesn't have.
    PULL_NOT_FOUND = "errors.containerd.pullNotFound" => "The registry has no such image or tag.";
    /// A pull the registry refused without credentials.
    PULL_AUTH_REQUIRED = "errors.containerd.pullAuthRequired" => "The registry refused to let Homeport pull this image.";

    // Snapshots.

    /// A snapshot that isn't there.
    SNAPSHOT_NOT_FOUND = "errors.containerd.snapshotNotFound" => "There's no snapshot {key} in this snapshotter.";
    /// Removing a snapshot others are built on.
    SNAPSHOT_HAS_CHILD = "errors.containerd.snapshotHasChild" => "A snapshot is built on {key}, so it can't be removed.";
    SNAPSHOT_HAS_CHILDREN = "errors.containerd.snapshotHasChildren" => "{count} snapshots are built on {key}, so it can't be removed.";
    /// Removing a snapshot a container or an image uses.
    SNAPSHOT_USED = "errors.containerd.snapshotUsed" => "{key} is used by {users}, so it can't be removed.";
    SNAPSHOT_IN_USE_HINT = "errors.containerd.snapshotInUse.hint" => "Remove what uses it first: the container, or the image whose layers it holds. Snapshots nothing uses can be removed here.";

    // Namespaces.

    /// Creating a namespace under a name another one has.
    NAMESPACE_EXISTS = "errors.containerd.namespaceExists" => "A namespace named {name} already exists.";
    /// Removing a namespace that holds something.
    NAMESPACE_STILL_HOLDS = "errors.containerd.namespaceStillHolds" => "The namespace {name} still holds {holds}, so it can't be removed.";
    NAMESPACE_NOT_EMPTY = "errors.containerd.namespaceNotEmpty" => "The namespace {name} isn't empty, so it can't be removed.";
    NAMESPACE_NOT_EMPTY_HINT = "errors.containerd.namespaceNotEmpty.hint" => "Remove its containers, images and snapshots first; containerd only removes empty namespaces.";
    /// A namespace name containerd wouldn't accept.
    NAMESPACE_INVALID = "errors.containerd.namespaceInvalid" => "\"{name}\" isn't a namespace name: use letters and digits, separated by single dots, dashes or underscores, up to 76 characters.";
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use homeport_core::error::arguments;

    #[test]
    fn keys_are_unique_and_arguments_are_plain_names() {
        let mut keys = BTreeSet::new();
        for m in super::ALL {
            assert!(m.key.starts_with("errors.containerd."), "{}", m.key);
            assert!(keys.insert(m.key), "{} is declared twice", m.key);
            for arg in arguments(m.en) {
                assert!(
                    !arg.is_empty() && arg.chars().all(|c| c.is_ascii_alphanumeric()),
                    "{}: {{{arg}}}",
                    m.key
                );
            }
        }
    }
}

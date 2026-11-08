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

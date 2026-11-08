//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    // Connecting.

    /// Something answered on the socket or address, but not as Incus.
    NOT_INCUS = "errors.incus.notIncus" => "{name} doesn't answer as Incus.";
    NOT_INCUS_HINT = "errors.incus.notIncus.hint" => "Check the socket path or address on the host page: Incus's socket is usually /var/lib/incus/unix.socket, and its HTTPS address port 8443.";
    /// An Incus that lacks an API extension Homeport needs.
    EXTENSION_MISSING = "errors.incus.extensionMissing" => "The Incus on {name} lacks the {extension} API extension Homeport needs.";
    EXTENSION_MISSING_HINT = "errors.incus.extensionMissing.hint" => "Update Incus to 6.0 or later from your distribution or the Zabbly packages.";
    /// Incus answered over HTTPS but doesn't trust Homeport's certificate.
    UNTRUSTED = "errors.incus.untrusted" => "{name} doesn't trust Homeport's certificate.";
    UNTRUSTED_HINT = "errors.incus.untrusted.hint" => "Its certificate may have been removed from the trust store. Create a new trust token on the server and add the host again with it.";
    /// A request that took too long.
    TIMED_OUT = "errors.incus.timedOut" => "Incus didn't answer in time.";
    /// An answer that isn't the JSON Incus sends.
    UNREADABLE = "errors.incus.unreadable" => "Incus sent an answer Homeport couldn't read.";

    // Trust tokens (ADR 0021).

    /// Text that isn't an Incus trust token.
    TOKEN_INVALID = "errors.incus.tokenInvalid" => "This isn't an Incus trust token.";
    TOKEN_INVALID_HINT = "errors.incus.tokenInvalid.hint" => "Paste the whole token this prints on the Incus server, a single line of letters and digits:";
    /// A token past its expiry.
    TOKEN_EXPIRED = "errors.incus.tokenExpired" => "This trust token expired at {time}.";
    TOKEN_EXPIRED_HINT = "errors.incus.tokenExpired.hint" => "Make a new one on the Incus server, and paste it within its lifetime:";
    /// Incus refused the token.
    TOKEN_REFUSED = "errors.incus.tokenRefused" => "Incus refused the trust token.";
    TOKEN_REFUSED_HINT = "errors.incus.tokenRefused.hint" => "A token works once: it may have been used already, or revoked with incus config trust revoke-token. Make a new one on the server:";
    /// No address of the token answered.
    NOT_LISTENING = "errors.incus.notListening" => "Homeport couldn't reach Incus at {addresses}.";
    NOT_LISTENING_HINT = "errors.incus.notListening.hint" => "Incus may not listen on the network, or a firewall may block port 8443. On the server, have it listen on every address:";
    /// An address presented another certificate than the token's.
    TOKEN_FINGERPRINT_MISMATCH = "errors.incus.tokenFingerprintMismatch" => "{address} presented a certificate with fingerprint {presented}, not the {expected} the token names.";
    TOKEN_FINGERPRINT_MISMATCH_HINT = "errors.incus.tokenFingerprintMismatch.hint" => "Something other than this Incus answers at that address, or something intercepts the connection. Check the server's fingerprint with incus info on it before trusting anything.";
    /// Making the client certificate failed.
    CERTIFICATE_FAILED = "errors.incus.certificateFailed" => "Homeport couldn't make its client certificate.";

    // Incus's answers.

    /// Incus has no such object.
    NOT_FOUND = "errors.incus.notFound" => "Incus has no such object; it may have just been removed.";
    /// An object with this name exists already.
    ALREADY_EXISTS = "errors.incus.alreadyExists" => "Incus already has an object with this name.";
    ALREADY_EXISTS_HINT = "errors.incus.alreadyExists.hint" => "Pick another name.";
    /// Incus refused because of the object's state.
    CONFLICT = "errors.incus.conflict" => "Incus refused, because of the state the object is in.";
    /// Incus refused the user.
    FORBIDDEN = "errors.incus.forbidden" => "Incus refused to do this for Homeport.";
    FORBIDDEN_HINT = "errors.incus.forbidden.hint" => "The restricted socket and restricted certificates only reach their own project. Details has Incus's own answer.";
    /// Incus rejected the request's content.
    INVALID = "errors.incus.invalid" => "Incus rejected what Homeport sent.";
    INVALID_HINT = "errors.incus.invalid.hint" => "Details has Incus's own answer, which says what it didn't accept.";
    /// Incus failed in a way not told apart.
    FAILED = "errors.incus.failed" => "Incus couldn't do this.";
    DETAILS_HINT = "errors.incus.details.hint" => "Details has Incus's own answer.";
    /// An object changed since the page loaded it (ETag mismatch).
    CHANGED_SINCE_LOADED = "errors.incus.changedSinceLoaded" => "Someone changed this since Homeport loaded it, so it wasn't saved.";
    CHANGED_SINCE_LOADED_HINT = "errors.incus.changedSinceLoaded.hint" => "Reload it, then make your changes again.";
    /// An object other objects still use.
    IN_USE = "errors.incus.inUse" => "{name} is used by {users}, so it can't be removed.";
    IN_USE_UNNAMED = "errors.incus.inUseUnnamed" => "{name} is in use, so it can't be removed.";
    IN_USE_HINT = "errors.incus.inUse.hint" => "Remove it from what uses it first.";
    /// A disk or pool that filled up.
    DISK_FULL = "errors.incus.diskFull" => "The storage pool has no space left.";
    DISK_FULL_HINT = "errors.incus.diskFull.hint" => "Free space in the pool, delete unused images or snapshots, or grow the pool.";
    /// An operation that didn't finish in time.
    OPERATION_TIMED_OUT = "errors.incus.operationTimedOut" => "Incus is still working on this after {seconds} seconds.";
    OPERATION_TIMED_OUT_HINT = "errors.incus.operationTimedOut.hint" => "It may still finish: refresh in a moment, or follow it with incus operation list on the server.";
    /// An operation that was cancelled.
    OPERATION_CANCELLED = "errors.incus.operationCancelled" => "The operation was cancelled.";
    /// Exec in a virtual machine whose agent doesn't run.
    AGENT_NOT_RUNNING = "errors.incus.agentNotRunning" => "The virtual machine's Incus agent isn't running, so Homeport can't run commands in it.";
    AGENT_NOT_RUNNING_HINT = "errors.incus.agentNotRunning.hint" => "Exec in a virtual machine needs incus-agent, which Incus's own images include and start on boot. Wait for the boot to finish, or use the Console tab.";
    /// A virtual machine on a Host without hardware virtualization.
    VIRTUALIZATION_UNAVAILABLE = "errors.incus.virtualizationUnavailable" => "This host can't run virtual machines.";
    VIRTUALIZATION_UNAVAILABLE_HINT = "errors.incus.virtualizationUnavailable.hint" => "Incus needs KVM (/dev/kvm) and QEMU for virtual machines. Enable virtualization in the firmware, or create a system container instead.";
    /// A stateful snapshot the Instance can't take.
    STATEFUL_UNSUPPORTED = "errors.incus.statefulUnsupported" => "This Instance can't take stateful snapshots.";
    STATEFUL_UNSUPPORTED_HINT = "errors.incus.statefulUnsupported.hint" => "Stateful snapshots need migration.stateful set on the Instance (and, for containers, CRIU on the host). Take a stateless snapshot instead, or set it:";
    /// Acting on an Instance that doesn't run.
    INSTANCE_NOT_RUNNING = "errors.incus.instanceNotRunning" => "The Instance isn't running.";
    INSTANCE_NOT_RUNNING_HINT = "errors.incus.instanceNotRunning.hint" => "Start it first.";
    /// Starting an Instance that already runs.
    INSTANCE_ALREADY_RUNNING = "errors.incus.instanceAlreadyRunning" => "The Instance is already running.";
    /// An image or alias the server doesn't have.
    IMAGE_NOT_FOUND = "errors.incus.imageNotFound" => "The image server has no such image.";
    IMAGE_NOT_FOUND_HINT = "errors.incus.imageNotFound.hint" => "Check the alias, or pick the image from the list: aliases look like debian/12 or ubuntu/24.04/cloud.";
    /// An Instance that isn't in the project.
    INSTANCE_NOT_FOUND = "errors.incus.instanceNotFound" => "There's no Instance {name} in this project; it may have just been deleted.";
    INSTANCE_NOT_FOUND_HINT = "errors.incus.instanceNotFound.hint" => "Refresh the list, or pick another project under the host switcher.";
    /// A graceful stop the guest didn't finish in time.
    STOP_TIMED_OUT = "errors.incus.stopTimedOut" => "{name} didn't shut down within {seconds} seconds.";
    STOP_TIMED_OUT_HINT = "errors.incus.stopTimedOut.hint" => "Its system may be ignoring the shutdown request (virtual machines need ACPI or the Incus agent). Stop it with Force, which cuts it off like pulling the plug.";
    /// Deleting a running Instance.
    INSTANCE_RUNNING = "errors.incus.instanceRunning" => "{name} is running, so it can't be deleted.";
    INSTANCE_RUNNING_HINT = "errors.incus.instanceRunning.hint" => "Stop it first, or delete it with Stop it first checked.";
    /// Stopping a stopped Instance, freezing a frozen one…
    INSTANCE_ALREADY_THERE = "errors.incus.instanceAlreadyThere" => "The Instance is already in that state.";
    /// The project picked isn't there any more.
    PROJECT_NOT_FOUND = "errors.incus.projectNotFound" => "There's no project {project} on {name}.";
    PROJECT_NOT_FOUND_HINT = "errors.incus.projectNotFound.hint" => "Pick another project under the host switcher.";
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use homeport_core::error::arguments;

    #[test]
    fn keys_are_unique_and_arguments_are_plain_names() {
        let mut keys = BTreeSet::new();
        for m in super::ALL {
            assert!(m.key.starts_with("errors.incus."), "{}", m.key);
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

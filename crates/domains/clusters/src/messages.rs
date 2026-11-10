//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    /// Saving a Cluster without a name.
    NAME_EMPTY = "errors.clusters.nameEmpty" => "Give the cluster a name.";
    /// Saving a Cluster under a name another one has.
    NAME_IN_USE = "errors.clusters.nameInUse" => "A cluster named \"{name}\" already exists.";
    NAME_IN_USE_HINT = "errors.clusters.nameInUse.hint" => "Pick another name.";
    /// A Cluster whose API server isn't an http(s) URL.
    SERVER_INVALID = "errors.clusters.serverInvalid" => "\"{server}\" isn't the URL of a Kubernetes API server, like https://127.0.0.1:6443.";
    /// A Cluster of a kind a newer Homeport stored.
    UNKNOWN_KIND = "errors.clusters.unknownKind" => "Homeport doesn't know clusters of kind {kind}.";
    /// Saving a connection or credentials of another kind than the Cluster's.
    KIND_CHANGED = "errors.clusters.kindChanged" => "A cluster can't change kind; add a new one instead.";
    /// A command of one kind of Cluster sent to a Cluster of another.
    WRONG_KIND = "errors.clusters.wrongKind" => "{name} is a {kind} cluster, which this doesn't work with.";
    WRONG_KIND_HINT = "errors.clusters.wrongKind.hint" => "Select a cluster of the right kind in the switcher.";

    // Reaching a Cluster.

    /// A Cluster that was connected stopped answering.
    CONNECTION_LOST = "errors.clusters.connectionLost" => "Homeport lost the connection to {name}.";
    CONNECTION_LOST_HINT = "errors.clusters.connectionLost.hint" => "The cluster or the network may have restarted. Homeport keeps retrying.";
    /// A command for a Cluster that isn't connected.
    NOT_CONNECTED = "errors.clusters.notConnected" => "{name} isn't connected.";
    /// …and is no longer stored.
    NOT_CONNECTED_UNKNOWN = "errors.clusters.notConnectedUnknown" => "This cluster isn't connected.";
    /// …because it's disabled.
    NOT_CONNECTED_DISABLED_HINT = "errors.clusters.notConnected.hintDisabled" => "Enable the cluster on its page to use it.";
    /// …because Homeport is still connecting.
    NOT_CONNECTED_CONNECTING_HINT = "errors.clusters.notConnected.hintConnecting" => "Homeport is still connecting; try again in a moment.";
    /// …for another reason, shown on its page.
    NOT_CONNECTED_HINT = "errors.clusters.notConnected.hint" => "Open the cluster's page to see why and fix it.";
    /// Refreshing a Cluster that was pasted, without pasting it again.
    NOT_REFRESHABLE = "errors.clusters.kubeconfig.notRefreshable" => "{name} was pasted, so it can't be refreshed from this computer's kubeconfig.";
    NOT_REFRESHABLE_HINT = "errors.clusters.kubeconfig.notRefreshable.hint" => "Paste its kubeconfig again.";
    /// Refreshing a Cluster that wasn't imported from a kubeconfig.
    NO_SOURCE = "errors.clusters.kubeconfig.noSource" => "{name} wasn't imported from a kubeconfig.";
    /// Importing with nothing picked.
    IMPORT_NOTHING = "errors.clusters.kubeconfig.importNothing" => "Pick at least one context to import.";
    /// Importing two contexts under one name.
    IMPORT_SAME_NAME = "errors.clusters.kubeconfig.importSameName" => "Two clusters would be named \"{name}\".";
}

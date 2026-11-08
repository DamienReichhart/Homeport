//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    /// A Cluster removed while it was in use.
    GONE = "errors.clusters.gone" => "This cluster no longer exists.";
    /// Saving a Cluster without a name.
    NAME_EMPTY = "errors.clusters.nameEmpty" => "Give the cluster a name.";
    /// Saving a Cluster under a name another one has.
    NAME_IN_USE = "errors.clusters.nameInUse" => "A cluster named \"{name}\" already exists.";
    NAME_IN_USE_HINT = "errors.clusters.nameInUse.hint" => "Pick another name.";
    /// A Cluster whose API server isn't an http(s) URL.
    SERVER_INVALID = "errors.clusters.serverInvalid" => "\"{server}\" isn't the URL of a Kubernetes API server, like https://127.0.0.1:6443.";
    /// A Cluster of a kind a newer Homeport stored.
    UNKNOWN_KIND = "errors.clusters.unknownKind" => "Homeport doesn't know clusters of kind {kind}.";

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
    /// The API server refused the credentials (401).
    UNAUTHORIZED = "errors.clusters.unauthorized" => "{name} refused the credentials from its kubeconfig.";
    UNAUTHORIZED_HINT = "errors.clusters.unauthorized.hint" => "They may have expired or been revoked. Check that kubectl still works with this context, then refresh the cluster from its kubeconfig.";
    /// The API server didn't answer in time.
    TIMEOUT = "errors.clusters.timeout" => "{name} didn't answer in time.";
    TIMEOUT_HINT = "errors.clusters.timeout.hint" => "Check that the cluster is running and reachable from this computer, then reconnect.";
    /// Any other answer of the Kubernetes API.
    API_ERROR = "errors.clusters.apiError" => "{name} answered with an error.";
    API_ERROR_HINT = "errors.clusters.apiError.hint" => "See Details for the Kubernetes API's answer.";
    /// The Kubernetes API answered with something Homeport can't read.
    UNREADABLE = "errors.clusters.unreadable" => "{name} sent an answer Homeport can't read.";

    // The kubeconfig's certificates and keys.

    /// A CA certificate, client certificate or key that isn't PEM.
    CREDENTIALS_UNUSABLE = "errors.clusters.credentialsUnusable" => "The certificates or keys from the kubeconfig of {name} can't be used.";
    CREDENTIALS_UNUSABLE_HINT = "errors.clusters.credentialsUnusable.hint" => "Check that kubectl works with this context, then refresh the cluster from its kubeconfig.";
    /// The API server's certificate isn't signed by the kubeconfig's CA.
    TLS_UNKNOWN_CA = "errors.clusters.tls.unknownCa" => "The certificate of {name} isn't signed by the CA in its kubeconfig.";
    TLS_UNKNOWN_CA_HINT = "errors.clusters.tls.unknownCa.hint" => "The cluster may have been recreated with a new CA. Refresh it from its kubeconfig.";
    /// The API server's certificate expired.
    TLS_CERT_EXPIRED = "errors.clusters.tls.certExpired" => "The certificate of {name} has expired.";
    TLS_CERT_EXPIRED_HINT = "errors.clusters.tls.certExpired.hint" => "Renew the API server's certificate, or check that this computer's date and time are right.";
    /// The API server's certificate doesn't name the server.
    TLS_NAME_MISMATCH = "errors.clusters.tls.nameMismatch" => "The certificate of {name} isn't made for {server}.";
    TLS_NAME_MISMATCH_HINT = "errors.clusters.tls.nameMismatch.hint" => "Use the server address the certificate lists, or set tls-server-name in the kubeconfig and refresh the cluster.";
    /// The API server refused the client certificate.
    TLS_CLIENT_REJECTED = "errors.clusters.tls.clientRejected" => "{name} rejected the client certificate from its kubeconfig.";
    TLS_CLIENT_REJECTED_HINT = "errors.clusters.tls.clientRejected.hint" => "It may have expired or been made for another cluster. Refresh the cluster from its kubeconfig.";
    /// The server doesn't speak TLS.
    TLS_NOT_SUPPORTED = "errors.clusters.tls.notSupported" => "{name} doesn't speak TLS at {server}.";
    TLS_NOT_SUPPORTED_HINT = "errors.clusters.tls.notSupported.hint" => "Check the server URL in the kubeconfig: Kubernetes API servers usually listen on https, port 6443 or 443.";
    /// Any other TLS failure.
    TLS_HANDSHAKE_FAILED = "errors.clusters.tls.handshakeFailed" => "The secure connection to {name} failed.";
    TLS_HANDSHAKE_FAILED_HINT = "errors.clusters.tls.handshakeFailed.hint" => "See Details, and check that kubectl works with this context.";
}

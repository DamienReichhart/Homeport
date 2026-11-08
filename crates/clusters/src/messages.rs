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

    // Kubeconfigs.

    /// The user has no home folder to find `~/.kube` in.
    NO_HOME_FOLDER = "errors.clusters.kubeconfig.noHomeFolder" => "Homeport can't find your home folder.";
    /// Neither `KUBECONFIG`'s files nor `~/.kube/config` exist.
    KUBECONFIG_MISSING = "errors.clusters.kubeconfig.missing" => "There's no kubeconfig at {path}.";
    KUBECONFIG_MISSING_HINT = "errors.clusters.kubeconfig.missing.hint" => "Paste a kubeconfig instead, or set KUBECONFIG before starting Homeport.";
    /// A kubeconfig file that isn't YAML kubectl reads.
    KUBECONFIG_INVALID_FILE = "errors.clusters.kubeconfig.invalidFile" => "{path} isn't a kubeconfig Homeport can read.";
    /// A pasted kubeconfig that isn't YAML kubectl reads.
    KUBECONFIG_INVALID = "errors.clusters.kubeconfig.invalid" => "That isn't a kubeconfig Homeport can read.";
    KUBECONFIG_INVALID_HINT = "errors.clusters.kubeconfig.invalid.hint" => "Check that it's the YAML kubectl uses; see Details for where it went wrong.";
    /// A pasted kubeconfig without any context.
    KUBECONFIG_NO_CONTEXTS = "errors.clusters.kubeconfig.noContexts" => "That kubeconfig has no contexts to import.";
    /// A pasted kubeconfig that refers to a file by a relative path.
    KUBECONFIG_RELATIVE_PATH = "errors.clusters.kubeconfig.relativePath" => "It refers to {path}, which can't be found from a pasted kubeconfig.";
    KUBECONFIG_RELATIVE_PATH_HINT = "errors.clusters.kubeconfig.relativePath.hint" => "Paste a kubeconfig with its certificates inlined:";
    /// A file the kubeconfig refers to over 1 MB.
    KUBECONFIG_FILE_TOO_LARGE = "errors.clusters.kubeconfig.fileTooLarge" => "{path} is too large to be a certificate, key or token.";
    /// A `*-data` field that isn't base64 PEM.
    KUBECONFIG_DATA_INVALID = "errors.clusters.kubeconfig.dataInvalid" => "A certificate or key in the kubeconfig isn't valid base64.";
    /// The context the user picked is gone from the kubeconfig.
    CONTEXT_GONE = "errors.clusters.kubeconfig.contextGone" => "The context {context} is no longer in the kubeconfig.";
    CONTEXT_GONE_HINT = "errors.clusters.kubeconfig.contextGone.hint" => "Read the kubeconfig again and pick from what it holds now.";
    /// A context whose cluster entry is missing.
    CONTEXT_NO_CLUSTER = "errors.clusters.kubeconfig.noCluster" => "Its cluster {cluster} isn't in the kubeconfig.";
    /// A context whose cluster has no server.
    CONTEXT_NO_SERVER = "errors.clusters.kubeconfig.noServer" => "Its cluster has no server address.";
    /// A context whose user entry is missing.
    CONTEXT_NO_USER = "errors.clusters.kubeconfig.noUser" => "Its user {user} isn't in the kubeconfig.";
    /// A user of a removed auth provider.
    AUTH_PROVIDER_UNSUPPORTED = "errors.clusters.kubeconfig.authProviderUnsupported" => "Its user signs in with the {provider} auth provider, which Kubernetes no longer supports.";
    AUTH_PROVIDER_GCP_HINT = "errors.clusters.kubeconfig.authProviderUnsupported.hintGcp" => "Install gke-gcloud-auth-plugin and fetch the cluster's credentials again, which switches the kubeconfig to it:";
    AUTH_PROVIDER_AZURE_HINT = "errors.clusters.kubeconfig.authProviderUnsupported.hintAzure" => "Switch the kubeconfig to kubelogin, Azure's exec plugin:";
    AUTH_PROVIDER_OTHER_HINT = "errors.clusters.kubeconfig.authProviderUnsupported.hintOther" => "Switch this user to an exec plugin, as kubectl now requires.";
    /// Refreshing a Cluster that was pasted, without pasting it again.
    NOT_REFRESHABLE = "errors.clusters.kubeconfig.notRefreshable" => "{name} was pasted, so it can't be refreshed from this computer's kubeconfig.";
    NOT_REFRESHABLE_HINT = "errors.clusters.kubeconfig.notRefreshable.hint" => "Paste its kubeconfig again.";
    /// Refreshing a Cluster that wasn't imported from a kubeconfig.
    NO_SOURCE = "errors.clusters.kubeconfig.noSource" => "{name} wasn't imported from a kubeconfig.";
    /// Importing with nothing picked.
    IMPORT_NOTHING = "errors.clusters.kubeconfig.importNothing" => "Pick at least one context to import.";
    /// Importing two contexts under one name.
    IMPORT_SAME_NAME = "errors.clusters.kubeconfig.importSameName" => "Two clusters would be named \"{name}\".";

    // Exec credential plugins.

    /// The plugin's program isn't installed.
    EXEC_MISSING = "errors.clusters.exec.missing" => "{command} isn't installed on this computer.";
    /// …and the kubeconfig says how to install it (in Details).
    EXEC_MISSING_HINT_KUBECONFIG = "errors.clusters.exec.missing.hintKubeconfig" => "The kubeconfig says how to install it; see Details. Then reconnect.";
    EXEC_MISSING_HINT_AWS = "errors.clusters.exec.missing.hintAws" => "Install the AWS CLI (version 2) in a standard place, check it works, then reconnect:";
    EXEC_MISSING_HINT_GKE = "errors.clusters.exec.missing.hintGke" => "Install the Google Cloud CLI and its GKE plugin, then reconnect:";
    EXEC_MISSING_HINT_KUBELOGIN = "errors.clusters.exec.missing.hintKubelogin" => "Install kubelogin, for example with the Azure CLI, then reconnect:";
    EXEC_MISSING_HINT_AZ = "errors.clusters.exec.missing.hintAz" => "Install the Azure CLI, then reconnect.";
    EXEC_MISSING_HINT = "errors.clusters.exec.missing.hint" => "Install it, or put it in a folder on your PATH, then reconnect.";
    /// The plugin exited with an error or printed no credentials.
    EXEC_FAILED = "errors.clusters.exec.failed" => "{command} couldn't get credentials for {name}.";
    EXEC_FAILED_HINT = "errors.clusters.exec.failed.hint" => "See Details for what it said. Homeport tries again on its own.";
    /// The plugin needs a terminal or a browser to sign in.
    EXEC_INTERACTIVE = "errors.clusters.exec.interactive" => "{command} needs you to sign in, which it can't do inside Homeport.";
    EXEC_INTERACTIVE_HINT = "errors.clusters.exec.interactive.hint" => "Run kubectl once in a terminal with this context to sign in, then reconnect:";

    // What runs on a Cluster.

    /// The credentials may not do this in one Namespace.
    FORBIDDEN_IN = "errors.clusters.forbiddenIn" => "Your credentials can't {verb} {resource} in {namespace}.";
    /// …in every Namespace at once.
    FORBIDDEN_EVERYWHERE = "errors.clusters.forbiddenEverywhere" => "Your credentials can't {verb} {resource} across all namespaces.";
    FORBIDDEN_EVERYWHERE_HINT = "errors.clusters.forbiddenEverywhere.hint" => "Pick a namespace you're allowed to see, or ask the cluster's administrator for this permission. Check with:";
    /// …on a resource that isn't in a Namespace.
    FORBIDDEN = "errors.clusters.forbidden" => "Your credentials can't {verb} {resource}.";
    FORBIDDEN_HINT = "errors.clusters.forbidden.hint" => "Ask the cluster's administrator for this permission. Check with:";
    /// The resource was deleted while it was open.
    RESOURCE_GONE = "errors.clusters.resourceGone" => "{kind} {name} no longer exists.";
    /// Scaling to fewer than no Pods.
    REPLICAS_NEGATIVE = "errors.clusters.replicasNegative" => "A workload can't run fewer than 0 Pods.";
    /// An action a kind of workload doesn't have, such as scaling a DaemonSet.
    ACTION_NOT_SUPPORTED = "errors.clusters.actionNotSupported" => "A {kind} can't {action}.";
    /// The object changed between reading it and changing it.
    CHANGED_MEANWHILE = "errors.clusters.changedMeanwhile" => "{kind} {name} changed while you were working on it.";
    CHANGED_MEANWHILE_HINT = "errors.clusters.changedMeanwhile.hint" => "Look at it again, then retry.";
    /// A Secret's key that is gone by the time it's revealed.
    SECRET_KEY_GONE = "errors.clusters.secretKeyGone" => "{name} no longer has a key {key}.";
}

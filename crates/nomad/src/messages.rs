//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    // The address and the TLS material, before connecting.

    /// An address that isn't a Nomad agent's URL.
    ADDRESS_INVALID = "errors.nomad.addressInvalid" => "\"{address}\" isn't the address of a Nomad agent, like http://127.0.0.1:4646.";
    /// Homeport couldn't build its HTTP client.
    HTTPS_CLIENT = "errors.nomad.httpsClient" => "Homeport couldn't prepare its HTTPS client.";
    /// A CA certificate that isn't PEM.
    CA_UNUSABLE = "errors.nomad.caUnusable" => "The CA certificate of {name} can't be read.";
    CA_UNUSABLE_HINT = "errors.nomad.caUnusable.hint" => "Paste the whole PEM file, from -----BEGIN CERTIFICATE----- to -----END CERTIFICATE-----, often named nomad-agent-ca.pem.";
    /// A client certificate or key that can't be used.
    CLIENT_PAIR_UNUSABLE = "errors.nomad.clientPairUnusable" => "The client certificate or key of {name} can't be used.";
    CLIENT_PAIR_UNUSABLE_HINT = "errors.nomad.clientPairUnusable.hint" => "Paste the PEM certificate and the private key made for it together, such as global-cli-nomad.pem and global-cli-nomad-key.pem.";
    /// Only one of a client certificate and its key.
    CLIENT_PAIR_INCOMPLETE = "errors.nomad.clientPairIncomplete" => "Give both the client certificate and its key, or neither.";
    /// A TLS server name that isn't a name.
    SERVER_NAME_INVALID = "errors.nomad.serverNameInvalid" => "\"{serverName}\" isn't a name a certificate can be checked against.";

    // Reaching the agent.

    /// The agent didn't answer in time.
    TIMEOUT = "errors.nomad.timeout" => "{name} didn't answer in time.";
    TIMEOUT_HINT = "errors.nomad.timeout.hint" => "Check that the Nomad agent is running and reachable from this computer, then reconnect.";
    /// The agent's certificate has expired.
    TLS_CERT_EXPIRED = "errors.nomad.tls.certExpired" => "The certificate of {name} has expired.";
    TLS_CERT_EXPIRED_HINT = "errors.nomad.tls.certExpired.hint" => "Renew the Nomad agent's certificate, or check this computer's clock.";
    /// The agent's certificate isn't signed by a CA Homeport trusts.
    TLS_UNKNOWN_CA = "errors.nomad.tls.unknownCa" => "The certificate of {name} isn't signed by a CA Homeport trusts.";
    TLS_UNKNOWN_CA_HINT = "errors.nomad.tls.unknownCa.hint" => "Paste the CA certificate that signed the agent's certificate on the cluster's page, the one NOMAD_CACERT points to for the nomad CLI.";
    /// The agent's certificate is for other names.
    TLS_NAME_MISMATCH = "errors.nomad.tls.nameMismatch" => "The certificate of {name} isn't valid for {server}.";
    TLS_NAME_MISMATCH_HINT = "errors.nomad.tls.nameMismatch.hint" => "Nomad's certificates are usually made for server.global.nomad, with the cluster's region in place of global. Set the TLS server name on the cluster's page to that name, as NOMAD_TLS_SERVER_NAME does for the nomad CLI.";
    /// The agent refused Homeport's client certificate, or wanted one.
    TLS_CLIENT_REJECTED = "errors.nomad.tls.clientRejected" => "{name} refused Homeport's client certificate.";
    TLS_CLIENT_REJECTED_HINT = "errors.nomad.tls.clientRejected.hint" => "The agent asks for a client certificate signed by its CA (verify_https_client). Paste one and its key on the cluster's page.";
    /// Another TLS failure.
    TLS_HANDSHAKE_FAILED = "errors.nomad.tls.handshakeFailed" => "Homeport couldn't open a secure connection to {name}.";
    TLS_HANDSHAKE_FAILED_HINT = "errors.nomad.tls.handshakeFailed.hint" => "Details has the TLS error.";
    /// HTTPS to an agent that answers plain HTTP.
    TLS_NOT_SUPPORTED = "errors.nomad.tls.notSupported" => "{name} doesn't use TLS at {server}.";
    TLS_NOT_SUPPORTED_HINT = "errors.nomad.tls.notSupported.hint" => "Change the address to start with http://.";
    /// Plain HTTP to an agent that wants HTTPS.
    TLS_REQUIRED = "errors.nomad.tls.required" => "{name} only answers over HTTPS.";
    TLS_REQUIRED_HINT = "errors.nomad.tls.required.hint" => "Change the address to start with https://, and paste the agent's CA certificate if your own CA signed it.";

    // Nomad's answers.

    /// The ACL token is wrong, expired or deleted.
    TOKEN_INVALID = "errors.nomad.tokenInvalid" => "{name} doesn't know the ACL token Homeport sent.";
    TOKEN_INVALID_HINT = "errors.nomad.tokenInvalid.hint" => "It may have expired or been deleted. Make a new one and paste its secret ID on the cluster's page:";
    /// The token lacks a permission, in general.
    PERMISSION_DENIED = "errors.nomad.permissionDenied" => "{name} refused: the ACL token lacks a permission this needs.";
    PERMISSION_DENIED_HINT = "errors.nomad.permissionDenied.hint" => "Attach a policy that grants it to the token. Homeport's docs list the capabilities each page needs; Details has Nomad's answer.";
    /// No leader among the servers.
    NO_LEADER = "errors.nomad.noLeader" => "{name} has no leader: its servers can't agree on one.";
    NO_LEADER_HINT = "errors.nomad.noLeader.hint" => "Nomad needs a majority of its servers running and able to reach each other. Check them on a server:";
    /// Nomad has no such object.
    NOT_FOUND = "errors.nomad.notFound" => "{name} has no such object; it may have just been removed.";
    /// Nomad refused what Homeport sent.
    BAD_REQUEST = "errors.nomad.badRequest" => "{name} refused what Homeport sent.";
    BAD_REQUEST_HINT = "errors.nomad.badRequest.hint" => "Details has Nomad's answer, which says what it didn't accept.";
    /// Any other error answer.
    API_ERROR = "errors.nomad.apiError" => "{name} answered with an error.";
    DETAILS_HINT = "errors.nomad.details.hint" => "Details has Nomad's answer.";
    /// An answer that isn't the JSON Nomad sends.
    UNREADABLE = "errors.nomad.unreadable" => "{name} sent an answer Homeport couldn't read.";
    UNREADABLE_HINT = "errors.nomad.unreadable.hint" => "Check that the address points to a Nomad agent's HTTP API, usually on port 4646.";
}

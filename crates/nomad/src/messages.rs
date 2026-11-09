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
    // Jobs.

    /// No such job.
    JOB_NOT_FOUND = "errors.nomad.jobNotFound" => "{name} has no job {job}.";
    JOB_NOT_FOUND_HINT = "errors.nomad.jobNotFound.hint" => "It may have been stopped and purged, or be in another namespace.";
    /// A count below the scaling policy's minimum.
    SCALE_BELOW_MIN = "errors.nomad.scaleBelowMin" => "Task group {group} of {job} can't run fewer than {min}.";
    /// A count above the scaling policy's maximum.
    SCALE_ABOVE_MAX = "errors.nomad.scaleAboveMax" => "Task group {group} of {job} can't run more than {max}.";
    SCALE_BOUNDS_HINT = "errors.nomad.scaleBounds.hint" => "Its scaling policy sets the bounds. Change them in the job's definition to go further.";
    /// Nomad refused a scale for a reason of its own.
    SCALE_REFUSED_HINT = "errors.nomad.scaleRefused.hint" => "Details has Nomad's reason.";
    /// The token lacks submit-job or scale-job.
    SCALE_DENIED = "errors.nomad.scaleDenied" => "{name} refused: the ACL token can't scale jobs in this namespace.";
    SCALE_DENIED_HINT = "errors.nomad.scaleDenied.hint" => "Scaling needs the scale-job capability, which the write policy also grants, on the job's namespace.";
    /// The token lacks submit-job to stop.
    STOP_DENIED = "errors.nomad.stopDenied" => "{name} refused: the ACL token can't stop jobs in this namespace.";
    STOP_DENIED_HINT = "errors.nomad.stopDenied.hint" => "Stopping needs the submit-job capability on the job's namespace.";
    /// The token lacks read-job.
    READ_DENIED = "errors.nomad.readDenied" => "{name} refused: the ACL token can't read jobs in this namespace.";
    READ_DENIED_HINT = "errors.nomad.readDenied.hint" => "Listing jobs and their allocations needs the read-job capability (list-jobs and read-job), which the read policy grants.";

    // Running a job.

    /// A specification that isn't HCL Nomad can parse, at a place it named.
    PARSE_FAILED_AT = "errors.nomad.parseFailedAt" => "The job specification doesn't parse, at line {line}, column {column}.";
    /// …at no place Homeport could read.
    PARSE_FAILED = "errors.nomad.parseFailed" => "The job specification doesn't parse.";
    PARSE_FAILED_HINT = "errors.nomad.parseFailed.hint" => "Details has what Nomad found wrong. Variables need a default: Homeport doesn't pass variable files.";
    /// JSON that isn't a job.
    JSON_NOT_A_JOB = "errors.nomad.jsonNotAJob" => "This JSON isn't a Nomad job.";
    JSON_NOT_A_JOB_HINT = "errors.nomad.jsonNotAJob.hint" => "Paste a job in Nomad's API format, a Job object or one with ID and TaskGroups, as nomad job inspect prints it:";
    /// An empty specification.
    SPEC_EMPTY = "errors.nomad.specEmpty" => "Paste or open a job specification first.";
    /// Some task groups can't be placed.
    PLACEMENT_FAILED = "errors.nomad.placementFailed" => "Nomad can't place every allocation of {job}: {groups}.";
    PLACEMENT_FAILED_HINT = "errors.nomad.placementFailed.hint" => "No eligible node has the resources or matches the constraints. Running it anyway queues what can't be placed until a node can take it.";
    /// The job changed since it was planned.
    JOB_MODIFIED = "errors.nomad.jobModified" => "{job} changed since it was planned, so it wasn't run.";
    JOB_MODIFIED_HINT = "errors.nomad.jobModified.hint" => "Someone else ran or stopped it in between. Plan again to see the change against what runs now.";
    /// Running a new job whose ID another job has.
    JOB_EXISTS = "errors.nomad.jobExists" => "{job} was created by someone else since it was planned, so it wasn't run.";
    /// The token lacks submit-job.
    SUBMIT_DENIED = "errors.nomad.submitDenied" => "{name} refused: the ACL token can't run jobs in namespace {namespace}.";
    SUBMIT_DENIED_HINT = "errors.nomad.submitDenied.hint" => "Planning and running need the submit-job capability on the job's namespace, which the write policy grants.";

    // Allocations: their logs and terminals.

    /// No such allocation.
    ALLOC_NOT_FOUND = "errors.nomad.allocNotFound" => "{name} has no allocation {alloc}.";
    ALLOC_NOT_FOUND_HINT = "errors.nomad.allocNotFound.hint" => "It may have been garbage-collected. Open the job again to see its current allocations.";
    /// The task or allocation isn't running.
    ALLOC_NOT_RUNNING = "errors.nomad.allocNotRunning" => "Task {task} of allocation {alloc} isn't running.";
    ALLOC_NOT_RUNNING_HINT = "errors.nomad.allocNotRunning.hint" => "A shell needs a running task. Its logs stay readable until the allocation is garbage-collected.";
    /// The servers can't reach the allocation's node.
    NODE_UNREACHABLE = "errors.nomad.nodeUnreachable" => "{name} can't reach the node allocation {alloc} runs on.";
    NODE_UNREACHABLE_HINT = "errors.nomad.nodeUnreachable.hint" => "Logs and shells go through the servers to the client node. Check that the node is up and its Nomad agent connected, on the Nodes page or with:";
    /// The token lacks read-logs.
    LOGS_DENIED = "errors.nomad.logsDenied" => "{name} refused: the ACL token can't read logs in this namespace.";
    LOGS_DENIED_HINT = "errors.nomad.logsDenied.hint" => "Reading a task's logs needs the read-logs capability on the job's namespace.";
    /// The token lacks alloc-exec.
    EXEC_DENIED = "errors.nomad.execDenied" => "{name} refused: the ACL token can't open a shell in this namespace.";
    EXEC_DENIED_HINT = "errors.nomad.execDenied.hint" => "A shell needs the alloc-exec capability on the job's namespace, and alloc-node-exec for tasks using the raw_exec driver.";
    /// A command with a quote left open.
    COMMAND_UNCLOSED_QUOTE = "errors.nomad.commandUnclosedQuote" => "The command has a quote that isn't closed.";
    /// The command isn't in the task.
    EXEC_NO_COMMAND = "errors.nomad.execNoCommand" => "The task has no {command} to run.";
    EXEC_NO_COMMAND_HINT = "errors.nomad.execNoCommand.hint" => "Its image may not include a shell. Try another command, such as /bin/bash or /busybox/sh.";
    /// The terminal ended.
    TERMINAL_ENDED = "errors.nomad.terminalEnded" => "The terminal ended.";

    /// An answer that isn't the JSON Nomad sends.
    UNREADABLE = "errors.nomad.unreadable" => "{name} sent an answer Homeport couldn't read.";
    UNREADABLE_HINT = "errors.nomad.unreadable.hint" => "Check that the address points to a Nomad agent's HTTP API, usually on port 4646.";
}

//! The stable codes the frontend tells failures apart by.

use serde::{Deserialize, Serialize};

/// Stable machine-readable identifier of a failure, one per case the user can act on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// A failure that started in the frontend, outside any command.
    Unknown,
    // The app itself.
    Io,
    Json,
    Db,
    DbLocked,
    DbCorrupt,
    DbMigration,
    Tauri,
    Validation,
    NotFound,
    KeychainUnavailable,
    CredentialsLost,
    UpdateNetwork,
    UpdateSignatureInvalid,
    UpdatePermissionDenied,
    UpdateFailed,
    /// The OS refused the entry that starts Homeport at login.
    AutostartFailed,
    /// An export file that the passphrase doesn't open, or that was damaged.
    BundleUnreadable,
    /// An export file from a newer Homeport.
    BundleTooNew,
    // Reaching a Host.
    HostNotConnected,
    EngineNotInstalled,
    EngineNotRunning,
    PermissionDenied,
    DnsFailed,
    ConnectionRefused,
    Timeout,
    NetworkUnreachable,
    ConnectionLost,
    TlsRequired,
    TlsNotSupported,
    TlsCertificateExpired,
    TlsCertificateNotYetValid,
    TlsUnknownCa,
    TlsNameMismatch,
    TlsClientRejected,
    TlsHandshakeFailed,
    TlsMaterialInvalid,
    SshAuthFailed,
    SshAgentUnavailable,
    SshKeyInvalid,
    SshFailed,
    RemoteEngineUnavailable,
    HostKeyUntrusted,
    HostKeyChanged,
    /// A page or action the Host's kind of Engine doesn't offer, such as
    /// pods on a Docker Engine.
    EngineUnsupported,
    // Engine API responses.
    EngineError,
    BadRequest,
    Forbidden,
    Conflict,
    NameInUse,
    InUse,
    ContainerRunning,
    ContainerNotRunning,
    AlreadyDone,
    PortInUse,
    DiskFull,
    ImageNotFound,
    RegistryAuthRequired,
    RegistryRateLimited,
    RegistryUnreachable,
    RegistryLoginFailed,
    ApiVersionUnsupported,
    BuildFailed,
    /// A GPU was requested from a Host without the vendor's container toolkit.
    GpuUnavailable,
    /// Recreating a container failed, and so did putting the original back.
    RecreateRolledBack,
    // Podman.
    /// Removing a pod whose containers are running, without stopping them.
    PodRunning,
    /// An action the Host's Podman is too old for, such as Quadlet's API before 5.8.
    PodmanTooOld,
    /// A Quadlet unit file Podman can't use: a wrong name or a missing section.
    QuadletInvalid,
    /// Installing a Quadlet unit under a name another unit has.
    QuadletExists,
    /// systemd refused to start, stop or restart a unit's service, or isn't there.
    SystemdRefused,
    /// User units need the user's systemd instance, which only lingering keeps.
    UserUnitsNeedLinger,
    // containerd (ADR 0020).
    /// A containerd older than Homeport connects to, or than an action needs.
    ContainerdTooOld,
    /// Logs or exec on a containerd Host whose connection can't run commands.
    ContainerdNeedsShell,
    /// A containerd container whose output no tool recorded.
    ContainerdNoLogs,
    /// Removing a containerd namespace that still holds something.
    NamespaceNotEmpty,
    /// A containerd task that didn't stop after SIGKILL.
    TaskWontStop,
    // Incus (ADR 0021).
    /// An Incus without an API extension Homeport or an action needs.
    IncusExtensionMissing,
    /// An Incus trust token that can't be read, or has expired.
    IncusTokenInvalid,
    /// An Incus trust token Incus refused: already used, or revoked.
    IncusTokenRefused,
    /// A server that presented another certificate than the one pinned.
    CertificateMismatch,
    /// An Incus that doesn't listen on the network.
    IncusNotListening,
    /// A virtual machine on a Host without hardware virtualization.
    VirtualizationUnavailable,
    /// Exec in a virtual machine whose Incus agent isn't running.
    IncusAgentNotRunning,
    /// A stateful snapshot an Instance can't take.
    StatefulUnsupported,
    /// An object changed since the page loaded it.
    ChangedSinceLoaded,
    // Swarms (ADR 0016).
    /// No manager Host of a Swarm is connected.
    SwarmUnreachable,
    /// A node couldn't reach the managers of the Swarm it was joining.
    SwarmJoinFailed,
    /// The `docker` CLI isn't installed, which deploying a Stack to a Swarm needs.
    DockerCliMissing,
    // Clusters.
    /// A command for a Cluster that isn't connected.
    ClusterNotConnected,
    /// The Cluster's API server refused the credentials from its kubeconfig.
    ClusterUnauthorized,
    /// The Kubernetes API answered with an error Homeport has no better words for.
    ClusterApiError,
    /// A kubeconfig user authenticates with a removed auth provider (gcp, azure).
    KubeconfigAuthProviderUnsupported,
    /// The program of a Cluster's exec credential plugin isn't installed.
    ExecPluginMissing,
    /// A Cluster's exec credential plugin exited with an error or printed no credentials.
    ExecPluginFailed,
    /// A Cluster's exec credential plugin needs a terminal to sign in.
    ExecPluginInteractive,
    /// The private local proxy a CLI reaches a Cluster through couldn't open (ADR 0009).
    ClusterProxyFailed,
    // Nomad Clusters.
    /// A Nomad Cluster that doesn't know the ACL token: wrong, expired or deleted.
    NomadTokenInvalid,
    /// A Nomad ACL token that lacks a capability or policy an action needs.
    NomadPermissionDenied,
    /// A Nomad Cluster whose servers have no leader, so nothing can be changed or read.
    NomadNoLeader,
    /// A Nomad job that doesn't exist, or not in the namespace asked.
    NomadJobNotFound,
    /// A task group count outside its scaling policy's minimum and maximum.
    NomadScaleOutOfBounds,
    // Helm (ADR 0009).
    /// The `helm` CLI isn't installed on this computer.
    HelmMissing,
    /// The `helm` CLI found is Helm 2, which Homeport doesn't support.
    HelmOutdated,
    /// A release is locked by an install, upgrade or rollback still pending.
    HelmReleaseLocked,
    /// `helm` failed in a way Homeport has no better words for.
    HelmFailed,
    // Compose.
    ComposeMissing,
    ComposeOutdated,
    ComposeSpawnFailed,
    ComposeInvalidDefinition,
    ComposeMissingVariable,
    ComposeFailed,
    // Linked Stacks' folders on this machine.
    /// A Linked Stack's folder, or one of its compose files, is gone.
    StackFolderMissing,
    /// A Linked Stack's compose file can't be read or written.
    StackFileUnreadable,
    /// A Linked Stack's compose file changed on disk since it was opened.
    StackFilesChanged,
    // Synced Stacks' git repositories (ADR 0014).
    /// The `git` CLI isn't installed on this computer.
    GitMissing,
    /// The repository refused the deploy key or the token.
    GitAuthFailed,
    /// The repository doesn't exist, or the credentials can't see it.
    GitRepositoryNotFound,
    /// The repository's server can't be reached from this computer.
    GitUnreachable,
    /// The SSH server of a repository presented a different key than the trusted one.
    GitHostKeyChanged,
    /// `git` failed in a way Homeport has no better words for.
    GitFailed,
    /// A Synced Stack's compose file, branch or tag isn't in its repository.
    SyncedComposeFileMissing,
    // Helper containers, which open volumes (ADR 0015).
    /// The helper's image isn't on the Host and couldn't be pulled.
    HelperImageUnavailable,
    /// A file to restore a volume from that isn't a tar archive.
    NotATarArchive,
    /// Restoring into a volume that holds files, without replacing them.
    VolumeNotEmpty,
    // Files and exec inside containers.
    NoShell,
    PathNotFound,
    ContainerPermissionDenied,
    FileTooLarge,
}

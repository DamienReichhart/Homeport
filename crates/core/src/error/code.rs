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

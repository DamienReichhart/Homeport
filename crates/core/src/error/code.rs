//! The stable codes the frontend tells failures apart by.

use serde::Serialize;

/// Stable machine-readable identifier of a failure, one per case the user can act on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, specta::Type)]
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
    // Files and exec inside containers.
    NoShell,
    PathNotFound,
    ContainerPermissionDenied,
    FileTooLarge,
}

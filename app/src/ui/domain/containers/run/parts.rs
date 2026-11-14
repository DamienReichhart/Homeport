//! The parts of the Run dialog's form its messages name: its fields of
//! one line and of several, its selects, its switches, its rows, and the
//! sections of its Advanced area.

use super::model::{advanced, Errors};
use crate::t;

/// A field of one line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line {
    Image,
    Name,
    Command,
    HostPort(usize),
    ContainerPort(usize),
    Source(usize),
    Target(usize),
    Cpus,
    CpuShares,
    Memory,
    MemoryReservation,
    MemorySwap,
    HostDevice(usize),
    ContainerDevice(usize),
    Permissions(usize),
    GpuCount,
    GpuIds,
    HealthCommand,
    Interval,
    Timeout,
    Retries,
    StartPeriod,
    Hostname,
    Domainname,
    LogDriver,
    /// What is being typed into a list of capabilities.
    Capability(Caps),
}

/// A field of several lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Text {
    Env,
    Labels,
    Dns,
    DnsSearch,
    ExtraHosts,
    LogOptions,
}

/// A select.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Network,
    Restart,
    Pod,
    Protocol(usize),
    Gpus,
    Health,
}

/// A switch or a checkbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Switch {
    AutoRemove,
    ReadOnly(usize),
    Privileged,
    /// The Advanced area is open.
    Advanced,
}

/// Rows that are added and removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rows {
    Ports,
    Mounts,
    Devices,
}

/// The capabilities added or dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Caps {
    Add,
    Drop,
}

/// The sections of the Advanced area, in order, and the fields whose
/// errors each holds (`SECTIONS`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Resources,
    Labels,
    Security,
    Devices,
    Health,
    Network,
    Logging,
}

impl Section {
    pub const ALL: [Self; 7] = [
        Self::Resources,
        Self::Labels,
        Self::Security,
        Self::Devices,
        Self::Health,
        Self::Network,
        Self::Logging,
    ];

    pub fn label(self) -> String {
        match self {
            Self::Resources => t!("containers.advanced.resources"),
            Self::Labels => t!("containers.advanced.labels"),
            Self::Security => t!("containers.advanced.security"),
            Self::Devices => t!("containers.advanced.devices"),
            Self::Health => t!("containers.advanced.healthcheck"),
            Self::Network => t!("containers.advanced.network"),
            Self::Logging => t!("containers.advanced.logging"),
        }
    }

    /// Whether one of its fields can't be read.
    pub fn fails(self, errors: &Errors) -> bool {
        use advanced::Field as F;
        let fields: &[F] = match self {
            Self::Resources => &[
                F::Cpus,
                F::CpuShares,
                F::Memory,
                F::MemoryReservation,
                F::MemorySwap,
            ],
            Self::Devices => &[F::GpuCount],
            Self::Health => &[F::Interval, F::Timeout, F::Retries, F::StartPeriod],
            _ => &[],
        };
        fields.iter().any(|field| errors.has(*field))
    }
}

/// A field with suggestions (React's `datalist`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hint {
    /// The Host's images, by tag.
    Image,
    /// The capabilities not chosen yet.
    Capability(Caps),
    /// The log drivers most Hosts have.
    LogDriver,
}

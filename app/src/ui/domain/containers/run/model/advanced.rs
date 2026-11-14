//! The Advanced area of the Run dialog, one part per section
//! (`model/runAdvanced.ts`): what it holds as typed, what can't be read
//! in it, and the Run request's fields it makes.

use homeport_engine::containers::run::{DeviceSpec, GpuSpec, HealthcheckSpec, LogSpec, RunSpec};

use super::parse::{
    format_duration, format_size, parse_duration, parse_positive, parse_size, parse_whole, Read,
};
use crate::t;

/// CPU and memory limits (`--cpus`, `--cpu-shares`, `--memory…`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Resources {
    pub cpus: String,
    pub cpu_shares: String,
    pub memory: String,
    pub memory_reservation: String,
    /// A size, or `-1` for unlimited swap.
    pub memory_swap: String,
}

/// A device of the Host, as its row says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceRow {
    pub host_path: String,
    pub container_path: String,
    pub permissions: String,
}

impl Default for DeviceRow {
    /// A new row: `rwm`, as React's Add device.
    fn default() -> Self {
        Self {
            host_path: String::new(),
            container_path: String::new(),
            permissions: "rwm".to_owned(),
        }
    }
}

/// Which GPUs are asked for (`--gpus`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GpuMode {
    #[default]
    None,
    All,
    Count,
    Ids,
}

impl GpuMode {
    pub const ALL: [Self; 4] = [Self::None, Self::All, Self::Count, Self::Ids];

    pub fn label(self) -> String {
        match self {
            Self::None => t!("containers.advanced.none"),
            Self::All => t!("containers.advanced.gpus.all"),
            Self::Count => t!("containers.advanced.gpus.count"),
            Self::Ids => t!("containers.advanced.gpus.ids"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gpus {
    pub mode: GpuMode,
    pub count: String,
    pub ids: String,
}

impl Default for Gpus {
    fn default() -> Self {
        Self {
            mode: GpuMode::None,
            count: "1".to_owned(),
            ids: String::new(),
        }
    }
}

/// Capabilities, privileged mode, devices and GPUs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Security {
    pub cap_add: Vec<String>,
    pub cap_drop: Vec<String>,
    pub privileged: bool,
    pub devices: Vec<DeviceRow>,
    pub gpus: Gpus,
}

/// Keep the image's healthcheck, define one, or turn it off.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HealthMode {
    #[default]
    Image,
    Custom,
    Disabled,
}

impl HealthMode {
    pub const ALL: [Self; 3] = [Self::Image, Self::Custom, Self::Disabled];

    pub fn label(self) -> String {
        match self {
            Self::Image => t!("containers.advanced.health.image"),
            Self::Custom => t!("containers.advanced.health.custom"),
            Self::Disabled => t!("containers.advanced.health.disabled"),
        }
    }
}

/// A healthcheck run in the container's shell (`--health-cmd` and friends).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Healthcheck {
    pub mode: HealthMode,
    pub command: String,
    pub interval: String,
    pub timeout: String,
    pub retries: String,
    pub start_period: String,
}

/// Hostname, domain name, DNS and `/etc/hosts` entries; a list one entry
/// a line.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Identity {
    pub hostname: String,
    pub domainname: String,
    pub dns: String,
    pub dns_search: String,
    pub extra_hosts: String,
}

/// Where the container's output goes (`--log-driver`, `--log-opt`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Logging {
    /// Empty for the Engine's default driver.
    pub driver: String,
    /// One `key=value` a line.
    pub options: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Advanced {
    pub resources: Resources,
    /// One `key=value` a line.
    pub labels: String,
    pub security: Security,
    pub healthcheck: Healthcheck,
    pub identity: Identity,
    pub logging: Logging,
}

/// A field whose text the dialog reads, and so explains before sending.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Field {
    Cpus,
    CpuShares,
    Memory,
    MemoryReservation,
    MemorySwap,
    GpuCount,
    Interval,
    Timeout,
    Retries,
    StartPeriod,
}

impl Field {
    /// Why its text can't be read, in the language of the moment.
    pub fn message(self) -> String {
        match self {
            Self::Cpus => t!("containers.run.error.cpus"),
            Self::CpuShares => t!("containers.run.error.cpuShares"),
            Self::Memory | Self::MemoryReservation => t!("containers.run.error.size"),
            Self::MemorySwap => t!("containers.run.error.memorySwap"),
            Self::GpuCount => t!("containers.run.error.gpuCount"),
            Self::Interval | Self::Timeout | Self::StartPeriod => {
                t!("containers.run.error.duration")
            }
            Self::Retries => t!("containers.run.error.retries"),
        }
    }
}

/// Non-empty lines, trimmed, without `#` comments.
pub fn lines(text: &str) -> Vec<String> {
    text.split('\n')
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

fn swap(text: &str) -> Read<i64> {
    if text.trim() == "-1" {
        Ok(Some(-1))
    } else {
        parse_size(text)
    }
}

/// What can't be read in the Advanced area, in the order React checked
/// it. Empty means it can be sent.
pub fn errors(form: &Advanced) -> Vec<Field> {
    let mut errors = vec![];
    let mut check = |field, unreadable: bool| {
        if unreadable {
            errors.push(field);
        }
    };
    let r = &form.resources;
    check(Field::Cpus, parse_positive(&r.cpus, false).is_err());
    check(
        Field::CpuShares,
        parse_positive(&r.cpu_shares, true).is_err(),
    );
    check(Field::Memory, parse_size(&r.memory).is_err());
    check(
        Field::MemoryReservation,
        parse_size(&r.memory_reservation).is_err(),
    );
    check(Field::MemorySwap, swap(&r.memory_swap).is_err());
    if form.security.gpus.mode == GpuMode::Count {
        check(
            Field::GpuCount,
            parse_whole::<u32>(&form.security.gpus.count).is_err(),
        );
    }
    let h = &form.healthcheck;
    if h.mode == HealthMode::Custom {
        check(Field::Interval, parse_duration(&h.interval).is_err());
        check(Field::Timeout, parse_duration(&h.timeout).is_err());
        check(Field::StartPeriod, parse_duration(&h.start_period).is_err());
        check(Field::Retries, parse_whole::<u32>(&h.retries).is_err());
    }
    errors
}

/// A value read, or none when it is missing or can't be read.
fn value<T>(read: Read<T>) -> Option<T> {
    read.ok().flatten()
}

fn blank(text: &str) -> Option<String> {
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}

fn gpus(form: &Gpus) -> Option<GpuSpec> {
    match form.mode {
        GpuMode::None => None,
        GpuMode::All => Some(GpuSpec::All),
        GpuMode::Count => Some(GpuSpec::Count {
            count: value(parse_whole(&form.count)).unwrap_or(1),
        }),
        GpuMode::Ids => Some(GpuSpec::Ids {
            ids: form
                .ids
                .split(|c: char| c.is_whitespace() || c == ',')
                .filter(|id| !id.is_empty())
                .map(str::to_owned)
                .collect(),
        }),
    }
}

fn healthcheck(form: &Healthcheck) -> Option<HealthcheckSpec> {
    if form.mode == HealthMode::Image {
        return None;
    }
    Some(HealthcheckSpec {
        command: form.command.trim().to_owned(),
        interval_s: value(parse_duration(&form.interval)),
        timeout_s: value(parse_duration(&form.timeout)),
        retries: value(parse_whole(&form.retries)),
        start_period_s: value(parse_duration(&form.start_period)),
        disabled: form.mode == HealthMode::Disabled,
    })
}

fn log(form: &Logging) -> Option<LogSpec> {
    Some(LogSpec {
        driver: blank(&form.driver)?,
        options: lines(&form.options),
    })
}

/// Fills in the Run request's advanced fields; call once [`errors`] is
/// empty.
pub fn apply(form: &Advanced, spec: &mut RunSpec) {
    let r = &form.resources;
    spec.cpus = value(parse_positive(&r.cpus, false));
    spec.cpu_shares = value(parse_whole(&r.cpu_shares));
    spec.memory = value(parse_size(&r.memory));
    spec.memory_reservation = value(parse_size(&r.memory_reservation));
    spec.memory_swap = value(swap(&r.memory_swap));
    spec.labels = lines(&form.labels);
    let security = &form.security;
    spec.cap_add.clone_from(&security.cap_add);
    spec.cap_drop.clone_from(&security.cap_drop);
    spec.privileged = security.privileged;
    spec.devices = security
        .devices
        .iter()
        .filter(|d| !d.host_path.trim().is_empty())
        .map(|d| DeviceSpec {
            host_path: d.host_path.trim().to_owned(),
            container_path: blank(&d.container_path),
            permissions: blank(&d.permissions).unwrap_or_else(|| "rwm".to_owned()),
        })
        .collect();
    spec.gpus = gpus(&security.gpus);
    spec.healthcheck = healthcheck(&form.healthcheck);
    spec.hostname = blank(&form.identity.hostname);
    spec.domainname = blank(&form.identity.domainname);
    spec.dns = lines(&form.identity.dns);
    spec.dns_search = lines(&form.identity.dns_search);
    spec.extra_hosts = lines(&form.identity.extra_hosts);
    spec.log = log(&form.logging);
}

/// `value` as React wrote a number into a field (`String(n)`).
fn text<T: ToString>(value: Option<T>) -> String {
    value.map_or_else(String::new, |value| value.to_string())
}

/// The Advanced area showing `spec`, as read back from a container.
pub fn of_spec(spec: &RunSpec) -> Advanced {
    let resources = Resources {
        cpus: text(spec.cpus),
        cpu_shares: text(spec.cpu_shares),
        memory: spec.memory.map(format_size).unwrap_or_default(),
        memory_reservation: spec.memory_reservation.map(format_size).unwrap_or_default(),
        memory_swap: match spec.memory_swap {
            Some(-1) => "-1".to_owned(),
            other => other.map(format_size).unwrap_or_default(),
        },
    };
    let gpus = match &spec.gpus {
        None => Gpus::default(),
        Some(GpuSpec::All) => Gpus {
            mode: GpuMode::All,
            ..Gpus::default()
        },
        Some(GpuSpec::Count { count }) => Gpus {
            mode: GpuMode::Count,
            count: count.to_string(),
            ..Gpus::default()
        },
        Some(GpuSpec::Ids { ids }) => Gpus {
            mode: GpuMode::Ids,
            ids: ids.join(", "),
            ..Gpus::default()
        },
    };
    let healthcheck = spec
        .healthcheck
        .as_ref()
        .map_or_else(Healthcheck::default, |h| {
            let duration = |seconds: Option<u32>| seconds.map(format_duration).unwrap_or_default();
            Healthcheck {
                mode: if h.disabled {
                    HealthMode::Disabled
                } else {
                    HealthMode::Custom
                },
                command: h.command.clone(),
                interval: duration(h.interval_s),
                timeout: duration(h.timeout_s),
                retries: text(h.retries),
                start_period: duration(h.start_period_s),
            }
        });
    Advanced {
        resources,
        labels: spec.labels.join("\n"),
        security: Security {
            cap_add: spec.cap_add.clone(),
            cap_drop: spec.cap_drop.clone(),
            privileged: spec.privileged,
            devices: spec
                .devices
                .iter()
                .map(|d| DeviceRow {
                    host_path: d.host_path.clone(),
                    container_path: d.container_path.clone().unwrap_or_default(),
                    permissions: d.permissions.clone(),
                })
                .collect(),
            gpus,
        },
        healthcheck,
        identity: Identity {
            hostname: spec.hostname.clone().unwrap_or_default(),
            domainname: spec.domainname.clone().unwrap_or_default(),
            dns: spec.dns.join("\n"),
            dns_search: spec.dns_search.join("\n"),
            extra_hosts: spec.extra_hosts.join("\n"),
        },
        logging: spec
            .log
            .as_ref()
            .map_or_else(Logging::default, |log| Logging {
                driver: log.driver.clone(),
                options: log.options.join("\n"),
            }),
    }
}

/// `cap_net_admin` as `NET_ADMIN`, the way the list shows names.
pub fn capability(name: &str) -> String {
    let upper = name.to_uppercase();
    upper
        .strip_prefix("CAP_")
        .map_or(upper.clone(), str::to_owned)
}

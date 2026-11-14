//! The Run dialog's form as typed, and the Run request it makes
//! (`model/runForm.ts`, `fromRunSpec.ts`, `duplicate.ts`, `commit.ts`).
//! The request is the crate's [`RunSpec`]: the one the Tauri command took,
//! which `homeport_engine::containers::run::create_body` turns into the
//! Engine's create request whichever app sent it.

pub mod advanced;
pub mod parse;

#[cfg(test)]
mod tests;

use homeport_engine::{
    containers::{
        commit::CommitSpec,
        run::{MountSpec, PortSpec, RestartPolicy, RunSpec},
    },
    images::Image,
};

pub use self::advanced::{Advanced, Field};
use crate::t;

/// A published port, as its row says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortRow {
    pub host_port: String,
    pub container_port: String,
    /// `tcp` or `udp`.
    pub protocol: String,
}

impl Default for PortRow {
    fn default() -> Self {
        Self {
            host_port: String::new(),
            container_port: String::new(),
            protocol: "tcp".to_owned(),
        }
    }
}

/// A volume or a bind mount, as its row says.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MountRow {
    pub source: String,
    pub target: String,
    pub read_only: bool,
}

/// The restart policies, in the order the select lists them.
pub const RESTART: [RestartPolicy; 4] = [
    RestartPolicy::No,
    RestartPolicy::OnFailure,
    RestartPolicy::UnlessStopped,
    RestartPolicy::Always,
];

/// What the select calls a restart policy.
pub fn restart_label(policy: RestartPolicy) -> String {
    match policy {
        RestartPolicy::No => t!("containers.run.restart.no"),
        RestartPolicy::OnFailure => t!("containers.run.restart.onFailure"),
        RestartPolicy::UnlessStopped => t!("containers.run.restart.unlessStopped"),
        RestartPolicy::Always => t!("containers.run.restart.always"),
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RunForm {
    pub image: String,
    pub name: String,
    pub command: String,
    /// One `KEY=value` a line.
    pub env: String,
    pub ports: Vec<PortRow>,
    pub mounts: Vec<MountRow>,
    pub network: String,
    pub restart: RestartPolicy,
    pub auto_remove: bool,
    pub advanced: Advanced,
    /// The container this form was read from (Duplicate, Edit), whose
    /// other settings are kept.
    pub based_on: Option<String>,
    /// The Podman pod to run it in, by name; empty for none. Its ports and
    /// network are the pod's.
    pub pod: String,
}

/// What can't be read in the form.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Errors {
    /// The Advanced area's fields, in the order React checked them.
    pub advanced: Vec<Field>,
    /// The rows of ports whose host or container port is not one.
    pub ports: Vec<usize>,
}

impl Errors {
    pub fn is_empty(&self) -> bool {
        self.advanced.is_empty() && self.ports.is_empty()
    }

    pub fn has(&self, field: Field) -> bool {
        self.advanced.contains(&field)
    }

    /// Why `field` can't be read, if it can't.
    pub fn message(&self, field: Field) -> Option<String> {
        self.has(field).then(|| field.message())
    }
}

/// A port as a row writes it: a number from 1 to 65 535.
fn port(text: &str) -> Option<u16> {
    text.trim().parse().ok().filter(|port| *port > 0)
}

fn blank(text: &str) -> Option<String> {
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}

impl RunForm {
    /// The empty form, with `image` filled in (`emptyRunForm`).
    pub fn new(image: &str) -> Self {
        Self {
            image: image.to_owned(),
            ..Self::default()
        }
    }

    fn in_pod(&self) -> bool {
        !self.pod.trim().is_empty()
    }

    /// The rows of ports that are sent: those with a container port, and
    /// none for a container run in a pod, whose ports are the pod's.
    fn sent_ports(&self) -> impl Iterator<Item = (usize, &PortRow)> {
        let in_pod = self.in_pod();
        self.ports
            .iter()
            .enumerate()
            .filter(move |(_, row)| !in_pod && !row.container_port.trim().is_empty())
    }

    /// What can't be read in the form; empty when it can be sent.
    pub fn errors(&self) -> Errors {
        let ports = self
            .sent_ports()
            .filter(|(_, row)| {
                port(&row.container_port).is_none()
                    || (!row.host_port.trim().is_empty() && port(&row.host_port).is_none())
            })
            .map(|(index, _)| index)
            .collect();
        Errors {
            advanced: advanced::errors(&self.advanced),
            ports,
        }
    }

    /// The Run request for the form (`toRunSpec`); rows left empty are
    /// dropped, and so are the ports and network of a container run into
    /// a pod. Call it once [`RunForm::errors`] is empty.
    pub fn to_spec(&self) -> RunSpec {
        let in_pod = self.in_pod();
        let mut spec = RunSpec {
            image: self.image.trim().to_owned(),
            name: blank(&self.name),
            command: blank(&self.command),
            env: advanced::lines(&self.env),
            ports: self
                .sent_ports()
                .filter_map(|(_, row)| {
                    Some(PortSpec {
                        host_ip: None,
                        host_port: port(&row.host_port),
                        container_port: port(&row.container_port)?,
                        protocol: row.protocol.clone(),
                    })
                })
                .collect(),
            mounts: self
                .mounts
                .iter()
                .filter(|m| !m.source.trim().is_empty() || !m.target.trim().is_empty())
                .map(|m| MountSpec {
                    source: m.source.trim().to_owned(),
                    target: m.target.trim().to_owned(),
                    read_only: m.read_only,
                })
                .collect(),
            network: if in_pod { None } else { blank(&self.network) },
            restart: self.restart,
            auto_remove: self.auto_remove,
            based_on: self.based_on.clone(),
            ..RunSpec::default()
        };
        advanced::apply(&self.advanced, &mut spec);
        spec
    }

    /// The pod the form runs the container in, for `containers_run`.
    pub fn pod(&self) -> Option<String> {
        blank(&self.pod)
    }

    /// The form showing `spec`, as read back from a container
    /// (`fromRunSpec`). Duplicate and Edit never run into a pod.
    pub fn of_spec(spec: &RunSpec) -> Self {
        Self {
            image: spec.image.clone(),
            name: spec.name.clone().unwrap_or_default(),
            command: spec.command.clone().unwrap_or_default(),
            env: spec.env.join("\n"),
            ports: spec
                .ports
                .iter()
                .map(|p| PortRow {
                    host_port: p.host_port.map(|p| p.to_string()).unwrap_or_default(),
                    container_port: p.container_port.to_string(),
                    protocol: p.protocol.clone(),
                })
                .collect(),
            mounts: spec
                .mounts
                .iter()
                .map(|m| MountRow {
                    source: m.source.clone(),
                    target: m.target.clone(),
                    read_only: m.read_only,
                })
                .collect(),
            network: spec.network.clone().unwrap_or_default(),
            restart: spec.restart,
            auto_remove: spec.auto_remove,
            advanced: advanced::of_spec(spec),
            based_on: spec.based_on.clone(),
            pod: String::new(),
        }
    }

    /// Whether the Advanced area has anything set, so it opens by itself.
    pub fn has_advanced(&self) -> bool {
        self.advanced != Advanced::default()
    }
}

/// `<name>-copy`, or `-copy-2`, `-copy-3`… when that is taken.
pub fn copy_name(name: &str, taken: &[&str]) -> String {
    let first = format!("{name}-copy");
    if !taken.contains(&first.as_str()) {
        return first;
    }
    (2..)
        .map(|n| format!("{name}-copy-{n}"))
        .find(|name| !taken.contains(&name.as_str()))
        .unwrap_or(first)
}

/// The fixed host ports in `form` that the `original` container
/// publishes too.
pub fn shared_host_ports(form: &RunForm, original: &RunForm) -> Vec<String> {
    let published: Vec<String> = original
        .ports
        .iter()
        .filter(|p| !p.host_port.trim().is_empty())
        .map(|p| format!("{}/{}", p.host_port.trim(), p.protocol))
        .collect();
    form.ports
        .iter()
        .filter(|p| {
            !p.host_port.trim().is_empty()
                && published.contains(&format!("{}/{}", p.host_port.trim(), p.protocol))
        })
        .map(|p| p.host_port.trim().to_owned())
        .collect()
}

/// The form of the Commit dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitForm {
    pub reference: String,
    pub comment: String,
    pub author: String,
    /// One Dockerfile instruction a line.
    pub changes: String,
    pub pause: bool,
}

/// `web:2026-11-04` for container `web` on that day (UTC), lower-cased as
/// image names must be.
pub fn commit_name(container: &str, today: &str) -> String {
    let repo: String = container
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' | '.' | '_' | '/' | '-' => c,
            _ => '-',
        })
        .collect();
    format!("{repo}:{today}")
}

/// Today's date in UTC, as `toISOString().slice(0, 10)`.
pub fn today() -> String {
    jiff::Timestamp::now()
        .to_zoned(jiff::tz::TimeZone::UTC)
        .date()
        .to_string()
}

impl CommitForm {
    /// The form for `container`, named after it and today.
    pub fn new(container: &str, today: &str) -> Self {
        Self {
            reference: commit_name(container, today),
            comment: String::new(),
            author: String::new(),
            changes: String::new(),
            pause: true,
        }
    }

    /// The request (`toCommitSpec`).
    pub fn to_spec(&self) -> CommitSpec {
        CommitSpec {
            reference: self.reference.trim().to_owned(),
            comment: blank(&self.comment),
            author: blank(&self.author),
            pause: self.pause,
            changes: self
                .changes
                .split('\n')
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_owned)
                .collect(),
        }
    }
}

/// The image that already has the name `reference`, whose tag the commit
/// moves (`imageNamed`).
pub fn image_named<'a>(reference: &str, images: &'a [Image]) -> Option<&'a Image> {
    let name = reference.trim();
    let last_part = name.rsplit('/').next().unwrap_or(name);
    let full = if last_part.contains(':') {
        name.to_owned()
    } else {
        format!("{name}:latest")
    };
    images.iter().find(|image| image.tags.contains(&full))
}

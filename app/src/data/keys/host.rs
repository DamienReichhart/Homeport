//! Keys of the Hosts and of what each one's Engine has (`hostKeys`,
//! `hostFeatureKeys` and `volumeKeys` in React).

use crate::data::Key;

/// What an Incus Host's pages list, each under its own key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncusResource {
    Instances,
    Images,
    Profiles,
    Storage,
    Networks,
}

impl IncusResource {
    fn part(self) -> &'static str {
        match self {
            Self::Instances => "instances",
            Self::Images => "images",
            Self::Profiles => "profiles",
            Self::Storage => "storage",
            Self::Networks => "networks",
        }
    }
}

/// The Hosts, with their statuses.
pub fn list() -> Key {
    Key::new(["hosts"])
}

/// Engines answering on this machine's sockets, Hosts or not.
pub fn local_engines() -> Key {
    Key::new(["local-engines"])
}

/// Everything read from one Host.
pub fn scope(host_id: &str) -> Key {
    Key::new(["host", host_id])
}

pub fn containers(host_id: &str) -> Key {
    scope(host_id).with("containers")
}

pub fn images(host_id: &str) -> Key {
    scope(host_id).with("images")
}

/// Under images, so a pull or removal refreshes it.
pub fn freshness(host_id: &str) -> Key {
    images(host_id).with("freshness")
}

pub fn volumes(host_id: &str) -> Key {
    scope(host_id).with("volumes")
}

/// Apart from volumes, so a container event doesn't make the Engine measure
/// them again.
pub fn volume_sizes(host_id: &str) -> Key {
    scope(host_id).with("volume-sizes")
}

pub fn networks(host_id: &str) -> Key {
    scope(host_id).with("networks")
}

pub fn stacks(host_id: &str) -> Key {
    scope(host_id).with("stacks")
}

/// What the Engine is and the disk it uses, each under it.
pub fn system(host_id: &str) -> Key {
    scope(host_id).with("system")
}

pub fn system_info(host_id: &str) -> Key {
    system(host_id).with("info")
}

pub fn disk_usage(host_id: &str) -> Key {
    system(host_id).with("df")
}

/// The latest page of the Host's kept Engine events.
pub fn events(host_id: &str) -> Key {
    scope(host_id).with("events")
}

/// Podman's pods, and each one's inspect document under it.
pub fn pods(host_id: &str) -> Key {
    scope(host_id).with("pods")
}

/// Podman's Quadlet units, and each one's file under it.
pub fn quadlets(host_id: &str) -> Key {
    scope(host_id).with("quadlets")
}

/// A containerd Host's namespaces, with what each holds.
pub fn containerd_namespaces(host_id: &str) -> Key {
    scope(host_id).with("containerd-namespaces")
}

/// A containerd Host's containers in `namespace`; without it, every
/// namespace's, for invalidating them all.
pub fn containerd_containers(host_id: &str, namespace: Option<&str>) -> Key {
    narrowed(scope(host_id).with("containerd-containers"), namespace)
}

/// A containerd Host's images in `namespace`, or every namespace's.
pub fn containerd_images(host_id: &str, namespace: Option<&str>) -> Key {
    narrowed(scope(host_id).with("containerd-images"), namespace)
}

/// A containerd Host's snapshots in `namespace`, or every namespace's.
pub fn containerd_snapshots(host_id: &str, namespace: Option<&str>) -> Key {
    narrowed(scope(host_id).with("containerd-snapshots"), namespace)
}

/// An Incus Host's projects, with what each holds.
pub fn incus_projects(host_id: &str) -> Key {
    scope(host_id).with("incus-projects")
}

/// An Incus Host's `resource` in `project`; without it, every project's,
/// for invalidating them all.
pub fn incus(host_id: &str, resource: IncusResource, project: Option<&str>) -> Key {
    narrowed(scope(host_id).with("incus").with(resource.part()), project)
}

/// The helper container's image (ADR 0015), and whether the Host has it.
pub fn volume_helper_image(host_id: &str) -> Key {
    scope(host_id).with("volume-helper-image")
}

/// A volume being browsed: each folder read is filed under its session.
pub fn volume_browse(host_id: &str, session: &str) -> Key {
    scope(host_id).with("volume-browse").with(session)
}

/// What to run on an SSH Host so it accepts Homeport's key. Under the
/// Hosts, so a changed Host asks again.
pub fn ssh_setup(host_id: &str) -> Key {
    list().with(host_id).with("ssh-setup")
}

/// What `source` (`sshConfig`, `dockerContexts`) offers to import.
pub fn import_preview(source: &str) -> Key {
    list().with("import").with(source)
}

/// What an Incus trust token says of the server it joins.
pub fn incus_trust_token(token: &str) -> Key {
    Key::new(["incus-trust-token", token])
}

/// The images an Incus image server offers a Host. Not under the Host: the
/// catalogue is the server's, and slow to read again.
pub fn incus_remote_images(host_id: &str, server: &str) -> Key {
    Key::new(["incus-remote-images", host_id, server])
}

fn narrowed(key: Key, by: Option<&str>) -> Key {
    match by {
        Some(part) => key.with(part),
        None => key,
    }
}

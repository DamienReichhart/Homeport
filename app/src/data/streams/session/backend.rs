//! What a terminal runs in: the five backends' exec and console, with
//! what they run and the size they are opened at.

use homeport_containerd::containers::exec::ContainerdExecOptions;
use homeport_engine::containers::exec::ExecOptions;
use homeport_incus::instances::terminal::IncusExecOptions;
use homeport_kubernetes::streams::terminal::PodExecOptions;
use homeport_nomad::allocations::exec::NomadExecOptions;

/// What a terminal runs in, and what it runs.
#[derive(Debug, Clone)]
pub enum Backend {
    /// A shell in a container of a Docker or Podman Engine.
    Engine {
        host_id: String,
        container_id: String,
        options: ExecOptions,
    },
    /// A process in a containerd container's running task.
    Containerd {
        host_id: String,
        namespace: String,
        id: String,
        options: ContainerdExecOptions,
    },
    /// A command in an Incus Instance.
    IncusExec {
        host_id: String,
        project: String,
        name: String,
        options: IncusExecOptions,
    },
    /// An Incus Instance's text console.
    IncusConsole {
        host_id: String,
        project: String,
        name: String,
        cols: u16,
        rows: u16,
    },
    /// A shell in a Pod's container.
    Pod {
        cluster_id: String,
        namespace: String,
        pod: String,
        options: PodExecOptions,
    },
    /// A command in a Nomad allocation's task.
    Nomad {
        cluster_id: String,
        alloc_id: String,
        options: NomadExecOptions,
    },
}

/// Which store of terminals a session is in: what its input, size and
/// end go through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Engine,
    Containerd,
    Incus,
    Pod,
    Nomad,
}

impl Backend {
    /// The backend with the terminal's size, as the session opens it:
    /// options left at their default would ask for a terminal of no cells.
    #[must_use]
    pub(super) fn sized(mut self, cols: u16, rows: u16) -> Self {
        let (cols, rows) = (cols.max(1), rows.max(1));
        match &mut self {
            Self::Engine { options, .. } => (options.cols, options.rows) = (cols, rows),
            Self::Containerd { options, .. } => (options.cols, options.rows) = (cols, rows),
            Self::IncusExec { options, .. } => (options.cols, options.rows) = (cols, rows),
            Self::IncusConsole {
                cols: c, rows: r, ..
            } => (*c, *r) = (cols, rows),
            Self::Pod { options, .. } => (options.cols, options.rows) = (cols, rows),
            Self::Nomad { options, .. } => (options.cols, options.rows) = (cols, rows),
        }
        self
    }

    /// The size the terminal is asked for.
    #[cfg(test)]
    pub(super) fn size(&self) -> (u16, u16) {
        let size = match self {
            Self::Engine { options, .. } => (options.cols, options.rows),
            Self::Containerd { options, .. } => (options.cols, options.rows),
            Self::IncusExec { options, .. } => (options.cols, options.rows),
            Self::IncusConsole { cols, rows, .. } => (*cols, *rows),
            Self::Pod { options, .. } => (options.cols, options.rows),
            Self::Nomad { options, .. } => (options.cols, options.rows),
        };
        (size.0.max(1), size.1.max(1))
    }

    pub fn kind(&self) -> Kind {
        match self {
            Self::Engine { .. } => Kind::Engine,
            Self::Containerd { .. } => Kind::Containerd,
            Self::IncusExec { .. } | Self::IncusConsole { .. } => Kind::Incus,
            Self::Pod { .. } => Kind::Pod,
            Self::Nomad { .. } => Kind::Nomad,
        }
    }

    /// What tells two terminals apart, past the page's own number: where
    /// they run and what.
    pub(super) fn identity(&self) -> Vec<&str> {
        match self {
            Self::Engine {
                host_id,
                container_id,
                options,
            } => vec![
                "engine",
                host_id,
                container_id,
                options.command.as_deref().unwrap_or_default(),
            ],
            Self::Containerd {
                host_id,
                namespace,
                id,
                options,
            } => vec![
                "containerd",
                host_id,
                namespace,
                id,
                options.command.as_deref().unwrap_or_default(),
            ],
            Self::IncusExec {
                host_id,
                project,
                name,
                options,
            } => vec![
                "incus-exec",
                host_id,
                project,
                name,
                options.command.as_deref().unwrap_or_default(),
            ],
            Self::IncusConsole {
                host_id,
                project,
                name,
                ..
            } => vec!["incus-console", host_id, project, name],
            Self::Pod {
                cluster_id,
                namespace,
                pod,
                options,
            } => vec![
                "pod",
                cluster_id,
                namespace,
                pod,
                options.container.as_deref().unwrap_or_default(),
                options.command.as_deref().unwrap_or_default(),
            ],
            Self::Nomad {
                cluster_id,
                alloc_id,
                options,
            } => vec![
                "nomad",
                cluster_id,
                alloc_id,
                &options.task,
                options.command.as_deref().unwrap_or_default(),
            ],
        }
    }
}

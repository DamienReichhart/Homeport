//! What the shell remembers in the Settings: the selection and the
//! namespace or project picked for each Host and Cluster. The change shows
//! at once; it is saved behind, off the UI thread.

use std::sync::Arc;

use iced::Task;

use homeport_core::error::AppError;
use homeport_settings::{Selection, Settings};

use crate::{api::app::settings, app::AppState};

/// One thing to remember.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Remember {
    Selection(Selection),
    /// A Cluster's Namespace; `None` is every Namespace, stored as `""`.
    ClusterNamespace {
        cluster_id: String,
        namespace: Option<String>,
    },
    ContainerdNamespace {
        host_id: String,
        namespace: String,
    },
    IncusProject {
        host_id: String,
        project: String,
    },
}

/// How saving went.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Saved {
    pub what: Remember,
    /// Why it couldn't be saved, if it couldn't.
    pub error: Option<Box<AppError>>,
}

/// All Namespaces, as the settings store the choice.
pub const ALL_NAMESPACES: &str = "";

impl Remember {
    /// Writes it into `settings`, as `patchSettings` did.
    pub fn apply(&self, settings: &mut Settings) {
        match self {
            Self::Selection(selection) => {
                if let Selection::Host { id } = selection {
                    // Keeps an older Homeport opening on the same Host.
                    settings.last_host_id = Some(id.clone());
                }
                settings.last_selection = Some(selection.clone());
            }
            Self::ClusterNamespace {
                cluster_id,
                namespace,
            } => {
                let stored = namespace
                    .clone()
                    .unwrap_or_else(|| ALL_NAMESPACES.to_owned());
                settings
                    .cluster_namespaces
                    .insert(cluster_id.clone(), stored);
            }
            Self::ContainerdNamespace { host_id, namespace } => {
                settings
                    .containerd_namespaces
                    .insert(host_id.clone(), namespace.clone());
            }
            Self::IncusProject { host_id, project } => {
                settings
                    .incus_projects
                    .insert(host_id.clone(), project.clone());
            }
        }
    }

    /// Saves it: the stored settings are read, changed and written back,
    /// so nothing another page saved meanwhile is undone.
    pub fn save(self, state: &Arc<AppState>) -> Task<Saved> {
        let state = state.clone();
        Task::perform(
            async move {
                let saved = settings::get_settings(&state).and_then(|mut stored| {
                    self.apply(&mut stored);
                    settings::update_settings(&state, stored)
                });
                Saved {
                    error: saved.err().map(Box::new),
                    what: self,
                }
            },
            |saved| saved,
        )
    }
}

//! What a container's page keeps: which container, which tab, what its
//! header's buttons and menu are doing, and each tab's own state.

use std::{collections::HashSet, sync::Arc};

use homeport_core::error::{AppError, Message as Text};
use homeport_engine::containers::{files::FileEntry, health::Health, Container, Processes};

use super::{
    model::inspect::Inspected,
    queries,
    tabs::{files::Files, health, inspect::Inspect, logs::Logs, processes, stats::Stats},
};
use crate::{
    data::{keys, Key, Query, Resource},
    t,
    ui::{
        domain::containers::{Actions, Target, CAME_FROM},
        shell::{router::Route, Context},
        widgets::{basic::technical, overlay::menu},
    },
};

/// How a container was when the log stream of this number ended: what
/// tells, once the list is read again, whether it started again.
#[derive(Debug)]
pub(super) struct LogsEnded {
    pub(super) generation: u64,
    /// Its status then (`Up 3 hours`).
    pub(super) status: Option<String>,
    /// It was seen not running since.
    pub(super) stopped_since: bool,
}

/// The window's width from which cards stand two a row (`lg`).
pub(super) const TWO_COLUMNS_FROM: f32 = 1024.0;

/// What stands in place of a container that is not in the Host's list.
pub(super) fn gone() -> AppError {
    AppError::not_found(Text {
        key: "containers.detail.gone",
        en: "This container no longer exists.",
    })
}

/// The catalogue's words for an id several containers' start with.
const AMBIGUOUS: &str = "containers.detail.ambiguous";

/// What stands in place of a container whose id several start with.
pub(super) fn ambiguous(id: &str) -> AppError {
    let text = Text {
        key: AMBIGUOUS,
        en: "More than one container's id starts with {id}.",
    };
    AppError::not_found(text.with("id", technical(id)))
}

/// A tab of the page, in the order they show (`TABS`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tab {
    Logs,
    Inspect,
    Health,
    Exec,
    Files,
    Stats,
    Processes,
}

impl Tab {
    pub const ALL: [Self; 7] = [
        Self::Logs,
        Self::Inspect,
        Self::Health,
        Self::Exec,
        Self::Files,
        Self::Stats,
        Self::Processes,
    ];

    /// What the route calls it (`containers/:id/:tab?`).
    pub fn slug(self) -> &'static str {
        match self {
            Self::Logs => "logs",
            Self::Inspect => "inspect",
            Self::Health => "health",
            Self::Exec => "exec",
            Self::Files => "files",
            Self::Stats => "stats",
            Self::Processes => "processes",
        }
    }

    /// The tab a route names; the logs when it names none, or one there
    /// isn't.
    pub fn of(slug: Option<&str>) -> Self {
        Self::ALL
            .into_iter()
            .find(|tab| Some(tab.slug()) == slug)
            .unwrap_or(Self::Logs)
    }

    pub fn label(self) -> String {
        match self {
            Self::Logs => t!("containers.tab.logs"),
            Self::Inspect => t!("containers.tab.inspect"),
            Self::Health => t!("containers.tab.health"),
            Self::Exec => t!("containers.tab.exec"),
            Self::Files => t!("containers.tab.files"),
            Self::Stats => t!("containers.tab.stats"),
            Self::Processes => t!("containers.tab.processes"),
        }
    }

    /// The tabs a container has: Health only with a healthcheck.
    pub fn shown(container: &Container) -> Vec<Self> {
        Self::ALL
            .into_iter()
            .filter(|tab| *tab != Self::Health || container.health.is_some())
            .collect()
    }
}

#[derive(Debug)]
pub struct State {
    /// The Host the page is about: the one its gate let through.
    pub(super) host_id: String,
    /// The container's id as the route has it: whole, or its start.
    pub(super) id: String,
    pub(super) tab: Tab,
    /// The container's whole id, as it was last found in the list.
    pub(super) found: Option<String>,
    /// The page was opened from a list that is not its Host's Containers
    /// page: Back returns there.
    pub(super) came_from_elsewhere: bool,
    /// The header's buttons, and the confirmations and the Rename dialog
    /// they open: the same as a list's.
    pub(super) actions: Actions,
    /// The menu beside them.
    pub(super) more: menu::State,
    /// The window's size, once it said: the log viewer takes the height
    /// the page leaves, and the charts stand two a row when it is wide.
    pub(super) window: Option<iced::Size>,
    /// The container's state when the page last looked.
    pub(super) seen_state: Option<String>,
    /// How the container was when its log stream ended.
    pub(super) logs_ended: Option<LogsEnded>,
    /// What the page already asked the shell to read: a resource is
    /// asked for once, when the tab that shows it first needs it.
    pub(super) watched: HashSet<Key>,
    pub(super) logs: Logs,
    pub(super) inspect: Inspect,
    pub(super) processes: processes::Tab,
    pub(super) health: health::Tab,
    pub(super) files: Files,
    pub(super) stats: Stats,
}

impl State {
    pub(super) fn new(ctx: &Context<'_>) -> Self {
        let (id, tab) = match &ctx.location.route {
            Route::Container { id, tab } => (id.clone(), Tab::of(tab.as_deref())),
            _ => (String::new(), Tab::Logs),
        };
        Self {
            host_id: ctx.subject.host_id().unwrap_or_default().to_owned(),
            id,
            tab,
            found: None,
            came_from_elsewhere: ctx.location.param(CAME_FROM).is_some(),
            actions: Actions::of_host(),
            more: menu::State::default(),
            window: None,
            seen_state: None,
            logs_ended: None,
            watched: HashSet::new(),
            logs: Logs::new(),
            inspect: Inspect::default(),
            processes: processes::Tab::default(),
            health: health::Tab::default(),
            files: Files::new(),
            stats: Stats::new(),
        }
    }

    /// The Host's containers, as the data layer has them.
    pub(super) fn containers(&self, ctx: &Context<'_>) -> Resource<Arc<Vec<Container>>> {
        ctx.data.read(&keys::host::containers(&self.host_id))
    }

    /// The container the page is about, among `list`: the one whose id is
    /// the route's, else the one of that name, else the one whose id starts
    /// with it if no other does. A start several ids share names none:
    /// the page would otherwise be about whichever came first.
    pub(super) fn find<'a>(&self, list: &'a [Container]) -> Option<&'a Container> {
        let exact = list.iter().find(|container| container.id == self.id);
        let named = || list.iter().find(|container| container.name == self.id);
        exact.or_else(named).or_else(|| {
            // An empty id is the start of every id, and names none.
            if self.id.is_empty() {
                return None;
            }
            let mut starting = list.iter().filter(|c| c.id.starts_with(&self.id));
            match (starting.next(), starting.next()) {
                (Some(only), None) => Some(only),
                _ => None,
            }
        })
    }

    /// Why no container of `list` is the page's: none is, or several ids
    /// start with what the route names.
    pub(super) fn missing(&self, list: &[Container]) -> AppError {
        let starting = list.iter().filter(|c| c.id.starts_with(&self.id)).count();
        if !self.id.is_empty() && starting > 1 {
            ambiguous(&self.id)
        } else {
            gone()
        }
    }

    /// The container the page is about, as the data layer has it now.
    pub(super) fn container(&self, ctx: &Context<'_>) -> Option<Container> {
        let containers = self.containers(ctx);
        let list = containers.data().map_or(&[][..], |list| list.as_slice());
        self.find(list).cloned()
    }

    /// Asks the shell to read `query` and keep it fresh while the page
    /// shows it, once, and counts it among what is `wanted` now.
    pub(super) fn watch<T: Send + Sync + 'static>(
        &mut self,
        query: Query<T>,
        wanted: &mut HashSet<Key>,
        ctx: &Context<'_>,
    ) {
        wanted.insert(query.key().clone());
        if self.watched.insert(query.key().clone()) {
            ctx.watch(query);
        }
    }

    /// Lets go of what the page watched and no longer shows: a tab that
    /// was left, a folder it moved on from, the document of a state the
    /// container is no longer in.
    pub(super) fn unwatch_all_but(&mut self, wanted: &HashSet<Key>, ctx: &Context<'_>) {
        let gone: Vec<Key> = self
            .watched
            .iter()
            .filter(|key| !wanted.contains(*key))
            .cloned()
            .collect();
        for key in gone {
            self.watched.remove(&key);
            ctx.unwatch(key);
        }
    }

    /// The inspect document of `container`, as the data layer has it.
    pub(super) fn inspected(
        &self,
        container: &Container,
        ctx: &Context<'_>,
    ) -> Resource<Arc<Inspected>> {
        let key = queries::inspect_key(&self.host_id, &container.id, &container.state);
        ctx.data.read(&key)
    }

    /// What runs in `container`, as the data layer has it.
    pub(super) fn processes_read(
        &self,
        container: &Container,
        ctx: &Context<'_>,
    ) -> Resource<Arc<Processes>> {
        ctx.data
            .read(&queries::processes_key(&self.host_id, &container.id))
    }

    /// What the folder the Files tab shows holds.
    pub(super) fn files_read(
        &self,
        container: &Container,
        ctx: &Context<'_>,
    ) -> Resource<Arc<Vec<FileEntry>>> {
        let key = queries::files_key(&self.host_id, &container.id, &self.files.path);
        ctx.data.read(&key)
    }

    /// The healthcheck of `container`, read for the status the list
    /// shows.
    pub(super) fn health_read(
        &self,
        container: &Container,
        ctx: &Context<'_>,
    ) -> Resource<Arc<Option<Health>>> {
        let key = queries::health_key(&self.host_id, &container.id, container.health.as_deref());
        ctx.data.read(&key)
    }

    /// Whether the window is wide enough for two cards a row
    /// (`lg:grid-cols-2`); it is until the window said its size.
    pub(super) fn is_wide(&self) -> bool {
        self.window
            .is_none_or(|size| size.width >= TWO_COLUMNS_FROM)
    }

    /// The container, as an action is asked for it.
    pub(super) fn target(&self, container: &Container, ctx: &Context<'_>) -> Target {
        let name = ctx.host().map_or("", |view| view.host.name.as_str());
        Target::of(&self.host_id, name, container)
    }

    /// The route of a tab of this page.
    /// It names the container by its whole id once it was found, so the
    /// route stays about it whatever the list holds later.
    pub(super) fn route(&self, tab: Tab) -> Route {
        Route::Container {
            id: self.found.clone().unwrap_or_else(|| self.id.clone()),
            tab: Some(tab.slug().to_owned()),
        }
    }
}

//! What a container's page keeps: which container, which tab, what its
//! header's buttons and menu are doing, and each tab's own state.

use std::{collections::HashSet, sync::Arc};

use homeport_core::error::{AppError, Message as Text};
use homeport_engine::containers::{health::Health, Container, Processes};

use super::{
    model::inspect::Inspected,
    queries,
    tabs::{health, inspect::Inspect, logs::Logs, processes, stats::Stats},
};
use crate::{
    data::{keys, Key, Query, Resource},
    t,
    ui::{
        domain::containers::{Actions, Target, CAME_FROM},
        shell::{router::Route, Context},
        widgets::overlay::menu,
    },
};

/// The window's width from which cards stand two a row (`lg`).
pub(super) const TWO_COLUMNS_FROM: f32 = 1024.0;

/// What stands in place of a container that is not in the Host's list.
pub(super) fn gone() -> AppError {
    AppError::not_found(Text {
        key: "containers.detail.gone",
        en: "This container no longer exists.",
    })
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
    /// What the page already asked the shell to read: a resource is
    /// asked for once, when the tab that shows it first needs it.
    pub(super) watched: HashSet<Key>,
    pub(super) logs: Logs,
    pub(super) inspect: Inspect,
    pub(super) processes: processes::Tab,
    pub(super) health: health::Tab,
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
            came_from_elsewhere: ctx.location.param(CAME_FROM).is_some(),
            actions: Actions::of_host(),
            more: menu::State::default(),
            window: None,
            seen_state: None,
            watched: HashSet::new(),
            logs: Logs::new(),
            inspect: Inspect::default(),
            processes: processes::Tab::default(),
            health: health::Tab::default(),
            stats: Stats::new(),
        }
    }

    /// The Host's containers, as the data layer has them.
    pub(super) fn containers(&self, ctx: &Context<'_>) -> Resource<Arc<Vec<Container>>> {
        ctx.data.read(&keys::host::containers(&self.host_id))
    }

    /// The container the page is about, among `list`: the one whose id is
    /// the route's, or starts with it.
    pub(super) fn find<'a>(&self, list: &'a [Container]) -> Option<&'a Container> {
        list.iter()
            .find(|container| container.id == self.id)
            .or_else(|| {
                // An empty id is the start of every id, and names none.
                (!self.id.is_empty())
                    .then(|| list.iter().find(|c| c.id.starts_with(&self.id)))
                    .flatten()
            })
    }

    /// The container the page is about, as the data layer has it now.
    pub(super) fn container(&self, ctx: &Context<'_>) -> Option<Container> {
        let containers = self.containers(ctx);
        let list = containers.data().map_or(&[][..], |list| list.as_slice());
        self.find(list).cloned()
    }

    /// Asks the shell to read `query` and keep it fresh while the page
    /// shows, once.
    pub(super) fn watch<T: Send + Sync + 'static>(&mut self, query: Query<T>, ctx: &Context<'_>) {
        if self.watched.insert(query.key().clone()) {
            ctx.watch(query);
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
    pub(super) fn route(&self, tab: Tab) -> Route {
        Route::Container {
            id: self.id.clone(),
            tab: Some(tab.slug().to_owned()),
        }
    }
}

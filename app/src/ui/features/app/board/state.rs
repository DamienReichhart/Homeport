//! What the Board keeps: its table, its filter, each Host's usage, and
//! the rows it made of every connected Host's containers.

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use homeport_engine::containers::{stats::ContainerUsage, Container};
use homeport_hosts::service::HostView;

use super::model::{self, Filter, Row, Source};
use crate::{
    data::{keys, resources},
    ui::{
        domain::containers::{model::Usage, Actions},
        shell::{registry::Capability, status, Context},
        widgets::{
            form::select,
            overlay::menu,
            table::{self, Mode, Order, Sort},
        },
    },
};

/// `ROW_HEIGHT`: a row holds a name over a short id.
pub(super) const ROW_HEIGHT: f32 = 53.0;

#[derive(Debug)]
pub struct State {
    pub(super) table: Box<table::State<String>>,
    pub(super) filter: Filter,
    /// Each Host's usage, as last reported, by Host then by container.
    pub(super) usage: HashMap<String, Usage>,
    pub(super) actions: Actions,
    pub(super) hosts_menu: menu::State,
    pub(super) state_select: select::State,
    pub(super) stack_select: select::State,
    /// The Hosts whose containers the page asked to read.
    watched: HashSet<String>,
    /// The connected Hosts and their lists, as the rows were made of them.
    pub(super) sources: Vec<Source>,
    /// Every container of every connected Host, by Host then by name.
    pub(super) rows: Vec<Row>,
    /// The Stacks among the rows, for the filter.
    pub(super) stacks: Vec<String>,
    /// How many containers run, and how many there are.
    pub(super) counts: (usize, usize),
    /// Changes whenever the rows, the filter or the usage do: the table
    /// goes through twenty thousand rows again only then.
    pub(super) revision: u64,
}

/// The Hosts whose Engine speaks the Docker Engine API (`useEveryHost`):
/// the Board asks those that are connected and names the others.
pub(super) fn docker_hosts<'a>(ctx: &Context<'a>) -> impl Iterator<Item = &'a HostView> {
    ctx.world
        .host_list()
        .iter()
        .filter(|view| status::engine::supports(Some(view), Capability::Docker))
}

pub(super) fn is_connected(view: &HostView) -> bool {
    status::host::is_connected(&view.status)
}

impl State {
    pub(super) fn new(filter: Filter) -> Self {
        Self {
            table: Box::new(
                table::State::new()
                    .selecting(Mode::Multi)
                    // A press opens the container on its Host.
                    .opening_on_press()
                    // By one column or another, never by none, as React's.
                    .always_sorted()
                    .sorted(Sort::by("host", Order::Ascending))
                    .row_height(ROW_HEIGHT),
            ),
            filter,
            usage: HashMap::new(),
            actions: Actions::across_hosts(),
            hosts_menu: menu::State::default(),
            state_select: select::State::default(),
            stack_select: select::State::default(),
            watched: HashSet::new(),
            sources: vec![],
            rows: vec![],
            stacks: vec![],
            counts: (0, 0),
            revision: 0,
        }
    }

    /// The rows or what is shown of them changed.
    pub(super) fn touch(&mut self) {
        self.revision += 1;
    }

    pub(super) fn usage_of(&self, row: &Row) -> Option<&ContainerUsage> {
        self.usage.get(&*row.host_id)?.get(&row.container().id)
    }

    /// Watches the containers of the Hosts that connected since, and
    /// makes the rows again when a Host came, went, or was read again.
    /// While every list is the one it has seen, it does nothing.
    pub(super) fn derive(&mut self, ctx: &Context<'_>) {
        let mut sources = vec![];
        for view in docker_hosts(ctx).filter(|view| is_connected(view)) {
            let id = &view.host.id;
            if self.watched.insert(id.clone()) {
                ctx.watch(resources::containers(id));
            }
            let containers = ctx
                .data
                .read::<Vec<Container>>(&keys::host::containers(id))
                .data()
                .cloned();
            sources.push(Source {
                host_id: Arc::from(id.as_str()),
                host_name: Arc::from(view.host.name.as_str()),
                containers,
            });
        }
        let same = sources.len() == self.sources.len()
            && sources.iter().zip(&self.sources).all(|(a, b)| a.is(b));
        if same {
            return;
        }
        self.rows = model::rows(&sources);
        self.stacks = model::stacks_of(&self.rows);
        let running = self
            .rows
            .iter()
            .filter(|row| row.container().state == "running")
            .count();
        self.counts = (running, self.rows.len());
        // A Host that went takes its usage with it.
        self.usage.retain(|host, _| {
            sources
                .iter()
                .any(|source| &*source.host_id == host.as_str())
        });
        self.sources = sources;
        self.touch();
    }
}

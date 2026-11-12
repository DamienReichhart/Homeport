//! What the Containers page keeps: its table, its search, the usage it
//! was told, and what it took from the data layer for its rows.

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use homeport_engine::{
    containers::Container,
    images::freshness::{Freshness, FreshnessResult},
    podman::pods::Pod,
};

use crate::{
    data::{keys, Resource},
    ui::{
        domain::{
            containers::{model::Usage, Actions},
            freshness, pods,
        },
        shell::Context,
        widgets::table::{self, Mode, Order, Sort},
    },
};

/// `h-[53px]`: a row holds a name over a short id.
pub(super) const ROW_HEIGHT: f32 = 53.0;

/// A list as it was read last, to tell whether it is the same one.
type Read<T> = Option<Arc<Vec<T>>>;

fn same<T>(a: &Read<T>, b: &Read<T>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
        _ => false,
    }
}

#[derive(Debug)]
pub struct State {
    /// The Host the page is about: the one its gate let through.
    pub(super) host_id: String,
    /// The sort, the selection, the closed Stacks, the row the keyboard is
    /// on. Boxed: a page's state is one variant among fifty.
    pub(super) table: Box<table::State<String>>,
    pub(super) query: String,
    pub(super) only_running: bool,
    /// The CPU and memory of the running containers, as last reported.
    pub(super) usage: Usage,
    pub(super) actions: Actions,
    pub(super) check: freshness::Check,
    /// The Run dialog was asked for. The dialog is the run ticket's
    /// (#259), which draws it from here; until then a stand-in shows.
    pub(super) run_asked: bool,
    /// What the three below were made from.
    seen: (Read<Container>, Read<FreshnessResult>, Read<Pod>),
    /// The containers whose tag has a newer image in its registry, by id.
    pub(super) newer: HashSet<String>,
    /// The Podman pod of each container that is in one: its id and name.
    pub(super) pods: HashMap<String, (String, String)>,
    /// How many containers run, and how many there are.
    pub(super) counts: (usize, usize),
}

impl State {
    pub(super) fn new(host_id: &str, query: Option<&str>) -> Self {
        Self {
            host_id: host_id.to_owned(),
            table: Box::new(
                table::State::new()
                    .selecting(Mode::Multi)
                    // A press opens the container, as the React row did;
                    // the checkboxes select.
                    .opening_on_press()
                    .sorted(Sort::by("name", Order::Ascending))
                    .row_height(ROW_HEIGHT),
            ),
            query: query.unwrap_or_default().to_owned(),
            only_running: false,
            usage: Usage::new(),
            actions: Actions::of_host(),
            check: freshness::Check::default(),
            run_asked: false,
            seen: (None, None, None),
            newer: HashSet::new(),
            pods: HashMap::new(),
            counts: (0, 0),
        }
    }

    /// The Host's containers, as the data layer has them.
    pub(super) fn containers(&self, ctx: &Context<'_>) -> Resource<Arc<Vec<Container>>> {
        ctx.data.read(&keys::host::containers(&self.host_id))
    }

    /// Takes from the data layer what the rows look up: which containers
    /// have a newer image, which pod each is in, how many run. Nothing is
    /// worked out again while the lists are the ones it has seen.
    pub(super) fn derive(&mut self, ctx: &Context<'_>) {
        let host = &self.host_id;
        let containers = self.containers(ctx).data().cloned();
        let checked = ctx
            .data
            .read::<Vec<FreshnessResult>>(&keys::host::freshness(host))
            .data()
            .cloned();
        let in_pods = ctx
            .data
            .read::<Vec<Pod>>(&keys::host::pods(host))
            .data()
            .cloned();
        let (was, was_checked, was_in_pods) = &self.seen;
        if same(was, &containers) && same(was_checked, &checked) && same(was_in_pods, &in_pods) {
            return;
        }
        let list = containers.as_deref().map_or(&[][..], Vec::as_slice);
        let index = freshness::index(checked.as_deref().map(Vec::as_slice));
        self.newer = list
            .iter()
            .filter(|container| index.of_container(container) == Freshness::Newer)
            .map(|container| container.id.clone())
            .collect();
        self.pods = pods::by_container(in_pods.as_deref().map_or(&[][..], Vec::as_slice))
            .into_iter()
            .map(|(id, pod)| (id.to_owned(), (pod.id.to_owned(), pod.name.to_owned())))
            .collect();
        let running = list.iter().filter(|c| c.state == "running").count();
        self.counts = (running, list.len());
        self.seen = (containers, checked, in_pods);
    }
}

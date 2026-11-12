//! The Board's pure parts (`model/board.ts`): every Host's containers as
//! rows, the filter kept in the page's location, and how a column sorts.

#[cfg(test)]
mod tests;

use std::{cmp::Ordering, sync::Arc};

use homeport_engine::containers::{stats::ContainerUsage, Container};

use crate::ui::{domain::containers::Target, shell::router::Location, widgets::table::sort};

/// A connected Host and the containers read from it, if they were.
#[derive(Debug, Clone)]
pub struct Source {
    pub host_id: Arc<str>,
    pub host_name: Arc<str>,
    pub containers: Option<Arc<Vec<Container>>>,
}

impl Source {
    /// Whether `other` is the same Host with the same list, read once.
    pub fn is(&self, other: &Self) -> bool {
        let same_list = match (&self.containers, &other.containers) {
            (None, None) => true,
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            _ => false,
        };
        same_list && self.host_id == other.host_id && self.host_name == other.host_name
    }
}

/// One container on the Board, with the Host it runs on. It holds its
/// Host's list and its place there: twenty thousand rows copy no container.
#[derive(Debug, Clone)]
pub struct Row {
    /// Unique across Hosts: `<hostId>/<containerId>`.
    pub key: String,
    pub host_id: Arc<str>,
    pub host_name: Arc<str>,
    list: Arc<Vec<Container>>,
    at: usize,
}

impl Row {
    pub fn container(&self) -> &Container {
        &self.list[self.at]
    }

    pub fn target(&self) -> Target {
        Target::of(&self.host_id, &self.host_name, self.container())
    }
}

pub fn row_key(host_id: &str, container_id: &str) -> String {
    format!("{host_id}/{container_id}")
}

/// Every Host's containers as rows, by Host then by name: the order ties
/// keep whatever column sorts, and the order of a Board that isn't sorted.
pub fn rows(sources: &[Source]) -> Vec<Row> {
    let mut rows: Vec<Row> = sources
        .iter()
        .filter_map(|source| Some((source, source.containers.as_ref()?)))
        .flat_map(|(source, list)| {
            list.iter().enumerate().map(|(at, container)| Row {
                key: row_key(&source.host_id, &container.id),
                host_id: source.host_id.clone(),
                host_name: source.host_name.clone(),
                list: list.clone(),
                at,
            })
        })
        .collect();
    rows.sort_by(|a, b| {
        sort::natural(&a.host_name, &b.host_name)
            .then_with(|| sort::natural(&a.container().name, &b.container().name))
    });
    rows
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StateFilter {
    #[default]
    All,
    Running,
    Stopped,
    Unhealthy,
}

impl StateFilter {
    pub const ALL: [Self; 4] = [Self::All, Self::Running, Self::Stopped, Self::Unhealthy];

    /// As the address wrote it (`?state=running`).
    fn name(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Running => "running",
            Self::Stopped => "stopped",
            Self::Unhealthy => "unhealthy",
        }
    }

    fn matches(self, container: &Container) -> bool {
        match self {
            Self::All => true,
            Self::Running => container.state == "running",
            Self::Stopped => container.state != "running",
            Self::Unhealthy => container.health.as_deref() == Some("unhealthy"),
        }
    }
}

/// What the Board shows of all there is.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Filter {
    /// The Hosts shown; empty for every Host.
    pub hosts: Vec<String>,
    pub state: StateFilter,
    /// One Stack's containers; `None` for any.
    pub stack: Option<String>,
    pub query: String,
}

impl Filter {
    /// The filter the location carries:
    /// `?host=a&host=b&state=running&stack=web&q=api` (`readFilter`).
    pub fn read(location: &Location) -> Self {
        let state = location.param("state");
        Self {
            hosts: location.params_of("host").map(str::to_owned).collect(),
            state: StateFilter::ALL
                .into_iter()
                .find(|filter| Some(filter.name()) == state)
                .unwrap_or_default(),
            stack: location
                .param("stack")
                .filter(|stack| !stack.is_empty())
                .map(str::to_owned),
            query: location.query.clone().unwrap_or_default(),
        }
    }

    /// The search text and the parameters to keep in the location
    /// (`writeFilter`).
    pub fn write(&self) -> (Option<String>, Vec<(String, String)>) {
        let mut params: Vec<(String, String)> = self
            .hosts
            .iter()
            .map(|host| ("host".to_owned(), host.clone()))
            .collect();
        if self.state != StateFilter::All {
            params.push(("state".to_owned(), self.state.name().to_owned()));
        }
        if let Some(stack) = &self.stack {
            params.push(("stack".to_owned(), stack.clone()));
        }
        let query = (!self.query.is_empty()).then(|| self.query.clone());
        (query, params)
    }

    /// Whether anything is filtered out: "Clear filters" shows.
    pub fn is_set(&self) -> bool {
        *self != Self::default()
    }

    pub fn shows_host(&self, host_id: &str) -> bool {
        self.hosts.is_empty() || self.hosts.iter().any(|host| host == host_id)
    }

    /// Whether the row passes; the search looks in the name, the short
    /// id, the image, the Stack and the Host's name (`filterRows`).
    pub fn keeps(&self, row: &Row) -> bool {
        let container = row.container();
        if !self.shows_host(&row.host_id) || !self.state.matches(container) {
            return false;
        }
        if self.stack.is_some() && container.stack != self.stack {
            return false;
        }
        let query = self.query.trim().to_lowercase();
        if query.is_empty() {
            return true;
        }
        let short_id = crate::i18n::format::short_id(&container.id);
        let stack = container.stack.as_deref().unwrap_or_default();
        [
            &container.name,
            short_id,
            &container.image,
            stack,
            &row.host_name,
        ]
        .iter()
        .any(|text| text.to_lowercase().contains(&query))
    }
}

/// The Stacks the rows belong to, sorted, for the Stack filter.
pub fn stacks_of(rows: &[Row]) -> Vec<String> {
    let mut stacks: Vec<String> = rows
        .iter()
        .filter_map(|row| row.container().stack.clone())
        .filter(|stack| !stack.is_empty())
        .collect();
    stacks.sort();
    stacks.dedup();
    stacks
}

/// What a column of the Board sorts by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Host,
    Name,
    Image,
    Status,
    Stack,
    Cpu,
    Memory,
    Created,
}

/// Text whatever its case, character by character, as `toLowerCase()`
/// compared with `<`.
fn caseless(a: &str, b: &str) -> Ordering {
    let lower = |text: &'_ str| {
        text.chars()
            .flat_map(char::to_lowercase)
            .collect::<String>()
    };
    lower(a).cmp(&lower(b))
}

/// How two rows compare by `key` (`sortRows`). Rows that compare equal
/// keep the order of [`rows`], by Host then by name, whichever way the
/// column sorts: the table's sort is stable.
pub fn compare<'a>(
    key: SortKey,
    a: &'a Row,
    b: &'a Row,
    usage: impl Fn(&'a Row) -> Option<&'a ContainerUsage>,
) -> Ordering {
    let (x, y) = (a.container(), b.container());
    match key {
        SortKey::Host => caseless(&a.host_name, &b.host_name),
        SortKey::Name => caseless(&x.name, &y.name),
        SortKey::Image => caseless(&x.image, &y.image),
        SortKey::Status => x.state.cmp(&y.state),
        SortKey::Stack => x
            .stack
            .as_deref()
            .unwrap_or("")
            .cmp(y.stack.as_deref().unwrap_or("")),
        // A container that reports nothing is under every one that does.
        SortKey::Cpu => {
            let of = |row| usage(row).map_or(-1.0, |usage| usage.cpu_percent);
            of(a).partial_cmp(&of(b)).unwrap_or(Ordering::Equal)
        }
        SortKey::Memory => {
            let of = |row| usage(row).map(|usage| usage.memory_usage);
            of(a).cmp(&of(b))
        }
        SortKey::Created => x.created.cmp(&y.created),
    }
}

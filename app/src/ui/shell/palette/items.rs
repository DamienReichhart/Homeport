//! The palette's rows and how they rank (`src/app/palette/model/items.ts`).

use homeport_hosts::service::HostView;

use crate::{
    t,
    ui::{
        shell::{
            registry::{nav, Command, Feature, Heading, Run},
            router::Route,
            status,
        },
        theme::{tokens::Tone, Icon},
    },
};

/// The heading a row is listed under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Recent,
    GoTo,
    Hosts,
    Actions,
    Settings,
}

impl Group {
    pub fn label(self) -> String {
        match self {
            Self::Recent => t!("app.palette.recent"),
            Self::GoTo => t!("app.palette.goTo"),
            Self::Hosts => t!("app.palette.hosts"),
            Self::Actions => t!("app.palette.actions"),
            Self::Settings => t!("settings.palette.group"),
        }
    }
}

/// What choosing a row does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Act {
    /// Opens a page.
    Open(Route),
    /// Selects a Host.
    SelectHost(String),
    /// Runs a command.
    Run(Run),
}

/// One row of the palette.
#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub group: Group,
    pub label: String,
    /// What it's found by besides its label.
    pub keywords: Vec<String>,
    pub icon: Option<Icon>,
    pub shortcut: Option<String>,
    /// A Host's status, next to its name.
    pub status: Option<(String, Tone)>,
    pub act: Act,
}

/// What the palette lists from.
pub struct Source<'a> {
    /// The features whose pages show for the selection.
    pub features: Vec<&'a Feature>,
    /// Commands of the shell itself, such as Show keyboard shortcuts.
    pub app_commands: &'a [Command],
    pub hosts: &'a [HostView],
    /// The selected Host, whose status hides commands that need it connected.
    pub selected: Option<&'a HostView>,
    /// The sidebar entries' keys, by path.
    pub nav_keys: Vec<(String, String)>,
}

/// The features the palette offers: those without a sidebar entry, and
/// those whose entry shows. The pages of what isn't selected, and their
/// commands, aren't offered.
pub fn shown<'a>(features: &[&'a Feature], entries: &[nav::Entry]) -> Vec<&'a Feature> {
    features
        .iter()
        .copied()
        .filter(|feature| {
            feature.nav.is_none() || entries.iter().any(|entry| entry.feature == feature.id)
        })
        .collect()
}

/// Every item the palette lists, in its groups' order: Go to, Hosts, then
/// commands.
pub fn items(source: &Source<'_>) -> Vec<Item> {
    let connected = source
        .selected
        .is_some_and(|host| status::host::is_connected(&host.status));
    let mut items: Vec<Item> = source
        .features
        .iter()
        .filter_map(|feature| feature.nav.as_ref())
        .map(|entry| {
            let path = entry.route.path();
            let page = (entry.label)();
            Item {
                shortcut: source
                    .nav_keys
                    .iter()
                    .find(|(of, _)| of == &path)
                    .map(|(_, keys)| keys.clone()),
                id: format!("nav:{path}"),
                group: Group::GoTo,
                label: t!("app.shortcuts.goTo", page = page.as_str()),
                keywords: vec![page],
                icon: Some(entry.icon),
                status: None,
                act: Act::Open(entry.route.clone()),
            }
        })
        .collect();
    for view in source.hosts {
        let summary = status::host::describe(&view.status);
        items.push(Item {
            id: format!("host:{}", view.host.id),
            group: Group::Hosts,
            label: t!("app.palette.switchTo", name = view.host.name.as_str()),
            keywords: vec![view.host.name.clone()],
            icon: Some(Icon::Server),
            shortcut: None,
            status: Some((summary.label, summary.tone)),
            act: Act::SelectHost(view.host.id.clone()),
        });
    }
    let commands = source
        .features
        .iter()
        .flat_map(|feature| feature.commands)
        .chain(source.app_commands);
    for command in commands {
        if command.needs_host && !connected {
            continue;
        }
        items.push(Item {
            id: format!("command:{}", command.id),
            group: match command.heading {
                Heading::Actions => Group::Actions,
                Heading::Settings => Group::Settings,
            },
            label: (command.label)(),
            keywords: command.keywords.iter().map(|k| (*k).to_owned()).collect(),
            icon: Some(command.icon),
            shortcut: command.shortcut.map(str::to_owned),
            status: None,
            act: Act::Run(command.run.clone()),
        });
    }
    items
}

fn rank(text: &str, query: &str) -> f64 {
    let is_separator = |c: char| c.is_whitespace() || "-_/.:".contains(c);
    if text.starts_with(query) {
        0.9
    } else if text.split(is_separator).any(|word| word.starts_with(query)) {
        0.8
    } else if text.contains(query) {
        0.5
    } else {
        0.0
    }
}

/// How well `item` matches `query`, from 0 (hidden) to 1: its label or a
/// keyword starting with it first, then a word in it starting with it, then
/// containing it. Pages win a tie, so "vol" opens Volumes.
pub fn score(item: &Item, query: &str) -> f64 {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return 1.0;
    }
    let best = item
        .keywords
        .iter()
        .map(|keyword| rank(&keyword.to_lowercase(), &query) * 0.95)
        .fold(rank(&item.label.to_lowercase(), &query), f64::max);
    if best == 0.0 {
        0.0
    } else if item.id.starts_with("nav:") {
        (best + 0.02).min(1.0)
    } else {
        best
    }
}

/// At most this many recently used items lead the palette while it's empty.
pub const RECENT_LIMIT: usize = 5;

/// The ids of recently run items, most recent first, with `id` moved to
/// the front.
pub fn remember(recent: &[String], id: &str) -> Vec<String> {
    std::iter::once(id.to_owned())
        .chain(recent.iter().filter(|other| *other != id).cloned())
        .take(RECENT_LIMIT)
        .collect()
}

/// The recently used items that are still listed, most recent first.
pub fn recent_items<'a>(items: &'a [Item], recent: &[String]) -> Vec<&'a Item> {
    recent
        .iter()
        .filter_map(|id| items.iter().find(|item| &item.id == id))
        .collect()
}

/// The rows under one heading.
pub struct Section<'a> {
    pub group: Group,
    pub items: Vec<&'a Item>,
}

fn by_group<'a>(items: impl IntoIterator<Item = &'a Item>) -> Vec<Section<'a>> {
    let mut sections: Vec<Section<'a>> = vec![];
    for item in items {
        match sections.iter_mut().find(|s| s.group == item.group) {
            Some(section) => section.items.push(item),
            None => sections.push(Section {
                group: item.group,
                items: vec![item],
            }),
        }
    }
    sections
}

/// Everything, grouped, with the recently used first.
pub fn browsing<'a>(items: &'a [Item], recent: &[String]) -> Vec<Section<'a>> {
    let last = recent_items(items, recent);
    let mut sections = vec![];
    if !last.is_empty() {
        sections.push(Section {
            group: Group::Recent,
            items: last,
        });
    }
    sections.extend(by_group(items));
    sections
}

/// The items matching `query`, best first, grouped in the order their best
/// appears.
pub fn matching<'a>(items: &'a [Item], query: &str) -> Vec<Section<'a>> {
    let mut ranked: Vec<(f64, &Item)> = items
        .iter()
        .map(|item| (score(item, query), item))
        .filter(|(score, _)| *score > 0.0)
        .collect();
    // A stable sort: equal scores keep the list's order.
    ranked.sort_by(|a, b| b.0.total_cmp(&a.0));
    by_group(ranked.into_iter().map(|(_, item)| item))
}

/// What the palette shows for `query`: what matches it, or everything.
pub fn sections<'a>(items: &'a [Item], query: &str, recent: &[String]) -> Vec<Section<'a>> {
    if query.trim().is_empty() {
        browsing(items, recent)
    } else {
        matching(items, query)
    }
}

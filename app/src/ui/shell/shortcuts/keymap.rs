//! The keymap: every shortcut that works now, as one table. The shell
//! builds it from what shows (the sidebar's entries, the page) and resolves
//! a key press against it; the help sheet and the palette read the same
//! table, so what is listed is what works.

use super::keys::{self, Press, Target};
use crate::{
    t,
    ui::shell::{
        registry::{self, nav, Label, Run},
        router::Route,
    },
};

/// The help sheet's heading a shortcut is listed under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    General,
    Navigation,
    Hosts,
    ThisPage,
}

impl Group {
    pub fn label(self) -> String {
        match self {
            Self::General => t!("app.shortcuts.general"),
            Self::Navigation => t!("app.shortcuts.navigation"),
            Self::Hosts => t!("app.shortcuts.hosts"),
            Self::ThisPage => t!("ui.shortcuts.thisPage"),
        }
    }
}

/// What a shortcut does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Opens the command palette, or closes it.
    TogglePalette,
    /// The next (1) or previous (-1) Host, round at the ends.
    StepHost(isize),
    /// Reads the Hosts and the selected Host's lists again.
    Refresh,
    Run(Run),
}

/// What a shortcut is listed as.
#[derive(Debug, Clone)]
pub enum Says {
    Plain(Label),
    /// "Go to" and the page's name.
    GoTo(Label),
}

impl Says {
    pub fn text(&self) -> String {
        match self {
            Self::Plain(label) => label(),
            Self::GoTo(page) => t!("app.shortcuts.goTo", page = page()),
        }
    }
}

/// One line of the keymap.
#[derive(Debug, Clone)]
pub struct Binding {
    /// As in `Mod+K`, `Alt+ArrowUp`, `n`, `?`.
    pub keys: String,
    pub says: Says,
    pub group: Group,
    pub command: Command,
}

impl Binding {
    fn new(keys: &str, label: Label, group: Group, command: Command) -> Self {
        Self {
            keys: keys.to_owned(),
            says: Says::Plain(label),
            group,
            command,
        }
    }
}

pub const PALETTE: &str = "Mod+K";
pub const HELP: &str = "?";
pub const SETTINGS: &str = "Mod+,";

/// The shortcuts that work on the page of `route`, given the sidebar's
/// `entries` and whether a Host is selected. A later line wins over an
/// earlier one for the same keys, so a page's shortcut wins over the app's.
pub fn keymap(entries: &[nav::Entry], route: &Route, host_selected: bool) -> Vec<Binding> {
    let mut map: Vec<Binding> = nav::shortcuts(entries)
        .into_iter()
        .map(|(entry, keys)| Binding {
            keys,
            says: Says::GoTo(entry.label),
            group: Group::Navigation,
            command: Command::Run(Run::Open(entry.route.clone(), None)),
        })
        .collect();
    map.push(Binding::new(
        SETTINGS,
        || t!("app.shortcuts.settings"),
        Group::General,
        Command::Run(Run::Open(Route::Settings, None)),
    ));
    map.push(Binding::new(
        "Alt+ArrowUp",
        || t!("app.shortcuts.previousHost"),
        Group::Hosts,
        Command::StepHost(-1),
    ));
    map.push(Binding::new(
        "Alt+ArrowDown",
        || t!("app.shortcuts.nextHost"),
        Group::Hosts,
        Command::StepHost(1),
    ));
    if host_selected {
        map.push(Binding::new(
            "Mod+R",
            || t!("app.shortcuts.refresh"),
            Group::Hosts,
            Command::Refresh,
        ));
    }
    map.push(Binding::new(
        PALETTE,
        || t!("app.palette.open"),
        Group::General,
        Command::TogglePalette,
    ));
    let feature = registry::feature(route.feature());
    for shortcut in feature.shortcuts.iter().filter(|s| (s.on)(route)) {
        map.push(Binding::new(
            shortcut.keys,
            shortcut.label,
            Group::ThisPage,
            Command::Run(shortcut.run.clone()),
        ));
    }
    map.push(Binding::new(
        HELP,
        || t!("app.shortcuts.help"),
        Group::General,
        Command::Run(Run::ShortcutsHelp),
    ));
    map
}

/// The shortcut `press` is for at `target`: the latest line that matches.
pub fn resolve<'a>(
    map: &'a [Binding],
    press: &Press,
    mac: bool,
    target: Target,
) -> Option<&'a Binding> {
    map.iter().rev().find(|binding| {
        let combo = keys::parse(&binding.keys);
        keys::matches(&combo, press, mac) && keys::fires_at(&combo, target)
    })
}

/// The keymap as the help sheet lists it: each shortcut once, under its
/// heading, the headings in the order their first shortcut comes.
pub fn grouped(map: &[Binding]) -> Vec<(Group, Vec<&Binding>)> {
    let mut groups: Vec<(Group, Vec<&Binding>)> = vec![];
    for binding in map {
        let listed = groups
            .iter()
            .flat_map(|(_, list)| list)
            .any(|other| other.keys == binding.keys && other.says.text() == binding.says.text());
        if listed {
            continue;
        }
        match groups.iter_mut().find(|(group, _)| *group == binding.group) {
            Some((_, list)) => list.push(binding),
            None => groups.push((binding.group, vec![binding])),
        }
    }
    groups
}

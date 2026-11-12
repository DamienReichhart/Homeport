//! The shell: what `src/app` and `src/core` were in React. It is what the
//! window shows once the app has started: the sidebar, the bars over the
//! page, and the page the route names, behind its gate.
//!
//! - [`router`]: every page as a [`Route`], and where the window has been;
//! - [`registry`]: every feature, as data: its sidebar entry, what it
//!   requires, its palette commands and shortcuts;
//! - [`selection`]: the one Host or Cluster the pages show, remembered;
//! - [`gates`]: what stands before a page until the selection can show it;
//! - [`pickers`]: the namespace or project a Host's or Cluster's pages show;
//! - [`palette`] and [`shortcuts`]: Mod+K, `?` and the keymap;
//! - [`page`]: what a feature's page is given and how the shell hosts it.
//!
//! The shell knows no feature: [`Shell`] is generic over [`Pages`], which
//! `ui::features` implements.

pub mod gates;
pub mod layout;
pub mod page;
pub mod palette;
pub mod pickers;
pub mod registry;
pub mod router;
pub mod selection;
pub mod shortcuts;
pub mod sidebar;
pub mod status;
#[cfg(test)]
mod tests;
mod update;

use std::sync::Arc;

use iced::{widget::container, Length, Subscription};

use homeport_settings::Settings;

use self::{
    gates::Subject,
    page::{Effects, Outcomes, Pages, Update, World},
    palette::items::{self, Item},
    registry::{nav, Command, Run, Scope, FEATURES},
    router::{History, Request, Route},
    selection::{remember::Saved, Selection},
    shortcuts::{
        help,
        keymap::{self, Binding},
        keys::{self, Press, Target},
    },
};
use crate::{
    app::AppState,
    data::{self, action::Finished, Action, Data, Key},
    i18n,
    ui::{
        theme::{self, metrics::space, Element},
        widgets::{
            basic::scroll_area,
            feedback::{
                toast::{self, Said},
                Surfaced,
            },
            focus::key_area,
            line,
            overlay::{self, menu},
            stack,
        },
    },
};

pub use self::{
    page::{stub, Context, Outcome},
    router::Location,
};

/// The commands of the shell itself, after the features' in the palette.
static APP_COMMANDS: &[Command] = &[Command::new(
    "app:shortcuts",
    || crate::t!("app.shortcuts.help"),
    theme::Icon::Keyboard,
    &["keys", "keyboard", "help"],
    Run::ShortcutsHelp,
)
.shortcut(keymap::HELP)];

/// The page that shows, and what it holds.
struct Open<P> {
    /// Its own among the pages opened in this session: a message carries
    /// it, so one that comes back late doesn't reach another page.
    id: u64,
    /// The navigation it answers and what its gate let through: when
    /// either changes, the page is opened anew.
    key: (u64, Subject),
    page: P,
    /// What it asked the data layer to keep fresh.
    watched: Vec<Key>,
}

/// What the shell asks of the window around it.
#[derive(Debug, Clone)]
pub enum Event {
    /// A failure to say in a toast, under what was being done.
    Failed(Option<Said>, Surfaced),
    /// Something to say in a toast: what an action did.
    Said(toast::Kind, Said),
    /// The Details of a failure, asked for.
    Details(Surfaced),
    /// The theme was chosen in the palette.
    Theme(homeport_settings::Theme),
    /// Text for the clipboard.
    Copy(String),
    /// A link for the system browser.
    OpenUrl(String),
    /// Look for a new version now.
    CheckForUpdates,
}

#[derive(Debug, Clone)]
pub enum Message<M> {
    /// A message of the page opened under this number; dropped once
    /// another page shows.
    Page(u64, M),
    Go(Request),
    Back,
    Forward,
    Select(Selection),
    /// A key was pressed; `captured` when a field or a dialog used it.
    Key {
        press: Press,
        captured: bool,
    },
    Palette(palette::Message),
    CloseHelp,
    Sidebar(sidebar::Message),
    Gate(gates::view::Action),
    /// What was found on this machine was added, or couldn't be.
    Added(Finished<Option<Selection>>),
    /// A reconnection asked for at a gate ended.
    Reconnected(Finished<()>),
    /// What was to be remembered is saved, or couldn't be.
    Saved(Saved),
    DismissUpdate,
    /// What the updater found: a newer version, or none.
    UpdateFound(Option<Update>),
    /// A read of the data layer ended, or it is time for one.
    Data(data::Message),
    /// Something the shell reads changed outside it (an event of the
    /// backend put a status in the cache).
    Changed,
}

/// The shell's state.
pub struct Shell<P: Pages> {
    state: Arc<AppState>,
    world: World,
    /// What the user chose, or what was remembered; the selection is what
    /// it resolves to among the Hosts and Clusters that exist.
    chosen: Option<Selection>,
    history: History,
    page: Option<Open<P>>,
    /// The navigation whose action the page was already asked to start.
    acted: Option<u64>,
    /// What the shell itself asked the data layer to keep fresh.
    held: Vec<Key>,
    /// The sidebar's entries for the selection.
    entries: Vec<nav::Entry>,
    effects: Effects,
    /// Where the actions pages started tell what they did.
    outcomes: Outcomes,
    palette: palette::State,
    help: bool,
    switcher: menu::State,
    pickers: pickers::State,
    /// The version whose notice was dismissed.
    dismissed: Option<String>,
    reconnect: Action,
    /// A Host or a Cluster found on this machine is being added.
    adding: Action<Option<Selection>>,
    /// How many pages were opened: the next one's number.
    opened: u64,
    /// A save of what is remembered is on its way; the others wait.
    saving: bool,
    to_save: std::collections::VecDeque<selection::remember::Remember>,
    events: Vec<Event>,
    mac: bool,
}

impl<P: Pages> Shell<P> {
    /// The shell of a started app, opening on what `settings` remember.
    pub fn new(state: Arc<AppState>, settings: Settings) -> Self {
        Self {
            state,
            chosen: selection::remembered(&settings),
            world: World::new(settings),
            history: History::default(),
            page: None,
            acted: None,
            held: vec![],
            entries: vec![],
            effects: Effects::default(),
            outcomes: Outcomes::default(),
            palette: palette::State::default(),
            help: false,
            switcher: menu::State::default(),
            pickers: pickers::State::default(),
            dismissed: None,
            reconnect: Action::default(),
            adding: Action::default(),
            opened: 0,
            saving: false,
            to_save: std::collections::VecDeque::new(),
            events: vec![],
            mac: keys::is_mac(),
        }
    }

    /// What the shell asked of the window since the last call.
    pub fn take_events(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }

    pub fn location(&self) -> &Location {
        self.history.current()
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    /// The sidebar's entries for what is selected.
    fn visible_entries(&self) -> Vec<nav::Entry> {
        let host = self.world.host();
        // With no Host selected, the Docker pages show, each explaining
        // how to add one.
        let supported = |capability| match (host, capability) {
            (None, registry::Capability::Docker) => true,
            _ => status::engine::supports(host, capability),
        };
        nav::visible(
            &nav::entries_of(FEATURES),
            self.world.selection.as_ref().map(selection::kind),
            supported,
            self.world.cluster().map(|view| view.cluster.kind),
        )
    }

    /// The shortcuts that work now.
    pub fn keymap(&self) -> Vec<Binding> {
        keymap::keymap(
            &self.entries,
            &self.location().route,
            self.world.host_id().is_some(),
        )
    }

    /// The palette's rows for what is selected.
    fn palette_items(&self) -> Vec<Item> {
        let nav_keys = nav::shortcuts(&self.entries)
            .into_iter()
            .map(|(entry, keys)| (entry.route.path(), keys))
            .collect();
        items::items(&items::Source {
            features: items::shown(FEATURES, &self.entries),
            app_commands: APP_COMMANDS,
            hosts: self.world.host_list(),
            selected: self.world.host(),
            nav_keys,
        })
    }

    /// The newer version to tell of, unless its notice was dismissed.
    fn update_notice(&self) -> Option<&Update> {
        self.world
            .update
            .as_ref()
            .filter(|update| self.dismissed.as_deref() != Some(update.version.as_str()))
    }

    /// The sidebar, the bars and the page, with the palette or the
    /// shortcuts' sheet over them, and the keys of the whole window.
    pub fn view<'a>(&'a self, data: &'a Data) -> Element<'a, Message<P::Message>> {
        let route = &self.location().route;
        let sidebar = sidebar::view(&sidebar::Sidebar {
            world: &self.world,
            entries: &self.entries,
            route,
            switcher: &self.switcher,
            pickers: &self.pickers,
            mac: self.mac,
        })
        .map(Message::Sidebar);
        let verdict = gates::decide(route, &self.world);
        let busy = self.reconnect.is_pending() || self.adding.is_pending();
        let mut page_dialog = None;
        let content = match (gates::view::gate(&verdict, &self.world, busy), &self.page) {
            (Some(gate), _) => gate.map(Message::Gate),
            (None, Some(open)) => {
                let ctx = Context {
                    state: &self.state,
                    data,
                    world: &self.world,
                    location: self.history.current(),
                    subject: &open.key.1,
                    direction: i18n::direction(),
                    pickers: &self.pickers,
                    effects: &self.effects,
                    outcomes: &self.outcomes,
                };
                let id = open.id;
                page_dialog = open
                    .page
                    .dialog(&ctx)
                    .map(|modal| modal.map(move |m| Message::Page(id, m)));
                open.page.view(&ctx).map(move |m| Message::Page(id, m))
            }
            // Between a gate that opens and the page it opens.
            (None, None) => crate::ui::widgets::feedback::spinner(),
        };
        let mut main: Vec<Element<'a, Message<P::Message>>> = vec![];
        if let Some(update) = self.update_notice() {
            main.push(layout::banners::update(update, Message::DismissUpdate));
        }
        main.extend(
            self.world
                .capabilities
                .as_deref()
                .and_then(layout::banners::keychain),
        );
        main.push(
            scroll_area(container(content).padding(space::S6).width(Length::Fill))
                .height(Length::Fill)
                .into(),
        );
        let body = container(
            line(vec![sidebar, stack(main).height(Length::Fill).into()])
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::page);
        let dialog = if self.help {
            Some(help::sheet(&self.keymap(), self.mac, Message::CloseHelp))
        } else {
            palette::view::palette(&self.palette, &self.palette_items(), self.mac)
                .map(|palette| palette.map(Message::Palette))
        };
        // Only a key the keymap has is taken: Tab and the rest go on to the
        // window. Plain keys reach the keymap when nothing used them; those
        // with Mod are heard from a field or a dialog too.
        let (mac, map) = (self.mac, std::rc::Rc::new(self.keymap()));
        let pressed = |captured: bool| {
            let map = map.clone();
            move |key: &iced::keyboard::Key, modifiers| {
                let press = Press::from_iced(key, modifiers)?;
                let target = if captured { Target::Text } else { Target::Page };
                keymap::resolve(&map, &press, mac, target)?;
                Some(Message::Key { press, captured })
            }
        };
        // The page's dialog over the sidebar and the page, and the palette
        // or the shortcuts' sheet over that: the same layers whether there
        // is one or not, so what is under keeps its state and its focus.
        let layers = overlay::over(overlay::over(body, page_dialog), dialog);
        key_area(layers, pressed(false))
            .always()
            .on_captured(pressed(true))
            .into()
    }

    /// The mouse's back and forward buttons, and what the page that shows
    /// follows: a stream runs while its page returns it.
    pub fn subscription(&self, data: &Data) -> Subscription<Message<P::Message>> {
        use iced::mouse::{Button, Event};
        let buttons = iced::event::listen_with(|event, _status, _window| match event {
            iced::Event::Mouse(Event::ButtonPressed(Button::Back)) => Some(Message::Back),
            iced::Event::Mouse(Event::ButtonPressed(Button::Forward)) => Some(Message::Forward),
            _ => None,
        });
        // Behind its gate a page reads nothing, so it follows nothing.
        let gated = !matches!(
            gates::decide(&self.location().route, &self.world),
            gates::Verdict::Open(_)
        );
        let Some(open) = self.page.as_ref().filter(|_| !gated) else {
            return buttons;
        };
        let ctx = Context {
            state: &self.state,
            data,
            world: &self.world,
            location: self.history.current(),
            subject: &open.key.1,
            direction: i18n::direction(),
            pickers: &self.pickers,
            effects: &self.effects,
            outcomes: &self.outcomes,
        };
        // The page's number is part of what tells its streams apart: a
        // page opened anew starts its own.
        let page = open
            .page
            .subscription(&ctx)
            .with(open.id)
            .map(|(id, message)| Message::Page(id, message));
        Subscription::batch([buttons, page])
    }

    /// Whether the page of `scope` may show for what is selected.
    fn allows(&self, scope: Scope) -> bool {
        !selection::sends_to_overview(scope, self.world.selection.as_ref())
    }

    fn at(&self, route: &Route) -> bool {
        &self.location().route == route
    }
}

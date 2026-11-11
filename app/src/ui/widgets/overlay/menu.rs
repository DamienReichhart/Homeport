//! `DropdownMenu`: a menu under its trigger, with submenus, driven by the
//! keyboard as Radix's: the arrows go through the items (round at the
//! ends), Home and End to the first and last, typing to the item that
//! starts with it, Enter or Space choose, the arrow towards the end opens a
//! submenu and the one towards the start closes it, Escape closes.
//!
//! The page keeps a [`State`] and feeds it the [`Event`]s:
//!
//! ```ignore
//! menu::dropdown(
//!     &self.menu,
//!     button::icon(Icon::Ellipsis).on_press(Message::Menu(menu::Event::Toggle)),
//!     vec![
//!         menu::Item::action(t!("containers.start"), Message::Start),
//!         menu::Item::Separator,
//!         menu::Item::action(t!("common.remove"), Message::Remove).destructive(),
//!     ],
//!     Message::Menu,
//! )
//! ```
//!
//! Choosing an item sends `Event::Close`, then the item's own message.

use iced::{
    keyboard::{key::Named, Key},
    widget::{container, rule, space as gap, text},
    Length,
};

use super::{anchored, Align, Placement, Side};
use crate::{
    i18n,
    ui::{
        theme::{
            self, icon,
            metrics::{space, text as size},
            Element, Icon,
        },
        widgets::{
            basic::fit,
            focus::{key_area, pressable, roving, Axis},
            line, start,
        },
    },
};

/// `min-w-32`: a menu is never narrower.
const MIN_WIDTH: f32 = 128.0;

/// Which menus are open: nothing, the menu, or the menu and the submenus
/// that lead to the one in front (the index of each among its menu's items).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    open: Option<Vec<usize>>,
}

/// What the user did to a menu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// The trigger was pressed.
    Toggle,
    /// An item was chosen, Escape was pressed or something else was.
    Close,
    /// The item `index` of the menu at `depth` opens its submenu.
    OpenSub { depth: usize, index: usize },
    /// The submenu opened from the menu at `depth` closes.
    CloseSub { depth: usize },
}

impl State {
    pub fn is_open(&self) -> bool {
        self.open.is_some()
    }

    /// The submenu open from the menu at `depth`, if any.
    fn sub(&self, depth: usize) -> Option<usize> {
        self.open.as_ref()?.get(depth).copied()
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::Toggle => {
                self.open = match self.open {
                    Some(_) => None,
                    None => Some(vec![]),
                };
            }
            Event::Close => self.open = None,
            Event::OpenSub { depth, index } => {
                if let Some(path) = &mut self.open {
                    path.truncate(depth);
                    path.push(index);
                }
            }
            Event::CloseSub { depth } => {
                if let Some(path) = &mut self.open {
                    path.truncate(depth);
                }
            }
        }
    }
}

/// One line of a menu.
pub enum Item<Message> {
    Action {
        label: String,
        glyph: Option<Icon>,
        shortcut: Option<String>,
        /// `None`: shown, but it can't be chosen.
        message: Option<Message>,
        destructive: bool,
        /// A check at the end: `DropdownMenuCheckboxItem`.
        checked: Option<bool>,
    },
    /// `DropdownMenuLabel`: names the items under it.
    Label(String),
    Separator,
    Sub {
        label: String,
        glyph: Option<Icon>,
        items: Vec<Item<Message>>,
    },
}

impl<Message> Item<Message> {
    pub fn action(label: impl Into<String>, message: Message) -> Self {
        Self::Action {
            label: label.into(),
            glyph: None,
            shortcut: None,
            message: Some(message),
            destructive: false,
            checked: None,
        }
    }

    pub fn sub(label: impl Into<String>, items: Vec<Self>) -> Self {
        Self::Sub {
            label: label.into(),
            glyph: None,
            items,
        }
    }

    #[must_use]
    pub fn glyph(mut self, icon: Icon) -> Self {
        if let Self::Action { glyph, .. } | Self::Sub { glyph, .. } = &mut self {
            *glyph = Some(icon);
        }
        self
    }

    /// The keys that do the same, shown at the end of the line.
    #[must_use]
    pub fn shortcut(mut self, keys: impl Into<String>) -> Self {
        if let Self::Action { shortcut, .. } = &mut self {
            *shortcut = Some(keys.into());
        }
        self
    }

    #[must_use]
    pub fn destructive(mut self) -> Self {
        if let Self::Action { destructive, .. } = &mut self {
            *destructive = true;
        }
        self
    }

    #[must_use]
    pub fn checked(mut self, on: bool) -> Self {
        if let Self::Action { checked, .. } = &mut self {
            *checked = Some(on);
        }
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        if let (true, Self::Action { message, .. }) = (disabled, &mut self) {
            *message = None;
        }
        self
    }

    /// What typing looks for; `None` for what takes no focus.
    fn label(&self) -> Option<&str> {
        match self {
            Self::Action {
                label,
                message: Some(_),
                ..
            }
            | Self::Sub { label, .. } => Some(label),
            _ => None,
        }
    }
}

/// `trigger` with the menu of `items` under it while `state` says so. The
/// trigger sends `on_event(Event::Toggle)` itself.
pub fn dropdown<'a, Message: Clone + 'a>(
    state: &State,
    trigger: impl Into<Element<'a, Message>>,
    items: Vec<Item<Message>>,
    on_event: impl Fn(Event) -> Message + Clone + 'a,
) -> Element<'a, Message> {
    let direction = i18n::direction();
    let menu = state
        .is_open()
        .then(|| panel(state, items, 0, on_event.clone()));
    anchored(
        trigger,
        menu,
        Placement::new(Side::Bottom, Align::start(direction)),
    )
    .on_dismiss(on_event(Event::Close))
    .into()
}

fn icon_of<'a, Message: 'a>(glyph: Icon, destructive: bool) -> Element<'a, Message> {
    let glyph = icon(glyph);
    if destructive {
        glyph.style(theme::svg::destructive).into()
    } else {
        glyph.into()
    }
}

fn panel<'a, Message: Clone + 'a>(
    state: &State,
    items: Vec<Item<Message>>,
    depth: usize,
    on_event: impl Fn(Event) -> Message + Clone + 'a,
) -> Element<'a, Message> {
    let rtl = i18n::direction().is_rtl();
    let (towards_end, towards_start) = if rtl {
        (Named::ArrowLeft, Named::ArrowRight)
    } else {
        (Named::ArrowRight, Named::ArrowLeft)
    };
    let labels = items
        .iter()
        .filter_map(|item| item.label().map(str::to_owned))
        .collect();
    let label_of = |label: String| {
        text(label)
            .size(size::SM)
            .width(Length::Fill)
            .align_x(start())
    };
    let mut lines: Vec<Element<'a, Message>> = vec![];
    for (index, item) in items.into_iter().enumerate() {
        let line: Element<'a, Message> = match item {
            Item::Separator => container(rule::horizontal(1))
                .padding([space::S1, 0.0])
                .into(),
            Item::Label(label) => container(
                text(label)
                    .size(size::XS)
                    .font(theme::font::SANS_MEDIUM)
                    .style(theme::text::muted),
            )
            .padding([space::S1, space::S1_5])
            .width(Length::Fill)
            .align_x(start())
            .into(),
            Item::Action {
                label,
                glyph,
                shortcut,
                message,
                destructive,
                checked,
            } => {
                let mut parts: Vec<Element<'a, Message>> = vec![];
                parts.extend(glyph.map(|glyph| icon_of(glyph, destructive)));
                parts.push(label_of(label).into());
                parts.extend(
                    shortcut.map(|keys| text(keys).size(size::XS).style(theme::text::muted).into()),
                );
                if let Some(on) = checked {
                    parts.push(if on {
                        icon(Icon::Check).into()
                    } else {
                        gap().width(theme::metrics::icon::MD).into()
                    });
                }
                let row = pressable(line(parts).spacing(space::S1_5))
                    .width(Length::Fill)
                    .padding([space::S1, space::S1_5])
                    .before(on_event(Event::Close))
                    .on_press_maybe(message);
                if destructive {
                    row.style(theme::button::part::destructive_item).into()
                } else {
                    row.style(theme::button::part::item(false)).into()
                }
            }
            Item::Sub {
                label,
                glyph,
                items,
            } => {
                let open = state.sub(depth) == Some(index);
                let chevron = if rtl {
                    Icon::ChevronLeft
                } else {
                    Icon::ChevronRight
                };
                let mut parts: Vec<Element<'a, Message>> = vec![];
                parts.extend(glyph.map(|glyph| icon_of(glyph, false)));
                parts.push(label_of(label).into());
                parts.push(icon(chevron).style(theme::svg::muted).into());
                let opens = on_event(Event::OpenSub { depth, index });
                let by_key = opens.clone();
                let row = pressable(line(parts).spacing(space::S1_5))
                    .width(Length::Fill)
                    .padding([space::S1, space::S1_5])
                    .on_press(opens)
                    .on_key(move |key, _| (*key == Key::Named(towards_end)).then(|| by_key.clone()))
                    .style(theme::button::part::item(open));
                let closes = on_event(Event::CloseSub { depth });
                let back = closes.clone();
                let submenu = open.then(|| {
                    key_area(
                        panel(state, items, depth + 1, on_event.clone()),
                        move |key, _| (*key == Key::Named(towards_start)).then(|| back.clone()),
                    )
                    .into()
                });
                anchored(
                    row,
                    submenu,
                    Placement::new(Side::end(i18n::direction()), Align::Near).gap(0.0),
                )
                .on_dismiss(closes)
                .into()
            }
        };
        lines.push(line);
    }
    container(roving(Axis::Vertical, fit(lines).min_width(MIN_WIDTH)).labels(labels))
        .padding(space::S1)
        .style(theme::container::popover)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_state_follows_the_menus_that_open_and_close() {
        let mut state = State::default();
        assert!(!state.is_open());
        state.update(Event::Toggle);
        assert!(state.is_open());
        state.update(Event::OpenSub { depth: 0, index: 2 });
        state.update(Event::OpenSub { depth: 1, index: 0 });
        assert_eq!((state.sub(0), state.sub(1)), (Some(2), Some(0)));
        // Another item of the first menu replaces the whole branch.
        state.update(Event::OpenSub { depth: 0, index: 3 });
        assert_eq!((state.sub(0), state.sub(1)), (Some(3), None));
        state.update(Event::CloseSub { depth: 0 });
        assert!(state.is_open() && state.sub(0).is_none());
        state.update(Event::Close);
        assert!(!state.is_open());
        // A submenu can't open under a closed menu.
        state.update(Event::OpenSub { depth: 0, index: 1 });
        assert!(!state.is_open());
    }
}

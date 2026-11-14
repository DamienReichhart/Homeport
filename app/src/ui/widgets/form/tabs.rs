//! `Tabs`, `ToggleGroup` and the segmented control: a row of buttons of
//! which one (or, for toggles, some) is chosen.
//!
//! The keyboard, as Radix's: Left and Right go to the button on that side
//! (round at the ends), Home and End to the first and last, counted as the
//! language reads. A tab is chosen as focus reaches it; a toggle waits for Enter
//! or Space.
//!
//! These draw the row only: the page shows what the chosen tab stands for.

use iced::{
    border::Radius,
    widget::{button as iced_button, container, text, Column, Row, Space},
    Length,
};

use crate::{
    i18n,
    ui::{
        theme::{
            self, font, icon,
            metrics::{control, radius, space, text as size},
            Element, Icon, Theme,
        },
        widgets::{
            basic::flow,
            focus::{pressable, roving, Axis},
            line,
        },
    },
};

/// The React `TabsList`'s `variant`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsVariant {
    /// Chips on a quiet band.
    #[default]
    Default,
    /// Text with a line under the chosen tab.
    Line,
}

/// `TabsList`: one tab per label, `selected` chosen. `on_select` gets the
/// index of the tab pressed or reached with the arrows.
pub fn tabs<'a, Message: Clone + 'a>(
    labels: Vec<String>,
    selected: usize,
    variant: TabsVariant,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message> {
    tab_list(labels, selected, variant, on_select, false)
}

/// [`tabs`] that go on the next line when the row is full (`h-auto
/// flex-wrap`), starting where the language reads: many sections in a
/// narrow dialog.
pub fn wrapping_tabs<'a, Message: Clone + 'a>(
    labels: Vec<String>,
    selected: usize,
    variant: TabsVariant,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message> {
    tab_list(labels, selected, variant, on_select, true)
}

fn tab_list<'a, Message: Clone + 'a>(
    labels: Vec<String>,
    selected: usize,
    variant: TabsVariant,
    on_select: impl Fn(usize) -> Message + 'a,
    wrap: bool,
) -> Element<'a, Message> {
    let tab = |(index, label): (usize, String)| -> Element<'a, Message> {
        let chosen = index == selected;
        let label = text(label).size(size::SM).font(font::medium());
        match variant {
            TabsVariant::Default => pressable(container(label).center_y(Length::Fill))
                .height(control::HEIGHT - 6.0)
                .padding([0.0, space::S2])
                .on_press(on_select(index))
                .style(theme::button::part::tab(chosen))
                .into(),
            TabsVariant::Line => {
                // `after:h-0.5`: the line under the chosen tab.
                let under = container(Space::new()).height(2).width(Length::Fill).style(
                    move |theme: &Theme| container::Style {
                        background: chosen.then_some(theme.tokens().foreground.into()),
                        ..container::Style::default()
                    },
                );
                let face = Column::new()
                    .push(
                        container(label)
                            .center_y(Length::Fill)
                            .padding([0.0, space::S1_5]),
                    )
                    .push(under)
                    .width(Length::Shrink);
                pressable(face)
                    .height(control::HEIGHT)
                    .padding(0)
                    .on_press(on_select(index))
                    .style(theme::button::part::line_tab(chosen))
                    .into()
            }
        }
    };
    let items = labels.into_iter().enumerate().map(tab).collect();
    let spacing = match variant {
        TabsVariant::Default => 0.0,
        TabsVariant::Line => space::S1,
    };
    let row: Element<'a, Message> = if wrap {
        flow(items).spacing(spacing).hugging().into()
    } else {
        line(items).spacing(spacing).into()
    };
    let list = roving(Axis::Horizontal, row).on_move(on_select);
    match variant {
        TabsVariant::Default => container(list)
            .padding(3)
            .style(theme::container::muted)
            .into(),
        TabsVariant::Line => list.into(),
    }
}

/// One toggle of a group: an icon, a label, or both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Toggle {
    pub glyph: Option<Icon>,
    pub label: Option<String>,
}

impl Toggle {
    pub fn text(label: impl Into<String>) -> Self {
        Self {
            glyph: None,
            label: Some(label.into()),
        }
    }

    pub fn icon(glyph: Icon) -> Self {
        Self {
            glyph: Some(glyph),
            label: None,
        }
    }
}

/// The corners of the toggle at `index` of `count`, drawn joined to its
/// neighbours: only the ends of the row are rounded.
fn corners(index: usize, count: usize) -> Radius {
    let rtl = i18n::direction().is_rtl();
    let (first, last) = (index == 0, index + 1 == count);
    let (left, right) = if rtl { (last, first) } else { (first, last) };
    let mut corners = Radius::default();
    if left {
        corners = corners.left(radius::LG);
    }
    if right {
        corners = corners.right(radius::LG);
    }
    corners
}

/// `ToggleGroup`: toggles joined in one row. `is_on` says which are on (one
/// for `type="single"`, any for `"multiple"`), `on_press` gets the index
/// of the one pressed, and `outlined` draws their borders.
pub fn toggle_group<'a, Message: Clone + 'a>(
    toggles: Vec<Toggle>,
    is_on: impl Fn(usize) -> bool,
    outlined: bool,
    on_press: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message> {
    let count = toggles.len();
    let items = toggles.into_iter().enumerate().map(|(index, toggle)| {
        let mut parts: Vec<Element<'a, Message>> = vec![];
        parts.extend(toggle.glyph.map(|glyph| icon(glyph).into()));
        parts.extend(
            toggle
                .label
                .map(|label| text(label).size(size::SM).font(font::medium()).into()),
        );
        let on = is_on(index);
        let style = theme::button::part::toggle(on, outlined);
        let corners = corners(index, count);
        pressable(container(line(parts).spacing(space::S1)).center(Length::Fill))
            .height(control::HEIGHT)
            .padding([0.0, space::S2])
            .on_press(on_press(index))
            .style(move |theme: &Theme, status: iced_button::Status| {
                let mut style = style(theme, status);
                style.border.radius = corners;
                style
            })
            .into()
    });
    // Joined: no gap, and each border laid over its neighbour's.
    let row = Row::from_vec(i18n::direction().ordered(items.collect())).spacing(if outlined {
        -1.0
    } else {
        0.0
    });
    roving(Axis::Horizontal, row).into()
}

/// The segmented control: an outlined toggle group of which exactly one is
/// on, as a view switch ("List", "Board").
pub fn segmented<'a, Message: Clone + 'a>(
    labels: Vec<String>,
    selected: usize,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message> {
    let toggles = labels.into_iter().map(Toggle::text).collect();
    toggle_group(toggles, |index| index == selected, true, on_select)
}

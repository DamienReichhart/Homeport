//! The sidebar (`Sidebar.tsx`): the mark, the switcher of what the pages
//! show with the pickers under it, the pages of what is selected, the
//! Swarms, and the links out of the app. It is written start to end, so it
//! sits on the right and reads from the right in Arabic.

pub mod switcher;

use iced::{
    widget::{container, rule, text},
    Length,
};

use super::{
    layout::logo, page::World, pickers, registry::nav::Entry, router::Route, selection::Selection,
    shortcuts::keys, status,
};
use crate::{
    i18n, t,
    ui::{
        theme::{
            self, font, icon,
            metrics::{space, text as size},
            Element, Icon,
        },
        widgets::{
            basic::{self, scroll_area},
            filling,
            focus::pressable,
            line,
            overlay::{self, hovered, menu, Align, Placement, Side},
            stack,
        },
    },
};

/// `w-56`.
pub const WIDTH: f32 = 224.0;

/// The documentation and the repository (`src/shared/lib/links.ts`).
pub const DOCS_URL: &str = "https://damienreichhart.github.io/Homeport/docs/";
pub const REPO_URL: &str = "https://github.com/DamienReichhart/Homeport";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Go(Route),
    Switcher(menu::Event),
    Select(Selection),
    Picker(pickers::Message),
    /// The Details of why the selected Host or Cluster can't be used.
    WhyUnusable,
    /// A link out of the app, for the system browser.
    External(&'static str),
}

/// What the sidebar draws from.
pub struct Sidebar<'a> {
    pub world: &'a World,
    /// The entries that show for the selection.
    pub entries: &'a [Entry],
    /// Where the window is.
    pub route: &'a Route,
    pub switcher: &'a menu::State,
    pub pickers: &'a pickers::State,
    pub mac: bool,
}

/// Whether the page of `route` is `entry`'s or under it (`NavLink`).
pub fn is_current(entry: &Route, route: &Route) -> bool {
    let (entry, path) = (entry.path(), route.path());
    path == entry || (entry != "/" && path.starts_with(&format!("{entry}/")))
}

fn link<'a>(
    glyph: Icon,
    label: String,
    current: bool,
    on_press: Message,
    after: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let glyph = if current {
        icon(glyph).style(theme::svg::sidebar_primary)
    } else {
        icon(glyph).style(theme::svg::muted)
    };
    let label = text(label).size(size::SM);
    let label = if current {
        label.font(font::medium())
    } else {
        label
    };
    let mut parts: Vec<Element<'a, Message>> = vec![glyph.into(), filling(label).into()];
    parts.extend(after);
    pressable(line(parts).spacing(space::S2_5).width(Length::Fill))
        .width(Length::Fill)
        .padding([space::S1_5, space::S3])
        .on_press(on_press)
        .style(theme::button::part::nav(current))
        .into()
}

/// `WithShortcut`: what the entry does and its keys, beside it.
fn with_keys<'a>(
    entry: Element<'a, Message>,
    label: String,
    keys: &[String],
) -> Element<'a, Message> {
    let tip = container(
        line(vec![text(label).size(size::XS).into(), basic::keys(keys)]).spacing(space::S2),
    )
    .padding([space::S1_5, space::S3])
    .style(theme::container::tooltip);
    let side = Side::end(i18n::direction());
    hovered(
        entry,
        tip,
        Placement::new(side, Align::Center),
        overlay::TOOLTIP_DELAY,
    )
    .into()
}

pub fn view<'a>(sidebar: &Sidebar<'a>) -> Element<'a, Message> {
    let Sidebar { world, route, .. } = *sidebar;
    let mut top: Vec<Element<'a, Message>> = vec![switcher::view(world, sidebar.switcher)];
    top.extend(
        pickers::view::under_switcher(world, sidebar.pickers)
            .into_iter()
            .map(|picker| picker.map(Message::Picker)),
    );
    let shortcuts = super::registry::nav::shortcuts(sidebar.entries);
    let mut pages: Vec<Element<'a, Message>> = vec![];
    for entry in sidebar.entries {
        let label = (entry.label)();
        let item = link(
            entry.icon,
            label.clone(),
            is_current(&entry.route, route),
            Message::Go(entry.route.clone()),
            None,
        );
        let keys = shortcuts
            .iter()
            .find(|(of, _)| of.feature == entry.feature)
            .map(|(_, spec)| keys::format(spec, sidebar.mac));
        pages.push(match keys {
            Some(keys) => with_keys(item, label, &keys),
            None => item,
        });
    }
    // The Swarms' section shows only once a Host belongs to one.
    if !world.swarm_list().is_empty() {
        pages.push(
            container(
                text(t!("swarm.nav"))
                    .size(size::XS)
                    .font(font::medium())
                    .style(theme::text::muted),
            )
            .padding(iced::Padding {
                top: space::S3,
                bottom: space::S1,
                left: space::S3,
                right: space::S3,
            })
            .into(),
        );
        for swarm in world.swarm_list() {
            let to = Route::Swarm {
                swarm_id: swarm.id.clone(),
                tab: None,
            };
            pages.push(link(
                Icon::Network,
                status::swarm::label(swarm, world.host_list()),
                route.swarm_id() == Some(swarm.id.as_str()),
                Message::Go(to),
                None,
            ));
        }
    }
    let external = |url, label: String, glyph| {
        let out = icon(Icon::ExternalLink)
            .width(theme::metrics::icon::SM)
            .height(theme::metrics::icon::SM)
            .style(theme::svg::muted);
        link(
            glyph,
            label,
            false,
            Message::External(url),
            Some(out.into()),
        )
    };
    let body = stack(vec![
        container(logo::logo())
            .padding(iced::Padding {
                top: space::S1,
                bottom: space::S3,
                left: space::S1_5,
                right: space::S1_5,
            })
            .into(),
        stack(top).spacing(space::S1_5).into(),
        // The pages scroll when the window is short; the links below stay.
        container(scroll_area(stack(pages).spacing(space::S0_5)))
            .padding([space::S3, 0.0])
            .height(Length::Fill)
            .into(),
        rule::horizontal(1).style(theme::rule::sidebar).into(),
        container(
            stack(vec![
                external(DOCS_URL, t!("app.nav.docs"), Icon::BookOpen),
                external(REPO_URL, t!("app.nav.github"), Icon::Github),
            ])
            .spacing(space::S0_5),
        )
        .padding(iced::Padding {
            top: space::S3,
            ..iced::Padding::ZERO
        })
        .into(),
    ]);
    line(vec![
        container(body)
            .width(WIDTH)
            .height(Length::Fill)
            .padding(space::S3)
            .style(theme::container::sidebar)
            .into(),
        rule::vertical(1).style(theme::rule::sidebar).into(),
    ])
    .height(Length::Fill)
    .align_y(iced::alignment::Vertical::Top)
    .into()
}

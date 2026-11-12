//! The sidebar's picker of what the pages show: a Host, or a Cluster once
//! there are some (`HostSwitcher.tsx`).

use iced::{
    widget::{container, text},
    Length,
};

use homeport_core::error::AppError;

use super::Message;
use crate::{
    t,
    ui::{
        shell::{page::World, router::Route, selection::Selection, status},
        theme::{
            self, icon,
            metrics::{control, space, text as size},
            Element, Icon,
        },
        widgets::{
            basic::button::{self, Variant},
            filling,
            focus::pressable,
            line,
            overlay::{
                menu::{self, Item},
                tooltip, Side,
            },
        },
    },
};

/// The selected Host or Cluster that can't be used, and why.
pub struct Unusable<'a> {
    pub name: &'a str,
    pub label: String,
    pub error: &'a AppError,
}

/// Why what is selected can't be used, when its status has a cause.
pub fn unusable(world: &World) -> Option<Unusable<'_>> {
    if let Some(host) = world.host() {
        return Some(Unusable {
            name: &host.host.name,
            label: status::host::describe(&host.status).label,
            error: status::host::error(&host.status)?,
        });
    }
    let cluster = world.cluster()?;
    Some(Unusable {
        name: &cluster.cluster.name,
        label: status::cluster::describe(&cluster.status, cluster.cluster.kind).label,
        error: cluster.status.error()?,
    })
}

/// The name of what is selected, as the switcher shows it.
pub fn selected_name(world: &World) -> Option<&str> {
    world
        .host()
        .map(|view| view.host.name.as_str())
        .or_else(|| world.cluster().map(|view| view.cluster.name.as_str()))
}

fn items(world: &World) -> Vec<Item<Message>> {
    let mut items = vec![];
    let hosts = world.host_list();
    let clusters = world.cluster_list();
    if !hosts.is_empty() {
        items.push(Item::Label(t!("hosts.switcher.hosts")));
    }
    for view in hosts {
        let selection = Selection::Host {
            id: view.host.id.clone(),
        };
        let chosen = world.selection.as_ref() == Some(&selection);
        items.push(
            Item::action(view.host.name.clone(), Message::Select(selection))
                .status(status::host::describe(&view.status).tone)
                .checked(chosen),
        );
    }
    if !clusters.is_empty() {
        items.push(Item::Label(t!("hosts.switcher.clusters")));
    }
    for view in clusters {
        let selection = Selection::Cluster {
            id: view.cluster.id.clone(),
        };
        let chosen = world.selection.as_ref() == Some(&selection);
        let tone = status::cluster::describe(&view.status, view.cluster.kind).tone;
        items.push(
            Item::action(view.cluster.name.clone(), Message::Select(selection))
                .status(tone)
                .checked(chosen),
        );
    }
    if !hosts.is_empty() || !clusters.is_empty() {
        items.push(Item::Separator);
    }
    let go = |label: String, glyph, route| Item::action(label, Message::Go(route)).glyph(glyph);
    items.push(go(t!("hosts.switcher.add"), Icon::Plus, Route::AddHost));
    items.push(go(
        t!("hosts.switcher.manage"),
        Icon::ServerCog,
        Route::Hosts,
    ));
    items.push(Item::Separator);
    items.push(go(
        t!("hosts.switcher.addCluster"),
        Icon::Plus,
        Route::AddCluster,
    ));
    if !clusters.is_empty() {
        items.push(go(
            t!("hosts.switcher.manageClusters"),
            Icon::ShipWheel,
            Route::Clusters,
        ));
    }
    items
}

pub fn view<'a>(world: &'a World, state: &'a menu::State) -> Element<'a, Message> {
    let shown = match selected_name(world) {
        Some(name) => text(name.to_owned()).size(size::SM),
        None => text(t!("hosts.switcher.placeholder"))
            .size(size::SM)
            .style(theme::text::muted),
    };
    let face = line(vec![
        filling(shown).into(),
        icon(Icon::ChevronDown).style(theme::svg::muted).into(),
    ])
    .spacing(space::S1_5)
    .width(Length::Fill);
    let trigger = pressable(container(face).center_y(Length::Fill).width(Length::Fill))
        .width(Length::Fill)
        .height(control::HEIGHT)
        .padding([0.0, space::S2_5])
        .on_press(Message::Switcher(menu::Event::Toggle))
        .style(theme::button::part::field(false));
    let switcher = container(menu::dropdown(
        state,
        trigger,
        items(world),
        Message::Switcher,
    ))
    .width(Length::Fill);
    let mut parts: Vec<Element<'a, Message>> = vec![switcher.into()];
    if let Some(unusable) = unusable(world) {
        let why = button::icon(Icon::TriangleAlert)
            .variant(Variant::Ghost)
            .on_press(Message::WhyUnusable);
        let says = t!(
            "hosts.statusTitle",
            label = unusable.label,
            detail = crate::i18n::errors::message(unusable.error)
        );
        parts.push(tooltip(why, says, Side::Bottom).into());
    }
    line(parts).spacing(space::S1).width(Length::Fill).into()
}

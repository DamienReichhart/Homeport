//! The pickers as they are drawn.

use iced::Length;

use homeport_clusters::model::ClusterKind;

use super::{Message, State};
use crate::{
    t,
    ui::{
        shell::page::World,
        theme::{self, icon, metrics::space, Element, Icon},
        widgets::{
            form::{input, select},
            line,
            overlay::{tooltip, Side},
        },
    },
};

/// `w-48`: a Namespace picker in a page's header.
const WIDTH: f32 = 192.0;

fn with_icon<'a>(glyph: Icon, picker: Element<'a, Message>) -> Element<'a, Message> {
    line(vec![icon(glyph).style(theme::svg::muted).into(), picker])
        .spacing(space::S1_5)
        .width(Length::Fill)
        .into()
}

/// The pickers under the Host switcher: the containerd namespace or the
/// Incus project, while a connected Host of that Engine is selected.
pub fn under_switcher<'a>(world: &'a World, state: &'a State) -> Vec<Element<'a, Message>> {
    let mut pickers = vec![];
    if let Some((_, options)) = super::containerd(world) {
        let picker = select::select_owned(&state.containerd, options.labels, options.selected)
            .placeholder(t!("core.containerd.namespace"))
            .width(Length::Fill)
            .view()
            .map(Message::Containerd);
        pickers.push(with_icon(Icon::FolderTree, picker));
    }
    if let Some((_, options)) = super::incus(world) {
        let picker = select::select_owned(&state.incus, options.labels, options.selected)
            .placeholder(t!("core.incus.project"))
            .width(Length::Fill)
            .view()
            .map(Message::Incus);
        pickers.push(with_icon(Icon::FolderKanban, picker));
    }
    pickers
}

/// The Namespace picker of a Cluster page's header: a list with "All
/// namespaces" first, or a field to type the one the credentials may use
/// when they can't list them. `None` when there is nothing to pick.
pub fn namespace<'a>(world: &'a World, state: &'a State) -> Option<Element<'a, Message>> {
    if let Some((_, options)) = super::namespace(world) {
        return Some(
            select::select_owned(&state.namespace, options.labels, options.selected)
                .width(WIDTH)
                .view()
                .map(Message::Namespace),
        );
    }
    let cluster = world.cluster()?;
    let cant_list =
        cluster.cluster.kind == ClusterKind::Kubernetes && world.namespaces.error().is_some();
    cant_list.then(|| {
        let field = input(&t!("core.namespace.type"), &state.typed(world))
            .on_input(Message::Typed)
            .on_submit(Message::TypedSubmit)
            .width(WIDTH);
        tooltip(field, t!("core.namespace.cantList"), Side::Bottom).into()
    })
}

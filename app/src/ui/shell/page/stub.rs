//! The page of a feature that isn't ported yet: its title and nothing to
//! list. Each feature's folder re-exports this until its ticket writes its
//! own `State`, `Message`, `open`, `update`, `view` and `request`
//! (`docs/agents/iced-migration.md`, Replacing a stub).

use iced::{Subscription, Task};

use super::Context;
use crate::{
    t,
    ui::{
        shell::{layout::page_header, pickers, registry::FeatureId, router::Route},
        theme::{metrics::space, Element, Icon},
        widgets::{feedback::empty_state, overlay::Modal, stack},
    },
};

#[derive(Debug, Default)]
pub struct State;

#[derive(Debug, Clone)]
pub enum Message {
    /// The Namespace picker of a Cluster page's header.
    Picker(pickers::Message),
}

pub fn open(_ctx: &Context<'_>) -> (State, Task<Message>) {
    (State, Task::none())
}

pub fn update(_state: &mut State, message: Message, ctx: &Context<'_>) -> Task<Message> {
    match message {
        Message::Picker(message) => ctx.picked(message),
    }
    Task::none()
}

pub fn request(_state: &mut State, _action: &str, _ctx: &Context<'_>) -> Task<Message> {
    Task::none()
}

pub fn dialog<'a>(_state: &'a State, _ctx: &Context<'a>) -> Option<Modal<'a, Message>> {
    None
}

pub fn refresh(_state: &mut State, _ctx: &Context<'_>) -> Task<Message> {
    Task::none()
}

pub fn subscription(_state: &State, _ctx: &Context<'_>) -> Subscription<Message> {
    Subscription::none()
}

/// What the page is called: its sidebar entry, or what the switcher or
/// the sidebar's Swarms call it.
fn title(ctx: &Context<'_>) -> String {
    let feature = ctx.feature();
    match (&feature.nav, feature.id) {
        (Some(nav), _) => (nav.label)(),
        (None, FeatureId::Clusters) => t!("clusters.title"),
        (None, FeatureId::Swarm) => t!("swarm.nav"),
        (None, _) => t!("hosts.title"),
    }
}

/// The features whose list pages let the user pick a Namespace.
fn picks_namespace(route: &Route) -> bool {
    matches!(
        route,
        Route::Workloads { .. }
            | Route::Pods
            | Route::ApiResources { .. }
            | Route::Rbac { .. }
            | Route::Apply
            | Route::HelmReleases
            | Route::NomadJobs
    )
}

pub fn view<'a>(_state: &'a State, ctx: &Context<'a>) -> Element<'a, Message> {
    let title = title(ctx);
    let picker = picks_namespace(&ctx.location.route)
        .then(|| ctx.namespace_picker(Message::Picker))
        .flatten();
    let glyph = ctx
        .feature()
        .nav
        .as_ref()
        .map_or(Icon::Server, |nav| nav.icon);
    stack(vec![
        page_header(title.clone(), None, picker.into_iter().collect()),
        empty_state(title)
            .icon(glyph)
            .description(t!("ui.table.empty"))
            .view(),
    ])
    .spacing(space::S4)
    .into()
}

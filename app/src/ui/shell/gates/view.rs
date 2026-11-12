//! The gates as they are drawn: the same words and buttons as React's.

use iced::{widget::text, Length};

use super::{cluster_down, host_down, welcome, Down, Verdict};
use crate::{
    t,
    ui::{
        shell::{
            layout::logo,
            page::World,
            registry::Capability,
            router::Route,
            status::{cluster::kind_label, engine},
        },
        theme::{
            self,
            metrics::{space, text as size},
            Element, Icon,
        },
        widgets::{
            basic::button::{self, Variant},
            feedback::{empty_state, error_message, error_message::Event, spinner, spinner_saying},
            line, stack,
        },
    },
};

/// What the user asked of a gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Go to a page: the Host's settings, the Overview, Add a host.
    Go(Route),
    /// Connect again now, or read again what couldn't be read.
    Retry,
    /// The Details of the failure the gate shows.
    Details,
    /// A command of the failure's hint, for the clipboard.
    Copy(String),
    /// Add the Engine found on this socket of this machine as a Host.
    AddEngine(String),
    /// Add the local cluster this kubeconfig context reaches.
    AddCluster(String),
    /// The Details of why the Engine on this socket can't be added.
    WhyNot(String),
}

fn failure(event: Event) -> Action {
    match event {
        Event::Details => Action::Details,
        Event::Retry => Action::Retry,
        Event::Copy(command) => Action::Copy(command),
    }
}

pub(super) fn go<'a>(label: String, route: Route, variant: Variant) -> Element<'a, Action> {
    button::text(label)
        .variant(variant)
        .on_press(Action::Go(route))
        .into()
}

pub(super) fn buttons(items: Vec<Element<'_, Action>>) -> Element<'_, Action> {
    line(items).spacing(space::S2).into()
}

/// A Host or a Cluster that isn't connected: why, and what to do. A cause
/// is shown whole, under the title; `busy` while a retry is on its way.
fn down<'a>(
    glyph: Icon,
    down: Down<'_>,
    settings: Element<'a, Action>,
    busy: bool,
) -> Element<'a, Action> {
    let mut actions = vec![];
    if down.retry_now {
        actions.push(
            button::labelled(Icon::RefreshCw, t!("core.gate.retryNow"))
                .variant(Variant::Outline)
                .on_press_maybe((!busy).then_some(Action::Retry))
                .into(),
        );
    }
    actions.push(settings);
    let mut state = empty_state(down.title).icon(glyph).action(buttons(actions));
    if let Some(detail) = down.detail {
        state = state.description(detail);
    }
    if let Some(error) = down.error {
        let mut parts = vec![error_message(error, true, failure)];
        if let Some(secs) = down.retrying_in {
            parts.push(
                text(t!("core.gate.retryingOnItsOwn", secs = secs.to_string()))
                    .size(size::SM)
                    .style(theme::text::muted)
                    .into(),
            );
        }
        state = state.detail(stack(parts).spacing(space::S2));
    }
    state.view()
}

fn unsupported<'a>(
    name: &str,
    engine: String,
    requires: Capability,
    host: Route,
) -> Element<'a, Action> {
    let description = match requires {
        Capability::Podman => t!("core.unsupported.podman", name = name),
        Capability::Containerd => t!("core.unsupported.containerd", name = name, engine = engine),
        Capability::Incus => t!("core.unsupported.incus", name = name, engine = engine),
        Capability::Docker => t!("core.unsupported.docker", name = name, engine = engine),
    };
    empty_state(t!("core.unsupported.title", name = name))
        .icon(Icon::CircleSlash)
        .description(description)
        .action(buttons(vec![
            go(
                t!("core.unsupported.overview"),
                Route::Overview,
                Variant::Outline,
            ),
            go(t!("core.gate.openSettings"), host, Variant::Primary),
        ]))
        .view()
}

/// What shows in place of the page for a verdict that isn't `Open`; `None`
/// for one that is.
pub fn gate<'a>(verdict: &Verdict<'_>, world: &World, busy: bool) -> Option<Element<'a, Action>> {
    Some(match verdict {
        Verdict::Open(_) => return None,
        Verdict::Loading => spinner(),
        Verdict::Connecting(name) => spinner_saying(t!("core.gate.connecting", name = *name)),
        Verdict::Welcome => empty_state(t!("core.gate.welcome.title"))
            .media(logo::mark(56.0))
            .description(t!("core.gate.welcome.description"))
            .action(welcome::actions(world, busy))
            .view(),
        Verdict::Unsupported { host, requires } => unsupported(
            &host.host.name,
            engine::label(host.host.engine),
            *requires,
            Route::Host {
                host_id: host.host.id.clone(),
            },
        ),
        Verdict::HostDown(view) => {
            let route = Route::Host {
                host_id: view.host.id.clone(),
            };
            let settings = go(t!("core.gate.openSettings"), route, Variant::Primary);
            down(Icon::ServerOff, host_down(view), settings, busy)
        }
        Verdict::ClustersFailed(error) | Verdict::SwarmFailed(error) => {
            error_message(error, true, failure)
        }
        Verdict::NoCluster => empty_state(t!("core.cluster.gate.none"))
            .icon(Icon::ShipWheel)
            .description(t!("core.cluster.gate.noneDescription"))
            .action(go(
                t!("core.cluster.gate.add"),
                Route::AddCluster,
                Variant::Primary,
            ))
            .view(),
        Verdict::WrongKind { cluster, wanted } => {
            let name = cluster.cluster.name.as_str();
            let route = Route::Cluster {
                cluster_id: cluster.cluster.id.clone(),
            };
            empty_state(t!("core.cluster.unsupported.title", name = name))
                .icon(Icon::CircleSlash)
                .description(t!(
                    "core.cluster.unsupported.description",
                    name = name,
                    kind = kind_label(*wanted),
                    current = kind_label(cluster.cluster.kind)
                ))
                .action(buttons(vec![
                    go(
                        t!("core.unsupported.overview"),
                        Route::Overview,
                        Variant::Outline,
                    ),
                    go(
                        t!("core.cluster.gate.openSettings"),
                        route,
                        Variant::Primary,
                    ),
                ]))
                .view()
        }
        Verdict::ClusterDown(view) => {
            let route = Route::Cluster {
                cluster_id: view.cluster.id.clone(),
            };
            let settings = go(
                t!("core.cluster.gate.openSettings"),
                route,
                Variant::Primary,
            );
            down(Icon::ShipWheel, cluster_down(view), settings, busy)
        }
        Verdict::SwarmGone => empty_state(t!("core.swarm.gone.title"))
            .icon(Icon::Network)
            .description(t!("core.swarm.gone.description"))
            .view(),
    })
    .map(|gate: Element<'a, Action>| iced::widget::container(gate).width(Length::Fill).into())
}

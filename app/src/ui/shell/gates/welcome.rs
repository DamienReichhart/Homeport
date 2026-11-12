//! What the welcome offers besides its two buttons: the Engines answering
//! on this machine's sockets and the Kubernetes clusters running on it,
//! one click to add (`LocalEngines.tsx`, `LocalClusters.tsx`).

use iced::{widget::text, Length};

use homeport_core::host::{EngineKind, HostConnection};
use homeport_hosts::{detect::DetectedEngine, service::HostInput};
use homeport_kubernetes::kubeconfig::{
    detect::{LocalCluster, LocalTool, MissingLocalCluster},
    ContextChoice,
};

use super::view::{buttons, go, Action};
use crate::{
    t,
    ui::{
        shell::{page::World, router::Route},
        theme::{
            self, font, icon,
            metrics::{space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::{
                self,
                button::{self, Variant},
                BadgeVariant,
            },
            feedback::{error_message, error_message::Event},
            filling, line, stack,
        },
    },
};

/// The Engines found on this machine that aren't Hosts yet.
pub fn engines(world: &World) -> Vec<&DetectedEngine> {
    let added = |socket: &str| {
        world.host_list().iter().any(|view| {
            matches!(&view.host.connection, HostConnection::Local { socket_path } if socket_path == socket)
        })
    };
    let found = world
        .local_engines
        .data()
        .map_or(&[][..], |list| list.as_slice());
    found
        .iter()
        .filter(|engine| !added(&engine.socket_path))
        .collect()
}

/// The clusters found on this machine that aren't Clusters yet, and those
/// the kubeconfig doesn't reach.
pub fn clusters(world: &World) -> (Vec<&LocalCluster>, &[MissingLocalCluster]) {
    let Some(detected) = world.local_clusters.data() else {
        return (vec![], &[]);
    };
    let found = detected
        .found
        .iter()
        .filter(|c| c.added.is_none())
        .collect();
    (found, &detected.missing)
}

/// The Host one click makes of the Engine found on `socket`.
pub fn host_input(world: &World, socket: &str) -> Option<HostInput> {
    let engine = engines(world)
        .into_iter()
        .find(|engine| engine.socket_path == socket && engine.problem.is_none())?;
    Some(HostInput {
        name: engine.label.clone(),
        connection: HostConnection::Local {
            socket_path: engine.socket_path.clone(),
        },
        engine: engine.engine,
        credentials: None,
        bastion_keys: vec![],
    })
}

/// The Cluster one click makes of the local cluster `context` reaches.
pub fn context_choice(world: &World, context: &str) -> Option<ContextChoice> {
    let (found, _) = clusters(world);
    let local = found
        .into_iter()
        .find(|local| local.context == context && local.server_version.is_some())?;
    Some(ContextChoice {
        context: local.context.clone(),
        name: local.suggested_name.clone(),
    })
}

pub fn tool_label(tool: LocalTool) -> String {
    match tool {
        LocalTool::DockerDesktop => t!("core.localClusters.tool.dockerDesktop"),
        LocalTool::Orbstack => t!("core.localClusters.tool.orbstack"),
        LocalTool::RancherDesktop => t!("core.localClusters.tool.rancherDesktop"),
        LocalTool::Kind => t!("core.localClusters.tool.kind"),
        LocalTool::K3d => t!("core.localClusters.tool.k3d"),
        LocalTool::Minikube => t!("core.localClusters.tool.minikube"),
    }
}

fn version(engine: &DetectedEngine) -> String {
    let version = engine.engine_version.as_str();
    match engine.engine {
        EngineKind::Docker => t!("core.host.engineVersion", version = version),
        EngineKind::Containerd => t!("core.host.containerdVersion", version = version),
        EngineKind::Incus => t!("core.host.incusVersion", version = version),
    }
}

/// A name, what it is beside it in the quieter colour, and where it is
/// under them in the machine's letters.
fn named<'a>(glyph: Icon, name: String, what: String, place: String) -> Element<'a, Action> {
    let title = line(vec![
        text(name).size(size::SM).font(font::medium()).into(),
        text(what).size(size::SM).style(theme::text::muted).into(),
    ])
    .spacing(space::S1_5);
    let place = text(place)
        .size(size::XS)
        .font(font::MONO)
        .style(theme::text::muted);
    line(vec![
        icon(glyph).style(theme::svg::muted).into(),
        stack(vec![title.into(), place.into()]).into(),
    ])
    .spacing(space::S2)
    .width(Length::Fill)
    .into()
}

fn add<'a>(label: String, action: Action, busy: bool) -> Element<'a, Action> {
    button::labelled(Icon::Plus, label)
        .size(ButtonSize::Sm)
        .on_press_maybe((!busy).then_some(action))
        .into()
}

fn engine_row<'a>(engine: &DetectedEngine, busy: bool) -> Element<'a, Action> {
    let socket = engine.socket_path.clone();
    if let Some(problem) = &engine.problem {
        let name = line(vec![
            text(engine.label.clone())
                .size(size::SM)
                .font(font::medium())
                .into(),
            text(socket.clone())
                .size(size::XS)
                .font(font::MONO)
                .style(theme::text::muted)
                .into(),
        ])
        .spacing(space::S1_5);
        let why = error_message(problem, false, move |event| match event {
            Event::Copy(command) => Action::Copy(command),
            Event::Details | Event::Retry => Action::WhyNot(socket.clone()),
        });
        return stack(vec![name.into(), why]).spacing(space::S1).into();
    }
    line(vec![
        named(
            Icon::Monitor,
            engine.label.clone(),
            format!("· {}", version(engine)),
            socket.clone(),
        ),
        add(t!("core.localEngines.add"), Action::AddEngine(socket), busy),
    ])
    .spacing(space::S3)
    .width(Length::Fill)
    .into()
}

fn cluster_row<'a>(local: &LocalCluster, busy: bool) -> Element<'a, Action> {
    let tool = tool_label(local.tool);
    let mut what = vec![];
    if local.suggested_name != tool {
        what.push(format!("· {tool}"));
    }
    if let Some(version) = &local.server_version {
        what.push(format!(
            "· {}",
            t!("core.cluster.serverVersion", version = version.as_str())
        ));
    }
    let state = if local.server_version.is_some() {
        add(
            t!("core.localClusters.add"),
            Action::AddCluster(local.context.clone()),
            busy,
        )
    } else {
        basic::badge(t!("core.localClusters.notRunning"), BadgeVariant::Outline)
    };
    line(vec![
        named(
            Icon::ShipWheel,
            local.suggested_name.clone(),
            what.join(" "),
            local.server.clone(),
        ),
        state,
    ])
    .spacing(space::S3)
    .width(Length::Fill)
    .into()
}

fn missing_row<'a>(missing: &MissingLocalCluster) -> Element<'a, Action> {
    let says = t!(
        "core.localClusters.missing",
        tool = tool_label(missing.tool),
        name = missing.name.as_str()
    );
    let command = iced::widget::container(
        line(vec![
            filling(
                text(missing.command.clone())
                    .size(size::XS)
                    .font(font::MONO),
            )
            .into(),
            button::icon(Icon::Copy)
                .variant(Variant::Ghost)
                .size(ButtonSize::Sm)
                .on_press(Action::Copy(missing.command.clone()))
                .into(),
        ])
        .spacing(space::S2)
        .width(Length::Fill),
    )
    .padding([space::S1, space::S2])
    .style(theme::container::muted);
    stack(vec![text(says).size(size::SM).into(), command.into()])
        .spacing(space::S1)
        .into()
}

/// `max-w-xl`: the cards under the welcome.
const WIDTH: f32 = 576.0;

/// What the welcome offers: a card of the Engines found here, one of the
/// clusters, then the two buttons React always showed under them.
pub fn actions<'a>(world: &World, busy: bool) -> Element<'a, Action> {
    let mut parts: Vec<Element<'a, Action>> = vec![];
    let engines = engines(world);
    if !engines.is_empty() {
        let rows = engines.into_iter().map(|e| engine_row(e, busy)).collect();
        parts.push(
            basic::card(stack(rows).spacing(space::S2))
                .title(t!("core.localEngines.title"))
                .description(t!("core.localEngines.description"))
                .view()
                .width(WIDTH)
                .into(),
        );
    }
    let (found, missing) = clusters(world);
    if !found.is_empty() || !missing.is_empty() {
        let rows = found
            .into_iter()
            .map(|local| cluster_row(local, busy))
            .chain(missing.iter().map(missing_row))
            .collect();
        parts.push(
            basic::card(stack(rows).spacing(space::S2))
                .title(t!("core.localClusters.title"))
                .description(t!("core.localClusters.description"))
                .view()
                .width(WIDTH)
                .into(),
        );
    }
    parts.push(buttons(vec![
        go(t!("core.gate.addAnother"), Route::AddHost, Variant::Primary),
        go(
            t!("core.gate.addCluster"),
            Route::AddCluster,
            Variant::Outline,
        ),
    ]));
    iced::widget::Column::with_children(parts)
        .spacing(space::S4)
        .align_x(iced::alignment::Horizontal::Center)
        .into()
}

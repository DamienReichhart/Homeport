//! The rows of the Containers page: what they look up, how they are
//! filtered and nested under their Stacks, and what each column shows.

use std::collections::{HashMap, HashSet};

use iced::Length;

use homeport_engine::containers::Container;

use super::super::{Message, State};
use crate::{
    i18n::{self, format},
    t,
    ui::{
        domain::{
            containers::{
                model::{self, Usage},
                Actions, Target,
            },
            freshness,
        },
        shell::{router::Route, status, Context},
        theme::{
            self, font,
            metrics::{space, text as size},
            Element, Tone,
        },
        widgets::{
            basic::{badge, truncated, BadgeVariant},
            feedback::status_dot,
            focus::pressable,
            line,
            overlay::{tooltip, Side},
            stack,
            table::{self, sort, Align, Column, GroupHeader, Rows, Width},
        },
    },
};

/// The label Podman puts on the containers a Quadlet unit created.
const QUADLET_UNIT: &str = "PODMAN_SYSTEMD_UNIT";

/// What stands for a number that isn't known: a container that isn't running.
const NONE: &str = "—";

/// What separates a Stack's name from its kind in a group's name: a
/// Compose project and a swarm stack of one name are two groups, the
/// Compose project first.
const KIND: char = '\u{1f}';

/// The group a container is nested under, if it belongs to a Stack.
fn group_of(container: &Container) -> Option<String> {
    model::stack_of(container).map(|(stack, swarm)| {
        let kind = if swarm { 's' } else { 'c' };
        format!("{stack}{KIND}{kind}")
    })
}

/// A group's Stack, and whether it is a swarm stack.
fn stack_of_group(group: &str) -> (&str, bool) {
    match group.rsplit_once(KIND) {
        Some((stack, kind)) => (stack, kind == "s"),
        None => (group, false),
    }
}

/// What a row looks up, borrowed from the page's state apart from its
/// table: `update` hands the table its messages while the columns read
/// the rest.
#[derive(Clone, Copy)]
pub struct Look<'a> {
    pub host_id: &'a str,
    pub host_name: &'a str,
    /// The Swarm the Host belongs to, where a swarm stack's page is.
    pub swarm_id: Option<&'a str>,
    pub query: &'a str,
    pub only_running: bool,
    pub usage: &'a Usage,
    pub newer: &'a HashSet<String>,
    pub pods: &'a HashMap<String, (String, String)>,
    pub actions: &'a Actions,
}

impl State {
    /// The table, and what its rows look up in the rest of the state.
    pub fn split<'a>(&'a mut self, ctx: &Context<'a>) -> (&'a mut table::State<String>, Look<'a>) {
        let swarm = status::swarm::of_host(ctx.world.swarm_list(), &self.host_id);
        let look = Look {
            host_id: &self.host_id,
            host_name: ctx.host().map_or("", |view| view.host.name.as_str()),
            swarm_id: swarm.map(|swarm| swarm.id.as_str()),
            query: &self.query,
            only_running: self.only_running,
            usage: &self.usage,
            newer: &self.newer,
            pods: &self.pods,
            actions: &self.actions,
        };
        (&mut self.table, look)
    }

    pub fn look<'a>(&'a self, ctx: &Context<'a>) -> Look<'a> {
        let swarm = status::swarm::of_host(ctx.world.swarm_list(), &self.host_id);
        Look {
            host_id: &self.host_id,
            host_name: ctx.host().map_or("", |view| view.host.name.as_str()),
            swarm_id: swarm.map(|swarm| swarm.id.as_str()),
            query: &self.query,
            only_running: self.only_running,
            usage: &self.usage,
            newer: &self.newer,
            pods: &self.pods,
            actions: &self.actions,
        }
    }
}

impl<'a> Look<'a> {
    /// The containers as the table reads them: those the search and the
    /// switch keep, under their Stacks (`groupByStack`).
    pub fn rows<'r>(self, list: &'r [Container]) -> Rows<'r, Container, String>
    where
        'a: 'r,
    {
        Rows::new(list, |container| container.id.clone())
            .filter(move |container| {
                model::matches(container, self.query)
                    && (!self.only_running || container.state == "running")
            })
            .group_by(group_of)
    }

    pub fn target(self, container: &Container) -> Target {
        Target::of(self.host_id, self.host_name, container)
    }

    fn cpu(self, container: &Container) -> Option<f64> {
        self.usage.get(&container.id).map(|usage| usage.cpu_percent)
    }

    fn memory(self, container: &Container) -> Option<u64> {
        self.usage
            .get(&container.id)
            .map(|usage| usage.memory_usage)
    }
}

/// A badge that leads somewhere, and says where under the pointer.
fn link_badge<'a>(
    label: &str,
    variant: BadgeVariant,
    says: String,
    route: Route,
) -> Element<'a, Message> {
    let badge = pressable(badge(label, variant))
        .padding(0.0)
        .style(theme::button::part::bare)
        .on_press(Message::Go(route));
    tooltip(badge, says, Side::Top).into()
}

/// What a container is called in its row: its service under its Stack,
/// its name otherwise.
fn shown_name(container: &Container) -> &str {
    let service = container.service.as_deref().filter(|s| !s.is_empty());
    match (model::stack_of(container), service) {
        (Some(_), Some(service)) => service,
        _ => &container.name,
    }
}

/// `text` taking the room its row leaves, and cut with an ellipsis where
/// that ends: what stands beside it (a badge, a mark) keeps its place.
fn fill<'a>(text: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    iced::widget::container(text)
        .width(Length::Fill)
        .align_x(crate::ui::widgets::start())
        .into()
}

fn name_cell<'a>(look: Look<'a>, container: &Container) -> Element<'a, Message> {
    let name = truncated(shown_name(container)).font(font::medium());
    let mut title: Vec<Element<'a, Message>> = vec![fill(name)];
    let unit = container.labels.get(QUADLET_UNIT).filter(|u| !u.is_empty());
    if let Some(unit) = unit {
        title.push(link_badge(
            unit,
            BadgeVariant::Secondary,
            t!("containers.fromUnit", unit = unit.as_str()),
            Route::Quadlet { name: unit.clone() },
        ));
    }
    if let Some((id, name)) = look.pods.get(&container.id) {
        title.push(link_badge(
            name,
            BadgeVariant::Outline,
            t!("containers.inPod", pod = name.as_str()),
            Route::PodmanPod { id: id.clone() },
        ));
    }
    let id = truncated(format::short_id(&container.id))
        .font(font::MONO)
        .size(size::XS)
        .style(theme::text::muted);
    // The row fills its cell, so the dot keeps its place whichever side
    // it is on and the name and the id are cut, not the dot.
    line(vec![
        status_dot(model::tone(&container.state, container.health.as_deref())),
        stack(vec![
            line(title).spacing(space::S1_5).width(Length::Fill).into(),
            id.into(),
        ])
        .into(),
    ])
    .spacing(space::S2)
    .width(Length::Fill)
    .into()
}

fn image_cell<'a>(look: Look<'a>, container: &Container) -> Element<'a, Message> {
    // The whole reference under the pointer, as the React cell's `title`.
    let image = tooltip(
        truncated(container.image.clone()),
        container.image.clone(),
        Side::Top,
    );
    let mut parts = vec![fill(image)];
    if look.newer.contains(&container.id) {
        parts.push(freshness::newer_mark(freshness::newer_label(&[
            &container.image
        ])));
    }
    line(parts).spacing(space::S1_5).width(Length::Fill).into()
}

fn status_cell<'a>(container: &Container) -> Element<'a, Message> {
    let status = if container.status.is_empty() {
        &container.state
    } else {
        &container.status
    };
    // The health keeps its room; the status is cut before it.
    let mut parts: Vec<Element<'a, Message>> = vec![fill(truncated(status.clone()))];
    if let Some(health) = &container.health {
        let variant = if health == "unhealthy" {
            BadgeVariant::Destructive
        } else {
            BadgeVariant::Outline
        };
        parts.push(badge(health.clone(), variant));
    }
    line(parts).spacing(space::S1_5).width(Length::Fill).into()
}

/// How many ports a row shows.
const PORTS: usize = 3;

/// Up to three ports, wrapped onto the lines the row has
/// (`flex flex-wrap`); the published ones, which come first, open.
fn ports_cell<'a>(container: &Container) -> Element<'a, Message> {
    let ports = model::port_labels(container).into_iter().take(PORTS);
    let labels = ports.map(|port| -> Element<'a, Message> {
        let label = truncated(port.label).font(font::MONO).size(size::XS);
        match port.public_port.filter(|public| *public != 0) {
            // Where it is published is a link to it.
            Some(public) => pressable(label)
                .padding(0.0)
                .style(theme::button::link)
                .on_press(Message::OpenPort(public))
                .into(),
            None => label.style(theme::text::muted).into(),
        }
    });
    // Left to right whatever the language, as the mappings read.
    iced::widget::Row::from_vec(labels.collect())
        .spacing(space::S1)
        .wrap()
        .vertical_spacing(1.0)
        .into()
}

/// The columns of the Containers page. React's table had no sort; here
/// every column but the ports sorts the containers under their Stack.
pub fn columns<'a>(look: Look<'a>) -> Vec<Column<'a, Container, Message>> {
    // As wide as the language writes a size: "1023.9 MB", "1023.9 ميغابايت".
    let memory = table::width_for(&format::bytes(1023.9 * 1024.0 * 1024.0));
    let number = |value: Option<String>| -> Element<'a, Message> {
        table::text(value.unwrap_or_else(|| NONE.to_owned())).into()
    };
    vec![
        Column::new(
            "name",
            t!("containers.column.name"),
            move |c: &Container| name_cell(look, c),
        )
        .width(Width::Fill(4))
        .min_width(108.0)
        .hideable(false)
        .sort_by(|a, b| sort::natural(&a.name, &b.name))
        .typed(|c| shown_name(c).to_owned()),
        Column::new(
            "image",
            t!("containers.column.image"),
            move |c: &Container| image_cell(look, c),
        )
        .width(Width::Fill(3))
        .min_width(100.0)
        .sort_by(|a, b| sort::natural(&a.image, &b.image)),
        Column::new("status", t!("containers.column.status"), status_cell)
            .width(Width::Fill(3))
            .min_width(112.0)
            .sort_by(|a, b| sort::natural(&a.state, &b.state)),
        Column::new("ports", t!("containers.column.ports"), ports_cell)
            .width(Width::Fill(3))
            .min_width(96.0),
        Column::new("cpu", t!("containers.column.cpu"), move |c: &Container| {
            number(look.cpu(c).map(format::percent))
        })
        .width(Width::Fill(1))
        .between(58.0, 90.0)
        .align(Align::End)
        .sort_by(move |a, b| {
            look.cpu(a)
                .partial_cmp(&look.cpu(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .descending_first(),
        Column::new(
            "memory",
            t!("containers.column.memory"),
            move |c: &Container| {
                #[allow(clippy::cast_precision_loss)]
                number(look.memory(c).map(|bytes| format::bytes(bytes as f64)))
            },
        )
        .width(Width::Fill(2))
        .between(memory, memory + 24.0)
        .align(Align::End)
        .sort_by(move |a, b| look.memory(a).cmp(&look.memory(b)))
        .descending_first(),
        Column::new(
            "created",
            t!("containers.column.created"),
            |c: &Container| {
                truncated(format::relative(c.created))
                    .style(theme::text::muted)
                    .into()
            },
        )
        .width(Width::Fill(2))
        .between(96.0, 140.0)
        .sort_by(|a, b| a.created.cmp(&b.created))
        .descending_first(),
        // The start and stop button, before the menu.
        Column::new("toggle", String::new(), move |c: &Container| {
            look.actions.toggle(look.target(c)).map(Message::Actions)
        })
        .width(Width::Fixed(36.0))
        .resizable(false)
        .hideable(false),
    ]
}

/// A Stack's header: its status, its name and how many of its containers
/// run, their CPU and memory together, and the way to its page
/// (`StackGroupRows`).
pub fn stack_header<'a>(
    look: Look<'a>,
    group: &str,
    members: &[&Container],
) -> GroupHeader<'a, Message> {
    let (name, swarm) = stack_of_group(group);
    let running = members.iter().filter(|c| c.state == "running").count();
    let tone = match running {
        0 => Tone::Muted,
        all if all == members.len() => Tone::Success,
        _ => Tone::Warning,
    };
    let mut label: Vec<Element<'a, Message>> = vec![
        status_dot(tone),
        table::text(name).font(font::medium()).into(),
    ];
    if swarm {
        label.push(badge(t!("containers.stack.swarm"), BadgeVariant::Outline));
    }
    label.push(
        // The two numbers are one fraction: "1/2", in Arabic too.
        table::text(i18n::joined(&t!(
            "containers.stack.running",
            running = running,
            count = members.len()
        )))
        .size(size::XS)
        .style(theme::text::muted)
        .into(),
    );
    let total = |value: String| -> Element<'a, Message> {
        table::text(if running == 0 { NONE.to_owned() } else { value }).into()
    };
    let cpu: f64 = members.iter().filter_map(|c| look.cpu(c)).sum();
    let memory: u64 = members.iter().filter_map(|c| look.memory(c)).sum();
    #[allow(clippy::cast_precision_loss)]
    let mut cells = vec![
        ("cpu", total(format::percent(cpu))),
        ("memory", total(format::bytes(memory as f64))),
    ];
    // A swarm stack's page is its Swarm's: there is none to open on a Host
    // whose Swarm isn't known.
    let route = match (swarm, look.swarm_id) {
        (false, _) => Some(Route::Stack {
            name: name.to_owned(),
        }),
        (true, Some(swarm_id)) => Some(Route::SwarmStack {
            swarm_id: swarm_id.to_owned(),
            name: name.to_owned(),
        }),
        (true, None) => None,
    };
    if let Some(route) = route {
        // One line, cut where its column ends, as every cell.
        let open = pressable(table::text(t!("containers.stack.open")).font(font::medium()))
            .padding(0.0)
            .style(theme::button::link)
            .on_press(Message::Go(route));
        cells.push(("created", open.into()));
    }
    GroupHeader {
        label: line(label).spacing(space::S2).into(),
        cells,
    }
}

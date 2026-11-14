//! The Inspect tab: the document's sections, each in its card, or the
//! document itself as a tree.

use std::sync::Arc;

use iced::{widget::text, Length};

use crate::{
    data::Resource,
    i18n::format,
    t,
    ui::{
        features::docker::containers::detail::{
            model::{
                inspect::{Check, Details, Inspected, Mount, Network, Pair, Port},
                zone,
            },
            tabs::inspect::{Inspect, Message, Section},
        },
        theme::{
            self, font,
            metrics::{space, text as size},
            Element,
        },
        widgets::{
            basic::{badge, card, technical, truncated, BadgeVariant},
            data::{json, key_value, key_value::NONE, Item},
            feedback::{error_message, spinner},
            form::{tabs, TabsVariant},
            line, paragraph, stack,
            table::{self, Column, Rows, Table, Width},
        },
    },
};

/// What the Engine calls no restart policy: its word, of no language.
const NO_POLICY: &str = "no";

/// How tall the raw document may grow before it scrolls.
const RAW_HEIGHT: f32 = 560.0;

/// A cell of something a machine named: whole under the pointer when it
/// lacks room.
fn mono<'a>(value: &str) -> Element<'a, Message> {
    truncated(technical(value))
        .font(font::MONO)
        .size(size::XS)
        .with_tip()
        .into()
}

fn plain<'a>(value: impl Into<String>) -> Element<'a, Message> {
    truncated(value).with_tip().into()
}

/// The last runs of the healthcheck: when, how it ended, what it said.
pub fn checks<'a>() -> Vec<Column<'a, Check, Message>> {
    vec![
        Column::new(
            "started",
            t!("containers.inspect.column.started"),
            |check: &Check| {
                let at = check.started.map(|at| format::time_in(at, &zone()));
                plain(at.unwrap_or_else(|| NONE.to_owned()))
            },
        )
        .width(Width::Fixed(112.0)),
        Column::new(
            "exit",
            t!("containers.inspect.column.exit"),
            |check: &Check| plain(check.exit_code.to_string()),
        )
        .width(Width::Fixed(56.0)),
        Column::new(
            "output",
            t!("containers.inspect.column.output"),
            |check: &Check| mono(&check.output),
        )
        .width(Width::Fill(3)),
    ]
}

/// A name and its value: the environment's variables, or the labels.
pub fn pairs<'a>(labels: bool) -> Vec<Column<'a, Pair, Message>> {
    let name = if labels {
        t!("containers.inspect.column.label")
    } else {
        t!("containers.inspect.column.variable")
    };
    vec![
        Column::new("name", name, |pair: &Pair| mono(&pair.name)).width(Width::Fill(2)),
        Column::new(
            "value",
            t!("containers.inspect.column.value"),
            |pair: &Pair| mono(&pair.value),
        )
        .width(Width::Fill(3)),
    ]
}

pub fn mounts<'a>() -> Vec<Column<'a, Mount, Message>> {
    vec![
        Column::new(
            "type",
            t!("containers.inspect.column.type"),
            |mount: &Mount| plain(technical(&mount.kind)),
        )
        .width(Width::Fixed(72.0)),
        Column::new(
            "source",
            t!("containers.inspect.column.source"),
            |mount: &Mount| mono(&mount.source),
        )
        .width(Width::Fill(2)),
        Column::new(
            "destination",
            t!("containers.inspect.column.destination"),
            |mount: &Mount| mono(&mount.destination),
        )
        .width(Width::Fill(2)),
        Column::new(
            "mode",
            t!("containers.inspect.column.mode"),
            |mount: &Mount| {
                plain(if mount.writable {
                    t!("containers.inspect.readWrite")
                } else {
                    t!("containers.inspect.readOnly")
                })
            },
        )
        .width(Width::Fixed(96.0)),
    ]
}

pub fn ports<'a>() -> Vec<Column<'a, Port, Message>> {
    vec![
        Column::new(
            "container",
            t!("containers.inspect.column.container"),
            |port: &Port| mono(&port.port),
        ),
        Column::new(
            "published",
            t!("containers.inspect.column.publishedOn"),
            |port: &Port| match &port.published {
                Some(address) => mono(address),
                None => plain(t!("containers.inspect.notPublished")),
            },
        )
        .width(Width::Fill(2)),
    ]
}

pub fn networks<'a>() -> Vec<Column<'a, Network, Message>> {
    let address = |address: &Option<String>| mono(address.as_deref().unwrap_or(NONE));
    vec![
        Column::new(
            "network",
            t!("containers.inspect.column.network"),
            |network: &Network| plain(technical(&network.name)),
        )
        .width(Width::Fill(2)),
        Column::new(
            "ip",
            t!("containers.inspect.column.ip"),
            move |network: &Network| address(&network.ip),
        )
        .width(Width::Fill(2)),
        Column::new(
            "gateway",
            t!("containers.inspect.column.gateway"),
            move |network: &Network| address(&network.gateway),
        )
        .width(Width::Fill(2)),
        Column::new(
            "aliases",
            t!("containers.inspect.column.aliases"),
            |network: &Network| plain(technical(&network.aliases.join(", "))),
        )
        .width(Width::Fill(2)),
    ]
}

/// A section's rows in the kit's table, or "None" when it has none
/// (`SimpleTable`).
fn listed<'a, T>(
    state: &'a table::State<usize>,
    columns: Vec<Column<'a, T, Message>>,
    rows: &[T],
    at: impl Fn(&T) -> usize,
    section: Section,
) -> Element<'a, Message> {
    if rows.is_empty() {
        return paragraph(t!("common.none"))
            .style(theme::text::muted)
            .into();
    }
    Table::new(state, columns, Rows::new(rows, at), move |message| {
        Message::Table(section, message)
    })
    .view()
}

/// What the container is: `running`, or how it ended.
fn state_said(details: &Details) -> String {
    let status = technical(&details.status);
    let state = if details.running {
        status
    } else {
        t!(
            "containers.inspect.exited",
            status = status,
            code = details.exit_code
        )
    };
    if details.out_of_memory {
        t!("containers.inspect.outOfMemory", state = state)
    } else {
        state
    }
}

fn restart_said(details: &Details) -> String {
    let count = details.restart_count;
    match &details.restart {
        None => technical(NO_POLICY),
        Some((policy, 0)) => t!(
            "containers.inspect.restarted",
            policy = technical(policy),
            count = count
        ),
        Some((policy, max)) => t!(
            "containers.inspect.restartedMax",
            policy = technical(policy),
            max = *max,
            count = count
        ),
    }
}

fn container<'a>(details: &Details) -> Element<'a, Message> {
    let time = |at: Option<i64>| at.map(|at| format::date_time_in(at, &zone()));
    let or_dash = |value: &Option<String>| value.clone().unwrap_or_else(|| NONE.to_owned());
    let items = vec![
        Item::mono(t!("containers.inspect.id"), details.id.clone()),
        Item::text(t!("containers.inspect.image"), technical(&details.image)),
        Item::maybe(t!("containers.inspect.created"), time(details.created)),
        Item::text(t!("containers.inspect.state"), state_said(details)),
        Item::maybe(t!("containers.inspect.started"), time(details.started)),
        Item::mono(
            t!("containers.inspect.entrypoint"),
            or_dash(&details.entrypoint),
        ),
        Item::mono(t!("containers.inspect.command"), or_dash(&details.command)),
        Item::text(
            t!("containers.inspect.workingDir"),
            technical(&or_dash(&details.working_dir)),
        ),
        Item::text(
            t!("containers.inspect.user"),
            details
                .user
                .as_deref()
                .map_or_else(|| t!("containers.inspect.imageDefault"), technical),
        ),
        Item::text(
            t!("containers.inspect.restartPolicy"),
            restart_said(details),
        ),
        Item::text(
            t!("containers.inspect.networkMode"),
            technical(&details.network_mode),
        ),
    ];
    card(key_value(items))
        .title(t!("containers.inspect.container"))
        .into()
}

fn health<'a>(inspect: &'a Inspect, details: &Details) -> Element<'a, Message> {
    let body: Element<'a, Message> = match &details.health {
        None => paragraph(t!("containers.inspect.noHealthcheck"))
            .style(theme::text::muted)
            .into(),
        Some(health) => {
            let variant = if health.status == "unhealthy" {
                BadgeVariant::Destructive
            } else {
                BadgeVariant::Outline
            };
            let streak = t!(
                "containers.inspect.failingStreak",
                count = health.failing_streak
            );
            stack(vec![
                line(vec![
                    badge(technical(&health.status), variant),
                    text(streak).size(size::SM).style(theme::text::muted).into(),
                ])
                .spacing(space::S2)
                .into(),
                listed(
                    &inspect.tables.checks,
                    checks(),
                    &health.checks,
                    |check| check.at,
                    Section::Checks,
                ),
            ])
            .spacing(space::S2)
            .into()
        }
    };
    card(body).title(t!("containers.inspect.health")).into()
}

/// Two sections side by side, each half the row.
fn pair<'a>(first: Element<'a, Message>, second: Element<'a, Message>) -> Element<'a, Message> {
    line(vec![first, second])
        .spacing(space::S4)
        .width(Length::Fill)
        .align_y(iced::alignment::Vertical::Top)
        .into()
}

fn details<'a>(inspect: &'a Inspect, details: &Details, wide: bool) -> Element<'a, Message> {
    let tables = &inspect.tables;
    let section = |title: String, body: Element<'a, Message>| -> Element<'a, Message> {
        card(body).title(title).into()
    };
    let environment = section(
        t!("containers.inspect.environment"),
        listed(
            &tables.environment,
            pairs(false),
            &details.env,
            |pair| pair.at,
            Section::Environment,
        ),
    );
    let mounted = section(
        t!("containers.inspect.mounts"),
        listed(
            &tables.mounts,
            mounts(),
            &details.mounts,
            |mount| mount.at,
            Section::Mounts,
        ),
    );
    let published = section(
        t!("containers.inspect.ports"),
        listed(
            &tables.ports,
            ports(),
            &details.ports,
            |port| port.at,
            Section::Ports,
        ),
    );
    let attached = section(
        t!("containers.inspect.networks"),
        listed(
            &tables.networks,
            networks(),
            &details.networks,
            |network| network.at,
            Section::Networks,
        ),
    );
    let labelled = section(
        t!("containers.inspect.labels"),
        listed(
            &tables.labels,
            pairs(true),
            &details.labels,
            |pair| pair.at,
            Section::Labels,
        ),
    );
    let about = container(details);
    let healthy = health(inspect, details);
    // `lg:grid-cols-2`, the labels across both.
    let sections = if wide {
        vec![
            pair(about, healthy),
            pair(environment, mounted),
            pair(published, attached),
            labelled,
        ]
    } else {
        vec![
            about,
            healthy,
            environment,
            mounted,
            published,
            attached,
            labelled,
        ]
    };
    stack(sections).spacing(space::S4).into()
}

pub fn view<'a>(
    inspect: &'a Inspect,
    read: &Resource<Arc<Inspected>>,
    wide: bool,
) -> Element<'a, Message> {
    // `inspect.data ? .. : inspect.error ? <ErrorMessage /> : <Spinner />`
    let Some(read) = read.data() else {
        return match read.error() {
            Some(error) => error_message(error, true, Message::Failure),
            None => spinner(),
        };
    };
    let views = tabs(
        vec![t!("common.details"), t!("containers.inspect.raw")],
        usize::from(inspect.raw),
        TabsVariant::Default,
        Message::Show,
    );
    let shown = match (&inspect.json, inspect.raw) {
        (Some(json), true) => json::View::new(json, Message::Json)
            .max_height(RAW_HEIGHT)
            .view(),
        // The tree is made as soon as the document is read.
        (None, true) => spinner(),
        (_, false) => details(inspect, &read.details, wide),
    };
    stack(vec![views, shown]).spacing(space::S4).into()
}

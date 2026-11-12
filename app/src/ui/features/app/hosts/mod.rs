//! The Hosts page: every Host with its connection, its address and its
//! status, in the kit's table. A placeholder for `src/features/hosts`,
//! which its ticket ports whole (Add host, the bundles, a Host's
//! settings); it already stands on what a real feature does: the shell's
//! interface, the Hosts read through the data layer, and the table.

use std::sync::Arc;

use iced::Task;

use homeport_core::host::HostConnection;
use homeport_hosts::service::HostView;

use crate::{
    data::{keys, resources, Resource},
    t,
    ui::{
        shell::{layout::page_header, router::Route, status, Context},
        theme::{
            font,
            metrics::{space, text as size},
            Element,
        },
        widgets::{
            basic::{badge, BadgeVariant},
            feedback::{error_message, error_message::Event, status_dot, Surfaced},
            line, stack,
            table::{self, Column, Table, Width},
        },
    },
};

#[derive(Debug)]
pub struct State {
    /// The table's sort, its columns' widths and the row the keyboard is
    /// on. Boxed: a page's state is one variant among fifty.
    table: Box<table::State<String>>,
}

impl State {
    fn new() -> Self {
        Self {
            // A press opens the Host, as `onRowClick` did.
            table: Box::new(table::State::new().opening_on_press()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    /// What was asked of the failure the page shows.
    Failure(Event),
    Table(table::Message<String>),
}

pub fn open(ctx: &Context<'_>) -> (State, Task<Message>) {
    ctx.watch(resources::hosts());
    (State::new(), Task::none())
}

fn hosts(ctx: &Context<'_>) -> Resource<Arc<Vec<HostView>>> {
    ctx.data.read(&keys::host::list())
}

fn id(view: &HostView) -> String {
    view.host.id.clone()
}

pub fn update(state: &mut State, message: Message, ctx: &Context<'_>) -> Task<Message> {
    match message {
        Message::Failure(Event::Retry) => ctx.invalidate(keys::host::list()),
        Message::Failure(Event::Details) => {
            if let Some(error) = hosts(ctx).error() {
                ctx.details(Surfaced::new(error.clone(), "hosts_list"));
            }
        }
        Message::Failure(Event::Copy(command)) => ctx.copy(command),
        Message::Table(message) => {
            let hosts = hosts(ctx);
            let rows = hosts.data().map_or(&[][..], |list| list.as_slice());
            if let Some(table::Event::Opened(host_id)) = state.table.update(message, rows, id) {
                ctx.open(Route::Host { host_id });
            }
        }
    }
    Task::none()
}

pub fn request(_state: &mut State, _action: &str, _ctx: &Context<'_>) -> Task<Message> {
    Task::none()
}

/// What a connection is called (`connectionLabel`).
fn kind(connection: &HostConnection) -> String {
    match connection {
        HostConnection::Local { .. } => t!("core.connection.local"),
        HostConnection::Tcp { .. } => t!("hosts.kind.tcp"),
        HostConnection::Tls { .. } => t!("hosts.kind.tls"),
        HostConnection::Ssh { .. } => t!("hosts.kind.ssh"),
    }
}

/// Where a Host lives, in one line: a socket path, an address or
/// `user@server` (`describeAddress`).
fn address(connection: &HostConnection) -> String {
    match connection {
        HostConnection::Local { socket_path } => socket_path.clone(),
        HostConnection::Tcp { address } | HostConnection::Tls { address } => address.clone(),
        HostConnection::Ssh {
            host, port, user, ..
        } => {
            if *port == 22 {
                format!("{user}@{host}")
            } else {
                format!("{user}@{host}:{port}")
            }
        }
    }
}

/// The Hosts table's columns (`hostColumns`).
fn columns<'a>() -> Vec<Column<'a, HostView, Message>> {
    vec![
        Column::new("name", t!("hosts.column.name"), |view: &HostView| {
            table::text(view.host.name.clone())
                .font(font::medium())
                .into()
        })
        .width(Width::Fill(2))
        .sort_by(|a, b| a.host.name.cmp(&b.host.name)),
        Column::new("kind", t!("hosts.column.type"), |view: &HostView| {
            badge(kind(&view.host.connection), BadgeVariant::Outline)
        })
        .width(Width::Fixed(120.0))
        .sort_by(|a, b| kind(&a.host.connection).cmp(&kind(&b.host.connection))),
        Column::new("address", t!("hosts.column.address"), |view: &HostView| {
            table::text(address(&view.host.connection))
                .font(font::MONO)
                .size(size::XS)
                .into()
        })
        .width(Width::Fill(3))
        .sort_by(|a, b| address(&a.host.connection).cmp(&address(&b.host.connection))),
        Column::new("status", t!("hosts.column.status"), |view: &HostView| {
            let summary = status::host::describe(&view.status);
            line(vec![
                status_dot(summary.tone),
                table::text(summary.label).into(),
            ])
            .spacing(space::S1_5)
            .into()
        })
        .width(Width::Fixed(150.0))
        .sort_by(|a, b| {
            let label = |view: &HostView| status::host::describe(&view.status).label;
            label(a).cmp(&label(b))
        }),
    ]
}

pub fn view<'a>(state: &'a State, ctx: &Context<'a>) -> Element<'a, Message> {
    let hosts = hosts(ctx);
    let mut parts = vec![page_header(
        t!("hosts.title"),
        Some(t!("hosts.description")),
        vec![],
    )];
    if let Some(error) = hosts.error() {
        parts.push(error_message(error, true, Message::Failure));
    }
    // A list that could not be read and was never read shows its failure
    // alone, as the React page did.
    if hosts.data().is_some() || hosts.error().is_none() {
        let rows = hosts.data().map_or(&[][..], |list| list.as_slice());
        parts.push(
            Table::new(&state.table, columns(), rows, id, Message::Table)
                .loading(hosts.is_pending())
                .empty_label(t!("hosts.empty"))
                .view(),
        );
    }
    stack(parts).spacing(space::S4).into()
}

#[cfg(test)]
mod tests;

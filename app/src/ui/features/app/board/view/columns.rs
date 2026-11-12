//! The rows of the Board: what they look up, how they are filtered, and
//! what each column shows and sorts by (`BoardTable`, `BoardRowView`).

use std::collections::HashMap;

use iced::Length;

use homeport_engine::containers::stats::ContainerUsage;

use super::super::{
    model::{self, Filter, Row, SortKey},
    Message, State,
};
use crate::{
    i18n::format,
    t,
    ui::{
        domain::containers::{
            cells,
            model::{port_labels, tone, Usage},
            Actions,
        },
        theme::{
            self, font,
            metrics::{space, text as size},
            Element,
        },
        widgets::{
            basic::truncated,
            feedback::status_dot,
            line,
            overlay::{tooltip, Side},
            stack,
            table::{self, Align, Column, Rows, Width},
        },
    },
};

/// What stands for what a container doesn't have: no Stack, no usage.
const NONE: &str = "—";

/// What a row looks up, borrowed from the page's state apart from its
/// table: `update` hands the table its messages while the columns read
/// the rest.
#[derive(Clone, Copy)]
pub struct Look<'a> {
    pub rows: &'a [Row],
    pub filter: &'a Filter,
    pub usage: &'a HashMap<String, Usage>,
    pub actions: &'a Actions,
    pub revision: u64,
}

impl State {
    /// The table, and what its rows look up in the rest of the state.
    pub fn split(&mut self) -> (&mut table::State<String>, Look<'_>) {
        let look = Look {
            rows: &self.rows,
            filter: &self.filter,
            usage: &self.usage,
            actions: &self.actions,
            revision: self.revision,
        };
        (&mut self.table, look)
    }

    pub fn look(&self) -> Look<'_> {
        Look {
            rows: &self.rows,
            filter: &self.filter,
            usage: &self.usage,
            actions: &self.actions,
            revision: self.revision,
        }
    }
}

impl<'a> Look<'a> {
    fn usage_of(self, row: &Row) -> Option<&'a ContainerUsage> {
        self.usage.get(&*row.host_id)?.get(&row.container().id)
    }
}

/// The rows as the table reads them: those the filter keeps. The table
/// goes through them again only when the page says they changed.
pub fn rows(look: Look<'_>) -> Rows<'_, Row, String> {
    Rows::new(look.rows, |row| row.key.clone())
        .filter(move |row| look.filter.keeps(row))
        .revision(look.revision)
}

/// `text` taking the room its row leaves, and cut with an ellipsis where
/// that ends: what stands beside it keeps its place.
fn fill<'a>(text: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    iced::widget::container(text)
        .width(Length::Fill)
        .align_x(crate::ui::widgets::start())
        .into()
}

fn name_cell<'a>(row: &Row) -> Element<'a, Message> {
    let container = row.container();
    let id = truncated(format::short_id(&container.id))
        .font(font::MONO)
        .size(size::XS)
        .style(theme::text::muted)
        .with_tip();
    let name = truncated(container.name.clone())
        .font(font::medium())
        .with_tip();
    line(vec![
        status_dot(tone(&container.state, container.health.as_deref())),
        stack(vec![name.into(), id.into()]).into(),
    ])
    .spacing(space::S2)
    .width(Length::Fill)
    .into()
}

/// The first port and how many more there are; all of them under the
/// pointer, as the React cell's `title`.
fn ports_cell<'a>(row: &Row) -> Element<'a, Message> {
    let ports = port_labels(row.container());
    let Some(first) = ports.first() else {
        return table::text(String::new()).into();
    };
    let label = match ports.len() {
        1 => first.label.clone(),
        more => format!("{} +{}", first.label, more - 1),
    };
    let all: Vec<&str> = ports.iter().map(|port| port.label.as_str()).collect();
    let shown = truncated(label)
        .font(font::MONO)
        .size(size::XS)
        .style(theme::text::muted);
    tooltip(shown, format::list(&all), Side::Top).into()
}

/// The Board's columns. Each but the ports sorts, ascending first, by the
/// value the React Board sorted by; ties keep the Host, then the name.
pub fn columns<'a>(look: Look<'a>) -> Vec<Column<'a, Row, Message>> {
    let by = move |key: SortKey| {
        move |a: &Row, b: &Row| model::compare(key, a, b, |row| look.usage_of(row))
    };
    let number = |value: Option<String>| -> Element<'a, Message> {
        table::text(value.unwrap_or_else(|| NONE.to_owned())).into()
    };
    let muted = |value: String| -> Element<'a, Message> {
        truncated(value)
            .style(theme::text::muted)
            .lines(2)
            .with_tip()
            .into()
    };
    // As wide as the language writes a size: "1023.9 MB", "1023.9 ميغابايت".
    let memory = table::width_for(&format::bytes(1023.9 * 1024.0 * 1024.0));
    vec![
        Column::new("host", t!("board.column.host"), |row: &Row| {
            truncated(row.host_name.to_string())
                .lines(2)
                .with_tip()
                .into()
        })
        .width(Width::Fill(2))
        .min_width(64.0)
        .sort_by(by(SortKey::Host)),
        Column::new("name", t!("containers.column.name"), name_cell)
            .width(Width::Fill(4))
            .min_width(88.0)
            .hideable(false)
            .sort_by(by(SortKey::Name))
            .typed(|row| row.container().name.clone()),
        Column::new("image", t!("containers.column.image"), |row: &Row| {
            let image = row.container().image.clone();
            truncated(image).with_tip().into()
        })
        .width(Width::Fill(3))
        .min_width(60.0)
        .sort_by(by(SortKey::Image)),
        Column::new("status", t!("containers.column.status"), |row: &Row| {
            cells::status(row.container())
        })
        .width(Width::Fill(3))
        .min_width(112.0)
        .sort_by(by(SortKey::Status)),
        Column::new("stack", t!("board.column.stack"), move |row: &Row| {
            muted(
                row.container()
                    .stack
                    .clone()
                    .unwrap_or_else(|| NONE.to_owned()),
            )
        })
        .width(Width::Fill(2))
        .min_width(56.0)
        .sort_by(by(SortKey::Stack)),
        Column::new("ports", t!("containers.column.ports"), ports_cell)
            .width(Width::Fill(2))
            .min_width(54.0),
        Column::new("cpu", t!("containers.column.cpu"), move |row: &Row| {
            number(
                look.usage_of(row)
                    .map(|usage| format::percent(usage.cpu_percent)),
            )
        })
        .width(Width::Fill(1))
        .between(50.0, 90.0)
        .align(Align::End)
        .sort_by(by(SortKey::Cpu)),
        Column::new(
            "memory",
            t!("containers.column.memory"),
            move |row: &Row| {
                #[allow(clippy::cast_precision_loss)]
                number(
                    look.usage_of(row)
                        .map(|usage| format::bytes(usage.memory_usage as f64)),
                )
            },
        )
        .width(Width::Fill(2))
        .between(memory, memory + 24.0)
        .align(Align::End)
        .sort_by(by(SortKey::Memory)),
        Column::new(
            "created",
            t!("containers.column.created"),
            move |row: &Row| muted(format::relative(row.container().created)),
        )
        .width(Width::Fill(2))
        .between(80.0, 140.0)
        .sort_by(by(SortKey::Created)),
        // The start and stop button, before the menu.
        Column::new("toggle", String::new(), move |row: &Row| {
            look.actions.toggle(row.target()).map(Message::Actions)
        })
        .width(Width::Fixed(36.0))
        .resizable(false)
        .hideable(false),
    ]
}

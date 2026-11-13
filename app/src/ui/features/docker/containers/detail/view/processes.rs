//! The Processes tab: the Engine's `top`, its titles as the columns.

use std::sync::Arc;

use homeport_engine::containers::Processes;

use crate::{
    data::Resource,
    t,
    ui::{
        features::docker::containers::detail::tabs::processes::{Message, Row, Tab},
        theme::{self, font, metrics::text as size, Element},
        widgets::{
            basic::{technical, truncated},
            feedback::{error_message, spinner},
            paragraph,
            table::{Align, Column, Rows, Table, Width},
        },
    },
};

/// What a column is known by: its place among the titles. `ps` gives
/// eight or so; past these, the Engine's other titles are not shown.
const IDS: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
];

/// The columns of the titles the Engine gave: the last (the command)
/// takes the room, the others are numbers.
pub fn columns<'a>(titles: &[String]) -> Vec<Column<'a, Row, Message>> {
    let last = titles.len().saturating_sub(1);
    titles
        .iter()
        .zip(IDS)
        .enumerate()
        .map(|(index, (title, id))| {
            let cell = move |row: &Row| -> Element<'a, Message> {
                let value = row.cells.get(index).cloned().unwrap_or_default();
                let value = truncated(technical(&value));
                if index == last {
                    value.font(font::MONO).size(size::XS).with_tip().into()
                } else {
                    value.with_tip().into()
                }
            };
            let column = Column::new(id, technical(title), cell);
            if index == last {
                column.width(Width::Fill(4))
            } else {
                column.align(Align::End).width(Width::Fixed(80.0))
            }
        })
        .collect()
}

pub fn rows(tab: &Tab) -> Rows<'_, Row, usize> {
    Rows::new(&tab.rows, |row| row.at)
}

pub fn view<'a>(
    tab: &'a Tab,
    read: &Resource<Arc<Processes>>,
    running: bool,
) -> Element<'a, Message> {
    if !running {
        return paragraph(t!("containers.notRunning"))
            .style(theme::text::muted)
            .into();
    }
    // A list read before stays while it is read again.
    if read.data().is_none() {
        return match read.error() {
            Some(error) => error_message(error, true, Message::Failure),
            None => spinner(),
        };
    }
    if let Some(error) = read.error() {
        return error_message(error, true, Message::Failure);
    }
    Table::new(&tab.table, columns(&tab.titles), rows(tab), Message::Table).view()
}

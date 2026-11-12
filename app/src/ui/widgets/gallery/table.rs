//! The data table, on rows that look like containers: sorted, selected,
//! resized and scrolled, with ten rows or a hundred thousand, and in each
//! of the states that stand for the rows.

use iced::widget::text;

use super::{failure, Gallery, Message};
use crate::{
    t,
    ui::{
        theme::{self, font, metrics::space, metrics::ButtonSize, Element, Tone},
        widgets::{
            basic::button::{self, Variant},
            feedback::{empty_state, error_message, status_dot},
            line,
            overlay::menu,
            stack,
            table::{self, Align, Column, Table, Width},
        },
    },
};

/// A row of the gallery's table.
#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    pub id: u32,
    pub name: String,
    pub image: &'static str,
    pub state: usize,
    /// Tenths of a percent.
    pub cpu: u32,
    /// Mebibytes.
    pub memory: u32,
}

const NAMES: [&str; 8] = [
    "web", "db", "cache", "proxy", "worker", "queue", "api", "auth",
];
const IMAGES: [&str; 4] = ["nginx:1.27", "postgres:17", "redis:7.4", "traefik:3.1"];
const TONES: [Tone; 3] = [Tone::Success, Tone::Muted, Tone::Danger];

/// What the table shows in place of its rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Show {
    #[default]
    Rows,
    Loading,
    Empty,
    Failed,
}

/// `count` rows, the same every time.
pub fn samples(count: u32) -> Vec<Sample> {
    (0..count)
        .map(|id| {
            let at = id as usize;
            Sample {
                id,
                name: format!("{}-{}", NAMES[at % NAMES.len()], id + 1),
                image: IMAGES[(at / 3) % IMAGES.len()],
                state: (at / 2) % TONES.len(),
                cpu: (id.wrapping_mul(37) + 11) % 1000,
                memory: (id.wrapping_mul(53) + 64) % 2048,
            }
        })
        .collect()
}

fn state_label(state: usize) -> String {
    match state {
        0 => t!("board.state.running"),
        1 => t!("board.state.stopped"),
        _ => t!("board.state.unhealthy"),
    }
}

/// The columns of the gallery's table: text, a status, and two of numbers.
pub fn columns<'a>() -> Vec<Column<'a, Sample, Message>> {
    vec![
        Column::new("name", t!("containers.column.name"), |row: &Sample| {
            table::text(row.name.clone()).font(font::medium()).into()
        })
        .width(Width::Fill(2))
        .min_width(120.0)
        .sort_by(|a, b| a.name.cmp(&b.name)),
        Column::new("image", t!("containers.column.image"), |row: &Sample| {
            table::text(row.image).style(theme::text::muted).into()
        })
        .width(Width::Fill(2))
        .sort_by(|a, b| a.image.cmp(b.image)),
        Column::new("status", t!("containers.column.status"), |row: &Sample| {
            line(vec![
                status_dot(TONES[row.state]),
                table::text(state_label(row.state)).into(),
            ])
            .spacing(space::S1_5)
            .into()
        })
        .width(Width::Fixed(130.0))
        .sort_by(|a, b| a.state.cmp(&b.state)),
        Column::new("cpu", t!("containers.column.cpu"), |row: &Sample| {
            table::text(format!("{}.{} %", row.cpu / 10, row.cpu % 10)).into()
        })
        .width(Width::Fixed(90.0))
        .align(Align::End)
        .sort_by(|a, b| a.cpu.cmp(&b.cpu))
        .descending_first(),
        Column::new("memory", t!("containers.column.memory"), |row: &Sample| {
            table::text(format!("{} MiB", row.memory)).into()
        })
        .width(Width::Fixed(110.0))
        .align(Align::End)
        .sort_by(|a, b| a.memory.cmp(&b.memory))
        .descending_first(),
    ]
}

/// The gallery's table over `rows`, as a page would build it.
pub fn view<'a>(state: &'a table::State<u32>, rows: &[Sample], show: Show) -> Element<'a, Message> {
    let rows = if show == Show::Rows { rows } else { &[] };
    let selected = state.selection().len();
    let bar = (selected > 0).then(|| {
        line(vec![
            text(t!("containers.selected", count = selected))
                .size(theme::metrics::text::SM)
                .style(theme::text::muted)
                .into(),
            button::text(t!("containers.action.stop"))
                .variant(Variant::Outline)
                .size(ButtonSize::Sm)
                .on_press(Message::Chosen("Stop the selected"))
                .into(),
            button::text(t!("common.remove"))
                .variant(Variant::Destructive)
                .size(ButtonSize::Sm)
                .on_press(Message::Chosen("Remove the selected"))
                .into(),
        ])
        .spacing(space::S2)
        .into()
    });
    let failed = (show == Show::Failed).then(|| error_message(&failure(), true, Message::Failure));
    let table = Table::new(state, columns(), rows, |row| row.id, Message::Table)
        .row_menu(|_| {
            vec![
                menu::Item::action(t!("containers.action.start"), Message::Chosen("Start")),
                menu::Item::action(t!("containers.action.stop"), Message::Chosen("Stop")),
                menu::Item::Separator,
                menu::Item::action(t!("common.remove"), Message::Chosen("Remove")).destructive(),
            ]
        })
        .loading(show == Show::Loading)
        .failed(failed)
        .selection_bar(bar);
    if show == Show::Empty {
        table
            .empty(
                empty_state(t!("containers.empty.title"))
                    .icon(theme::Icon::Box)
                    .description(t!("containers.empty.description"))
                    .view(),
            )
            .view()
    } else {
        table.view()
    }
}

impl Gallery {
    pub(super) fn tables(&self) -> Element<'_, Message> {
        let choice = |label: &'static str, message: Message, on: bool| -> Element<'_, Message> {
            button::text(label)
                .variant(if on {
                    Variant::Secondary
                } else {
                    Variant::Outline
                })
                .size(ButtonSize::Sm)
                .on_press(message)
                .into()
        };
        let rows = self.table_rows.len();
        let shows = |show| self.table_show == show;
        let switches = line(vec![
            choice(
                "12 rows",
                Message::TableRows(12),
                shows(Show::Rows) && rows == 12,
            ),
            choice(
                "100,000 rows",
                Message::TableRows(100_000),
                shows(Show::Rows) && rows == 100_000,
            ),
            choice(
                "Loading",
                Message::TableShow(Show::Loading),
                shows(Show::Loading),
            ),
            choice("Empty", Message::TableShow(Show::Empty), shows(Show::Empty)),
            choice(
                "Failed",
                Message::TableShow(Show::Failed),
                shows(Show::Failed),
            ),
        ])
        .spacing(space::S2);
        stack(vec![
            switches.into(),
            view(&self.table, &self.table_rows, self.table_show),
        ])
        .spacing(space::S4)
        .into()
    }
}

//! The Logs tab: the kit's viewer under the controls of the stream.

use iced::widget::text;

use crate::{
    i18n::format,
    t,
    ui::{
        features::docker::containers::detail::tabs::logs::{
            since_labels, tail_labels, Channel, Logs, Message,
        },
        theme::{
            self,
            metrics::{space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::button::{self, Variant},
            feedback::error_message,
            form::{select::select_owned, toggle_group, Toggle},
            line, log,
            overlay::{tooltip, Side},
        },
    },
};

/// `w-32`: the two selects of the stream.
const SELECT_WIDTH: f32 = 128.0;

/// The least height of the viewer (`min-h-[24rem]`).
pub const MIN_HEIGHT: f32 = 384.0;

/// What the window holds above and under the viewer: the page's padding,
/// its header, its tabs and what is between them.
const CHROME: f32 = 180.0;

/// How tall the viewer is in a window `window` tall: what the page leaves
/// (`flex-1`), and never less than [`MIN_HEIGHT`]. The page scrolls, so
/// the viewer is given a height rather than left to fill.
pub fn height(window: Option<f32>) -> f32 {
    window.map_or(MIN_HEIGHT, |window| (window - CHROME).max(MIN_HEIGHT))
}

/// The controls before the viewer's own: how much history, how far back,
/// which of the two streams, and how far a save is.
fn controls(logs: &Logs) -> Vec<Element<'_, Message>> {
    let streams = toggle_group(
        vec![
            Toggle::text(t!("containers.logs.stdout")),
            Toggle::text(t!("containers.logs.stderr")),
        ],
        |index| [logs.stdout, logs.stderr][index],
        true,
        |index| Message::Toggle([Channel::Stdout, Channel::Stderr][index]),
    );
    let mut controls = vec![
        tooltip(
            select_owned(&logs.tail_select, tail_labels(), Some(logs.tail))
                .width(SELECT_WIDTH)
                .view()
                .map(Message::Tail),
            t!("containers.logs.history"),
            Side::Top,
        )
        .into(),
        tooltip(
            select_owned(&logs.since_select, since_labels(), Some(logs.since))
                .width(SELECT_WIDTH)
                .view()
                .map(Message::Since),
            t!("containers.logs.sinceLabel"),
            Side::Top,
        )
        .into(),
        streams,
    ];
    if let Some(wrote) = logs.saving() {
        let percent = format::percent(f64::from(wrote.percent()));
        let stop = button::icon(Icon::X)
            .variant(Variant::Ghost)
            .size(ButtonSize::Sm)
            .on_press(Message::StopSaving);
        controls.push(
            line(vec![
                text(t!("containers.logs.saving", percent = percent))
                    .size(size::SM)
                    .style(theme::text::muted)
                    .into(),
                tooltip(stop, t!("common.cancel"), Side::Top).into(),
            ])
            .spacing(space::S1)
            .into(),
        );
    }
    controls
}

pub fn view(logs: &Logs, window: Option<f32>) -> Element<'_, Message> {
    let failure = logs
        .failure
        .as_ref()
        .map(|failure| error_message(&failure.error, true, Message::Failure));
    log::Viewer::new(&logs.view, Message::Viewer)
        .toolbar(controls(logs))
        .failed(failure)
        .height(height(window))
        .view()
}

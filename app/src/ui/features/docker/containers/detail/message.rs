//! What happens on a container's page.

use super::{
    state::Tab,
    tabs::{files, health, inspect, logs, processes, stats},
};
use crate::ui::{
    domain::containers::actions,
    widgets::{feedback::error_message, overlay::menu},
};

#[derive(Debug, Clone)]
pub enum Message {
    /// The arrow before the container's name.
    Back,
    /// "Back to containers", under a container that is gone.
    BackToList,
    /// A tab was pressed, or reached with the arrows.
    Tab(Tab),
    /// A button of the header, an item of its menu, or the dialog one of
    /// them opened.
    Actions(actions::Message),
    /// The menu beside the header's buttons.
    More(menu::Event),
    /// What was asked of the failure shown in place of the page.
    Failure(error_message::Event),
    /// The window's size: at first, then each time it changes.
    Resized(iced::Size),
    Logs(logs::Message),
    Inspect(inspect::Message),
    Processes(processes::Message),
    Health(health::Message),
    Files(files::Message),
    Stats(stats::Message),
    /// When the container had started (`State.StartedAt`), as the read
    /// for the log stream said; `None` if it could not be read.
    Started(super::restarts::StartRead, Option<String>),
}

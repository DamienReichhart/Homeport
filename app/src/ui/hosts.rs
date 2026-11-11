//! The Hosts page: every Host with its connection status.

use std::sync::Arc;

use iced::{
    widget::{column, row, text},
    Element, Task,
};

use homeport_hosts::{registry::HostStatus, service::HostView};

use crate::app::{events::Event, AppState};

#[derive(Default)]
pub struct Hosts {
    list: Vec<HostView>,
    error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<Arc<Vec<HostView>>, String>),
}

impl Hosts {
    pub fn load(&mut self, state: &Arc<AppState>) -> Task<Message> {
        let state = state.clone();
        Task::perform(
            async move {
                state
                    .hosts
                    .list()
                    .map(Arc::new)
                    .map_err(|e| e.message.to_string())
            },
            Message::Loaded,
        )
    }

    pub fn update(&mut self, _state: &Arc<AppState>, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(Ok(list)) => {
                // The list is only shared with this message.
                self.list = Arc::try_unwrap(list).unwrap_or_default();
                self.error = None;
            }
            Message::Loaded(Err(e)) => self.error = Some(e),
        }
        Task::none()
    }

    pub fn on_event(&mut self, event: &Event) {
        if let Event::HostStatus { host_id, status } = event {
            if let Some(view) = self.list.iter_mut().find(|v| &v.host.id == host_id) {
                view.status = (**status).clone();
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut rows = column![text("Hosts").size(24)].spacing(8);
        if let Some(e) = &self.error {
            rows = rows.push(text(e.as_str()));
        }
        for view in &self.list {
            rows = rows.push(row![text(view.host.name.as_str()), text(label(&view.status))].spacing(16));
        }
        rows.into()
    }
}

fn label(status: &HostStatus) -> &'static str {
    match status {
        HostStatus::Disabled => "Disabled",
        HostStatus::Connecting => "Connecting",
        HostStatus::Connected { .. } => "Connected",
        HostStatus::Unreachable { .. } => "Unreachable",
        HostStatus::Unauthorized { .. } => "Unauthorized",
        _ => "Other",
    }
}

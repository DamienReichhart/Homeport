//! The Hosts page: every Host with its connection status.

use std::sync::Arc;

use iced::{
    widget::{column, text, Row},
    Alignment, Element, Task,
};

use homeport_hosts::{registry::HostStatus, service::HostView};

use crate::{
    app::{events::Event, AppState},
    i18n, t,
};

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
                    .map_err(|e| i18n::errors::message(&e))
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
        let direction = i18n::direction();
        let mut rows = column![text(t!("hosts.title")).size(24)]
            .spacing(8)
            .align_x(direction.pick(Alignment::Start, Alignment::End));
        if let Some(e) = &self.error {
            rows = rows.push(text(e.as_str()));
        }
        if self.list.is_empty() && self.error.is_none() {
            rows = rows.push(text(t!("hosts.empty")));
        }
        for view in &self.list {
            // Name first, then status, from the side the language starts on.
            let cells = direction.ordered(vec![
                text(view.host.name.as_str()).into(),
                text(label(&view.status)).into(),
            ]);
            rows = rows.push(Row::with_children(cells).spacing(16));
        }
        rows.into()
    }
}

fn label(status: &HostStatus) -> String {
    match status {
        HostStatus::Disabled => t!("core.status.disabled"),
        HostStatus::Connecting => t!("core.status.connecting"),
        HostStatus::Connected { .. } => t!("core.status.connected"),
        HostStatus::HostKeyUntrusted { .. } => t!("core.status.hostKeyUntrusted"),
        _ => t!("core.status.unreachable"),
    }
}

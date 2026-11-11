//! The Hosts page: every Host with its connection status.

use std::sync::Arc;

use iced::{
    widget::{column, text, Row},
    Alignment, Element, Length, Task,
};

use homeport_core::error::AppError;
use homeport_hosts::{registry::HostStatus, service::HostView};

use crate::{
    app::{events::Event, AppState},
    i18n, t,
};

#[derive(Default)]
pub struct Hosts {
    list: Vec<HostView>,
    /// Kept as the error, so it reads in the language of the moment.
    error: Option<AppError>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<Arc<Vec<HostView>>, AppError>),
}

impl Hosts {
    pub fn load(&mut self, state: &Arc<AppState>) -> Task<Message> {
        let state = state.clone();
        Task::perform(
            async move { state.hosts.list().map(Arc::new) },
            Message::Loaded,
        )
    }

    pub fn update(&mut self, _state: &Arc<AppState>, message: Message) -> Task<Message> {
        self.apply(message);
        Task::none()
    }

    /// What a message changes; none starts anything yet.
    fn apply(&mut self, message: Message) {
        match message {
            Message::Loaded(Ok(list)) => {
                // The list is only shared with this message.
                self.list = Arc::try_unwrap(list).unwrap_or_default();
                self.error = None;
            }
            Message::Loaded(Err(e)) => self.error = Some(e),
        }
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
            .width(Length::Fill)
            .align_x(direction.pick(Alignment::Start, Alignment::End));
        if let Some(e) = &self.error {
            rows = rows.push(text(i18n::errors::message(e)));
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

#[cfg(test)]
mod tests {
    use iced::Theme;

    use homeport_core::error::ErrorCode;

    use crate::test_support::{
        fixtures::{connected, host_view},
        snapshot::assert_snapshot,
    };

    use super::*;

    fn hosts() -> Hosts {
        let mut hosts = Hosts::default();
        let list = vec![
            host_view("prod", connected()),
            host_view("staging", HostStatus::Connecting),
            host_view("backup", HostStatus::Disabled),
        ];
        hosts.apply(Message::Loaded(Ok(Arc::new(list))));
        hosts
    }

    #[test]
    fn loading_fills_the_list_and_a_failure_keeps_it() {
        let mut hosts = hosts();
        assert_eq!(hosts.list.len(), 3);
        assert_eq!(hosts.error, None);
        let locked = AppError::new(ErrorCode::DbLocked, homeport_storage::messages::DB_LOCKED);
        hosts.apply(Message::Loaded(Err(locked.clone())));
        assert_eq!(hosts.list.len(), 3);
        assert_eq!(hosts.error, Some(locked));
    }

    #[test]
    fn a_status_event_updates_its_host_only() {
        let mut hosts = hosts();
        hosts.on_event(&Event::HostStatus {
            host_id: "staging-id".to_owned(),
            status: Arc::new(connected()),
        });
        assert!(matches!(hosts.list[1].status, HostStatus::Connected { .. }));
        assert!(matches!(hosts.list[2].status, HostStatus::Disabled));
        hosts.on_event(&Event::HostsChanged);
        assert_eq!(hosts.list.len(), 3);
    }

    #[test]
    fn the_page_in_english() {
        let hosts = hosts();
        assert_snapshot("hosts-en", "en", &Theme::Light, (360.0, 160.0), || {
            hosts.view()
        });
    }

    #[test]
    fn the_page_in_arabic_is_mirrored() {
        let hosts = hosts();
        assert_snapshot("hosts-ar", "ar", &Theme::Light, (360.0, 160.0), || {
            hosts.view()
        });
    }

    #[test]
    fn the_page_with_no_host() {
        let hosts = Hosts::default();
        assert_snapshot(
            "hosts-empty-en",
            "en",
            &Theme::Light,
            (360.0, 160.0),
            || hosts.view(),
        );
    }
}

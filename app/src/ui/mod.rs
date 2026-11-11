//! The window: its state, its messages and what it draws.

mod hosts;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};

use iced::{
    futures::{stream, Stream},
    widget::{column, container, text},
    Element, Length, Subscription, Task,
};
use tokio::sync::mpsc::{self, UnboundedReceiver};

use homeport_core::error::AppError;
use homeport_settings::SYSTEM_LANGUAGE;

use crate::{
    api::app::lifecycle,
    app::{
        events::{Event, Events},
        setup, AppState,
    },
    i18n,
};

pub fn run() -> iced::Result {
    iced::application(Homeport::boot, Homeport::update, Homeport::view)
        .title("Homeport")
        .subscription(Homeport::subscription)
        // The window asks first, so the app can tidy up its Hosts.
        .exit_on_close_request(false)
        .window_size((1100.0, 720.0))
        .run()
}

/// The receiving half of the backend's events, taken once by the subscription.
static EVENTS: OnceLock<Mutex<Option<UnboundedReceiver<Event>>>> = OnceLock::new();

enum Phase {
    Starting,
    Failed(Failure),
    Ready(Arc<AppState>),
}

struct Homeport {
    phase: Phase,
    hosts: hosts::Hosts,
}

/// Why the app didn't start, kept as it is so it reads in the language of
/// the moment.
#[derive(Debug, Clone)]
pub enum Failure {
    /// The database at this path couldn't be opened.
    Database(Box<AppError>, PathBuf),
    Other(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    Started(Result<Arc<AppState>, Failure>),
    Backend(Event),
    Hosts(hosts::Message),
    /// The window was asked to close.
    CloseRequested,
    /// The Hosts were tidied up: the app may exit.
    ShutDown,
    /// Something done on the side ended; nothing to show.
    Done,
    /// The Settings value of the language changed: `system` or a tag.
    LanguageChanged(String),
}

impl Homeport {
    fn boot() -> (Self, Task<Message>) {
        // The OS's language until the Settings say otherwise, so even a
        // failed start is told in it.
        i18n::set_language(SYSTEM_LANGUAGE, &i18n::locale::preferred());
        let (sender, receiver) = mpsc::unbounded_channel();
        let _ = EVENTS.set(Mutex::new(Some(receiver)));
        let events = Events::new(sender);
        let start = Task::perform(
            async move {
                match setup::state(&events) {
                    Ok(state) => Ok(Arc::new(state)),
                    Err(setup::Failure::Database(e, path)) => {
                        let error = e.logged("failed to open the database");
                        Err(Failure::Database(Box::new(error), path))
                    }
                    Err(setup::Failure::Other(e)) => Err(Failure::Other(e.to_string())),
                }
            },
            Message::Started,
        );
        (
            Self {
                phase: Phase::Starting,
                hosts: hosts::Hosts::default(),
            },
            start,
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Started(Ok(state)) => {
                if let Ok(settings) = state.settings.get() {
                    i18n::set_language(&settings.language, &i18n::locale::preferred());
                }
                self.phase = Phase::Ready(state.clone());
                self.hosts.load(&state).map(Message::Hosts)
            }
            Message::Started(Err(e)) => {
                self.phase = Phase::Failed(e);
                Task::none()
            }
            Message::Backend(event) => {
                self.hosts.on_event(&event);
                match (&self.phase, event) {
                    (Phase::Ready(state), Event::HostStatus { host_id, status }) => {
                        let state = state.clone();
                        Task::perform(
                            async move {
                                lifecycle::host_status_changed(&state, &host_id, &status).await;
                            },
                            |()| Message::Done,
                        )
                    }
                    _ => Task::none(),
                }
            }
            Message::CloseRequested => match &self.phase {
                Phase::Ready(state) => {
                    let state = state.clone();
                    Task::perform(async move { lifecycle::shutdown(&state).await }, |()| {
                        Message::ShutDown
                    })
                }
                _ => iced::exit(),
            },
            Message::ShutDown => iced::exit(),
            Message::Done => Task::none(),
            Message::LanguageChanged(language) => {
                // Every view reads the app's language, so the next draw is in it.
                i18n::set_language(&language, &i18n::locale::preferred());
                Task::none()
            }
            Message::Hosts(message) => {
                let Phase::Ready(state) = &self.phase else {
                    return Task::none();
                };
                self.hosts.update(state, message).map(Message::Hosts)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let body: Element<'_, Message> = match &self.phase {
            Phase::Starting => text(crate::t!("common.loading")).into(),
            Phase::Failed(failure) => column![
                text(crate::t!("app.startup.title")).size(20),
                text(match failure {
                    Failure::Database(error, path) =>
                        format!("{} ({})", i18n::errors::message(error), path.display()),
                    Failure::Other(why) => why.clone(),
                })
            ]
            .spacing(8)
            .into(),
            Phase::Ready(_) => self.hosts.view().map(Message::Hosts),
        };
        container(column![body].spacing(12))
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            Subscription::run(backend_events).map(Message::Backend),
            iced::window::close_requests().map(|_| Message::CloseRequested),
        ])
    }
}

fn backend_events() -> impl Stream<Item = Event> {
    let receiver = EVENTS.get().and_then(|slot| slot.lock().ok()?.take());
    stream::unfold(receiver, |receiver| async move {
        let mut receiver = receiver?;
        let event = receiver.recv().await?;
        Some((event, Some(receiver)))
    })
}

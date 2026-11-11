//! The window: its state, its messages and what it draws.

mod hosts;
pub mod theme;
pub mod widgets;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};

use iced::{
    futures::{stream, Stream},
    widget::{column, container, text},
    Length, Subscription, Task,
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
    data::{self, Data},
    i18n,
};
use theme::{Element, Theme};

pub fn run() -> iced::Result {
    let app = iced::application(Homeport::boot, Homeport::update, Homeport::view)
        .title("Homeport")
        .subscription(Homeport::subscription)
        // The window asks first, so the app can tidy up its Hosts.
        .exit_on_close_request(false)
        .theme(Homeport::theme)
        .scale_factor(|app| app.scale.factor())
        .default_font(theme::font::DEFAULT)
        .window_size((1100.0, 720.0));
    theme::font::files()
        .fold(app, |app, font| app.font(font))
        .run()
}

/// Set by the smoke test: the app closes itself as soon as it has started,
/// the way a user closing the window would.
pub const EXIT_WHEN_READY: &str = "HOMEPORT_EXIT_WHEN_READY";

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
    /// Everything pages read from Hosts and Clusters, cached.
    data: Data,
    /// The Settings value of the theme; `System` until they are read.
    mode: theme::Mode,
    scale: theme::Scale,
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
    /// A read of the data layer ended, or it is time for one.
    Data(data::Message),
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
                data: Data::default(),
                mode: theme::Mode::default(),
                scale: theme::Scale::default(),
            },
            start,
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Started(Ok(state)) => {
                if let Ok(settings) = state.settings.get() {
                    i18n::set_language(&settings.language, &i18n::locale::preferred());
                    self.mode = settings.theme.into();
                }
                self.phase = Phase::Ready(state.clone());
                if std::env::var_os(EXIT_WHEN_READY).is_some() {
                    return Task::done(Message::CloseRequested);
                }
                self.hosts.load(&state).map(Message::Hosts)
            }
            Message::Started(Err(e)) => {
                self.phase = Phase::Failed(e);
                Task::none()
            }
            Message::Backend(event) => {
                self.hosts.on_event(&event);
                let Phase::Ready(state) = &self.phase else {
                    return Task::none();
                };
                // What is cached follows what the backend just told.
                let refresh = self.data.on_event(state, &event).map(Message::Data);
                let Event::HostStatus { host_id, status } = event else {
                    return refresh;
                };
                let state = state.clone();
                let tidy = Task::perform(
                    async move {
                        lifecycle::host_status_changed(&state, &host_id, &status).await;
                    },
                    |()| Message::Done,
                );
                Task::batch([refresh, tidy])
            }
            Message::Data(message) => {
                let Phase::Ready(state) = &self.phase else {
                    return Task::none();
                };
                self.data.update(state, message).map(Message::Data)
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

    /// `None` while the user follows the OS: iced then tracks it, live.
    fn theme(&self) -> Option<Theme> {
        self.mode.theme()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            Subscription::run(backend_events).map(Message::Backend),
            iced::window::close_requests().map(|_| Message::CloseRequested),
            self.data.subscription().map(Message::Data),
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

//! The window: its state, its messages and what it draws.

mod hosts;
#[cfg(test)]
mod tests;
pub mod theme;
pub mod widgets;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};

use iced::{
    futures::{stream, Stream},
    widget::{container, text},
    Length, Subscription, Task,
};
use tokio::sync::mpsc::{self, UnboundedReceiver};

use homeport_core::{
    error::{AppError, ErrorCode},
    messages as msg,
};
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
use theme::{
    font,
    metrics::{space, text as size},
    Element, Theme,
};
use widgets::{
    feedback::{
        copy, error_details, error_message, error_message::Event as FailureEvent, toast, About,
        Details, Surfaced, Toasts,
    },
    focus, overlay, paragraph, stack,
};

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
    /// What the OS prefers, for the parts that draw their own colours (the
    /// terminal, a chart) while the user follows it: see [`Self::shown`].
    system: iced::theme::Mode,
    scale: theme::Scale,
    /// What happened, said in a corner for a few seconds.
    toasts: Toasts,
    /// The Details of a failure, when asked for.
    details: Option<Details>,
    about: About,
}

/// Why the app didn't start, kept as it is so it reads in the language of
/// the moment.
#[derive(Debug, Clone)]
pub enum Failure {
    /// The database at this path couldn't be opened.
    Database(Box<AppError>, PathBuf),
    /// Anything else, with what was reported as its detail.
    Other(Box<AppError>),
}

impl Failure {
    fn error(&self) -> &AppError {
        match self {
            Self::Database(error, _) | Self::Other(error) => error,
        }
    }
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
    /// The Settings value of the theme changed.
    ThemeChanged(homeport_settings::Theme),
    /// What the OS prefers, now and each time it changes.
    SystemTheme(iced::theme::Mode),
    /// Tab, or Shift+Tab (`true`), that no dialog or panel kept.
    Tab(bool),
    Toast(toast::Message),
    Details(error_details::Message),
    /// What was asked of the failure the window shows in place.
    Failure(FailureEvent),
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
                    Err(setup::Failure::Other(e)) => {
                        let error = AppError::new(ErrorCode::Io, msg::IO_FAILED)
                            .with_detail(e)
                            .logged("failed to start");
                        Err(Failure::Other(Box::new(error)))
                    }
                }
            },
            Message::Started,
        );
        let system = theme::mode::system().map(Message::SystemTheme);
        (Self::new(), Task::batch([start, system]))
    }

    fn new() -> Self {
        let app = crate::api::app::get_app_info();
        Self {
            phase: Phase::Starting,
            hosts: hosts::Hosts::default(),
            data: Data::default(),
            mode: theme::Mode::default(),
            system: iced::theme::Mode::None,
            scale: theme::Scale::default(),
            toasts: Toasts::default(),
            details: None,
            about: About {
                name: app.name,
                version: app.version,
                os: app.os.to_owned(),
                arch: app.arch.to_owned(),
            },
        }
    }

    /// The theme on screen, whoever chose it: for what draws its own
    /// colours and cannot ask iced.
    fn shown(&self) -> Theme {
        self.mode.resolve(self.system)
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
            Message::ThemeChanged(theme) => {
                // `theme()` reads it at the next draw.
                self.mode = theme.into();
                Task::none()
            }
            Message::SystemTheme(system) => {
                self.system = system;
                Task::none()
            }
            Message::Tab(back) => focus::traverse(back),
            Message::Toast(message) => {
                if let Some(surfaced) = self.toasts.update(message) {
                    self.details = Some(Details::new(surfaced));
                }
                Task::none()
            }
            Message::Failure(FailureEvent::Details) => {
                if let Phase::Failed(failure) = &self.phase {
                    // Logged where it happened.
                    let surfaced =
                        Surfaced::at(failure.error().clone(), None, jiff::Timestamp::now());
                    self.details = Some(Details::new(surfaced));
                }
                Task::none()
            }
            Message::Failure(FailureEvent::Copy(command)) => iced::clipboard::write(command),
            // Starting again is starting the app again.
            Message::Failure(FailureEvent::Retry) => Task::none(),
            Message::Details(message) => self.details_asked(message),
            Message::Hosts(message) => {
                let Phase::Ready(state) = &self.phase else {
                    return Task::none();
                };
                self.hosts.update(state, message).map(Message::Hosts)
            }
        }
    }

    fn details_asked(&mut self, message: error_details::Message) -> Task<Message> {
        let Some(details) = &mut self.details else {
            return Task::none();
        };
        match details.update(message, Some(&self.about)) {
            error_details::Effect::None => Task::none(),
            error_details::Effect::Close => {
                self.details = None;
                Task::none()
            }
            error_details::Effect::Copy(report) => {
                copy(report, Message::Details(error_details::Message::CopyShown))
            }
            error_details::Effect::OpenLogFolder => {
                // The log folder and what opens a folder come with the
                // platform layer (#253, #254); until then the use case says
                // it cannot, in the dialog.
                let logs = dirs::data_dir().unwrap_or_default().join("homeport/logs");
                let opened = crate::api::app::open_log_folder(&logs, |_| {
                    Err(std::io::Error::from(std::io::ErrorKind::Unsupported))
                });
                Task::done(Message::Details(error_details::Message::LogFolderOpened(
                    opened,
                )))
            }
        }
    }

    /// The page, the Details dialog over it and the toasts over both.
    fn view(&self) -> Element<'_, Message> {
        let body: Element<'_, Message> = match &self.phase {
            Phase::Starting => text(crate::t!("common.loading")).size(size::SM).into(),
            Phase::Failed(failure) => Self::failed(failure),
            Phase::Ready(_) => self.hosts.view().map(Message::Hosts),
        };
        let page = container(body)
            .padding(space::S4)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::container::page);
        let dialog = self
            .details
            .as_ref()
            .map(|details| details.view(Some(&self.about)).map(Message::Details));
        toast::over(
            overlay::over(page, dialog),
            self.toasts.view().map(|toasts| toasts.map(Message::Toast)),
        )
    }

    /// Why the app didn't start: what it is, where the database is when it
    /// is the database, and the failure with its hint and its Details.
    fn failed(failure: &Failure) -> Element<'_, Message> {
        let mut parts: Vec<Element<'_, Message>> = vec![text(crate::t!("app.startup.title"))
            .size(size::XL)
            .font(font::semibold())
            .into()];
        if let Failure::Database(_, path) = failure {
            parts.push(paragraph(crate::t!("app.startup.description")).into());
            parts.push(
                text(path.display().to_string())
                    .size(size::SM)
                    .font(font::MONO)
                    .into(),
            );
        }
        parts.push(error_message(failure.error(), false, Message::Failure));
        stack(parts).spacing(space::S3).into()
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
            focus::tab_presses().map(Message::Tab),
            self.toasts.subscription().map(Message::Toast),
            theme::mode::system_changes().map(Message::SystemTheme),
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

//! The window: its state, its messages and what it draws.

pub mod domain;
pub mod features;
pub mod shell;
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
    platform::Platform,
};
use theme::{
    font,
    icon::Icon,
    metrics::{space, text as size},
    Element, Theme,
};
use widgets::{
    basic::button,
    feedback::{
        copy, error_details, error_message, error_message::Event as FailureEvent, toast, About,
        Details, Surfaced, Toasts,
    },
    focus, overlay, paragraph, stack,
};

pub fn run() -> iced::Result {
    // The one place the app asks for the OS's own dialogs and handlers.
    let platform = Platform::native();
    let boot = move || Homeport::boot(platform.clone());
    let app = iced::application(boot, Homeport::update, Homeport::view)
        .title("Homeport")
        .subscription(Homeport::subscription)
        // The window asks first, so the app can tidy up its Hosts.
        .exit_on_close_request(false)
        .theme(Homeport::theme)
        .scale_factor(|app| app.scale.factor())
        .default_font(theme::font::DEFAULT)
        .window(window());
    theme::font::files()
        .fold(app, |app, font| app.font(font))
        .run()
}

/// The window as it opens: 1100×720, never under 800×520, with the app's
/// icon. Where it was and how big, the tray and what closing does are the
/// platform layer's (#252).
fn window() -> iced::window::Settings {
    iced::window::Settings {
        size: iced::Size::new(1100.0, 720.0),
        min_size: Some(iced::Size::new(800.0, 520.0)),
        icon: iced::window::icon::from_file_data(include_bytes!("../../assets/icon.png"), None)
            .ok(),
        ..iced::window::Settings::default()
    }
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

/// The shell over the app's features.
type Shell = shell::Shell<features::Page>;

// `Data::set_hidden` is for the platform layer (#252) to call when the
// window goes to the tray and comes back: nothing here hides it yet.
struct Homeport {
    phase: Phase,
    /// What shows once the app has started.
    shell: Option<Shell>,
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
    /// The system the app runs on: its dialogs, its browser, its file
    /// manager. The shell lends it to every page.
    platform: Platform,
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
    Shell(shell::Message<features::Message>),
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
    /// Show the folder the database is in, from the failed start.
    OpenDataFolder,
    /// The OS was asked to open a link or a folder, under this use
    /// case's name: only a failure is told.
    Opened(&'static str, Result<(), AppError>),
}

impl Homeport {
    fn boot(platform: Platform) -> (Self, Task<Message>) {
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
        (Self::new(platform), Task::batch([start, system]))
    }

    /// A window on `platform`: the OS's for the app, a fake for a test.
    fn new(platform: Platform) -> Self {
        let app = crate::api::app::get_app_info();
        Self {
            phase: Phase::Starting,
            shell: None,
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
            platform,
        }
    }

    /// The theme on screen, whoever chose it: for what draws its own
    /// colours and cannot ask iced.
    fn shown(&self) -> Theme {
        self.mode.resolve(self.system)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        let task = self.handle(message);
        // What a page made next knows the theme on screen, which a
        // message may just have changed.
        let shown = self.shown();
        if let Some(shell) = &mut self.shell {
            shell.show_theme(shown);
        }
        task
    }

    fn handle(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Started(Ok(state)) => {
                let settings = state.settings.get().unwrap_or_default();
                i18n::set_language(&settings.language, &i18n::locale::preferred());
                self.mode = settings.theme.into();
                self.phase = Phase::Ready(state.clone());
                self.shell = Some(Shell::new(state, settings, self.platform.clone()));
                if std::env::var_os(EXIT_WHEN_READY).is_some() {
                    return Task::done(Message::CloseRequested);
                }
                // The shell starts reading the Hosts and the Clusters.
                self.hand_shell(shell::Message::Changed)
            }
            Message::Started(Err(e)) => {
                self.phase = Phase::Failed(e);
                Task::none()
            }
            Message::Backend(event) => {
                let Phase::Ready(state) = &self.phase else {
                    return Task::none();
                };
                let state = state.clone();
                // What is cached follows what the backend just told, and
                // the shell follows what is cached.
                let refresh = Task::batch([
                    self.data.on_event(&state, &event).map(Message::Data),
                    self.hand_shell(shell::Message::Changed),
                ]);
                let Event::HostStatus { host_id, status } = event else {
                    return refresh;
                };
                let tidy = Task::perform(
                    async move {
                        lifecycle::host_status_changed(&state, &host_id, &status).await;
                    },
                    |()| Message::Done,
                );
                Task::batch([refresh, tidy])
            }
            Message::Data(message) => self.hand_shell(shell::Message::Data(message)),
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
                match self.toasts.update(message) {
                    Some(toast::Pressed::Details(surfaced)) => {
                        self.details = Some(Details::new(surfaced));
                    }
                    // A toast's button goes where its action said.
                    Some(toast::Pressed::Action(then)) => {
                        if let Some(request) = then.get::<shell::router::Request>() {
                            return self.hand_shell(shell::Message::Go(request.clone()));
                        }
                    }
                    None => {}
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
            Message::Shell(message) => self.hand_shell(message),
            Message::OpenDataFolder => {
                // The app has no state when its database didn't open:
                // the folder is the one the file was looked for in.
                let Phase::Failed(Failure::Database(_, database)) = &self.phase else {
                    return Task::none();
                };
                let Some(folder) = database.parent() else {
                    return Task::none();
                };
                Task::perform(self.platform.open_folder(folder.to_owned()), |opened| {
                    Message::Opened("open_data_folder", opened)
                })
            }
            Message::Opened(command, opened) => {
                if let Err(error) = opened {
                    self.toasts.failure(Surfaced::new(error, command));
                }
                Task::none()
            }
        }
    }

    /// Hands the shell a message, then does what the shell asks of the
    /// window: a toast, the Details dialog, the theme, the clipboard.
    fn hand_shell(&mut self, message: shell::Message<features::Message>) -> Task<Message> {
        let Some(shell) = &mut self.shell else {
            return Task::none();
        };
        let mut tasks = vec![shell.update(message, &mut self.data).map(Message::Shell)];
        for event in shell.take_events() {
            tasks.push(self.asked(event));
        }
        Task::batch(tasks)
    }

    /// Does what the shell asked of the window.
    fn asked(&mut self, event: shell::Event) -> Task<Message> {
        match event {
            shell::Event::Failed(Some(title), surfaced) => {
                self.toasts.failure_of(move || title.now(), surfaced);
            }
            shell::Event::Said(kind, said) => {
                self.toasts.say(kind, move || said.now());
            }
            shell::Event::SaidGoing(kind, said, label, request) => {
                self.toasts
                    .say_with(kind, said, label, toast::Then::new(request));
            }
            shell::Event::Failed(None, surfaced) => {
                self.toasts.failure(surfaced);
            }
            shell::Event::Details(surfaced) => self.details = Some(Details::new(surfaced)),
            shell::Event::Theme(theme) => return self.set_theme(theme),
            shell::Event::Copy(text) => return iced::clipboard::write(text),
            // The browser is the OS's to start, off the window's thread.
            shell::Event::OpenUrl(url) => {
                return Task::perform(self.platform.open_url(url), |opened| {
                    Message::Opened("open_url", opened)
                });
            }
            // The updater comes with its own ticket (#255): until then
            // the request is only logged.
            shell::Event::CheckForUpdates => log::info!("asked to check for updates"),
        }
        Task::none()
    }

    /// The theme chosen in the palette: shown at once, saved behind.
    fn set_theme(&mut self, theme: homeport_settings::Theme) -> Task<Message> {
        self.mode = theme.into();
        let Phase::Ready(state) = &self.phase else {
            return Task::none();
        };
        let state = state.clone();
        Task::perform(
            async move {
                let saved = crate::api::app::settings::patch_settings(&state, |stored| {
                    stored.theme = theme;
                });
                if let Err(error) = saved {
                    drop(error.logged("failed to save the theme"));
                }
            },
            |()| Message::Done,
        )
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
            // The dialog says a failure itself, under its button.
            error_details::Effect::OpenLogFolder => {
                Task::perform(self.platform.open_log_folder(), |opened| {
                    Message::Details(error_details::Message::LogFolderOpened(opened))
                })
            }
        }
    }

    /// The page, the Details dialog over it and the toasts over both.
    fn view(&self) -> Element<'_, Message> {
        fn padded(body: Element<'_, Message>) -> Element<'_, Message> {
            container(body)
                .padding(space::S4)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::container::page)
                .into()
        }
        let page: Element<'_, Message> = match (&self.phase, &self.shell) {
            (Phase::Failed(failure), _) => padded(Self::failed(failure)),
            (Phase::Ready(_), Some(shell)) => shell.view(&self.data).map(Message::Shell),
            _ => padded(text(crate::t!("common.loading")).size(size::SM).into()),
        };
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
        if matches!(failure, Failure::Database(..)) {
            let open = button::labelled(Icon::FolderOpen, crate::t!("app.startup.openFolder"))
                .on_press(Message::OpenDataFolder);
            parts.push(open.into());
        }
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
            self.shell
                .as_ref()
                .map_or_else(Subscription::none, |shell| {
                    shell.subscription(&self.data).map(Message::Shell)
                }),
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

//! The Exec tab (`ExecTab`, `startExec`): terminals into the running
//! container, several at once, a tab each. The first is a shell; "New
//! terminal" opens another with a command, a user and a working folder of
//! its own. A terminal closed is its session closed; a terminal whose
//! program exited says how and stays until it is closed.
//!
//! Each terminal is a [`TerminalSession`] on the Engine's exec
//! (`data::streams::session`), followed while the tab shows and the
//! container runs: leaving the tab, or the container stopping, lets go of
//! every session, and the tab starts over when it shows again, as React's
//! tab was unmounted and mounted.

use std::sync::Arc;

use iced::{Subscription, Task};

use homeport_engine::containers::exec::ExecOptions;

use crate::{
    app::AppState,
    data::streams::{
        session::{self, Backend, Exit, SessionUpdate, TerminalSession, Written},
        Update,
    },
    t,
    ui::{
        shell::Context,
        widgets::{
            feedback::{error_message, Surfaced},
            terminal,
        },
    },
};

/// A field of "New terminal".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Draft {
    Command,
    User,
    Directory,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// What the session of this terminal said.
    Session(u32, SessionUpdate),
    /// What its writer ended with.
    Written(u32, Written),
    Terminal(u32, terminal::Message),
    /// Its tab was pressed.
    Show(u32),
    /// Its ×.
    Close(u32),
    /// "New terminal": its popover opens or closes.
    New,
    Typed(Draft, String),
    /// "Open" in the popover.
    Open,
    Failure(u32, error_message::Event),
}

/// What a terminal runs, as "New terminal" was filled in.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Spec {
    pub command: String,
    pub user: String,
    pub directory: String,
}

impl Spec {
    /// The exec's options: what is empty is the default (`|| null`).
    fn options(&self) -> ExecOptions {
        let given = |text: &str| (!text.is_empty()).then(|| text.to_owned());
        ExecOptions {
            command: given(&self.command),
            user: given(&self.user),
            working_dir: given(&self.directory),
            cols: 0,
            rows: 0,
        }
    }
}

/// What a terminal's tab says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Title {
    /// The first: "Shell".
    Shell,
    /// One opened without a command: "Shell 2".
    Numbered(u32),
    /// The command it runs.
    Command(String),
}

impl Title {
    pub fn text(&self) -> String {
        match self {
            Self::Shell => t!("containers.exec.shell"),
            Self::Numbered(number) => t!("containers.exec.shellNumber", number = *number),
            Self::Command(command) => command.clone(),
        }
    }
}

/// One terminal and its session.
#[derive(Debug)]
pub struct Shell {
    pub key: u32,
    pub title: Title,
    pub spec: Spec,
    pub terminal: terminal::State,
    pub session: TerminalSession,
    /// The session broke, could not start, or a write failed.
    pub failure: Option<Surfaced>,
}

#[derive(Debug, Default)]
pub struct Exec {
    pub shells: Vec<Shell>,
    /// The terminal that shows.
    pub active: u32,
    /// "New terminal"'s popover, and what is typed in it.
    pub asking: bool,
    pub draft: Spec,
    /// The sessions were let go of (the tab was left, or the container
    /// stopped): the tab starts over when it shows a running container.
    stale: bool,
}

/// What a session's end says, in the terminal, dimmed.
pub fn exit_text(exit: &Exit) -> String {
    match exit.code {
        None => t!("containers.exec.exited"),
        Some(code) => t!("containers.exec.exitedWithCode", code = code),
    }
}

impl Exec {
    /// The tab as React's mounted: one shell.
    pub fn new(host_id: &str, container_id: &str, dark: bool) -> Self {
        let mut exec = Self::default();
        let shell = Shell::open(
            1,
            Title::Shell,
            Spec::default(),
            host_id,
            container_id,
            dark,
        );
        exec.shells.push(shell);
        exec.active = 1;
        exec
    }

    /// No terminal yet: they are made when the tab shows a running
    /// container.
    pub fn left() -> Self {
        Self {
            stale: true,
            ..Self::default()
        }
    }

    /// The sessions are let go of; the tab starts over next time.
    pub fn let_go(&mut self) {
        self.stale = true;
    }

    pub fn is_stale(&self) -> bool {
        self.stale
    }

    fn shell(&mut self, key: u32) -> Option<&mut Shell> {
        self.shells.iter_mut().find(|shell| shell.key == key)
    }

    /// The sessions of every terminal, while the tab shows a running
    /// container.
    pub fn subscription(&self, state: &Arc<AppState>) -> Subscription<Message> {
        Subscription::batch(self.shells.iter().map(|shell| {
            let key = shell.key;
            shell
                .session
                .subscription(state)
                .with(key)
                .map(|(key, said)| Message::Session(key, said))
        }))
    }

    pub fn update(
        &mut self,
        message: Message,
        target: (&str, &str),
        dark: bool,
        ctx: &Context<'_>,
    ) -> Task<Message> {
        match message {
            Message::Session(key, said) => {
                let Some(shell) = self.shell(key) else {
                    return Task::none();
                };
                return shell.said(said, ctx);
            }
            Message::Written(key, written) => {
                if let Some(shell) = self.shell(key) {
                    // Only this writer's failure: one the user closed stays so.
                    if let Some(error) = shell.session.written(written) {
                        shell.failure = Some(Surfaced::new(error.clone(), "exec_input"));
                    }
                }
            }
            Message::Terminal(key, message) => {
                let Some(shell) = self.shell(key) else {
                    return Task::none();
                };
                let (task, events) = shell.terminal.update(message);
                shell.told(events, ctx);
                return task.map(move |message| Message::Terminal(key, message));
            }
            Message::Show(key) => self.active = key,
            Message::Close(key) => {
                // Dropped, its session is no longer followed: it closes.
                self.shells.retain(|shell| shell.key != key);
                if self.active == key {
                    self.active = self.shells.last().map_or(0, |shell| shell.key);
                }
            }
            Message::New => self.asking = !self.asking,
            Message::Typed(field, text) => match field {
                Draft::Command => self.draft.command = text,
                Draft::User => self.draft.user = text,
                Draft::Directory => self.draft.directory = text,
            },
            Message::Open => {
                let key = self.shells.iter().map(|shell| shell.key).max().unwrap_or(0) + 1;
                let spec = self.draft.clone();
                let title = if spec.command.is_empty() {
                    Title::Numbered(key)
                } else {
                    Title::Command(spec.command.clone())
                };
                let (host, container) = target;
                let shell = Shell::open(key, title, spec, host, container, dark);
                self.shells.push(shell);
                self.active = key;
                self.asking = false;
            }
            Message::Failure(key, event) => {
                let Some(shell) = self.shell(key) else {
                    return Task::none();
                };
                match event {
                    error_message::Event::Details => {
                        if let Some(failure) = &shell.failure {
                            ctx.details(failure.clone());
                        }
                    }
                    error_message::Event::Copy(command) => ctx.copy(command),
                    error_message::Event::Retry => {}
                }
            }
        }
        Task::none()
    }
}

impl Shell {
    fn open(key: u32, title: Title, spec: Spec, host: &str, container: &str, dark: bool) -> Self {
        // Told the theme now, so a program that asks for the colours
        // before the first draw is answered in them.
        let terminal = terminal::State::new().dark(dark);
        let backend = Backend::Engine {
            host_id: host.to_owned(),
            container_id: container.to_owned(),
            options: spec.options(),
        };
        Self {
            key,
            title,
            session: TerminalSession::new(backend, terminal.size()),
            terminal,
            spec,
            failure: None,
        }
    }

    /// What its session said.
    fn said(&mut self, said: SessionUpdate, ctx: &Context<'_>) -> Task<Message> {
        // What a session before this one said late is let go of.
        let Some(update) = self.session.take(said) else {
            return Task::none();
        };
        let key = self.key;
        match update {
            Update::Started(id) => {
                let writes = self.session.started(ctx.state.clone(), id);
                // The terminal takes the keyboard once it opened (`onOpen`).
                let focus = iced::widget::operation::focus(self.terminal.id());
                return Task::batch([
                    Task::perform(writes, move |written| Message::Written(key, written)),
                    focus,
                ]);
            }
            Update::Batch { items, .. } => {
                let output = session::output(items);
                let events = self.terminal.feed(&output.text);
                if let Some(exit) = &output.exit {
                    self.session.ended();
                    self.terminal.notice(&exit_text(exit));
                    if let Some(error) = &exit.error {
                        self.failure = Some(Surfaced::new(error.clone(), "exec_start"));
                    }
                }
                if let Some(error) = output.error {
                    self.failure = Some(Surfaced::new(error, "exec_start"));
                }
                self.told(events, ctx);
            }
            Update::Failed(error) => {
                self.session.ended();
                self.failure = Some(Surfaced::new(error, "exec_start"));
            }
            Update::Ended => self.session.ended(),
        }
        Task::none()
    }

    /// What the terminal says goes to the session, in order.
    fn told(&mut self, events: Vec<terminal::Event>, ctx: &Context<'_>) {
        for event in events {
            match event {
                terminal::Event::Input(bytes) => self.session.input(bytes),
                terminal::Event::Resize { cols, rows } => self.session.resize(cols, rows),
                terminal::Event::Copy(text) => ctx.copy(text),
                terminal::Event::Open(url) => ctx.open_url(url),
                terminal::Event::Title(_) | terminal::Event::Bell => {}
            }
        }
    }
}

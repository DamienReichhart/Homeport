//! The dialogs of a container's menu that are forms (`ContainerDialogs`):
//! Duplicate (`DuplicateDialog`), Edit (`EditDialog`, [`edit`]) and
//! Commit (`CommitDialog`, [`commit`]). Duplicate and Edit first read the
//! container's settings (`containers_run_spec`) in a dialog of their own
//! (`LoadingDialog`): a spinner, or the failure with Retry.
//!
//! The settings and Edit's losses are read by the dialog itself, by
//! `Task::perform`: React read them with `staleTime: 0` and `gcTime: 0`,
//! once a dialog opened, and kept nothing; a read answered after its
//! dialog closed, or after another opened, is let go of by its number.
//!
//! # What differs from the React dialogs, on purpose
//!
//! - Edit's confirmation takes the place of its form while it shows,
//!   which comes back as it was when it is cancelled; React laid it over
//!   the form.
//! - After Edit, the new container's page is opened as a new step of the
//!   history; React replaced the old one's (`replace: true`), whose page
//!   no longer exists. Back from a container's page goes to the list.
//! - A loss's path and Stack are laid out left to right in every language
//!   (`technical`).

pub mod commit;
pub mod edit;

#[cfg(all(test, unix))]
mod tests;

use iced::Task;

use homeport_core::error::AppResult;
use homeport_engine::containers::{run::RunSpec, Container};

use super::Target;
use crate::{
    api::engine::containers::containers_run_spec,
    data::keys,
    t,
    ui::{
        domain::containers::run::{self, model::RunForm},
        shell::{router::Route, selection::Selection, Context},
        widgets::{
            basic::button::{self, Variant},
            feedback::{error_message, spinner, Surfaced},
            overlay::{dialog, modal, Modal},
        },
    },
};

/// What a container's settings are read for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum For {
    Duplicate,
    Edit,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// The settings of the read of this number.
    Spec(u64, Box<AppResult<RunSpec>>),
    /// Retry, Details or a hint's command of a read that failed.
    Loading(error_message::Event),
    Run(run::dialog::Message),
    Edit(edit::Message),
    Commit(commit::Message),
    /// Cancel, Escape or the scrim of the loading dialog.
    Close,
}

#[derive(Debug)]
enum Shown {
    /// The settings are being read, or could not be.
    Loading {
        what: For,
        target: Target,
        failure: Option<Box<Surfaced>>,
    },
    Duplicate(Box<run::Dialog>),
    Edit(Box<edit::Edit>),
    Commit(Box<commit::Commit>),
}

/// The form dialog that is open, and the number of the last read.
#[derive(Debug, Default)]
pub struct Forms {
    shown: Option<Shown>,
    number: u64,
}

/// A container `id` of the Host `host_id` was made: the window shows its
/// logs, its Host selected first on a list of every Host's containers.
pub(super) fn show_new(host_id: &str, id: String, across_hosts: bool, ctx: &Context<'_>) {
    if across_hosts {
        ctx.select(Selection::Host {
            id: host_id.to_owned(),
        });
    }
    ctx.open(Route::Container {
        id,
        tab: Some("logs".to_owned()),
    });
}

impl Forms {
    pub fn is_open(&self) -> bool {
        self.shown.is_some()
    }

    /// Reads `target`'s settings, for a copy or an edit.
    pub fn ask(&mut self, what: For, target: Target, ctx: &Context<'_>) -> Task<Message> {
        self.number += 1;
        let number = self.number;
        let (state, host, id) = (ctx.state.clone(), target.host_id.clone(), target.id.clone());
        self.shown = Some(Shown::Loading {
            what,
            target,
            failure: None,
        });
        Task::perform(
            async move { containers_run_spec(&state, host, id).await },
            move |result| Message::Spec(number, Box::new(result)),
        )
    }

    /// Commits `target` to an image.
    pub fn ask_commit(&mut self, target: Target, ctx: &Context<'_>) {
        self.shown = Some(Shown::Commit(Box::new(commit::Commit::new(target, ctx))));
    }

    pub fn update(&mut self, message: Message, across: bool, ctx: &Context<'_>) -> Task<Message> {
        match message {
            Message::Spec(number, result) if number == self.number => self.read(*result, ctx),
            Message::Spec(..) => {}
            Message::Loading(event) => return self.loading(event, ctx),
            Message::Close => {
                if matches!(self.shown, Some(Shown::Loading { .. })) {
                    self.shown = None;
                }
            }
            Message::Run(message) => {
                let Some(Shown::Duplicate(dialog)) = self.shown.as_mut() else {
                    return Task::none();
                };
                let host = dialog.fields.host_id.clone();
                let (task, step) = dialog.update(message, ctx);
                match step {
                    run::Step::Stay => {}
                    run::Step::Closed => self.shown = None,
                    run::Step::Started(id) => {
                        self.shown = None;
                        show_new(&host, id, across, ctx);
                    }
                }
                return task.map(Message::Run);
            }
            Message::Edit(message) => {
                let Some(Shown::Edit(edit)) = self.shown.as_mut() else {
                    return Task::none();
                };
                let (task, step) = edit.update(message, ctx);
                match step {
                    edit::Step::Stay => {}
                    edit::Step::Closed => self.shown = None,
                    edit::Step::CommitFirst(target) => self.ask_commit(target, ctx),
                    edit::Step::Recreated(host, id) => {
                        self.shown = None;
                        show_new(&host, id, across, ctx);
                    }
                }
                return task.map(Message::Edit);
            }
            Message::Commit(message) => {
                let Some(Shown::Commit(commit)) = self.shown.as_mut() else {
                    return Task::none();
                };
                let (task, closes) = commit.update(message, ctx);
                if closes {
                    self.shown = None;
                }
                return task.map(Message::Commit);
            }
        }
        Task::none()
    }

    /// The settings came: the form shows them, or the dialog says why it
    /// could not read them.
    fn read(&mut self, result: AppResult<RunSpec>, ctx: &Context<'_>) {
        let Some(Shown::Loading {
            what,
            target,
            failure,
        }) = self.shown.as_mut()
        else {
            return;
        };
        let spec = match result {
            Ok(spec) => spec,
            Err(error) => {
                *failure = Some(Box::new(Surfaced::new(error, "containers_run_spec")));
                return;
            }
        };
        let target = target.clone();
        let original = RunForm::of_spec(&spec);
        self.shown = Some(match what {
            For::Duplicate => {
                let containers = ctx
                    .data
                    .read::<Vec<Container>>(&keys::host::containers(&target.host_id));
                let taken: Vec<&str> = containers
                    .data()
                    .map(|list| list.iter().map(|c| c.name.as_str()).collect())
                    .unwrap_or_default();
                let name = run::model::copy_name(&target.name, &taken);
                let dialog =
                    run::Dialog::duplicate(&target.host_id, &target.name, original, name, ctx);
                Shown::Duplicate(Box::new(dialog))
            }
            For::Edit => Shown::Edit(Box::new(edit::Edit::new(target, original, ctx))),
        });
    }

    fn loading(&mut self, event: error_message::Event, ctx: &Context<'_>) -> Task<Message> {
        let Some(Shown::Loading {
            what,
            target,
            failure,
        }) = self.shown.as_ref()
        else {
            return Task::none();
        };
        match event {
            error_message::Event::Retry => {
                let (what, target) = (*what, target.clone());
                return self.ask(what, target, ctx);
            }
            error_message::Event::Details => {
                if let Some(failure) = failure {
                    ctx.details((**failure).clone());
                }
            }
            error_message::Event::Copy(command) => ctx.copy(command),
        }
        Task::none()
    }

    pub fn dialog<'a>(&'a self, ctx: &Context<'a>) -> Option<Modal<'a, Message>> {
        match self.shown.as_ref()? {
            Shown::Loading {
                what,
                target,
                failure,
            } => {
                let name = target.name.as_str();
                let title = match what {
                    For::Duplicate => t!("containers.duplicate.title", name = name),
                    For::Edit => t!("containers.edit.title", name = name),
                };
                let body = match failure {
                    Some(failure) => error_message(&failure.error, true, Message::Loading),
                    None => spinner(),
                };
                let cancel = button::text(t!("common.cancel"))
                    .variant(Variant::Ghost)
                    .on_press(Message::Close);
                let content = dialog(title).push(body).footer(vec![cancel.into()]);
                Some(
                    modal(content)
                        .key("loading")
                        .on_close(Message::Close)
                        .close_on_scrim(),
                )
            }
            Shown::Duplicate(dialog) => Some(dialog.view(ctx).map(Message::Run)),
            Shown::Edit(edit) => Some(edit.view(ctx).map(Message::Edit)),
            Shown::Commit(commit) => Some(commit.view(ctx).map(Message::Commit)),
        }
    }
}

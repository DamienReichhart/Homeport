//! Edit (`EditDialog`, `EditConfirmation`, `losses.ts`): the Run
//! dialog's fields filled in from a container, its name kept, applied by
//! recreating it. Before it is, a confirmation says what the new container
//! won't have (`containers_edit_losses`): an anonymous volume, files
//! changed inside it (with "Commit to an image first…"), the time it is
//! down, a Stack's next deploy. It is red when something other than the
//! downtime is lost.
//!
//! The recreate goes through `ctx.act`: "Container recreated" and the list
//! read again are the shell's. The confirmation closes when it ends; a
//! failure is said in the form, which stays for another try. While it
//! runs, neither dialog can be closed.

use iced::{widget::container, Length, Task};

use homeport_core::error::AppResult;
use homeport_engine::containers::recreate::Loss;

use super::super::Target;
use crate::{
    api::engine::containers::{containers_edit_losses, containers_recreate},
    data::keys,
    t,
    ui::{
        domain::containers::run::{self, dialog::Presence, model::RunForm},
        shell::{Context, Outcome},
        theme::{metrics::space, Element},
        widgets::{
            basic::{
                button::{self, Variant},
                scroll_area, technical,
            },
            feedback::{
                confirm::{self, Ask, Outcome as Confirmed},
                confirm_dialog_with, error_message, spinner, Surfaced,
            },
            filling,
            overlay::{dialog, modal, modal::WIDTH_WIDE, Modal},
            paragraph, stack,
        },
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    Form(run::form::Message),
    /// Apply, or Enter in a field: the confirmation opens.
    Submit,
    /// Cancel, Escape or the scrim.
    Close,
    /// What the read of this number says is lost.
    Losses(u64, AppResult<Vec<Loss>>),
    /// Retry, Details or a hint's command of the losses' read.
    LossesFailure(error_message::Event),
    Confirm(confirm::Message),
    /// "Commit to an image first…".
    CommitFirst,
    /// The Engine answered the recreate.
    Recreated(AppResult<String>),
    Failure(error_message::Event),
}

/// What the actions do after a message of Edit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Stay,
    Closed,
    /// Leave Edit for Commit, to keep the files changed inside.
    CommitFirst(Target),
    /// The container was recreated as this id, on this Host.
    Recreated(String, String),
}

/// The confirmation, open: what is lost, as read.
#[derive(Debug)]
struct Confirming {
    state: confirm::State,
    number: u64,
    losses: Option<AppResult<Vec<Loss>>>,
}

#[derive(Debug)]
pub struct Edit {
    target: Target,
    fields: run::Form,
    /// The Engine is recreating it.
    pending: bool,
    failure: Option<Surfaced>,
    confirming: Option<Confirming>,
    presence: Presence,
}

/// One line of the confirmation per thing the new container won't have.
pub fn loss_text(loss: &Loss) -> String {
    match loss {
        Loss::AnonymousVolume { path } => {
            t!("containers.loss.anonymousVolume", path = technical(path))
        }
        Loss::FilesystemChanges { count } => {
            t!("containers.loss.filesystemChanges", count = *count)
        }
        Loss::Downtime => t!("containers.loss.downtime"),
        Loss::StackRedeploy { stack } => {
            t!("containers.loss.stackRedeploy", stack = technical(stack))
        }
    }
}

impl Edit {
    pub fn new(target: Target, original: RunForm, ctx: &Context<'_>) -> Self {
        let form = RunForm {
            name: target.name.clone(),
            ..original
        };
        let mut fields = run::Form::new(&target.host_id, form);
        fields.lock_name = true;
        // Its lists, for the selects, and the image's login.
        fields.read(ctx);
        Self {
            target,
            fields,
            pending: false,
            failure: None,
            confirming: None,
            presence: Presence::new(),
        }
    }

    /// Closed: what its fields read is let go of, and an answer that comes
    /// after is said in a toast.
    pub fn let_go(&mut self, ctx: &Context<'_>) {
        self.presence.set(false);
        self.fields.let_go(ctx);
    }

    pub fn fields(&self) -> &run::Form {
        &self.fields
    }

    /// Whether the confirmation is red: something other than the
    /// downtime is lost.
    pub fn is_red(&self) -> bool {
        self.ask().destructive
    }

    fn ask(&self) -> Ask {
        let losses = self.confirming.as_ref().and_then(|c| c.losses.as_ref());
        let destructive =
            matches!(losses, Some(Ok(list)) if list.iter().any(|l| *l != Loss::Downtime));
        let ask = Ask::new(
            t!(
                "containers.edit.confirmTitle",
                name = self.target.name.as_str()
            ),
            String::new(),
            t!("containers.edit.recreate"),
        );
        if destructive {
            ask.destructive()
        } else {
            ask
        }
    }

    /// Apply, or Enter: the confirmation opens, once the form can be sent.
    fn apply(&mut self, ctx: &Context<'_>) -> Task<Message> {
        if !self.pending && self.confirming.is_none() {
            self.fields.add_drafts();
        }
        if !self.pending && self.confirming.is_none() && self.fields.is_ready() {
            return self.read_losses(ctx);
        }
        Task::none()
    }

    /// Reads what recreating the container loses.
    fn read_losses(&mut self, ctx: &Context<'_>) -> Task<Message> {
        // Numbered across dialogs: a closed Edit's answer is never this one's.
        let number = run::dialog::fresh();
        self.confirming = Some(Confirming {
            state: confirm::State::default(),
            number,
            losses: None,
        });
        let (state, host, id) = (
            ctx.state.clone(),
            self.target.host_id.clone(),
            self.target.id.clone(),
        );
        Task::perform(
            async move { containers_edit_losses(&state, host, id).await },
            move |result| Message::Losses(number, result),
        )
    }

    pub fn update(&mut self, message: Message, ctx: &Context<'_>) -> (Task<Message>, Step) {
        let stay = |task| (task, Step::Stay);
        match message {
            Message::Submit => return stay(self.apply(ctx)),
            Message::Form(message) => {
                let (task, submits) = self.fields.update(message, ctx);
                let task = task.map(Message::Form);
                if submits {
                    return stay(Task::batch([task, self.apply(ctx)]));
                }
                return stay(task);
            }
            Message::Close if !self.pending => return (Task::none(), Step::Closed),
            Message::Close => {}
            Message::Losses(number, result) => {
                if let Some(confirming) = self.confirming.as_mut().filter(|c| c.number == number) {
                    confirming.losses = Some(result);
                }
            }
            Message::LossesFailure(event) => return stay(self.losses_failure(event, ctx)),
            Message::Confirm(message) => return stay(self.confirm(message, ctx)),
            Message::CommitFirst if !self.pending => {
                return (Task::none(), Step::CommitFirst(self.target.clone()));
            }
            Message::CommitFirst => {}
            Message::Recreated(result) if self.pending => {
                self.pending = false;
                // The confirmation closes either way: a failure is the form's.
                self.confirming = None;
                match result {
                    Ok(id) => {
                        let host = self.target.host_id.clone();
                        return (Task::none(), Step::Recreated(host, id));
                    }
                    Err(error) => {
                        self.failure = Some(Surfaced::new(error, "containers_recreate"));
                    }
                }
            }
            Message::Recreated(_) => {}
            Message::Failure(error_message::Event::Details) => {
                if let Some(failure) = &self.failure {
                    ctx.details(failure.clone());
                }
            }
            Message::Failure(error_message::Event::Copy(command)) => ctx.copy(command),
            Message::Failure(error_message::Event::Retry) => {}
        }
        stay(Task::none())
    }

    fn losses_failure(&mut self, event: error_message::Event, ctx: &Context<'_>) -> Task<Message> {
        let failed = self
            .confirming
            .as_ref()
            .and_then(|c| c.losses.as_ref())
            .and_then(|losses| losses.as_ref().err())
            .cloned();
        match event {
            error_message::Event::Retry => return self.read_losses(ctx),
            error_message::Event::Details => {
                if let Some(error) = failed {
                    ctx.details(Surfaced::new(error, "containers_edit_losses"));
                }
            }
            error_message::Event::Copy(command) => ctx.copy(command),
        }
        Task::none()
    }

    fn confirm(&mut self, message: confirm::Message, ctx: &Context<'_>) -> Task<Message> {
        let ask = self.ask();
        let Some(confirming) = self.confirming.as_mut() else {
            return Task::none();
        };
        match confirming.state.update(message, &ask) {
            Confirmed::Pending => Task::none(),
            Confirmed::Cancelled => {
                self.confirming = None;
                Task::none()
            }
            Confirmed::Confirmed => {
                self.pending = true;
                self.failure = None;
                let spec = self.fields.form.to_spec();
                let (state, host, id) = (
                    ctx.state.clone(),
                    self.target.host_id.clone(),
                    self.target.id.clone(),
                );
                let scope = keys::host::containers(&host);
                let shown = self.presence.handle();
                ctx.act(
                    async move { containers_recreate(&state, host, id, spec).await },
                    move |result| match result {
                        Ok(_) => Outcome::quiet()
                            .saying(|| t!("containers.edit.recreated"))
                            .invalidating(scope.clone()),
                        // Said in the form, or in a toast once it is gone.
                        Err(error) if !shown.load(std::sync::atomic::Ordering::SeqCst) => {
                            Outcome::quiet()
                                .failing(Surfaced::new(error.clone(), "containers_recreate"))
                        }
                        Err(_) => Outcome::quiet(),
                    },
                    Message::Recreated,
                )
            }
        }
    }

    /// What the confirmation says: what is lost, as it is read.
    fn losses<'a>(&self, losses: Option<&AppResult<Vec<Loss>>>) -> Element<'a, Message> {
        match losses {
            None => spinner(),
            Some(Err(error)) => error_message(error, true, Message::LossesFailure),
            Some(Ok(list)) if list.is_empty() => {
                paragraph(t!("containers.edit.nothingLost")).into()
            }
            Some(Ok(list)) => {
                let mut parts: Vec<Element<'a, Message>> =
                    vec![paragraph(t!("containers.edit.keptBut")).into()];
                // `list-disc ps-5`: a bullet, a sign of no language, before
                // lines that hang under the first.
                parts.extend(list.iter().map(|loss| {
                    crate::ui::widgets::line(vec![
                        iced::widget::text("\u{2022}").into(),
                        filling(paragraph(loss_text(loss))).into(),
                    ])
                    .spacing(space::S2)
                    .width(Length::Fill)
                    .align_y(iced::alignment::Vertical::Top)
                    .into()
                }));
                if list
                    .iter()
                    .any(|l| matches!(l, Loss::FilesystemChanges { .. }))
                {
                    let commit = button::text(t!("containers.edit.commitFirst"))
                        .variant(Variant::Outline)
                        .size(crate::ui::theme::metrics::ButtonSize::Sm)
                        .on_press_maybe((!self.pending).then_some(Message::CommitFirst));
                    parts.push(commit.into());
                }
                stack(parts).spacing(space::S2).into()
            }
        }
    }

    pub fn view<'a>(&'a self, ctx: &Context<'a>) -> Modal<'a, Message> {
        if let Some(confirming) = &self.confirming {
            let more = self.losses(confirming.losses.as_ref());
            return confirm_dialog_with(
                &self.ask(),
                &confirming.state,
                Some(more),
                Message::Confirm,
            )
            .key("edit-confirm");
        }
        let lists = self.fields.lists(ctx);
        let login = self.fields.login(ctx);
        let fields = run::fields(&self.fields, &lists, login.as_ref()).map(Message::Form);
        let mut content = dialog(t!(
            "containers.edit.title",
            name = self.target.name.as_str()
        ))
        .description(t!("containers.edit.description"))
        .push(fields);
        if let Some(failure) = &self.failure {
            content = content.push(error_message(&failure.error, false, Message::Failure));
        }
        let waits = self.pending;
        let cancel = button::text(t!("common.cancel"))
            .variant(Variant::Ghost)
            .on_press_maybe((!waits).then_some(Message::Close));
        let label = if waits {
            t!("containers.edit.recreating")
        } else {
            t!("containers.edit.apply")
        };
        let ready = !waits && self.fields.is_ready();
        let apply = button::text(label).on_press_maybe(ready.then_some(Message::Submit));
        let content = content.footer(vec![cancel.into(), apply.into()]);
        let scrolled = scroll_area(container(content).width(Length::Fill));
        modal(scrolled)
            .key("edit")
            .width(WIDTH_WIDE)
            .on_close_maybe((!waits).then_some(Message::Close))
            .close_on_scrim()
    }
}

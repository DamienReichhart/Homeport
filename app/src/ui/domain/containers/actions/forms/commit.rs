//! Commit (`CommitDialog`, `commit.ts`): a container's files and settings
//! kept as a new image, named after it and the day, with a message, an
//! author, Dockerfile instructions and whether it pauses meanwhile. A name
//! another image has is said: the tag moves to the new one.
//!
//! The commit goes through `ctx.act`: "Committed to …", its "Open images"
//! button and the images read again are the shell's. A failure is said in the dialog, which
//! stays; while it runs the dialog can't be closed.

use iced::{
    widget::{text_editor, toggler},
    Length, Task,
};

use homeport_core::error::AppResult;
use homeport_engine::images::Image;

use super::super::Target;
use crate::{
    api::engine::containers::containers_commit,
    data::{keys, resources},
    t,
    ui::{
        domain::containers::run::{
            dialog::Presence,
            model::{image_named, today, CommitForm},
        },
        shell::{router::Route, Context, Outcome},
        theme::{
            self,
            metrics::{control, space, text as size},
            Element,
        },
        widgets::{
            basic::{
                button::{self, Variant},
                technical,
            },
            feedback::{alert, error_message, Surfaced},
            form::{field, input, textarea},
            line,
            overlay::{dialog, modal, Modal},
            paragraph, stack,
        },
    },
};

/// A sample of the changes: Dockerfile instructions, of no language.
const CHANGES_SAMPLE: &str = "ENV MODE=prod\nCMD [\"nginx\", \"-g\", \"daemon off;\"]";

/// `sm:max-w-lg`.
const WIDTH: f32 = 512.0;

#[derive(Debug, Clone)]
pub enum Message {
    Reference(String),
    Comment(String),
    Author(String),
    Changes(text_editor::Action),
    Pause(bool),
    Submit,
    Close,
    Committed(AppResult<String>),
    Failure(error_message::Event),
}

#[derive(Debug)]
pub struct Commit {
    target: Target,
    pub form: CommitForm,
    changes: text_editor::Content,
    pending: bool,
    failure: Option<Surfaced>,
    presence: Presence,
}

impl Commit {
    pub fn new(target: Target, ctx: &Context<'_>) -> Self {
        ctx.watch(resources::images(&target.host_id));
        Self {
            form: CommitForm::new(&target.name, &today()),
            target,
            changes: text_editor::Content::new(),
            pending: false,
            failure: None,
            presence: Presence::new(),
        }
    }

    /// Closed: the images it read are let go of, and an answer that comes
    /// after is said in a toast.
    pub fn let_go(&mut self, ctx: &Context<'_>) {
        self.presence.set(false);
        ctx.unwatch(keys::host::images(&self.target.host_id));
    }

    /// The image whose tag the commit would move, as the Host's images say.
    fn taken(&self, ctx: &Context<'_>) -> bool {
        let images = ctx
            .data
            .read::<Vec<Image>>(&keys::host::images(&self.target.host_id));
        images
            .data()
            .is_some_and(|list| image_named(&self.form.reference, list).is_some())
    }

    /// What `message` started, and whether the dialog closes.
    pub fn update(&mut self, message: Message, ctx: &Context<'_>) -> (Task<Message>, bool) {
        match message {
            Message::Reference(typed) => self.form.reference = typed,
            Message::Comment(typed) => self.form.comment = typed,
            Message::Author(typed) => self.form.author = typed,
            Message::Changes(action) => {
                let edits = action.is_edit();
                self.changes.perform(action);
                if edits {
                    self.form.changes = self.changes.text();
                }
            }
            Message::Pause(on) => self.form.pause = on,
            Message::Submit => {
                if self.pending || self.form.reference.trim().is_empty() {
                    return (Task::none(), false);
                }
                self.pending = true;
                self.failure = None;
                let spec = self.form.to_spec();
                let reference = spec.reference.clone();
                let (state, host, id) = (
                    ctx.state.clone(),
                    self.target.host_id.clone(),
                    self.target.id.clone(),
                );
                let images = keys::host::images(&host);
                let shown = self.presence.handle();
                // The toast and the images read again are the shell's.
                let task = ctx.act(
                    async move { containers_commit(&state, host, id, spec).await },
                    move |result| match result {
                        Ok(_) => {
                            let reference = reference.clone();
                            Outcome::quiet()
                                .saying(move || {
                                    t!("containers.commit.done", reference = technical(&reference))
                                })
                                .going(|| t!("containers.commit.openImages"), Route::Images)
                                .invalidating(images.clone())
                        }
                        // Said in the dialog, or in a toast once it is gone.
                        Err(error) if !shown.load(std::sync::atomic::Ordering::SeqCst) => {
                            Outcome::quiet()
                                .failing(Surfaced::new(error.clone(), "containers_commit"))
                        }
                        Err(_) => Outcome::quiet(),
                    },
                    Message::Committed,
                );
                return (task, false);
            }
            Message::Close => return (Task::none(), !self.pending),
            Message::Committed(result) if self.pending => {
                self.pending = false;
                match result {
                    Ok(_) => return (Task::none(), true),
                    Err(error) => self.failure = Some(Surfaced::new(error, "containers_commit")),
                }
            }
            Message::Committed(_) => {}
            Message::Failure(error_message::Event::Details) => {
                if let Some(failure) = &self.failure {
                    ctx.details(failure.clone());
                }
            }
            Message::Failure(error_message::Event::Copy(command)) => ctx.copy(command),
            Message::Failure(error_message::Event::Retry) => {}
        }
        (Task::none(), false)
    }

    pub fn view<'a>(&'a self, ctx: &Context<'a>) -> Modal<'a, Message> {
        let form = &self.form;
        let half = |part: Element<'a, Message>| -> Element<'a, Message> {
            iced::widget::container(part).width(Length::Fill).into()
        };
        let reference = input("", &form.reference)
            .font(theme::font::MONO)
            .on_input(Message::Reference)
            .on_submit(Message::Submit);
        let mut parts: Vec<Element<'a, Message>> =
            vec![field(t!("containers.commit.imageName"), reference)
                .help(t!("containers.commit.imageNameHint"))
                .into()];
        if self.taken(ctx) {
            let name = technical(form.reference.trim());
            parts.push(alert(
                None,
                t!("containers.commit.nameTaken", name = name),
                false,
            ));
        }
        let comment = input(&t!("containers.commit.messagePlaceholder"), &form.comment)
            .on_input(Message::Comment)
            .on_submit(Message::Submit);
        let author = input(&t!("common.optional"), &form.author)
            .on_input(Message::Author)
            .on_submit(Message::Submit);
        parts.push(
            line(vec![
                half(field(t!("containers.commit.message"), comment).into()),
                half(field(t!("containers.commit.author"), author).into()),
            ])
            .spacing(space::S3)
            .width(Length::Fill)
            .into(),
        );
        let changes = textarea(&self.changes, Message::Changes)
            .placeholder(CHANGES_SAMPLE)
            .font(theme::font::MONO)
            .size(size::XS);
        parts.push(
            field(t!("containers.commit.changes"), changes)
                .help(t!("containers.commit.changesHint"))
                .into(),
        );
        let pause = toggler(form.pause)
            .label(t!("containers.commit.pause"))
            .size(control::SWITCH_HEIGHT)
            .text_size(size::SM)
            .on_toggle(Message::Pause);
        let says = if form.pause {
            t!("containers.commit.pauseOn")
        } else {
            t!("containers.commit.pauseOff")
        };
        parts.push(
            stack(vec![
                pause.into(),
                paragraph(says)
                    .size(size::XS)
                    .style(theme::text::muted)
                    .into(),
            ])
            .spacing(space::S1)
            .into(),
        );
        if let Some(failure) = &self.failure {
            parts.push(error_message(&failure.error, false, Message::Failure));
        }
        let waits = self.pending;
        let cancel = button::text(t!("common.cancel"))
            .variant(Variant::Ghost)
            .on_press_maybe((!waits).then_some(Message::Close));
        let label = if waits {
            t!("containers.commit.committing")
        } else {
            t!("containers.commit.submit")
        };
        let ready = !waits && !form.reference.trim().is_empty();
        let commit = button::text(label).on_press_maybe(ready.then_some(Message::Submit));
        let content = dialog(t!(
            "containers.commit.title",
            name = self.target.name.as_str()
        ))
        .description(t!("containers.commit.description"))
        .push(stack(parts).spacing(space::S3))
        .footer(vec![cancel.into(), commit.into()]);
        modal(content)
            .key("commit")
            .width(WIDTH)
            .on_close_maybe((!waits).then_some(Message::Close))
            .close_on_scrim()
    }
}

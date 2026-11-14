//! The Run dialog itself (`RunContainerDialog`, and `DuplicateDialog`
//! once the original's settings were read): its fields, Cancel and Run,
//! what the Engine said when it refused, and, for a copy, the host ports
//! the original publishes too.
//!
//! Run goes through `ctx.act`: "Container started" and the list read
//! again are the shell's to say, whether or not the dialog is still open
//! when the Engine answers; the dialog then closes and the window goes
//! to the container's logs.

use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};

use iced::{widget::container, Length, Task};

use homeport_core::error::AppResult;

use super::{
    form::{self, Form},
    model::{shared_host_ports, RunForm},
    view,
};
use crate::{
    api::engine::containers::containers_run,
    data::{keys, resources},
    t,
    ui::{
        shell::{Context, Outcome},
        widgets::{
            basic::{
                button::{self, Variant},
                scroll_area,
            },
            feedback::{alert, error_message, Surfaced},
            overlay::{dialog, modal, modal::WIDTH_WIDE, Modal},
        },
    },
};

/// What the dialog was opened for.
#[derive(Debug)]
enum Kind {
    Run,
    /// A copy of the container named so, whose settings are `original`.
    Duplicate {
        name: String,
        original: Box<RunForm>,
    },
}

#[derive(Debug, Clone)]
pub enum Message {
    Form(form::Message),
    /// Run, or Enter in a field.
    Submit,
    /// Cancel, Escape or the scrim.
    Close,
    /// The Engine answered the run of this number.
    Ran(u64, AppResult<String>),
    Failure(crate::ui::widgets::feedback::error_message::Event),
}

/// What the page does after a message of the dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Stay,
    /// The dialog closed, nothing done.
    Closed,
    /// The container of this id started: the dialog closed, and the page
    /// shows its logs.
    Started(String),
}

/// The open Run dialog.
#[derive(Debug)]
pub struct Dialog {
    kind: Kind,
    pub fields: Form,
    /// The Engine was asked, and has not answered.
    pending: bool,
    /// Why the last run failed, shown in place.
    failure: Option<Surfaced>,
    /// The run asked last: an older one's answer is not this dialog's.
    number: u64,
    /// Whether it shows: an answer that comes while it doesn't is said
    /// in a toast.
    presence: Presence,
}

/// Whether a dialog shows, as its action's report reads it once the
/// Engine answers: a failure it can't say in place is said in a toast.
/// The dialog going away (the page left, another dialog in its place)
/// is it no longer showing.
#[derive(Debug)]
pub struct Presence(Arc<AtomicBool>);

impl Presence {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(true)))
    }

    pub fn set(&self, shown: bool) {
        self.0.store(shown, Ordering::SeqCst);
    }

    pub fn is_shown(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }

    /// What a report reads.
    pub fn handle(&self) -> Arc<AtomicBool> {
        self.0.clone()
    }
}

impl Default for Presence {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Presence {
    fn drop(&mut self) {
        self.set(false);
    }
}

/// A number no action asked before, whatever dialog asked it: an older
/// dialog's answer is never taken for a newer one's.
pub fn fresh() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

impl Dialog {
    fn open(kind: Kind, fields: Form, ctx: &Context<'_>) -> Self {
        let mut fields = fields;
        fields.read(ctx);
        if fields.offers_pods {
            ctx.watch(resources::pods(&fields.host_id));
        }
        Self {
            kind,
            fields,
            pending: false,
            failure: None,
            number: 0,
            presence: Presence::new(),
        }
    }

    /// Run a container on `host_id` from `image` (empty for none), in the
    /// pod `pod` when given; `offers_pods` on a Podman Host.
    pub fn run(
        host_id: &str,
        image: &str,
        pod: &str,
        offers_pods: bool,
        ctx: &Context<'_>,
    ) -> Self {
        let form = RunForm {
            pod: pod.to_owned(),
            ..RunForm::new(image)
        };
        let mut fields = Form::new(host_id, form);
        fields.offers_pods = offers_pods;
        Self::open(Kind::Run, fields, ctx)
    }

    /// Run a copy of the container `name`, whose settings are `original`,
    /// named so no other container is (`DuplicateDialog`). A copy keeps
    /// its original's settings, which a pod can't take.
    pub fn duplicate(
        host_id: &str,
        name: &str,
        original: RunForm,
        copy_name: String,
        ctx: &Context<'_>,
    ) -> Self {
        let copy = RunForm {
            name: copy_name,
            ..original.clone()
        };
        let kind = Kind::Duplicate {
            name: name.to_owned(),
            original: Box::new(original),
        };
        Self::open(kind, Form::new(host_id, copy), ctx)
    }

    /// Whether it shows.
    pub fn is_shown(&self) -> bool {
        self.presence.is_shown()
    }

    /// Closed, kept as it is for when it opens again (React's dialog stayed
    /// mounted): it lets go of what it read, but the pods a Podman page
    /// reads itself.
    pub fn hide(&mut self, ctx: &Context<'_>) {
        self.presence.set(false);
        self.fields.let_go(ctx);
    }

    /// Opened again, as it was.
    pub fn show(&mut self, ctx: &Context<'_>) {
        self.presence.set(true);
        self.fields.read(ctx);
    }

    /// Why the last run failed, while the dialog says it.
    pub fn failure(&self) -> Option<&Surfaced> {
        self.failure.as_ref()
    }

    pub fn update(&mut self, message: Message, ctx: &Context<'_>) -> (Task<Message>, Step) {
        match message {
            Message::Submit => return self.submit(ctx),
            Message::Form(message) => {
                let (task, submits) = self.fields.update(message, ctx);
                let task = task.map(Message::Form);
                if submits {
                    let (sent, step) = self.submit(ctx);
                    return (Task::batch([task, sent]), step);
                }
                return (task, Step::Stay);
            }
            Message::Close => return (Task::none(), Step::Closed),
            Message::Ran(number, result) if number == self.number && self.pending => {
                self.pending = false;
                match result {
                    Ok(id) => return (Task::none(), Step::Started(id)),
                    Err(error) => self.failure = Some(Surfaced::new(error, "containers_run")),
                }
            }
            Message::Ran(..) => {}
            Message::Failure(event) => {
                use crate::ui::widgets::feedback::error_message::Event;
                match event {
                    Event::Details => {
                        if let Some(failure) = &self.failure {
                            ctx.details(failure.clone());
                        }
                    }
                    Event::Copy(command) => ctx.copy(command),
                    Event::Retry => {}
                }
            }
        }
        (Task::none(), Step::Stay)
    }

    fn submit(&mut self, ctx: &Context<'_>) -> (Task<Message>, Step) {
        if !self.pending {
            self.fields.add_drafts();
        }
        if self.pending || !self.fields.is_ready() {
            return (Task::none(), Step::Stay);
        }
        self.pending = true;
        self.failure = None;
        self.number = fresh();
        let number = self.number;
        let shown = self.presence.handle();
        let form = &self.fields.form;
        let spec = form.to_spec();
        let pod = if self.fields.offers_pods {
            form.pod()
        } else {
            None
        };
        let (state, host) = (ctx.state.clone(), self.fields.host_id.clone());
        let scope = keys::host::containers(&host);
        let task = ctx.act(
            async move { containers_run(&state, host, spec, pod).await },
            move |result| match result {
                Ok(_) => Outcome::quiet()
                    .saying(|| t!("containers.run.started"))
                    .invalidating(scope.clone()),
                // Said in the dialog (`meta: { silent: true }`), or in a
                // toast when it doesn't show.
                Err(error) if !shown.load(Ordering::SeqCst) => {
                    Outcome::quiet().failing(Surfaced::new(error.clone(), "containers_run"))
                }
                Err(_) => Outcome::quiet(),
            },
            move |result| Message::Ran(number, result),
        );
        (task, Step::Stay)
    }

    /// The host ports a copy shares with its original, said over the
    /// buttons.
    fn notice(&self) -> Option<crate::ui::theme::Element<'_, Message>> {
        let Kind::Duplicate { name, original } = &self.kind else {
            return None;
        };
        let shared = shared_host_ports(&self.fields.form, original);
        if shared.is_empty() {
            return None;
        }
        let count = shared.len();
        let says = format!(
            "{} {}",
            t!(
                "containers.duplicate.sharedPorts",
                name = name.as_str(),
                count = count,
                ports = shared.join(", ")
            ),
            t!("containers.duplicate.sharedPortsFix", count = count)
        );
        Some(alert(None, says, false))
    }

    pub fn view<'a>(&'a self, ctx: &Context<'a>) -> Modal<'a, Message> {
        let title = match &self.kind {
            Kind::Run => t!("containers.run.title"),
            Kind::Duplicate { name, .. } => t!("containers.duplicate.title", name = name.as_str()),
        };
        let lists = self.fields.lists(ctx);
        let login = self.fields.login(ctx);
        let fields = view::fields(&self.fields, &lists, login.as_ref()).map(Message::Form);
        let mut content = dialog(title)
            .description(t!("containers.run.description"))
            .push(fields);
        if let Some(notice) = self.notice() {
            content = content.push(notice);
        }
        if let Some(failure) = &self.failure {
            content = content.push(error_message(&failure.error, false, Message::Failure));
        }
        let cancel = button::text(t!("common.cancel"))
            .variant(Variant::Ghost)
            .on_press(Message::Close);
        let label = if self.pending {
            t!("containers.run.starting")
        } else {
            t!("containers.run.submit")
        };
        let ready = !self.pending && self.fields.is_ready();
        let run = button::text(label).on_press_maybe(ready.then_some(Message::Submit));
        let content = content.footer(vec![cancel.into(), run.into()]);
        let scrolled = scroll_area(container(content).width(Length::Fill));
        modal(scrolled)
            .key("run")
            .width(WIDTH_WIDE)
            .on_close(Message::Close)
            .close_on_scrim()
    }
}

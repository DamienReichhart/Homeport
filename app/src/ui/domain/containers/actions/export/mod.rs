//! Export: a container's whole filesystem saved to a tar on this
//! computer (`ExportDialog`, `exportContainer`, `TransferDialog`). The
//! save dialog asks where, then a dialog shows how far the Engine's
//! export is, with Cancel, then Close.
//!
//! The export runs by `ctx.act`: its toast is the shell's, so it is said
//! even when the page was left meanwhile, and the export goes on to its
//! end then. In React leaving the page closed the dialog, which stopped
//! the export.

use std::path::PathBuf;

use iced::{
    futures::{pin_mut, StreamExt},
    Task,
};
use tokio::sync::watch;

use homeport_core::error::AppResult;
use homeport_engine::transfer::TransferEvent;

use super::Target;
use crate::{
    api::{app::streams::stream_cancel, engine::images::containers_export_subscribe},
    data::streams::{shapes, Update},
    i18n::format,
    platform::dialogs::{Filter, SaveFile},
    t,
    ui::{
        shell::{Context, Outcome},
        theme::{
            self, font,
            metrics::{space, text as size},
            Element,
        },
        widgets::{
            basic::{
                button::{self, Variant},
                progress, technical,
            },
            feedback::{error_message, Surfaced},
            line,
            overlay::{dialog, modal, Modal},
            paragraph, stack,
        },
    },
};

/// How an export ended, when it did not fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exported {
    Whole,
    /// Cancel was pressed: the Engine was told to stop, no file is left.
    Stopped,
}

/// How far it is: bytes written, of how many if the Engine said.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Written {
    pub bytes: u64,
    pub total: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// The save dialog answered, for this container.
    To(Target, AppResult<Option<PathBuf>>),
    /// How far the export of this number is.
    Wrote(u64, Written),
    /// The export of this number ended.
    Ended(u64, AppResult<Exported>),
    /// Cancel, or Close once it ended.
    Close,
    Failure(error_message::Event),
}

/// An export the dialog shows.
#[derive(Debug)]
pub struct Export {
    number: u64,
    target: Target,
    path: PathBuf,
    written: Written,
    done: bool,
    failure: Option<Surfaced>,
    stop: watch::Sender<bool>,
}

/// The save dialog's question, the container's name made a file's.
pub fn request(target: &Target) -> SaveFile {
    SaveFile::new(
        t!("containers.export.pick", name = target.name.as_str()),
        &format!("{}.tar", target.name),
    )
    .filter(Filter::new(t!("containers.export.tarArchive"), &["tar"]))
}

/// What the shell says of an export.
pub(crate) fn exported(destination: String, result: &AppResult<Exported>) -> Outcome {
    match result {
        Ok(Exported::Whole) => Outcome::quiet().saying(move || {
            t!(
                "containers.export.done",
                destination = technical(&destination)
            )
        }),
        Ok(Exported::Stopped) => Outcome::quiet(),
        Err(error) => {
            Outcome::quiet().failing(Surfaced::new(error.clone(), "containers_export_subscribe"))
        }
    }
}

/// Runs the Engine's export to its end, saying how far it is, until it
/// ends, fails, or is stopped.
async fn run(
    state: std::sync::Arc<crate::app::AppState>,
    target: Target,
    destination: String,
    wrote: watch::Sender<Written>,
    mut stopped: watch::Receiver<bool>,
) -> AppResult<Exported> {
    let (host, id) = (target.host_id, target.id);
    let job = shapes::job(state.clone(), move |state, sink| async move {
        containers_export_subscribe(&state, host, id, destination, move |event| sink.send(event))
            .await
    });
    pin_mut!(job);
    let mut started = None;
    loop {
        tokio::select! {
            update = job.next() => match update {
                Some(Update::Started(id)) => started = Some(id),
                Some(Update::Failed(error)) => return Err(error),
                Some(Update::Batch { items, .. }) => {
                    for event in items {
                        match event {
                            TransferEvent::Progress { bytes, total } => {
                                let _ = wrote.send(Written { bytes, total });
                            }
                            TransferEvent::Done { .. } => return Ok(Exported::Whole),
                            TransferEvent::Error { error } => return Err(error),
                            TransferEvent::Line { .. } => {}
                        }
                    }
                }
                // Over without saying it was done: it was cancelled.
                Some(Update::Ended) | None => return Ok(Exported::Stopped),
            },
            _ = stopped.wait_for(|stop| *stop) => {
                // A job goes on until it is told: the Engine's export
                // ends, and its unfinished file goes.
                if let Some(id) = started {
                    stream_cancel(&state, id);
                }
                return Ok(Exported::Stopped);
            }
        }
    }
}

/// The values of a watch after its first, until its sender goes.
fn steps(
    receiver: watch::Receiver<Written>,
) -> impl iced::futures::Stream<Item = Written> + Send + 'static {
    iced::futures::stream::unfold(receiver, |mut receiver| async move {
        receiver.changed().await.ok()?;
        let value = *receiver.borrow_and_update();
        Some((value, receiver))
    })
}

/// The export of one container at a time, and the number of the last.
#[derive(Debug, Default)]
pub struct Exports {
    pub shown: Option<Export>,
    /// The save dialog is open.
    pub asking: bool,
    started: u64,
}

impl Exports {
    /// Asks where to save `target`'s filesystem.
    pub fn ask(&mut self, target: Target, ctx: &Context<'_>) -> Task<Message> {
        if self.asking || self.shown.is_some() {
            return Task::none();
        }
        self.asking = true;
        let asked = ctx.platform.save_file(request(&target));
        asked.map(move |answer| Message::To(target.clone(), answer))
    }

    pub fn update(&mut self, message: Message, ctx: &Context<'_>) -> Task<Message> {
        match message {
            Message::To(target, answer) => {
                self.asking = false;
                match answer {
                    Ok(Some(path)) => return self.start(target, path, ctx),
                    Ok(None) => {}
                    Err(error) => ctx.failed(Surfaced::new(error, "save_file")),
                }
            }
            Message::Wrote(number, written) => {
                // Steps may come after the end: only more is taken.
                if let Some(export) = self.shown.as_mut().filter(|e| e.number == number) {
                    if written.bytes > export.written.bytes {
                        export.written = written;
                    }
                }
            }
            Message::Ended(number, result) => {
                if let Some(export) = self.shown.as_mut().filter(|e| e.number == number) {
                    match result {
                        Ok(Exported::Whole) => export.done = true,
                        Ok(Exported::Stopped) => self.shown = None,
                        Err(error) => {
                            export.failure =
                                Some(Surfaced::new(error, "containers_export_subscribe"));
                        }
                    }
                }
            }
            Message::Close => {
                if let Some(export) = self.shown.take() {
                    let _ = export.stop.send(true);
                }
            }
            Message::Failure(error_message::Event::Details) => {
                if let Some(failure) = self.shown.as_ref().and_then(|e| e.failure.clone()) {
                    ctx.details(failure);
                }
            }
            Message::Failure(error_message::Event::Copy(command)) => ctx.copy(command),
            Message::Failure(error_message::Event::Retry) => {}
        }
        Task::none()
    }

    fn start(&mut self, target: Target, path: PathBuf, ctx: &Context<'_>) -> Task<Message> {
        self.started += 1;
        let number = self.started;
        let (stop, stopped) = watch::channel(false);
        let (wrote, written) = watch::channel(Written::default());
        let destination = path.to_string_lossy().into_owned();
        let work = run(
            ctx.state.clone(),
            target.clone(),
            destination.clone(),
            wrote,
            stopped,
        );
        self.shown = Some(Export {
            number,
            target,
            path,
            written: Written::default(),
            done: false,
            failure: None,
            stop,
        });
        let done = ctx.act(
            work,
            move |result| exported(destination, result),
            move |result| Message::Ended(number, result),
        );
        let steps = steps(written).map(move |written| Message::Wrote(number, written));
        Task::batch([done, Task::run(steps, |message| message)])
    }

    /// The dialog of the export that shows.
    pub fn dialog(&self) -> Option<Modal<'_, Message>> {
        let export = self.shown.as_ref()?;
        let busy = !export.done && export.failure.is_none();
        #[allow(clippy::cast_precision_loss)]
        let bytes = |count: u64| format::bytes(count as f64);
        let Written {
            bytes: wrote,
            total,
        } = export.written;
        let count = match total {
            Some(total) => format!("{} / {}", bytes(wrote), bytes(total)),
            None => bytes(wrote),
        };
        #[allow(clippy::cast_precision_loss)]
        let percent = match (export.done, total) {
            (true, _) => 100.0,
            (false, Some(total)) if total > 0 => wrote as f32 / total as f32 * 100.0,
            _ => 0.0,
        };
        let counted = line(vec![
            // Words of the language: its own face, where text starts.
            iced::widget::container(
                iced::widget::text(t!("containers.export.written"))
                    .size(size::XS)
                    .style(theme::text::muted),
            )
            .width(iced::Length::Fill)
            .align_x(crate::ui::widgets::start())
            .into(),
            iced::widget::text(count)
                .size(size::XS)
                .font(font::MONO)
                .style(theme::text::muted)
                .into(),
        ])
        .width(iced::Length::Fill);
        let mut body: Vec<Element<'_, Message>> =
            vec![counted.into(), progress(percent, None).into()];
        if let Some(failure) = &export.failure {
            body.push(error_message(&failure.error, false, Message::Failure));
        }
        if export.done {
            body.push(
                paragraph(t!("containers.export.finished"))
                    .style(theme::text::tone(theme::Tone::Success))
                    .into(),
            );
        }
        let close = if busy {
            button::text(t!("common.cancel")).variant(Variant::Ghost)
        } else {
            button::text(t!("common.close"))
        }
        .on_press(Message::Close);
        let path = technical(&export.path.display().to_string());
        let content = dialog(t!(
            "containers.export.title",
            name = export.target.name.as_str()
        ))
        .description(t!("containers.export.to", path = path))
        .push(stack(body).spacing(space::S2))
        .footer(vec![close.into()]);
        Some(modal(content).key("export").on_close(Message::Close))
    }
}

#[cfg(all(test, unix))]
mod tests;

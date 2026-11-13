//! The Logs tab (`LogViewer`): the container's log stream in the kit's
//! viewer, how far back it starts, which of its streams it shows, and the
//! lines saved to a file.
//!
//! The stream is opened again, on an empty viewer, when what the Engine
//! applies changes: the history, how far back, the timestamps, stdout and
//! stderr. Each stream has a number: what a stream that was let go of
//! still sends is not the current one's and changes nothing.

use std::path::PathBuf;

use iced::{Subscription, Task};

use homeport_core::{
    error::{AppError, AppResult},
    logs::{LogEvent, LogOptions},
};
use homeport_engine::containers::logs::LogSource;

use crate::{
    app::AppState,
    data::streams::{self, shapes, Update},
    platform::{
        dialogs::SaveFile,
        files::{self, Progress, Saved, Stop},
    },
    t,
    ui::{
        shell::{Context, Outcome},
        widgets::{
            feedback::{error_message, Surfaced},
            form::select,
            log,
        },
    },
};

/// How many lines the viewer keeps (`MAX_KEPT_LINES`).
pub const KEPT_LINES: usize = 100_000;

/// How many lines of history a stream starts with (`TAILS`); `None` is
/// all of it.
pub const TAILS: [Option<u32>; 4] = [Some(100), Some(1000), Some(10_000), None];

/// How far back a stream starts, in seconds (`SINCE`); `None` is any time.
pub const SINCE: [Option<i32>; 4] = [None, Some(300), Some(3600), Some(86_400)];

/// The history a viewer starts with: the last 1 000 lines.
const DEFAULT_TAIL: usize = 1;

pub fn tail_labels() -> Vec<String> {
    vec![
        t!("containers.logs.tail.100"),
        t!("containers.logs.tail.1000"),
        t!("containers.logs.tail.10000"),
        t!("containers.logs.tail.all"),
    ]
}

pub fn since_labels() -> Vec<String> {
    vec![
        t!("containers.logs.since.all"),
        t!("containers.logs.since.300"),
        t!("containers.logs.since.3600"),
        t!("containers.logs.since.86400"),
    ]
}

/// One of the container's two streams.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// A message of the viewer.
    Viewer(log::Message),
    Tail(select::Message),
    Since(select::Message),
    /// The toggle of one of the two streams.
    Toggle(Channel),
    /// What the stream of this number sent.
    Lines(u64, Update<LogEvent>),
    /// What was asked of the failure shown over the lines.
    Failure(error_message::Event),
    /// The save dialog answered: a path, a cancel, or why none showed.
    SaveTo(AppResult<Option<PathBuf>>),
    /// How far the file being written is.
    Wrote(Progress),
    /// The file is in place, the save was stopped, or it failed: the
    /// shell said which.
    Saved,
    /// The button that stops a save.
    StopSaving,
}

/// What a stream is opened with: what tells one from another.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Stream {
    host_id: String,
    container_id: String,
    tail: Option<u32>,
    since: Option<i32>,
    timestamps: bool,
    stdout: bool,
    stderr: bool,
    generation: u64,
}

impl Stream {
    fn options(&self) -> LogOptions {
        LogOptions {
            tail: self.tail,
            timestamps: self.timestamps,
            since: self.since,
            stdout: self.stdout,
            stderr: self.stderr,
            follow: true,
        }
    }
}

/// The lines being saved, from the dialog to the file.
#[derive(Debug, Default)]
struct Saving {
    /// The lines the save dialog is open for.
    asked: Option<log::Download>,
    /// How far the file is, while it is written, and what stops it.
    wrote: Option<(Progress, Stop)>,
}

#[derive(Debug)]
pub struct Logs {
    pub view: log::State,
    /// Which of [`TAILS`] and of [`SINCE`] the stream is asked with.
    pub tail: usize,
    pub since: usize,
    pub stdout: bool,
    pub stderr: bool,
    pub tail_select: select::State,
    pub since_select: select::State,
    /// The number of the stream that feeds the viewer.
    generation: u64,
    /// The Unix time the stream starts from, worked out when it opened:
    /// "the last 5 minutes" are those before that moment.
    since_at: Option<i32>,
    /// Whether the stream was asked for timestamps.
    timestamps: bool,
    /// Why the stream broke, or couldn't start.
    pub failure: Option<Surfaced>,
    /// The stream is over: the Engine has no more of this container.
    ended: bool,
    saving: Saving,
}

fn now() -> i64 {
    jiff::Timestamp::now().as_second()
}

impl Logs {
    pub fn new() -> Self {
        let mut logs = Self {
            view: log::State::new(KEPT_LINES),
            tail: DEFAULT_TAIL,
            since: 0,
            stdout: true,
            stderr: true,
            tail_select: select::State::default(),
            since_select: select::State::default(),
            generation: 0,
            since_at: None,
            timestamps: false,
            failure: None,
            ended: false,
            saving: Saving::default(),
        };
        logs.restart(now());
        logs
    }

    /// The number of the stream that feeds the viewer.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Whether the stream said it has no more, or failed.
    pub fn is_over(&self) -> bool {
        self.ended || self.failure.is_some()
    }

    /// How far a save is, while a file is written.
    pub fn saving(&self) -> Option<Progress> {
        self.saving.wrote.as_ref().map(|(wrote, _)| *wrote)
    }

    /// Opens the stream again, on an empty viewer, with the options as
    /// they are at `now` (Unix seconds): the next [`Logs::subscription`]
    /// is another stream, and the one before is let go of.
    pub fn restart(&mut self, now: i64) {
        self.generation += 1;
        self.view.reset();
        self.failure = None;
        self.ended = false;
        self.timestamps = self.view.shows_timestamps();
        self.since_at = SINCE[self.since]
            .map(|seconds| i32::try_from(now - i64::from(seconds)).unwrap_or(i32::MAX));
    }

    fn stream(&self, host_id: &str, container_id: &str) -> Stream {
        Stream {
            host_id: host_id.to_owned(),
            container_id: container_id.to_owned(),
            tail: TAILS[self.tail],
            since: self.since_at,
            timestamps: self.timestamps,
            stdout: self.stdout,
            stderr: self.stderr,
            generation: self.generation,
        }
    }

    /// The options the Engine is asked with now.
    pub fn options(&self) -> LogOptions {
        self.stream("", "").options()
    }

    /// The container's log stream, while the page returns it: opened
    /// when first returned, cancelled when it no longer is, and another
    /// one each time [`Logs::restart`] changed its number.
    pub fn subscription(
        &self,
        state: &std::sync::Arc<AppState>,
        host_id: &str,
        container_id: &str,
    ) -> Subscription<Message> {
        let stream = self.stream(host_id, container_id);
        let generation = stream.generation;
        streams::subscription(state, stream, |source| {
            let stream = source.params.clone();
            let sources = vec![LogSource {
                container_id: stream.container_id.clone(),
                label: None,
            }];
            let options = stream.options();
            shapes::logs(source.state.clone(), stream.host_id, sources, options)
        })
        .with(generation)
        .map(|(generation, update)| Message::Lines(generation, update))
    }

    /// What the stream sent.
    fn lines(&mut self, update: Update<LogEvent>) -> Task<Message> {
        match update {
            Update::Started(_) => Task::none(),
            Update::Failed(error) => {
                self.failure = Some(Surfaced::new(error, "logs_subscribe"));
                Task::none()
            }
            Update::Batch { items, dropped } => {
                let batch = shapes::log_lines(items);
                let stamped = self.timestamps;
                self.view.skipped(dropped);
                let lines = batch
                    .lines
                    .into_iter()
                    .map(|line| log::Line::from(line).timestamped(stamped));
                let read = self.view.extend(lines);
                if batch.ended {
                    self.view.end();
                    self.ended = true;
                }
                if let Some(error) = batch.error {
                    self.failure = Some(Surfaced::new(error, "logs_subscribe"));
                }
                read.map(Message::Viewer)
            }
            Update::Ended => {
                self.ended = true;
                Task::none()
            }
        }
    }

    /// The viewer asked for its lines to be saved: the dialog first.
    fn download(&mut self, lines: log::Download, ctx: &Context<'_>) -> Task<Message> {
        // Pressed again while its dialog is open, or while a file is
        // written: the lines of the first press are the ones saved.
        if self.saving.asked.is_some() || self.saving.wrote.is_some() {
            return Task::none();
        }
        self.saving.asked = Some(lines);
        let request = SaveFile::new(t!("containers.logs.saveTitle"), "logs.txt");
        ctx.platform.save_file(request).map(Message::SaveTo)
    }

    /// The dialog answered: the file is written off the window's thread,
    /// and the shell says how it ended, whether or not the page is still
    /// there (`useSaveLogs`).
    fn save_to(&mut self, target: AppResult<Option<PathBuf>>, ctx: &Context<'_>) -> Task<Message> {
        let lines = self.saving.asked.take();
        let path = match target {
            Ok(Some(path)) => path,
            // Cancelled: an answer, not a failure.
            Ok(None) => return Task::none(),
            Err(error) => {
                ctx.failed(Surfaced::new(error, "save_file"));
                return Task::none();
            }
        };
        let Some(lines) = lines else {
            return Task::none();
        };
        let saving = files::save(path, move |out, meter| {
            let total = lines.len() as u64;
            lines.write_counted(out, |written| meter.at(written as u64, total))
        });
        self.saving.wrote = Some((Progress::default(), saving.stop));
        let done = ctx.act(saving.done, saved, |_| Message::Saved);
        Task::batch([done, Task::run(saving.progress, Message::Wrote)])
    }

    pub fn update(&mut self, message: Message, ctx: &Context<'_>) -> Task<Message> {
        match message {
            Message::Viewer(message) => {
                let (task, event) = self.view.update(message);
                let task = task.map(Message::Viewer);
                return match event {
                    Some(log::Event::Copy(text)) => {
                        ctx.copy(text);
                        task
                    }
                    Some(log::Event::Download(lines)) => {
                        Task::batch([task, self.download(lines, ctx)])
                    }
                    // The Engine writes the timestamps: the stream is
                    // asked again with them, or without.
                    Some(log::Event::Timestamps(_)) => {
                        self.restart(now());
                        task
                    }
                    Some(log::Event::Cleared) | None => task,
                };
            }
            Message::Tail(message) => {
                if let Some(index) = self.tail_select.update(message, &tail_labels()) {
                    if index != self.tail {
                        self.tail = index;
                        self.restart(now());
                    }
                }
            }
            Message::Since(message) => {
                if let Some(index) = self.since_select.update(message, &since_labels()) {
                    if index != self.since {
                        self.since = index;
                        self.restart(now());
                    }
                }
            }
            Message::Toggle(channel) => {
                match channel {
                    Channel::Stdout => self.stdout = !self.stdout,
                    Channel::Stderr => self.stderr = !self.stderr,
                }
                self.restart(now());
            }
            // A stream that was let go of may still have sent this.
            Message::Lines(generation, _) if generation != self.generation => {}
            Message::Lines(_, update) => return self.lines(update),
            Message::Failure(error_message::Event::Retry) => self.restart(now()),
            Message::Failure(error_message::Event::Details) => {
                if let Some(failure) = &self.failure {
                    ctx.details(failure.clone());
                }
            }
            Message::Failure(error_message::Event::Copy(command)) => ctx.copy(command),
            Message::SaveTo(target) => return self.save_to(target, ctx),
            Message::Wrote(wrote) => {
                // A step that comes after the end changes nothing.
                if let Some((shown, _)) = &mut self.saving.wrote {
                    *shown = wrote;
                }
            }
            Message::Saved => self.saving.wrote = None,
            Message::StopSaving => {
                if let Some((_, stop)) = &self.saving.wrote {
                    stop.now();
                }
            }
        }
        Task::none()
    }
}

/// What the shell says of a save: that the logs are saved, why they
/// couldn't be, or nothing of a save that was stopped.
pub(crate) fn saved(result: &Result<Saved, AppError>) -> Outcome {
    match result {
        Ok(Saved::Whole) => Outcome::quiet().saying(|| t!("containers.logs.saved")),
        Ok(Saved::Stopped) => Outcome::quiet(),
        Err(error) => Outcome::quiet().failing(Surfaced::new(error.clone(), "write_text_file")),
    }
}

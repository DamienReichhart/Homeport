//! The log viewer, on lines as containers write them: colours, several
//! sources, timestamps, stderr, lines too long for the viewer; with a
//! handful of lines, a million, or a stream of fifty thousand a second.

use std::time::Duration;

use std::path::PathBuf;

use homeport_core::error::{AppError, AppResult};
use iced::{widget::text, Subscription, Task};

use super::{Gallery, Message};
use crate::{
    platform::{
        dialogs::{Filter, SaveFile},
        files::{self, Progress, Saved},
    },
    t,
    ui::{
        theme::{
            metrics::{space, text as size, ButtonSize},
            Element,
        },
        widgets::{
            basic::button::{self, Variant},
            feedback::{toast::Kind, Surfaced},
            line,
            log::{self, Line, Stream},
            stack,
        },
    },
};

/// The gallery's messages about saving its viewer's lines: what a page
/// with a log viewer does of `log::Event::Download`.
#[derive(Debug, Clone)]
pub enum Save {
    /// The save dialog answered: a path, `None` when it was cancelled, or
    /// why no dialog could be shown.
    To(AppResult<Option<PathBuf>>),
    /// How far the file being written is.
    Wrote(Progress),
    /// The file is in place, the save was stopped, or why it failed.
    Done(Result<Saved, AppError>),
}

/// The lines the gallery is saving, from the dialog to the file.
#[derive(Debug, Default)]
pub struct Saving {
    /// The lines the save dialog is open for.
    asked: Option<log::Download>,
    /// How far the file is, while it is written.
    pub wrote: Option<Progress>,
}

/// How tall the gallery's viewer is: the gallery's page scrolls.
pub const HEIGHT: f32 = 420.0;
/// How many lines the gallery's viewer keeps.
pub const CAP: usize = 1_000_000;
/// The lines a second the gallery's stream writes, and how many times a
/// second they come (`data::streams::UPDATES_PER_SECOND`).
pub const RATE: usize = 50_000;
const FRAMES: usize = 60;

/// What the gallery's viewer is filled with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fill {
    Sample,
    Million,
    /// Lines keep coming, [`RATE`] a second.
    Stream(bool),
}

const SERVICES: [&str; 3] = ["web.1", "db.1", "worker.12"];
const LEVELS: [(&str, &str); 4] = [
    ("\x1b[32mINFO\x1b[0m", "request served"),
    ("\x1b[33mWARN\x1b[0m", "slow query"),
    ("\x1b[1;31mERROR\x1b[0m", "connection reset by peer"),
    ("\x1b[2mDEBUG\x1b[0m", "cache miss"),
];

/// A few lines of everything a log can hold, the same every time.
pub fn sample() -> Vec<Line> {
    let stamped = |n: usize, text: &str| {
        Line::new(format!("2026-05-01T10:00:{:02}.123456789Z {text}", n % 60)).timestamped(true)
    };
    let mut lines = vec![
        stamped(0, "listening on \x1b[1m:8080\x1b[0m").source(SERVICES[0]),
        stamped(1, "database system is ready to accept connections").source(SERVICES[1]),
        stamped(
            2,
            "\x1b[32mINFO\x1b[0m  GET /api/v1/containers \x1b[36m200\x1b[0m in \x1b[2m12 ms\x1b[0m",
        )
        .source(SERVICES[0]),
        stamped(
            3,
            "\x1b[33mWARN\x1b[0m  slow query: \x1b[3mSELECT * FROM stacks\x1b[0m took \x1b[4m1.2 s\x1b[0m",
        )
        .source(SERVICES[1]),
        stamped(
            4,
            "\x1b[1;31mERROR\x1b[0m failed to bind 0.0.0.0:80: \x1b[31maddress already in use\x1b[0m",
        )
        .source(SERVICES[0])
        .stream(Stream::Stderr),
        stamped(5, "panic: runtime error: index out of range [5] with length 3")
            .source(SERVICES[2])
            .stream(Stream::Stderr),
        stamped(
            6,
            "\x1b[38;5;208m256 colours\x1b[0m \x1b[38;2;90;160;255mtruecolour\x1b[0m \
             \x1b[38;2;20;20;60mdark blue\x1b[0m \x1b[38;2;250;250;120mpale yellow\x1b[0m \
             \x1b[7minverse\x1b[0m \x1b[30;43m black on yellow \x1b[0m \x1b[9mstruck\x1b[0m",
        )
        .source(SERVICES[2]),
        stamped(
            7,
            "a line too long for the viewer: {\"level\":\"info\",\"msg\":\"reconciled\",\
             \"stack\":\"shop\",\"services\":[\"web\",\"db\",\"cache\",\"worker\"],\
             \"took_ms\":183,\"host\":\"prod-eu-1\",\"attempt\":3,\"next\":\"2026-05-01T10:05:00Z\"}",
        )
        .source(SERVICES[2]),
        stamped(8, "\x1b]0;a title that is never drawn\x07\x1b[2Kcursor moves are taken out")
            .source(SERVICES[0]),
        stamped(9, "日本語のログ, العربية and émojis 🚀 keep their place").source(SERVICES[1]),
    ];
    lines.extend((10..24).map(|n| {
        let (level, says) = LEVELS[n % LEVELS.len()];
        let line = stamped(n, &format!("{level}  {says} ({n})")).source(SERVICES[n % 3]);
        if n % LEVELS.len() == 2 {
            line.stream(Stream::Stderr)
        } else {
            line
        }
    }));
    lines
}

/// `count` lines as a busy service writes them, from the line `from`:
/// about a hundred bytes each, a colour in most, the same every time.
pub fn many(from: usize, count: usize) -> impl Iterator<Item = Line> {
    (from..from + count).map(|n| {
        let (level, says) = LEVELS[n % LEVELS.len()];
        let text = format!(
            "2026-05-01T10:{:02}:{:02}.{:09}Z {level}  {says} id={n} path=/api/v1/containers/{:x} \
             took={} ms",
            n / 3600 % 60,
            n / 60 % 60,
            n * 7919 % 1_000_000_000,
            n * 2_654_435_761 % 0xffff_ffff,
            n % 977,
        );
        let line = Line::new(text).timestamped(true);
        if n % 16 == 2 {
            line.stream(Stream::Stderr)
        } else {
            line
        }
    })
}

/// A viewer that holds `lines`.
pub fn filled(cap: usize, lines: impl IntoIterator<Item = Line>) -> log::State {
    let mut state = log::State::new(cap).with_timestamps(true);
    // Nothing is searched yet: there is nothing to run.
    drop(state.extend(lines));
    state
}

/// The gallery's viewer, as a page would build it.
pub fn view(state: &log::State) -> Element<'_, Message> {
    log::Viewer::new(state, Message::Log).height(HEIGHT).view()
}

impl Gallery {
    pub(super) fn log_message(&mut self, message: log::Message) -> Task<Message> {
        let (task, event) = self.log.update(message);
        let task = task.map(Message::Log);
        match event {
            Some(log::Event::Copy(text)) => {
                let lines = text.lines().count();
                self.toasts
                    .say(Kind::Info, move || format!("Copied {lines} lines"));
                Task::batch([task, iced::clipboard::write(text)])
            }
            // Pressed again while its dialog is open: the lines of the
            // first press are the ones the dialog is about.
            Some(log::Event::Download(_)) if self.log_saving.asked.is_some() => task,
            Some(log::Event::Download(lines)) => {
                // The lines are kept as they were when the button was
                // pressed, while the dialog is open.
                self.log_saving.asked = Some(lines);
                let request = SaveFile::new(t!("containers.logs.saveTitle"), "logs.txt")
                    .filter(Filter::new("Text", &["txt", "log"]));
                let asked = self.platform.save_file(request);
                Task::batch([task, asked.map(|path| Message::LogSave(Save::To(path)))])
            }
            Some(log::Event::Timestamps(_) | log::Event::Cleared) | None => task,
        }
    }

    /// The save dialog answered, the file moves on, or it ended.
    pub(super) fn log_save(&mut self, save: Save) -> Task<Message> {
        match save {
            // Cancelled: an answer, not a failure. Nothing is said.
            Save::To(Ok(None)) => self.log_saving.asked = None,
            // No dialog could be shown: that is said, as any failure.
            Save::To(Err(error)) => {
                self.log_saving.asked = None;
                self.toasts.failure(Surfaced::new(error, "save_file"));
            }
            Save::To(Ok(Some(path))) => {
                let Some(lines) = self.log_saving.asked.take() else {
                    return Task::none();
                };
                self.log_saving.wrote = Some(Progress::default());
                // Counting a million lines and writing them are both
                // done off the window's thread.
                let saving = files::save(path, move |out, meter| {
                    let total = lines.len() as u64;
                    lines.write_counted(out, |written| meter.at(written as u64, total))
                });
                // A page hands `saving.done` to `ctx.act`, so the toast
                // is said even when the page was left meanwhile.
                return Task::batch([
                    Task::perform(saving.done, |saved| Message::LogSave(Save::Done(saved))),
                    Task::run(saving.progress, |wrote| {
                        Message::LogSave(Save::Wrote(wrote))
                    }),
                ]);
            }
            Save::Wrote(wrote) => {
                // A step that comes after the end changes nothing.
                if let Some(shown) = &mut self.log_saving.wrote {
                    *shown = wrote;
                }
            }
            Save::Done(saved) => {
                self.log_saving.wrote = None;
                match saved {
                    Ok(Saved::Whole) => {
                        self.toasts
                            .say(Kind::Success, || t!("containers.logs.saved"));
                    }
                    // Stopped on the way: nothing was saved, nothing to say.
                    Ok(Saved::Stopped) => {}
                    Err(error) => {
                        self.toasts.failure(Surfaced::new(error, "write_text_file"));
                    }
                }
            }
        }
        Task::none()
    }

    pub(super) fn log_fill(&mut self, fill: Fill) -> Task<Message> {
        self.log_streaming = false;
        match fill {
            Fill::Sample => self.log = filled(CAP, sample()),
            Fill::Million => self.log = filled(CAP, many(0, CAP)),
            Fill::Stream(on) => self.log_streaming = on,
        }
        Task::none()
    }

    /// A sixtieth of a second of the stream.
    pub(super) fn log_tick(&mut self) -> Task<Message> {
        let from = self.log_written;
        self.log_written += RATE / FRAMES;
        self.log.extend(many(from, RATE / FRAMES)).map(Message::Log)
    }

    pub(super) fn log_subscription(&self) -> Subscription<Message> {
        if self.log_streaming && self.page == super::Page::Log {
            // A second is sixty frames.
            #[allow(clippy::cast_possible_truncation)]
            let frame = Duration::from_micros(1_000_000 / FRAMES as u64);
            iced::time::every(frame).map(|_| Message::LogTick)
        } else {
            Subscription::none()
        }
    }

    pub(super) fn logs(&self) -> Element<'_, Message> {
        let choice = |label: &'static str, fill: Fill, on: bool| -> Element<'_, Message> {
            button::text(label)
                .variant(if on {
                    Variant::Secondary
                } else {
                    Variant::Outline
                })
                .size(ButtonSize::Sm)
                .on_press(Message::LogFill(fill))
                .into()
        };
        let mut switches = vec![
            choice("A sample", Fill::Sample, false),
            choice("1,000,000 lines", Fill::Million, false),
            choice(
                "50,000 lines a second",
                Fill::Stream(!self.log_streaming),
                self.log_streaming,
            ),
        ];
        if let Some(wrote) = self.log_saving.wrote {
            let percent = wrote.percent();
            switches.push(text(format!("Saving… {percent} %")).size(size::SM).into());
        }
        let switches = line(switches).spacing(space::S2);
        stack(vec![switches.into(), view(&self.log)])
            .spacing(space::S4)
            .into()
    }
}

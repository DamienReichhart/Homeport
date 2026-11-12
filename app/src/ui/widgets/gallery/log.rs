//! The log viewer, on lines as containers write them: colours, several
//! sources, timestamps, stderr, lines too long for the viewer; with a
//! handful of lines, a million, or a stream of fifty thousand a second.

use std::time::Duration;

use iced::{Subscription, Task};

use super::{Gallery, Message};
use crate::ui::{
    theme::{
        metrics::{space, ButtonSize},
        Element,
    },
    widgets::{
        basic::button::{self, Variant},
        feedback::toast::Kind,
        line,
        log::{self, Line, Stream},
        stack,
    },
};

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
            Some(log::Event::Download(lines)) => {
                // The file dialog is the platform's; the gallery counts.
                let count = lines.len();
                self.toasts
                    .say(Kind::Info, move || format!("{count} lines to save"));
                task
            }
            Some(log::Event::Timestamps(_) | log::Event::Cleared) | None => task,
        }
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
        let switches = line(vec![
            choice("A sample", Fill::Sample, false),
            choice("1,000,000 lines", Fill::Million, false),
            choice(
                "50,000 lines a second",
                Fill::Stream(!self.log_streaming),
                self.log_streaming,
            ),
        ])
        .spacing(space::S2);
        stack(vec![switches.into(), view(&self.log)])
            .spacing(space::S4)
            .into()
    }
}

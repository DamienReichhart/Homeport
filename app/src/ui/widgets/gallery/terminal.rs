//! The terminal, on what programs write: a shell's output with every
//! colour and attribute, wide text, emoji and a link, and a full-screen
//! program's frame on the alternate screen. What is typed in the shell is
//! echoed as a line discipline would, so the keys can be tried; `yes` and
//! a rain of letters (`cmatrix`) can be poured in to see it keep up.

use std::time::Duration;

use iced::{widget::text, Subscription, Task};

use super::{Gallery, Message};
use crate::ui::{
    theme::{
        metrics::{space, text as size, ButtonSize},
        Element,
    },
    widgets::{
        basic::button::{self, Variant},
        feedback::toast::Kind,
        line, stack,
        terminal::{self, Terminal},
    },
};

/// How tall the gallery's shell is (the gallery's page scrolls), and its
/// full-screen program: 24 rows, the size its frame was recorded at.
pub const HEIGHT: f32 = 300.0;
pub const APP_HEIGHT: f32 = 24.0 * 16.0 + 18.0;
/// How many times a second a pour comes, as a stream's batches do.
const FRAMES: u64 = 60;
/// How much of `yes` comes in a sixtieth of a second: 50 MB in about
/// five seconds.
pub const YES_BATCH: usize = 160 * 1024;

const PROMPT: &str = "\x1b[1;32mops@web-1\x1b[0m:\x1b[1;34m/srv/shop\x1b[0m$ ";

/// Which of the gallery's terminals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Which {
    Shell,
    App,
}

/// What pours into the shell, a frame at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pour {
    Yes,
    Rain,
}

#[derive(Debug, Clone)]
pub enum Event {
    Terminal(Which, terminal::Message),
    Pour(Option<Pour>),
    Tick,
    FontSize(f32),
}

/// The gallery's two terminals and what pours into the first.
#[derive(Debug)]
pub struct Terminals {
    pub shell: terminal::State,
    pub app: terminal::State,
    pub pour: Option<Pour>,
    /// How many frames have poured: the rain's seed.
    poured: u64,
    /// The title the shell's program gave, and how many times it rang.
    pub title: Option<String>,
    pub bells: usize,
}

impl Default for Terminals {
    fn default() -> Self {
        let mut shell = terminal::State::new();
        drop(shell.feed(&shell_sample()));
        let mut app = terminal::State::new();
        drop(app.feed_bytes(APP_SAMPLE));
        Self {
            shell,
            app,
            pour: None,
            poured: 0,
            title: None,
            bells: 0,
        }
    }
}

/// A full-screen program's frame (htop's meters, header and rows, drawn
/// by curses on the alternate screen), as recorded.
pub const APP_SAMPLE: &[u8] = include_bytes!("../../../../tests/fixtures/terminal/meters.bin");
const UTF8_SAMPLE: &[u8] = include_bytes!("../../../../tests/fixtures/terminal/utf8.bin");

/// A shell's session: a command whose output has every colour and
/// attribute, wide text and emoji, then a link and a listing.
pub fn shell_sample() -> String {
    let mut sample = format!("{PROMPT}./colours.sh\r\n");
    sample.push_str(&String::from_utf8_lossy(UTF8_SAMPLE));
    sample.push_str("Lines: ╭─╮ ┏━┓ ╔═╦═╗ ├─┼─┤ ╌╌ ┄┄┄ ┈┈┈┈  Blocks: █▓▒░ ▀▄▌▐ ▖▗▘▝\r\n");
    sample.push_str("       │ │ ┃ ┃ ╠═╬═╣ │ │ │ ╎  ┆   ┊             ▁▂▃▄▅▆▇█\r\n");
    sample.push_str("       ╰─╯ ┗━┛ ╚═╩═╝ └─┴─┘\r\n");
    sample.push_str(&format!(
        "{PROMPT}cat NOTES\r\nDocs at https://homeport.dev/docs (see \x1b]8;;https://example.com/runbook\x1b\\the runbook\x1b]8;;\x1b\\).\r\n"
    ));
    sample.push_str(&format!(
        "{PROMPT}ls --color\r\n\x1b[1;34massets\x1b[0m  \x1b[1;32mdeploy.sh\x1b[0m  docker-compose.yml  \x1b[1;36mlogs\x1b[0m  README.md\r\n{PROMPT}"
    ));
    sample
}

/// `bytes` of what `yes` writes.
pub fn yes_output(bytes: usize) -> String {
    "y\r\n".repeat(bytes.div_ceil(3))
}

/// A frame of a rain of letters, as `cmatrix` draws it: every cell of a
/// `cols` by `rows` screen written again, in greens, with a cursor move
/// before each column.
pub fn rain_frame(seed: u64, cols: usize, rows: usize) -> String {
    let mut next = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    let mut random = move || {
        next = next
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        next >> 33
    };
    let mut frame = String::with_capacity(cols * rows * 12);
    frame.push_str("\x1b[?2026h");
    for col in 0..cols {
        for row in 0..rows {
            frame.push_str(&format!("\x1b[{};{}H", row + 1, col + 1));
            let glyph = char::from(b'!' + u8::try_from(random() % 90).unwrap_or(0));
            let shade = if random() % 7 == 0 { "1;37" } else { "32" };
            let bold = if random() % 3 == 0 { ";1" } else { "" };
            frame.push_str(&format!("\x1b[{shade}{bold}m{glyph}"));
        }
    }
    frame.push_str("\x1b[0m\x1b[?2026l");
    frame
}

/// What the gallery's shell does with what is typed: it echoes it, as a
/// line discipline would, and answers Enter with a new prompt.
fn echoed(input: &str) -> String {
    let mut echo = String::new();
    for c in input.chars() {
        match c {
            '\r' => {
                echo.push_str("\r\n");
                echo.push_str(PROMPT);
            }
            '\x7f' => echo.push_str("\x08 \x08"),
            '\x03' => {
                echo.push_str("^C\r\n");
                echo.push_str(PROMPT);
            }
            c if c.is_control() => {}
            c => echo.push(c),
        }
    }
    // Sequences (an arrow, a report of the pointer) are not echoed.
    if input.starts_with('\x1b') {
        String::new()
    } else {
        echo
    }
}

/// The gallery's terminal `which`, as a page would build it.
pub fn view(state: &terminal::State, which: Which) -> Element<'_, Message> {
    Terminal::new(state, move |message| {
        Message::Terminal(Event::Terminal(which, message))
    })
    .height(match which {
        Which::Shell => HEIGHT,
        Which::App => APP_HEIGHT,
    })
    .view()
}

impl Gallery {
    pub(super) fn terminal_event(&mut self, event: Event) -> Task<Message> {
        let terminals = &mut self.terminals;
        match event {
            Event::Terminal(which, message) => {
                let state = match which {
                    Which::Shell => &mut terminals.shell,
                    Which::App => &mut terminals.app,
                };
                let (task, event) = state.update(message);
                let task =
                    task.map(move |message| Message::Terminal(Event::Terminal(which, message)));
                let mut asked = vec![];
                match event {
                    // A page sends this to its session; the gallery echoes.
                    Some(terminal::Event::Input(input)) => {
                        if which == Which::Shell {
                            asked = state.feed(&echoed(&input));
                        }
                    }
                    Some(terminal::Event::Copy(text)) => {
                        let lines = text.lines().count();
                        self.toasts
                            .say(Kind::Info, move || format!("Copied {lines} lines"));
                        return Task::batch([task, iced::clipboard::write(text)]);
                    }
                    Some(terminal::Event::Open(url)) => {
                        // A page calls `ctx.open_url`: web, mail and phone links only.
                        let opening = self.platform.open_url(url);
                        return Task::batch([task, Task::perform(opening, |_| Message::Nothing)]);
                    }
                    Some(terminal::Event::Title(title)) => terminals.title = title,
                    Some(terminal::Event::Bell) => terminals.bells += 1,
                    Some(terminal::Event::Resize { .. }) | None => {}
                }
                for event in asked {
                    if let terminal::Event::Title(title) = event {
                        terminals.title = title;
                    }
                }
                task
            }
            Event::Pour(pour) => {
                terminals.pour = pour;
                Task::none()
            }
            Event::Tick => {
                terminals.poured += 1;
                let (cols, rows) = terminals.shell.size();
                let output = match terminals.pour {
                    Some(Pour::Yes) => yes_output(YES_BATCH),
                    Some(Pour::Rain) => rain_frame(terminals.poured, cols, rows),
                    None => return Task::none(),
                };
                drop(terminals.shell.feed(&output));
                Task::none()
            }
            Event::FontSize(size) => {
                terminals.shell.set_font_size(size);
                terminals.app.set_font_size(size);
                Task::none()
            }
        }
    }

    pub(super) fn terminal_subscription(&self) -> Subscription<Message> {
        if self.terminals.pour.is_some() && self.page == super::Page::Terminal {
            let frame = Duration::from_micros(1_000_000 / FRAMES);
            iced::time::every(frame).map(|_| Message::Terminal(Event::Tick))
        } else {
            Subscription::none()
        }
    }

    pub(super) fn terminals(&self) -> Element<'_, Message> {
        let terminals = &self.terminals;
        let choice = |label: &'static str, event: Event, on: bool| -> Element<'_, Message> {
            button::text(label)
                .variant(if on {
                    Variant::Secondary
                } else {
                    Variant::Outline
                })
                .size(ButtonSize::Sm)
                .on_press(Message::Terminal(event))
                .into()
        };
        let pouring = |pour| terminals.pour == Some(pour);
        let font = terminals.shell.font_size();
        let controls = line(vec![
            choice(
                "Pour yes",
                Event::Pour((!pouring(Pour::Yes)).then_some(Pour::Yes)),
                pouring(Pour::Yes),
            ),
            choice(
                "Rain",
                Event::Pour((!pouring(Pour::Rain)).then_some(Pour::Rain)),
                pouring(Pour::Rain),
            ),
            choice("A−", Event::FontSize(font - 1.0), false),
            choice("A+", Event::FontSize(font + 1.0), false),
            text(format!(
                "{} × {} · {} lines · title: {} · bells: {}",
                terminals.shell.size().0,
                terminals.shell.size().1,
                terminals.shell.total_lines(),
                terminals.title.as_deref().unwrap_or("none"),
                terminals.bells,
            ))
            .size(size::SM)
            .into(),
        ])
        .spacing(space::S2);
        stack(vec![
            controls.into(),
            Self::section("A shell", view(&terminals.shell, Which::Shell)),
            Self::section("A full-screen program", view(&terminals.app, Which::App)),
        ])
        .spacing(space::S4)
        .into()
    }
}

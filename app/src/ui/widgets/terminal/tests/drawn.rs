//! The terminal drawn: a shell's colours and attributes, wide text, emoji,
//! box drawing and links, with a selection and a search's matches; a
//! full-screen program's frame on the alternate screen; in both themes
//! and under Arabic chrome, where the grid still reads left to right.

use iced::{widget::container, Length};

use super::screen;
use crate::{
    test_support::{snapshot::assert_snapshot, window::drive},
    ui::{
        theme::{self, Element, Theme},
        widgets::{
            gallery::{
                self,
                terminal::{view, Event as Gallery, Terminals, Which},
            },
            terminal::{Click, Message, State},
        },
    },
};

type Page = gallery::Message;

const SIZE: (f32, f32) = (760.0, 380.0);
/// Room for 80 cells by 24 rows, around the gallery's terminal.
const APP_SIZE: (f32, f32) = (674.0, 450.0);

fn on_page(part: Element<'_, Page>) -> Element<'_, Page> {
    container(part)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::page)
        .into()
}

/// Lays `state` out as the window would and tells it the room it has, as
/// its first frame does.
fn fit(state: &mut State, which: Which, size: (f32, f32)) {
    let frame = iced::Event::Window(iced::window::Event::RedrawRequested(
        std::time::Instant::now(),
    ));
    let messages = drive(
        size,
        || on_page(view(state, which)),
        |window| {
            let _ = window.simulate([frame]);
        },
    );
    for message in messages {
        if let Page::Terminal(Gallery::Terminal(_, message)) = message {
            let _ = state.update(message);
        }
    }
}

/// The gallery's shell, searched for wide text, with a line of a listing
/// selected.
fn shell() -> State {
    let mut state = Terminals::default().shell;
    let _ = state.update(Message::OpenSearch);
    let _ = state.update(Message::Query("日本".to_owned()));
    fit(&mut state, Which::Shell, SIZE);
    let rows = screen(&state);
    let listing = rows
        .iter()
        .position(|row| row.contains("deploy.sh"))
        .expect("the listing shows");
    let line = alacritty_terminal::index::Line(i32::try_from(listing).unwrap());
    let point =
        |col| alacritty_terminal::index::Point::new(line, alacritty_terminal::index::Column(col));
    let _ = state.update(Message::Press {
        at: point(8),
        side: alacritty_terminal::index::Side::Left,
        click: Click::Single,
        extend: false,
    });
    let _ = state.update(Message::Drag {
        at: point(36),
        side: alacritty_terminal::index::Side::Right,
    });
    state
}

#[test]
fn a_shell_with_its_colours_wide_text_a_selection_and_a_match() {
    let state = shell();
    assert!(state.has_selection());
    assert_eq!(state.search().counted(), (1, 1, false));
    for (name, language, theme) in [
        ("terminal-shell-light-en", "en", Theme::Light),
        ("terminal-shell-dark-en", "en", Theme::Dark),
        ("terminal-shell-light-ar", "ar", Theme::Light),
    ] {
        assert_snapshot(name, language, &theme, SIZE, || {
            on_page(view(&state, Which::Shell))
        });
    }
}

#[test]
fn a_full_screen_program_s_frame_in_both_themes() {
    // Room for the 80 by 24 the frame was recorded at.
    let mut state = Terminals::default().app;
    fit(&mut state, Which::App, APP_SIZE);
    assert_eq!(state.size(), (80, 24));
    for (name, theme) in [
        ("terminal-app-light-en", Theme::Light),
        ("terminal-app-dark-en", Theme::Dark),
    ] {
        assert_snapshot(name, "en", &theme, APP_SIZE, || {
            on_page(view(&state, Which::App))
        });
    }
}

/// What real programs drew, recorded under a pty at 80 by 24, on a
/// terminal of that size: the closest these tests come to a container.
#[test]
fn recorded_programs_as_they_drew_themselves() {
    for (name, theme) in [
        ("vim", Theme::Light),
        ("less", Theme::Light),
        ("top", Theme::Light),
        ("tmux", Theme::Light),
        ("vim", Theme::Dark),
    ] {
        let mut state = State::new();
        fit(&mut state, Which::App, APP_SIZE);
        assert_eq!(state.size(), (80, 24));
        let _ = state.feed_bytes(&super::fixture(&format!("{name}.bin")));
        let look = if theme.is_dark() { "dark" } else { "light" };
        let snapshot = format!("terminal-{name}-{look}-en");
        assert_snapshot(&snapshot, "en", &theme, APP_SIZE, || {
            on_page(view(&state, Which::App))
        });
    }
}

/// Rule 6 of the migration guide: the search bar draws no literal. Its
/// "Aa" and ".*" are the log viewer's, the exception the rule names.
#[test]
fn the_search_bar_draws_no_literal_of_its_own() {
    let source = include_str!("../toolbar.rs");
    let lookup: String = ['t', '!', '('].iter().collect();
    let code: String = source
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    let mut literals = vec![];
    let mut rest = code.as_str();
    while let Some(start) = rest.find('"') {
        let before = &rest[..start];
        let after = &rest[start + 1..];
        let end = after.find('"').expect("a closed literal");
        if !before.trim_end().ends_with(&lookup) {
            literals.push(after[..end].to_owned());
        }
        rest = &after[end + 1..];
    }
    assert_eq!(literals, Vec::<String>::new());
}

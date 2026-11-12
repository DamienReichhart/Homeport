//! The viewer drawn: in both themes, wrapped and not, with a search, with
//! a program's colours, paused, filtered, and under Arabic chrome.

use iced::{widget::container, Length};

use super::feed;
use crate::{
    test_support::snapshot::assert_snapshot,
    ui::{
        theme::{self, Element, Theme},
        widgets::{
            gallery::{
                self,
                log::{filled, sample, view},
            },
            log::{Line, Mark, Message, Metrics, Scroll, State, Stream},
        },
    },
};

type Page = gallery::Message;

const SIZE: (f32, f32) = (900.0, 540.0);

fn on_page(part: Element<'_, Page>) -> Element<'_, Page> {
    container(part)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::page)
        .into()
}

fn tell(state: &mut State, messages: impl IntoIterator<Item = Message>) {
    for message in messages {
        let _ = state.update(message);
    }
}

/// The sample, in a viewer that knows about how much room it has, as
/// after its first layout.
fn sampled() -> State {
    let mut state = filled(1000, sample());
    tell(
        &mut state,
        [Message::Viewport(Metrics {
            cols: 116,
            rows: 24,
        })],
    );
    state
}

#[test]
fn the_sample_wrapped_in_both_themes() {
    let state = sampled();
    for (name, theme) in [
        ("log-sample-en", Theme::Light),
        ("log-sample-dark-en", Theme::Dark),
    ] {
        assert_snapshot(name, "en", &theme, SIZE, || on_page(view(&state)));
    }
}

#[test]
fn under_arabic_chrome_the_text_still_reads_left_to_right() {
    let mut state = sampled();
    // Paused, so the way back shows, at the end of the row as Arabic reads.
    tell(&mut state, [Message::Scroll(Scroll::Rows(-2))]);
    assert_snapshot("log-sample-ar", "ar", &Theme::Light, SIZE, || {
        on_page(view(&state))
    });
}

#[test]
fn not_wrapped_long_lines_are_cut_and_scroll_sideways() {
    let mut state = sampled();
    tell(&mut state, [Message::Wrap(false)]);
    assert_snapshot("log-nowrap-en", "en", &Theme::Light, SIZE, || {
        on_page(view(&state))
    });
    tell(&mut state, [Message::Scroll(Scroll::Across(40))]);
    assert_snapshot("log-nowrap-across-en", "en", &Theme::Light, SIZE, || {
        on_page(view(&state))
    });
}

#[test]
fn without_timestamps_the_lines_start_at_their_source() {
    let mut state = sampled();
    tell(&mut state, [Message::Timestamps(false)]);
    assert_snapshot("log-no-timestamps-en", "en", &Theme::Dark, SIZE, || {
        on_page(view(&state))
    });
}

#[test]
fn the_matches_of_a_search_are_marked_and_the_current_one_stands_out() {
    for (name, theme) in [
        ("log-search-en", Theme::Light),
        ("log-search-dark-en", Theme::Dark),
    ] {
        let mut state = sampled();
        tell(
            &mut state,
            [
                Message::Query("re".to_owned()),
                Message::Next,
                Message::Next,
                Message::Next,
            ],
        );
        assert!(state.matches().0.is_some());
        assert_snapshot(name, "en", &theme, SIZE, || on_page(view(&state)));
    }
}

#[test]
fn filtered_only_the_lines_that_match_show() {
    let mut state = sampled();
    tell(
        &mut state,
        [
            Message::Regex(true),
            Message::Query("ERROR|WARN".to_owned()),
            Message::Filter(true),
        ],
    );
    assert_snapshot("log-filter-en", "en", &Theme::Light, SIZE, || {
        on_page(view(&state))
    });
}

#[test]
fn a_query_that_cannot_be_read_says_so_in_the_field() {
    let mut state = sampled();
    tell(
        &mut state,
        [Message::Regex(true), Message::Query("(unclosed".to_owned())],
    );
    assert!(state.search_error().is_some());
    assert_snapshot("log-invalid-regex-en", "en", &Theme::Light, SIZE, || {
        on_page(view(&state))
    });
}

#[test]
fn paused_with_a_selection_and_the_way_back_to_the_latest_line() {
    let mut state = sampled();
    tell(
        &mut state,
        [
            Message::Scroll(Scroll::Rows(-3)),
            Message::Press {
                at: Mark::new(2, 31),
                extend: false,
            },
            Message::Drag(Mark::new(5, 60)),
        ],
    );
    assert!(!state.follows());
    assert_snapshot("log-paused-en", "en", &Theme::Light, SIZE, || {
        on_page(view(&state))
    });
}

/// Every colour and emphasis a program can ask for.
fn colours() -> State {
    let mut state = State::new(100);
    let row = |codes: std::ops::RangeInclusive<u8>, text: &dyn Fn(u8) -> String| -> Line {
        Line::new(codes.map(text).collect::<String>() + "\x1b[0m")
    };
    feed(
        &mut state,
        [
            Line::new("the 16, as text and behind it:"),
            row(30..=37, &|n| format!("\x1b[{n}m colour{} ", n - 30)),
            row(90..=97, &|n| format!("\x1b[{n}m bright{} ", n - 90)),
            row(40..=47, &|n| format!("\x1b[{n}m colour{} ", n - 40)),
            row(100..=107, &|n| format!("\x1b[{n}m bright{} ", n - 100)),
            Line::new("the cube and the greys of the 256, as text:"),
            row(16..=51, &|n| format!("\x1b[38;5;{n}m{n:>4}")),
            row(196..=231, &|n| format!("\x1b[38;5;{n}m{n:>4}")),
            row(232..=255, &|n| format!("\x1b[38;5;{n}m{n:>4}")),
            Line::new("truecolour, as text and behind it:"),
            row(0..=47, &|n| {
                format!("\x1b[38;2;{};{};255m#", n * 5, 255 - n * 5)
            }),
            row(0..=47, &|n| {
                format!("\x1b[48;2;{};{};90m ", n * 5, 255 - n * 5)
            }),
            Line::new(
                "\x1b[1mbold\x1b[0m \x1b[2mdim\x1b[0m \x1b[3mitalic\x1b[0m \
                 \x1b[4munderline\x1b[0m \x1b[7minverse\x1b[0m \x1b[9mstrike\x1b[0m \
                 \x1b[1;31mbold red\x1b[0m \x1b[2;32mdim green\x1b[0m \
                 \x1b[3;4;34mitalic underlined blue\x1b[0m \x1b[7;33minverse yellow\x1b[0m",
            ),
            Line::new("\x1b[31mred that goes on"),
            Line::new("to the next line\x1b[0m, then stops"),
            Line::new("stderr is tinted, whatever its colours: \x1b[32mok\x1b[0m")
                .stream(Stream::Stderr),
        ],
    );
    state
}

#[test]
fn a_programs_colours_read_in_both_themes() {
    let state = colours();
    for (name, theme) in [
        ("log-ansi-en", Theme::Light),
        ("log-ansi-dark-en", Theme::Dark),
    ] {
        assert_snapshot(name, "en", &theme, SIZE, || on_page(view(&state)));
    }
}

#[test]
fn the_end_of_a_log_and_lines_never_shown_are_said_between_the_lines() {
    let mut state = State::new(100);
    feed(&mut state, [Line::new("first"), Line::new("second")]);
    state.skipped(1234);
    feed(&mut state, [Line::new("after the gap")]);
    state.end();
    assert_snapshot(
        "log-markers-en",
        "en",
        &Theme::Light,
        (700.0, 300.0),
        || on_page(view(&state)),
    );
}

#[test]
fn a_stream_that_breaks_is_explained_over_the_lines_that_came() {
    use crate::ui::widgets::{feedback::error_message, log::Viewer};
    let mut state = State::new(100);
    feed(
        &mut state,
        [
            Line::new("listening on :80"),
            Line::new("warning: slow").stream(Stream::Stderr),
        ],
    );
    let failure = gallery::failure();
    assert_snapshot("log-failed-en", "en", &Theme::Light, (800.0, 380.0), || {
        on_page(
            Viewer::new(&state, Page::Log)
                .failed(Some(error_message(&failure, true, Page::Failure)))
                .view(),
        )
    });
}

/// The gallery's own page, where the viewer is in a page that scrolls.
#[test]
fn the_gallery_shows_the_viewer_on_its_log_page() {
    let gallery = gallery::Gallery {
        page: gallery::Page::Log,
        ..gallery::Gallery::default()
    };
    assert_snapshot(
        "widgets-log-light-en",
        "en",
        &Theme::Light,
        (1100.0, 640.0),
        || gallery.view(),
    );
}

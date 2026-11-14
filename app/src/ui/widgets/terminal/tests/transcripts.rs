//! Escape sequences, against what real programs wrote to a terminal of
//! 80 by 24: recorded under a pty with `TERM=xterm-256color`
//! (`app/tests/fixtures/terminal`). htop is not on the machine the
//! transcripts were recorded on: `top`, and a curses program that draws
//! htop's meters, header and rows (`meters.bin`), stand for it.

use alacritty_terminal::{
    term::{cell::Flags, TermMode},
    vte::ansi::{Color, NamedColor},
};

use super::{answers, cell, fixture, screen, sized};
use crate::ui::widgets::terminal::{Event, Message, State};

fn played(name: &str) -> (State, Vec<Event>) {
    let mut state = sized(80, 24);
    let events = state.feed_bytes(&fixture(name));
    (state, events)
}

fn named(colour: NamedColor) -> Color {
    Color::Named(colour)
}

#[test]
fn vim_starts_on_the_alternate_screen_with_its_colours_and_its_status_line() {
    let (state, events) = played("vim.bin");
    let mode = state.mode();
    for asked in [
        TermMode::ALT_SCREEN,
        TermMode::BRACKETED_PASTE,
        TermMode::APP_CURSOR,
        TermMode::FOCUS_IN_OUT,
    ] {
        assert!(mode.contains(asked), "{asked:?} in {mode:?}");
    }
    let rows = screen(&state);
    assert_eq!(rows[0], "  1 #!/usr/bin/env python3");
    assert_eq!(rows[3], "  4 import sys");
    assert_eq!(rows[6], "  7 def greet(name: str) -> str:");
    assert_eq!(rows[13], "~");
    assert!(rows[22].starts_with("~/hello.py") && rows[22].ends_with("All"));
    assert_eq!(rows[23], "\"~/hello.py\" 13L, 234B");
    // Line numbers in colour 130 of the 256, `import` in magenta, `greet`
    // and `str` in cyan, the comment in blue.
    assert_eq!(cell(&state, 0, 2).fg, Color::Indexed(130));
    assert_eq!(cell(&state, 3, 4).fg, named(NamedColor::Magenta));
    assert_eq!(cell(&state, 6, 8).fg, named(NamedColor::Cyan));
    assert_eq!(cell(&state, 6, 20).fg, named(NamedColor::Cyan));
    assert_eq!(cell(&state, 6, 19).fg, named(NamedColor::Foreground));
    assert_eq!(cell(&state, 7, 8).fg, named(NamedColor::Blue));
    // The status line is drawn reversed and bold.
    assert!(cell(&state, 22, 0)
        .flags
        .contains(Flags::INVERSE | Flags::BOLD));
    // vim asked where the cursor was, which terminal this is, and its
    // colours; each was answered, the colours the theme's.
    assert_eq!(
        answers(&events),
        "\x1b[2;2R\x1b[3;1R\x1b[>0;2600;1c\x1b]10;rgb:1414/1414/1414\x07\x1b]11;rgb:ffff/ffff/ffff\x07"
    );
    // The cursor waits on the first letter of the file.
    let cursor = state.term().grid().cursor.point;
    assert_eq!((cursor.line.0, cursor.column.0), (0, 4));
}

#[test]
fn a_program_that_asks_the_colours_of_a_dark_terminal_is_told_them() {
    let mut state = sized(80, 24);
    let _ = state.update(Message::Dark(true));
    let events = state.feed("\x1b]10;?\x07\x1b]11;?\x07");
    assert_eq!(
        answers(&events),
        "\x1b]10;rgb:f7f7/f7f7/f4f4\x07\x1b]11;rgb:0f0f/0f0f/0f0f\x07"
    );
}

#[test]
fn less_shows_its_page_and_marks_what_it_found() {
    let (state, _) = played("less.bin");
    assert!(state.mode().contains(TermMode::ALT_SCREEN));
    let rows = screen(&state);
    assert_eq!(
        rows[0],
        "line 042: the quick brown fox jumps over the lazy dog"
    );
    assert_eq!(
        rows[22],
        "line 064: the quick brown fox jumps over the lazy dog"
    );
    assert_eq!(rows[23], ":");
    // The match is in standout, the rest of the line is not.
    for col in 5..8 {
        assert!(
            cell(&state, 0, col).flags.contains(Flags::INVERSE),
            "cell {col}"
        );
    }
    assert!(!cell(&state, 0, 9).flags.contains(Flags::INVERSE));
}

#[test]
fn top_draws_its_summary_in_bold_and_its_header_reversed() {
    let (state, events) = played("top.bin");
    assert!(events.is_empty());
    // top hides the cursor and stays on the main screen.
    assert!(!state.mode().contains(TermMode::SHOW_CURSOR));
    assert!(!state.mode().contains(TermMode::ALT_SCREEN));
    let rows = screen(&state);
    assert!(rows[0].starts_with("top - "));
    assert!(rows[1].starts_with("Tasks:  80 total,"));
    assert!(rows[6].trim_start().starts_with("PID USER"));
    assert_eq!(rows.iter().filter(|row| row.contains(" root ")).count(), 17);
    // "80" is bold, "total," is not.
    assert!(cell(&state, 1, 9).flags.contains(Flags::BOLD));
    assert!(!cell(&state, 1, 12).flags.contains(Flags::BOLD));
    assert!(cell(&state, 6, 2).flags.contains(Flags::INVERSE));
}

#[test]
fn tmux_draws_its_status_line_in_its_colours_and_is_answered() {
    let (state, events) = played("tmux.bin");
    let rows = screen(&state);
    assert_eq!(rows[0], "# echo hello from tmux");
    assert_eq!(rows[1], "hello from tmux");
    assert!(rows[23].starts_with("[hp] 0:shell*"));
    let status = cell(&state, 23, 0);
    assert_eq!(status.fg, named(NamedColor::Black));
    assert_eq!(status.bg, named(NamedColor::Green));
    // Padded to the end of the line in the same colours.
    assert_eq!(cell(&state, 23, 40).bg, named(NamedColor::Green));
    assert_eq!(
        answers(&events),
        "\x1b[?6c\x1b[>0;2600;1c\x1b]10;rgb:1414/1414/1414\x1b\\\x1b]11;rgb:ffff/ffff/ffff\x1b\\"
    );
}

#[test]
fn an_htop_like_frame_has_its_bars_header_selection_and_wide_text() {
    let (state, _) = played("meters.bin");
    assert!(state.mode().contains(TermMode::ALT_SCREEN));
    let rows = screen(&state);
    assert!(rows[0].starts_with("  0[||||||||||||||||||||||") && rows[0].contains("66.0%]"));
    assert!(rows[4].starts_with(" Mem["));
    // Green, then red, bars; the memory bar's third part in colour 208.
    assert_eq!(cell(&state, 0, 4).fg, named(NamedColor::Green));
    assert_eq!(cell(&state, 0, 22).fg, named(NamedColor::Red));
    assert_eq!(cell(&state, 4, 33).fg, Color::Indexed(208));
    // The header: black on green across the line.
    assert_eq!(cell(&state, 6, 0).bg, named(NamedColor::Green));
    assert_eq!(cell(&state, 6, 79).bg, named(NamedColor::Green));
    // The row the cursor is on is reversed.
    assert!(cell(&state, 8, 0).flags.contains(Flags::INVERSE));
    assert!(!cell(&state, 9, 0).flags.contains(Flags::INVERSE));
    // A wide character takes two cells, the second a spacer.
    let wide = rows[10]
        .find('日')
        .map(|byte| rows[10][..byte].chars().count())
        .unwrap();
    assert!(cell(&state, 10, wide).flags.contains(Flags::WIDE_CHAR));
    assert!(cell(&state, 10, wide + 1)
        .flags
        .contains(Flags::WIDE_CHAR_SPACER));
    assert_eq!(rows[23], "F1Help  F10Quit");
}

#[test]
fn utf8_emoji_cjk_and_combining_marks_take_their_cells() {
    let (state, _) = played("utf8.bin");
    let rows = screen(&state);
    assert_eq!(
        rows[6],
        "CJK: 日本語のテキスト 中文 한국어 | Emoji: 🚀 ✅ 🐳 | Combining: e\u{301} n\u{303} a\u{30a}"
    );
    assert_eq!(rows[7], "Box: ┌─┬─┐ Wide|日|end");
    // 日 is two cells: "CJK: " then 日 at 5, 本 at 7.
    assert_eq!(cell(&state, 6, 5).c, '日');
    assert!(cell(&state, 6, 5).flags.contains(Flags::WIDE_CHAR));
    assert_eq!(cell(&state, 6, 7).c, '本');
    // An accent joins the letter before it, in the same cell.
    let e = rows[6].chars().take_while(|c| *c != '\u{301}').count() - 1;
    let wide_before = rows[6]
        .chars()
        .take(e)
        .filter(|c| unicode_width::UnicodeWidthChar::width(*c) == Some(2))
        .count();
    let col = e + wide_before;
    assert_eq!(cell(&state, 6, col).c, 'e');
    assert_eq!(cell(&state, 6, col).zerowidth(), Some(&['\u{301}'][..]));
    // The attributes of the first line.
    let at = |text: &str| rows[0].find(text).unwrap();
    assert!(cell(&state, 0, at("bold")).flags.contains(Flags::BOLD));
    assert!(cell(&state, 0, at("italic")).flags.contains(Flags::ITALIC));
    assert!(cell(&state, 0, at("underline"))
        .flags
        .contains(Flags::UNDERLINE));
    assert!(cell(&state, 0, at("inverse"))
        .flags
        .contains(Flags::INVERSE));
    assert!(cell(&state, 0, at("strike"))
        .flags
        .contains(Flags::STRIKEOUT));
    assert!(cell(&state, 0, at("dim")).flags.contains(Flags::DIM));
    assert!(cell(&state, 0, 0).flags.is_empty());
    // The 16, the 256 and 24-bit colours.
    assert_eq!(cell(&state, 1, 5).fg, named(NamedColor::Red));
    assert_eq!(cell(&state, 2, 5).fg, named(NamedColor::BrightRed));
    assert_eq!(cell(&state, 3, 1).bg, named(NamedColor::BrightBlack));
    assert_eq!(cell(&state, 4, 0).bg, Color::Indexed(16));
    assert_eq!(cell(&state, 4, 30).bg, Color::Indexed(46));
    let truecolor = rows[5].find("truecolor").unwrap();
    assert!(
        matches!(cell(&state, 5, truecolor).fg, Color::Spec(rgb) if (rgb.r, rgb.g, rgb.b) == (255, 120, 0))
    );
}

#[test]
fn a_transcript_read_a_byte_at_a_time_draws_what_it_draws_whole() {
    for name in ["vim.bin", "utf8.bin", "meters.bin"] {
        let (whole, _) = played(name);
        let mut bytewise = sized(80, 24);
        for byte in fixture(name) {
            let _ = bytewise.feed_bytes(&[byte]);
        }
        assert_eq!(screen(&bytewise), screen(&whole), "{name}");
    }
}

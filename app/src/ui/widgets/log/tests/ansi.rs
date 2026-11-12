//! What the parser makes of a terminal's text.

use crate::ui::widgets::log::ansi::{
    strip, Attrs, Colour, Parser, Run, Style, CSI_MAX, STRING_MAX,
};

/// The text and the runs of `input`, read in one go by a new parser.
fn read(input: &str) -> (String, Vec<Run>) {
    let mut text = String::new();
    let mut runs = vec![];
    Parser::default().feed(input, &mut text, 0, &mut runs);
    (text, runs)
}

/// The style of the first run.
fn style_of(input: &str) -> Style {
    read(input).1.first().expect("a run").style
}

fn fg(colour: Colour) -> Style {
    Style {
        fg: colour,
        ..Style::default()
    }
}

#[test]
fn text_without_sequences_is_itself_and_plain() {
    assert_eq!(
        read("listening on :80"),
        ("listening on :80".to_owned(), vec![])
    );
    assert_eq!(read("").0, "");
    assert_eq!(read("día ñ 日本語 🚀").0, "día ñ 日本語 🚀");
}

#[test]
fn the_sixteen_colours_and_their_bright_forms() {
    for n in 0..8_u8 {
        assert_eq!(
            style_of(&format!("\x1b[{}mx", 30 + n)),
            fg(Colour::Indexed(n))
        );
        assert_eq!(
            style_of(&format!("\x1b[{}mx", 90 + n)),
            fg(Colour::Indexed(8 + n))
        );
        let bg = |colour| Style {
            bg: colour,
            ..Style::default()
        };
        assert_eq!(
            style_of(&format!("\x1b[{}mx", 40 + n)),
            bg(Colour::Indexed(n))
        );
        assert_eq!(
            style_of(&format!("\x1b[{}mx", 100 + n)),
            bg(Colour::Indexed(8 + n))
        );
    }
}

#[test]
fn the_256_colours_and_truecolour_with_semicolons_or_colons() {
    assert_eq!(style_of("\x1b[38;5;208mx"), fg(Colour::Indexed(208)));
    assert_eq!(style_of("\x1b[38:5:208mx"), fg(Colour::Indexed(208)));
    assert_eq!(
        style_of("\x1b[38;2;10;20;30mx"),
        fg(Colour::Rgb(10, 20, 30))
    );
    assert_eq!(
        style_of("\x1b[38:2:10:20:30mx"),
        fg(Colour::Rgb(10, 20, 30))
    );
    assert_eq!(
        style_of("\x1b[38:2::10:20:30mx"),
        fg(Colour::Rgb(10, 20, 30))
    );
    let both = style_of("\x1b[1;38;5;9;48;2;1;2;3mx");
    assert_eq!(both.fg, Colour::Indexed(9));
    assert_eq!(both.bg, Colour::Rgb(1, 2, 3));
    assert!(both.attrs.has(Attrs::BOLD));
    // A colour that is not one: the sequence is dropped from there on,
    // and what came before it holds.
    assert_eq!(style_of("\x1b[4;38;5;999;31mx").fg, Colour::Default);
    assert!(style_of("\x1b[4;38;7mx").attrs.has(Attrs::UNDERLINE));
    assert_eq!(read("\x1b[38mx").1, vec![]);
}

#[test]
fn emphasis_is_set_and_taken_away() {
    let all = style_of("\x1b[1;2;3;4;7;9mx").attrs;
    for attr in [
        Attrs::BOLD,
        Attrs::DIM,
        Attrs::ITALIC,
        Attrs::UNDERLINE,
        Attrs::INVERSE,
        Attrs::STRIKE,
    ] {
        assert!(all.has(attr));
    }
    // 22 takes bold and dim away, and each of the others has its own.
    assert_eq!(read("\x1b[1;2;3;4;7;9m\x1b[22;23;24;27;29mx").1, vec![]);
    assert!(!style_of("\x1b[1;4m\x1b[24mx").attrs.has(Attrs::UNDERLINE));
    assert!(style_of("\x1b[4:3mx").attrs.has(Attrs::UNDERLINE), "curly");
    assert_eq!(read("\x1b[4m\x1b[4:0mx").1, vec![]);
    // What a log can't show is ignored, not mistaken for something else.
    assert_eq!(read("\x1b[5;8;53mx").1, vec![]);
}

#[test]
fn reset_makes_text_plain_again() {
    for reset in ["\x1b[0m", "\x1b[m", "\x1b[;m"] {
        let (text, runs) = read(&format!("\x1b[31;1merror{reset} done"));
        assert_eq!(text, "error done");
        assert_eq!(runs.len(), 1, "{reset:?}");
        assert_eq!((runs[0].start, runs[0].end), (0, 5));
    }
    // 39 and 49 give back the terminal's own colours alone.
    let kept = style_of("\x1b[31;42;1m\x1b[39;49mx");
    assert_eq!((kept.fg, kept.bg), (Colour::Default, Colour::Default));
    assert!(kept.attrs.has(Attrs::BOLD));
}

#[test]
fn runs_cover_the_bytes_of_their_text_and_join_when_alike() {
    let (text, runs) = read("a \x1b[32mgrün\x1b[32m ok\x1b[0m b \x1b[4mc\x1b[0m");
    assert_eq!(text, "a grün ok b c");
    let green = Run {
        start: 2,
        end: 10,
        style: fg(Colour::Indexed(2)),
    };
    assert_eq!(runs[0], green, "the same style twice is one run");
    assert_eq!(&text[2..10], "grün ok");
    assert_eq!((runs[1].start, runs[1].end), (13, 14));
}

#[test]
fn every_other_sequence_is_taken_out() {
    for (input, text) in [
        ("\x1b[2Kcleared", "cleared"),
        ("up\x1b[1A\x1b[10;20Hthere", "upthere"),
        ("\x1b[?25lhidden\x1b[?25h", "hidden"),
        ("\x1b]0;a title\x07after", "after"),
        (
            "\x1b]8;;https://example.com\x1b\\link\x1b]8;;\x1b\\!",
            "link!",
        ),
        ("\x1bPq#0;2;0;0;0\x1b\\sixel", "sixel"),
        ("\x1b_app command\x1b\\x", "x"),
        ("\x1b(Bcharset", "charset"),
        ("\x1b7saved\x1b8", "saved"),
        ("\x1bcreset", "reset"),
        ("\u{9b}31mC1\u{9b}0m", "C1"),
        ("bell\x07 and \x08backspace", "bell and backspace"),
        ("\x1b[>4;2mprivate", "private"),
        ("\x1b[1 qcursor", "cursor"),
    ] {
        assert_eq!(strip(input), text, "{input:?}");
    }
    // A private sequence that ends in `m` is not a colour.
    assert_eq!(read("\x1b[?1mx").1, vec![]);
    assert_eq!(read("\x1b[>4;2mx").1, vec![]);
}

#[test]
fn tabs_are_spaces_to_the_next_stop_and_a_return_starts_the_line_over() {
    assert_eq!(strip("a\tb"), "a       b");
    assert_eq!(strip("12345678\tb"), "12345678        b");
    assert_eq!(strip("\tx"), "        x");
    // A progress bar keeps its last state; `\r\n` is only a line's end.
    assert_eq!(strip("10 %\r50 %\r100 %"), "100 %");
    assert_eq!(strip("done\r"), "done");
    let (text, runs) = read("\x1b[31mred\r\x1b[32mgreen");
    assert_eq!(text, "green");
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].style, fg(Colour::Indexed(2)));
}

#[test]
fn what_is_malformed_is_dropped_without_the_text_after_it() {
    // Not a sequence after all: the text shows.
    assert_eq!(strip("\x1b[31\u{e9}t\u{e9}"), "t\u{e9}");
    // A lone escape takes one byte with it, as in a terminal.
    assert_eq!(strip("a\x1b"), "a");
    assert_eq!(strip("\x1b\x1b[31mx"), "x");
    // Too many parameters: read to its end, and nothing of it applies.
    let long = format!("\x1b[{}mafter", "1;".repeat(CSI_MAX));
    assert_eq!(read(&long), ("after".to_owned(), vec![]));
    // A title that never ends is given up, and text is text again.
    let endless = format!("\x1b]0;{}tail", "x".repeat(2 * STRING_MAX));
    let left = strip(&endless);
    assert!(
        left.ends_with("xtail") && left.len() < STRING_MAX + 8,
        "{}",
        left.len()
    );
    // Numbers too large to be anything.
    assert_eq!(
        read("\x1b[99999999999999999999mx"),
        ("x".to_owned(), vec![])
    );
    assert_eq!(read("\x1b[38;2;300;0;0mx").1, vec![]);
}

/// Runs that touch and are alike, as one: two pieces of a line read one
/// after the other each start their own run.
fn joined(runs: Vec<Run>) -> Vec<Run> {
    let mut all: Vec<Run> = vec![];
    for run in runs {
        match all.last_mut() {
            Some(last) if last.end == run.start && last.style == run.style => last.end = run.end,
            _ => all.push(run),
        }
    }
    all
}

/// Everything a line may hold, cut in two at every byte: the two pieces
/// read one after the other give what the whole gives.
#[test]
fn a_sequence_cut_anywhere_by_the_end_of_a_chunk_is_finished_by_the_next() {
    let whole = "a\x1b[1;38;2;10;20;30mb\x1b]8;;http://x\x1b\\c\x1b[0m\u{e9}\x1b[4:3md\x1b(Be\tf";
    let (text, runs) = read(whole);
    assert_eq!(text, "abc\u{e9}de  f");
    for cut in (0..=whole.len()).filter(|cut| whole.is_char_boundary(*cut)) {
        let mut parser = Parser::default();
        let (mut cut_text, mut cut_runs) = (String::new(), vec![]);
        parser.feed(&whole[..cut], &mut cut_text, 0, &mut cut_runs);
        parser.feed(&whole[cut..], &mut cut_text, 0, &mut cut_runs);
        assert_eq!(cut_text, text, "cut at {cut}");
        assert_eq!(joined(cut_runs), runs, "cut at {cut}");
    }
}

#[test]
fn a_colour_holds_from_one_line_to_the_next_until_it_is_reset() {
    let mut parser = Parser::default();
    let mut text = String::new();
    let mut runs = vec![];
    parser.feed("\x1b[31mpanic:", &mut text, 0, &mut runs);
    assert_eq!(parser.style(), fg(Colour::Indexed(1)));
    // The second line starts where the first ended in the chunk's text.
    let start = text.len();
    let first = runs.len();
    parser.feed("  at main.go:12\x1b[0m ok", &mut text, start, &mut runs);
    assert_eq!(&text[start..], "  at main.go:12 ok");
    assert_eq!(runs.len(), first + 1);
    let run = runs[first];
    assert_eq!((run.start, run.end), (0, 15), "counted from its own line");
    assert_eq!(run.style, fg(Colour::Indexed(1)));
    assert!(parser.style().is_plain());
}

#[test]
fn a_code_too_large_to_be_one_changes_nothing() {
    // It was read as 0, a reset of everything before it.
    assert_eq!(style_of("\x1b[31m\x1b[99999mx"), fg(Colour::Indexed(1)));
    assert_eq!(
        style_of("\x1b[31;99999999999999999999;4mx").fg,
        Colour::Indexed(1)
    );
    // Left out, a code is still 0.
    assert_eq!(read("\x1b[31m\x1b[;mx").1, vec![]);
}

#[test]
fn the_colour_of_an_underline_is_read_and_not_taken_for_codes() {
    // `58;5;1` is an underline in red: its 5 is not a blink nor its 1 bold.
    assert_eq!(read("\x1b[58;5;1mx").1, vec![]);
    assert_eq!(read("\x1b[58:5:1mx").1, vec![]);
    // `58;2;1;2;3` takes five parameters: 1, 2 and 3 are not bold, dim, italic.
    assert_eq!(style_of("\x1b[58;2;1;2;3;31mx"), fg(Colour::Indexed(1)));
    assert_eq!(style_of("\x1b[58:2::1:2:3;31mx"), fg(Colour::Indexed(1)));
}

#[test]
fn a_tab_goes_to_the_next_stop_in_cells() {
    // Two wide characters are four cells: four more to the stop.
    assert_eq!(strip("日本\tb"), "日本    b");
    assert_eq!(strip("e\u{301}\tb"), "e\u{301}       b");
    // Every tab of a line of nothing else is eight cells, however many.
    let tabs = "\t".repeat(20_000);
    assert_eq!(strip(&tabs).len(), 160_000);
    // After a carriage return the stops start over.
    assert_eq!(strip("abc\r\tb"), "        b");
}

#[test]
fn a_title_that_never_ends_takes_at_most_its_limit() {
    let input = format!("\x1b]{}tail", "x".repeat(10_000));
    let kept = strip(&input);
    let taken = 10_000 + 4 - kept.len();
    assert!(taken <= STRING_MAX + 1, "{taken} bytes taken");
    assert!(kept.ends_with("tail"));
}

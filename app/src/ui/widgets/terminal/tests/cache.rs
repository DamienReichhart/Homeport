//! What a frame builds again, counted on a window of its own: a row that
//! changed is built again and the others are taken as they were; another
//! theme works the colours out again; another size of text builds and
//! shapes every row anew; and the colours kept stay bounded under a
//! program of 24-bit colours.

use super::perf::Bench;
use crate::ui::{
    theme::Theme,
    widgets::terminal::{
        pane::{
            draw::{COLOURED, COLOURS_KEPT, HELD},
            rows::{BUILT, SHAPED},
        },
        Message, State,
    },
};

fn counted<T>(
    counter: &'static std::thread::LocalKey<std::cell::Cell<usize>>,
    run: impl FnOnce() -> T,
) -> usize {
    let before = counter.with(std::cell::Cell::get);
    run();
    counter.with(std::cell::Cell::get) - before
}

/// A frame of `state` in `theme`, its messages handed back.
fn frame(bench: &mut Bench, state: &mut State, theme: &Theme) {
    let (_, messages) = bench.frame_in(state, theme);
    for message in messages {
        let _ = state.update(message);
    }
}

#[test]
fn only_what_changed_is_built_again_and_a_new_size_of_text_builds_all() {
    let mut bench = Bench::new();
    let mut state = State::new();
    bench.settle(&mut state);
    let lines: String = (0..30)
        .map(|i| format!("line {i} \x1b[31mred {i}\x1b[0m\r\n"))
        .collect();
    let _ = state.feed(&lines);
    frame(&mut bench, &mut state, &Theme::Light);
    let rows = state.size().1;
    // Nothing changed: nothing is built, nothing shaped.
    let light = Theme::Light;
    assert_eq!(counted(&BUILT, || frame(&mut bench, &mut state, &light)), 0);
    assert_eq!(
        counted(&SHAPED, || frame(&mut bench, &mut state, &light)),
        0
    );
    // One row written over: that row only.
    let _ = state.feed("\x1b[3;1H\x1b[2Kchanged");
    let built = counted(&BUILT, || frame(&mut bench, &mut state, &light));
    assert_eq!(built, 1);
    // Another theme: no row is built again, the colours are.
    let dark = Theme::Dark;
    assert_eq!(counted(&BUILT, || frame(&mut bench, &mut state, &dark)), 0);
    let _ = state.update(Message::Dark(false));
    assert!(counted(&COLOURED, || frame(&mut bench, &mut state, &light)) > 0);
    // Another size of text: every row, shaped anew.
    state.set_font_size(15.0);
    let mut shaped = 0;
    let built = counted(&BUILT, || {
        shaped = counted(&SHAPED, || frame(&mut bench, &mut state, &light));
    });
    assert!(built >= rows.min(state.size().1), "{built} rows of {rows}");
    assert!(shaped > 0);
}

#[test]
fn the_colours_kept_stay_bounded_under_24_bit_colours() {
    let mut bench = Bench::new();
    let mut state = State::new();
    bench.settle(&mut state);
    let (cols, rows) = state.size();
    for screen in 0..(COLOURS_KEPT / (cols * rows) + 3) {
        let mut output = String::from("\x1b[H");
        for cell in 0..cols * rows {
            let n = screen * cols * rows + cell;
            let (r, g, b) = (n % 251, n / 251 % 251, n / 63_001 % 251);
            output.push_str(&format!("\x1b[38;2;{r};{g};{b}mx"));
        }
        let _ = state.feed(&output);
        frame(&mut bench, &mut state, &Theme::Light);
    }
    assert!(HELD.with(std::cell::Cell::get) <= COLOURS_KEPT);
}

//! A scroll step and a batch appended, among a million lines.

use std::time::{Duration, Instant};

use super::*;

/// Frames that each do `step` first: the middle one.
fn stepping(bench: &mut Bench, state: &mut State, step: impl Fn(usize) -> Scroll) -> Frame {
    bench.settle(state);
    median(
        (0..40)
            .map(|turn| {
                let start = Instant::now();
                let _ = state.update(Message::Scroll(step(turn)));
                let update = start.elapsed();
                let (mut frame, _) = bench.frame(state);
                frame.update = update;
                frame
            })
            .collect(),
    )
}

#[test]
#[ignore = "a measurement: run it alone with --nocapture and read what it prints"]
fn a_scroll_step_among_a_million_lines() {
    let _english = in_language("en");
    let mut bench = Bench::new();
    println!(
        "\nA window of {} by {}, software renderer, {}:",
        WINDOW.width,
        WINDOW.height,
        build()
    );
    let mut few = full(1000, true);
    let scrolling_few = scrolling(&mut bench, &mut few);
    report("1,000 lines of three rows, a scroll step", scrolling_few);

    // Lines of three rows each: every row in view is part of a line
    // that wraps.
    let mut state = full(MILLION, true);
    bench.settle(&mut state);
    assert!(
        state.rows_of(500_000, state.metrics.cols) >= 3,
        "the lines wrap"
    );
    let still = median((0..40).map(|_| bench.frame(&state).0).collect());
    report("1,000,000 lines of three rows, still", still);
    let wrapped = scrolling(&mut bench, &mut state);
    report("1,000,000 lines of three rows, a scroll step", wrapped);
    let paging = stepping(&mut bench, &mut state, |_| Scroll::Pages(1));
    report("the same, a page down (every row new)", paging);
    // The thumb dragged from one end to the other.
    #[allow(clippy::cast_precision_loss)]
    let dragging = stepping(&mut bench, &mut state, |turn| {
        Scroll::To(turn as f32 / 40.0)
    });
    report("the same, the thumb dragged", dragging);
    let _ = state.update(Message::Wrap(false));
    let unwrapped = scrolling(&mut bench, &mut state);
    report("the same lines not wrapped, a scroll step", unwrapped);
    drop(state);

    // Lines that fit a row: more lines in view, and more pieces.
    let mut state = full(MILLION, false);
    let fitting = scrolling(&mut bench, &mut state);
    report("1,000,000 lines that fit a row, a scroll step", fitting);
    let paging_fitting = stepping(&mut bench, &mut state, |_| Scroll::Pages(1));
    report("the same, a page down (every row new)", paging_fitting);

    within("a scroll step among lines that wrap", wrapped.own(), SCROLL);
    within("a scroll step, not wrapped", unwrapped.own(), SCROLL);
    within("a scroll step among lines that fit", fitting.own(), SCROLL);
    within("a page down among lines that wrap", paging.own(), FOLLOW);
    within(
        "a page down among lines that fit",
        paging_fitting.own(),
        FOLLOW,
    );
    within("the thumb dragged", dragging.own(), FOLLOW);
    // What makes it a virtual list: a thousand times the lines is not a
    // thousand times the work.
    assert!(
        wrapped.own() < scrolling_few.own() * 4,
        "1,000,000 lines cost {:?} a scroll step, 1,000 lines {:?}",
        wrapped.own(),
        scrolling_few.own()
    );
}

#[test]
#[ignore = "a measurement: run it alone with --nocapture and read what it prints"]
fn a_batch_appended_to_a_million_lines() {
    let mut state = full(MILLION, false);
    assert_eq!(state.len(), MILLION);
    println!("\n{}:", build());
    // The lines are made before the clock starts: they are the stream's.
    let mut next = MILLION;
    let mut timed = |state: &mut State, size: usize, turns: usize| {
        let times = (0..turns)
            .map(|_| {
                let lines: Vec<Line> = many(next, size).collect();
                next += size;
                let start = Instant::now();
                drop(state.extend(lines));
                start.elapsed()
            })
            .collect();
        middle(times)
    };
    let frame = timed(&mut state, BATCH, 120);
    println!(
        "{BATCH} lines appended to 1,000,000 (the oldest dropped):      {:>7.3} ms",
        ms(frame)
    );
    let second = timed(&mut state, 50_000, 10);
    println!(
        "50,000 lines appended at once (a second's worth):          {:>7.3} ms",
        ms(second)
    );
    // With a search on that has read everything, the new lines are read
    // for it as they come: the count of matches grows with them.
    searching(&mut state);
    let before = state.matches().1;
    let hits_before = state.search.hits.back();
    let searched = timed(&mut state, BATCH, 120);
    assert!(
        !state.search.busy && !state.is_searching(),
        "every line was read"
    );
    assert_ne!(state.search.hits.back(), hits_before, "new lines matched");
    // A quarter of the lines say ERROR, and as many were dropped as came.
    let matched = state.matches().1;
    assert!(matched.abs_diff(before) <= 2, "{before} then {matched}");
    println!(
        "{BATCH} lines appended and searched for \"ERROR\":               {:>7.3} ms",
        ms(searched)
    );
    assert_eq!(state.len(), MILLION, "the cap holds");
    within("a frame's worth of lines appended", frame, APPEND);
    within("the same with a search on", searched, APPEND * 2);
    drop(state);
    // Appending does not look at what is already there: a batch costs
    // the same in a buffer a thousand times smaller.
    let mut small = full(1000, false);
    let small_frame = timed(&mut small, BATCH, 120);
    println!(
        "{BATCH} lines appended to 1,000:                               {:>7.3} ms",
        ms(small_frame)
    );
    assert!(
        frame < small_frame * 4 + Duration::from_micros(200),
        "{frame:?} into a million lines, {small_frame:?} into a thousand"
    );
}

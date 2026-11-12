//! A view that follows a stream, and a search over a million lines.

use std::time::{Duration, Instant};

use super::*;

/// Two seconds of a stream of 50,000 lines a second, a frame at a time.
fn stream(
    bench: &mut Bench,
    state: &mut State,
    next: &mut usize,
    lines: fn(usize, usize) -> Vec<Line>,
) -> Vec<Frame> {
    (0..120)
        .map(|_| {
            let batch = lines(*next, BATCH);
            *next += BATCH;
            let start = Instant::now();
            drop(state.extend(batch));
            let update = start.elapsed();
            let (mut frame, _) = bench.frame(state);
            frame.update = update;
            frame
        })
        .collect()
}

/// The worst frame of `frames`, held to half a frame in the build that
/// ships and to two frames in one that is not optimised.
fn worst(what: &str, frames: &[Frame]) -> Duration {
    let worst = frames
        .iter()
        .map(|frame| frame.own())
        .max()
        .expect("frames");
    let allowed = if optimised() {
        WORST
    } else {
        WORST_UNOPTIMISED
    };
    assert!(
        worst <= allowed,
        "{what}: its worst frame took {:.3} ms of the viewer's own work, over {:.0} ms",
        ms(worst),
        ms(allowed)
    );
    worst
}

#[test]
#[ignore = "a measurement: run it alone with --nocapture and read what it prints"]
fn following_a_stream_of_fifty_thousand_lines_a_second() {
    let _english = in_language("en");
    let mut bench = Bench::new();
    println!("\n{}:", build());
    let fitting: fn(usize, usize) -> Vec<Line> = |from, count| many(from, count).collect();
    let wrapping: fn(usize, usize) -> Vec<Line> = |from, count| long(from, count).collect();
    let mut next = MILLION;

    let mut state = full(MILLION, false);
    bench.settle(&mut state);
    let frames = stream(&mut bench, &mut state, &mut next, fitting);
    assert!(state.follows());
    assert_eq!(state.len(), MILLION);
    let whole: Duration = frames.iter().map(|frame| frame.own()).sum();
    let worst_frame = worst("following lines that fit a row", &frames);
    let following = median(frames);
    report("following, lines that fit a row, a frame", following);
    println!(
        "  the worst frame {:.3} ms; two seconds of the stream took {:.1} ms of the viewer's work \
         ({:.1} % of the time)",
        ms(worst_frame),
        ms(whole),
        ms(whole) / 2000.0 * 100.0
    );
    // Paused, the lines still come and the view stays: nothing is shaped.
    let _ = state.update(Message::Scroll(Scroll::Rows(-200)));
    bench.settle(&mut state);
    let paused = median(stream(&mut bench, &mut state, &mut next, fitting));
    report("the same, paused 200 rows up", paused);
    assert!(!state.follows());
    // Following again, with a search on that has read every line: the
    // lines that come are read for it, and matches are found in them.
    let _ = state.update(Message::Scroll(Scroll::End));
    searching(&mut state);
    bench.settle(&mut state);
    let last_hit = state.search.hits.back();
    let frames = stream(&mut bench, &mut state, &mut next, fitting);
    assert!(
        !state.search.busy && !state.is_searching(),
        "every line was read"
    );
    assert_ne!(state.search.hits.back(), last_hit, "new lines matched");
    worst("following with a search on", &frames);
    let searched = median(frames);
    report("following, with a search on", searched);
    drop(state);

    // Lines of three rows each.
    let mut state = full(MILLION, true);
    bench.settle(&mut state);
    let frames = stream(&mut bench, &mut state, &mut next, wrapping);
    let worst_wrapped = worst("following lines that wrap", &frames);
    let wrapped = median(frames);
    report("following, lines of three rows, a frame", wrapped);
    println!("  the worst frame {:.3} ms", ms(worst_wrapped));

    within("a frame of a view that follows", following.own(), FOLLOW);
    within("a frame while paused", paused.own(), FOLLOW);
    within(
        "a frame that follows with a search on",
        searched.own(),
        FOLLOW,
    );
    within(
        "a frame that follows lines that wrap",
        wrapped.own(),
        FOLLOW,
    );
    println!(
        "  a frame is {:.1} % of a 60 fps frame with the software renderer's pixels, {:.1} % \
         without",
        ms(following.total()) / (1000.0 / 60.0) * 100.0,
        ms(following.own()) / (1000.0 / 60.0) * 100.0,
    );
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "a measurement: run it alone with --nocapture and read what it prints"]
async fn a_search_over_a_million_lines_never_holds_update() {
    let mut state = full(MILLION, false);
    println!("\n{}:", build());
    for (query, regex) in [("ERROR", false), ("id=\\d+7 path", true), ("e", false)] {
        let _ = state.update(Message::Regex(regex));
        let start = Instant::now();
        let (mut task, _) = state.update(Message::Query(query.to_owned()));
        let mut worst = start.elapsed();
        let mut answers = 0;
        loop {
            let found = messages(task).await;
            let Some(message) = found.into_iter().next() else {
                break;
            };
            answers += 1;
            let update = Instant::now();
            task = state.update(message).0;
            worst = worst.max(update.elapsed());
        }
        let whole = start.elapsed();
        let (_, total) = state.matches();
        println!(
            "{query:?}{}: {total} matches in {answers} answers, {:.0} ms in all on another \
             thread; the longest update {:.3} ms",
            if regex {
                " as a regular expression"
            } else {
                ""
            },
            ms(whole),
            ms(worst)
        );
        assert!(!state.is_searching());
        assert!(total > 0);
        within(
            &format!("an update of the search for {query:?}"),
            worst,
            SEARCH,
        );
    }
    // Stepping through matches of a million lines: the first step counts
    // the matches before it, the others don't.
    let start = Instant::now();
    let _ = state.update(Message::Previous);
    let first = start.elapsed();
    let start = Instant::now();
    for _ in 0..100 {
        let _ = state.update(Message::Previous);
    }
    let step = start.elapsed() / 100;
    println!(
        "the first step to a match {:.3} ms, each one after {:.4} ms",
        ms(first),
        ms(step)
    );
    within("the first step to a match", first, SEARCH * 2);
    within("a step to a match", step, SEARCH);
}

/// The one query built in `update`: plain text of up to
/// [`QUICK`](crate::ui::widgets::log::search::QUICK) bytes. The worst
/// of them, whatever the case, is built within the search's budget;
/// everything else is built by a task.
#[test]
#[ignore = "a measurement: run it alone with --nocapture and read what it prints"]
fn the_query_built_in_update_is_built_within_the_budget() {
    use crate::ui::widgets::log::search::{Query, QUICK};
    println!("\n{}:", build());
    let texts = [
        "e".to_owned(),
        "ERROR".to_owned(),
        "connection reset by peer".to_owned(),
        "aZ".repeat(QUICK / 2),
        "é".repeat(QUICK / 2),
        ".*+?()[]{}|^$\\".repeat(QUICK / 14),
    ];
    let mut worst = Duration::ZERO;
    for text in texts {
        let query = Query {
            text,
            regex: false,
            case: false,
        };
        assert!(query.is_quick());
        let times = (0..20)
            .map(|_| {
                let start = Instant::now();
                assert!(query.compile().expect("text is never wrong").is_some());
                start.elapsed()
            })
            .collect();
        let took = middle(times);
        println!(
            "{:>4} bytes of plain text, whatever the case: {:.3} ms",
            query.text.len(),
            ms(took)
        );
        worst = worst.max(took);
    }
    within("the query built in update", worst, SEARCH);
}

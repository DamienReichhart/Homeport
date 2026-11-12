//! What the buffer keeps, drops and hands to another thread.

use std::sync::Arc;

use super::{feed, numbered, viewer};
use crate::ui::widgets::log::{
    buffer::{Buffer, CHUNK},
    Line, Message, Stream,
};

fn push(buffer: &mut Buffer, texts: impl IntoIterator<Item = String>) -> u64 {
    buffer.append(texts, |chunk, text| {
        chunk.push(Stream::Stdout, None, false, |out, _, _| out.push_str(&text));
    })
}

fn texts(buffer: &Buffer) -> Vec<String> {
    buffer
        .range()
        .map(|seq| buffer.line(seq).expect("a kept line").text.to_owned())
        .collect()
}

#[test]
fn lines_keep_their_number_as_others_are_dropped() {
    let mut buffer = Buffer::new(5);
    assert_eq!(push(&mut buffer, (0..3).map(|n| format!("l{n}"))), 0);
    assert_eq!(buffer.range(), 0..3);
    // Past the cap the oldest go, and what is kept is known as before.
    assert_eq!(push(&mut buffer, (3..9).map(|n| format!("l{n}"))), 4);
    assert_eq!(buffer.range(), 4..9);
    assert_eq!(buffer.len(), 5);
    assert_eq!(texts(&buffer), ["l4", "l5", "l6", "l7", "l8"]);
    assert_eq!(buffer.line(6).expect("kept").text, "l6");
    assert!(buffer.line(3).is_none(), "dropped");
    assert!(buffer.line(9).is_none(), "not there yet");
}

#[test]
fn a_buffer_far_longer_than_a_chunk_drops_whole_chunks() {
    let cap = 3 * CHUNK + 7;
    let mut buffer = Buffer::new(cap);
    let total = 10 * CHUNK + 123;
    // In batches of every size, across the ends of chunks.
    let mut next = 0;
    for batch in [1, CHUNK - 1, 2, 5 * CHUNK, 17, total] {
        let end = (next + batch).min(total);
        push(&mut buffer, (next..end).map(|n| format!("l{n}")));
        next = end;
        assert_eq!(buffer.len(), next.min(cap));
    }
    assert_eq!(buffer.range(), (total - cap) as u64..total as u64);
    for seq in [
        buffer.range().start,
        buffer.range().end - 1,
        9 * CHUNK as u64,
    ] {
        assert_eq!(buffer.line(seq).expect("kept").text, format!("l{seq}"));
    }
    // A smaller cap drops at once; a larger one keeps what is there.
    assert_eq!(buffer.set_cap(10), (cap - 10) as u64);
    assert_eq!(buffer.len(), 10);
    assert_eq!(buffer.set_cap(1000), 0);
    assert_eq!(
        buffer.line(buffer.range().start).expect("kept").text,
        format!("l{}", total - 10)
    );
}

#[test]
fn cleared_the_numbering_goes_on() {
    let mut buffer = Buffer::new(100);
    push(&mut buffer, (0..10).map(|n| format!("l{n}")));
    buffer.clear();
    assert!(buffer.is_empty());
    assert_eq!(buffer.range(), 10..10);
    push(&mut buffer, ["again".to_owned()]);
    assert_eq!(buffer.line(10).expect("kept").text, "again");
    assert!(buffer.line(3).is_none());
}

#[test]
fn a_snapshot_is_the_lines_as_they_were_whatever_comes_after() {
    let mut buffer = Buffer::new(4 * CHUNK);
    push(&mut buffer, (0..CHUNK + 10).map(|n| format!("l{n}")));
    let snapshot = buffer.snapshot(5..(CHUNK + 10) as u64);
    assert_eq!(snapshot.len(), CHUNK + 5);
    // The buffer goes on, dropping what the snapshot holds.
    push(&mut buffer, (0..8 * CHUNK).map(|n| format!("later {n}")));
    assert!(buffer.line(5).is_none());
    let read: Vec<(u64, String)> = snapshot
        .lines()
        .map(|(seq, line)| (seq, line.text.to_owned()))
        .collect();
    assert_eq!(read.len(), CHUNK + 5);
    assert_eq!(read[0], (5, "l5".to_owned()));
    assert_eq!(
        read[CHUNK + 4],
        ((CHUNK + 9) as u64, format!("l{}", CHUNK + 9))
    );
    // On another thread, as a search reads it.
    let shared = Arc::new(snapshot);
    let there = shared.clone();
    let count = std::thread::spawn(move || there.lines().count())
        .join()
        .expect("the thread ends");
    assert_eq!(count, CHUNK + 5);
    // Out of what is kept: nothing, and no panic.
    assert!(buffer.snapshot(0..3).is_empty());
    assert!(buffer.snapshot(7..7).is_empty());
    assert!(Buffer::new(5).snapshot(0..10).is_empty());
}

#[test]
fn a_line_keeps_its_stream_its_source_its_timestamp_and_its_colours() {
    let mut state = viewer(100);
    feed(
        &mut state,
        [
            Line::new("2024-05-01T10:00:00.123456789Z \x1b[31mfailed\x1b[0m to bind")
                .stream(Stream::Stderr)
                .source("web.1")
                .timestamped(true),
            Line::new("no stamp here").timestamped(true).source("db.1"),
            Line::new("日本語 ok"),
        ],
    );
    let first = state.buffer.line(0).expect("kept");
    assert_eq!(first.timestamp(), Some("2024-05-01T10:00:00.123456789Z"));
    assert_eq!(first.message(), "failed to bind");
    assert_eq!(first.cells, 14);
    assert_eq!(first.stream, Stream::Stderr);
    assert_eq!(first.source, Some(0));
    assert_eq!(first.runs.len(), 1);
    let run = first.runs[0];
    assert_eq!(&first.text[run.start as usize..run.end as usize], "failed");
    // The first word of a line that has no timestamp is only a word when
    // it is too long, or not ASCII, to be one; here "no" is taken for it,
    // as the React viewer did with whatever came before the first space.
    let second = state.buffer.line(1).expect("kept");
    assert_eq!(second.timestamp(), Some("no"));
    assert_eq!(second.source, Some(1));
    let third = state.buffer.line(2).expect("kept");
    assert_eq!((third.cells, third.ascii, third.stamp), (9, false, 0));
    assert_eq!(state.sources(), ["web.1".into(), "db.1".into()]);
}

#[test]
fn each_source_has_its_own_terminal_state() {
    let mut state = viewer(100);
    feed(
        &mut state,
        [
            Line::new("\x1b[32mgreen starts").source("a"),
            Line::new("plain").source("b"),
            Line::new("still green\x1b[0m").source("a"),
            Line::new("plain too"),
        ],
    );
    let runs = |seq| state.buffer.line(seq).expect("kept").runs.len();
    assert_eq!([runs(0), runs(1), runs(2), runs(3)], [1, 0, 1, 0]);
}

#[test]
fn the_cap_is_the_pages_and_can_change() {
    let mut state = viewer(50);
    assert_eq!(state.cap(), 50);
    feed(&mut state, numbered(0, 120));
    assert_eq!(state.len(), 50);
    state.set_cap(20);
    assert_eq!(state.len(), 20);
    assert_eq!(state.buffer.line(100).expect("kept").text, "line 100");
    let (_, event) = state.update(Message::Clear);
    assert!(matches!(
        event,
        Some(crate::ui::widgets::log::Event::Cleared)
    ));
    assert!(state.is_empty());
}

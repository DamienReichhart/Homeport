//! Output that came faster than it is read: kept in order and read a
//! megabyte a frame ([`READ_AT_ONCE`]), no more than [`UNREAD_MAX`] of it,
//! with the app's own lines (`State::notice`) waiting in their places in
//! it.

use std::collections::VecDeque;

use super::{
    emulator::Emulator,
    state::{READ_AT_ONCE, UNREAD_MAX},
};

#[derive(Debug, Default)]
pub(super) struct Backlog {
    /// Output not read yet.
    unread: VecDeque<u8>,
    /// How many bytes of it were dropped past [`UNREAD_MAX`].
    dropped: u64,
    /// How many bytes went through it, read or dropped: where it starts
    /// in what the session printed.
    passed: u64,
    /// The app's lines that wait behind unread output: where each goes
    /// (counted as `passed` is), and the bytes that write it.
    notices: VecDeque<(u64, String)>,
}

impl Backlog {
    /// Nothing waits: output may be read as it comes.
    pub(super) fn is_empty(&self) -> bool {
        self.unread.is_empty() && self.notices.is_empty()
    }

    /// How many bytes of output wait.
    pub(super) fn len(&self) -> usize {
        self.unread.len()
    }

    pub(super) fn dropped(&self) -> u64 {
        self.dropped
    }

    /// Output that came behind what waits: the oldest is dropped past
    /// [`UNREAD_MAX`], then a frame's worth is read.
    pub(super) fn push(&mut self, output: &[u8], emulator: &mut Emulator) {
        self.unread.extend(output);
        self.bound();
        self.read(emulator);
    }

    /// A line of the app's after the output that came so far: written now
    /// if none waits, else in its turn.
    pub(super) fn notice(&mut self, line: String, emulator: &mut Emulator) {
        let at = self.passed + self.unread.len() as u64;
        self.notices.push_back((at, line));
        self.write_notices(emulator);
    }

    /// What waits is let go of (a new session).
    pub(super) fn clear(&mut self) {
        self.passed += self.unread.len() as u64;
        self.unread.clear();
        self.notices.clear();
    }

    /// Drops the oldest unread output past [`UNREAD_MAX`], up to the start
    /// of a line, so what is read next starts where a line does.
    fn bound(&mut self) {
        let Some(over) = self.unread.len().checked_sub(UNREAD_MAX) else {
            return;
        };
        let line = self
            .unread
            .range(over..)
            .position(|byte| *byte == b'\n')
            .map_or(over, |at| over + at + 1);
        self.unread.drain(..line);
        self.dropped += line as u64;
        self.passed += line as u64;
    }

    /// Reads up to [`READ_AT_ONCE`] of what waits, and the app's lines
    /// that come within it, in their places.
    pub(super) fn read(&mut self, emulator: &mut Emulator) {
        let mut budget = READ_AT_ONCE;
        loop {
            self.write_notices(emulator);
            let until = self.notices.front().map_or(usize::MAX, |(at, _)| {
                usize::try_from(at - self.passed).unwrap_or(usize::MAX)
            });
            let take = self.unread.len().min(budget).min(until);
            if take == 0 {
                break;
            }
            let (front, back) = self.unread.as_slices();
            if front.len() >= take {
                emulator.advance(&front[..take]);
            } else {
                let mut part = Vec::with_capacity(take);
                part.extend_from_slice(front);
                part.extend_from_slice(&back[..take - front.len()]);
                emulator.advance(&part);
            }
            self.unread.drain(..take);
            self.passed += take as u64;
            budget -= take;
        }
    }

    /// Writes the app's lines whose place in the output was reached, after
    /// whatever a synchronised update holds.
    fn write_notices(&mut self, emulator: &mut Emulator) {
        while self
            .notices
            .front()
            .is_some_and(|(at, _)| *at <= self.passed)
        {
            if let Some((_, line)) = self.notices.pop_front() {
                emulator.release();
                emulator.advance(line.as_bytes());
            }
        }
    }
}

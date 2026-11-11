//! Where the window has been: the memory router of the React app. A
//! navigation adds a location after the current one and drops what was
//! ahead; back and forward move among them.

use super::{Location, Request};

/// The locations visited, and which one shows.
#[derive(Debug, Clone)]
pub struct History {
    entries: Vec<Location>,
    at: usize,
    /// The key of the next location: each navigation has its own, so a
    /// page tells a new request from the one it already answered.
    next: u64,
}

impl Default for History {
    fn default() -> Self {
        Self::starting_at(Request::default())
    }
}

impl History {
    /// A history whose only location is `request`.
    pub fn starting_at(request: Request) -> Self {
        Self {
            entries: vec![Location::new(request, 0)],
            at: 0,
            next: 1,
        }
    }

    pub fn current(&self) -> &Location {
        &self.entries[self.at]
    }

    fn key(&mut self) -> u64 {
        self.next += 1;
        self.next - 1
    }

    /// Goes to `request`: what was ahead is forgotten.
    pub fn push(&mut self, request: Request) -> &Location {
        let location = Location::new(request, self.key());
        self.entries.truncate(self.at + 1);
        self.entries.push(location);
        self.at += 1;
        self.current()
    }

    /// Shows `request` in place of the current location, as a redirect:
    /// going back doesn't return to the page that sent away.
    pub fn replace(&mut self, request: Request) -> &Location {
        let location = Location::new(request, self.key());
        self.entries[self.at] = location;
        self.current()
    }

    pub fn can_go_back(&self) -> bool {
        self.at > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.at + 1 < self.entries.len()
    }

    /// The location before this one; `None` at the first.
    pub fn back(&mut self) -> Option<&Location> {
        if !self.can_go_back() {
            return None;
        }
        self.at -= 1;
        Some(self.current())
    }

    /// The location a `back` left; `None` at the last.
    pub fn forward(&mut self) -> Option<&Location> {
        if !self.can_go_forward() {
            return None;
        }
        self.at += 1;
        Some(self.current())
    }

    /// How many locations it holds; never none.
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

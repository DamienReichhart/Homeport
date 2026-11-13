//! A JSON document as a list of its nodes in the order it is written,
//! each knowing how deep it is and where what it holds ends: what a
//! view of ten rows needs to show ten rows of a document of a million.
//!
//! The text is read once, straight into the list, by a reader of its
//! own (`Reader`): the members of an object stay in the order they
//! were written in and a name written twice is there twice, a number
//! is kept as it was written (`1e400`, `-0`, twenty digits: none is
//! made a float), and nothing recurses, so a document nests as deep as
//! it likes.

use std::fmt::{self, Write as _};

/// How a node is named by what holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Name {
    /// The document itself.
    Root,
    /// A member of an object.
    Member(String),
    /// An item of an array.
    Item(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// With how many members.
    Object(usize),
    /// With how many items.
    Array(usize),
    String,
    Number,
    Bool,
    Null,
}

impl Kind {
    pub fn holds(self) -> Option<usize> {
        match self {
            Self::Object(count) | Self::Array(count) => Some(count),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub name: Name,
    pub kind: Kind,
    /// A scalar as the document wrote it (a string in its quotes, with
    /// its escapes; a number digit for digit); empty for an object or
    /// an array.
    pub text: String,
    pub depth: u32,
    /// The node that holds this one; the root holds itself.
    pub parent: u32,
    /// One past the last node this one holds.
    pub end: u32,
}

/// The most nodes a document may have: their places are kept in 32 bits.
pub const MOST: usize = u32::MAX as usize - 1;

/// Why a text is no JSON document, and where: the byte it stopped at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invalid {
    pub offset: usize,
    pub why: Why,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Why {
    /// Something else was expected there.
    Unexpected,
    /// The text ends before the document does.
    Unfinished,
    /// More values than a view can number ([`MOST`]).
    TooLarge,
    /// The value could not be written as JSON at all.
    Unwritable,
}

impl fmt::Display for Invalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not JSON at byte {}: {:?}", self.offset, self.why)
    }
}

impl std::error::Error for Invalid {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Document {
    nodes: Vec<Node>,
}

/// A string as JSON writes it.
fn quoted(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_else(|_| format!("{text:?}"))
}

/// What a string in its quotes says: its escapes read.
pub fn unquoted(text: &str) -> Option<String> {
    serde_json::from_str(text).ok()
}

/// Reads a document without recursing: what is open is kept in a list.
struct Reader<'t> {
    text: &'t str,
    at: usize,
    nodes: Vec<Node>,
    /// The objects and arrays that are open: the node, how many it
    /// holds so far, and whether it is an object.
    open: Vec<(u32, usize, bool)>,
}

impl Reader<'_> {
    fn fail<T>(&self, why: Why) -> Result<T, Invalid> {
        let why = if self.at >= self.text.len() && why == Why::Unexpected {
            Why::Unfinished
        } else {
            why
        };
        Err(Invalid {
            offset: self.at.min(self.text.len()),
            why,
        })
    }

    fn peek(&self) -> Option<u8> {
        self.text.as_bytes().get(self.at).copied()
    }

    fn skip_space(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.at += 1;
        }
    }

    fn eat(&mut self, byte: u8) -> Result<(), Invalid> {
        if self.peek() == Some(byte) {
            self.at += 1;
            Ok(())
        } else {
            self.fail(Why::Unexpected)
        }
    }

    /// The string that starts here, in its quotes as it is written.
    fn string(&mut self) -> Result<&str, Invalid> {
        let start = self.at;
        self.eat(b'"')?;
        loop {
            match self.peek() {
                Some(b'"') => break,
                // Whatever is escaped is not the closing quote.
                Some(b'\\') => self.at += 2,
                Some(_) => self.at += 1,
                None => return self.fail(Why::Unfinished),
            }
        }
        self.at += 1;
        let Some(raw) = self.text.get(start..self.at) else {
            return self.fail(Why::Unexpected);
        };
        // Its escapes and what may not be in a string are JSON's to say.
        if unquoted(raw).is_none() {
            self.at = start;
            return self.fail(Why::Unexpected);
        }
        Ok(raw)
    }

    fn digits(&mut self) -> usize {
        let start = self.at;
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.at += 1;
        }
        self.at - start
    }

    /// The number that starts here, digit for digit: JSON's grammar,
    /// and no reading of what it is worth.
    fn number(&mut self) -> Result<&str, Invalid> {
        let start = self.at;
        if self.peek() == Some(b'-') {
            self.at += 1;
        }
        let leading_zero = self.peek() == Some(b'0');
        let whole = self.digits();
        if whole == 0 || (leading_zero && whole > 1) {
            return self.fail(Why::Unexpected);
        }
        if self.peek() == Some(b'.') {
            self.at += 1;
            if self.digits() == 0 {
                return self.fail(Why::Unexpected);
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.at += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.at += 1;
            }
            if self.digits() == 0 {
                return self.fail(Why::Unexpected);
            }
        }
        Ok(&self.text[start..self.at])
    }

    fn push(&mut self, name: Name, kind: Kind, text: String) -> Result<u32, Invalid> {
        let at = self.nodes.len();
        if at >= MOST {
            return self.fail(Why::TooLarge);
        }
        // Checked against `MOST` just above; a document that deep is
        // larger than that too.
        #[allow(clippy::cast_possible_truncation)]
        let (at, depth) = (at as u32, self.open.len() as u32);
        self.nodes.push(Node {
            name,
            kind,
            text,
            depth,
            parent: self.open.last().map_or(0, |open| open.0),
            end: at + 1,
        });
        Ok(at)
    }

    /// The name the next value has in what holds it: an item's place,
    /// or a member's name, read with its colon.
    fn name(&mut self) -> Result<Name, Invalid> {
        match self.open.last().copied() {
            None => Ok(Name::Root),
            Some((_, count, false)) => Ok(Name::Item(count)),
            Some((_, _, true)) => {
                let raw = self.string()?;
                let name = unquoted(raw).unwrap_or_default();
                self.skip_space();
                self.eat(b':')?;
                self.skip_space();
                Ok(Name::Member(name))
            }
        }
    }

    /// One value, or the start of one that holds others.
    fn value(&mut self) -> Result<(), Invalid> {
        let name = self.name()?;
        let literal = |reader: &mut Self, name: Name, word: &str, kind| {
            if reader.text[reader.at..].starts_with(word) {
                reader.at += word.len();
                reader.push(name, kind, word.to_owned()).map(|_| ())
            } else {
                reader.fail(Why::Unexpected)
            }
        };
        match self.peek() {
            Some(b'{') | Some(b'[') => {
                let object = self.peek() == Some(b'{');
                self.at += 1;
                let kind = if object {
                    Kind::Object(0)
                } else {
                    Kind::Array(0)
                };
                let node = self.push(name, kind, String::new())?;
                self.open.push((node, 0, object));
                return Ok(());
            }
            Some(b'"') => {
                let raw = self.string()?.to_owned();
                self.push(name, Kind::String, raw)?;
            }
            Some(b'-' | b'0'..=b'9') => {
                let raw = self.number()?.to_owned();
                self.push(name, Kind::Number, raw)?;
            }
            Some(b't') => literal(self, name, "true", Kind::Bool)?,
            Some(b'f') => literal(self, name, "false", Kind::Bool)?,
            Some(b'n') => literal(self, name, "null", Kind::Null)?,
            _ => return self.fail(Why::Unexpected),
        }
        self.ended();
        Ok(())
    }

    /// A value ended: what holds it counts one more.
    fn ended(&mut self) {
        if let Some(open) = self.open.last_mut() {
            open.1 += 1;
        }
    }

    fn read(mut self) -> Result<Vec<Node>, Invalid> {
        self.skip_space();
        self.value()?;
        while let Some((node, count, object)) = self.open.last().copied() {
            self.skip_space();
            let close = if object { b'}' } else { b']' };
            if self.peek() == Some(close) {
                self.at += 1;
                self.open.pop();
                // A document has fewer nodes than `MOST`.
                #[allow(clippy::cast_possible_truncation)]
                let end = self.nodes.len() as u32;
                let closed = &mut self.nodes[node as usize];
                closed.end = end;
                closed.kind = if object {
                    Kind::Object(count)
                } else {
                    Kind::Array(count)
                };
                self.ended();
                continue;
            }
            if count > 0 {
                self.eat(b',')?;
                self.skip_space();
            }
            self.value()?;
        }
        self.skip_space();
        if self.at < self.text.len() {
            return self.fail(Why::Unexpected);
        }
        Ok(self.nodes)
    }
}

impl Document {
    /// Reads `text`. This is a pass over the whole document: a page
    /// that shows megabytes reads them in a task, not in `update`.
    pub fn parse(text: &str) -> Result<Self, Invalid> {
        let reader = Reader {
            text,
            at: 0,
            nodes: vec![],
            open: vec![],
        };
        reader.read().map(|nodes| Self { nodes })
    }

    /// What `value` is as JSON, its fields in the order it writes them.
    pub fn of(value: &impl serde::Serialize) -> Result<Self, Invalid> {
        let unwritable = Invalid {
            offset: 0,
            why: Why::Unwritable,
        };
        Self::parse(&serde_json::to_string(value).map_err(|_| unwritable)?)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn node(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// The nodes that hold `index`, from the root down to it.
    pub fn lineage(&self, index: usize) -> Vec<usize> {
        let mut line = vec![];
        let mut at = index;
        while let Some(node) = self.nodes.get(at) {
            line.push(at);
            if at == 0 {
                break;
            }
            at = node.parent as usize;
        }
        line.reverse();
        line
    }

    /// Where the node is, as `jq` writes it: `.ports[0].name`, a member
    /// that is no plain word in brackets and quotes (`.labels["a.b"]`),
    /// and `.` for the document itself. `None` for a node that is not
    /// there. (A name written twice in one object gives two nodes the
    /// same path: the document's doing.)
    pub fn path(&self, index: usize) -> Option<String> {
        self.nodes.get(index)?;
        let mut path = String::new();
        for at in self.lineage(index) {
            match &self.nodes[at].name {
                Name::Root => {}
                Name::Item(n) => {
                    let _ = write!(path, "[{n}]");
                }
                Name::Member(name) if is_plain(name) => {
                    let _ = write!(path, ".{name}");
                }
                Name::Member(name) => {
                    // After the root the dot is the path's start.
                    if path.is_empty() {
                        path.push('.');
                    }
                    let _ = write!(path, "[{}]", quoted(name));
                }
            }
        }
        if path.is_empty() || path.starts_with('[') {
            path.insert(0, '.');
        }
        Some(path)
    }

    /// The node and what it holds, written out on indented lines as
    /// `JSON.stringify(value, null, 2)` wrote the whole document.
    pub fn pretty(&self, index: usize) -> String {
        let mut out = String::new();
        let Some(root) = self.nodes.get(index) else {
            return out;
        };
        // What is open, and how many of its members are still to come.
        let mut open: Vec<(Kind, usize)> = vec![];
        for node in &self.nodes[index..root.end as usize] {
            let indent = open.len();
            if !open.is_empty() {
                out.push('\n');
                out.extend(std::iter::repeat_n("  ", indent));
                if let Name::Member(name) = &node.name {
                    out.push_str(&quoted(name));
                    out.push_str(": ");
                }
            }
            match node.kind {
                Kind::Object(0) => out.push_str("{}"),
                Kind::Array(0) => out.push_str("[]"),
                Kind::Object(count) | Kind::Array(count) => {
                    out.push(if matches!(node.kind, Kind::Object(_)) {
                        '{'
                    } else {
                        '['
                    });
                    open.push((node.kind, count));
                    continue;
                }
                _ => out.push_str(&node.text),
            }
            // This value ends, and with it whatever it was the last of.
            while let Some((kind, left)) = open.last_mut() {
                *left -= 1;
                if *left > 0 {
                    out.push(',');
                    break;
                }
                let close = if matches!(kind, Kind::Object(_)) {
                    '}'
                } else {
                    ']'
                };
                open.pop();
                out.push('\n');
                out.extend(std::iter::repeat_n("  ", open.len()));
                out.push(close);
            }
        }
        out
    }
}

/// Whether `name` can follow a dot in a path as it is.
fn is_plain(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

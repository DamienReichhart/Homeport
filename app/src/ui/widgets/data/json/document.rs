//! A JSON document as a list of its nodes in the order it is written,
//! each knowing how deep it is and where what it holds ends: what a
//! view of ten rows needs to show ten rows of a document of a million.
//!
//! The text is read once, straight into the list (`serde_json` hands
//! each value as it reads it), so the members of an object stay in the
//! order they were written in, which a `serde_json::Value` would sort.

use std::fmt::{self, Write as _};

use serde::de::{DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};

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
    /// A scalar as JSON writes it (a string in its quotes, escaped);
    /// empty for an object or an array.
    pub text: String,
    pub depth: u16,
    /// The node that holds this one; the root holds itself.
    pub parent: u32,
    /// One past the last node this one holds.
    pub end: u32,
}

/// The most nodes a document may have: their places are kept in 32 bits.
pub const MOST: usize = u32::MAX as usize - 1;
/// How deep a document may nest (`serde_json` stops at 128 itself).
const DEEPEST: u16 = 512;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Document {
    nodes: Vec<Node>,
}

/// Reads one value into the list, under `parent`.
struct Read<'d> {
    nodes: &'d mut Vec<Node>,
    name: Name,
    parent: u32,
    depth: u16,
}

impl Read<'_> {
    fn scalar<E: serde::de::Error>(self, kind: Kind, text: String) -> Result<(), E> {
        let at = self.nodes.len();
        if at >= MOST {
            return Err(E::custom("the document has too many values"));
        }
        // Checked against `MOST` just above.
        #[allow(clippy::cast_possible_truncation)]
        let end = at as u32 + 1;
        self.nodes.push(Node {
            name: self.name,
            kind,
            text,
            depth: self.depth,
            parent: self.parent,
            end,
        });
        Ok(())
    }

    /// Pushes the node of an object or an array, to be closed once what
    /// it holds is read.
    fn open<E: serde::de::Error>(&mut self, kind: Kind) -> Result<u32, E> {
        if self.depth >= DEEPEST {
            return Err(E::custom("the document nests too deep"));
        }
        let at = self.nodes.len();
        if at >= MOST {
            return Err(E::custom("the document has too many values"));
        }
        #[allow(clippy::cast_possible_truncation)]
        let at = at as u32;
        self.nodes.push(Node {
            name: std::mem::replace(&mut self.name, Name::Root),
            kind,
            text: String::new(),
            depth: self.depth,
            parent: self.parent,
            end: at + 1,
        });
        Ok(at)
    }

    fn close(&mut self, at: u32, kind: Kind) {
        #[allow(clippy::cast_possible_truncation)]
        let end = self.nodes.len() as u32;
        let node = &mut self.nodes[at as usize];
        node.kind = kind;
        node.end = end;
    }
}

impl<'de> DeserializeSeed<'de> for Read<'_> {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_any(self)
    }
}

/// A string as JSON writes it.
fn quoted(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_else(|_| format!("{text:?}"))
}

impl<'de> Visitor<'de> for Read<'_> {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a JSON value")
    }

    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<(), E> {
        self.scalar(Kind::Bool, value.to_string())
    }

    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<(), E> {
        self.scalar(Kind::Number, value.to_string())
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<(), E> {
        self.scalar(Kind::Number, value.to_string())
    }

    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<(), E> {
        // As `serde_json` writes a float: `1.0`, `1e300`.
        let text = serde_json::Number::from_f64(value)
            .map_or_else(|| "null".to_owned(), |number| number.to_string());
        self.scalar(Kind::Number, text)
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<(), E> {
        self.scalar(Kind::String, quoted(value))
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<(), E> {
        self.scalar(Kind::Null, "null".to_owned())
    }

    fn visit_seq<A: SeqAccess<'de>>(mut self, mut items: A) -> Result<(), A::Error> {
        let at = self.open(Kind::Array(0))?;
        let mut count = 0;
        while items
            .next_element_seed(Read {
                nodes: self.nodes,
                name: Name::Item(count),
                parent: at,
                depth: self.depth + 1,
            })?
            .is_some()
        {
            count += 1;
        }
        self.close(at, Kind::Array(count));
        Ok(())
    }

    fn visit_map<A: MapAccess<'de>>(mut self, mut members: A) -> Result<(), A::Error> {
        let at = self.open(Kind::Object(0))?;
        let mut count = 0;
        while let Some(name) = members.next_key::<String>()? {
            members.next_value_seed(Read {
                nodes: self.nodes,
                name: Name::Member(name),
                parent: at,
                depth: self.depth + 1,
            })?;
            count += 1;
        }
        self.close(at, Kind::Object(count));
        Ok(())
    }
}

impl Document {
    /// Reads `text`. This is a pass over the whole document: a page
    /// that shows megabytes reads them in a task, not in `update`.
    pub fn parse(text: &str) -> Result<Self, serde_json::Error> {
        let mut nodes = vec![];
        let mut reader = serde_json::Deserializer::from_str(text);
        Read {
            nodes: &mut nodes,
            name: Name::Root,
            parent: 0,
            depth: 0,
        }
        .deserialize(&mut reader)?;
        reader.end()?;
        Ok(Self { nodes })
    }

    /// What `value` is as JSON, its fields in the order it writes them.
    pub fn of(value: &impl serde::Serialize) -> Result<Self, serde_json::Error> {
        Self::parse(&serde_json::to_string(value)?)
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
    /// and `.` for the document itself.
    pub fn path(&self, index: usize) -> String {
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
        path
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

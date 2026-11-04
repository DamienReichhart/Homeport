//! The text of an error or hint: a stable catalogue key, named parameters,
//! and the English rendering kept for the log (ADR 0011).
//!
//! Each crate declares its messages once with [`messages!`](crate::messages):
//!
//! ```
//! homeport_core::messages! {
//!     /// A port the Host already uses.
//!     PORT_IN_USE = "errors.engine.portInUse" => "Port {port} is already in use.";
//! }
//! let text = PORT_IN_USE.with("port", 8080);
//! assert_eq!(text.english(), "Port 8080 is already in use.");
//! ```
//!
//! Templates are ICU MessageFormat, the frontend's catalogue format: `{name}`
//! arguments, and an apostrophe before `{`, `}` or `'` quotes it (`'{'`).
//! Plurals aren't rendered here: write two messages instead.

use std::collections::BTreeMap;

use serde::Serialize;

/// A catalogue entry: its key and English template.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message {
    pub key: &'static str,
    pub en: &'static str,
}

/// A parameter of a message: text such as a Host's name, or a number.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(untagged)]
pub enum Param {
    Text(String),
    Number(i64),
}

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(t) => f.write_str(t),
            Self::Number(n) => write!(f, "{n}"),
        }
    }
}

macro_rules! param_from {
    (text: $($t:ty),*; number: $($n:ty),*) => {
        $(impl From<$t> for Param {
            fn from(v: $t) -> Self { Self::Text(v.to_string()) }
        })*
        $(impl From<$n> for Param {
            fn from(v: $n) -> Self { Self::Number(i64::from(v)) }
        })*
    };
}
param_from!(text: &str, String, &String, std::borrow::Cow<'_, str>; number: i64, i32, u32, u16, u8);

impl From<usize> for Param {
    fn from(v: usize) -> Self {
        Self::Number(i64::try_from(v).unwrap_or(i64::MAX))
    }
}

impl From<u64> for Param {
    fn from(v: u64) -> Self {
        Self::Number(i64::try_from(v).unwrap_or(i64::MAX))
    }
}

impl From<&std::path::Path> for Param {
    fn from(v: &std::path::Path) -> Self {
        Self::Text(v.display().to_string())
    }
}

impl From<&std::path::PathBuf> for Param {
    fn from(v: &std::path::PathBuf) -> Self {
        Self::Text(v.display().to_string())
    }
}

/// Named parameters, in a stable order.
pub type Params = BTreeMap<String, Param>;

/// A message with its parameters filled in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    pub key: String,
    pub params: Params,
    template: Template,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Template {
    Icu(&'static str),
    /// Text not yet in a `messages!` block, shown as it is.
    Plain(String),
}

impl Message {
    /// This message with parameter `name` set.
    pub fn with(self, name: &str, value: impl Into<Param>) -> Text {
        Text::from(self).with(name, value)
    }
}

impl Text {
    /// Sets parameter `name`. Never pass raw OS, Engine or library text:
    /// that belongs in the error's `detail`.
    pub fn with(mut self, name: &str, value: impl Into<Param>) -> Self {
        self.params.insert(name.to_owned(), value.into());
        self
    }

    /// The text in English, for the app log and as the frontend's fallback.
    pub fn english(&self) -> String {
        match &self.template {
            Template::Icu(template) => render(template, &self.params),
            Template::Plain(text) => text.clone(),
        }
    }
}

impl From<Message> for Text {
    fn from(m: Message) -> Self {
        Self {
            key: m.key.to_owned(),
            params: Params::new(),
            template: Template::Icu(m.en),
        }
    }
}

// Unkeyed text, while the crates move to `messages!`; it reaches the
// frontend with an empty key, which falls back to the English message.
impl From<&str> for Text {
    fn from(text: &str) -> Self {
        Self::from(text.to_owned())
    }
}

impl From<String> for Text {
    fn from(text: String) -> Self {
        Self {
            key: String::new(),
            params: Params::new(),
            template: Template::Plain(text),
        }
    }
}

/// Renders an ICU template's `{arguments}` and quoting; an argument without
/// a value stays as written.
pub fn render(template: &str, params: &Params) -> String {
    let mut out = String::with_capacity(template.len());
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\'' => match chars.peek() {
                Some('\'') => {
                    chars.next();
                    out.push('\'');
                }
                Some('{' | '}') => {
                    // Quoted literal text, up to the next lone apostrophe.
                    while let Some(q) = chars.next() {
                        if q == '\'' {
                            if chars.peek() == Some(&'\'') {
                                chars.next();
                                out.push('\'');
                            } else {
                                break;
                            }
                        } else {
                            out.push(q);
                        }
                    }
                }
                _ => out.push('\''),
            },
            '{' => {
                let name: String = chars.by_ref().take_while(|&n| n != '}').collect();
                match params.get(name.trim()) {
                    Some(value) => out.push_str(&value.to_string()),
                    None => {
                        out.push('{');
                        out.push_str(&name);
                        out.push('}');
                    }
                }
            }
            _ => out.push(c),
        }
    }
    out
}

/// The `{arguments}` a template names, outside quoted text.
pub fn arguments(template: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\'' if matches!(chars.peek(), Some('{' | '}')) => {
                for q in chars.by_ref() {
                    if q == '\'' {
                        break;
                    }
                }
            }
            '{' => names.push(chars.by_ref().take_while(|&n| n != '}').collect()),
            _ => {}
        }
    }
    names
}

/// Declares a crate's error messages as `Message` constants, plus `ALL`
/// listing them for the catalogue. Keys start with `errors.`.
#[macro_export]
macro_rules! messages {
    ($($(#[$doc:meta])* $name:ident = $key:literal => $en:literal;)*) => {
        $(
            $(#[$doc])*
            pub const $name: $crate::error::Message = $crate::error::Message { key: $key, en: $en };
        )*
        /// Every message declared here, for the generated catalogue.
        pub const ALL: &[$crate::error::Message] = &[$($name),*];
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: Message = Message {
        key: "errors.test.sample",
        en: "Port {port} on {host} isn't free.",
    };

    #[test]
    fn arguments_are_filled_in() {
        let text = SAMPLE.with("port", 8080).with("host", "prod");
        assert_eq!(text.english(), "Port 8080 on prod isn't free.");
        assert_eq!(text.key, "errors.test.sample");
        assert_eq!(
            serde_json::to_value(&text.params).unwrap(),
            serde_json::json!({ "host": "prod", "port": 8080 })
        );
    }

    #[test]
    fn quoting_follows_icu() {
        let none = Params::new();
        assert_eq!(render("Use '{'VAR'}' here", &none), "Use {VAR} here");
        assert_eq!(render("It''s fine, isn't it", &none), "It's fine, isn't it");
        assert_eq!(render("Missing {name}", &none), "Missing {name}");
    }

    #[test]
    fn arguments_are_listed_outside_quotes() {
        assert_eq!(arguments("{a} and '{b}' and {c}"), ["a", "c"]);
    }
}

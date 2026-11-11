//! The app's text (ADR 0011), on Fluent: every string the window shows is a
//! message of the catalogues under `app/i18n`, looked up by the key the
//! React app used (`containers.empty.title`).
//!
//! ```ignore
//! t!("containers.title")
//! t!("containers.summary", running = 3, count = 12)
//! ```
//!
//! The language is the app's, not a page's: [`set_language`] swaps it for the
//! whole process, and the next `view` draws in it. Layouts take
//! [`direction`] instead of asking which language it is.

pub mod catalogue;
#[cfg(test)]
mod check;
pub mod direction;
pub mod errors;
pub mod format;
pub mod locale;
pub mod number;
pub mod rich;

use std::sync::{Arc, LazyLock, RwLock};

use fluent_bundle::{FluentArgs, FluentValue};

use self::{catalogue::Catalogue, format::Formats};
pub use self::{
    catalogue::{Locale, DEFAULT_LOCALE, LOCALES},
    direction::Direction,
    rich::{Span, Tag},
};

/// The language the app shows, with everything built for it.
pub struct Language {
    pub locale: &'static Locale,
    messages: Catalogue,
    pub formats: Formats,
}

static ENGLISH: LazyLock<Catalogue> = LazyLock::new(|| Catalogue::load(DEFAULT_LOCALE));

static CURRENT: LazyLock<RwLock<Arc<Language>>> =
    LazyLock::new(|| RwLock::new(Arc::new(Language::new(DEFAULT_LOCALE, &[]))));

impl Language {
    /// The language of `tag` (English if the app has no such catalogue),
    /// formatting in the OS's variant of it among `preferred`.
    pub fn new(tag: &str, preferred: &[String]) -> Self {
        let locale = LOCALES
            .iter()
            .find(|l| l.tag == tag)
            .or_else(|| LOCALES.first())
            .expect("the English catalogue is built in");
        Self {
            locale,
            messages: Catalogue::load(locale.tag),
            formats: Formats::new(&locale::formatting(locale.tag, preferred)),
        }
    }

    /// The text of `key`; in English when this language lacks it, and the
    /// key itself when English does too.
    pub fn text(&self, key: &str, args: Option<&FluentArgs<'_>>) -> String {
        let id = catalogue::id(key);
        self.messages
            .own(&id, args)
            .or_else(|| ENGLISH.own(&id, args))
            .unwrap_or_else(|| {
                log::warn!("no message is named {key}");
                key.to_owned()
            })
    }

    /// Whether `key` names a message, here or in English.
    pub fn has(&self, key: &str) -> bool {
        let id = catalogue::id(key);
        self.messages.has(&id) || ENGLISH.has(&id)
    }
}

/// The language the app is showing.
pub fn current() -> Arc<Language> {
    CURRENT
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
}

/// Shows the app in the language the Settings value `language` stands for
/// (`system`, or a tag), given the OS's preferred languages. Returns its tag.
pub fn set_language(language: &str, preferred: &[String]) -> &'static str {
    let tag = locale::resolve(language, preferred);
    if current().locale.tag != tag || current().formats.tag() != locale::formatting(tag, preferred)
    {
        let next = Arc::new(Language::new(tag, preferred));
        *CURRENT
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = next;
        // The crates render the text the OS draws (tray, notifications)
        // themselves; English is their own.
        homeport_core::error::set_translations(if tag == DEFAULT_LOCALE {
            std::collections::BTreeMap::new()
        } else {
            catalogue::native_templates(tag)
        });
    }
    tag
}

/// The writing direction of the app's language.
pub fn direction() -> Direction {
    current().locale.direction
}

/// A value a message is given: text, or a number its plurals count.
pub enum Arg {
    Text(String),
    Number(f64),
}

macro_rules! arg_from {
    (text: $($t:ty),*; number: $($n:ty),*) => {
        $(impl From<$t> for Arg {
            fn from(v: $t) -> Self { Self::Text(v.to_string()) }
        })*
        $(impl From<$n> for Arg {
            // Counts stay far below 2^53.
            #[allow(clippy::cast_precision_loss, clippy::cast_lossless)]
            fn from(v: $n) -> Self { Self::Number(v as f64) }
        })*
    };
}
arg_from!(text: &str, String, &String, char, std::borrow::Cow<'_, str>;
          number: u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

/// The arguments of one message, by name.
#[derive(Default)]
pub struct Args(pub(super) FluentArgs<'static>);

impl Args {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with(mut self, name: &'static str, value: impl Into<Arg>) -> Self {
        self.set(name, value.into(), false);
        self
    }

    /// An argument whose name isn't known when the code is written.
    pub(super) fn set_owned(&mut self, name: String, value: Arg) {
        self.set(name, value, false);
    }

    fn set(&mut self, name: impl Into<std::borrow::Cow<'static, str>>, value: Arg, rich: bool) {
        let value = match value {
            Arg::Text(text) if rich => FluentValue::from(rich::escape(&text)),
            Arg::Text(text) => FluentValue::from(text),
            Arg::Number(number) => FluentValue::from(number),
        };
        self.0.set(name, value);
    }

    /// The same arguments for a message with tags: their text can't be
    /// taken for markup.
    #[must_use]
    pub fn rich(mut self, name: &'static str, value: impl Into<Arg>) -> Self {
        self.set(name, value.into(), true);
        self
    }
}

/// The text of `key` with `args`, in the app's language.
pub fn text(key: &str, args: &Args) -> String {
    current().text(key, Some(&args.0))
}

/// The text of a message with tags, as the spans the window styles. Give
/// its arguments with [`Args::rich`].
pub fn spans(key: &str, args: &Args) -> Vec<Span> {
    rich::spans(&text(key, args))
}

/// The text of a message in the app's language: `t!` with its key, then
/// `name = value` for each of for its arguments.
#[macro_export]
macro_rules! t {
    ($key:literal) => {
        $crate::i18n::text($key, &$crate::i18n::Args::new())
    };
    ($key:literal, $($name:ident = $value:expr),+ $(,)?) => {
        $crate::i18n::text(
            $key,
            &$crate::i18n::Args::new()$(.with(stringify!($name), $value))+,
        )
    };
}

/// The spans of a message with tags: `rich!` with its key and arguments, as [`t!`].
#[macro_export]
macro_rules! rich {
    ($key:literal) => {
        $crate::i18n::spans($key, &$crate::i18n::Args::new())
    };
    ($key:literal, $($name:ident = $value:expr),+ $(,)?) => {
        $crate::i18n::spans(
            $key,
            &$crate::i18n::Args::new()$(.rich(stringify!($name), $value))+,
        )
    };
}

#[cfg(test)]
mod tests;

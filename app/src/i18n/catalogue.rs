//! The catalogues: one folder of Fluent files per language under
//! `app/i18n`, built into the binary, and the bundle that renders one
//! language's messages.

use std::{borrow::Cow, collections::BTreeMap, sync::LazyLock};

use fluent_bundle::{concurrent::FluentBundle, FluentArgs, FluentResource, FluentValue};
use fluent_syntax::{ast, parser};
use icu::decimal::DecimalFormatter;
use include_dir::{include_dir, Dir};
use unic_langid::LanguageIdentifier;

use super::{direction::Direction, number};

static FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/i18n");

/// The language every other falls back to, and the one keys are written in.
pub const DEFAULT_LOCALE: &str = "en";

/// A language the app has a catalogue for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locale {
    /// Its tag, as stored in Settings: `en`, `fr`, `pt-BR`.
    pub tag: &'static str,
    /// The language's own name for itself: `Français`, `日本語`.
    pub name: String,
    pub direction: Direction,
}

/// The app's languages: English first, then by tag. Every folder of
/// `app/i18n` is one, so adding a language needs no code.
pub static LOCALES: LazyLock<Vec<Locale>> = LazyLock::new(|| {
    let mut locales: Vec<Locale> = FILES
        .dirs()
        .filter_map(|dir| {
            let tag = dir.path().file_name()?.to_str()?;
            let meta = Catalogue::of(tag, &["meta"]);
            Some(Locale {
                tag,
                name: meta.own("meta-name", None)?,
                direction: match meta.own("meta-dir", None)?.as_str() {
                    "rtl" => Direction::Rtl,
                    _ => Direction::Ltr,
                },
            })
        })
        .collect();
    locales.sort_by_key(|l| (l.tag != DEFAULT_LOCALE, l.tag));
    locales
});

/// Whether a Fluent file says its language is written right to left.
fn is_rtl(text: &str) -> bool {
    text.lines()
        .filter_map(|line| line.split_once('='))
        .any(|(key, value)| key.trim() == "meta-dir" && value.trim() == "rtl")
}

/// The Fluent files of a language, as (namespace, text).
pub fn files(tag: &str) -> Vec<(&'static str, &'static str)> {
    let Some(dir) = FILES.get_dir(tag) else {
        return vec![];
    };
    let mut files: Vec<_> = dir
        .files()
        .filter(|f| f.path().extension().is_some_and(|e| e == "ftl"))
        .filter_map(|f| Some((f.path().file_stem()?.to_str()?, f.contents_utf8()?)))
        .collect();
    files.sort_unstable();
    files
}

/// One language's messages.
pub struct Catalogue {
    bundle: FluentBundle<FluentResource>,
}

impl Catalogue {
    /// Every namespace of `tag`.
    pub fn load(tag: &str) -> Self {
        Self::build(tag, files(tag).into_iter().map(|(_, text)| text))
    }

    /// Only the `namespaces` of `tag`.
    fn of(tag: &str, namespaces: &[&str]) -> Self {
        let wanted = files(tag)
            .into_iter()
            .filter(|(name, _)| namespaces.contains(name));
        Self::build(tag, wanted.map(|(_, text)| text))
    }

    fn build(tag: &str, texts: impl Iterator<Item = &'static str>) -> Self {
        let texts: Vec<&'static str> = texts.collect();
        let language: LanguageIdentifier = tag.parse().unwrap_or_default();
        let mut bundle = FluentBundle::new_concurrent(vec![language]);
        // What is put into a sentence (a path, a name, a number) is told
        // apart from it with Unicode's isolates in a right-to-left
        // language, or the bidi algorithm reorders it with the sentence:
        // `/var/run/docker.sock` in Arabic read `.var/run/docker.sock/`.
        // Left to right, nothing needs them and the text stays as written.
        bundle.set_use_isolating(texts.iter().any(|text| is_rtl(text)));
        let numbers = number::formatter(tag);
        let added = bundle.add_function("NUM", move |positional, _named| {
            num(numbers.as_ref(), positional)
        });
        if added.is_err() {
            log::error!("the {tag} catalogue has no NUM function");
        }
        for text in texts {
            // What parses is kept; the catalogue check fails on the rest.
            let resource = FluentResource::try_new(text.to_owned()).unwrap_or_else(|(r, _)| r);
            if let Err(errors) = bundle.add_resource(resource) {
                log::error!("the {tag} catalogue names a message twice: {errors:?}");
            }
        }
        Self { bundle }
    }

    /// Whether this language has `id` itself.
    pub fn has(&self, id: &str) -> bool {
        self.bundle.has_message(id)
    }

    /// The text of `id` with `args`, if this language has it.
    pub fn own(&self, id: &str, args: Option<&FluentArgs<'_>>) -> Option<String> {
        let pattern = self.bundle.get_message(id)?.value()?;
        let mut errors = vec![];
        let text = self.bundle.format_pattern(pattern, args, &mut errors);
        if !errors.is_empty() {
            log::warn!("the message {id} was rendered with problems: {errors:?}");
        }
        Some(text.into_owned())
    }
}

/// `NUM($count)`: the count as the language writes it.
fn num<'a>(numbers: Option<&DecimalFormatter>, positional: &[FluentValue<'a>]) -> FluentValue<'a> {
    match positional.first() {
        Some(FluentValue::Number(n)) => match numbers {
            Some(numbers) => FluentValue::String(Cow::Owned(number::format(numbers, n.value))),
            None => FluentValue::Number(n.clone()),
        },
        Some(other) => other.clone(),
        None => FluentValue::Error,
    }
}

/// The namespaces the crates render themselves, for the text the OS draws:
/// the tray menu and notifications, and the errors a notification quotes.
const NATIVE: [&str; 2] = ["native", "errors"];

/// The templates of `tag` the crates render (`homeport_core::error`), by
/// dotted key, as `Text {name}`. A message with a plural or a select has no
/// such template and stays in English there.
pub fn native_templates(tag: &str) -> BTreeMap<String, String> {
    let mut templates = BTreeMap::new();
    for (_, text) in files(tag)
        .into_iter()
        .filter(|(name, _)| NATIVE.contains(name))
    {
        let resource = parser::parse(text).unwrap_or_else(|(resource, _)| resource);
        for entry in &resource.body {
            let ast::Entry::Message(message) = entry else {
                continue;
            };
            if let Some(template) = message.value.as_ref().and_then(template) {
                templates.insert(message.id.name.replace('-', "."), template);
            }
        }
    }
    templates
}

/// A pattern of text and plain arguments as the crates' template.
fn template(pattern: &ast::Pattern<&str>) -> Option<String> {
    let mut out = String::new();
    for element in &pattern.elements {
        match element {
            ast::PatternElement::TextElement { value } => out.push_str(value),
            ast::PatternElement::Placeable {
                expression: ast::Expression::Inline(inline),
            } => match inline {
                ast::InlineExpression::VariableReference { id } => {
                    out.push('{');
                    out.push_str(id.name);
                    out.push('}');
                }
                ast::InlineExpression::StringLiteral { value } => {
                    out.push_str(&value.replace("\\u000A", "\n"));
                }
                _ => return None,
            },
            ast::PatternElement::Placeable { .. } => return None,
        }
    }
    Some(out)
}

/// `containers.empty.title` as Fluent names it: its ids have no dots.
pub fn id(key: &str) -> String {
    key.replace('.', "-")
}

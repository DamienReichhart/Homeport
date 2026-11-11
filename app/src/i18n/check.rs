//! The catalogue check (ADR 0011), on every test run: each Fluent file
//! parses, every language has exactly the English messages with the same
//! arguments and tags, every plural has the categories its language needs,
//! the code only names messages that exist, and the catalogues render what
//! formatjs renders from the catalogues they were written from.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use fluent_syntax::{ast, parser};
use intl_pluralrules::{PluralRuleType, PluralRules};
use unic_langid::LanguageIdentifier;

use super::{
    catalogue::{self, DEFAULT_LOCALE, LOCALES},
    Arg, Args, Language,
};

/// The namespaces whose pages are all ported: each of their messages must
/// be named by the code. The others still wait for their page, so an
/// unnamed message there isn't a leftover yet. The cleanup ticket ends this
/// list by checking every namespace.
const PORTED: &[&str] = &[];

/// Messages the code reaches without naming them in a `t!`: the languages'
/// own names, and the errors and OS text the crates name by key.
const NAMED_ELSEWHERE: &[&str] = &["meta", "errors", "native", "format"];

const CATEGORIES: [&str; 6] = ["zero", "one", "two", "few", "many", "other"];

/// What a message names.
#[derive(Debug, Default, PartialEq, Eq)]
struct Shape {
    variables: BTreeSet<String>,
    tags: BTreeSet<String>,
    /// The categories of each plural, by its variable.
    plurals: BTreeMap<String, BTreeSet<String>>,
}

fn inline(expression: &ast::InlineExpression<&str>, shape: &mut Shape) {
    match expression {
        ast::InlineExpression::VariableReference { id } => {
            shape.variables.insert(id.name.to_owned());
        }
        ast::InlineExpression::FunctionReference { arguments, .. } => {
            for argument in &arguments.positional {
                inline(argument, shape);
            }
        }
        ast::InlineExpression::Placeable { expression } => walk_expression(expression, shape),
        _ => {}
    }
}

fn walk_expression(expression: &ast::Expression<&str>, shape: &mut Shape) {
    match expression {
        ast::Expression::Inline(expression) => inline(expression, shape),
        ast::Expression::Select { selector, variants } => {
            inline(selector, shape);
            let names: Vec<&str> = variants
                .iter()
                .filter_map(|v| match &v.key {
                    ast::VariantKey::Identifier { name } => Some(*name),
                    ast::VariantKey::NumberLiteral { .. } => None,
                })
                .collect();
            let plural = names.iter().all(|name| CATEGORIES.contains(name));
            if let (true, ast::InlineExpression::VariableReference { id }) = (plural, selector) {
                shape
                    .plurals
                    .entry(id.name.to_owned())
                    .or_default()
                    .extend(names.iter().map(|n| (*n).to_owned()));
            }
            for variant in variants {
                walk(&variant.value, shape);
            }
        }
    }
}

fn walk(pattern: &ast::Pattern<&str>, shape: &mut Shape) {
    for element in &pattern.elements {
        match element {
            ast::PatternElement::TextElement { value } => {
                let mut rest = *value;
                while let Some(open) = rest.find('<') {
                    rest = &rest[open + 1..];
                    let name: String = rest
                        .chars()
                        .take_while(char::is_ascii_alphanumeric)
                        .collect();
                    if !name.is_empty() && rest[name.len()..].starts_with('>') {
                        shape.tags.insert(name);
                    }
                }
            }
            ast::PatternElement::Placeable { expression } => walk_expression(expression, shape),
        }
    }
}

/// The shapes of a language's messages by id, and what's wrong with its files.
fn shapes(tag: &str, problems: &mut Vec<String>) -> BTreeMap<String, Shape> {
    let mut found = BTreeMap::new();
    for (namespace, text) in catalogue::files(tag) {
        let resource = match parser::parse(text) {
            Ok(resource) => resource,
            Err((resource, errors)) => {
                problems.push(format!("{tag}/{namespace}.ftl doesn't parse: {errors:?}"));
                resource
            }
        };
        for entry in &resource.body {
            let ast::Entry::Message(message) = entry else {
                continue;
            };
            let mut shape = Shape::default();
            if let Some(pattern) = &message.value {
                walk(pattern, &mut shape);
            }
            let id = message.id.name;
            if !id.starts_with(&format!("{namespace}-")) {
                problems.push(format!(
                    "{tag}/{namespace}.ftl holds {id}, of another namespace"
                ));
            }
            if found.insert(id.to_owned(), shape).is_some() {
                problems.push(format!("{tag} has {id} twice"));
            }
        }
    }
    found
}

/// The plural categories counts up to 1000 fall in: French's `many` only
/// covers millions, which falls back to `other` without harm.
fn categories(tag: &str) -> BTreeSet<String> {
    let language: LanguageIdentifier = tag.parse().unwrap();
    let base: LanguageIdentifier = language.language.as_str().parse().unwrap();
    let rules = PluralRules::create(language, PluralRuleType::CARDINAL)
        .or_else(|_| PluralRules::create(base, PluralRuleType::CARDINAL))
        .unwrap_or_else(|_| panic!("no plural rules for {tag}"));
    (0..=1000u32)
        .map(|n| format!("{:?}", rules.select(n).unwrap()).to_lowercase())
        .collect()
}

fn assert_none(problems: &[String]) {
    assert!(
        problems.is_empty(),
        "{} catalogue problem(s); see docs/i18n/README.md:\n{}",
        problems.len(),
        problems.join("\n")
    );
}

#[test]
fn every_language_has_the_english_messages_with_their_shape() {
    let mut problems = vec![];
    let english = shapes(DEFAULT_LOCALE, &mut problems);
    assert!(english.len() > 4000, "the English catalogue is built in");
    assert_eq!(LOCALES.len(), 12, "English and the 11 languages");
    for locale in LOCALES.iter() {
        let tag = locale.tag;
        let own = shapes(tag, &mut problems);
        let needed = categories(tag);
        for (id, shape) in &english {
            let Some(translated) = own.get(id) else {
                problems.push(format!("{tag} lacks {id}"));
                continue;
            };
            if translated.variables != shape.variables {
                problems.push(format!(
                    "{tag} {id} takes {:?}, English {:?}",
                    translated.variables, shape.variables
                ));
            }
            if translated.tags != shape.tags {
                problems.push(format!(
                    "{tag} {id} has the tags {:?}, English {:?}",
                    translated.tags, shape.tags
                ));
            }
            for (variable, given) in &translated.plurals {
                let missing: Vec<_> = needed.difference(given).collect();
                if !missing.is_empty() {
                    problems.push(format!("{tag} {id} lacks {missing:?} for ${variable}"));
                }
            }
        }
        for id in own.keys().filter(|id| !english.contains_key(*id)) {
            problems.push(format!("{tag} has {id}, which English doesn't"));
        }
    }
    assert_none(&problems);
}

/// Every key a `t!` or a `rich!` of the sources names.
fn named_by_the_code() -> BTreeMap<String, String> {
    fn scan(dir: &Path, found: &mut BTreeMap<String, String>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                scan(&path, found);
            } else if path.extension().is_some_and(|e| e == "rs")
                && !path.ends_with("i18n/check.rs")
            {
                let text = fs::read_to_string(&path).unwrap();
                for marker in ["t!(", "rich!("] {
                    for (at, _) in text.match_indices(marker) {
                        let before = text[..at].chars().next_back();
                        if before.is_some_and(|c| c.is_alphanumeric() || c == '_') {
                            continue;
                        }
                        let rest = text[at + marker.len()..].trim_start();
                        if let Some(key) = rest.strip_prefix('"').and_then(|r| r.split('"').next())
                        {
                            found.insert(key.to_owned(), path.display().to_string());
                        }
                    }
                }
            }
        }
    }
    let mut found = BTreeMap::new();
    scan(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut found,
    );
    found
}

#[test]
fn the_code_and_the_english_catalogue_name_the_same_messages() {
    let mut problems = vec![];
    let english = shapes(DEFAULT_LOCALE, &mut problems);
    let named = named_by_the_code();
    for (key, file) in &named {
        if !english.contains_key(&catalogue::id(key)) {
            problems.push(format!("{file} names {key}, which no catalogue has"));
        }
    }
    let used: BTreeSet<String> = named.keys().map(|key| catalogue::id(key)).collect();
    for id in english.keys() {
        let namespace = id.split('-').next().unwrap_or_default();
        if PORTED.contains(&namespace)
            && !NAMED_ELSEWHERE.contains(&namespace)
            && !used.contains(id)
        {
            problems.push(format!("{id} is named by no code"));
        }
    }
    assert_none(&problems);
}

type Sample = (String, BTreeMap<String, serde_json::Value>, String);

#[test]
fn the_catalogues_render_what_formatjs_renders() {
    let golden = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/i18n-golden.json");
    let golden: BTreeMap<String, Vec<Sample>> =
        serde_json::from_str(&fs::read_to_string(golden).unwrap()).unwrap();
    assert_eq!(golden.len(), LOCALES.len());
    let mut problems = vec![];
    let mut checked = 0;
    for (tag, samples) in &golden {
        let language = Language::new(tag, &[]);
        assert_eq!(language.locale.tag, tag);
        for (key, values, expected) in samples {
            let mut args = Args::new();
            for (name, value) in values {
                let value = match value {
                    serde_json::Value::Number(n) => Arg::Number(n.as_f64().unwrap()),
                    other => Arg::Text(other.as_str().unwrap().to_owned()),
                };
                args.set_owned(name.clone(), value);
            }
            let text = language.text(key, Some(&args.0));
            if &text != expected {
                problems.push(format!(
                    "{tag} {key} {values:?}: {text:?}, formatjs {expected:?}"
                ));
            }
            checked += 1;
        }
    }
    assert!(checked > 4000, "only {checked} renderings were compared");
    assert_none(&problems);
}

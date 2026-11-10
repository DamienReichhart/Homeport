//! Removing secrets, and on request names, from log text.
//!
//! Homeport doesn't log secrets on purpose, but a log quotes what Engines,
//! SSH servers, git and Clusters answer, and those answers can carry one: a
//! URL with a token, a header, a kubeconfig. Each rule below removes one
//! such shape; what it removes is replaced by `[redacted]`, so the line
//! still reads.

use std::sync::LazyLock;

use regex::{Captures, Regex};

const REDACTED: &str = "[redacted]";

/// The names and addresses to replace in logs, each with its label.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Aliases(Vec<(String, String)>);

impl Aliases {
    /// Replaces `name` with `label` wherever it appears as a word.
    pub fn add(&mut self, name: &str, label: &str) {
        let name = name.trim();
        // A one-letter name would rewrite half the log.
        if name.chars().count() >= 2 {
            self.0.push((name.to_owned(), label.to_owned()));
        }
    }

    /// Replaces an address (`host:port`, or a URL) and its host on its own.
    pub fn add_address(&mut self, address: &str, label: &str) {
        let label = format!("{label}-address");
        let without_scheme = address.split("://").last().unwrap_or(address);
        let authority = without_scheme.split(['/', '?']).next().unwrap_or("");
        let host = match authority.strip_prefix('[') {
            // `[::1]:2376`
            Some(rest) => rest.split(']').next().unwrap_or(""),
            None => authority.rsplit_once(':').map_or(authority, |(h, _)| h),
        };
        self.add(address, &label);
        self.add(authority, &label);
        if !matches!(host, "localhost" | "127.0.0.1" | "::1") {
            self.add(host, &label);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Longest first, so `prod.example.com:2376` goes before `prod`.
    fn sorted(&self) -> Vec<&(String, String)> {
        let mut all: Vec<_> = self.0.iter().collect();
        all.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));
        all.dedup_by(|a, b| a.0 == b.0);
        all
    }
}

fn rule(pattern: &str) -> Regex {
    Regex::new(pattern).expect("a valid redaction rule")
}

/// A PEM block, on several lines or with its line breaks escaped.
static PEM: LazyLock<Regex> =
    LazyLock::new(|| rule(r"-----BEGIN ([A-Z0-9 ]+)-----(?s:.*?)-----END ([A-Z0-9 ]+)-----"));
/// A PEM block cut off before its end, to the end of the text.
static PEM_OPEN: LazyLock<Regex> = LazyLock::new(|| rule(r"-----BEGIN ([A-Z0-9 ]+)-----(?s:.*)"));
/// A header that carries a credential, with its scheme.
static HEADER: LazyLock<Regex> = LazyLock::new(|| {
    rule(
        r#"(?i)\b((?:proxy-)?authorization|x-nomad-token|x-vault-token|x-registry-auth|x-auth-token|cookie|set-cookie)("?\s*[:=]\s*"?)(?:(bearer|basic|token)\s+)?[^\s"',;]+"#,
    )
});
/// `Bearer …` or `Basic …` on its own.
static SCHEME: LazyLock<Regex> =
    LazyLock::new(|| rule(r"(?i)\b(bearer|basic)\s+[A-Za-z0-9._~+/=-]{8,}"));
/// `key=value`, `key: value` and `"key": "value"` for keys that name a secret.
static KEY_VALUE: LazyLock<Regex> = LazyLock::new(|| {
    rule(
        r#"(?i)\b([a-z0-9_.-]*(?:password|passwd|passphrase|secret|token|api[_-]?key|access[_-]?key|private[_-]?key|client-key-data|client-certificate-data|certificate-authority-data|credential|auth)[a-z0-9_-]*)("?\s*[:=]\s*)("[^"]*"|'[^']*'|[^\s&,;"'}\]]+)"#,
    )
});
/// The user and password of a URL: `https://user:secret@host`.
static URL_USERINFO: LazyLock<Regex> =
    LazyLock::new(|| rule(r"\b([a-z][a-z0-9+.-]*://)[^\s/@:]+(?::[^\s/@]*)?@"));
/// Tokens whose shape gives them away, wherever they are.
static KNOWN_TOKENS: LazyLock<Regex> = LazyLock::new(|| {
    rule(
        r"\b(?:gh[pousr]_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|glpat-[A-Za-z0-9_-]{16,}|dckr_pat_[A-Za-z0-9_-]{16,}|eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,})",
    )
});

/// `text` without the secrets a log can carry, and with every name and
/// address in `aliases` replaced by its label.
pub fn redact(text: &str, aliases: &Aliases) -> String {
    let text = PEM.replace_all(text, |c: &Captures<'_>| format!("[redacted {}]", &c[1]));
    let text = PEM_OPEN.replace_all(&text, |c: &Captures<'_>| format!("[redacted {}]", &c[1]));
    let text = URL_USERINFO.replace_all(&text, |c: &Captures<'_>| format!("{}{REDACTED}@", &c[1]));
    let text = HEADER.replace_all(&text, |c: &Captures<'_>| {
        format!("{}{}{REDACTED}", &c[1], &c[2])
    });
    let text = SCHEME.replace_all(&text, |c: &Captures<'_>| format!("{} {REDACTED}", &c[1]));
    let text = KEY_VALUE.replace_all(&text, |c: &Captures<'_>| {
        // What an earlier rule left stays as it is.
        if c[3].starts_with("[redacted") {
            c[0].to_owned()
        } else {
            format!("{}{}{REDACTED}", &c[1], &c[2])
        }
    });
    let mut text = KNOWN_TOKENS.replace_all(&text, REDACTED).into_owned();
    for (name, label) in aliases.sorted() {
        text = replace_word(&text, name, label);
    }
    text
}

/// Replaces `name` where it isn't part of a longer word, whatever its case.
fn replace_word(text: &str, name: &str, label: &str) -> String {
    let pattern = format!(r"(?i){}", regex::escape(name));
    let Ok(pattern) = Regex::new(&pattern) else {
        return text.to_owned();
    };
    let is_word = |c: char| c.is_alphanumeric() || c == '_';
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for found in pattern.find_iter(text) {
        let before = text[..found.start()].chars().next_back();
        let after = text[found.end()..].chars().next();
        let starts = name.chars().next().is_some_and(is_word);
        let ends = name.chars().next_back().is_some_and(is_word);
        if (starts && before.is_some_and(is_word)) || (ends && after.is_some_and(is_word)) {
            continue;
        }
        out.push_str(&text[last..found.start()]);
        out.push_str(label);
        last = found.end();
    }
    out.push_str(&text[last..]);
    out
}

#[cfg(test)]
mod tests;

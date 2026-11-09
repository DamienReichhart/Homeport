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
mod tests {
    use super::*;

    /// Realistic log lines, one per kind of secret, each with the secret it
    /// must lose and something it must keep. Every secret is made up; the
    /// ones shaped like a real service's token are written in two pieces, so
    /// secret scanners don't take this file for a leak.
    const CORPUS: &[(&str, &str, &str)] = &[
        (
            "[2026-11-02][10:01:12][homeport_hosts][WARN] ssh key rejected: -----BEGIN OPENSSH PRIVATE KEY-----\nb3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQ\nAAAAMwAAAAtzc2gtZWQyNTUxOQ\n-----END OPENSSH PRIVATE KEY-----\n",
            "b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQ",
            "ssh key rejected",
        ),
        (
            r#"[WARN] kubeconfig: {"client-key":"-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAvX\n-----END RSA PRIVATE KEY-----\n"}"#,
            "MIIEowIBAAKCAQEAvX",
            "kubeconfig",
        ),
        (
            "[ERROR] TLS material invalid: -----BEGIN PRIVATE KEY-----\nMIGHAgEAMBMGByqGSM49AgEGCCqGSM49",
            "MIGHAgEAMBMGByqGSM49",
            "TLS material invalid",
        ),
        (
            "[WARN] registry answered 401 for GET /v2/ with Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJvY3RvIn0.c2lnbmF0dXJlLXZhbHVl",
            "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9",
            "registry answered 401",
        ),
        (
            "[WARN] hub login failed: authorization: Basic b2N0bzpodW50ZXIy",
            "b2N0bzpodW50ZXIy",
            "hub login failed",
        ),
        (
            "[WARN] nomad refused the request (X-Nomad-Token: 5b1d7c3e-9f4a-4c2b-8e1d-2a6f0c9b7e34) with 403",
            "5b1d7c3e-9f4a-4c2b-8e1d-2a6f0c9b7e34",
            "with 403",
        ),
        (
            "[ERROR] compose exited 1: environment DB_PASSWORD=hunter2-s3cret could not be interpolated",
            "hunter2-s3cret",
            "could not be interpolated",
        ),
        (
            concat!(
                "[WARN] git fetch failed: fatal: unable to access 'https://octo:gh",
                "p_16C7e42F292c6912E7710c838347Ae178B4a@github.com/acme/infra.git/': timeout"
            ),
            concat!("gh", "p_16C7e42F292c6912E7710c838347Ae178B4a"),
            "github.com/acme/infra.git",
        ),
        (
            "[WARN] GET https://registry.example.com/token?service=registry&token=abc123def456ghi789&scope=pull failed",
            "abc123def456ghi789",
            "scope=pull",
        ),
        (
            "[WARN] kubeconfig user:\n    token: sha256~Zm9vYmFyYmF6cXV4\n    client-key-data: LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0t\n",
            "sha256~Zm9vYmFyYmF6cXV4",
            "kubeconfig user",
        ),
        (
            "[WARN] kubeconfig user:\n    client-key-data: LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0t\n",
            "LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0t",
            "kubeconfig user",
        ),
        (
            r#"[WARN] incus answered: {"error":"not authorized","trust_token":"eyJjbGllbnRfbmFtZSI6ImhvbWVwb3J0In0"}"#,
            "eyJjbGllbnRfbmFtZSI6ImhvbWVwb3J0In0",
            "not authorized",
        ),
        (
            r#"[WARN] helm: Error: failed to fetch (password: "p@ss word!")"#,
            "p@ss word!",
            "failed to fetch",
        ),
        (
            "[WARN] ssh: passphrase='correct horse battery' rejected for key",
            "correct horse battery",
            "rejected for key",
        ),
        (
            concat!(
                "[WARN] pull failed, the engine said: unauthorized gl",
                "pat-xYz12345AbCdEfGhIjKl for project"
            ),
            concat!("gl", "pat-xYz12345AbCdEfGhIjKl"),
            "for project",
        ),
        (
            concat!(
                "[WARN] docker login: dckr_",
                "pat_AbCdEfGhIjKlMnOpQrStUvWx was refused"
            ),
            concat!("dckr_", "pat_AbCdEfGhIjKlMnOpQrStUvWx"),
            "was refused",
        ),
        (
            "[WARN] oidc refresh failed: id-token=eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJrOHMifQ.c2lnbmF0dXJl refresh-token=q1w2e3r4t5y6",
            "q1w2e3r4t5y6",
            "oidc refresh failed",
        ),
        (
            "[WARN] webhook secret=whsec_0123456789abcdef; retrying",
            "whsec_0123456789abcdef",
            "retrying",
        ),
        (
            concat!(
                "[WARN] AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG",
                "/bPxRfiCYEXAMPLEKEY exec plugin failed"
            ),
            concat!("wJalrXUtnFEMI/K7MDENG", "/bPxRfiCYEXAMPLEKEY"),
            "exec plugin failed",
        ),
    ];

    #[test]
    fn no_secret_of_the_corpus_survives() {
        for (line, secret, kept) in CORPUS {
            let clean = redact(line, &Aliases::default());
            assert!(!clean.contains(secret), "{secret} survives in {clean:?}");
            assert!(clean.contains(kept), "{kept:?} is gone from {clean:?}");
            assert!(clean.contains("[redacted"), "{clean:?}");
        }
    }

    #[test]
    fn a_secret_is_replaced_once() {
        assert_eq!(
            redact(
                "Authorization: Bearer abcdefgh12345678 refused",
                &Aliases::default()
            ),
            "Authorization: [redacted] refused"
        );
        assert_eq!(
            redact(
                "token=abc123def456 and password: hunter2",
                &Aliases::default()
            ),
            "token=[redacted] and password: [redacted]"
        );
    }

    #[test]
    fn ordinary_lines_are_left_alone() {
        for line in [
            "[2026-11-02][10:01:12][homeport_hosts::registry][INFO] host prod connected: Docker Engine 29.1.0, API 1.52",
            "[INFO] deployed stack shop on laptop in 4.2s (3 services)",
            "[WARN] failed to start host connections: connection refused (os error 111)",
            "[INFO] checked for updates: 1.0.0 is the latest",
            "[WARN] the author of this image is unknown",
        ] {
            assert_eq!(redact(line, &Aliases::default()), line);
        }
    }

    #[test]
    fn names_and_addresses_become_their_labels() {
        let mut aliases = Aliases::default();
        aliases.add("prod", "host-1");
        aliases.add_address("prod.example.com:2376", "host-1");
        aliases.add("Kind Dev", "cluster-1");
        aliases.add_address("https://10.1.2.3:6443", "cluster-1");
        let clean = redact(
            "[INFO] host prod (prod.example.com:2376) connected; PROD is up; production is not it\n\
             [WARN] cluster Kind Dev at https://10.1.2.3:6443/api unreachable, 10.1.2.3 timed out",
            &aliases,
        );
        assert_eq!(
            clean,
            "[INFO] host host-1 (host-1-address) connected; host-1 is up; production is not it\n\
             [WARN] cluster cluster-1 at cluster-1-address/api unreachable, cluster-1-address timed out"
        );
    }

    #[test]
    fn this_computer_is_not_an_address_to_hide() {
        let mut aliases = Aliases::default();
        aliases.add_address("127.0.0.1:2375", "host-1");
        let clean = redact("dial 127.0.0.1:2375 failed; 127.0.0.1 refused", &aliases);
        assert_eq!(clean, "dial host-1-address failed; 127.0.0.1 refused");
    }
}

//! `.dockerignore`, with the Docker CLI's rules: one pattern per line, `#`
//! comments, `!` exceptions, `*`, `?`, `[…]` and `**`; the last pattern that
//! matches a path, or one of its parent folders, decides.

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pattern {
    /// Path segments joined with `/`, with no leading `/` or `./`.
    glob: String,
    exception: bool,
}

#[derive(Debug, Default, Clone)]
pub(super) struct DockerIgnore {
    patterns: Vec<Pattern>,
}

impl DockerIgnore {
    pub(super) fn parse(text: &str) -> Self {
        let patterns = text
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .filter_map(|line| {
                let (exception, rest) = match line.strip_prefix('!') {
                    Some(rest) => (true, rest.trim()),
                    None => (false, line),
                };
                let glob = clean(rest);
                (!glob.is_empty()).then_some(Pattern { glob, exception })
            })
            .collect();
        Self { patterns }
    }

    /// Whether `path` (relative to the context, `/`-separated) is left out.
    pub(super) fn excludes(&self, path: &str) -> bool {
        let path = clean(path);
        let mut excluded = false;
        for pattern in &self.patterns {
            if excluded == !pattern.exception {
                // This pattern can't change the outcome.
                continue;
            }
            if matches_or_parent(&pattern.glob, &path) {
                excluded = !pattern.exception;
            }
        }
        excluded
    }

    /// Whether some `!` pattern could bring back a file inside an excluded folder.
    pub(super) fn has_exceptions(&self) -> bool {
        self.patterns.iter().any(|p| p.exception)
    }
}

/// Docker cleans patterns like paths: no `./`, no leading or trailing `/`,
/// no empty segments.
fn clean(path: &str) -> String {
    path.split('/')
        .filter(|s| !s.is_empty() && *s != ".")
        .collect::<Vec<_>>()
        .join("/")
}

/// A pattern matches a path, or any of the folders it's in.
fn matches_or_parent(glob: &str, path: &str) -> bool {
    if glob_match(glob.as_bytes(), path.as_bytes()) {
        return true;
    }
    path.match_indices('/')
        .any(|(i, _)| glob_match(glob.as_bytes(), &path.as_bytes()[..i]))
}

/// Shell-style matching where `*` and `?` stop at `/`, and `**` crosses it.
fn glob_match(pattern: &[u8], text: &[u8]) -> bool {
    match pattern {
        [] => text.is_empty(),
        [b'*', b'*', rest @ ..] => {
            // `**/` also matches no folder at all.
            let rest = rest.strip_prefix(b"/").unwrap_or(rest);
            if rest.is_empty() {
                return true;
            }
            (0..=text.len())
                .any(|i| (i == 0 || text[i - 1] == b'/') && glob_match(rest, &text[i..]))
        }
        [b'*', rest @ ..] => {
            let mut i = 0;
            loop {
                if glob_match(rest, &text[i..]) {
                    return true;
                }
                if i == text.len() || text[i] == b'/' {
                    return false;
                }
                i += 1;
            }
        }
        [b'?', rest @ ..] => {
            matches!(text.first(), Some(c) if *c != b'/') && glob_match(rest, &text[1..])
        }
        [b'[', rest @ ..] => match (class_match(rest, text.first().copied()), text) {
            (Some((true, after)), [_, tail @ ..]) => glob_match(after, tail),
            _ => false,
        },
        [b'\\', c, rest @ ..] => text.first() == Some(c) && glob_match(rest, &text[1..]),
        [c, rest @ ..] => text.first() == Some(c) && glob_match(rest, &text[1..]),
    }
}

/// Matches one character against a `[…]` class (after its `[`); returns
/// whether it matched and the pattern after the `]`. `None` for a class
/// with no closing `]`.
fn class_match(class: &[u8], c: Option<u8>) -> Option<(bool, &[u8])> {
    let c = c.filter(|c| *c != b'/')?;
    let (negated, mut rest) = match class {
        [b'!' | b'^', rest @ ..] => (true, rest),
        _ => (false, class),
    };
    let mut matched = false;
    let mut first = true;
    loop {
        match rest {
            [] => return None,
            [b']', after @ ..] if !first => return Some((matched != negated, after)),
            [lo, b'-', hi, after @ ..] if *hi != b']' => {
                matched |= (*lo..=*hi).contains(&c);
                rest = after;
            }
            [b'\\', x, after @ ..] | [x, after @ ..] => {
                matched |= *x == c;
                rest = after;
            }
        }
        first = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ignore(text: &str) -> DockerIgnore {
        DockerIgnore::parse(text)
    }

    #[test]
    fn plain_names_exclude_files_and_whole_folders() {
        let i = ignore("node_modules\n# a comment\n\n.git\nsecret.env\n");
        assert!(i.excludes("node_modules"));
        assert!(i.excludes("node_modules/react/index.js"));
        assert!(i.excludes(".git/HEAD"));
        assert!(i.excludes("secret.env"));
        assert!(!i.excludes("src/node_modules_backup.txt"));
        assert!(
            !i.excludes("src/secret.env"),
            "patterns are anchored at the context"
        );
        assert!(!i.excludes("Dockerfile"));
    }

    #[test]
    fn stars_stay_within_a_folder() {
        let i = ignore("*.log\ntemp?\nbuild/*.o\n");
        assert!(i.excludes("debug.log"));
        assert!(!i.excludes("logs/debug.log"));
        assert!(i.excludes("temp1"));
        assert!(!i.excludes("temp12"));
        assert!(i.excludes("build/a.o"));
        assert!(!i.excludes("build/sub/a.o"));
    }

    #[test]
    fn double_stars_cross_folders() {
        let i = ignore("**/*.log\n**/cache\ndocs/**\n");
        assert!(i.excludes("debug.log"));
        assert!(i.excludes("a/b/c/debug.log"));
        assert!(i.excludes("cache/x"));
        assert!(i.excludes("deep/in/cache/x"));
        assert!(i.excludes("docs/a/b.md"));
        assert!(!i.excludes("src/main.rs"));
    }

    #[test]
    fn exceptions_bring_files_back_and_the_last_match_wins() {
        let i = ignore("*.md\n!README.md\nREADME*.md\n!README-keep.md\n");
        assert!(i.excludes("CHANGES.md"));
        assert!(
            i.excludes("README.md"),
            "README*.md comes after the exception"
        );
        assert!(!i.excludes("README-keep.md"));
        assert!(i.has_exceptions());

        let i = ignore("docs\n!docs/keep.txt\n");
        assert!(i.excludes("docs/other.txt"));
        assert!(!i.excludes("docs/keep.txt"));
    }

    #[test]
    fn patterns_are_cleaned_like_paths() {
        let i = ignore("./dist/\n/target\n");
        assert!(i.excludes("dist/app.js"));
        assert!(i.excludes("target/debug/x"));
        assert!(!ignore("").has_exceptions());
    }

    #[test]
    fn classes_and_escapes() {
        let i = ignore("file[0-9].txt\n[!a]x\n\\*literal\n");
        assert!(i.excludes("file3.txt"));
        assert!(!i.excludes("filea.txt"));
        assert!(i.excludes("bx"));
        assert!(!i.excludes("ax"));
        assert!(i.excludes("*literal"));
        assert!(!i.excludes("xliteral"));
    }
}

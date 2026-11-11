//! The subset of `ssh_config(5)` Homeport reads: `Host` blocks with their
//! patterns, options in them, and `Include`. `Match` blocks are skipped.

use std::path::{Path, PathBuf};

/// How deep `Include` may nest before Homeport stops following it, as ssh does.
const MAX_INCLUDE_DEPTH: usize = 16;

/// A parsed config: its blocks in file order, includes expanded in place.
#[derive(Debug, Default)]
pub(crate) struct SshConfig {
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    patterns: Vec<Pattern>,
    /// Lower-case keyword and its value, in the order written.
    options: Vec<(String, String)>,
}

#[derive(Debug)]
struct Pattern {
    glob: String,
    negated: bool,
}

/// What the config says about one alias: the first value of each option,
/// and every `IdentityFile`, as ssh applies them.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Resolved {
    pub host_name: Option<String>,
    pub user: Option<String>,
    pub port: Option<String>,
    pub identity_files: Vec<String>,
    pub proxy_jump: Option<String>,
    pub proxy_command: Option<String>,
}

impl SshConfig {
    /// Reads the config at `path`. `ssh_dir` is where relative `Include`
    /// paths start (`~/.ssh`), `home` what `~` stands for.
    pub(crate) fn read(path: &Path, ssh_dir: &Path, home: &Path) -> std::io::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        let mut config = Self::default();
        let mut parser = Parser {
            config: &mut config,
            current: None,
            ssh_dir,
            home,
        };
        parser.parse(&text, 0);
        Ok(config)
    }

    #[cfg(test)]
    pub(crate) fn parse(text: &str) -> Self {
        let mut config = Self::default();
        let dir = Path::new("/nonexistent");
        Parser {
            config: &mut config,
            current: None,
            ssh_dir: dir,
            home: dir,
        }
        .parse(text, 0);
        config
    }

    /// The aliases named without wildcards or negation, in order: the
    /// servers the config defines.
    pub(crate) fn aliases(&self) -> Vec<String> {
        let mut aliases: Vec<String> = Vec::new();
        for p in self.entries.iter().flat_map(|e| &e.patterns) {
            if !p.negated && !p.glob.contains(['*', '?']) && !aliases.contains(&p.glob) {
                aliases.push(p.glob.clone());
            }
        }
        aliases
    }

    pub(crate) fn resolve(&self, alias: &str) -> Resolved {
        let mut r = Resolved::default();
        for entry in self.entries.iter().filter(|e| e.applies_to(alias)) {
            for (keyword, value) in &entry.options {
                let first = |slot: &mut Option<String>| {
                    slot.get_or_insert_with(|| value.clone());
                };
                match keyword.as_str() {
                    "hostname" => first(&mut r.host_name),
                    "user" => first(&mut r.user),
                    "port" => first(&mut r.port),
                    "proxyjump" => first(&mut r.proxy_jump),
                    "proxycommand" => first(&mut r.proxy_command),
                    "identityfile" => r.identity_files.push(value.clone()),
                    _ => {}
                }
            }
        }
        r
    }
}

impl Entry {
    fn applies_to(&self, alias: &str) -> bool {
        let mut matched = false;
        for p in &self.patterns {
            if glob_matches(&p.glob, alias) {
                if p.negated {
                    return false;
                }
                matched = true;
            }
        }
        matched
    }
}

struct Parser<'a> {
    config: &'a mut SshConfig,
    /// The block options go to; `None` inside a `Match` block.
    current: Option<usize>,
    ssh_dir: &'a Path,
    home: &'a Path,
}

impl Parser<'_> {
    fn parse(&mut self, text: &str, depth: usize) {
        for line in text.lines() {
            let Some((keyword, args)) = split_line(line) else {
                continue;
            };
            match keyword.as_str() {
                "host" => {
                    let patterns = args
                        .into_iter()
                        .map(|a| match a.strip_prefix('!') {
                            Some(glob) => Pattern {
                                glob: glob.to_owned(),
                                negated: true,
                            },
                            None => Pattern {
                                glob: a,
                                negated: false,
                            },
                        })
                        .collect();
                    self.config.entries.push(Entry {
                        patterns,
                        options: Vec::new(),
                    });
                    self.current = Some(self.config.entries.len() - 1);
                }
                "match" => self.current = None,
                "include" if depth < MAX_INCLUDE_DEPTH => {
                    let paths: Vec<PathBuf> =
                        args.iter().flat_map(|a| self.include_paths(a)).collect();
                    for path in paths {
                        if let Ok(text) = std::fs::read_to_string(&path) {
                            self.parse(&text, depth + 1);
                        }
                    }
                }
                "include" => {}
                _ => {
                    let Some(value) = args.into_iter().next() else {
                        continue;
                    };
                    let index = match self.current {
                        Some(i) => i,
                        None if self.config.entries.is_empty() => {
                            // Options before any Host apply to every host.
                            self.config.entries.push(Entry {
                                patterns: vec![Pattern {
                                    glob: "*".into(),
                                    negated: false,
                                }],
                                options: Vec::new(),
                            });
                            self.current = Some(0);
                            0
                        }
                        None => continue,
                    };
                    self.config.entries[index].options.push((keyword, value));
                }
            }
        }
    }

    /// The files an `Include` argument names, sorted; wildcards are allowed
    /// in the file name.
    fn include_paths(&self, arg: &str) -> Vec<PathBuf> {
        let path = match arg.strip_prefix("~/") {
            Some(rest) => self.home.join(rest),
            None if Path::new(arg).is_absolute() => PathBuf::from(arg),
            None => self.ssh_dir.join(arg),
        };
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if !name.contains(['*', '?']) {
            return vec![path];
        }
        let Some(dir) = path.parent() else {
            return vec![];
        };
        let mut found: Vec<PathBuf> = std::fs::read_dir(dir)
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| glob_matches(&name, &e.file_name().to_string_lossy()))
            .map(|e| e.path())
            .filter(|p| p.is_file())
            .collect();
        found.sort();
        found
    }
}

/// A line's keyword, lower-cased, and its arguments, with quotes removed.
/// `Keyword value`, `Keyword=value` and `Keyword = value` are all accepted.
fn split_line(line: &str) -> Option<(String, Vec<String>)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let end = line
        .find(|c: char| c.is_whitespace() || c == '=')
        .unwrap_or(line.len());
    let keyword = line[..end].to_ascii_lowercase();
    let rest = line[end..].trim_start();
    let rest = rest.strip_prefix('=').unwrap_or(rest).trim_start();
    let mut args = Vec::new();
    let mut chars = rest.chars().peekable();
    while chars.peek().is_some() {
        while chars.peek().is_some_and(|c| c.is_whitespace()) {
            chars.next();
        }
        let mut arg = String::new();
        let mut quoted = false;
        while let Some(&c) = chars.peek() {
            if c == '"' {
                quoted = !quoted;
            } else if c.is_whitespace() && !quoted {
                break;
            } else {
                arg.push(c);
            }
            chars.next();
        }
        if !arg.is_empty() {
            args.push(arg);
        }
    }
    Some((keyword, args))
}

/// `*` and `?` wildcards, as in `Host` patterns. Host names compare without case.
pub(crate) fn glob_matches(glob: &str, text: &str) -> bool {
    fn go(g: &[char], t: &[char]) -> bool {
        match g.split_first() {
            None => t.is_empty(),
            Some(('*', rest)) => (0..=t.len()).any(|i| go(rest, &t[i..])),
            Some(('?', rest)) => !t.is_empty() && go(rest, &t[1..]),
            Some((c, rest)) => t
                .split_first()
                .is_some_and(|(x, t)| x.eq_ignore_ascii_case(c) && go(rest, t)),
        }
    }
    let g: Vec<char> = glob.chars().collect();
    let t: Vec<char> = text.chars().collect();
    go(&g, &t)
}

#[cfg(test)]
mod tests;

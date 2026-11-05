//! The servers of `~/.ssh/config` as drafts of SSH Hosts: their address,
//! user, port, key and `ProxyJump` bastions.

mod parse;

use std::path::{Path, PathBuf};

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    host::{Bastion, BastionAuth},
    local_files,
};

use self::parse::SshConfig;
use super::{free_name, Env, ImportDraft};
use crate::{messages as msg, HostConnection, SshAuth};
use homeport_transport as transport;

/// The Engine socket of a server described in the config.
pub const DEFAULT_SOCKET: &str = "/var/run/docker.sock";

/// How deep `ProxyJump` chains may go before Homeport calls it a loop.
const MAX_JUMPS: usize = 8;

/// A key file the config names, read.
#[derive(Debug, Clone)]
pub(crate) struct KeyFile {
    pub path: PathBuf,
    pub text: String,
    pub encrypted: bool,
}

/// One server on the way: a bastion, or the Host.
#[derive(Debug, Clone)]
struct Hop {
    host: String,
    port: u16,
    user: String,
    key: Option<KeyFile>,
}

/// What importing one alias creates.
#[derive(Debug)]
pub(crate) struct Plan {
    pub connection: HostConnection,
    /// The Host's own key; `None` uses ssh-agent.
    pub key: Option<KeyFile>,
    /// The keys of bastions using their own, by index.
    pub bastion_keys: Vec<(usize, KeyFile)>,
    pub warnings: Vec<AppError>,
}

/// The user's config: `~/.ssh/config`, read with its includes.
pub(crate) fn read(env: &Env) -> AppResult<SshConfig> {
    let ssh_dir = env.home.join(".ssh");
    let path = ssh_dir.join("config");
    SshConfig::read(&path, &ssh_dir, &env.home).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            AppError::new(
                ErrorCode::NotFound,
                msg::SSH_CONFIG_MISSING.with("path", &path),
            )
        } else {
            homeport_core::error::local_file(&path, e)
        }
    })
}

/// Every alias of `config` as a draft. `taken` holds the existing Hosts' names.
pub(crate) fn drafts(config: &SshConfig, env: &Env, taken: &[String]) -> Vec<ImportDraft> {
    config
        .aliases()
        .into_iter()
        .map(|alias| {
            let name_taken = free_name(&alias, taken) != alias;
            match plan(config, env, &alias) {
                Ok(plan) => ImportDraft {
                    files: plan
                        .key
                        .iter()
                        .chain(plan.bastion_keys.iter().map(|(_, k)| k))
                        .map(|k| k.path.display().to_string())
                        .collect(),
                    needs_passphrase: plan
                        .key
                        .iter()
                        .chain(plan.bastion_keys.iter().map(|(_, k)| k))
                        .any(|k| k.encrypted),
                    warnings: plan.warnings,
                    connection: Some(plan.connection),
                    problem: None,
                    name: alias.clone(),
                    source: alias,
                    name_taken,
                },
                Err(problem) => ImportDraft {
                    name: alias.clone(),
                    source: alias,
                    connection: None,
                    files: vec![],
                    needs_passphrase: false,
                    problem: Some(problem),
                    warnings: vec![],
                    name_taken,
                },
            }
        })
        .collect()
}

/// What importing `alias` creates, or why it can't be.
pub(crate) fn plan(config: &SshConfig, env: &Env, alias: &str) -> AppResult<Plan> {
    let mut warnings = Vec::new();
    let resolved = config.resolve(alias);
    if resolved.proxy_jump.is_none() && resolved.proxy_command.is_some() {
        warnings.push(AppError::validation(msg::IMPORT_PROXY_COMMAND));
    }
    let target = hop(config, env, alias, None, None, &mut warnings)?;
    let mut visiting = vec![alias.to_owned()];
    let chain = jumps(config, env, alias, &mut visiting, &mut warnings)?;

    let mut bastions = Vec::with_capacity(chain.len());
    let mut bastion_keys = Vec::new();
    for (i, b) in chain.into_iter().enumerate() {
        let same_key = match (&b.key, &target.key) {
            (None, None) => true,
            (Some(a), Some(t)) => a.path == t.path,
            _ => false,
        };
        let auth = match &b.key {
            _ if same_key => BastionAuth::SameAsHost,
            None => BastionAuth::Agent,
            Some(key) => {
                bastion_keys.push((i, key.clone()));
                BastionAuth::Imported
            }
        };
        bastions.push(Bastion {
            host: b.host,
            port: b.port,
            user: b.user,
            auth,
            public_key: None,
            host_key: None,
        });
    }
    let connection = HostConnection::Ssh {
        host: target.host,
        port: target.port,
        user: target.user,
        socket_path: DEFAULT_SOCKET.into(),
        auth: if target.key.is_some() {
            SshAuth::Imported
        } else {
            SshAuth::Agent
        },
        public_key: None,
        bastions,
    };
    connection.validate()?;
    Ok(Plan {
        connection,
        key: target.key,
        bastion_keys,
        warnings,
    })
}

/// The bastions on the way to `alias`, in order: the first jump's own chain,
/// then each server of its `ProxyJump`, as `ssh -J` goes through them.
fn jumps(
    config: &SshConfig,
    env: &Env,
    alias: &str,
    visiting: &mut Vec<String>,
    warnings: &mut Vec<AppError>,
) -> AppResult<Vec<Hop>> {
    let Some(proxy_jump) = config.resolve(alias).proxy_jump else {
        return Ok(vec![]);
    };
    if proxy_jump.eq_ignore_ascii_case("none") {
        return Ok(vec![]);
    }
    let specs: Vec<JumpSpec> = proxy_jump
        .split(',')
        .map(|s| JumpSpec::parse(s.trim()))
        .collect::<AppResult<_>>()?;
    let mut chain = Vec::new();
    for (i, spec) in specs.iter().enumerate() {
        if visiting.iter().any(|v| v.eq_ignore_ascii_case(&spec.host)) || visiting.len() > MAX_JUMPS
        {
            return Err(AppError::validation(
                msg::IMPORT_JUMP_LOOP.with("alias", &spec.host),
            ));
        }
        if i == 0 {
            visiting.push(spec.host.clone());
            chain.extend(jumps(config, env, &spec.host, visiting, warnings)?);
            visiting.pop();
        }
        chain.push(hop(
            config,
            env,
            &spec.host,
            spec.user.clone(),
            spec.port,
            warnings,
        )?);
    }
    Ok(chain)
}

/// `alias` as the config describes it, with what a `ProxyJump` entry overrides.
fn hop(
    config: &SshConfig,
    env: &Env,
    alias: &str,
    user: Option<String>,
    port: Option<u16>,
    warnings: &mut Vec<AppError>,
) -> AppResult<Hop> {
    let r = config.resolve(alias);
    let host = r
        .host_name
        .map(|h| h.replace("%h", alias).replace("%%", "%"))
        .unwrap_or_else(|| alias.to_owned());
    let port = match (port, &r.port) {
        (Some(p), _) => p,
        (None, Some(p)) => p.parse().ok().filter(|&p| p > 0).ok_or_else(|| {
            AppError::validation(
                msg::IMPORT_PORT_INVALID
                    .with("alias", alias)
                    .with("port", p),
            )
        })?,
        (None, None) => 22,
    };
    let user = user.or(r.user).unwrap_or_else(|| env.local_user.clone());
    let tokens = Tokens {
        env,
        alias,
        host: &host,
        user: &user,
        port,
    };
    let files: Vec<PathBuf> = r.identity_files.iter().map(|f| tokens.path(f)).collect();
    let key = files.iter().find_map(|path| {
        let text = local_files::read_text(path).ok()?;
        Some(KeyFile {
            encrypted: transport::ssh::needs_passphrase(&text),
            path: path.clone(),
            text,
        })
    });
    if key.is_none() && !files.is_empty() {
        warnings.push(AppError::validation(
            msg::IMPORT_KEY_UNREADABLE
                .with("alias", alias)
                .with("path", files[0].as_path()),
        ));
    }
    Ok(Hop {
        host,
        port,
        user,
        key,
    })
}

/// One server of a `ProxyJump` list: `[user@]host[:port]` or `ssh://…`.
#[derive(Debug, PartialEq, Eq)]
struct JumpSpec {
    user: Option<String>,
    host: String,
    port: Option<u16>,
}

impl JumpSpec {
    fn parse(spec: &str) -> AppResult<Self> {
        let invalid = || AppError::validation(msg::IMPORT_JUMP_INVALID.with("jump", spec));
        let rest = spec.strip_prefix("ssh://").unwrap_or(spec);
        let (user, rest) = match rest.rsplit_once('@') {
            Some((u, r)) => (Some(u.to_owned()), r),
            None => (None, rest),
        };
        let (host, port) = if let Some(inner) = rest.strip_prefix('[') {
            let (host, after) = inner.split_once(']').ok_or_else(invalid)?;
            (host, after.strip_prefix(':'))
        } else {
            match rest.rsplit_once(':') {
                Some((h, p)) if !h.contains(':') => (h, Some(p)),
                _ => (rest, None),
            }
        };
        let port = port
            .map(|p| p.parse::<u16>().ok().filter(|&p| p > 0).ok_or_else(invalid))
            .transpose()?;
        if host.is_empty() {
            return Err(invalid());
        }
        Ok(Self {
            user,
            host: host.to_owned(),
            port,
        })
    }
}

/// The `%` tokens and `~` of an `IdentityFile`.
struct Tokens<'a> {
    env: &'a Env,
    alias: &'a str,
    host: &'a str,
    user: &'a str,
    port: u16,
}

impl Tokens<'_> {
    fn path(&self, file: &str) -> PathBuf {
        let mut out = String::new();
        let mut chars = file.chars();
        while let Some(c) = chars.next() {
            if c != '%' {
                out.push(c);
                continue;
            }
            match chars.next() {
                Some('%') => out.push('%'),
                Some('d') => out.push_str(&self.env.home.to_string_lossy()),
                Some('h') => out.push_str(self.host),
                Some('n') => out.push_str(self.alias),
                Some('u') => out.push_str(&self.env.local_user),
                Some('r') => out.push_str(self.user),
                Some('p') => out.push_str(&self.port.to_string()),
                Some(other) => {
                    out.push('%');
                    out.push(other);
                }
                None => out.push('%'),
            }
        }
        if out == "~" {
            return self.env.home.clone();
        }
        match out.strip_prefix("~/") {
            Some(rest) => self.env.home.join(rest),
            None if Path::new(&out).is_absolute() => PathBuf::from(out),
            None => self.env.home.join(out),
        }
    }
}

#[cfg(test)]
mod tests;

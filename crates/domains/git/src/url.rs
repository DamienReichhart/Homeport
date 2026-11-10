//! Repository URLs: `git@host:owner/repo.git` and `ssh://` reach a server
//! over SSH, `https://` over HTTPS. Nothing else is accepted, so a token is
//! never sent in clear and no local path is read as a repository.

use homeport_core::error::{AppError, AppResult};

use crate::messages as msg;

/// How a repository is reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Ssh,
    Https,
}

/// A checked repository URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoUrl {
    pub transport: Transport,
    /// The server's host name, without the port.
    pub host: String,
    pub port: Option<u16>,
    /// The repository's path on the server, without a leading `/`.
    pub path: String,
}

impl RepoUrl {
    /// Checks `url`, trimmed.
    pub fn parse(url: &str) -> AppResult<Self> {
        let url = url.trim();
        let invalid = || AppError::validation(msg::URL_INVALID);
        if url.is_empty() {
            return Err(AppError::validation(msg::URL_EMPTY));
        }
        if url.chars().any(char::is_whitespace) {
            return Err(invalid());
        }
        let (transport, rest) = if let Some(rest) = url.strip_prefix("ssh://") {
            (Transport::Ssh, rest)
        } else if let Some(rest) = url.strip_prefix("https://") {
            (Transport::Https, rest)
        } else if url.contains("://") {
            return Err(AppError::validation(msg::URL_SCHEME));
        } else {
            return scp_like(url).ok_or_else(invalid);
        };
        let (authority, path) = rest.split_once('/').ok_or_else(invalid)?;
        let host_port = match authority.rsplit_once('@') {
            Some((user, host)) => {
                if transport == Transport::Https && user.contains(':') {
                    return Err(AppError::validation(msg::URL_PASSWORD));
                }
                host
            }
            None => authority,
        };
        let (host, port) = split_port(host_port).ok_or_else(invalid)?;
        let path = path.trim_end_matches('/');
        if !host_ok(host) || !user_ok(authority) || path.is_empty() {
            return Err(invalid());
        }
        Ok(Self {
            transport,
            host: host.to_owned(),
            port,
            path: path.to_owned(),
        })
    }
}

/// `[user@]host:path`, the form git takes for SSH without a scheme.
fn scp_like(url: &str) -> Option<RepoUrl> {
    let (authority, path) = url.split_once(':')?;
    // `C:\repo` or `./a:b` are paths on this machine, not servers.
    if authority.contains('/') || authority.contains('\\') || path.starts_with("//") {
        return None;
    }
    let host = authority.rsplit_once('@').map_or(authority, |(_, h)| h);
    let path = path.trim_start_matches('/').trim_end_matches('/');
    if host.len() < 2 || !host_ok(host) || !user_ok(authority) || path.is_empty() {
        return None;
    }
    Some(RepoUrl {
        transport: Transport::Ssh,
        host: host.to_owned(),
        port: None,
        path: path.to_owned(),
    })
}

/// A host name or address: nothing `ssh` could read as an option.
fn host_ok(host: &str) -> bool {
    !host.is_empty()
        && !host.starts_with('-')
        && host
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || ".-_:".contains(c))
}

/// The user before `@`, if any, can't be read as an option either.
fn user_ok(authority: &str) -> bool {
    match authority.rsplit_once('@') {
        Some((user, _)) => !user.starts_with('-') && !user.is_empty(),
        None => true,
    }
}

/// `host`, `host:port` or `[v6]:port`.
fn split_port(host_port: &str) -> Option<(&str, Option<u16>)> {
    if let Some(rest) = host_port.strip_prefix('[') {
        let (host, after) = rest.split_once(']')?;
        return match after.strip_prefix(':') {
            Some(port) => Some((host, Some(port.parse().ok().filter(|p| *p > 0)?))),
            None if after.is_empty() => Some((host, None)),
            None => None,
        };
    }
    match host_port.rsplit_once(':') {
        Some((host, "")) => Some((host, None)),
        Some((host, port)) => Some((host, Some(port.parse().ok().filter(|p| *p > 0)?))),
        None => Some((host_port, None)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use homeport_core::error::ErrorCode;

    fn ok(url: &str) -> (Transport, String, Option<u16>, String) {
        let u = RepoUrl::parse(url).unwrap_or_else(|e| panic!("{url}: {e}"));
        (u.transport, u.host, u.port, u.path)
    }

    #[test]
    fn ssh_and_https_urls_are_accepted() {
        assert_eq!(
            ok("git@github.com:owner/repo.git"),
            (
                Transport::Ssh,
                "github.com".into(),
                None,
                "owner/repo.git".into()
            )
        );
        assert_eq!(
            ok("ssh://git@gitlab.example.com:2222/group/sub/repo.git"),
            (
                Transport::Ssh,
                "gitlab.example.com".into(),
                Some(2222),
                "group/sub/repo.git".into()
            )
        );
        assert_eq!(
            ok("  https://codeberg.org/owner/repo/  "),
            (
                Transport::Https,
                "codeberg.org".into(),
                None,
                "owner/repo".into()
            )
        );
        assert_eq!(
            ok("https://oauth2@gitlab.com/owner/repo.git").1,
            "gitlab.com"
        );
        assert_eq!(ok("ssh://git@[::1]:22/r.git").1, "::1");
    }

    #[test]
    fn other_urls_are_refused() {
        for url in [
            "",
            "http://github.com/owner/repo.git",
            "file:///srv/repo.git",
            "/srv/repo.git",
            "./repo",
            "C:\\repos\\app",
            "https://github.com",
            "https://github.com/",
            "git@github.com:",
            "ssh://host:0/repo",
            "https://example.com:notaport/repo",
            "git@github.com:owner/my repo.git",
            "-oProxyCommand=touch$IFS/tmp/x:repo",
            "ssh://-oProxyCommand=x/repo",
            "-me@host:repo",
        ] {
            assert!(RepoUrl::parse(url).is_err(), "{url:?} was accepted");
        }
    }

    #[test]
    fn passwords_in_https_urls_are_refused_with_why() {
        let e = RepoUrl::parse("https://me:hunter2@github.com/o/r.git").unwrap_err();
        assert!(e.is(ErrorCode::Validation));
        assert!(!e.message.contains("hunter2"));
        assert_eq!(e.key, "errors.git.urlPassword");
    }
}

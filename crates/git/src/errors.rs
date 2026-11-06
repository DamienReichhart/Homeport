//! Telling git's failures apart from what it printed, with `LC_ALL=C`.

use homeport_core::error::{AppError, ErrorCode};

use crate::{
    messages as msg,
    store::Secret,
    url::{RepoUrl, Transport},
};

/// What a failing command was reaching, for the message.
pub(crate) struct Context<'a> {
    pub(crate) url: &'a str,
    pub(crate) secret: &'a Secret,
}

/// Lines of stderr kept for Details.
const TAIL: usize = 20;

fn tail(stderr: &str) -> String {
    let lines: Vec<&str> = stderr.lines().filter(|l| !l.trim().is_empty()).collect();
    lines[lines.len().saturating_sub(TAIL)..].join("\n")
}

/// The error for a failed `git <verb>`; `remote` when it reached a server.
pub(crate) fn failure(verb: &str, stderr: &str, remote: Option<&Context<'_>>) -> AppError {
    let detail = tail(stderr);
    let generic = || {
        AppError::new(ErrorCode::GitFailed, msg::FAILED.with("verb", verb))
            .with_hint(msg::FAILED_HINT)
            .with_detail(&detail)
    };
    let Some(remote) = remote else {
        return generic();
    };
    // A branch or tag the remote doesn't have, which `fetch` names.
    if let Some(full) = missing_ref(stderr) {
        return crate::ops::ref_missing(&full).with_detail(&detail);
    }
    let (host, path, transport) = match RepoUrl::parse(remote.url) {
        Ok(u) => (u.host, u.path, Some(u.transport)),
        Err(_) => (remote.url.to_owned(), String::new(), None),
    };
    let lower = stderr.to_ascii_lowercase();
    let has = |needles: &[&str]| needles.iter().any(|n| lower.contains(n));

    let error = if has(&[
        "remote host identification has changed",
        "host key verification failed",
    ]) {
        AppError::new(
            ErrorCode::GitHostKeyChanged,
            msg::HOST_KEY_CHANGED.with("host", &host),
        )
        .with_hint(msg::HOST_KEY_CHANGED_HINT)
    } else if has(&[
        "could not resolve host",
        "could not resolve hostname",
        "name or service not known",
        "connection refused",
        "connection timed out",
        "operation timed out",
        "network is unreachable",
        "no route to host",
        "failed to connect to",
        "could not connect to server",
        "connection reset",
        "operation too slow",
    ]) {
        AppError::new(
            ErrorCode::GitUnreachable,
            msg::UNREACHABLE.with("host", &host),
        )
        .with_hint(msg::UNREACHABLE_HINT)
    } else if has(&[
        "repository not found",
        "does not appear to be a git repository",
        "project you were looking for could not be found",
        "not found",
    ]) && !has(&["couldn't find remote ref", "permission denied"])
    {
        AppError::new(
            ErrorCode::GitRepositoryNotFound,
            msg::NOT_FOUND.with("host", &host).with("path", &path),
        )
        .with_hint(msg::NOT_FOUND_HINT)
    } else if has(&[
        "permission denied",
        "authentication failed",
        "access denied",
        "could not read username",
        "could not read password",
        "terminal prompts disabled",
        "invalid username or password",
        "the requested url returned error: 401",
        "the requested url returned error: 403",
    ]) {
        match (remote.secret, transport) {
            (Secret::SshKey { .. }, _) | (_, Some(Transport::Ssh)) => AppError::new(
                ErrorCode::GitAuthFailed,
                msg::KEY_REFUSED.with("host", &host),
            )
            .with_hint(msg::KEY_REFUSED_HINT),
            (Secret::Token { username, .. }, _) => AppError::new(
                ErrorCode::GitAuthFailed,
                msg::TOKEN_REFUSED
                    .with("host", &host)
                    .with("username", username),
            )
            .with_hint(msg::TOKEN_REFUSED_HINT),
            (Secret::None, _) => AppError::new(
                ErrorCode::GitAuthFailed,
                msg::CREDENTIALS_NEEDED.with("host", &host),
            )
            .with_hint(msg::CREDENTIALS_NEEDED_HINT),
        }
    } else {
        return generic();
    };
    error.with_detail(&detail)
}

/// The branch or tag `fetch` said the remote doesn't have, if that's why it failed.
fn missing_ref(stderr: &str) -> Option<String> {
    let lower = stderr.to_ascii_lowercase();
    let at = lower.find("couldn't find remote ref ")?;
    let rest = &stderr[at + "couldn't find remote ref ".len()..];
    Some(rest.split_whitespace().next()?.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code(stderr: &str, url: &str, secret: &Secret) -> ErrorCode {
        failure("fetch", stderr, Some(&Context { url, secret })).code
    }

    const SSH: &str = "git@github.com:owner/repo.git";
    const HTTPS: &str = "https://gitlab.com/owner/repo.git";

    #[test]
    fn what_git_says_is_sorted_into_what_the_user_can_do() {
        let key = Secret::SshKey {
            private_key: String::new(),
        };
        let token = Secret::Token {
            username: "bot".into(),
            token: "t".into(),
        };
        assert_eq!(
            code(
                "git@github.com: Permission denied (publickey).\nfatal: Could not read from remote repository.",
                SSH,
                &key
            ),
            ErrorCode::GitAuthFailed
        );
        assert_eq!(
            code(
                "ERROR: Repository not found.\nfatal: Could not read from remote repository.",
                SSH,
                &key
            ),
            ErrorCode::GitRepositoryNotFound
        );
        assert_eq!(
            code(
                "@@@@@@@@@@@\n@    WARNING: REMOTE HOST IDENTIFICATION HAS CHANGED!     @\nHost key verification failed.",
                SSH,
                &key
            ),
            ErrorCode::GitHostKeyChanged
        );
        assert_eq!(
            code(
                "ssh: Could not resolve hostname nope.invalid: Name or service not known",
                SSH,
                &key
            ),
            ErrorCode::GitUnreachable
        );
        assert_eq!(
            code("remote: HTTP Basic: Access denied\nfatal: Authentication failed for 'https://gitlab.com/owner/repo.git/'", HTTPS, &token),
            ErrorCode::GitAuthFailed
        );
        assert_eq!(
            code("fatal: unable to access 'https://gitlab.com/x/': Failed to connect to gitlab.com port 443", HTTPS, &token),
            ErrorCode::GitUnreachable
        );
        assert_eq!(
            code("fatal: something else entirely", HTTPS, &token),
            ErrorCode::GitFailed
        );
    }

    #[test]
    fn a_public_repository_that_asks_for_credentials_says_it_is_probably_private() {
        let e = failure(
            "ls-remote",
            "fatal: could not read Username for 'https://github.com': terminal prompts disabled",
            Some(&Context {
                url: "https://github.com/o/private.git",
                secret: &Secret::None,
            }),
        );
        assert_eq!(e.key, "errors.git.credentialsNeeded");
        assert!(e.message.contains("github.com"));
    }

    #[test]
    fn token_errors_name_the_user_never_the_token() {
        let e = failure(
            "fetch",
            "fatal: Authentication failed",
            Some(&Context {
                url: HTTPS,
                secret: &Secret::Token {
                    username: "bot".into(),
                    token: "hunter2".into(),
                },
            }),
        );
        assert!(e.message.contains("bot"));
        assert!(!serde_json::to_string(&e).unwrap().contains("hunter2"));
    }

    #[test]
    fn a_branch_the_remote_lacks_is_named_not_a_missing_repository() {
        let e = failure(
            "fetch",
            "fatal: couldn't find remote ref refs/tags/v9\n",
            Some(&Context {
                url: HTTPS,
                secret: &Secret::None,
            }),
        );
        assert_eq!(e.code, ErrorCode::SyncedComposeFileMissing);
        assert_eq!(e.message, "The repository has no tag named v9.");
        assert!(e.detail.as_deref().unwrap().contains("refs/tags/v9"));
    }

    #[test]
    fn missing_refs_are_named() {
        assert_eq!(
            missing_ref("fatal: couldn't find remote ref refs/heads/nope\n").as_deref(),
            Some("refs/heads/nope")
        );
        assert_eq!(missing_ref("fatal: other"), None);
    }
}

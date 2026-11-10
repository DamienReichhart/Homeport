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

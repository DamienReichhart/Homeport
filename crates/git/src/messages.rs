//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    // Repository URLs

    /// Saving a repository without a URL.
    URL_EMPTY = "errors.git.urlEmpty" => "Enter the repository's URL.";
    /// A URL that is neither SSH nor HTTPS.
    URL_INVALID = "errors.git.urlInvalid" => "Enter the repository's URL, like git@github.com:owner/repo.git or https://gitlab.com/owner/repo.git.";
    /// An http://, file:// or other URL.
    URL_SCHEME = "errors.git.urlScheme" => "Homeport reads repositories over SSH or HTTPS only. Use an ssh:// or https:// URL, or one like git@github.com:owner/repo.git.";
    /// An HTTPS URL with a password or token in it.
    URL_PASSWORD = "errors.git.urlPassword" => "Leave the password or token out of the URL, and enter it in the token field instead.";

    // Stored repositories

    /// A deploy key chosen for an HTTPS URL.
    KEY_NEEDS_SSH = "errors.git.keyNeedsSsh" => "A deploy key works with an SSH URL, like git@github.com:owner/repo.git.";
    /// A token chosen for an SSH URL.
    TOKEN_NEEDS_HTTPS = "errors.git.tokenNeedsHttps" => "A token works with an https:// URL.";
    /// An SSH URL without a deploy key.
    SSH_NEEDS_KEY = "errors.git.sshNeedsKey" => "SSH repositories need a deploy key. Choose one, or use the repository's https:// URL.";
    /// Saving a token without its user name.
    USERNAME_EMPTY = "errors.git.usernameEmpty" => "Enter the user name the token belongs to.";
    /// Saving a new token without the token.
    TOKEN_EMPTY = "errors.git.tokenEmpty" => "Enter the access token.";
    /// Adding a repository that is already stored.
    REPOSITORY_EXISTS = "errors.git.repositoryExists" => "{url} is already added.";
    REPOSITORY_EXISTS_HINT = "errors.git.repositoryExists.hint" => "Pick it from the list instead.";
    /// A repository removed while it was in use.
    GONE = "errors.git.gone" => "This repository is no longer in Homeport.";
    /// Removing a repository a Synced Stack still uses.
    IN_USE = "errors.git.inUse" => "Synced stacks still use this repository.";
    IN_USE_HINT = "errors.git.inUse.hint" => "Forget those stacks first, then remove it.";
    /// Changing the key or token of a repository that uses another kind of access.
    WRONG_AUTH = "errors.git.wrongAuth" => "This repository doesn't use that kind of access.";

    // The git CLI

    /// `git` isn't installed.
    MISSING = "errors.git.missing" => "git isn't installed on this computer. Synced stacks need it.";
    /// How to install it on Linux, followed by the command.
    INSTALL_LINUX_HINT = "errors.git.installLinux.hint" => "Install git. On Debian or Ubuntu:";
    /// How to install it on macOS, followed by the command.
    INSTALL_MACOS_HINT = "errors.git.installMacos.hint" => "Install Apple's command line tools, which include git:";
    /// How to install it on Windows, followed by the command.
    INSTALL_WINDOWS_HINT = "errors.git.installWindows.hint" => "Install Git for Windows:";
    /// `git` was found but couldn't be started.
    SPAWN_FAILED = "errors.git.spawnFailed" => "Homeport couldn't start git.";
    SPAWN_FAILED_HINT = "errors.git.spawnFailed.hint" => "Check that git runs from a terminal, then try again.";
    /// The private SSH agent for one command couldn't open.
    AGENT_FAILED = "errors.git.agentFailed" => "Homeport couldn't open the private connection git uses for the deploy key.";

    // What the repository's server said

    /// The server refused the deploy key.
    KEY_REFUSED = "errors.git.keyRefused" => "{host} refused Homeport's deploy key for this repository.";
    KEY_REFUSED_HINT = "errors.git.keyRefused.hint" => "Add the deploy key to the repository with read access, then test again.";
    /// The server refused the token.
    TOKEN_REFUSED = "errors.git.tokenRefused" => "{host} refused the token of {username}.";
    TOKEN_REFUSED_HINT = "errors.git.tokenRefused.hint" => "Check that the token hasn't expired and may read this repository's contents, then replace it.";
    /// A repository added without credentials that asks for some.
    CREDENTIALS_NEEDED = "errors.git.credentialsNeeded" => "{host} asks for credentials to read this repository.";
    CREDENTIALS_NEEDED_HINT = "errors.git.credentialsNeeded.hint" => "It's probably private. Add it again with a deploy key or a token.";
    /// The repository doesn't exist, or the credentials can't see it.
    NOT_FOUND = "errors.git.notFound" => "{host} has no repository at {path}, or doesn't let Homeport see it.";
    NOT_FOUND_HINT = "errors.git.notFound.hint" => "Check the URL, and that the deploy key or token has access to the repository.";
    /// The server can't be reached.
    UNREACHABLE = "errors.git.unreachable" => "Homeport can't reach {host}.";
    UNREACHABLE_HINT = "errors.git.unreachable.hint" => "Check the URL, and this computer's internet access, DNS and proxy settings.";
    /// The SSH server's key isn't the one trusted for it.
    HOST_KEY_CHANGED = "errors.git.hostKeyChanged" => "{host} presented a different SSH key than the one Homeport trusts for it.";
    HOST_KEY_CHANGED_HINT = "errors.git.hostKeyChanged.hint" => "If the forge changed its key, check its new fingerprint on its website, then reset the trusted key in Settings → Repositories. Otherwise, someone may be intercepting the connection.";
    /// A branch or tag that isn't in the repository.
    REF_MISSING_BRANCH = "errors.git.refMissingBranch" => "The repository has no branch named {name}.";
    REF_MISSING_TAG = "errors.git.refMissingTag" => "The repository has no tag named {name}.";
    REF_MISSING_HINT = "errors.git.refMissing.hint" => "Pick one of the repository's branches or tags.";
    /// `git` failed in a way Homeport has no better words for.
    FAILED = "errors.git.failed" => "git {verb} failed.";
    FAILED_HINT = "errors.git.failed.hint" => "What git said is in Details.";
}

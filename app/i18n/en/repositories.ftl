### The `repositories` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/repositories.ts:
### change the text there, then run it again.

repositories-title = Repositories
repositories-description = Git repositories Synced Stacks are deployed from, on every Host. Homeport only reads them.
repositories-add = Add repository
repositories-empty = No repositories yet. Add one to create a Synced Stack.
repositories-manageOne = Manage access to { $repository }
repositories-removeOne = Remove { $repository }
repositories-remove-title = Remove { $repository }?
repositories-remove-description = Homeport forgets its deploy key or token and deletes its clone on this computer. The repository itself is left alone. Remove the deploy key from it on your forge too.
repositories-noKeychain = Deploy keys and tokens are encrypted with a key kept in the OS keychain, and none is available, so only public repositories can be added.
repositories-public = A public repository: Homeport reads it without credentials.
repositories-added = Added { $repository }
repositories-removed = Removed { $repository }
repositories-keyRotated = Made a new deploy key. Add it to the repository in place of the old one.
repositories-tokenReplaced = Replaced the token
repositories-hostKeyReset = Homeport will trust the key the server presents next time.
repositories-auth-withKey = With a deploy key, on { $host }
repositories-auth-withToken = As { $username } with a token, on { $host }
repositories-auth-public = Public, on { $host }
repositories-dialog-addTitle = Add a repository
repositories-dialog-description = Homeport clones it on this computer to deploy Synced Stacks from it. It never commits or pushes.
repositories-dialog-url = Repository URL
repositories-dialog-auth = Access
repositories-dialog-authKey = Deploy key
repositories-dialog-authToken = Token
repositories-dialog-authNone = Public
repositories-dialog-authKeyHelp = Homeport makes a key for this repository only. You add it to the repository with read access; your own SSH keys are never used.
repositories-dialog-authTokenHelp = An access token that may read the repository’s contents, sent over HTTPS. Encrypted on this computer and never shown again.
repositories-dialog-authNoneHelp = For a public repository over HTTPS. Nothing is stored.
repositories-dialog-username = User name
repositories-dialog-token = Access token
repositories-dialog-add = Add
repositories-dialog-addWithKey = Add and make a deploy key
repositories-missing-url = Enter the repository’s URL.
repositories-missing-validUrl = Enter an SSH or HTTPS URL, like git@github.com:owner/repo.git or https://gitlab.com/owner/repo.git.
repositories-missing-auth = Choose how Homeport reaches this repository.
repositories-missing-username = Enter the user name the token belongs to.
repositories-missing-token = Enter the access token.
repositories-deployKey-title = Deploy key
repositories-deployKey-github = GitHub: the repository’s <strong>Settings → Deploy keys → Add deploy key</strong>.
repositories-deployKey-gitlab = GitLab: the project’s <strong>Settings → Repository → Deploy keys</strong>.
repositories-deployKey-gitea = Gitea and Forgejo: the repository’s <strong>Settings → Deploy keys → Add deploy key</strong>.
repositories-deployKey-readOnly = Leave write access off: Homeport only reads. Then test access below.
repositories-deployKey-fingerprint = Fingerprint: <mono>{ $fingerprint }</mono>
repositories-rotate-button = New key
repositories-rotate-title = Make a new deploy key?
repositories-rotate-description = The current key stops working for Homeport. Add the new one to the repository, and remove the old one there.
repositories-rotate-confirm = Make a new key
repositories-token-replace = Replace the token
repositories-token-save = Save token
repositories-hostKey-trusted = Trusted key of { $host }: <mono>{ $fingerprint }</mono>
repositories-hostKey-reset = Reset
repositories-hostKey-resetTitle = Reset the trusted key of { $host }?
repositories-hostKey-resetDescription = Do this only if { $host } changed its SSH key and you checked the new fingerprint on its website. Homeport will trust whatever key it presents next.
repositories-test-button = Test access
repositories-test-testing = Testing…
repositories-test-success = Homeport can read it: { $branches ->
        [one] { NUM($branches) } branch
       *[other] { NUM($branches) } branches
    } and { $tags ->
        [one] { NUM($tags) } tag
       *[other] { NUM($tags) } tags
    }.

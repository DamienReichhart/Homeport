//! Git repositories for Synced Stacks, kept for every Host
//! (Settings → Repositories, ADR 0014).

use homeport_core::error::AppResult;
use homeport_git::{cli, RemoteRefs, Repository, RepositoryDraft};

use crate::app::AppState;

/// Every stored repository, never with its key or token.
pub fn repositories_list(state: &AppState) -> AppResult<Vec<Repository>> {
    state.repositories.store.list()
}

/// Adds a repository, generating its deploy key when it uses one.
pub fn repositories_create(state: &AppState, draft: RepositoryDraft) -> AppResult<Repository> {
    state.repositories.store.create(&draft)
}

/// Replaces a repository's deploy key with a new one.
pub fn repositories_rotate_key(state: &AppState, id: String) -> AppResult<Repository> {
    state.repositories.store.rotate_key(&id)
}

/// Replaces a repository's user name and token.
pub fn repositories_set_token(
    state: &AppState,
    id: String,
    username: String,
    token: String,
) -> AppResult<Repository> {
    state.repositories.store.set_token(&id, &username, &token)
}

/// Forgets the SSH server key trusted for a repository, so the next
/// command trusts the one the server presents.
pub fn repositories_reset_host_key(state: &AppState, id: String) -> AppResult<Repository> {
    state.repositories.store.reset_host_key(&id)
}

/// Removes a repository and its clone; refused while a Synced Stack uses it.
pub fn repositories_remove(state: &AppState, id: String) -> AppResult<()> {
    state.repositories.delete(&id)
}

/// "Test access": the repository's branches and tags, as this computer sees them.
pub async fn repositories_test(state: &AppState, id: String) -> AppResult<RemoteRefs> {
    let git = cli::detect().await?;
    state.repositories.test(&git, &id).await
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;
    use homeport_git::{RepoAuth, RepositoryDraft};

    use super::super::fixtures;
    use crate::app::testing;

    fn draft(url: &str, auth: RepoAuth) -> RepositoryDraft {
        let token = (auth == RepoAuth::HttpsToken).then(|| "secret".to_owned());
        RepositoryDraft {
            url: url.into(),
            auth,
            username: token.as_ref().map(|_| "me".to_owned()),
            token,
        }
    }

    #[tokio::test]
    async fn repositories_are_added_listed_and_removed() {
        let app = testing::app();
        assert!(super::repositories_list(&app.state).unwrap().is_empty());
        let added = super::repositories_create(&app.state, fixtures::repository_draft()).unwrap();
        let listed = super::repositories_list(&app.state).unwrap();
        assert_eq!(listed, std::slice::from_ref(&added));
        super::repositories_remove(&app.state, added.id).unwrap();
        assert!(super::repositories_list(&app.state).unwrap().is_empty());
    }

    #[tokio::test]
    async fn a_repository_a_synced_stack_uses_stays() {
        let app = testing::app();
        let stack = fixtures::synced(&app.state, &fixtures::host(&app.state));
        assert!(super::repositories_remove(&app.state, stack.repository_id).is_err());
        assert_eq!(super::repositories_list(&app.state).unwrap().len(), 1);
    }

    #[tokio::test]
    async fn a_deploy_key_is_rotated_and_a_token_replaced() {
        let app = testing::app();
        let ssh = draft("git@example.com:acme/stacks.git", RepoAuth::SshKey);
        let ssh = super::repositories_create(&app.state, ssh).unwrap();
        let rotated = super::repositories_rotate_key(&app.state, ssh.id.clone()).unwrap();
        assert!(rotated.public_key.is_some());
        assert_ne!(rotated.public_key, ssh.public_key);
        let reset = super::repositories_reset_host_key(&app.state, ssh.id.clone()).unwrap();
        assert_eq!(reset.host_key_fingerprint, None);
        // A deploy key's repository has no token to replace.
        let e =
            super::repositories_set_token(&app.state, ssh.id, "me".into(), "t".into()).unwrap_err();
        assert_eq!(e.code, ErrorCode::Validation);

        let https = draft("https://example.com/acme/other.git", RepoAuth::HttpsToken);
        let https = super::repositories_create(&app.state, https).unwrap();
        let set =
            super::repositories_set_token(&app.state, https.id.clone(), "you".into(), "t".into())
                .unwrap();
        assert_eq!(set.username.as_deref(), Some("you"));
        assert!(set.has_token);
        let e = super::repositories_rotate_key(&app.state, https.id).unwrap_err();
        assert_eq!(e.code, ErrorCode::Validation);
    }

    #[tokio::test]
    async fn testing_access_needs_a_stored_repository() {
        let app = testing::app();
        assert!(super::repositories_test(&app.state, "gone".into())
            .await
            .is_err());
        let e = super::repositories_reset_host_key(&app.state, "gone".into()).unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
    }
}

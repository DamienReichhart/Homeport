//! Repositories and their clones on this machine: one bare clone each, in
//! the app data folder, shared by the Synced Stacks made from it.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use homeport_core::error::AppResult;

use crate::{
    access::Remote,
    cli::GitCli,
    ops::{self, RefKind, RemoteRefs},
    store::{Access, RepositoryStore},
};

pub struct Repositories {
    pub store: RepositoryStore,
    /// Where the clones live: `<root>/<id>.git`.
    root: PathBuf,
    /// One fetch at a time per clone, so two never race for its refs.
    locks: Mutex<HashMap<String, Arc<tokio::sync::Mutex<()>>>>,
}

impl Repositories {
    pub fn new(store: RepositoryStore, root: impl Into<PathBuf>) -> Self {
        Self {
            store,
            root: root.into(),
            locks: Mutex::default(),
        }
    }

    /// The bare clone of repository `id` on this machine.
    pub fn clone_dir(&self, id: &str) -> PathBuf {
        self.root.join(format!("{id}.git"))
    }

    /// "Test access": lists the repository's branches and tags, trusting its
    /// SSH server's key on first use and refusing a changed one.
    pub async fn test(&self, cli: &GitCli, id: &str) -> AppResult<RemoteRefs> {
        let access = self.store.access(id)?;
        let (refs, learned) = ops::list_remote(cli, &remote(&access)).await?;
        self.learn(id, learned)?;
        Ok(refs)
    }

    /// Fetches `refs` into repository `id`'s clone, making it on first use.
    /// Returns where the clone is.
    pub async fn fetch(
        &self,
        cli: &GitCli,
        id: &str,
        refs: &[(RefKind, &str)],
    ) -> AppResult<PathBuf> {
        let lock = self
            .locks
            .lock()
            .expect("repository locks poisoned")
            .entry(id.to_owned())
            .or_default()
            .clone();
        let _fetching = lock.lock().await;
        let access = self.store.access(id)?;
        let dir = self.clone_dir(id);
        let learned = ops::fetch(cli, &dir, &remote(&access), refs).await?;
        self.learn(id, learned)?;
        Ok(dir)
    }

    /// Removes repository `id`'s clone, if it has one.
    pub fn remove_clone(&self, id: &str) {
        remove_dir(&self.clone_dir(id));
    }

    /// Removes the repository and its clone; refused while a Synced Stack uses it.
    pub fn delete(&self, id: &str) -> AppResult<()> {
        self.store.delete(id)?;
        self.remove_clone(id);
        Ok(())
    }

    fn learn(&self, id: &str, learned: Option<String>) -> AppResult<()> {
        match learned {
            Some(line) => self.store.set_host_key(id, &line),
            None => Ok(()),
        }
    }
}

fn remote(access: &Access) -> Remote<'_> {
    Remote {
        url: &access.repository.url,
        secret: &access.secret,
        host_key: access.host_key.as_deref(),
    }
}

fn remove_dir(dir: &Path) {
    if dir.exists() {
        if let Err(e) = std::fs::remove_dir_all(dir) {
            log::warn!("couldn't remove {}: {e}", dir.display());
        }
    }
}

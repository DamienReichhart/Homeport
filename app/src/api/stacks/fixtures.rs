//! What the Stacks' tests start from: a stored Host nothing connects to, a
//! repository and Stacks of each kind on them.

use homeport_core::host::{EngineKind, HostConnection};
use homeport_git::{RefKind, RepoAuth, RepositoryDraft};
use homeport_hosts::store::HostDraft;
use homeport_stacks::{
    linked::{files::files_fingerprint, LinkedStack},
    store::{ComposeFile, ManagedStack},
    synced::{SyncedDraft, SyncedStack},
    StackOwner,
};

use crate::app::{testing::TestApp, AppState};

pub const COMPOSE: &str = "services:\n  web:\n    image: nginx\n";

/// A Host that's stored but was never started, so it has no Engine.
pub fn host(state: &AppState) -> StackOwner {
    let host = state
        .hosts
        .store
        .create(HostDraft {
            name: "Test".into(),
            connection: HostConnection::Local {
                socket_path: "/nonexistent/docker.sock".into(),
            },
            engine: EngineKind::Docker,
            secrets: None,
        })
        .unwrap();
    StackOwner::Host(host.id)
}

pub fn files() -> Vec<ComposeFile> {
    vec![ComposeFile::new("compose.yaml", COMPOSE)]
}

pub fn managed(state: &AppState, owner: &StackOwner) -> ManagedStack {
    state
        .stacks
        .managed
        .create(owner, "web", &files(), "KEY=value")
        .unwrap()
}

/// A Linked Stack on a folder of the test app holding one compose file.
pub fn linked(app: &TestApp, owner: &StackOwner) -> LinkedStack {
    let folder = app.dir.path().join("project");
    std::fs::create_dir_all(&folder).unwrap();
    let file = folder.join("compose.yaml");
    std::fs::write(&file, COMPOSE).unwrap();
    // Compose files are kept by their full path.
    let files = [file.to_str().unwrap().to_owned()];
    app.state
        .stacks
        .linked
        .create(
            owner.clone(),
            "project",
            folder.to_str().unwrap(),
            &files,
            &files_fingerprint(&files),
        )
        .unwrap()
}

pub fn repository_draft() -> RepositoryDraft {
    RepositoryDraft {
        url: "https://example.com/acme/stacks.git".into(),
        auth: RepoAuth::None,
        username: None,
        token: None,
    }
}

/// A Synced Stack recorded as fetched, whose repository was never cloned.
pub fn synced(state: &AppState, owner: &StackOwner) -> SyncedStack {
    let repository = state
        .repositories
        .store
        .create(&repository_draft())
        .unwrap();
    let draft = SyncedDraft {
        name: "synced".into(),
        repository_id: repository.id,
        ref_kind: RefKind::Branch,
        ref_name: "main".into(),
        compose_path: "compose.yaml".into(),
    };
    state
        .stacks
        .synced
        .store
        .create(owner.clone(), &draft, "KEY=value", "0123abc")
        .unwrap()
}

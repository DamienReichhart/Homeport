//! Stacks in a backup of Homeport's data: every kind, as domain JSON rather
//! than table rows, so a backup from an older Homeport restores into a newer
//! one. Environments are opened in the backup and sealed again with this
//! computer's key on restoring. Deploy history isn't kept.

use std::fmt;

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_git::ops::RefKind;
use homeport_storage::{db::sqlite_error, SecretBox};

use crate::{
    linked::store::LinkedStore,
    model::owner::StackOwner,
    store::{ComposeFile, StackStore},
    synced::store::SyncedStore,
};

/// The Stacks section of a backup.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct StacksBackup {
    pub managed: Vec<ManagedBackup>,
    pub linked: Vec<LinkedBackup>,
    pub synced: Vec<SyncedBackup>,
    /// The Stacks of Swarms (ADR 0017), apart so that a Homeport from
    /// before 0.14 restores the rest and skips them. Their `host_id` is empty.
    pub swarms: SwarmStacksBackup,
}

/// The Stacks deployed to Swarms, each with its `swarm_id`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SwarmStacksBackup {
    pub managed: Vec<ManagedBackup>,
    pub linked: Vec<LinkedBackup>,
    pub synced: Vec<SyncedBackup>,
}

impl StacksBackup {
    /// How many Stacks it holds.
    pub fn count(&self) -> usize {
        self.managed.len()
            + self.linked.len()
            + self.synced.len()
            + self.swarms.managed.len()
            + self.swarms.linked.len()
            + self.swarms.synced.len()
    }
}

/// The owner of a backed-up Stack: its Swarm when it has one, else its Host.
fn owner_of(host_id: &str, swarm_id: &Option<String>) -> StackOwner {
    match swarm_id {
        Some(swarm) => StackOwner::Swarm(swarm.clone()),
        None => StackOwner::Host(host_id.to_owned()),
    }
}

fn split(owner: StackOwner) -> (String, Option<String>) {
    match owner {
        StackOwner::Host(host) => (host, None),
        StackOwner::Swarm(swarm) => (String::new(), Some(swarm)),
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedBackup {
    pub id: String,
    #[serde(default)]
    pub host_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swarm_id: Option<String>,
    #[serde(default)]
    pub registry_auth: bool,
    pub name: String,
    pub files: Vec<ComposeFile>,
    /// Its `.env`, opened; empty when it has none or it couldn't be opened.
    #[serde(default)]
    pub env: String,
    #[serde(default)]
    pub profiles: Vec<String>,
}

/// A Linked Stack: where its folder and files are on the computer it was
/// backed up on. Restored elsewhere, it shows as External until they exist.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedBackup {
    pub id: String,
    #[serde(default)]
    pub host_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swarm_id: Option<String>,
    #[serde(default)]
    pub registry_auth: bool,
    pub name: String,
    pub working_dir: String,
    pub config_files: Vec<String>,
    #[serde(default)]
    pub fingerprint: String,
    #[serde(default)]
    pub profiles: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncedBackup {
    pub id: String,
    #[serde(default)]
    pub host_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swarm_id: Option<String>,
    #[serde(default)]
    pub registry_auth: bool,
    pub name: String,
    pub repository_id: String,
    pub ref_kind: RefKind,
    pub ref_name: String,
    pub compose_path: String,
    #[serde(default)]
    pub env: String,
    #[serde(default)]
    pub profiles: Vec<String>,
    #[serde(default)]
    pub deployed_commit: Option<String>,
    #[serde(default)]
    pub fetched_commit: Option<String>,
    #[serde(default = "yes")]
    pub poll: bool,
    #[serde(default)]
    pub auto_deploy: bool,
}

fn yes() -> bool {
    true
}

impl fmt::Debug for ManagedBackup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ManagedBackup")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for SyncedBackup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SyncedBackup")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

/// The Stacks' stores, read for a backup.
pub struct Stores<'a> {
    pub managed: &'a StackStore,
    pub linked: &'a LinkedStore,
    pub synced: &'a SyncedStore,
}

/// Every Stack of the Hosts `host_ids`. A Stack whose environment can't be
/// opened is kept without it, and named in `lost`.
pub fn backup(
    stores: &Stores<'_>,
    host_ids: &[String],
    lost: &mut Vec<String>,
) -> AppResult<StacksBackup> {
    let mut section = StacksBackup::default();
    let mut env = |name: &str, opened: AppResult<String>| match opened {
        Ok(env) => Ok(env),
        Err(e) if e.is(ErrorCode::CredentialsLost) || e.is(ErrorCode::KeychainUnavailable) => {
            lost.push(name.to_owned());
            Ok(String::new())
        }
        Err(e) => Err(e),
    };
    let managed = |stack: crate::store::ManagedStack, env: String| {
        let (host_id, swarm_id) = split(stack.owner);
        ManagedBackup {
            env,
            id: stack.id,
            host_id,
            swarm_id,
            registry_auth: stack.registry_auth,
            name: stack.name,
            files: stack.files,
            profiles: stack.profiles,
        }
    };
    let linked = |stack: crate::linked::LinkedStack| {
        let (host_id, swarm_id) = split(stack.owner);
        LinkedBackup {
            id: stack.id,
            host_id,
            swarm_id,
            registry_auth: stack.registry_auth,
            name: stack.name,
            working_dir: stack.working_dir,
            config_files: stack.config_files,
            fingerprint: stack.fingerprint,
            profiles: stack.profiles,
        }
    };
    let synced = |stack: crate::synced::SyncedStack, env: String| {
        let (host_id, swarm_id) = split(stack.owner);
        SyncedBackup {
            env,
            id: stack.id,
            host_id,
            swarm_id,
            registry_auth: stack.registry_auth,
            name: stack.name,
            repository_id: stack.repository_id,
            ref_kind: stack.ref_kind,
            ref_name: stack.ref_name,
            compose_path: stack.compose_path,
            profiles: stack.profiles,
            deployed_commit: stack.deployed_commit,
            fetched_commit: stack.fetched_commit,
            poll: stack.poll,
            auto_deploy: stack.auto_deploy,
        }
    };
    for host_id in host_ids {
        for stack in stores.managed.list(host_id)? {
            let opened = env(&stack.name, stores.managed.env(&stack.id))?;
            section.managed.push(managed(stack, opened));
        }
        for stack in stores.linked.list(host_id)? {
            section.linked.push(linked(stack));
        }
        for stack in stores.synced.list(host_id)? {
            let opened = env(&stack.name, stores.synced.env(&stack.id))?;
            section.synced.push(synced(stack, opened));
        }
    }
    for stack in stores.managed.list_on_swarms()? {
        let opened = env(&stack.name, stores.managed.env(&stack.id))?;
        section.swarms.managed.push(managed(stack, opened));
    }
    for stack in stores.linked.list_on_swarms()? {
        section.swarms.linked.push(linked(stack));
    }
    for stack in stores.synced.list_all()? {
        if stack.owner.is_swarm() {
            let opened = env(&stack.name, stores.synced.env(&stack.id))?;
            section.swarms.synced.push(synced(stack, opened));
        }
    }
    Ok(section)
}

/// Writes a backup's Stacks in `tx`, into a database emptied of them and
/// holding their Hosts and repositories, with their environments sealed
/// under `secrets`.
pub fn restore(
    tx: &rusqlite::Transaction<'_>,
    secrets: &SecretBox,
    section: &StacksBackup,
) -> AppResult<()> {
    let seal = |env: &str| -> AppResult<Option<Vec<u8>>> {
        if env.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(secrets.seal(env.as_bytes())?))
        }
    };
    let sql = |e: rusqlite::Error| sqlite_error(e);
    let managed = section.managed.iter().chain(&section.swarms.managed);
    let linked = section.linked.iter().chain(&section.swarms.linked);
    let synced = section.synced.iter().chain(&section.swarms.synced);
    for stack in managed {
        let owner = owner_of(&stack.host_id, &stack.swarm_id);
        if stack.files.is_empty() {
            return Err(AppError::validation(crate::messages::NO_FILES));
        }
        tx.execute(
            "INSERT INTO stacks (id, host_id, swarm_id, name, env, profiles, registry_auth)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &stack.id,
                owner.host_id(),
                owner.swarm_id(),
                &stack.name,
                seal(&stack.env)?,
                serde_json::to_string(&stack.profiles)?,
                stack.registry_auth,
            ),
        )
        .map_err(sql)?;
        for (position, file) in stack.files.iter().enumerate() {
            tx.execute(
                "INSERT INTO stack_files (stack_id, position, name, content) VALUES (?1, ?2, ?3, ?4)",
                (&stack.id, position as i64, &file.name, &file.content),
            )
            .map_err(sql)?;
        }
    }
    for stack in linked {
        let owner = owner_of(&stack.host_id, &stack.swarm_id);
        tx.execute(
            "INSERT INTO linked_stacks
                 (id, host_id, swarm_id, registry_auth, name, working_dir, config_files,
                  fingerprint, profiles)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (
                &stack.id,
                owner.host_id(),
                owner.swarm_id(),
                stack.registry_auth,
                &stack.name,
                &stack.working_dir,
                serde_json::to_string(&stack.config_files)?,
                &stack.fingerprint,
                serde_json::to_string(&stack.profiles)?,
            ),
        )
        .map_err(sql)?;
    }
    for stack in synced {
        let owner = owner_of(&stack.host_id, &stack.swarm_id);
        tx.execute(
            "INSERT INTO synced_stacks
                 (id, host_id, swarm_id, registry_auth, name, repository_id, ref_kind, ref_name,
                  compose_path, env, profiles, deployed_commit, fetched_commit, poll, auto_deploy)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            (
                &stack.id,
                owner.host_id(),
                owner.swarm_id(),
                stack.registry_auth,
                &stack.name,
                &stack.repository_id,
                stack.ref_kind.text(),
                &stack.ref_name,
                &stack.compose_path,
                seal(&stack.env)?,
                serde_json::to_string(&stack.profiles)?,
                &stack.deployed_commit,
                &stack.fetched_commit,
                stack.poll,
                stack.auto_deploy,
            ),
        )
        .map_err(sql)?;
    }
    Ok(())
}

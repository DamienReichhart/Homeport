//! What each deploy of a Managed or Linked Stack used, kept so a deploy can
//! be looked back on, compared and rolled back to. Environments are sealed
//! like the Stack's own (ADR 0002) and only opened when one is rolled back.

use std::sync::Arc;

use rusqlite::{OptionalExtension, Row};
use serde::Serialize;

use homeport_core::error::{AppError, AppResult};
use homeport_storage::{Db, SecretBox};

use crate::{
    compose::ComposeAction,
    messages as msg,
    model::{definition, owner::StackOwner},
    runtime::SourceKind,
    store::{ComposeFile, ManagedStack, StackStore},
};

/// Deploys kept per Stack; older ones are dropped as new ones are recorded.
pub const KEPT: usize = 20;

/// One deploy, without its files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DeploySummary {
    pub id: String,
    pub stack_id: String,
    pub kind: SourceKind,
    pub action: ComposeAction,
    pub started_at: String,
    /// `None` while it runs, or when the run was cancelled.
    pub finished_at: Option<String>,
    pub success: Option<bool>,
    pub exit_code: Option<i32>,
    pub has_env: bool,
    /// The commit a Synced Stack deployed.
    pub commit: Option<String>,
    /// Started by a Synced Stack's auto-deploy, not by the user.
    pub automatic: bool,
}

/// One deploy with the definition it used.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Deploy {
    pub summary: DeploySummary,
    /// The compose files, in merge order: names for a Managed Stack, paths
    /// for a Linked one.
    pub files: Vec<ComposeFile>,
    pub profiles: Vec<String>,
}

/// A deploy as the history shows it: its definition, and the names of its
/// environment variables, never their values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DeployDetail {
    pub deploy: Deploy,
    /// `None` when it has an environment this computer's keychain can't open.
    pub env_keys: Option<Vec<String>>,
}

/// What a deploy is recorded with when it starts.
pub struct NewDeploy<'a> {
    /// The Host or Swarm the Stack is deployed to.
    pub owner: &'a StackOwner,
    pub stack_id: &'a str,
    pub kind: SourceKind,
    pub name: &'a str,
    pub action: ComposeAction,
    /// The compose files as the run uses them, by name, in order.
    pub files: &'a [(String, String)],
    /// The environment text, empty for none.
    pub env: &'a str,
    pub profiles: &'a [String],
    /// The commit of a Synced Stack.
    pub commit: Option<&'a str>,
    /// Started by auto-deploy.
    pub automatic: bool,
}

pub struct DeployHistory {
    db: Arc<Db>,
    secrets: Arc<SecretBox>,
}

const COLUMNS: &str = "id, stack_id, stack_kind, action, started_at, finished_at, success, \
     exit_code, env IS NOT NULL, definition, profiles, commit_hash, automatic";

impl DeployHistory {
    pub fn new(db: Arc<Db>, secrets: Arc<SecretBox>) -> Self {
        Self { db, secrets }
    }

    /// Records a deploy that's starting, and drops the Stack's oldest ones
    /// beyond [`KEPT`] in the same transaction. Returns its id.
    pub fn start(&self, deploy: &NewDeploy<'_>) -> AppResult<String> {
        let env = if deploy.env.trim().is_empty() {
            None
        } else {
            Some(self.secrets.seal(deploy.env.as_bytes())?)
        };
        let files: Vec<ComposeFile> = deploy
            .files
            .iter()
            .map(|(name, content)| ComposeFile::new(name, content))
            .collect();
        let definition = serde_json::to_string(&files)?;
        let profiles = serde_json::to_string(deploy.profiles)?;
        let id = uuid::Uuid::new_v4().to_string();
        self.db.with(|c| {
            let tx = c.transaction()?;
            tx.execute(
                "INSERT INTO stack_deploys
                 (id, host_id, swarm_id, stack_id, stack_kind, stack_name, action, definition, env,
                  profiles, commit_hash, automatic)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                (
                    &id,
                    deploy.owner.host_id(),
                    deploy.owner.swarm_id(),
                    deploy.stack_id,
                    kind_text(deploy.kind),
                    deploy.name,
                    action_text(deploy.action),
                    &definition,
                    &env,
                    &profiles,
                    deploy.commit,
                    deploy.automatic,
                ),
            )?;
            tx.execute(
                "DELETE FROM stack_deploys WHERE stack_id = ?1 AND id NOT IN (
                     SELECT id FROM stack_deploys WHERE stack_id = ?1
                     ORDER BY started_at DESC, rowid DESC LIMIT ?2)",
                (deploy.stack_id, KEPT as i64),
            )?;
            tx.commit()
        })?;
        Ok(id)
    }

    /// Notes how a recorded deploy ended.
    pub fn finish(&self, id: &str, success: bool, exit_code: Option<i32>) -> AppResult<()> {
        self.db.with(|c| {
            c.execute(
                "UPDATE stack_deploys SET finished_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                 success = ?2, exit_code = ?3 WHERE id = ?1",
                (id, success, exit_code),
            )
        })?;
        Ok(())
    }

    /// A Stack's deploys, newest first. Environments stay sealed, so this
    /// works without a keychain.
    pub fn list(&self, stack_id: &str) -> AppResult<Vec<DeploySummary>> {
        self.db.with(|c| {
            c.prepare(&format!(
                "SELECT {COLUMNS} FROM stack_deploys WHERE stack_id = ?1
                 ORDER BY started_at DESC, rowid DESC"
            ))?
            .query_map([stack_id], |r| read_row(r).map(|d| d.summary))?
            .collect()
        })
    }

    pub fn get(&self, id: &str) -> AppResult<Deploy> {
        self.db
            .with(|c| {
                c.query_row(
                    &format!("SELECT {COLUMNS} FROM stack_deploys WHERE id = ?1"),
                    [id],
                    read_row,
                )
                .optional()
            })?
            .ok_or_else(|| AppError::not_found(msg::DEPLOY_GONE))
    }

    /// The Stack's latest deploy that succeeded: what is deployed now, as far
    /// as Homeport knows.
    pub fn latest_successful(&self, stack_id: &str) -> AppResult<Option<Deploy>> {
        self.db.with(|c| {
            c.query_row(
                &format!(
                    "SELECT {COLUMNS} FROM stack_deploys WHERE stack_id = ?1 AND success = 1
                     ORDER BY started_at DESC, rowid DESC LIMIT 1"
                ),
                [stack_id],
                read_row,
            )
            .optional()
        })
    }

    /// The decrypted environment of a recorded deploy, empty when it had none.
    pub fn env(&self, id: &str) -> AppResult<String> {
        let sealed: Option<Vec<u8>> = self
            .db
            .with(|c| {
                c.query_row("SELECT env FROM stack_deploys WHERE id = ?1", [id], |r| {
                    r.get(0)
                })
                .optional()
            })?
            .ok_or_else(|| AppError::not_found(msg::DEPLOY_GONE))?;
        match sealed {
            Some(sealed) => Ok(String::from_utf8_lossy(&self.secrets.open(&sealed)?).into_owned()),
            None => Ok(String::new()),
        }
    }

    /// The names of a recorded deploy's environment variables; `None` when
    /// it has an environment the keychain can't open.
    pub fn env_keys(&self, id: &str) -> AppResult<Option<Vec<String>>> {
        match self.env(id) {
            Ok(env) => Ok(Some(
                definition::parse_env(&env)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(k, _)| k)
                    .collect(),
            )),
            Err(e) if e.is(homeport_core::error::ErrorCode::NotFound) => Err(e),
            Err(_) => Ok(None),
        }
    }
}

/// Puts back the compose files, environment and profiles a Managed Stack
/// was deployed with in `deploy_id`, ready to redeploy. A Linked Stack's
/// folder is its source, so it isn't rolled back from Homeport.
pub fn restore(
    history: &DeployHistory,
    stacks: &StackStore,
    deploy_id: &str,
) -> AppResult<ManagedStack> {
    let deploy = history.get(deploy_id)?;
    match deploy.summary.kind {
        SourceKind::Managed => {}
        SourceKind::Linked => {
            return Err(
                AppError::validation(msg::ROLLBACK_LINKED).with_hint(msg::ROLLBACK_LINKED_HINT)
            )
        }
        SourceKind::Synced => {
            return Err(
                AppError::validation(msg::ROLLBACK_SYNCED).with_hint(msg::ROLLBACK_SYNCED_HINT)
            )
        }
    }
    let env = history.env(deploy_id)?;
    stacks.update(&deploy.summary.stack_id, &deploy.files, &env)?;
    stacks.set_profiles(&deploy.summary.stack_id, &deploy.profiles)
}

fn kind_text(kind: SourceKind) -> &'static str {
    match kind {
        SourceKind::Managed => "managed",
        SourceKind::Linked => "linked",
        SourceKind::Synced => "synced",
    }
}

fn action_text(action: ComposeAction) -> &'static str {
    match action {
        ComposeAction::Up => "up",
        ComposeAction::Rebuild => "rebuild",
        ComposeAction::Update => "update",
        ComposeAction::Pull => "pull",
        ComposeAction::Down => "down",
        ComposeAction::DownWithVolumes => "downWithVolumes",
    }
}

fn read_row(row: &Row<'_>) -> rusqlite::Result<Deploy> {
    let invalid = |i: usize, e: serde_json::Error| {
        rusqlite::Error::FromSqlConversionFailure(i, rusqlite::types::Type::Text, e.into())
    };
    let kind: String = row.get(2)?;
    let action: String = row.get(3)?;
    let definition: String = row.get(9)?;
    let profiles: String = row.get(10)?;
    Ok(Deploy {
        summary: DeploySummary {
            id: row.get(0)?,
            stack_id: row.get(1)?,
            kind: match kind.as_str() {
                "linked" => SourceKind::Linked,
                "synced" => SourceKind::Synced,
                _ => SourceKind::Managed,
            },
            action: serde_json::from_value(serde_json::Value::String(action))
                .map_err(|e| invalid(3, e))?,
            started_at: row.get(4)?,
            finished_at: row.get(5)?,
            success: row.get(6)?,
            exit_code: row.get(7)?,
            has_env: row.get(8)?,
            commit: row.get(11)?,
            automatic: row.get(12)?,
        },
        files: serde_json::from_str(&definition).map_err(|e| invalid(9, e))?,
        profiles: serde_json::from_str(&profiles).map_err(|e| invalid(10, e))?,
    })
}

#[cfg(test)]
mod tests;

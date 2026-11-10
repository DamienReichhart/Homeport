//! Previewing and importing Hosts from `~/.ssh/config` and Docker contexts.
//! Keys and certificates are read again at import, on this side, so they
//! never travel through the frontend.

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::{BastionKeyInput, CredentialsInput, HostInput, HostService};
use crate::{
    import::{
        docker_context,
        ssh_config::{self, DEFAULT_SOCKET},
        Env, ImportChoice, ImportDraft, ImportFailure, ImportOutcome,
    },
    messages as msg, EngineKind, HostConnection,
};

impl HostService {
    fn env() -> AppResult<Env> {
        Env::current().ok_or_else(|| AppError::new(ErrorCode::NotFound, msg::NO_HOME_FOLDER))
    }

    fn host_names(&self) -> AppResult<Vec<String>> {
        Ok(self.store.list()?.into_iter().map(|h| h.name).collect())
    }

    /// The servers of `~/.ssh/config`, as drafts.
    pub fn preview_ssh_config(&self) -> AppResult<Vec<ImportDraft>> {
        self.preview_ssh_config_in(&Self::env()?)
    }

    pub fn preview_ssh_config_in(&self, env: &Env) -> AppResult<Vec<ImportDraft>> {
        let config = ssh_config::read(env)?;
        Ok(ssh_config::drafts(&config, env, &self.host_names()?))
    }

    /// Creates a Host for each chosen alias, with its key and bastions' keys.
    pub fn import_ssh_config(&self, choices: Vec<ImportChoice>) -> AppResult<ImportOutcome> {
        self.import_ssh_config_in(&Self::env()?, choices)
    }

    pub fn import_ssh_config_in(
        &self,
        env: &Env,
        choices: Vec<ImportChoice>,
    ) -> AppResult<ImportOutcome> {
        let config = ssh_config::read(env)?;
        Ok(self.import_each(choices, |choice| {
            let plan = ssh_config::plan(&config, env, &choice.source)?;
            let passphrase = choice.passphrase.clone().filter(|p| !p.is_empty());
            let mut connection = plan.connection;
            set_socket(&mut connection, choice.socket_path.as_deref());
            Ok(HostInput {
                name: choice.name.clone(),
                connection,
                engine: EngineKind::Docker,
                credentials: plan.key.map(|k| CredentialsInput::SshKey {
                    private_key: k.text,
                    passphrase: passphrase.clone(),
                }),
                bastion_keys: plan
                    .bastion_keys
                    .into_iter()
                    .map(|(i, k)| BastionKeyInput {
                        index: i as u32,
                        private_key: k.text,
                        passphrase: passphrase.clone(),
                    })
                    .collect(),
            })
        }))
    }

    /// The Docker CLI's contexts, as drafts.
    pub fn preview_docker_contexts(&self) -> AppResult<Vec<ImportDraft>> {
        self.preview_docker_contexts_in(&Self::env()?)
    }

    pub fn preview_docker_contexts_in(&self, env: &Env) -> AppResult<Vec<ImportDraft>> {
        Ok(docker_context::drafts(
            docker_context::read(env),
            &self.store.list()?,
        ))
    }

    /// Creates a Host for each chosen context, with its TLS material.
    pub fn import_docker_contexts(&self, choices: Vec<ImportChoice>) -> AppResult<ImportOutcome> {
        self.import_docker_contexts_in(&Self::env()?, choices)
    }

    pub fn import_docker_contexts_in(
        &self,
        env: &Env,
        choices: Vec<ImportChoice>,
    ) -> AppResult<ImportOutcome> {
        let mut contexts = docker_context::read(env);
        Ok(self.import_each(choices, |choice| {
            let index = contexts
                .iter()
                .position(|c| c.name == choice.source)
                .ok_or_else(|| {
                    AppError::validation(msg::IMPORT_SOURCE_GONE.with("source", &choice.source))
                })?;
            let plan = contexts.swap_remove(index).plan?;
            let mut connection = plan.connection;
            set_socket(&mut connection, choice.socket_path.as_deref());
            Ok(HostInput {
                name: choice.name.clone(),
                connection,
                engine: EngineKind::Docker,
                credentials: plan.tls.map(|t| CredentialsInput::Tls {
                    ca_pem: t.ca_pem,
                    cert_pem: t.cert_pem,
                    key_pem: t.key_pem,
                }),
                bastion_keys: vec![],
            })
        }))
    }

    /// Creates each chosen entry, carrying on past the ones that fail.
    fn import_each(
        &self,
        choices: Vec<ImportChoice>,
        mut input: impl FnMut(&ImportChoice) -> AppResult<HostInput>,
    ) -> ImportOutcome {
        let mut outcome = ImportOutcome {
            created: vec![],
            failed: vec![],
        };
        for choice in choices {
            match input(&choice).and_then(|i| self.create(i)) {
                Ok(view) => outcome.created.push(view),
                Err(error) => outcome.failed.push(ImportFailure {
                    source: choice.source,
                    error,
                }),
            }
        }
        outcome
    }
}

/// Puts the socket the user typed in the preview on an SSH connection.
fn set_socket(connection: &mut HostConnection, socket: Option<&str>) {
    if let HostConnection::Ssh { socket_path, .. } = connection {
        *socket_path = socket
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or(DEFAULT_SOCKET)
            .to_owned();
    }
}

#[cfg(test)]
mod tests;

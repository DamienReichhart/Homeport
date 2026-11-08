//! Running `helm` on this machine: against a Cluster through its private
//! proxy, or alone for what only touches helm's own configuration.

use serde::de::DeserializeOwned;

use homeport_core::error::{AppError, AppResult};

use super::{errors, program, FOREIGN_ENV};
use crate::{
    messages as msg,
    proxy::{self, CliCommand, CliEvent, ClusterAccess},
};

/// `helm`, ready to run against one Cluster or on its own.
pub struct Helm {
    program: std::path::PathBuf,
    access: Option<ClusterAccess>,
}

impl Helm {
    /// helm run against the Cluster `access` reaches.
    pub fn on(access: ClusterAccess) -> AppResult<Self> {
        Ok(Self {
            program: program()?,
            access: Some(access),
        })
    }

    /// helm run on its own, for chart repositories and charts.
    pub fn local() -> AppResult<Self> {
        Ok(Self {
            program: program()?,
            access: None,
        })
    }

    fn cluster_name(&self) -> Option<&str> {
        self.access.as_ref().map(|a| a.cluster.name.as_str())
    }

    fn command(&self, args: Vec<String>, stdin: Option<String>) -> CliCommand {
        CliCommand {
            program: self.program.clone(),
            args,
            stdin,
            env: vec![],
            env_remove: FOREIGN_ENV.iter().map(|v| (*v).to_owned()).collect(),
        }
    }

    /// What `helm <args>` printed on stdout.
    pub async fn text(&self, args: Vec<String>, stdin: Option<String>) -> AppResult<String> {
        let verb = args.first().cloned().unwrap_or_default();
        let command = self.command(args, stdin);
        let name = self.cluster_name();
        proxy::output(&command, self.access.as_ref(), |f| {
            errors::failure(&verb, name, f)
        })
        .await
    }

    /// What `helm <args> -o json` printed, read as `T`.
    pub async fn json<T: DeserializeOwned>(&self, mut args: Vec<String>) -> AppResult<T> {
        args.extend(["--output".into(), "json".into()]);
        let text = self.text(args, None).await?;
        serde_json::from_str(&text).map_err(|e| {
            AppError::new(
                homeport_core::error::ErrorCode::HelmFailed,
                msg::HELM_UNREADABLE,
            )
            .with_hint(msg::HELM_FAILED_HINT)
            .with_cause(&e)
        })
    }

    /// Runs `helm <args>`, streaming its output to `sink`.
    pub async fn stream(
        self,
        args: Vec<String>,
        stdin: Option<String>,
        sink: impl Fn(CliEvent) -> bool,
    ) {
        let verb = args.first().cloned().unwrap_or_default();
        let command = self.command(args, stdin);
        let name = self.cluster_name().map(str::to_owned);
        proxy::run_cli(
            command,
            self.access,
            |f| errors::failure(&verb, name.as_deref(), f),
            sink,
        )
        .await;
    }
}

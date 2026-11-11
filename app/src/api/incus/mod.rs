//! What Incus Hosts offer, through Incus's REST API (ADR 0021). Every
//! command about a project-scoped object names its project; a Host whose
//! Engine isn't Incus answers `EngineUnsupported`.

pub mod images;
pub mod instances;
pub mod networks;
pub mod profiles;
pub mod snapshots;
pub mod storage;
pub mod terminals;

use homeport_core::{
    error::AppResult,
    host::{EngineKind, HostConnection},
};
use homeport_hosts::service::{CredentialsInput, HostInput, HostView};
use homeport_incus::{
    projects::{self, IncusProject},
    trust::{self, TrustToken, TrustTokenSummary},
};

use crate::app::AppState;

/// What a pasted trust token says, for the Host form to show before adding.
pub fn incus_trust_token_read(token: String) -> AppResult<TrustTokenSummary> {
    Ok(TrustToken::parse(&token)?.summary())
}

/// Adds an Incus Host over HTTPS with a trust token from
/// `incus config trust add`: Homeport makes a client certificate, registers
/// it with the token, and keeps the server's certificate pinned (ADR 0021).
pub async fn incus_trust_add(state: &AppState, name: String, token: String) -> AppResult<HostView> {
    let token = TrustToken::parse(&token)?;
    let name = if name.trim().is_empty() {
        token.client_name.clone()
    } else {
        name.trim().to_owned()
    };
    let trusted = trust::join(&name, &token).await?;
    state.hosts.create(HostInput {
        name,
        connection: HostConnection::Tls {
            address: trusted.address,
        },
        engine: EngineKind::Incus,
        credentials: Some(CredentialsInput::Tls {
            ca_pem: trusted.server_pem,
            cert_pem: trusted.client.cert_pem,
            key_pem: trusted.client.key_pem,
        }),
        bastion_keys: vec![],
    })
}

/// The projects the Host's connection may use, with what each holds.
pub async fn incus_projects(state: &AppState, host_id: String) -> AppResult<Vec<IncusProject>> {
    projects::list(&state.hosts.registry.incus(&host_id)?).await
}

#[cfg(test)]
mod tests;

//! Nomad Clusters: what the add and edit form sends, how it becomes a
//! connection and sealed credentials, and the client made from them.

use serde::Deserialize;

use homeport_core::error::{AppError, AppResult};
use homeport_nomad::{normalize_address, Nomad, NomadConfig};

use crate::{
    messages as msg,
    model::{NomadConnection, NomadSecrets},
};

/// A Nomad Cluster as the add and edit form describe it. When editing, a
/// credential left `None` keeps the stored one and an empty one removes it.
#[derive(Clone, Default, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct NomadForm {
    pub name: String,
    /// Such as `nomad.example.com:4646`, `10.0.0.5` or `https://nomad:4646`.
    pub address: String,
    pub region: Option<String>,
    pub tls_server_name: Option<String>,
    /// The ACL token's secret ID.
    pub token: Option<String>,
    pub certificate_authority: Option<String>,
    pub client_certificate: Option<String>,
    pub client_key: Option<String>,
}

impl std::fmt::Debug for NomadForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NomadForm")
            .field("name", &self.name)
            .field("address", &self.address)
            .finish_non_exhaustive()
    }
}

/// `value` trimmed, `None` when empty.
fn clean(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
}

/// The form's credential when it gives one (empty removes it), else the stored one.
fn merged(given: Option<&String>, stored: Option<&String>) -> Option<String> {
    match given {
        Some(value) => clean(Some(value)),
        None => stored.cloned(),
    }
}

impl NomadForm {
    /// The connection and credentials the form describes, keeping the
    /// credentials of `stored` it leaves out.
    pub fn draft(
        &self,
        stored: Option<&NomadSecrets>,
    ) -> AppResult<(NomadConnection, NomadSecrets)> {
        let secrets = NomadSecrets {
            token: merged(self.token.as_ref(), stored.and_then(|s| s.token.as_ref())),
            certificate_authority: merged(
                self.certificate_authority.as_ref(),
                stored.and_then(|s| s.certificate_authority.as_ref()),
            ),
            client_certificate: merged(
                self.client_certificate.as_ref(),
                stored.and_then(|s| s.client_certificate.as_ref()),
            ),
            client_key: merged(
                self.client_key.as_ref(),
                stored.and_then(|s| s.client_key.as_ref()),
            ),
        };
        if secrets.client_certificate.is_some() != secrets.client_key.is_some() {
            return Err(AppError::new(
                homeport_core::error::ErrorCode::TlsMaterialInvalid,
                homeport_nomad::messages::CLIENT_PAIR_INCOMPLETE,
            ));
        }
        let tls = secrets.certificate_authority.is_some()
            || secrets.client_certificate.is_some()
            || clean(self.tls_server_name.as_deref()).is_some();
        let connection = NomadConnection {
            address: normalize_address(&self.address, tls)?,
            region: clean(self.region.as_deref()),
            tls_server_name: clean(self.tls_server_name.as_deref()),
            acl_token: secrets.token.is_some(),
            custom_ca: secrets.certificate_authority.is_some(),
            client_certificate: secrets.client_certificate.is_some(),
        };
        Ok((connection, secrets))
    }

    /// The name given, trimmed; refused when empty.
    pub fn name(&self) -> AppResult<&str> {
        let name = self.name.trim();
        if name.is_empty() {
            return Err(AppError::validation(msg::NAME_EMPTY));
        }
        Ok(name)
    }
}

/// The client of the Nomad Cluster `name`, from its connection and credentials.
pub fn client(
    name: &str,
    connection: &NomadConnection,
    secrets: &NomadSecrets,
) -> AppResult<Nomad> {
    Nomad::new(&NomadConfig {
        name: name.to_owned(),
        address: connection.address.clone(),
        region: connection.region.clone(),
        tls_server_name: connection.tls_server_name.clone(),
        token: secrets.token.clone(),
        certificate_authority: secrets.certificate_authority.clone(),
        client_certificate: secrets.client_certificate.clone(),
        client_key: secrets.client_key.clone(),
    })
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;

    fn form() -> NomadForm {
        NomadForm {
            name: " prod ".into(),
            address: "nomad.example.com".into(),
            token: Some(" secret ".into()),
            ..Default::default()
        }
    }

    #[test]
    fn a_new_cluster_is_drafted_with_its_address_completed() {
        let (connection, secrets) = form().draft(None).unwrap();
        assert_eq!(connection.address, "http://nomad.example.com:4646");
        assert!(connection.acl_token && !connection.custom_ca);
        assert_eq!(secrets.token.as_deref(), Some("secret"));
        assert_eq!(form().name().unwrap(), "prod");
    }

    #[test]
    fn tls_material_makes_a_bare_address_https() {
        let (connection, _) = NomadForm {
            certificate_authority: Some("-----BEGIN CERTIFICATE-----".into()),
            ..form()
        }
        .draft(None)
        .unwrap();
        assert_eq!(connection.address, "https://nomad.example.com:4646");
        assert!(connection.custom_ca);
    }

    #[test]
    fn editing_keeps_the_credentials_left_out_and_removes_the_emptied_ones() {
        let stored = NomadSecrets {
            token: Some("old".into()),
            certificate_authority: Some("CA".into()),
            ..Default::default()
        };
        let (connection, secrets) = NomadForm {
            token: None,
            certificate_authority: Some(String::new()),
            ..form()
        }
        .draft(Some(&stored))
        .unwrap();
        assert_eq!(secrets.token.as_deref(), Some("old"));
        assert_eq!(secrets.certificate_authority, None);
        assert!(connection.acl_token && !connection.custom_ca);
    }

    #[test]
    fn a_client_certificate_needs_its_key() {
        let e = NomadForm {
            client_certificate: Some("CERT".into()),
            ..form()
        }
        .draft(None)
        .unwrap_err();
        assert!(e.is(ErrorCode::TlsMaterialInvalid));
    }

    #[test]
    fn the_form_never_prints_its_secrets() {
        assert!(!format!("{:?}", form()).contains("secret"));
    }
}

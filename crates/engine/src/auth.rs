//! Registry logins as the Engine takes them. The shell finds the login in
//! the registries crate and hands it here; this crate never stores one.

use std::fmt;

use bollard::auth::DockerCredentials;

/// The Engine's address for Docker Hub's logins, as `docker login` records it.
const DOCKER_HUB_SERVER: &str = "https://index.docker.io/v1/";

/// A login for one registry, its secret in clear only in memory. `Debug`
/// hides the secret so it can't reach a log.
#[derive(Clone, PartialEq, Eq)]
pub struct RegistryAuth {
    /// Normalised: `docker.io`, `ghcr.io`, `registry.local:5000`.
    pub address: String,
    pub username: String,
    pub secret: String,
}

impl fmt::Debug for RegistryAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegistryAuth")
            .field("address", &self.address)
            .field("username", &self.username)
            .field("secret", &"<redacted>")
            .finish()
    }
}

impl RegistryAuth {
    /// The address the Engine files this login under.
    pub fn server_address(&self) -> String {
        if self.address == "docker.io" {
            DOCKER_HUB_SERVER.to_owned()
        } else {
            self.address.clone()
        }
    }

    /// The `X-Registry-Auth` credentials of a pull or a push.
    pub(crate) fn to_docker(&self) -> DockerCredentials {
        DockerCredentials {
            username: Some(self.username.clone()),
            password: Some(self.secret.clone()),
            serveraddress: Some(self.server_address()),
            ..Default::default()
        }
    }
}

/// Every stored login, and how to tell which one an image reference needs.
/// The registries crate owns that rule; the shell passes it in, so this
/// crate and the ones above it never depend on the registries crate.
#[derive(Clone)]
pub struct Logins {
    logins: Vec<RegistryAuth>,
    registry_of: fn(&str) -> String,
}

impl fmt::Debug for Logins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.logins).finish()
    }
}

impl Logins {
    pub fn new(logins: Vec<RegistryAuth>, registry_of: fn(&str) -> String) -> Self {
        Self {
            logins,
            registry_of,
        }
    }

    /// No logins: every pull is anonymous.
    pub fn none() -> Self {
        Self::new(Vec::new(), |_| String::new())
    }

    /// The login for the registry `reference` is pulled from or pushed to.
    pub fn for_reference(&self, reference: &str) -> Option<&RegistryAuth> {
        let registry = (self.registry_of)(reference);
        self.logins.iter().find(|l| l.address == registry)
    }

    pub fn all(&self) -> &[RegistryAuth] {
        &self.logins
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn login(address: &str) -> RegistryAuth {
        RegistryAuth {
            address: address.into(),
            username: "alice".into(),
            secret: "s3cret".into(),
        }
    }

    #[test]
    fn logins_become_the_engines_credentials() {
        let creds = login("ghcr.io").to_docker();
        assert_eq!(creds.username.as_deref(), Some("alice"));
        assert_eq!(creds.password.as_deref(), Some("s3cret"));
        assert_eq!(creds.serveraddress.as_deref(), Some("ghcr.io"));
        assert_eq!(
            login("docker.io").to_docker().serveraddress.as_deref(),
            Some("https://index.docker.io/v1/")
        );
    }

    #[test]
    fn secrets_stay_out_of_debug_output() {
        assert!(!format!("{:?}", login("ghcr.io")).contains("s3cret"));
        let logins = Logins::new(vec![login("ghcr.io")], |_| String::new());
        assert!(!format!("{logins:?}").contains("s3cret"));
    }

    #[test]
    fn logins_are_picked_by_the_reference_registry() {
        let registry_of = |r: &str| r.split('/').next().unwrap_or_default().to_owned();
        let logins = Logins::new(vec![login("ghcr.io")], registry_of);
        assert_eq!(
            logins
                .for_reference("ghcr.io/org/app")
                .map(|l| l.address.as_str()),
            Some("ghcr.io")
        );
        assert!(logins.for_reference("quay.io/org/app").is_none());
        assert!(Logins::none().for_reference("ghcr.io/x").is_none());
    }
}

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
    }
}

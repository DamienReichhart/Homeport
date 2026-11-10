//! The address of a Nomad agent, as the user types it and as Homeport keeps it.

use homeport_core::error::{AppError, AppResult};

use crate::messages as msg;

/// The port Nomad's HTTP API listens on unless configured otherwise.
pub const DEFAULT_PORT: u16 = 4646;

/// `input` as a full URL: `nomad.example.com` becomes
/// `http://nomad.example.com:4646` (`https://` when `tls`), a given scheme
/// and port are kept, and a trailing slash goes. Refuses anything with a
/// path, a query or another scheme.
pub fn normalize_address(input: &str, tls: bool) -> AppResult<String> {
    let trimmed = input.trim().trim_end_matches('/');
    let invalid = || AppError::validation(msg::ADDRESS_INVALID.with("address", input.trim()));
    if trimmed.is_empty() {
        return Err(invalid());
    }
    let full = if trimmed.contains("://") {
        trimmed.to_owned()
    } else {
        format!("{}://{trimmed}", if tls { "https" } else { "http" })
    };
    let uri: http::Uri = full.parse().map_err(|_| invalid())?;
    let scheme = uri.scheme_str().ok_or_else(invalid)?;
    if scheme != "http" && scheme != "https" {
        return Err(invalid());
    }
    let authority = uri.authority().ok_or_else(invalid)?;
    if authority.host().is_empty()
        || authority.as_str().contains('@')
        || uri.query().is_some()
        || !matches!(uri.path(), "" | "/")
    {
        return Err(invalid());
    }
    let port = authority.port_u16().unwrap_or(DEFAULT_PORT);
    Ok(format!("{scheme}://{}:{port}", authority.host()))
}

/// The `host:port` of a normalised address, as messages name it.
pub(crate) fn host_port(address: &str) -> &str {
    address.split_once("://").map_or(address, |(_, rest)| rest)
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;

    #[test]
    fn a_bare_host_gets_a_scheme_and_the_default_port() {
        assert_eq!(
            normalize_address("nomad.example.com", false).unwrap(),
            "http://nomad.example.com:4646"
        );
        assert_eq!(
            normalize_address(" nomad.example.com/ ", true).unwrap(),
            "https://nomad.example.com:4646"
        );
        assert_eq!(
            normalize_address("10.0.0.5:14646", false).unwrap(),
            "http://10.0.0.5:14646"
        );
    }

    #[test]
    fn a_given_scheme_and_port_are_kept() {
        assert_eq!(
            normalize_address("https://127.0.0.1:4646", false).unwrap(),
            "https://127.0.0.1:4646"
        );
        assert_eq!(
            normalize_address("http://[::1]", false).unwrap(),
            "http://[::1]:4646"
        );
    }

    #[test]
    fn anything_but_an_agent_address_is_refused() {
        for bad in [
            "",
            "ftp://nomad",
            "http://nomad:4646/ui",
            "http://nomad:4646?x=1",
            "http://user@nomad",
        ] {
            let e = normalize_address(bad, false).unwrap_err();
            assert!(e.is(ErrorCode::Validation), "{bad}: {e:?}");
        }
    }

    #[test]
    fn host_port_drops_the_scheme() {
        assert_eq!(host_port("https://nomad:4646"), "nomad:4646");
    }
}

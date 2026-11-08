//! Adding an Incus Host over HTTPS with a trust token from
//! `incus config trust add <name>` (ADR 0021): Homeport makes a client
//! certificate in memory, finds the address that presents the server
//! certificate the token names, and registers itself with the token.

use std::sync::Arc;

use base64::Engine as _;
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::{
    http::{self, Request},
    tls::{self, TlsDialer},
    Dialer,
};

use crate::{client, messages as msg};

/// What a trust token holds, as Incus encodes it (`api.CertificateAddToken`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TrustToken {
    /// The name the certificate will have in Incus's trust store.
    pub client_name: String,
    /// The SHA-256 fingerprint of the server's certificate.
    pub fingerprint: String,
    /// Where the server listens, `host:port`, in Incus's order.
    #[serde(default)]
    pub addresses: Vec<String>,
    pub secret: String,
    /// When the token stops working; Incus writes the zero time for never.
    #[serde(default)]
    pub expires_at: Option<String>,
    /// The text pasted, which Incus wants back whole.
    #[serde(skip)]
    pub raw: String,
}

/// A client certificate and its key, as PEM.
#[derive(Clone, PartialEq, Eq)]
pub struct ClientCertificate {
    pub cert_pem: String,
    pub key_pem: String,
}

impl std::fmt::Debug for ClientCertificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ClientCertificate(..)")
    }
}

/// An Incus that now trusts Homeport: where it answered, its certificate
/// to pin, and Homeport's own certificate for it.
#[derive(Debug, Clone)]
pub struct Trusted {
    pub address: String,
    pub server_pem: String,
    pub client: ClientCertificate,
}

/// What the Host form shows about a pasted token before adding the Host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TrustTokenSummary {
    pub client_name: String,
    pub fingerprint: String,
    pub addresses: Vec<String>,
    /// RFC 3339, `None` for a token that doesn't expire.
    pub expires_at: Option<String>,
}

impl TrustToken {
    /// Reads `text` as a trust token, refusing one that has expired.
    pub fn parse(text: &str) -> AppResult<Self> {
        let raw: String = text.split_whitespace().collect();
        let invalid = || {
            AppError::new(ErrorCode::IncusTokenInvalid, msg::TOKEN_INVALID)
                .with_hint(msg::TOKEN_INVALID_HINT)
                .with_command("incus config trust add homeport")
        };
        if raw.is_empty() {
            return Err(invalid());
        }
        let engines = [
            base64::engine::general_purpose::STANDARD,
            base64::engine::general_purpose::URL_SAFE,
        ];
        let json = engines
            .iter()
            .find_map(|engine| {
                engine
                    .decode(&raw)
                    .ok()
                    .or_else(|| engine.decode(pad(&raw)).ok())
            })
            .ok_or_else(invalid)?;
        let mut token: TrustToken =
            serde_json::from_slice(&json).map_err(|e| invalid().with_detail(e))?;
        if token.secret.is_empty() || token.fingerprint.is_empty() {
            return Err(invalid());
        }
        token.fingerprint = token.fingerprint.to_lowercase();
        token.raw = raw;
        if let Some(expiry) = token.expiry() {
            if expiry <= jiff::Timestamp::now() {
                return Err(AppError::new(
                    ErrorCode::IncusTokenInvalid,
                    msg::TOKEN_EXPIRED.with("time", expiry.to_string()),
                )
                .with_hint(msg::TOKEN_EXPIRED_HINT)
                .with_command(format!("incus config trust add {}", token.client_name)));
            }
        }
        Ok(token)
    }

    /// When it expires; `None` for never (Incus's zero time).
    fn expiry(&self) -> Option<jiff::Timestamp> {
        let time: jiff::Timestamp = self.expires_at.as_deref()?.parse().ok()?;
        (time.as_second() > 0).then_some(time)
    }

    pub fn summary(&self) -> TrustTokenSummary {
        TrustTokenSummary {
            client_name: self.client_name.clone(),
            fingerprint: self.fingerprint.clone(),
            addresses: self.addresses.clone(),
            expires_at: self.expiry().map(|t| t.to_string()),
        }
    }
}

/// `text` padded with `=` to a multiple of four.
fn pad(text: &str) -> String {
    let mut padded = text.trim_end_matches('=').to_owned();
    while padded.len() % 4 != 0 {
        padded.push('=');
    }
    padded
}

/// A new ECDSA P-384 client certificate named `name`, made in memory and
/// never written to disk: the Host's secrets keep it sealed (ADR 0002).
pub fn client_certificate(name: &str) -> AppResult<ClientCertificate> {
    let failed =
        |e: rcgen::Error| AppError::new(ErrorCode::Io, msg::CERTIFICATE_FAILED).with_detail(e);
    let key = rcgen::KeyPair::generate_for(&rcgen::PKCS_ECDSA_P384_SHA384).map_err(failed)?;
    let mut params = rcgen::CertificateParams::new(Vec::<String>::new()).map_err(failed)?;
    params
        .distinguished_name
        .push(rcgen::DnType::CommonName, name);
    params
        .distinguished_name
        .push(rcgen::DnType::OrganizationName, "Homeport");
    params.extended_key_usages = vec![rcgen::ExtendedKeyUsagePurpose::ClientAuth];
    let cert = params.self_signed(&key).map_err(failed)?;
    Ok(ClientCertificate {
        cert_pem: cert.pem(),
        key_pem: key.serialize_pem(),
    })
}

/// Registers a new client certificate with the Incus `token` names, at the
/// first of its addresses that presents the certificate it names, and
/// checks that Incus now trusts it. `name` is the Host's, for messages.
pub async fn join(name: &str, token: &TrustToken) -> AppResult<Trusted> {
    if token.addresses.is_empty() {
        return Err(not_listening(token));
    }
    let mut last: Option<AppError> = None;
    let mut mismatch: Option<AppError> = None;
    for address in &token.addresses {
        let label = format!("{name} ({address})");
        let der = match tls::server_certificate(&label, address).await {
            Ok(der) => der,
            Err(e) => {
                last = Some(e);
                continue;
            }
        };
        let presented = tls::fingerprint(&der);
        if presented != token.fingerprint {
            mismatch = Some(
                AppError::new(
                    ErrorCode::CertificateMismatch,
                    msg::TOKEN_FINGERPRINT_MISMATCH
                        .with("address", address)
                        .with("presented", &presented)
                        .with("expected", &token.fingerprint),
                )
                .with_hint(msg::TOKEN_FINGERPRINT_MISMATCH_HINT),
            );
            continue;
        }
        let server_pem = tls::pem(&der);
        let client = client_certificate(&format!("homeport-{}", token.client_name))?;
        let dialer: Arc<dyn Dialer> = Arc::new(
            TlsDialer::pinned(
                &label,
                address,
                &server_pem,
                &client.cert_pem,
                &client.key_pem,
            )
            .map_err(AppError::from)?,
        );
        register(&*dialer, token).await?;
        return Ok(Trusted {
            address: address.clone(),
            server_pem,
            client,
        });
    }
    Err(mismatch
        .or_else(|| {
            last.filter(|e| {
                !matches!(
                    e.code,
                    ErrorCode::ConnectionRefused
                        | ErrorCode::Timeout
                        | ErrorCode::NetworkUnreachable
                        | ErrorCode::DnsFailed
                )
            })
        })
        .unwrap_or_else(|| not_listening(token)))
}

/// Sends the token with Homeport's certificate and checks the result.
async fn register(dialer: &dyn Dialer, token: &TrustToken) -> AppResult<()> {
    let body = serde_json::json!({ "type": "client", "trust_token": token.raw });
    let response = http::send(dialer, Request::post("/1.0/certificates").json(&body)?).await?;
    if let Err(e) = client::read_envelope(&response) {
        return Err(match e.code {
            ErrorCode::Forbidden | ErrorCode::NotFound | ErrorCode::BadRequest => refused(token, e),
            _ => e,
        });
    }
    let check = http::send(dialer, Request::get("/1.0")).await?;
    let envelope = client::read_envelope(&check)?;
    let auth = envelope
        .metadata
        .get("auth")
        .and_then(|a| a.as_str())
        .unwrap_or_default();
    if auth != "trusted" {
        return Err(
            refused(token, AppError::new(ErrorCode::Forbidden, msg::FORBIDDEN))
                .with_detail(format!("auth: {auth:?}")),
        );
    }
    Ok(())
}

fn refused(token: &TrustToken, cause: AppError) -> AppError {
    AppError::new(ErrorCode::IncusTokenRefused, msg::TOKEN_REFUSED)
        .with_hint(msg::TOKEN_REFUSED_HINT)
        .with_command(format!("incus config trust add {}", token.client_name))
        .with_detail(cause.detail.unwrap_or(cause.message))
}

fn not_listening(token: &TrustToken) -> AppError {
    AppError::new(
        ErrorCode::IncusNotListening,
        msg::NOT_LISTENING.with("addresses", token.addresses.join(", ")),
    )
    .with_hint(msg::NOT_LISTENING_HINT)
    .with_command("incus config set core.https_address :8443")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(json: serde_json::Value) -> String {
        base64::engine::general_purpose::STANDARD.encode(json.to_string())
    }

    fn token(expires_at: &str) -> String {
        encode(serde_json::json!({
            "client_name": "homeport",
            "fingerprint": "8D6A1F0A3C",
            "addresses": ["10.0.0.5:8443", "[fd42::5]:8443"],
            "secret": "5e1c0f",
            "expires_at": expires_at
        }))
    }

    #[test]
    fn tokens_read_whatever_their_padding_and_wrapping() {
        let text = token("0001-01-01T00:00:00Z");
        let parsed = TrustToken::parse(&format!("  {text}\n")).unwrap();
        assert_eq!(parsed.client_name, "homeport");
        assert_eq!(parsed.fingerprint, "8d6a1f0a3c");
        assert_eq!(parsed.addresses, ["10.0.0.5:8443", "[fd42::5]:8443"]);
        assert_eq!(parsed.raw, text);
        assert_eq!(parsed.summary().expires_at, None);

        let unpadded = text.trim_end_matches('=');
        let wrapped = format!("{}\n{}", &unpadded[..20], &unpadded[20..]);
        assert_eq!(TrustToken::parse(&wrapped).unwrap().secret, "5e1c0f");
    }

    #[test]
    fn expired_and_garbled_tokens_are_refused() {
        let expired = TrustToken::parse(&token("2020-01-01T00:00:00Z")).unwrap_err();
        assert_eq!(expired.code, ErrorCode::IncusTokenInvalid);
        assert!(expired.message.contains("expired"), "{expired:?}");

        let later = TrustToken::parse(&token("2999-01-01T00:00:00Z")).unwrap();
        assert!(later.summary().expires_at.is_some());

        for text in [
            "",
            "not a token",
            &encode(serde_json::json!({ "secret": "x" })),
        ] {
            let e = TrustToken::parse(text).unwrap_err();
            assert_eq!(e.code, ErrorCode::IncusTokenInvalid, "{text}");
            assert!(e.hint.is_some());
        }
    }

    #[test]
    fn client_certificates_are_pem_that_tls_accepts() {
        let cert = client_certificate("homeport-test").unwrap();
        assert!(cert.cert_pem.starts_with("-----BEGIN CERTIFICATE-----"));
        assert!(cert.key_pem.contains("PRIVATE KEY"));
        assert!(format!("{cert:?}").contains(".."));
        // Its own certificate pinned as the server's: the material is usable.
        assert!(TlsDialer::pinned(
            "lab (10.0.0.5:8443)",
            "10.0.0.5:8443",
            &cert.cert_pem,
            &cert.cert_pem,
            &cert.key_pem
        )
        .is_ok());
    }

    #[tokio::test]
    async fn no_listening_address_says_how_to_make_incus_listen() {
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let mut parsed = TrustToken::parse(&token("0001-01-01T00:00:00Z")).unwrap();
        parsed.addresses = vec![format!("127.0.0.1:{port}")];
        let e = join("lab", &parsed).await.unwrap_err();
        assert_eq!(e.code, ErrorCode::IncusNotListening, "{e:?}");
        assert!(e.hint.unwrap().commands[0].contains("core.https_address"));
    }

    /// Runs the `incus` CLI on this machine, for the live tests' setup.
    fn incus_cli(args: &[&str]) -> String {
        let out = std::process::Command::new("incus")
            .args(args)
            .output()
            .expect("the incus CLI runs");
        assert!(out.status.success(), "incus {args:?}: {out:?}");
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    #[tokio::test]
    #[ignore = "needs a local Incus"]
    async fn incus_live_trust_tokens_register_homeport() {
        let name = format!("homeport-test-{}", std::process::id());
        let printed = incus_cli(&["config", "trust", "add", &name]);
        let token = TrustToken::parse(printed.lines().last().unwrap()).unwrap();
        let trusted = join("lab", &token).await.unwrap();
        let dialer: Arc<dyn Dialer> = Arc::new(
            TlsDialer::pinned(
                "lab",
                &trusted.address,
                &trusted.server_pem,
                &trusted.client.cert_pem,
                &trusted.client.key_pem,
            )
            .unwrap(),
        );
        let incus = crate::Incus::connect("lab", dialer).await.unwrap();
        assert!(!incus.server().version.is_empty());
        let list = incus_cli(&["config", "trust", "list", "--format", "csv"]);
        let line = list.lines().find(|l| l.contains(&name)).expect("trusted");
        let fingerprint = line.split(',').nth(3).unwrap_or_default().to_owned();
        incus_cli(&["config", "trust", "remove", &fingerprint]);
        // A token works once.
        let again = join("lab", &token).await.unwrap_err();
        assert_eq!(again.code, ErrorCode::IncusTokenRefused, "{again:?}");
    }
}

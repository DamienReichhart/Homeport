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

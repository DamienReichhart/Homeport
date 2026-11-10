use super::*;

/// Realistic log lines, one per kind of secret, each with the secret it
/// must lose and something it must keep. Every secret is made up; the
/// ones shaped like a real service's token are written in two pieces, so
/// secret scanners don't take this file for a leak.
const CORPUS: &[(&str, &str, &str)] = &[
        (
            "[2026-11-02][10:01:12][homeport_hosts][WARN] ssh key rejected: -----BEGIN OPENSSH PRIVATE KEY-----\nb3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQ\nAAAAMwAAAAtzc2gtZWQyNTUxOQ\n-----END OPENSSH PRIVATE KEY-----\n",
            "b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQ",
            "ssh key rejected",
        ),
        (
            r#"[WARN] kubeconfig: {"client-key":"-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAvX\n-----END RSA PRIVATE KEY-----\n"}"#,
            "MIIEowIBAAKCAQEAvX",
            "kubeconfig",
        ),
        (
            "[ERROR] TLS material invalid: -----BEGIN PRIVATE KEY-----\nMIGHAgEAMBMGByqGSM49AgEGCCqGSM49",
            "MIGHAgEAMBMGByqGSM49",
            "TLS material invalid",
        ),
        (
            "[WARN] registry answered 401 for GET /v2/ with Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJvY3RvIn0.c2lnbmF0dXJlLXZhbHVl",
            "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9",
            "registry answered 401",
        ),
        (
            "[WARN] hub login failed: authorization: Basic b2N0bzpodW50ZXIy",
            "b2N0bzpodW50ZXIy",
            "hub login failed",
        ),
        (
            "[WARN] nomad refused the request (X-Nomad-Token: 5b1d7c3e-9f4a-4c2b-8e1d-2a6f0c9b7e34) with 403",
            "5b1d7c3e-9f4a-4c2b-8e1d-2a6f0c9b7e34",
            "with 403",
        ),
        (
            "[ERROR] compose exited 1: environment DB_PASSWORD=hunter2-s3cret could not be interpolated",
            "hunter2-s3cret",
            "could not be interpolated",
        ),
        (
            concat!(
                "[WARN] git fetch failed: fatal: unable to access 'https://octo:gh",
                "p_16C7e42F292c6912E7710c838347Ae178B4a@github.com/acme/infra.git/': timeout"
            ),
            concat!("gh", "p_16C7e42F292c6912E7710c838347Ae178B4a"),
            "github.com/acme/infra.git",
        ),
        (
            "[WARN] GET https://registry.example.com/token?service=registry&token=abc123def456ghi789&scope=pull failed",
            "abc123def456ghi789",
            "scope=pull",
        ),
        (
            "[WARN] kubeconfig user:\n    token: sha256~Zm9vYmFyYmF6cXV4\n    client-key-data: LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0t\n",
            "sha256~Zm9vYmFyYmF6cXV4",
            "kubeconfig user",
        ),
        (
            "[WARN] kubeconfig user:\n    client-key-data: LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0t\n",
            "LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0t",
            "kubeconfig user",
        ),
        (
            r#"[WARN] incus answered: {"error":"not authorized","trust_token":"eyJjbGllbnRfbmFtZSI6ImhvbWVwb3J0In0"}"#,
            "eyJjbGllbnRfbmFtZSI6ImhvbWVwb3J0In0",
            "not authorized",
        ),
        (
            r#"[WARN] helm: Error: failed to fetch (password: "p@ss word!")"#,
            "p@ss word!",
            "failed to fetch",
        ),
        (
            "[WARN] ssh: passphrase='correct horse battery' rejected for key",
            "correct horse battery",
            "rejected for key",
        ),
        (
            concat!(
                "[WARN] pull failed, the engine said: unauthorized gl",
                "pat-xYz12345AbCdEfGhIjKl for project"
            ),
            concat!("gl", "pat-xYz12345AbCdEfGhIjKl"),
            "for project",
        ),
        (
            concat!(
                "[WARN] docker login: dckr_",
                "pat_AbCdEfGhIjKlMnOpQrStUvWx was refused"
            ),
            concat!("dckr_", "pat_AbCdEfGhIjKlMnOpQrStUvWx"),
            "was refused",
        ),
        (
            "[WARN] oidc refresh failed: id-token=eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJrOHMifQ.c2lnbmF0dXJl refresh-token=q1w2e3r4t5y6",
            "q1w2e3r4t5y6",
            "oidc refresh failed",
        ),
        (
            "[WARN] webhook secret=whsec_0123456789abcdef; retrying",
            "whsec_0123456789abcdef",
            "retrying",
        ),
        (
            concat!(
                "[WARN] AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG",
                "/bPxRfiCYEXAMPLEKEY exec plugin failed"
            ),
            concat!("wJalrXUtnFEMI/K7MDENG", "/bPxRfiCYEXAMPLEKEY"),
            "exec plugin failed",
        ),
    ];

#[test]
fn no_secret_of_the_corpus_survives() {
    for (line, secret, kept) in CORPUS {
        let clean = redact(line, &Aliases::default());
        assert!(!clean.contains(secret), "{secret} survives in {clean:?}");
        assert!(clean.contains(kept), "{kept:?} is gone from {clean:?}");
        assert!(clean.contains("[redacted"), "{clean:?}");
    }
}

#[test]
fn a_secret_is_replaced_once() {
    assert_eq!(
        redact(
            "Authorization: Bearer abcdefgh12345678 refused",
            &Aliases::default()
        ),
        "Authorization: [redacted] refused"
    );
    assert_eq!(
        redact(
            "token=abc123def456 and password: hunter2",
            &Aliases::default()
        ),
        "token=[redacted] and password: [redacted]"
    );
}

#[test]
fn ordinary_lines_are_left_alone() {
    for line in [
            "[2026-11-02][10:01:12][homeport_hosts::registry][INFO] host prod connected: Docker Engine 29.1.0, API 1.52",
            "[INFO] deployed stack shop on laptop in 4.2s (3 services)",
            "[WARN] failed to start host connections: connection refused (os error 111)",
            "[INFO] checked for updates: 1.0.0 is the latest",
            "[WARN] the author of this image is unknown",
        ] {
            assert_eq!(redact(line, &Aliases::default()), line);
        }
}

#[test]
fn names_and_addresses_become_their_labels() {
    let mut aliases = Aliases::default();
    aliases.add("prod", "host-1");
    aliases.add_address("prod.example.com:2376", "host-1");
    aliases.add("Kind Dev", "cluster-1");
    aliases.add_address("https://10.1.2.3:6443", "cluster-1");
    let clean = redact(
        "[INFO] host prod (prod.example.com:2376) connected; PROD is up; production is not it\n\
             [WARN] cluster Kind Dev at https://10.1.2.3:6443/api unreachable, 10.1.2.3 timed out",
        &aliases,
    );
    assert_eq!(
            clean,
            "[INFO] host host-1 (host-1-address) connected; host-1 is up; production is not it\n\
             [WARN] cluster cluster-1 at cluster-1-address/api unreachable, cluster-1-address timed out"
        );
}

#[test]
fn this_computer_is_not_an_address_to_hide() {
    let mut aliases = Aliases::default();
    aliases.add_address("127.0.0.1:2375", "host-1");
    let clean = redact("dial 127.0.0.1:2375 failed; 127.0.0.1 refused", &aliases);
    assert_eq!(clean, "dial host-1-address failed; 127.0.0.1 refused");
}

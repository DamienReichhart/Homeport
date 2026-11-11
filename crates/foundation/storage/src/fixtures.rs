//! Sample databases of released versions, for the upgrade tests.
//!
//! [`build`] makes a database as a released Homeport would have left it: that
//! release's own migrations, then rows of every kind it could store, written
//! in the shape it wrote them, with secrets sealed under [`KEY`]. The
//! `db_fixture` example saves one per release in `tests/fixtures/`, and the
//! tests open each with today's Homeport and expect every value back.
//!
//! Nothing here is a real credential: the key and every secret are the
//! constants below.

use std::path::Path;

use rusqlite::{params, Connection};

use crate::{crypto::Key, SecretBox};

/// The key every fixture's secrets are sealed under.
pub const KEY: Key = [0x42; 32];

pub const CREATED_AT: &str = "2026-01-05T08:00:00.000Z";
pub const UPDATED_AT: &str = "2026-02-06T09:30:00.000Z";

pub const PUBLIC_KEY: &str =
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAABAgMEBQYHCAkKCwwNDg8QERITFBUWFxgZGhscHR4f homeport-fixture";
pub const HOST_KEY: &str =
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAABAgMEBQYHCAkKCwwNDg8QERITFBUWFxgZGhscHR4f";
pub const REPOSITORY_HOST_KEY: &str =
    "github.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAABAgMEBQYHCAkKCwwNDg8QERITFBUWFxgZGhscHR4f";

pub const TLS_CA: &str = "-----BEGIN CERTIFICATE-----\nfixture-ca\n-----END CERTIFICATE-----\n";
pub const TLS_CERT: &str = "-----BEGIN CERTIFICATE-----\nfixture-cert\n-----END CERTIFICATE-----\n";
pub const TLS_KEY: &str = "-----BEGIN PRIVATE KEY-----\nfixture-key\n-----END PRIVATE KEY-----\n";
pub const SSH_PRIVATE_KEY: &str =
    "-----BEGIN OPENSSH PRIVATE KEY-----\nfixture-host-key\n-----END OPENSSH PRIVATE KEY-----\n";
pub const SSH_PASSPHRASE: &str = "fixture passphrase";
pub const BASTION_PRIVATE_KEY: &str =
    "-----BEGIN OPENSSH PRIVATE KEY-----\nfixture-bastion-key\n-----END OPENSSH PRIVATE KEY-----\n";

pub const COMPOSE: &str = "services:\n  web:\n    image: nginx:1.27\n";
pub const COMPOSE_PROD: &str = "services:\n  web:\n    restart: always\n";
pub const STACK_ENV: &str = "DB_PASSWORD=fixture-db-password\nTOKEN=fixture-token\n";
pub const DEPLOY_ENV: &str = "DB_PASSWORD=fixture-deployed-password\n";
pub const SYNCED_ENV: &str = "API_KEY=fixture-api-key\n";
pub const REGISTRY_SECRET: &str = "fixture-registry-token";
pub const REPOSITORY_PRIVATE_KEY: &str =
    "-----BEGIN OPENSSH PRIVATE KEY-----\nfixture-deploy-key\n-----END OPENSSH PRIVATE KEY-----\n";
pub const REPOSITORY_TOKEN: &str = "fixture-repository-token";
pub const CLUSTER_TOKEN: &str = "fixture-cluster-token";
pub const CLUSTER_CA: &str =
    "-----BEGIN CERTIFICATE-----\nfixture-cluster-ca\n-----END CERTIFICATE-----\n";
pub const NOMAD_TOKEN: &str = "fixture-nomad-token";

pub const DEPLOYED_COMMIT: &str = "1111111111111111111111111111111111111111";
pub const FETCHED_COMMIT: &str = "2222222222222222222222222222222222222222";
pub const POLL_ERROR: &str = r#"{"code":"timeout","key":"","params":{},"message":"The repository didn't answer in time.","hint":null,"detail":"fixture"}"#;

/// The schema versions at which a stored shape changed, by migration number.
pub mod since {
    pub const HOSTS: usize = 2;
    pub const STACKS: usize = 3;
    pub const REGISTRIES: usize = 4;
    pub const BASTIONS: usize = 5;
    pub const LINKED_STACKS: usize = 6;
    pub const STACK_FILES: usize = 7;
    pub const PROFILES: usize = 8;
    pub const DEPLOYS: usize = 9;
    pub const REPOSITORIES: usize = 10;
    pub const SYNCED_STACKS: usize = 11;
    pub const DEPLOY_COMMITS: usize = 12;
    pub const POLLING: usize = 13;
    pub const EVENTS: usize = 14;
    pub const SWARM_STACKS: usize = 15;
    pub const CLUSTERS: usize = 16;
    pub const ENGINES: usize = 17;
}

/// Whether `release` (`0.21.0`, `1.0.0-rc.1`) stores Nomad Clusters, which
/// came without a migration.
pub fn has_nomad(release: &str) -> bool {
    let mut parts = release
        .split(['.', '-'])
        .map(|part| part.parse::<u32>().unwrap_or(0));
    let (major, minor) = (parts.next().unwrap_or(0), parts.next().unwrap_or(0));
    (major, minor) >= (0, 21)
}

/// Makes the database of `release` at `path`: runs `migrations` (the SQL of
/// that release's migration files, in order) and fills every table they made.
pub fn build(path: &Path, migrations: &[String], release: &str) -> rusqlite::Result<()> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    for (index, sql) in migrations.iter().enumerate() {
        conn.execute_batch(sql)?;
        // As rusqlite_migration does after each one.
        conn.pragma_update(None, "user_version", index as i64 + 1)?;
    }
    conn.pragma_update(None, "foreign_keys", "ON")?;
    seed(&conn, migrations.len(), release)?;
    // Leave one file, as closing Homeport does.
    conn.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
    conn.close().map_err(|(_, e)| e)
}

fn seal(text: &str) -> Vec<u8> {
    SecretBox::from_key(&KEY)
        .seal(text.as_bytes())
        .expect("the fixture key seals")
}

fn seed(conn: &Connection, schema: usize, release: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES ('settings', ?1)",
        [r#"{"theme":"dark","lastHostId":"h-local"}"#],
    )?;
    if schema >= since::HOSTS {
        hosts(conn, schema)?;
    }
    if schema >= since::STACKS {
        stacks(conn, schema)?;
    }
    if schema >= since::REGISTRIES {
        conn.execute(
            "INSERT INTO registries (id, address, username, secret, created_at, updated_at)
             VALUES ('r-ghcr', 'ghcr.io', 'octo', ?1, ?2, ?3)",
            params![seal(REGISTRY_SECRET), CREATED_AT, UPDATED_AT],
        )?;
    }
    if schema >= since::LINKED_STACKS {
        conn.execute(
            "INSERT INTO linked_stacks
                 (id, host_id, name, working_dir, config_files, fingerprint, created_at, updated_at)
             VALUES ('l-blog', 'h-local', 'blog', '/srv/blog',
                 '[\"/srv/blog/compose.yaml\",\"/srv/blog/compose.override.yaml\"]', 'fp-1', ?1, ?2)",
            [CREATED_AT, UPDATED_AT],
        )?;
        if schema >= since::PROFILES {
            conn.execute(
                "UPDATE linked_stacks SET profiles = '[\"metrics\"]' WHERE id = 'l-blog'",
                [],
            )?;
        }
    }
    if schema >= since::DEPLOYS {
        let definition = serde_json::json!([{ "name": "compose.yaml", "content": COMPOSE }]);
        conn.execute(
            "INSERT INTO stack_deploys
                 (id, host_id, stack_id, stack_kind, stack_name, action, definition, env, profiles,
                  started_at, finished_at, success, exit_code)
             VALUES ('d-shop', 'h-local', 's-shop', 'managed', 'shop', 'up', ?1, ?2, '[\"debug\"]',
                  ?3, ?4, 1, 0)",
            params![
                definition.to_string(),
                seal(DEPLOY_ENV),
                CREATED_AT,
                UPDATED_AT
            ],
        )?;
    }
    if schema >= since::REPOSITORIES {
        conn.execute(
            "INSERT INTO git_repositories
                 (id, url, auth, private_key, public_key, host_key, created_at, updated_at)
             VALUES ('g-ssh', 'git@github.com:acme/infra.git', 'ssh_key', ?1, ?2, ?3, ?4, ?5)",
            params![
                seal(REPOSITORY_PRIVATE_KEY),
                PUBLIC_KEY,
                REPOSITORY_HOST_KEY,
                CREATED_AT,
                UPDATED_AT
            ],
        )?;
        conn.execute(
            "INSERT INTO git_repositories (id, url, auth, username, token, created_at, updated_at)
             VALUES ('g-https', 'https://gitlab.com/acme/site.git', 'https_token', 'octo', ?1, ?2, ?3)",
            params![seal(REPOSITORY_TOKEN), CREATED_AT, UPDATED_AT],
        )?;
    }
    if schema >= since::SYNCED_STACKS {
        synced(conn, schema)?;
    }
    if schema >= since::EVENTS {
        conn.execute_batch(
            "INSERT INTO engine_events (host_id, time_nano, kind, action, actor_id, attributes) VALUES
                 ('h-local', 1767600000000000001, 'container', 'start', 'abc123', '{\"name\":\"web\",\"image\":\"nginx:1.27\"}'),
                 ('h-local', 1767600000000000002, 'image', 'pull', 'nginx:1.27', '{}');",
        )?;
    }
    if schema >= since::SWARM_STACKS {
        conn.execute(
            "INSERT INTO stacks (id, swarm_id, name, env, profiles, registry_auth, created_at, updated_at)
             VALUES ('s-swarm', 'swarm-abc', 'edge', NULL, '[]', 1, ?1, ?2)",
            [CREATED_AT, UPDATED_AT],
        )?;
        conn.execute(
            "INSERT INTO stack_files (stack_id, position, name, content)
             VALUES ('s-swarm', 0, 'compose.yaml', ?1)",
            [COMPOSE],
        )?;
    }
    if schema >= since::CLUSTERS {
        clusters(conn, release)?;
    }
    Ok(())
}

fn hosts(conn: &Connection, schema: usize) -> rusqlite::Result<()> {
    let bastions = schema >= since::BASTIONS;
    let insert = "INSERT INTO hosts (id, name, connection, secrets, host_key, enabled, created_at)
                  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";
    let none: Option<Vec<u8>> = None;
    let no_key: Option<&str> = None;

    conn.execute(
        insert,
        params![
            "h-local",
            "laptop",
            r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#,
            none,
            no_key,
            true,
            CREATED_AT
        ],
    )?;
    conn.execute(
        insert,
        params![
            "h-tcp",
            "nas",
            r#"{"kind":"tcp","address":"192.168.1.20:2375"}"#,
            none,
            no_key,
            true,
            CREATED_AT
        ],
    )?;
    let tls = serde_json::json!({
        "kind": "tls", "ca_pem": TLS_CA, "cert_pem": TLS_CERT, "key_pem": TLS_KEY,
    });
    conn.execute(
        insert,
        params![
            "h-tls",
            "prod",
            r#"{"kind":"tls","address":"prod.example.com:2376"}"#,
            seal(&tls.to_string()),
            no_key,
            true,
            CREATED_AT
        ],
    )?;

    let mut ssh = serde_json::json!({
        "kind": "ssh", "host": "build.example.com", "port": 2222, "user": "deploy",
        "socketPath": "/var/run/docker.sock", "auth": "imported", "publicKey": PUBLIC_KEY,
    });
    let mut ssh_secrets = serde_json::json!({
        "kind": "ssh", "private_key": SSH_PRIVATE_KEY, "passphrase": SSH_PASSPHRASE,
    });
    let mut agent = serde_json::json!({
        "kind": "ssh", "host": "10.0.0.7", "port": 22, "user": "root",
        "socketPath": "/run/podman/podman.sock", "auth": "agent", "publicKey": null,
    });
    if bastions {
        ssh["bastions"] = serde_json::json!([{
            "host": "jump.example.com", "port": 22, "user": "jump", "auth": "imported",
            "publicKey": PUBLIC_KEY, "hostKey": HOST_KEY,
        }]);
        ssh_secrets["bastions"] = serde_json::json!({
            "jump.example.com:22": { "private_key": BASTION_PRIVATE_KEY, "passphrase": null },
        });
        agent["bastions"] = serde_json::json!([]);
    }
    conn.execute(
        insert,
        params![
            "h-ssh",
            "build",
            ssh.to_string(),
            seal(&ssh_secrets.to_string()),
            HOST_KEY,
            true,
            CREATED_AT
        ],
    )?;
    conn.execute(
        insert,
        params![
            "h-agent",
            "lab",
            agent.to_string(),
            none,
            no_key,
            false,
            CREATED_AT
        ],
    )?;

    if schema >= since::ENGINES {
        conn.execute_batch(&format!(
            "INSERT INTO hosts (id, name, connection, engine, created_at) VALUES
                 ('h-containerd', 'k3s-node', '{{\"kind\":\"local\",\"socketPath\":\"/run/containerd/containerd.sock\"}}', 'containerd', '{CREATED_AT}'),
                 ('h-incus', 'vms', '{{\"kind\":\"local\",\"socketPath\":\"/var/lib/incus/unix.socket\"}}', 'incus', '{CREATED_AT}');"
        ))?;
    }
    Ok(())
}

fn stacks(conn: &Connection, schema: usize) -> rusqlite::Result<()> {
    if schema < since::STACK_FILES {
        // One definition per Stack, in its own column.
        conn.execute(
            "INSERT INTO stacks (id, host_id, name, compose, env, created_at, updated_at)
             VALUES ('s-shop', 'h-local', 'shop', ?1, ?2, ?3, ?4)",
            params![COMPOSE, seal(STACK_ENV), CREATED_AT, UPDATED_AT],
        )?;
        conn.execute(
            "INSERT INTO stacks (id, host_id, name, compose, created_at, updated_at)
             VALUES ('s-plain', 'h-tcp', 'plain', ?1, ?2, ?3)",
            [COMPOSE, CREATED_AT, UPDATED_AT],
        )?;
        return Ok(());
    }
    conn.execute(
        "INSERT INTO stacks (id, host_id, name, env, created_at, updated_at)
         VALUES ('s-shop', 'h-local', 'shop', ?1, ?2, ?3)",
        params![seal(STACK_ENV), CREATED_AT, UPDATED_AT],
    )?;
    conn.execute(
        "INSERT INTO stacks (id, host_id, name, created_at, updated_at)
         VALUES ('s-plain', 'h-tcp', 'plain', ?1, ?2)",
        [CREATED_AT, UPDATED_AT],
    )?;
    conn.execute(
        "INSERT INTO stack_files (stack_id, position, name, content) VALUES
             ('s-shop', 0, 'compose.yaml', ?1),
             ('s-shop', 1, 'compose.prod.yaml', ?2),
             ('s-plain', 0, 'compose.yaml', ?1)",
        [COMPOSE, COMPOSE_PROD],
    )?;
    if schema >= since::PROFILES {
        conn.execute(
            "UPDATE stacks SET profiles = '[\"debug\"]' WHERE id = 's-shop'",
            [],
        )?;
    }
    Ok(())
}

fn synced(conn: &Connection, schema: usize) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO synced_stacks
             (id, host_id, name, repository_id, ref_kind, ref_name, compose_path, env, profiles,
              deployed_commit, fetched_commit, created_at, updated_at)
         VALUES ('y-site', 'h-local', 'site', 'g-ssh', 'branch', 'main', 'deploy/compose.yaml', ?1,
              '[\"cdn\"]', ?2, ?3, ?4, ?5)",
        params![
            seal(SYNCED_ENV),
            DEPLOYED_COMMIT,
            FETCHED_COMMIT,
            CREATED_AT,
            UPDATED_AT
        ],
    )?;
    conn.execute(
        "INSERT INTO synced_stacks
             (id, host_id, name, repository_id, ref_kind, ref_name, compose_path, created_at, updated_at)
         VALUES ('y-docs', 'h-tcp', 'docs', 'g-https', 'tag', 'v2', 'compose.yaml', ?1, ?2)",
        [CREATED_AT, UPDATED_AT],
    )?;
    let definition = serde_json::json!([{ "name": "deploy/compose.yaml", "content": COMPOSE }]);
    conn.execute(
        "INSERT INTO stack_deploys
             (id, host_id, stack_id, stack_kind, stack_name, action, definition, started_at)
         VALUES ('d-site', 'h-local', 'y-site', 'synced', 'site', 'update', ?1, ?2)",
        [definition.to_string().as_str(), UPDATED_AT],
    )?;
    if schema >= since::DEPLOY_COMMITS {
        conn.execute(
            "UPDATE stack_deploys SET commit_hash = ?1 WHERE id = 'd-site'",
            [DEPLOYED_COMMIT],
        )?;
    }
    if schema >= since::POLLING {
        conn.execute(
            "UPDATE synced_stacks SET poll = 0, auto_deploy = 1, last_polled_at = ?1,
                 last_poll_error = ?2, update_available = 1, auto_deploy_failed_commit = ?3,
                 auto_deploy_error = ?2
             WHERE id = 'y-site'",
            [UPDATED_AT, POLL_ERROR, FETCHED_COMMIT],
        )?;
        conn.execute(
            "UPDATE stack_deploys SET automatic = 1 WHERE id = 'd-site'",
            [],
        )?;
    }
    Ok(())
}

fn clusters(conn: &Connection, release: &str) -> rusqlite::Result<()> {
    let connection = serde_json::json!({
        "server": "https://127.0.0.1:6443", "tlsServerName": null,
        "insecureSkipTlsVerify": false, "proxyUrl": null, "namespace": "apps",
        "auth": { "method": "token" }, "source": { "context": "kind-dev", "pasted": false },
    });
    let secrets = serde_json::json!({ "certificateAuthority": CLUSTER_CA, "token": CLUSTER_TOKEN });
    conn.execute(
        "INSERT INTO clusters (id, name, kind, connection, secrets, enabled, created_at)
         VALUES ('c-kind', 'kind-dev', 'kubernetes', ?1, ?2, 1, ?3)",
        params![
            connection.to_string(),
            seal(&secrets.to_string()),
            CREATED_AT
        ],
    )?;
    conn.execute(
        "INSERT INTO clusters (id, name, connection, enabled, created_at)
         VALUES ('c-open', 'minikube', '{\"server\":\"https://192.168.49.2:8443\"}', 0, ?1)",
        [CREATED_AT],
    )?;
    if has_nomad(release) {
        let connection = serde_json::json!({
            "address": "https://nomad.example.com:4646", "region": "global",
            "tlsServerName": null, "aclToken": true, "customCa": false, "clientCertificate": false,
        });
        let secrets = serde_json::json!({ "token": NOMAD_TOKEN });
        conn.execute(
            "INSERT INTO clusters (id, name, kind, connection, secrets, enabled, created_at)
             VALUES ('c-nomad', 'batch', 'nomad', ?1, ?2, 1, ?3)",
            params![
                connection.to_string(),
                seal(&secrets.to_string()),
                CREATED_AT
            ],
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nomad_clusters_start_at_0_21() {
        assert!(!has_nomad("0.20.0"));
        assert!(has_nomad("0.21.0"));
        assert!(has_nomad("0.22.0"));
        assert!(has_nomad("1.0.0-rc.1"));
        assert!(has_nomad("1.0.0"));
    }
}

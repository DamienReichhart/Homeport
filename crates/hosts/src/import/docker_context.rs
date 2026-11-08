//! The Docker CLI's contexts as drafts of Hosts: each context's Engine
//! endpoint, and its TLS files when it has them. `~/.docker` is only read.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use homeport_core::{
    error::{AppError, AppResult},
    local_files,
};

use super::{free_name, ssh_config::DEFAULT_SOCKET, Env, ImportDraft};
use crate::{messages as msg, Host, HostConnection, SshAuth};

/// A context's `meta.json`, as far as Homeport reads it.
#[derive(Debug, Deserialize)]
struct Meta {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Endpoints", default)]
    endpoints: Endpoints,
}

#[derive(Debug, Default, Deserialize)]
struct Endpoints {
    docker: Option<Endpoint>,
}

#[derive(Debug, Deserialize)]
struct Endpoint {
    #[serde(rename = "Host")]
    host: Option<String>,
    #[serde(rename = "SkipTLSVerify", default)]
    skip_tls_verify: bool,
}

/// A context's mutual TLS material, read.
#[derive(Debug, Clone)]
pub(crate) struct TlsFiles {
    pub ca_pem: String,
    pub cert_pem: String,
    pub key_pem: String,
}

/// What importing one context creates.
#[derive(Debug)]
pub(crate) struct Plan {
    pub connection: HostConnection,
    pub tls: Option<TlsFiles>,
    pub files: Vec<PathBuf>,
}

/// One context, planned or refused.
pub(crate) struct Context {
    pub name: String,
    pub plan: AppResult<Plan>,
}

/// The CLI's config folder: `DOCKER_CONFIG`, or `~/.docker`.
fn docker_dir(env: &Env) -> PathBuf {
    env.docker_config
        .clone()
        .unwrap_or_else(|| env.home.join(".docker"))
}

/// Every context but the built-in `default`, sorted by name.
pub(crate) fn read(env: &Env) -> Vec<Context> {
    let root = docker_dir(env).join("contexts");
    let Ok(dirs) = std::fs::read_dir(root.join("meta")) else {
        return vec![];
    };
    let mut contexts: Vec<Context> = dirs
        .flatten()
        .filter_map(|dir| {
            let text = std::fs::read_to_string(dir.path().join("meta.json")).ok()?;
            let meta: Meta = serde_json::from_str(&text).ok()?;
            if meta.name == "default" {
                return None;
            }
            let tls_dir = root.join("tls").join(dir.file_name()).join("docker");
            Some(Context {
                plan: plan(&meta, &tls_dir, env),
                name: meta.name,
            })
        })
        .collect();
    contexts.sort_by_key(|c| c.name.to_lowercase());
    contexts
}

/// Each context as a draft. A context whose connection is already a Host
/// can't be imported again; its name is numbered when the name is taken.
pub(crate) fn drafts(contexts: Vec<Context>, hosts: &[Host]) -> Vec<ImportDraft> {
    let taken: Vec<String> = hosts.iter().map(|h| h.name.clone()).collect();
    contexts
        .into_iter()
        .map(|Context { name, plan }| {
            let plan = plan.and_then(|p| {
                match hosts
                    .iter()
                    .find(|h| same_engine(&h.connection, &p.connection))
                {
                    Some(existing) => Err(AppError::validation(
                        msg::IMPORT_ALREADY_A_HOST.with("name", &existing.name),
                    )),
                    None => Ok(p),
                }
            });
            match plan {
                Ok(plan) => ImportDraft {
                    name: free_name(&name, &taken),
                    source: name,
                    files: plan.files.iter().map(|f| f.display().to_string()).collect(),
                    connection: Some(plan.connection),
                    needs_passphrase: false,
                    problem: None,
                    warnings: vec![],
                    name_taken: false,
                },
                Err(problem) => ImportDraft {
                    name: name.clone(),
                    source: name,
                    connection: None,
                    files: vec![],
                    needs_passphrase: false,
                    problem: Some(problem),
                    warnings: vec![],
                    name_taken: false,
                },
            }
        })
        .collect()
}

fn plan(meta: &Meta, tls_dir: &Path, env: &Env) -> AppResult<Plan> {
    let unsupported = |endpoint: &str| {
        AppError::validation(msg::IMPORT_ENDPOINT_UNSUPPORTED.with("endpoint", endpoint))
    };
    let endpoint = meta
        .endpoints
        .docker
        .as_ref()
        .and_then(|e| e.host.as_deref())
        .ok_or_else(|| AppError::validation(msg::IMPORT_NO_DOCKER_ENDPOINT))?;
    let (scheme, rest) = endpoint
        .split_once("://")
        .ok_or_else(|| unsupported(endpoint))?;
    let simple = |connection| {
        Ok(Plan {
            connection,
            tls: None,
            files: vec![],
        })
    };
    match scheme {
        "unix" => simple(HostConnection::Local {
            socket_path: rest.to_owned(),
        }),
        // npipe:////./pipe/docker_engine names //./pipe/docker_engine.
        "npipe" => simple(HostConnection::Local {
            socket_path: rest.replace('\\', "/"),
        }),
        "tcp" => {
            let address = rest.trim_end_matches('/').to_owned();
            let names = ["ca.pem", "cert.pem", "key.pem"];
            let found: Vec<PathBuf> = names
                .iter()
                .map(|n| tls_dir.join(n))
                .filter(|p| p.is_file())
                .collect();
            match found.len() {
                0 => simple(HostConnection::Tcp { address }),
                3 if meta
                    .endpoints
                    .docker
                    .as_ref()
                    .is_some_and(|e| e.skip_tls_verify) =>
                {
                    Err(AppError::validation(msg::IMPORT_SKIP_TLS_VERIFY))
                }
                3 => {
                    let read = |p: &Path| local_files::read_text(p);
                    let tls = TlsFiles {
                        ca_pem: read(&found[0])?,
                        cert_pem: read(&found[1])?,
                        key_pem: read(&found[2])?,
                    };
                    Ok(Plan {
                        connection: HostConnection::Tls { address },
                        tls: Some(tls),
                        files: found,
                    })
                }
                _ => Err(AppError::validation(msg::IMPORT_TLS_INCOMPLETE)),
            }
        }
        "ssh" => {
            let rest = rest.trim_end_matches('/');
            let (user, host_port) = match rest.rsplit_once('@') {
                Some((u, h)) => (u.to_owned(), h),
                None => (env.local_user.clone(), rest),
            };
            let (host, port) = match host_port.rsplit_once(':') {
                Some((h, p)) if !h.contains(':') || h.ends_with(']') => (
                    h.trim_start_matches('[').trim_end_matches(']'),
                    p.parse().map_err(|_| unsupported(endpoint))?,
                ),
                _ => (host_port.trim_start_matches('[').trim_end_matches(']'), 22),
            };
            simple(HostConnection::Ssh {
                host: host.to_owned(),
                port,
                user,
                socket_path: DEFAULT_SOCKET.into(),
                auth: SshAuth::Agent,
                public_key: None,
                bastions: vec![],
            })
        }
        _ => Err(unsupported(endpoint)),
    }
}

/// Whether two connections reach the same Engine the same way.
fn same_engine(a: &HostConnection, b: &HostConnection) -> bool {
    match (a, b) {
        (HostConnection::Local { socket_path: x }, HostConnection::Local { socket_path: y }) => {
            x == y
        }
        (HostConnection::Tcp { address: x }, HostConnection::Tcp { address: y })
        | (HostConnection::Tls { address: x }, HostConnection::Tls { address: y }) => {
            x.eq_ignore_ascii_case(y)
        }
        (
            HostConnection::Ssh {
                host: h1,
                port: p1,
                user: u1,
                ..
            },
            HostConnection::Ssh {
                host: h2,
                port: p2,
                user: u2,
                ..
            },
        ) => h1.eq_ignore_ascii_case(h2) && p1 == p2 && u1 == u2,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Docker {
        dir: tempfile::TempDir,
        env: Env,
    }

    impl Docker {
        fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let env = Env {
                home: dir.path().join("home"),
                local_user: "local".into(),
                docker_config: Some(dir.path().join("docker")),
            };
            Self { dir, env }
        }

        /// Writes a context the way the CLI does, under the hash of its name
        /// (any folder name works for Homeport, which reads `Name`).
        fn context(&self, folder: &str, json: &str, tls: &[&str]) {
            let root = self.dir.path().join("docker/contexts");
            let meta = root.join("meta").join(folder);
            std::fs::create_dir_all(&meta).unwrap();
            std::fs::write(meta.join("meta.json"), json).unwrap();
            if !tls.is_empty() {
                let dir = root.join("tls").join(folder).join("docker");
                std::fs::create_dir_all(&dir).unwrap();
                for name in tls {
                    std::fs::write(dir.join(name), format!("PEM {name}")).unwrap();
                }
            }
        }
    }

    fn meta(name: &str, host: &str, skip: bool) -> String {
        format!(
            r#"{{"Name":"{name}","Metadata":{{}},"Endpoints":{{"docker":{{"Host":"{host}","SkipTLSVerify":{skip}}}}}}}"#
        )
    }

    fn connection(c: &Context) -> &HostConnection {
        &c.plan.as_ref().unwrap().connection
    }

    #[test]
    fn every_endpoint_kind_maps_to_a_host() {
        let d = Docker::new();
        d.context(
            "a1",
            &meta(
                "colima",
                "unix:///Users/me/.colima/default/docker.sock",
                false,
            ),
            &[],
        );
        d.context(
            "a2",
            &meta("desktop", "npipe:////./pipe/docker_engine", false),
            &[],
        );
        d.context("a3", &meta("lab", "tcp://10.0.0.2:2375", false), &[]);
        d.context(
            "a4",
            &meta("prod", "tcp://prod.example.com:2376", false),
            &["ca.pem", "cert.pem", "key.pem"],
        );
        d.context(
            "a5",
            &meta("edge", "ssh://deploy@edge.example.com:2222", false),
            &[],
        );
        d.context("a6", &meta("bare", "ssh://bare", false), &[]);
        let contexts = read(&d.env);
        let by = |name: &str| contexts.iter().find(|c| c.name == name).unwrap();

        assert_eq!(
            connection(by("colima")),
            &HostConnection::Local {
                socket_path: "/Users/me/.colima/default/docker.sock".into()
            }
        );
        assert_eq!(
            connection(by("desktop")),
            &HostConnection::Local {
                socket_path: "//./pipe/docker_engine".into()
            }
        );
        assert_eq!(
            connection(by("lab")),
            &HostConnection::Tcp {
                address: "10.0.0.2:2375".into()
            }
        );
        let prod = by("prod").plan.as_ref().unwrap();
        assert_eq!(
            prod.connection,
            HostConnection::Tls {
                address: "prod.example.com:2376".into()
            }
        );
        assert_eq!(prod.tls.as_ref().unwrap().key_pem, "PEM key.pem");
        assert_eq!(prod.files.len(), 3);
        assert!(matches!(
            connection(by("edge")),
            HostConnection::Ssh { host, port: 2222, user, auth: SshAuth::Agent, socket_path, .. }
                if host == "edge.example.com" && user == "deploy" && socket_path == DEFAULT_SOCKET
        ));
        assert!(matches!(
            connection(by("bare")),
            HostConnection::Ssh { port: 22, user, .. } if user == "local"
        ));
    }

    #[test]
    fn contexts_that_cant_be_imported_say_why() {
        let d = Docker::new();
        d.context(
            "b1",
            &meta("insecure", "tcp://h:2376", true),
            &["ca.pem", "cert.pem", "key.pem"],
        );
        d.context("b2", &meta("partial", "tcp://h:2376", false), &["ca.pem"]);
        d.context("b3", &meta("k8s", "kube://cluster", false), &[]);
        let contexts = read(&d.env);
        let problem = |name: &str| {
            contexts
                .iter()
                .find(|c| c.name == name)
                .unwrap()
                .plan
                .as_ref()
                .unwrap_err()
                .message
                .clone()
        };
        assert!(problem("insecure").contains("verif"));
        assert!(problem("partial").contains("three"));
        assert!(problem("k8s").contains("kube://cluster"));
    }

    #[test]
    fn the_default_context_and_broken_files_are_left_out() {
        let d = Docker::new();
        d.context(
            "c1",
            &meta("default", "unix:///var/run/docker.sock", false),
            &[],
        );
        d.context("c2", "not json", &[]);
        d.context("c3", &meta("Zed", "tcp://z:2375", false), &[]);
        d.context("c4", &meta("alpha", "tcp://a:2375", false), &[]);
        let names: Vec<_> = read(&d.env).into_iter().map(|c| c.name).collect();
        assert_eq!(names, ["alpha", "Zed"]);
    }

    #[test]
    fn no_docker_folder_means_no_contexts() {
        assert!(read(&Docker::new().env).is_empty());
    }

    #[test]
    fn known_engines_cant_be_imported_twice_and_names_are_numbered() {
        let d = Docker::new();
        d.context("d1", &meta("lab", "tcp://10.0.0.2:2375", false), &[]);
        d.context("d2", &meta("prod", "tcp://10.0.0.3:2375", false), &[]);
        let hosts = vec![
            Host {
                id: "1".into(),
                name: "office".into(),
                connection: HostConnection::Tcp {
                    address: "10.0.0.2:2375".into(),
                },
                engine: crate::EngineKind::Docker,
                enabled: true,
                host_key: None,
                created_at: String::new(),
            },
            Host {
                id: "2".into(),
                name: "prod".into(),
                connection: HostConnection::Local {
                    socket_path: "/x".into(),
                },
                engine: crate::EngineKind::Docker,
                enabled: true,
                host_key: None,
                created_at: String::new(),
            },
        ];
        let drafts = drafts(read(&d.env), &hosts);
        assert!(drafts[0]
            .problem
            .as_ref()
            .unwrap()
            .message
            .contains("office"));
        assert!(drafts[0].connection.is_none());
        assert_eq!(drafts[1].name, "prod (2)");
    }

    #[test]
    fn nothing_under_the_docker_folder_is_written() {
        let d = Docker::new();
        d.context(
            "e1",
            &meta("prod", "tcp://p:2376", false),
            &["ca.pem", "cert.pem", "key.pem"],
        );
        let before = snapshot(d.dir.path());
        let _ = drafts(read(&d.env), &[]);
        assert_eq!(snapshot(d.dir.path()), before);
    }

    fn snapshot(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
        let mut out = Vec::new();
        let mut stack = vec![dir.to_path_buf()];
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(d).unwrap().flatten() {
                if e.path().is_dir() {
                    stack.push(e.path());
                } else {
                    out.push((e.path(), std::fs::read(e.path()).unwrap()));
                }
            }
        }
        out.sort();
        out
    }
}

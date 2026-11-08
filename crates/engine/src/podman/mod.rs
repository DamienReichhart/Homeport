//! Podman's own API (libpod), on the same socket as its Docker-compatible
//! one: pods, containers created into them, and Quadlet units. Sent with
//! `homeport_transport::http` over the Host's dialer, since bollard only
//! speaks the Docker Engine API.

pub mod containers;
pub mod pods;

use std::sync::Arc;

use serde::de::DeserializeOwned;

use homeport_core::error::AppResult;
use homeport_transport::{
    http::{self, Request, Response},
    Dialer,
};

/// A connected Podman Engine: its dialer and the version it reported.
#[derive(Clone)]
pub struct Podman {
    dialer: Arc<dyn Dialer>,
    version: String,
}

impl Podman {
    pub fn new(dialer: Arc<dyn Dialer>, version: &str) -> Self {
        Self {
            dialer,
            version: version.to_owned(),
        }
    }

    /// The version of Podman, such as `5.8.1`.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Whether this Podman is `minimum` (`major.minor`) or later.
    pub fn at_least(&self, minimum: (u32, u32)) -> bool {
        version_at_least(&self.version, minimum)
    }

    /// `rest` under libpod's versioned prefix. Podman refuses an API version
    /// newer than its own, so ask for its major version's first.
    pub(crate) fn path(&self, rest: &str) -> String {
        let major = numbers(&self.version).first().copied().unwrap_or(4);
        format!("/v{major}.0.0/libpod{rest}")
    }

    pub(crate) async fn send(&self, request: Request) -> AppResult<Response> {
        http::send(&*self.dialer, request).await
    }

    pub(crate) async fn json<T: DeserializeOwned>(&self, request: Request) -> AppResult<T> {
        http::json(&*self.dialer, request).await
    }
}

/// The leading numbers of a version, so "5.8.1-dev" reads as [5, 8, 1].
fn numbers(version: &str) -> Vec<u32> {
    version
        .trim()
        .trim_start_matches('v')
        .split(['.', '-', '+'])
        .map_while(|part| part.parse().ok())
        .collect()
}

/// Whether `version` is `minimum` (`major.minor`) or later.
pub fn version_at_least(version: &str, (major, minor): (u32, u32)) -> bool {
    let n = numbers(version);
    let have = (
        n.first().copied().unwrap_or(0),
        n.get(1).copied().unwrap_or(0),
    );
    have >= (major, minor)
}

#[cfg(all(test, unix))]
pub(crate) mod testing {
    //! A fake Podman: a small HTTP server on a temporary socket that answers
    //! from a handler and records what it was asked.

    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use tokio::{
        io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
        net::UnixListener,
    };

    use homeport_transport::LocalDialer;

    use super::Podman;

    /// A request as the fake saw it.
    #[derive(Debug, Clone)]
    pub(crate) struct Seen {
        pub(crate) method: String,
        pub(crate) path: String,
        #[allow(dead_code)] // for tests that check headers
        pub(crate) headers: HashMap<String, String>,
        pub(crate) body: Vec<u8>,
    }

    impl Seen {
        pub(crate) fn json(&self) -> serde_json::Value {
            serde_json::from_slice(&self.body).unwrap()
        }
    }

    pub(crate) type Handler = Arc<dyn Fn(&Seen) -> (u16, String) + Send + Sync>;

    pub(crate) struct Fake {
        pub(crate) podman: Podman,
        pub(crate) seen: Arc<Mutex<Vec<Seen>>>,
        _dir: tempfile::TempDir,
    }

    impl Fake {
        pub(crate) fn requests(&self) -> Vec<Seen> {
            self.seen.lock().unwrap().clone()
        }
    }

    /// The Podman on this machine, for the tests marked "needs a local
    /// Podman Engine": at `$HOMEPORT_PODMAN_SOCKET`, else rootless then
    /// rootful Podman's socket. `None`, and the test skips, when none answers.
    pub(crate) async fn local() -> Option<(Podman, bollard::Docker)> {
        let mut sockets: Vec<String> = std::env::var("HOMEPORT_PODMAN_SOCKET")
            .into_iter()
            .collect();
        if let Ok(run) = std::env::var("XDG_RUNTIME_DIR") {
            sockets.push(format!("{run}/podman/podman.sock"));
        }
        sockets.push("/run/podman/podman.sock".into());
        for socket in sockets {
            let dialer = Arc::new(LocalDialer::new(&socket));
            let request = homeport_transport::http::Request::get("/version");
            let Ok(version) =
                homeport_transport::http::json::<serde_json::Value>(&*dialer, request).await
            else {
                continue;
            };
            let podman = version["Components"].as_array().and_then(|components| {
                components
                    .iter()
                    .find(|c| c["Name"] == "Podman Engine")
                    .and_then(|c| c["Version"].as_str())
                    .map(str::to_owned)
            });
            if let Some(podman) = podman {
                let docker = bollard::Docker::connect_with_socket(
                    &socket,
                    120,
                    bollard::API_DEFAULT_VERSION,
                )
                .ok()?;
                return Some((Podman::new(dialer, &podman), docker));
            }
        }
        eprintln!("no local Podman Engine answers; skipping");
        None
    }

    /// Serves `handler`'s answers (status and JSON body) as Podman `version`.
    pub(crate) fn podman(
        version: &str,
        handler: impl Fn(&Seen) -> (u16, String) + Send + Sync + 'static,
    ) -> Fake {
        let handler: Handler = Arc::new(handler);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("podman.sock");
        let listener = UnixListener::bind(&path).unwrap();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let recorded = seen.clone();
        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let handler = handler.clone();
                let recorded = recorded.clone();
                tokio::spawn(async move {
                    let (read, mut write) = stream.into_split();
                    let mut reader = BufReader::new(read);
                    let mut first = String::new();
                    if reader.read_line(&mut first).await.unwrap_or(0) == 0 {
                        return;
                    }
                    let mut parts = first.split_whitespace();
                    let method = parts.next().unwrap_or_default().to_owned();
                    let path = parts.next().unwrap_or_default().to_owned();
                    let mut headers = HashMap::new();
                    loop {
                        let mut line = String::new();
                        reader.read_line(&mut line).await.unwrap();
                        let line = line.trim_end();
                        if line.is_empty() {
                            break;
                        }
                        if let Some((k, v)) = line.split_once(':') {
                            headers.insert(k.trim().to_lowercase(), v.trim().to_owned());
                        }
                    }
                    let length = headers
                        .get("content-length")
                        .and_then(|l| l.parse().ok())
                        .unwrap_or(0);
                    let mut body = vec![0; length];
                    reader.read_exact(&mut body).await.unwrap();
                    let seen = Seen {
                        method,
                        path,
                        headers,
                        body,
                    };
                    let (status, reply) = handler(&seen);
                    recorded.lock().unwrap().push(seen);
                    let raw = format!(
                        "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{reply}",
                        reply.len()
                    );
                    write.write_all(raw.as_bytes()).await.ok();
                });
            }
        });
        let dialer = Arc::new(LocalDialer::new(path.to_str().unwrap()));
        Fake {
            podman: Podman::new(dialer, version),
            seen,
            _dir: dir,
        }
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn versions_compare_by_major_and_minor() {
        assert!(version_at_least("5.8.1", (5, 8)));
        assert!(version_at_least("5.10.0", (5, 8)));
        assert!(version_at_least("6.0.0-dev", (5, 7)));
        assert!(!version_at_least("5.7.2", (5, 8)));
        assert!(!version_at_least("4.9.4", (5, 7)));
        assert!(!version_at_least("", (5, 0)));
    }

    #[tokio::test]
    async fn libpod_paths_ask_for_the_podmans_major_version() {
        let fake = testing::podman("5.8.1", |_| (200, "{}".into()));
        assert_eq!(fake.podman.path("/pods/json"), "/v5.0.0/libpod/pods/json");
        let old = testing::podman("4.9.4", |_| (200, "{}".into()));
        assert_eq!(old.podman.path("/pods/json"), "/v4.0.0/libpod/pods/json");
        assert!(fake.podman.at_least((5, 8)));
        assert!(!old.podman.at_least((5, 0)));
    }
}

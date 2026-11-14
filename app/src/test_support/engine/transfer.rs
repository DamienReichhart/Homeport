//! What the fake Engine answers for a container's files: a path's stat,
//! its download and an upload (`archive`), and its export, each of which
//! may be held, stalled or cut as a test asks.

use std::sync::atomic::Ordering;

use base64::Engine as _;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::{
    wire::{archive_of, entry_names, query_value, stat_of, Held, Stall},
    Connection, Open, Request, Stream,
};

/// How many bytes an export sends.
pub const EXPORTED: usize = 3 * 1024 * 1024;

/// A tar block.
const BLOCK: usize = 512;

impl Connection {
    /// Waits while transfers are held; `false` when the client let go.
    async fn held(&mut self) -> bool {
        let mut closed = [0_u8; 64];
        while *self.held.borrow() {
            tokio::select! {
                _ = self.held.changed() => {}
                read = self.socket.read(&mut closed) => {
                    if !matches!(read, Ok(read) if read > 0) {
                        self.shared.abandoned.fetch_add(1, Ordering::SeqCst);
                        return false;
                    }
                }
            }
        }
        true
    }

    pub(super) async fn archive(&mut self, request: &Request) -> bool {
        let path = query_value(&request.query, "path").unwrap_or_default();
        let _open = Open::new(&self.shared, Stream::Transfer);
        if request.method == "PUT" {
            if !self.held().await {
                return false;
            }
            let names = entry_names(&request.body);
            let refused = self.shared.refused.lock().unwrap().clone();
            if refused.is_some_and(|name| names.contains(&name)) {
                let refusal = r#"{"message":"no space left on device"}"#;
                return self.json("500 Internal Server Error", refusal).await;
            }
            self.shared.uploaded.lock().unwrap().push((path, names));
            return self.answer("200 OK", &[], b"").await;
        }
        let held = self.shared.files.lock().unwrap().get(&path).cloned();
        let Some(held) = held else {
            let missing = r#"{"message":"no such file or directory"}"#;
            return self.json("404 Not Found", missing).await;
        };
        let name = path.rsplit('/').next().unwrap_or_default().to_owned();
        let stat = base64::engine::general_purpose::STANDARD.encode(stat_of(&name, &held));
        let headers = [
            ("X-Docker-Container-Path-Stat", stat),
            ("Content-Type", "application/x-tar".to_owned()),
        ];
        if request.method == "HEAD" {
            return self.answer("200 OK", &headers, b"").await;
        }
        let stall = *self.shared.stall.lock().unwrap();
        if stall.is_none() && !self.held().await {
            return false;
        }
        let archive = archive_of(&name, &held);
        let cut = self.shared.cut.load(Ordering::SeqCst);
        let data_end = BLOCK
            + if let Held::File(bytes) = &held {
                bytes.len()
            } else {
                0
            };
        let pause = match stall {
            Some(Stall::BeforeLastByte) => Some(data_end - 1),
            Some(Stall::AfterData) => Some(data_end),
            None => cut.then_some(archive.len() / 2),
        };
        if let Some(at) = pause {
            // Says the whole length, sends up to `at`, then waits (or,
            // cut, hangs up).
            let head = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/x-tar\r\n\r\n",
                archive.len()
            );
            let _ = self.socket.write_all(head.as_bytes()).await;
            let _ = self.socket.write_all(&archive[..at]).await;
            // Cut now, or once released if told to cut meanwhile.
            if cut || !self.held().await || self.shared.cut.load(Ordering::SeqCst) {
                let _ = self.socket.shutdown().await;
                return false;
            }
            return self.socket.write_all(&archive[at..]).await.is_ok();
        }
        self.answer("200 OK", &headers, &archive).await
    }

    /// A container's filesystem, as `docker export` streams it: three
    /// megabytes, so the export reports its progress on the way.
    pub(super) async fn export(&mut self) -> bool {
        let _open = Open::new(&self.shared, Stream::Transfer);
        if !self.held().await {
            return false;
        }
        let body = vec![b'x'; EXPORTED];
        let tar = [("Content-Type", "application/x-tar".to_owned())];
        self.answer("200 OK", &tar, &body).await
    }
}

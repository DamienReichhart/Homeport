//! What the fake Engine answers for a terminal in a container: the exec
//! is created, started on a connection the client takes over (a TTY: what
//! is typed is echoed back, as a shell's line discipline would), resized,
//! and inspected for its exit code once it ended.

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::broadcast,
};

use super::{Connection, Open, Push, Request, Stream};

/// The exec's id.
const EXEC: &str = r#"{"Id":"exec-1"}"#;

impl Connection {
    /// `POST /containers/{id}/exec`: what it was asked to run is kept.
    pub(super) async fn create_exec(&mut self, request: &Request) -> bool {
        let body = String::from_utf8_lossy(&request.body).into_owned();
        self.shared.execs.lock().unwrap().push(body);
        self.json("201 Created", EXEC).await
    }

    /// What its inspect says once it ended: the code a test asked for.
    pub(super) async fn inspect_exec(&mut self) -> bool {
        let code = *self.shared.exit_code.lock().unwrap();
        let inspected = format!(r#"{{"ID":"exec-1","Running":false,"ExitCode":{code}}}"#);
        self.json("200 OK", &inspected).await
    }

    /// `POST /exec/{id}/resize?h=…&w=…`: noted among what was typed.
    pub(super) async fn resize_exec(&mut self, request: &Request) -> bool {
        let size = format!("resize {}", request.query);
        self.shared.terminal.lock().unwrap().push(size);
        self.answer("201 Created", &[], b"").await
    }

    /// `POST /exec/{id}/start`, attached: the connection becomes the
    /// terminal's, until the client lets go of it or the test ends it.
    pub(super) async fn start_exec(&mut self) {
        let _open = Open::new(&self.shared, Stream::Exec);
        let mut pushes = self.pushes.subscribe();
        let head = "HTTP/1.1 101 UPGRADED\r\nContent-Type: application/vnd.docker.raw-stream\r\n\
                    Connection: Upgrade\r\nUpgrade: tcp\r\n\r\n";
        if self.socket.write_all(head.as_bytes()).await.is_err() {
            return;
        }
        let mut typed = [0_u8; 4096];
        loop {
            let output = tokio::select! {
                read = self.socket.read(&mut typed) => match read {
                    Ok(read) if read > 0 => {
                        let bytes = typed[..read].to_vec();
                        self.shared.typed.lock().unwrap().extend_from_slice(&bytes);
                        let input = format!("input {}", String::from_utf8_lossy(&bytes));
                        self.shared.terminal.lock().unwrap().push(input);
                        bytes
                    }
                    _ => return,
                },
                push = pushes.recv() => match push {
                    Ok(Push::Print(bytes)) => bytes,
                    Ok(Push::Exit) | Err(broadcast::error::RecvError::Closed) => {
                        let _ = self.socket.shutdown().await;
                        return;
                    }
                    _ => continue,
                },
            };
            if self.socket.write_all(&output).await.is_err() {
                return;
            }
        }
    }
}

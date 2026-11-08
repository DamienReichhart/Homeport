//! A connection to this machine's containerd for the tests marked
//! `#[ignore = "needs a local containerd"]`, and `ctr` to set things up.

use std::{process::Command, sync::Arc};

use homeport_core::host::EngineKind;
use homeport_transport::LocalDialer;

use crate::Containerd;

/// The namespace the tests work in, kept between runs so its image is
/// pulled once.
pub(crate) const NAMESPACE: &str = "homeport-test";

pub(crate) fn socket() -> String {
    std::env::var("HOMEPORT_CONTAINERD_SOCKET")
        .unwrap_or_else(|_| "/run/containerd/containerd.sock".into())
}

/// A small image the tests run, from a registry without Docker Hub's limits.
pub(crate) fn image() -> String {
    std::env::var("HOMEPORT_CONTAINERD_TEST_IMAGE")
        .unwrap_or_else(|_| "mirror.gcr.io/library/alpine:3.22".into())
}

pub(crate) async fn client() -> Containerd {
    let dialer = Arc::new(LocalDialer::for_engine(&socket(), EngineKind::Containerd));
    Containerd::connect("local", dialer)
        .await
        .expect("a local containerd answers")
}

/// Runs `ctr` in the tests' namespace; panics with its output on failure.
pub(crate) fn ctr(args: &[&str]) -> String {
    let output = Command::new("ctr")
        .args(["--address", &socket(), "--namespace", NAMESPACE])
        .args(args)
        .output()
        .expect("ctr is installed with containerd");
    assert!(
        output.status.success(),
        "ctr {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Pulls the tests' image once.
pub(crate) fn pulled() -> String {
    let image = image();
    if !ctr(&["images", "ls", "-q"]).lines().any(|l| l == image) {
        ctr(&["images", "pull", &image]);
    }
    image
}

/// A container id no other test uses.
pub(crate) fn unique(prefix: &str) -> String {
    format!(
        "{prefix}-{}",
        &uuid::Uuid::new_v4().simple().to_string()[..8]
    )
}

/// Removes a test's container, its task and snapshot when dropped, even
/// after a failed assertion.
pub(crate) struct Cleanup(pub(crate) String);

impl Drop for Cleanup {
    fn drop(&mut self) {
        let run = |args: &[&str]| {
            Command::new("ctr")
                .args(["--address", &socket(), "--namespace", NAMESPACE])
                .args(args)
                .output()
                .ok();
        };
        run(&["task", "kill", "--signal", "SIGKILL", &self.0]);
        std::thread::sleep(std::time::Duration::from_millis(200));
        run(&["task", "rm", "--force", &self.0]);
        run(&["containers", "rm", &self.0]);
        run(&["snapshots", "rm", &self.0]);
    }
}

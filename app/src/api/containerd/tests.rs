use homeport_core::error::ErrorCode;
use homeport_registries::messages as msg;

use super::*;
use crate::app::testing;

/// A Host the app has never heard of.
const NO_HOST: &str = "no-such-host";

#[track_caller]
fn not_connected<T>(result: AppResult<T>) {
    assert_eq!(
        result.err().map(|e| e.code),
        Some(ErrorCode::HostNotConnected)
    );
}

fn host() -> String {
    NO_HOST.to_owned()
}

fn ns() -> String {
    "default".to_owned()
}

fn id() -> String {
    "web".to_owned()
}

#[tokio::test]
async fn namespaces_need_a_connected_host() {
    let app = testing::app();
    let state = &app.state;
    not_connected(containerd_namespaces(state, host()).await);
    not_connected(containerd_namespace_create(state, host(), ns()).await);
    not_connected(containerd_namespace_remove(state, host(), ns()).await);
}

#[tokio::test]
async fn containers_need_a_connected_host() {
    let app = testing::app();
    let state = &app.state;
    not_connected(containerd_containers(state, host(), ns()).await);
    not_connected(containerd_container_inspect(state, host(), ns(), id()).await);
    for action in [
        TaskAction::Start,
        TaskAction::Stop,
        TaskAction::Pause,
        TaskAction::Resume,
    ] {
        not_connected(containerd_container_act(state, host(), ns(), id(), action).await);
    }
    not_connected(containerd_container_kill(state, host(), ns(), id(), "SIGTERM".into()).await);
    not_connected(containerd_container_remove(state, host(), ns(), id(), true, false).await);
}

#[tokio::test]
async fn logs_need_a_connected_host_and_start_no_stream() {
    let app = testing::app();
    let state = &app.state;
    not_connected(containerd_logs_source(state, host(), ns(), id()).await);
    let options = LogOptions {
        tail: Some(100),
        timestamps: false,
        since: None,
        stdout: true,
        stderr: true,
        follow: true,
    };
    let subscribed = containerd_logs_subscribe(state, host(), ns(), id(), options, |_| {
        panic!("no log may come from an unknown Host")
    })
    .await;
    not_connected(subscribed);
}

#[tokio::test]
async fn a_terminal_needs_a_connected_host() {
    let app = testing::app();
    let options = ContainerdExecOptions::default();
    let started = containerd_exec_start(&app.state, host(), ns(), id(), options, |_| {
        panic!("no output may come from an unknown Host")
    })
    .await;
    not_connected(started);
}

#[tokio::test]
async fn a_terminal_that_ended_refuses_input_and_closes_quietly() {
    let app = testing::app();
    let state = &app.state;
    let typed = containerd_exec_input(state, 7, "ls\n".into()).await;
    assert_eq!(typed.unwrap_err().code, ErrorCode::NotFound);
    let resized = containerd_exec_resize(state, 7, 120, 40).await;
    assert_eq!(resized.unwrap_err().code, ErrorCode::NotFound);
    assert!(containerd_exec_close(state, 7).await.is_ok());
}

#[tokio::test]
async fn images_need_a_connected_host() {
    let app = testing::app();
    let state = &app.state;
    not_connected(containerd_images(state, host(), ns()).await);
    not_connected(containerd_image_remove(state, host(), ns(), "alpine".into()).await);
    let pulled = containerd_images_pull_subscribe(state, host(), ns(), "alpine".into(), |_| {
        panic!("no progress may come from an unknown Host")
    })
    .await;
    not_connected(pulled);
}

#[tokio::test]
async fn snapshots_need_a_connected_host() {
    let app = testing::app();
    let state = &app.state;
    not_connected(containerd_snapshots(state, host(), ns(), None).await);
    let snapshotter = "overlayfs".to_owned();
    let key = "sha256:abc".to_owned();
    not_connected(containerd_snapshot_remove(state, host(), ns(), snapshotter, key).await);
}

#[tokio::test]
async fn a_refused_pull_points_at_the_missing_login() {
    let app = testing::app();
    let explain = explainer(&app.state, "ghcr.io/acme/private:latest");

    let refused = AppError::new(ErrorCode::RegistryAuthRequired, msg::LOGIN_FAILED);
    let explained = explain(refused.clone());
    assert_eq!(explained.code, ErrorCode::RegistryAuthRequired);
    assert_ne!(explained.message, refused.message);
    assert!(explained.hint.is_some());

    // Anything the registry didn't refuse is left as it is.
    let other = AppError::new(ErrorCode::NotFound, msg::HTTPS_CLIENT);
    assert_eq!(explain(other.clone()).message, other.message);
}

use homeport_clusters::{registry::ClusterStatus, ClusterKind};
use homeport_core::error::ErrorCode;
use homeport_nomad::jobs::run::SpecFormat;

use super::super::tests::{failed, gone, kubernetes, status_event, unknown};
use super::*;
use crate::app::testing::app;

/// A Nomad agent answering its status endpoints, an empty list to every
/// other request, and 403 to an ACL token other than `token`.
async fn fake_nomad(token: &'static str) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut buf = [0; 8192];
                while let Ok(n) = socket.read(&mut buf).await {
                    if n == 0 {
                        return;
                    }
                    let head = String::from_utf8_lossy(&buf[..n]).to_lowercase();
                    let path = head.split(' ').nth(1).unwrap_or_default().to_owned();
                    let authorised = head.contains(&format!("x-nomad-token: {token}"));
                    let (status, body) = if path.starts_with("/v1/status/leader") {
                        ("200 OK", r#""10.0.0.5:4647""#)
                    } else if !authorised {
                        ("403 Forbidden", "ACL token not found")
                    } else if path.starts_with("/v1/agent/self") {
                        (
                            "200 OK",
                            r#"{"config":{"Region":"global","Version":{"Version":"1.10.3"}}}"#,
                        )
                    } else {
                        ("200 OK", "[]")
                    };
                    let reply = format!(
                        "HTTP/1.1 {status}\r\ncontent-length: {}\r\n\r\n{body}",
                        body.len()
                    );
                    if socket.write_all(reply.as_bytes()).await.is_err() {
                        return;
                    }
                }
            });
        }
    });
    address
}

fn form(name: &str, address: &str, token: Option<&str>) -> NomadForm {
    NomadForm {
        name: name.into(),
        address: address.into(),
        token: token.map(str::to_owned),
        ..Default::default()
    }
}

fn log_options() -> LogOptions {
    LogOptions {
        tail: Some(10),
        timestamps: false,
        since: None,
        stdout: true,
        stderr: true,
        follow: false,
    }
}

fn exec_options() -> NomadExecOptions {
    NomadExecOptions {
        task: "web".into(),
        command: None,
        cols: 80,
        rows: 24,
    }
}

#[tokio::test]
async fn a_nomad_cluster_is_tested_added_edited_and_read() {
    let mut app = app();
    let address = fake_nomad("good").await;

    // A wrong token is told before anything is saved.
    let wrong = clusters_nomad_test(&app.state, form("dev", &address, Some("bad")), None).await;
    assert!(failed(wrong, ErrorCode::NomadTokenInvalid));
    let agent = clusters_nomad_test(&app.state, form("dev", &address, Some("good")), None)
        .await
        .unwrap();
    assert_eq!(agent.version.as_deref(), Some("1.10.3"));
    assert!(super::super::clusters_list(&app.state).unwrap().is_empty());

    let added = clusters_nomad_add(&app.state, form("dev", &address, Some("good")))
        .await
        .unwrap();
    let id = added.cluster.id.clone();
    assert_eq!(added.cluster.kind, ClusterKind::Nomad);
    let connected = status_event(&mut app, &id, ClusterStatus::is_connected).await;
    assert_eq!(
        connected,
        ClusterStatus::Connected {
            server_version: "1.10.3".into(),
            platform: None,
            region: Some("global".into()),
        }
    );

    // An edit that leaves the token out keeps it, and reconnects.
    let edited = clusters_nomad_update(&app.state, id.clone(), form("prod", &address, None))
        .await
        .unwrap();
    assert_eq!(edited.cluster.name, "prod");
    status_event(&mut app, &id, ClusterStatus::is_connected).await;
    let kept = clusters_nomad_test(&app.state, form("prod", &address, None), Some(id.clone()));
    assert!(kept.await.is_ok());

    let state = &app.state;
    assert!(clusters_nomad_jobs(state, id.clone(), None)
        .await
        .unwrap()
        .is_empty());
    assert!(clusters_nomad_nodes(state, id.clone())
        .await
        .unwrap()
        .is_empty());
    let watch = clusters_nomad_watch_subscribe(state, id.clone(), NomadWatch::Nodes, |_| true)
        .await
        .unwrap();
    state.streams.cancel(watch);
    // What runs on Kubernetes can't be asked of it.
    assert!(failed(
        super::super::on(state, &id).map(drop),
        ErrorCode::Validation
    ));
}

#[tokio::test]
async fn a_form_that_cannot_be_saved_is_refused() {
    let app = app();
    let state = &app.state;
    let nameless = form(" ", "nomad.example.com", None);
    assert!(clusters_nomad_add(state, nameless).await.is_err());
    assert!(clusters_nomad_add(state, form("dev", " ", None))
        .await
        .is_err());
    assert!(gone(
        clusters_nomad_update(state, unknown(), form("dev", "10.0.0.5", None)).await
    ));
    assert!(gone(
        clusters_nomad_test(state, form("dev", "10.0.0.5", None), Some(unknown())).await
    ));
    // A Kubernetes Cluster has no Nomad form to edit, nor Nomad jobs.
    let kube = kubernetes(&app, "kube").await.cluster.id;
    let edit = clusters_nomad_update(state, kube.clone(), form("dev", "10.0.0.5", None)).await;
    assert!(failed(edit, ErrorCode::Validation));
    assert!(failed(
        clusters_nomad_jobs(state, kube, None).await,
        ErrorCode::Validation
    ));
}

#[tokio::test]
async fn an_unknown_cluster_has_no_job_to_read_or_run() {
    let app = app();
    let s = &app.state;
    let job = || "web".to_owned();
    assert!(gone(clusters_nomad_jobs(s, unknown(), None).await));
    assert!(gone(clusters_nomad_job(s, unknown(), None, job()).await));
    assert!(gone(
        clusters_nomad_job_stop(s, unknown(), None, job(), false).await
    ));
    assert!(gone(
        clusters_nomad_job_scale(s, unknown(), None, job(), "group".into(), 2).await
    ));
    assert!(gone(
        clusters_nomad_job_plan(s, unknown(), None, "job \"web\" {}".into()).await
    ));
    let plan = NomadPlanToRun {
        job_id: job(),
        namespace: "default".into(),
        format: SpecFormat::Json,
        job: serde_json::json!({}),
        job_modify_index: 0,
    };
    assert!(gone(clusters_nomad_job_run(s, unknown(), plan, None).await));
    assert!(gone(
        clusters_nomad_job_source(s, unknown(), None, job()).await
    ));
    assert!(gone(clusters_nomad_namespaces(s, unknown()).await));
    let topic = NomadWatch::Jobs { namespace: None };
    let watch = clusters_nomad_watch_subscribe(s, unknown(), topic, |_| {
        panic!("an unknown Cluster's jobs changed")
    });
    assert!(gone(watch.await));
}

#[tokio::test]
async fn an_unknown_cluster_has_no_allocation_or_node() {
    let app = app();
    let s = &app.state;
    let (alloc, node) = (|| "alloc-1".to_owned(), || "node-1".to_owned());
    assert!(gone(clusters_nomad_allocation(s, unknown(), alloc()).await));
    let logs =
        clusters_nomad_logs_subscribe(s, unknown(), alloc(), "web".into(), log_options(), |_| {
            panic!("an unknown Cluster's task logged")
        });
    assert!(gone(logs.await));
    let exec = clusters_nomad_exec_start(s, unknown(), alloc(), exec_options(), |_| {
        panic!("an unknown Cluster's terminal wrote")
    });
    assert!(gone(exec.await));
    assert!(gone(clusters_nomad_nodes(s, unknown()).await));
    assert!(gone(clusters_nomad_node(s, unknown(), node()).await));
    let drain = NomadDrainSpec {
        deadline_secs: Some(60),
        ignore_system_jobs: false,
    };
    assert!(gone(
        clusters_nomad_node_drain(s, unknown(), node(), Some(drain), false).await
    ));
    assert!(gone(
        clusters_nomad_node_eligibility(s, unknown(), node(), true).await
    ));
}

#[tokio::test]
async fn a_terminal_that_ended_takes_no_input() {
    let app = app();
    let state = &app.state;
    assert!(clusters_nomad_exec_input(state, 3, "ls\n".into())
        .await
        .is_err());
    assert!(clusters_nomad_exec_resize(state, 3, 80, 24).await.is_err());
    // Closing it again does nothing.
    clusters_nomad_exec_close(state, 3).await.unwrap();
}

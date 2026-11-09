//! Against a real Nomad agent: `nomad agent -dev`, reached at `NOMAD_ADDR`
//! (http://127.0.0.1:4646 by default) with `NOMAD_TOKEN` when its ACLs are
//! on. Run with `cargo test -p homeport-nomad -- --ignored nomad_live`, as
//! CI's nomad-tests job does.

use homeport_nomad::{normalize_address, Nomad, NomadConfig};

fn dev_agent() -> Nomad {
    let address = std::env::var("NOMAD_ADDR").unwrap_or_else(|_| "http://127.0.0.1:4646".into());
    Nomad::new(&NomadConfig {
        name: "nomad-dev".into(),
        address: normalize_address(&address, false).unwrap(),
        token: std::env::var("NOMAD_TOKEN").ok(),
        ..Default::default()
    })
    .unwrap()
}

#[tokio::test]
#[ignore = "needs a local Nomad agent"]
async fn nomad_live_a_dev_agent_reports_its_version_and_region() {
    let agent = dev_agent().agent().await.unwrap();
    assert!(agent.version.is_some_and(|v| !v.is_empty()));
    assert_eq!(agent.region.as_deref(), Some("global"));
    assert!(!agent.leader.is_empty());
}

/// A small service job on the dev agent's docker driver.
fn web_job(name: &str, message: &str) -> String {
    format!(
        r#"job "{name}" {{
  datacenters = ["*"]
  group "web" {{
    count = 1
    scaling {{
      min = 1
      max = 3
    }}
    task "server" {{
      driver = "docker"
      config {{
        image   = "busybox:1.37"
        command = "sh"
        args    = ["-c", "echo {message}; sleep 3600"]
      }}
      resources {{
        cpu    = 50
        memory = 32
      }}
    }}
  }}
}}
"#
    )
}

/// Waits until `check` holds, for up to two minutes (the image is pulled first).
async fn until<F, Fut>(what: &str, mut check: F)
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    for _ in 0..240 {
        if check().await {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    panic!("timed out waiting for {what}");
}

/// Runs `spec` through Nomad's parse, plan and run, as the app does.
async fn run(nomad: &Nomad, spec: &str) -> homeport_nomad::run::NomadPlan {
    let (job, format) = nomad.parse_job(spec).await.unwrap();
    let plan = nomad.plan_job(job, format, None).await.unwrap();
    nomad
        .run_job(
            &homeport_nomad::run::NomadPlanToRun {
                job_id: plan.job_id.clone(),
                namespace: plan.namespace.clone(),
                format: plan.format,
                job: plan.job.clone(),
                job_modify_index: plan.job_modify_index,
            },
            Some(spec),
        )
        .await
        .unwrap();
    plan
}

#[tokio::test]
#[ignore = "needs a local Nomad agent"]
async fn nomad_live_a_job_runs_is_rerun_changed_scaled_and_stopped() {
    let nomad = dev_agent();
    let name = "homeport-live-web";
    let first = run(&nomad, &web_job(name, "first")).await;
    assert!(first.new_job);
    until("the job to run", || async {
        nomad
            .job(None, name)
            .await
            .is_ok_and(|d| d.job.groups[0].running == 1)
    })
    .await;
    assert!(nomad.jobs(None).await.unwrap().iter().any(|j| j.id == name));

    // Its source is kept by Nomad, and a changed one plans as an edit.
    let source = nomad.job_source(None, name).await.unwrap();
    assert!(source.submitted && source.source.contains("first"));
    let second = run(&nomad, &web_job(name, "second")).await;
    assert_eq!(second.change, "Edited");
    assert!(second.groups[0]
        .fields
        .iter()
        .any(|f| f.path.contains("args")));

    // A plan that someone else's run made stale isn't run.
    let (job, format) = nomad.parse_job(&web_job(name, "third")).await.unwrap();
    let stale = nomad.plan_job(job, format, None).await.unwrap();
    run(&nomad, &web_job(name, "fourth")).await;
    let e = nomad
        .run_job(
            &homeport_nomad::run::NomadPlanToRun {
                job_id: stale.job_id.clone(),
                namespace: stale.namespace.clone(),
                format: stale.format,
                job: stale.job.clone(),
                job_modify_index: stale.job_modify_index,
            },
            None,
        )
        .await
        .unwrap_err();
    assert!(
        e.is(homeport_core::error::ErrorCode::NomadJobModified),
        "{e:?}"
    );

    nomad.scale_task_group(None, name, "web", 2).await.unwrap();
    until("two allocations", || async {
        nomad
            .job(None, name)
            .await
            .is_ok_and(|d| d.groups[0].counts.desired == Some(2))
    })
    .await;
    let beyond = nomad
        .scale_task_group(None, name, "web", 4)
        .await
        .unwrap_err();
    assert!(beyond.is(homeport_core::error::ErrorCode::NomadScaleOutOfBounds));

    nomad.stop_job(None, name, true).await.unwrap();
    until("the job to be purged", || async {
        nomad.job(None, name).await.is_err()
    })
    .await;
}

#[tokio::test]
#[ignore = "needs a local Nomad agent"]
async fn nomad_live_a_tasks_log_is_read_and_a_command_runs_through_exec() {
    use std::sync::{Arc, Mutex};

    use homeport_core::{logs::LogEvent, logs::LogOptions, terminal::ExecEvent};
    use homeport_nomad::exec::{pump, NomadExecOptions, NomadTerminal};

    let nomad = dev_agent();
    let name = "homeport-live-logs";
    run(&nomad, &web_job(name, "hello-from-homeport")).await;
    until("a running allocation", || async {
        nomad
            .job(None, name)
            .await
            .is_ok_and(|d| d.allocations.iter().any(|a| a.client_status == "running"))
    })
    .await;
    let alloc = nomad
        .job(None, name)
        .await
        .unwrap()
        .allocations
        .into_iter()
        .find(|a| a.client_status == "running")
        .unwrap()
        .id;

    let lines = Arc::new(Mutex::new(vec![]));
    let seen = lines.clone();
    nomad
        .stream_logs(
            alloc.clone(),
            "server".into(),
            LogOptions {
                tail: None,
                timestamps: false,
                since: None,
                stdout: true,
                stderr: false,
                follow: false,
            },
            move |e| {
                if let LogEvent::Lines { lines } = e {
                    seen.lock()
                        .unwrap()
                        .extend(lines.into_iter().map(|l| l.text));
                }
                true
            },
        )
        .await;
    assert!(lines
        .lock()
        .unwrap()
        .iter()
        .any(|l| l == "hello-from-homeport"));

    let options = NomadExecOptions {
        task: "server".into(),
        command: Some("sh -c 'echo exec-from-homeport'".into()),
        cols: 80,
        rows: 24,
    };
    let (terminal, output) = NomadTerminal::open(&nomad, &alloc, &options).await.unwrap();
    let events = Arc::new(Mutex::new(vec![]));
    let seen = events.clone();
    tokio::time::timeout(
        std::time::Duration::from_secs(30),
        pump(
            nomad.clone(),
            alloc.clone(),
            "server".into(),
            terminal,
            output,
            move |e| {
                seen.lock().unwrap().push(e);
                true
            },
        ),
    )
    .await
    .unwrap();
    let events = events.lock().unwrap().clone();
    assert!(events
        .iter()
        .any(|e| matches!(e, ExecEvent::Output { data } if data.contains("exec-from-homeport"))));
    assert!(matches!(
        events.last(),
        Some(ExecEvent::Exit { code: Some(0), .. })
    ));

    nomad.stop_job(None, name, true).await.unwrap();
}

#[tokio::test]
#[ignore = "needs a local Nomad agent"]
async fn nomad_live_a_nodes_eligibility_is_switched_both_ways() {
    let nomad = dev_agent();
    let node = nomad.nodes().await.unwrap().remove(0);
    assert!(node.eligible);
    nomad.set_eligibility(&node.id, false).await.unwrap();
    assert!(!nomad.node(&node.id).await.unwrap().node.eligible);
    nomad.set_eligibility(&node.id, true).await.unwrap();
    assert!(nomad.nodes().await.unwrap()[0].eligible);
}

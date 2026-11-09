use super::*;
use crate::api::containerd::services::containers::v1::container::Runtime;

fn container(id: &str, labels: &[(&str, &str)]) -> Container {
    Container {
        id: id.into(),
        labels: labels
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        image: "docker.io/library/alpine:3.22".into(),
        runtime: Some(Runtime {
            name: "io.containerd.runc.v2".into(),
            options: None,
        }),
        snapshotter: "overlayfs".into(),
        snapshot_key: id.into(),
        created_at: Some(prost_types::Timestamp {
            seconds: 1_794_162_522,
            nanos: 0,
        }),
        ..Default::default()
    }
}

fn process(status: Status, pid: u32, exit_status: u32) -> Process {
    Process {
        container_id: "web".into(),
        pid,
        status: status.into(),
        exit_status,
        ..Default::default()
    }
}

#[test]
fn containers_join_their_task() {
    let running = summary(
        "default",
        &container("web", &[]),
        Some(&process(Status::Running, 42, 0)),
    );
    assert_eq!(running.state, TaskState::Running);
    assert_eq!(running.pid, Some(42));
    assert_eq!(running.exit_status, None);
    assert_eq!(running.runtime, "io.containerd.runc.v2");
    assert_eq!(running.created_at.as_deref(), Some("2026-11-08T18:28:42Z"));

    let stopped = summary(
        "default",
        &container("web", &[]),
        Some(&process(Status::Stopped, 42, 137)),
    );
    assert_eq!(stopped.state, TaskState::Stopped);
    assert_eq!(stopped.pid, None);
    assert_eq!(stopped.exit_status, Some(137));

    let none = summary("default", &container("web", &[]), None);
    assert_eq!(none.state, TaskState::None);
    assert_eq!(none.created_by, CreatedBy::Other);
    assert_eq!(none.name, "web");
}

#[test]
fn the_tool_that_made_a_container_is_told_from_its_labels() {
    let nerdctl = summary(
        "default",
        &container("3f2a", &[(NERDCTL_NAME, "web")]),
        None,
    );
    assert_eq!(nerdctl.created_by, CreatedBy::Nerdctl);
    assert_eq!(nerdctl.name, "web");

    let pod = summary(
        "k8s.io",
        &container(
            "9c1d",
            &[
                (KUBERNETES_POD, "coredns-7db6"),
                (KUBERNETES_POD_NAMESPACE, "kube-system"),
                (KUBERNETES_CONTAINER, "coredns"),
                (CRI_KIND, "container"),
            ],
        ),
        None,
    );
    assert_eq!(pod.created_by, CreatedBy::Kubernetes);
    assert_eq!(pod.name, "coredns-7db6/coredns");
    assert_eq!(pod.pod.as_deref(), Some("kube-system/coredns-7db6"));
    assert!(!pod.sandbox);

    let pause = summary(
        "k8s.io",
        &container(
            "a1",
            &[(KUBERNETES_POD, "coredns-7db6"), (CRI_KIND, "sandbox")],
        ),
        None,
    );
    assert!(pause.sandbox);

    let docker = summary("moby", &container("b2", &[]), None);
    assert_eq!(docker.created_by, CreatedBy::Docker);
}

mod live {
    use super::super::*;
    use crate::testing::{self, NAMESPACE};

    async fn state(client: &Containerd, id: &str) -> TaskState {
        inspect(client, NAMESPACE, id)
            .await
            .unwrap()
            .container
            .state
    }

    #[tokio::test]
    #[ignore = "needs a local containerd"]
    async fn containerd_live_tasks_start_stop_pause_and_remove() {
        let client = testing::client().await;
        let image = testing::pulled();
        let id = testing::unique("homeport-test-task");
        let _cleanup = testing::Cleanup(id.clone());
        // A process that ignores SIGTERM, so stopping ends with SIGKILL.
        testing::ctr(&[
            "run",
            "-d",
            &image,
            &id,
            "sh",
            "-c",
            "trap '' TERM; sleep 600",
        ]);

        let listed = list(&client, NAMESPACE).await.unwrap();
        let found = listed.iter().find(|c| c.id == id).expect("listed");
        assert_eq!(found.state, TaskState::Running);
        assert!(found.pid.is_some());
        assert_eq!(found.image, image);

        stop(&client, NAMESPACE, &id, Duration::from_millis(500))
            .await
            .unwrap();
        let stopped = inspect(&client, NAMESPACE, &id).await.unwrap().container;
        assert_eq!(stopped.state, TaskState::Stopped);
        assert_eq!(stopped.exit_status, Some(137));
        assert!(testing::ctr(&["task", "ls"]).contains(&id));

        act(&client, NAMESPACE, &id, TaskAction::Start)
            .await
            .unwrap();
        assert_eq!(state(&client, &id).await, TaskState::Running);
        act(&client, NAMESPACE, &id, TaskAction::Pause)
            .await
            .unwrap();
        assert_eq!(state(&client, &id).await, TaskState::Paused);
        act(&client, NAMESPACE, &id, TaskAction::Resume)
            .await
            .unwrap();
        assert_eq!(state(&client, &id).await, TaskState::Running);

        let refused = remove(&client, NAMESPACE, &id, false, false)
            .await
            .unwrap_err();
        assert_eq!(refused.code, ErrorCode::ContainerRunning);
        remove(&client, NAMESPACE, &id, true, false).await.unwrap();
        assert!(!testing::ctr(&["containers", "ls", "-q"]).contains(&id));
        assert!(
            !testing::ctr(&["snapshots", "ls"]).contains(&id),
            "the snapshot went too"
        );
        assert!(inspect(&client, NAMESPACE, &id)
            .await
            .unwrap_err()
            .is(ErrorCode::NotFound));
    }

    #[tokio::test]
    #[ignore = "needs a local containerd"]
    async fn containerd_live_removal_can_keep_the_snapshot() {
        let client = testing::client().await;
        let image = testing::pulled();
        let id = testing::unique("homeport-test-keep");
        let _cleanup = testing::Cleanup(id.clone());
        testing::ctr(&["containers", "create", &image, &id, "true"]);
        assert_eq!(state(&client, &id).await, TaskState::None);
        remove(&client, NAMESPACE, &id, false, true).await.unwrap();
        assert!(testing::ctr(&["snapshots", "ls"]).contains(&id));
        // Marked so containerd's garbage collector keeps it, now that no
        // container references it.
        let info = testing::ctr(&["snapshots", "info", &id]);
        assert!(info.contains(GC_ROOT), "{info}");
        testing::ctr(&["snapshots", "rm", &id]);
    }
}

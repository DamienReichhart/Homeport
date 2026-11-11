use std::collections::HashMap;

use super::*;

fn container(id: &str, labels: &[(&str, &str)]) -> Container {
    Container {
        id: id.into(),
        labels: labels
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        ..Default::default()
    }
}

#[test]
fn nerdctl_containers_log_beside_their_state() {
    let c = container(
        "3f2a",
        &[(
            NERDCTL_STATE_DIR,
            "/var/lib/nerdctl/1935db59/containers/default/3f2a",
        )],
    );
    assert_eq!(
        source(&c),
        ContainerdLogSource::File {
            path: "/var/lib/nerdctl/1935db59/containers/default/3f2a/3f2a-json.log".into(),
            format: ContainerdLogFormat::Json
        }
    );
    let journald = container(
        "3f2a",
        &[
            (
                NERDCTL_STATE_DIR,
                "/var/lib/nerdctl/x/containers/default/3f2a",
            ),
            (NERDCTL_LOG_CONFIG, r#"{"driver":"journald","opts":{}}"#),
        ],
    );
    assert_eq!(
        source(&journald),
        ContainerdLogSource::Driver {
            driver: "journald".into()
        }
    );
}

#[test]
fn kubernetes_containers_log_where_cri_says() {
    let mut c = container("9c1d", &[]);
    c.extensions = HashMap::from([(
        CRI_METADATA.to_owned(),
        prost_types::Any {
            type_url: "github.com/containerd/cri/pkg/store/container/Metadata".into(),
            value: br#"{"Version":"v1","Metadata":{"ID":"9c1d","Name":"coredns","LogPath":"/var/log/pods/kube-system_coredns-7db6_1a2b/coredns/0.log"}}"#.to_vec(),
        },
    )]);
    assert_eq!(
        source(&c),
        ContainerdLogSource::File {
            path: "/var/log/pods/kube-system_coredns-7db6_1a2b/coredns/0.log".into(),
            format: ContainerdLogFormat::Cri
        }
    );
    let labelled = container(
        "9c1d",
        &[
            (KUBERNETES_POD_NAMESPACE, "kube-system"),
            (KUBERNETES_POD, "coredns-7db6"),
            (KUBERNETES_POD_UID, "1a2b"),
            (KUBERNETES_CONTAINER, "coredns"),
        ],
    );
    assert_eq!(
        source(&labelled),
        ContainerdLogSource::Folder {
            path: "/var/log/pods/kube-system_coredns-7db6_1a2b/coredns".into()
        }
    );
}

#[test]
fn other_containers_keep_no_logs_and_say_why() {
    let c = container("t1", &[]);
    assert_eq!(source(&c), ContainerdLogSource::None);
    let options = LogOptions {
        tail: Some(10),
        timestamps: false,
        since: None,
        stdout: true,
        stderr: true,
        follow: true,
    };
    assert!(command(&ContainerdLogSource::None, &options).is_none());
    let e = unavailable(&ContainerdLogSource::None);
    assert_eq!(e.code, ErrorCode::ContainerdNoLogs);
    assert!(e.hint.unwrap().text.contains("ctr"));
}

#[test]
fn lines_of_both_formats_are_read() {
    let json = parse(
        ContainerdLogFormat::Json,
        r#"{"log":"hello\n","stream":"stderr","time":"2026-11-08T19:46:48.731552514Z"}"#,
    )
    .unwrap();
    assert_eq!(
        (json.stream.as_str(), json.text.as_str()),
        ("stderr", "hello")
    );
    assert_eq!(seconds(&json.time), Some(1_794_167_208));

    let full = parse(
        ContainerdLogFormat::Cri,
        "2026-11-08T19:46:48.7Z stdout F ready to serve",
    )
    .unwrap();
    assert_eq!(full.text, "ready to serve");
    assert!(!full.partial);
    let part = parse(
        ContainerdLogFormat::Cri,
        "2026-11-08T19:46:48.7Z stdout P long ",
    )
    .unwrap();
    assert!(part.partial);
    assert!(parse(ContainerdLogFormat::Json, "not json").is_none());
}

#[test]
fn files_are_followed_with_tail() {
    let source = ContainerdLogSource::File {
        path: "/var/log/it's.log".into(),
        format: ContainerdLogFormat::Cri,
    };
    let mut options = LogOptions {
        tail: Some(200),
        timestamps: false,
        since: None,
        stdout: true,
        stderr: true,
        follow: true,
    };
    assert_eq!(
        command(&source, &options).unwrap(),
        r"exec tail -n 200 -F -- '/var/log/it'\''s.log'"
    );
    options.tail = None;
    options.follow = false;
    assert_eq!(
        command(&source, &options).unwrap(),
        r"exec tail -n +1 -- '/var/log/it'\''s.log'"
    );
}

mod live {
    use super::super::*;
    use crate::testing::{self, NAMESPACE};

    #[tokio::test]
    #[ignore = "needs a local containerd"]
    async fn containerd_live_logs_explain_a_ctr_container_and_follow_a_nerdctl_one() {
        let client = testing::client().await;
        let image = testing::pulled();

        // ctr keeps no output.
        let quiet = testing::unique("homeport-test-logs");
        testing::ctr(&["containers", "create", &image, &quiet, "true"]);
        assert_eq!(
            locate(&client, NAMESPACE, &quiet).await.unwrap(),
            ContainerdLogSource::None
        );
        testing::ctr(&["containers", "rm", &quiet]);

        // A container labelled as nerdctl labels its own, with its log file.
        let state = tempfile::tempdir().unwrap();
        let id = testing::unique("homeport-test-nerdctl");
        let label = format!("nerdctl/state-dir={}", state.path().display());
        testing::ctr(&[
            "containers",
            "create",
            "--label",
            &label,
            &image,
            &id,
            "true",
        ]);
        std::fs::write(
            state.path().join(format!("{id}-json.log")),
            concat!(
                r#"{"log":"one\n","stream":"stdout","time":"2026-11-08T10:00:00Z"}"#,
                "\n",
                r#"{"log":"two\n","stream":"stderr","time":"2026-11-08T10:00:01Z"}"#,
                "\n"
            ),
        )
        .unwrap();
        let lines = std::sync::Mutex::new(Vec::new());
        let ended = std::sync::Mutex::new(false);
        let options = LogOptions {
            tail: None,
            timestamps: false,
            since: None,
            stdout: true,
            stderr: true,
            follow: false,
        };
        stream(client, NAMESPACE.into(), id.clone(), options, |event| {
            match event {
                LogEvent::Lines { lines: batch } => lines.lock().unwrap().extend(batch),
                LogEvent::End => *ended.lock().unwrap() = true,
                LogEvent::Error { error } => panic!("{error:?}"),
            }
            true
        })
        .await;
        testing::ctr(&["containers", "rm", &id]);
        let lines = lines.into_inner().unwrap();
        let text: Vec<(&str, &str)> = lines
            .iter()
            .map(|l| (l.stream.as_str(), l.text.as_str()))
            .collect();
        assert_eq!(text, [("stdout", "one"), ("stderr", "two")]);
        assert!(ended.into_inner().unwrap());
    }
}

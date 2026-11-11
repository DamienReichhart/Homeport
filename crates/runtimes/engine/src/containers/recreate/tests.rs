use std::sync::Mutex;

use super::*;

#[allow(unreachable_pub)]
mod sample {
    homeport_core::messages! {
        PORT_IN_USE = "errors.test.portInUse" => "Port 8080 is already in use.";
    }
}

/// Records the steps and fails the one named in `fail`.
struct Fake {
    log: Mutex<Vec<String>>,
    fail: Vec<&'static str>,
}

impl Fake {
    fn failing(fail: &[&'static str]) -> Self {
        Self {
            log: Mutex::new(vec![]),
            fail: fail.to_vec(),
        }
    }

    fn step(&self, step: String) -> AppResult<()> {
        let failed = self.fail.iter().any(|f| step.starts_with(f));
        self.log.lock().unwrap().push(step);
        if failed {
            Err(AppError::new(ErrorCode::PortInUse, sample::PORT_IN_USE))
        } else {
            Ok(())
        }
    }

    fn log(&self) -> Vec<String> {
        self.log.lock().unwrap().clone()
    }
}

impl Steps for Fake {
    async fn create(&self, name: &str) -> AppResult<String> {
        self.step(format!("create {name}"))
            .map(|_| "new".to_owned())
    }
    async fn stop(&self, id: &str) -> AppResult<()> {
        self.step(format!("stop {id}"))
    }
    async fn start(&self, id: &str) -> AppResult<()> {
        self.step(format!("start {id}"))
    }
    async fn rename(&self, id: &str, name: &str) -> AppResult<()> {
        self.step(format!("rename {id} {name}"))
    }
    async fn remove(&self, id: &str) -> AppResult<()> {
        self.step(format!("remove {id}"))
    }
}

const RUNNING: Old = Old {
    id: "old",
    name: "web",
    running: true,
};

#[tokio::test]
async fn a_recreate_swaps_the_containers_and_removes_the_original_last() {
    let fake = Fake::failing(&[]);
    assert_eq!(sequence(&fake, &RUNNING, "x").await.unwrap(), "new");
    assert_eq!(
        fake.log(),
        [
            "create web-new-x",
            "stop old",
            "rename old web-old-x",
            "rename new web",
            "start new",
            "remove old",
        ]
    );
}

#[tokio::test]
async fn a_stopped_container_stays_stopped() {
    let fake = Fake::failing(&[]);
    let stopped = Old {
        running: false,
        ..RUNNING
    };
    sequence(&fake, &stopped, "x").await.unwrap();
    assert!(
        !fake.log().iter().any(|s| s.starts_with("st")),
        "{:?}",
        fake.log()
    );
}

#[tokio::test]
async fn a_failed_create_changes_nothing() {
    let fake = Fake::failing(&["create"]);
    let e = sequence(&fake, &RUNNING, "x").await.unwrap_err();
    assert_eq!(e.code, ErrorCode::PortInUse);
    assert_eq!(fake.log(), ["create web-new-x"]);
}

#[tokio::test]
async fn a_failed_start_puts_the_original_back_running() {
    let fake = Fake::failing(&["start new"]);
    let e = sequence(&fake, &RUNNING, "x").await.unwrap_err();
    assert_eq!(e.code, ErrorCode::PortInUse, "the original failure");
    assert_eq!(
        fake.log(),
        [
            "create web-new-x",
            "stop old",
            "rename old web-old-x",
            "rename new web",
            "start new",
            "remove new",
            "rename old web",
            "start old",
        ]
    );
}

#[tokio::test]
async fn a_failed_rename_aside_restarts_the_original() {
    let fake = Fake::failing(&["rename old web-old"]);
    sequence(&fake, &RUNNING, "x").await.unwrap_err();
    let log = fake.log();
    assert_eq!(&log[log.len() - 2..], ["remove new", "start old"]);
}

#[tokio::test]
async fn a_failed_rollback_names_the_leftover_container() {
    let fake = Fake::failing(&["start"]);
    let e = sequence(&fake, &RUNNING, "x").await.unwrap_err();
    assert_eq!(e.code, ErrorCode::RecreateRolledBack);
    assert!(e.hint.unwrap().text.contains("web-old-x, stopped"));
    assert!(e
        .detail
        .unwrap()
        .starts_with("Port 8080 is already in use."));
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn editing_keeps_the_name_volume_and_networks() {
    use bollard::models::NetworkCreateRequest;

    use crate::testing;

    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let name = testing::unique("edit");
    let network = format!("{name}-net");
    docker
        .create_network(NetworkCreateRequest {
            name: network.clone(),
            ..Default::default()
        })
        .await
        .unwrap();
    let original = run::run(
        &docker,
        RunSpec {
            image: testing::IMAGE.into(),
            name: Some(name.clone()),
            command: Some("sleep 120".into()),
            env: vec!["A=1".into()],
            mounts: vec![run::MountSpec {
                source: format!("{name}-data"),
                target: "/data".into(),
                read_only: false,
            }],
            ..Default::default()
        },
        None,
    )
    .await
    .unwrap();
    docker
        .connect_network(
            &network,
            bollard::models::NetworkConnectRequest {
                container: original.clone(),
                endpoint_config: Some(bollard::models::EndpointSettings {
                    aliases: Some(vec!["db".into()]),
                    ..Default::default()
                }),
            },
        )
        .await
        .unwrap();

    let mut spec = run::spec_for(&docker, &original).await.unwrap();
    spec.env = vec!["A=2".into()];
    let edited = recreate(&docker, &original, spec, None).await.unwrap();

    let inspected = containers::inspect(&docker, &edited).await.unwrap();
    assert_eq!(inspected["Name"], format!("/{name}"));
    assert_eq!(inspected["State"]["Running"], true);
    let env = inspected["Config"]["Env"].as_array().unwrap();
    assert!(env.iter().any(|e| e == "A=2"), "{env:?}");
    assert_eq!(inspected["Mounts"][0]["Name"], format!("{name}-data"));
    let aliases = &inspected["NetworkSettings"]["Networks"][&network]["Aliases"];
    assert!(
        aliases.as_array().unwrap().iter().any(|a| a == "db"),
        "{aliases}"
    );
    assert!(
        docker.inspect_container(&original, None).await.is_err(),
        "the original is gone"
    );

    containers::remove(&docker, &edited, true, true)
        .await
        .unwrap();
    docker.remove_network(&network).await.unwrap();
    docker
        .remove_volume(
            &format!("{name}-data"),
            None::<bollard::query_parameters::RemoveVolumeOptions>,
        )
        .await
        .unwrap();
}

#[test]
fn losses_name_anonymous_volumes_changes_downtime_and_the_stack() {
    let original: bollard::models::ContainerInspectResponse =
        serde_json::from_value(serde_json::json!({
            "State": { "Running": true },
            "Config": { "Labels": { "com.docker.compose.project": "shop" } },
            "HostConfig": { "Binds": ["site:/srv"] },
            "Mounts": [
                { "Type": "volume", "Name": "site", "Destination": "/srv" },
                { "Type": "volume", "Name": "a".repeat(64), "Destination": "/var/cache" }
            ]
        }))
        .unwrap();
    assert_eq!(
        what_is_lost(&original, 3),
        [
            Loss::AnonymousVolume {
                path: "/var/cache".into()
            },
            Loss::FilesystemChanges { count: 3 },
            Loss::Downtime,
            Loss::StackRedeploy {
                stack: "shop".into()
            },
        ]
    );
    assert!(what_is_lost(&Default::default(), 0).is_empty());
}

use super::*;
use crate::testing;

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn runs_a_container_from_an_image() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let name = testing::unique("run");
    let id = run(
        &docker,
        RunSpec {
            image: testing::IMAGE.into(),
            name: Some(name.clone()),
            command: Some("sleep 30".into()),
            env: vec!["A=1".into()],
            cpus: Some(0.5),
            memory: Some(512 * 1024 * 1024),
            labels: vec!["org.example.team=web".into(), "tier=front".into()],
            ..Default::default()
        },
        None,
    )
    .await
    .unwrap();
    let listed = containers::list(&docker).await.unwrap();
    let c = listed.iter().find(|c| c.id == id).unwrap();
    assert_eq!(
        (c.name.as_str(), c.state.as_str()),
        (name.as_str(), "running")
    );
    let inspected = containers::inspect(&docker, &id).await.unwrap();
    assert_eq!(inspected["HostConfig"]["NanoCpus"], 500_000_000);
    assert_eq!(inspected["HostConfig"]["Memory"], 512 * 1024 * 1024);
    assert_eq!(inspected["Config"]["Labels"]["tier"], "front");
    containers::remove(&docker, &id, true, true).await.unwrap();
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn runs_with_dropped_capabilities_and_a_device() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let id = run(
        &docker,
        RunSpec {
            image: testing::IMAGE.into(),
            name: Some(testing::unique("run-security")),
            command: Some("sleep 30".into()),
            cap_drop: vec!["ALL".into()],
            devices: vec![DeviceSpec {
                host_path: "/dev/null".into(),
                container_path: Some("/dev/homeport-null".into()),
                permissions: "rw".into(),
            }],
            ..Default::default()
        },
        None,
    )
    .await
    .unwrap();
    let inspected = containers::inspect(&docker, &id).await.unwrap();
    assert_eq!(inspected["HostConfig"]["CapDrop"][0], "ALL");
    assert_eq!(
        inspected["HostConfig"]["Devices"][0]["PathInContainer"],
        "/dev/homeport-null"
    );
    containers::remove(&docker, &id, true, true).await.unwrap();
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn runs_with_a_healthcheck_and_an_extra_host() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let id = run(
        &docker,
        RunSpec {
            image: testing::IMAGE.into(),
            name: Some(testing::unique("run-health")),
            command: Some("sleep 60".into()),
            healthcheck: Some(HealthcheckSpec {
                command: "true".into(),
                interval_s: Some(1),
                ..Default::default()
            }),
            hostname: Some("homeport-test".into()),
            extra_hosts: vec!["nas.home:192.168.1.10".into()],
            ..Default::default()
        },
        None,
    )
    .await
    .unwrap();
    let mut status = String::new();
    for _ in 0..30 {
        let inspected = containers::inspect(&docker, &id).await.unwrap();
        status = inspected["State"]["Health"]["Status"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        if status == "healthy" {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    assert_eq!(status, "healthy");
    let dir = tempfile::tempdir().unwrap();
    let copy = dir.path().join("hosts");
    crate::containers::files::download(&docker, &id, "/etc/hosts", &copy)
        .await
        .unwrap();
    let hosts = std::fs::read_to_string(&copy).unwrap();
    assert!(hosts.contains("192.168.1.10\tnas.home"), "{hosts}");
    let inspected = containers::inspect(&docker, &id).await.unwrap();
    assert_eq!(inspected["Config"]["Hostname"], "homeport-test");
    containers::remove(&docker, &id, true, true).await.unwrap();
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn duplicates_a_container() {
    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let name = testing::unique("dup");
    let original = run(
        &docker,
        RunSpec {
            image: testing::IMAGE.into(),
            name: Some(name.clone()),
            command: Some("sleep 60".into()),
            env: vec!["A=1".into()],
            labels: vec!["team=web".into()],
            memory: Some(64 * 1024 * 1024),
            extra_hosts: vec!["nas:192.168.1.10".into()],
            ..Default::default()
        },
        None,
    )
    .await
    .unwrap();
    let mut spec = spec_for(&docker, &original).await.unwrap();
    assert_eq!(spec.based_on.as_deref(), Some(original.as_str()));
    spec.name = Some(format!("{name}-copy"));
    let copy = run(&docker, spec, None).await.unwrap();

    let a = containers::inspect(&docker, &original).await.unwrap();
    let b = containers::inspect(&docker, &copy).await.unwrap();
    for pointer in [
        "/Config/Env",
        "/Config/Cmd",
        "/Config/Labels",
        "/HostConfig/Memory",
        "/HostConfig/ExtraHosts",
        "/HostConfig/RestartPolicy",
    ] {
        assert_eq!(a.pointer(pointer), b.pointer(pointer), "{pointer}");
    }
    assert_ne!(a["Config"]["Hostname"], b["Config"]["Hostname"]);
    assert_eq!(b["Name"], format!("/{name}-copy"));
    containers::remove(&docker, &copy, true, true)
        .await
        .unwrap();
    containers::remove(&docker, &original, true, true)
        .await
        .unwrap();
}

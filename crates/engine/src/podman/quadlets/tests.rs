//! Quadlet units against a fake Podman, with JSON as Podman 5.8 writes it.

use std::collections::HashMap;

use super::*;
use crate::podman::testing;

/// Trimmed from Podman 5.8's `GET /libpod/quadlets/json`, rootless.
const LISTED: &str = r#"[
  {
    "Name": "web.container",
    "UnitName": "web.service",
    "Path": "/home/me/.config/containers/systemd/web.container",
    "Status": "active/running",
    "App": "",
    "Pod": "shop.pod"
  },
  {
    "Name": "cache.volume",
    "UnitName": "",
    "Path": "/home/me/.config/containers/systemd/cache.volume",
    "Status": "Not loaded",
    "App": "shop"
  }
]"#;

const INFO_ROOTLESS: &str = r#"{"host":{"security":{"rootless":true}}}"#;

fn container(id: &str, unit: Option<&str>, state: &str) -> Container {
    let mut labels = HashMap::new();
    if let Some(unit) = unit {
        labels.insert(UNIT_LABEL.to_owned(), unit.to_owned());
    }
    Container {
        id: id.into(),
        name: format!("{id}-name"),
        image: "img".into(),
        image_id: "sha".into(),
        command: String::new(),
        created: 0,
        state: state.into(),
        status: String::new(),
        health: None,
        ports: vec![],
        labels,
        stack: None,
        service: None,
        networks: vec![],
    }
}

fn podman(version: &str) -> testing::Fake {
    testing::podman(version, |req| {
        if req.path.ends_with("/info") {
            (200, INFO_ROOTLESS.into())
        } else if req.path.ends_with("/quadlets/json") {
            (200, LISTED.into())
        } else if req.path.ends_with("/quadlets/web.container/file") {
            (200, "[Container]\nImage=nginx\n".into())
        } else {
            (404, r#"{"message":"no such quadlet: x"}"#.into())
        }
    })
}

#[tokio::test]
async fn lists_units_from_the_api_with_their_containers() {
    let fake = podman("5.8.1");
    let containers = [
        container("c1", Some("web.service"), "running"),
        container("c2", Some("old.service"), "exited"),
        container("c3", None, "running"),
    ];
    let units = list(&fake.podman, &containers).await.unwrap();
    assert_eq!(units.source, QuadletSource::Api);
    assert!(units.manages_files);
    assert_eq!(
        units.runs_systemctl,
        cfg!(target_os = "linux"),
        "only a local socket on Linux runs systemctl: elsewhere Podman runs in a VM"
    );
    let names: Vec<_> = units.units.iter().map(|u| u.name.clone()).collect();
    assert_eq!(
        names,
        [
            Some("cache.volume".into()),
            Some("web.container".into()),
            None
        ]
    );
    let web = &units.units[1];
    assert_eq!(web.unit_name.as_deref(), Some("web.service"));
    assert_eq!(web.status, "active/running");
    assert_eq!(web.pod.as_deref(), Some("shop.pod"));
    assert_eq!(web.scope, UnitScope::User);
    assert_eq!(web.containers.len(), 1);
    let cache = &units.units[0];
    assert_eq!(
        (cache.unit_name.clone(), cache.app.as_deref()),
        (None, Some("shop"))
    );
    // A unit whose file is gone, still running a container by its label.
    let old = &units.units[2];
    assert_eq!(old.unit_name.as_deref(), Some("old.service"));
    assert_eq!(old.status, "exited");
    assert!(fake
        .requests()
        .iter()
        .any(|r| r.path == "/v5.0.0/libpod/quadlets/json"));
}

#[tokio::test]
async fn older_podman_lists_units_from_their_containers_labels() {
    let fake = podman("5.6.2");
    let containers = [
        container("c1", Some("web.service"), "exited"),
        container("c2", Some("web.service"), "running"),
    ];
    let units = list(&fake.podman, &containers).await.unwrap();
    assert_eq!(units.source, QuadletSource::Labels);
    assert!(!units.manages_files);
    assert_eq!(units.units.len(), 1);
    assert_eq!(units.units[0].status, "running");
    assert_eq!(units.units[0].containers.len(), 2);
    assert!(!fake
        .requests()
        .iter()
        .any(|r| r.path.contains("/quadlets/")));
}

#[test]
fn scope_comes_from_the_path_else_from_rootless() {
    assert_eq!(
        scope_of(Some("/etc/containers/systemd/web.container"), true),
        UnitScope::System
    );
    assert_eq!(
        scope_of(
            Some("/etc/containers/systemd/users/1000/web.container"),
            false
        ),
        UnitScope::User
    );
    assert_eq!(
        scope_of(Some("/home/me/.config/containers/systemd/a.pod"), false),
        UnitScope::User
    );
    assert_eq!(scope_of(None, true), UnitScope::User);
    assert_eq!(scope_of(None, false), UnitScope::System);
}

#[tokio::test]
async fn reads_a_units_file_from_5_8() {
    let fake = podman("5.8.1");
    let text = file(&fake.podman, "web.container").await.unwrap();
    assert!(text.starts_with("[Container]"));
    let gone = file(&fake.podman, "other.container").await.unwrap_err();
    assert!(gone.is(ErrorCode::NotFound));
    assert_eq!(gone.key, "errors.quadlets.gone");

    let old = podman("5.7.0");
    let err = file(&old.podman, "web.container").await.unwrap_err();
    assert!(err.is(ErrorCode::PodmanTooOld));
    assert!(err.message.contains("5.8"), "{err:?}");
    assert!(err.message.contains("5.7.0"), "{err:?}");
}

fn unit(name: &str, content: &str) -> QuadletFile {
    QuadletFile {
        name: name.into(),
        content: content.into(),
    }
}

#[test]
fn unit_files_are_checked_before_installing() {
    let ok = QuadletInstall {
        unit: unit("web.container", "# web\n[Container]\nImage=nginx\n"),
        extra: vec![],
        app: None,
    };
    assert_eq!(validate(&ok).unwrap(), None);
    for name in [
        "web",
        "web.service",
        "../web.container",
        "-web.container",
        "w b.container",
    ] {
        let bad = QuadletInstall {
            unit: unit(name, "[Container]"),
            ..ok.clone()
        };
        assert!(
            validate(&bad).unwrap_err().is(ErrorCode::Validation),
            "{name}"
        );
    }
    let wrong_section = QuadletInstall {
        unit: unit("web.pod", "[Container]\n"),
        ..ok.clone()
    };
    let err = validate(&wrong_section).unwrap_err();
    assert!(err.is(ErrorCode::QuadletInvalid));
    assert!(err.message.contains("[Pod]"), "{err:?}");

    let with_files = QuadletInstall {
        extra: vec![unit("Containerfile", "FROM alpine")],
        ..ok.clone()
    };
    assert_eq!(
        validate(&with_files).unwrap().as_deref(),
        Some("web"),
        "an app is needed"
    );
    for name in ["sub/file", "..", "web.container"] {
        let bad = QuadletInstall {
            extra: vec![unit(name, "x")],
            ..ok.clone()
        };
        assert!(validate(&bad).is_err(), "{name}");
    }
    let bad_app = QuadletInstall {
        app: Some("../etc".into()),
        ..ok.clone()
    };
    assert!(validate(&bad_app).is_err());
}

#[test]
fn files_travel_as_a_tar_with_the_unit_first() {
    let install = QuadletInstall {
        unit: unit("web.container", "[Container]\nImage=nginx\n"),
        extra: vec![unit("web.env", "A=1\n")],
        app: None,
    };
    let tar = tar_of(&install).unwrap();
    let mut archive = tar::Archive::new(&tar[..]);
    let entries: Vec<(String, u64)> = archive
        .entries()
        .unwrap()
        .map(|e| {
            let e = e.unwrap();
            (
                e.path().unwrap().display().to_string(),
                e.header().size().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        entries,
        [("web.container".into(), 24), ("web.env".into(), 4)]
    );
}

#[tokio::test]
async fn installs_through_the_api_and_reloads_systemd() {
    let fake = testing::podman("5.8.1", |_| {
        (
            200,
            r#"{"InstalledQuadlets":{"/tmp/q/web.container":"/home/me/.config/containers/systemd/shop/web.container"},"QuadletErrors":{}}"#.into(),
        )
    });
    let request = QuadletInstall {
        unit: unit("web.container", "[Container]\nImage=nginx\n"),
        extra: vec![unit("web.env", "A=1\n")],
        app: None,
    };
    let installed = install(&fake.podman, &request, true).await.unwrap();
    assert_eq!(
        installed,
        ["/home/me/.config/containers/systemd/shop/web.container"]
    );
    let req = fake.requests().pop().unwrap();
    assert_eq!(req.method, "POST");
    assert_eq!(
        req.path,
        "/v5.0.0/libpod/quadlets?replace=true&reload-systemd=true&application=web"
    );
    assert_eq!(
        req.headers.get("content-type").map(String::as_str),
        Some("application/x-tar")
    );
    assert_eq!(req.body, tar_of(&request).unwrap());
}

#[tokio::test]
async fn a_taken_name_and_an_old_podman_are_refused() {
    let fake = testing::podman("5.8.1", |_| {
        (
            400,
            r#"{"cause":"","message":"errors occurred installing some Quadlets: /tmp/libpod_quadlet1/quadlets/web.container: a Quadlet with name web.container already exists, refusing to overwrite","response":400}"#.into(),
        )
    });
    let request = QuadletInstall {
        unit: unit("web.container", "[Container]\n"),
        extra: vec![],
        app: None,
    };
    let err = install(&fake.podman, &request, false).await.unwrap_err();
    assert!(err.is(ErrorCode::QuadletExists), "{err:?}");
    assert!(err.hint.is_some());

    let old = testing::podman("5.7.1", |_| (200, "{}".into()));
    let err = install(&old.podman, &request, false).await.unwrap_err();
    assert!(err.is(ErrorCode::PodmanTooOld));
    assert!(old.requests().is_empty(), "nothing is sent");
}

#[tokio::test]
async fn removes_a_unit_stopping_it_when_asked() {
    let fake = testing::podman("5.8.1", |req| {
        if req.path.contains("force=false") {
            (
                400,
                r#"{"message":"quadlet web.container is running and force is not set, refusing to remove: quadlet is running"}"#.into(),
            )
        } else {
            (200, r#"{"Removed":["web.container"],"Errors":{}}"#.into())
        }
    });
    let err = remove(&fake.podman, "web.container", false)
        .await
        .unwrap_err();
    assert!(err.is(ErrorCode::Conflict));
    assert_eq!(err.key, "errors.quadlets.running");
    remove(&fake.podman, "web.container", true).await.unwrap();
    let req = fake.requests().pop().unwrap();
    assert_eq!(req.method, "DELETE");
    assert_eq!(
        req.path,
        "/v5.0.0/libpod/quadlets/web.container?force=true&reload-systemd=true"
    );
}

#[test]
fn systemctl_commands_are_fixed_and_quoted() {
    assert_eq!(
        unit_command("web.service", UnitScope::User, UnitAction::Start).unwrap(),
        "systemctl --user start 'web.service'"
    );
    assert_eq!(
        unit_command("shop-pod.service", UnitScope::System, UnitAction::Restart).unwrap(),
        "systemctl restart 'shop-pod.service'"
    );
    for bad in [
        "web",
        "web.service; reboot",
        "-x.service",
        "$(id).service",
        "web.socket",
    ] {
        assert!(
            unit_command(bad, UnitScope::User, UnitAction::Stop).is_err(),
            "{bad}"
        );
    }
}

#[test]
fn systemds_refusals_say_what_to_do() {
    let out = |stderr: &str| homeport_transport::CommandOutput {
        status: Some(1),
        stdout: String::new(),
        stderr: stderr.into(),
    };
    let linger = act::refused(
        "web.service",
        UnitAction::Start,
        &out("Failed to connect to bus: No medium found"),
    );
    assert!(linger.is(ErrorCode::UserUnitsNeedLinger));
    assert!(linger.hint.unwrap().commands[0].contains("loginctl enable-linger"));
    let root = act::refused(
        "web.service",
        UnitAction::Start,
        &out("Failed to start web.service: Interactive authentication required."),
    );
    assert!(root.is(ErrorCode::SystemdRefused));
    assert_eq!(root.key, "errors.quadlets.notAllowed");
    let missing = act::refused(
        "web.service",
        UnitAction::Stop,
        &out("Failed to stop web.service: Unit web.service not loaded."),
    );
    assert!(missing.is(ErrorCode::NotFound));
    let other = act::refused("web.service", UnitAction::Start, &out("Job failed."));
    assert!(other.is(ErrorCode::SystemdRefused));
    assert!(other.detail.unwrap().contains("Job failed."));
}

#[tokio::test]
async fn tcp_and_tls_hosts_cant_start_units() {
    struct Remote;
    impl homeport_transport::Dialer for Remote {
        fn connect(
            &self,
        ) -> homeport_transport::BoxFuture<'_, Result<(), homeport_transport::ConnectError>>
        {
            Box::pin(async { Ok(()) })
        }
        fn dial(
            &self,
        ) -> homeport_transport::BoxFuture<'_, std::io::Result<homeport_transport::BoxStream>>
        {
            Box::pin(async { Err(std::io::Error::other("unused")) })
        }
    }
    let podman = Podman::new(std::sync::Arc::new(Remote), "5.8.1");
    let err = act(&podman, "web.service", UnitScope::User, UnitAction::Start)
        .await
        .unwrap_err();
    assert!(err.is(ErrorCode::EngineUnsupported));
    assert!(err.hint.is_some());
}

#[tokio::test]
#[ignore = "needs a local Podman Engine"]
async fn installs_edits_lists_and_removes_a_unit_on_podman() {
    let Some((podman, docker)) = testing::local().await else {
        return;
    };
    let containers = crate::containers::list(&docker).await.unwrap();
    let units = list(&podman, &containers).await.unwrap();
    if !podman.at_least(FILES_SINCE) {
        assert_eq!(
            units.source == QuadletSource::Api,
            podman.at_least(LIST_SINCE)
        );
        eprintln!(
            "Podman {} can't install units; listed only",
            podman.version()
        );
        return;
    }
    let name = format!("{}.container", crate::testing::unique("quadlet"));
    let request = |image: &str| QuadletInstall {
        unit: unit(
            &name,
            &format!("[Container]\nImage={image}\nExec=sleep 60\n"),
        ),
        extra: vec![],
        app: None,
    };
    install(&podman, &request(crate::testing::IMAGE), false)
        .await
        .unwrap();
    let result = async {
        let listed = list(&podman, &[]).await?;
        assert!(listed
            .units
            .iter()
            .any(|u| u.name.as_deref() == Some(&name)));
        install(&podman, &request("busybox:1.37"), true).await?;
        assert!(file(&podman, &name).await?.contains("busybox:1.37"));
        AppResult::Ok(())
    }
    .await;
    let removed = remove(&podman, &name, true).await;
    result.unwrap();
    removed.unwrap();
    let after = list(&podman, &[]).await.unwrap();
    assert!(!after.units.iter().any(|u| u.name.as_deref() == Some(&name)));
}

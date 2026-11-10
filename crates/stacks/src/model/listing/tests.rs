use std::{collections::HashMap, sync::Arc};

use homeport_hosts::{
    store::{HostDraft, HostStore},
    HostConnection,
};
use homeport_storage::{Db, SecretBox};

use super::*;
use crate::{
    linked::detect::{CONFIG_FILES, WORKING_DIR},
    StackKind, StackProblem,
};

struct Setup {
    managed: StackStore,
    linked: LinkedStore,
    synced: SyncedStore,
    host: String,
    cache: FileCache,
}

fn setup() -> Setup {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let secrets = Arc::new(SecretBox::from_key(&[4; 32]));
    let host = HostStore::new(db.clone(), secrets.clone())
        .create(HostDraft {
            name: "local".into(),
            connection: HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            engine: homeport_hosts::EngineKind::Docker,
            secrets: None,
        })
        .unwrap()
        .id;
    Setup {
        managed: StackStore::new(db.clone(), secrets.clone()),
        synced: SyncedStore::new(db.clone(), secrets),
        linked: LinkedStore::new(db),
        host,
        cache: FileCache::new(),
    }
}

fn container(project: &str, service: &str, dir: &std::path::Path) -> Container {
    let file = dir.join("compose.yaml").to_string_lossy().into_owned();
    Container {
        id: format!("{project}-{service}"),
        name: format!("{project}-{service}-1"),
        image: "img".into(),
        image_id: "sha".into(),
        command: String::new(),
        created: 0,
        state: "running".into(),
        status: String::new(),
        health: None,
        ports: vec![],
        labels: HashMap::from([
            (WORKING_DIR.to_owned(), dir.to_string_lossy().into_owned()),
            (CONFIG_FILES.to_owned(), file),
        ]),
        stack: Some(project.into()),
        service: Some(service.into()),
        networks: vec![],
    }
}

#[test]
fn stacks_from_a_folder_here_become_linked_and_fall_back_to_external() {
    let s = setup();
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("compose.yaml"),
        "services:\n  web:\n    image: nginx\n",
    )
    .unwrap();
    let containers = [container("site", "web", dir.path())];

    let stacks = list(
        &s.host,
        &containers,
        &s.managed,
        &s.linked,
        &s.synced,
        &s.cache,
    )
    .unwrap();
    assert_eq!(stacks[0].kind, StackKind::Linked);
    assert_eq!(
        stacks[0].working_dir.as_deref(),
        Some(dir.path().to_string_lossy().as_ref())
    );
    let recorded = s.linked.list(&s.host).unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(
        recorded[0].fingerprint,
        files::files_fingerprint(&recorded[0].config_files)
    );

    // Seen again: recorded once.
    list(
        &s.host,
        &containers,
        &s.managed,
        &s.linked,
        &s.synced,
        &s.cache,
    )
    .unwrap();
    assert_eq!(s.linked.list(&s.host).unwrap().len(), 1);

    std::fs::remove_file(dir.path().join("compose.yaml")).unwrap();
    let stacks = list(
        &s.host,
        &containers,
        &s.managed,
        &s.linked,
        &s.synced,
        &s.cache,
    )
    .unwrap();
    assert_eq!(stacks[0].kind, StackKind::External);
    assert_eq!(stacks[0].problem, Some(StackProblem::FilesMissing));
    assert_eq!(
        s.linked.list(&s.host).unwrap().len(),
        1,
        "nothing is forgotten"
    );

    std::fs::write(
        dir.path().join("compose.yaml"),
        "services:\n  web:\n    image: nginx\n",
    )
    .unwrap();
    let stacks = list(
        &s.host,
        &containers,
        &s.managed,
        &s.linked,
        &s.synced,
        &s.cache,
    )
    .unwrap();
    assert_eq!(stacks[0].kind, StackKind::Linked);
}

#[test]
fn stacks_from_elsewhere_stay_external() {
    let s = setup();
    let containers = [container(
        "remote",
        "web",
        std::path::Path::new("/nowhere/on/this/machine"),
    )];
    let stacks = list(
        &s.host,
        &containers,
        &s.managed,
        &s.linked,
        &s.synced,
        &s.cache,
    )
    .unwrap();
    assert_eq!(stacks[0].kind, StackKind::External);
    assert_eq!(stacks[0].problem, None);
    assert!(s.linked.list(&s.host).unwrap().is_empty());
}

#[tokio::test]
#[ignore = "needs a local Docker Engine and the compose CLI"]
async fn a_stack_started_in_a_folder_here_is_linked() {
    use homeport_engine::testing;

    let docker = testing::docker();
    testing::ensure_image(&docker).await;
    let project = testing::unique("linked");
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("compose.yaml"),
        format!(
            "services:\n  app:\n    image: {}\n    command: [\"sleep\", \"60\"]\n",
            testing::IMAGE
        ),
    )
    .unwrap();
    let up = std::process::Command::new("docker")
        .args(["compose", "--project-name", &project, "up", "--detach"])
        .current_dir(dir.path())
        .env_remove("COMPOSE_FILE")
        .status()
        .unwrap();
    assert!(up.success());

    let s = setup();
    let containers = homeport_engine::containers::list(&docker).await.unwrap();
    let stacks = list(
        &s.host,
        &containers,
        &s.managed,
        &s.linked,
        &s.synced,
        &s.cache,
    )
    .unwrap();
    crate::runtime::lifecycle::remove(&docker, &project)
        .await
        .unwrap();
    let stack = stacks.iter().find(|st| st.name == project).unwrap();
    assert_eq!(stack.kind, StackKind::Linked, "{stack:?}");
}

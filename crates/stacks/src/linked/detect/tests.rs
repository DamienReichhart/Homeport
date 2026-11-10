use std::collections::HashMap;

use super::*;

fn container(service: &str, state: &str, origin: Option<&Origin>) -> Container {
    let mut labels = HashMap::new();
    if let Some(o) = origin {
        labels.insert(WORKING_DIR.to_owned(), o.working_dir.clone());
        labels.insert(CONFIG_FILES.to_owned(), o.config_files.join(","));
    }
    Container {
        id: format!("{service}-id"),
        name: format!("app-{service}-1"),
        image: "img".into(),
        image_id: "sha".into(),
        command: String::new(),
        created: 0,
        state: state.into(),
        status: String::new(),
        health: None,
        ports: vec![],
        labels,
        stack: Some("app".into()),
        service: Some(service.into()),
        networks: vec![],
    }
}

/// A folder holding `files` (name, content), and its origin.
fn folder(files: &[(&str, &str)]) -> (tempfile::TempDir, Origin) {
    let dir = tempfile::tempdir().unwrap();
    let mut config_files = vec![];
    for (name, content) in files {
        let path = dir.path().join(name);
        std::fs::write(&path, content).unwrap();
        config_files.push(path.to_string_lossy().into_owned());
    }
    let origin = Origin {
        working_dir: dir.path().to_string_lossy().into_owned(),
        config_files,
    };
    (dir, origin)
}

const BASE: &str = "services:\n  web:\n    image: nginx\n  db:\n    image: postgres\n  debug:\n    image: busybox\n    profiles: [debug]\n";

#[test]
fn labels_give_the_folder_and_files_in_order() {
    let o = Origin {
        working_dir: "/srv/app".into(),
        config_files: vec!["/srv/app/a.yaml".into(), "/srv/app/b.yaml".into()],
    };
    let c = container("web", "running", Some(&o));
    assert_eq!(origin(&[&c]), Some(o));
    assert_eq!(origin(&[&container("web", "running", None)]), None);
}

#[test]
fn files_that_define_the_running_services_match() {
    let (_dir, o) = folder(&[
        ("compose.yaml", BASE),
        (
            "compose.override.yaml",
            "services:\n  web:\n    ports: [\"80:80\"]\n",
        ),
    ]);
    let cache = FileCache::new();
    let (web, db) = (
        container("web", "running", Some(&o)),
        container("db", "exited", Some(&o)),
    );
    assert!(matches(&cache, &o, &[&web, &db]));
    // A service behind a profile may run or not.
    let debug = container("debug", "running", Some(&o));
    assert!(matches(&cache, &o, &[&web, &db, &debug]));
}

#[test]
fn a_path_that_exists_here_by_coincidence_does_not_match() {
    let (_dir, o) = folder(&[("compose.yaml", "services:\n  other:\n    image: x\n")]);
    let cache = FileCache::new();
    let web = container("web", "running", Some(&o));
    assert!(!matches(&cache, &o, &[&web]));
    // Nor does a folder missing a service it defines.
    let (_dir, o) = folder(&[("compose.yaml", BASE)]);
    let web = container("web", "running", Some(&o));
    assert!(!matches(&cache, &o, &[&web]));
}

#[test]
fn missing_folders_and_unsupported_files_do_not_match() {
    let (dir, o) = folder(&[("compose.yaml", BASE)]);
    let cache = FileCache::new();
    let members = [
        container("web", "running", Some(&o)),
        container("db", "running", Some(&o)),
    ];
    let members: Vec<&Container> = members.iter().collect();
    assert!(matches(&cache, &o, &members));
    assert!(files_present(&cache, &o.config_files));
    drop(dir);
    assert!(!matches(&cache, &o, &members));
    assert!(!files_present(&cache, &o.config_files));

    let (_dir, o) = folder(&[(
        "compose.yaml",
        "include:\n  - other.yaml\nservices:\n  web:\n    image: nginx\n",
    )]);
    let web = container("web", "running", Some(&o));
    assert!(!matches(&cache, &o, &[&web]));
}

#[test]
fn unchanged_files_are_read_once() {
    let (_dir, o) = folder(&[("compose.yaml", BASE)]);
    let cache = FileCache::new();
    let path = Path::new(&o.config_files[0]);
    let first = cache.parsed(path).unwrap();
    assert!(Arc::ptr_eq(&first, &cache.parsed(path).unwrap()));
    std::fs::write(path, "services:\n  web:\n    image: nginx:1.27\n").unwrap();
    let edited = cache.parsed(path).unwrap();
    assert_eq!(edited.services.len(), 1);
}

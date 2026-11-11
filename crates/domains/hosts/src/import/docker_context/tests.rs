use super::*;

struct Docker {
    dir: tempfile::TempDir,
    env: Env,
}

impl Docker {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let env = Env {
            home: dir.path().join("home"),
            local_user: "local".into(),
            docker_config: Some(dir.path().join("docker")),
        };
        Self { dir, env }
    }

    /// Writes a context the way the CLI does, under the hash of its name
    /// (any folder name works for Homeport, which reads `Name`).
    fn context(&self, folder: &str, json: &str, tls: &[&str]) {
        let root = self.dir.path().join("docker/contexts");
        let meta = root.join("meta").join(folder);
        std::fs::create_dir_all(&meta).unwrap();
        std::fs::write(meta.join("meta.json"), json).unwrap();
        if !tls.is_empty() {
            let dir = root.join("tls").join(folder).join("docker");
            std::fs::create_dir_all(&dir).unwrap();
            for name in tls {
                std::fs::write(dir.join(name), format!("PEM {name}")).unwrap();
            }
        }
    }
}

fn meta(name: &str, host: &str, skip: bool) -> String {
    format!(
        r#"{{"Name":"{name}","Metadata":{{}},"Endpoints":{{"docker":{{"Host":"{host}","SkipTLSVerify":{skip}}}}}}}"#
    )
}

fn connection(c: &Context) -> &HostConnection {
    &c.plan.as_ref().unwrap().connection
}

#[test]
fn every_endpoint_kind_maps_to_a_host() {
    let d = Docker::new();
    d.context(
        "a1",
        &meta(
            "colima",
            "unix:///Users/me/.colima/default/docker.sock",
            false,
        ),
        &[],
    );
    d.context(
        "a2",
        &meta("desktop", "npipe:////./pipe/docker_engine", false),
        &[],
    );
    d.context("a3", &meta("lab", "tcp://10.0.0.2:2375", false), &[]);
    d.context(
        "a4",
        &meta("prod", "tcp://prod.example.com:2376", false),
        &["ca.pem", "cert.pem", "key.pem"],
    );
    d.context(
        "a5",
        &meta("edge", "ssh://deploy@edge.example.com:2222", false),
        &[],
    );
    d.context("a6", &meta("bare", "ssh://bare", false), &[]);
    let contexts = read(&d.env);
    let by = |name: &str| contexts.iter().find(|c| c.name == name).unwrap();

    assert_eq!(
        connection(by("colima")),
        &HostConnection::Local {
            socket_path: "/Users/me/.colima/default/docker.sock".into()
        }
    );
    assert_eq!(
        connection(by("desktop")),
        &HostConnection::Local {
            socket_path: "//./pipe/docker_engine".into()
        }
    );
    assert_eq!(
        connection(by("lab")),
        &HostConnection::Tcp {
            address: "10.0.0.2:2375".into()
        }
    );
    let prod = by("prod").plan.as_ref().unwrap();
    assert_eq!(
        prod.connection,
        HostConnection::Tls {
            address: "prod.example.com:2376".into()
        }
    );
    assert_eq!(prod.tls.as_ref().unwrap().key_pem, "PEM key.pem");
    assert_eq!(prod.files.len(), 3);
    assert!(matches!(
        connection(by("edge")),
        HostConnection::Ssh { host, port: 2222, user, auth: SshAuth::Agent, socket_path, .. }
            if host == "edge.example.com" && user == "deploy" && socket_path == DEFAULT_SOCKET
    ));
    assert!(matches!(
        connection(by("bare")),
        HostConnection::Ssh { port: 22, user, .. } if user == "local"
    ));
}

#[test]
fn contexts_that_cant_be_imported_say_why() {
    let d = Docker::new();
    d.context(
        "b1",
        &meta("insecure", "tcp://h:2376", true),
        &["ca.pem", "cert.pem", "key.pem"],
    );
    d.context("b2", &meta("partial", "tcp://h:2376", false), &["ca.pem"]);
    d.context("b3", &meta("k8s", "kube://cluster", false), &[]);
    let contexts = read(&d.env);
    let problem = |name: &str| {
        contexts
            .iter()
            .find(|c| c.name == name)
            .unwrap()
            .plan
            .as_ref()
            .unwrap_err()
            .message
            .clone()
    };
    assert!(problem("insecure").contains("verif"));
    assert!(problem("partial").contains("three"));
    assert!(problem("k8s").contains("kube://cluster"));
}

#[test]
fn the_default_context_and_broken_files_are_left_out() {
    let d = Docker::new();
    d.context(
        "c1",
        &meta("default", "unix:///var/run/docker.sock", false),
        &[],
    );
    d.context("c2", "not json", &[]);
    d.context("c3", &meta("Zed", "tcp://z:2375", false), &[]);
    d.context("c4", &meta("alpha", "tcp://a:2375", false), &[]);
    let names: Vec<_> = read(&d.env).into_iter().map(|c| c.name).collect();
    assert_eq!(names, ["alpha", "Zed"]);
}

#[test]
fn no_docker_folder_means_no_contexts() {
    assert!(read(&Docker::new().env).is_empty());
}

#[test]
fn known_engines_cant_be_imported_twice_and_names_are_numbered() {
    let d = Docker::new();
    d.context("d1", &meta("lab", "tcp://10.0.0.2:2375", false), &[]);
    d.context("d2", &meta("prod", "tcp://10.0.0.3:2375", false), &[]);
    let hosts = vec![
        Host {
            id: "1".into(),
            name: "office".into(),
            connection: HostConnection::Tcp {
                address: "10.0.0.2:2375".into(),
            },
            engine: crate::EngineKind::Docker,
            enabled: true,
            host_key: None,
            created_at: String::new(),
        },
        Host {
            id: "2".into(),
            name: "prod".into(),
            connection: HostConnection::Local {
                socket_path: "/x".into(),
            },
            engine: crate::EngineKind::Docker,
            enabled: true,
            host_key: None,
            created_at: String::new(),
        },
    ];
    let drafts = drafts(read(&d.env), &hosts);
    assert!(drafts[0]
        .problem
        .as_ref()
        .unwrap()
        .message
        .contains("office"));
    assert!(drafts[0].connection.is_none());
    assert_eq!(drafts[1].name, "prod (2)");
}

#[test]
fn nothing_under_the_docker_folder_is_written() {
    let d = Docker::new();
    d.context(
        "e1",
        &meta("prod", "tcp://p:2376", false),
        &["ca.pem", "cert.pem", "key.pem"],
    );
    let before = snapshot(d.dir.path());
    let _ = drafts(read(&d.env), &[]);
    assert_eq!(snapshot(d.dir.path()), before);
}

fn snapshot(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(d).unwrap().flatten() {
            if e.path().is_dir() {
                stack.push(e.path());
            } else {
                out.push((e.path(), std::fs::read(e.path()).unwrap()));
            }
        }
    }
    out.sort();
    out
}

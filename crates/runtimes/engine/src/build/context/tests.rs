use super::*;

fn folder(files: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for (path, content) in files {
        let path = dir.path().join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }
    dir
}

async fn names(context: Context) -> Vec<String> {
    let mut rx = context.stream();
    let mut tar = Vec::new();
    while let Some(chunk) = rx.recv().await {
        tar.extend_from_slice(&chunk.unwrap());
    }
    let mut archive = tar::Archive::new(tar.as_slice());
    let mut names: Vec<String> = archive
        .entries()
        .unwrap()
        .map(|e| e.unwrap().path().unwrap().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[tokio::test]
async fn ignored_files_are_not_sent_but_the_dockerfile_always_is() {
    let dir = folder(&[
        ("Dockerfile", "FROM alpine"),
        (
            ".dockerignore",
            "node_modules\n*.log\nDockerfile\n!keep.log\n",
        ),
        ("src/main.js", "x"),
        ("node_modules/a/index.js", "x"),
        ("debug.log", "x"),
        ("keep.log", "x"),
    ]);
    let context = Context::open(dir.path(), "Dockerfile").unwrap();
    assert_eq!(
        names(context).await,
        [
            ".dockerignore",
            "Dockerfile",
            "keep.log",
            "src",
            "src/main.js"
        ]
    );
}

#[tokio::test]
async fn exceptions_reach_inside_excluded_folders() {
    let dir = folder(&[
        ("Dockerfile", "FROM alpine"),
        (".dockerignore", "docs\n!docs/keep.md\n"),
        ("docs/keep.md", "x"),
        ("docs/drop.md", "x"),
    ]);
    let context = Context::open(dir.path(), "Dockerfile").unwrap();
    assert_eq!(
        names(context).await,
        [".dockerignore", "Dockerfile", "docs/keep.md"]
    );
}

#[test]
fn the_dockerfile_must_be_inside_the_context() {
    let dir = folder(&[("build/app.Dockerfile", "FROM alpine")]);
    let ok = Context::open(dir.path(), "./build/app.Dockerfile").unwrap();
    assert_eq!(ok.dockerfile, "build/app.Dockerfile");
    for bad in ["../Dockerfile", "/etc/Dockerfile", ""] {
        let err = Context::open(dir.path(), bad).unwrap_err();
        assert!(
            err.message.contains("inside the build folder"),
            "{bad}: {err:?}"
        );
    }
    let err = Context::open(dir.path(), "Dockerfile").unwrap_err();
    assert!(err.message.contains("There's no Dockerfile"));
    let err = Context::open(&dir.path().join("nope"), "Dockerfile").unwrap_err();
    assert!(err.message.contains("isn't a folder"));
}

#[tokio::test]
async fn large_contexts_arrive_in_chunks() {
    let big = "x".repeat(CHUNK * 3);
    let dir = folder(&[("Dockerfile", "FROM alpine"), ("big.bin", &big)]);
    let mut rx = Context::open(dir.path(), "Dockerfile").unwrap().stream();
    let mut chunks = 0;
    while let Some(chunk) = rx.recv().await {
        assert!(chunk.unwrap().len() <= CHUNK * 2);
        chunks += 1;
    }
    assert!(chunks > 3);
}

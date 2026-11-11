use homeport_core::error::ErrorCode;

use super::*;
use crate::api::engine::testing::{assert_all, code, host};
use crate::app::testing::app;

use homeport_core::error::Message;

const REFUSED: Message = Message {
    key: "tests.refused",
    en: "refused",
};

fn build_spec(tag: &str) -> BuildSpec {
    BuildSpec {
        context: ".".into(),
        dockerfile: String::new(),
        build_args: Vec::new(),
        target: None,
        tags: vec![tag.to_owned()],
        no_cache: false,
        pull: false,
    }
}

#[tokio::test]
async fn a_host_that_isnt_connected_is_refused() {
    let app = app();
    let s = &app.state;
    let id = || "nginx:1".to_owned();
    let tar = || app.dir.path().join("image.tar").display().to_string();
    let import = ImportSpec::default();
    let results = [
        ("list", code(images_list(s, host()).await)),
        ("inspect", code(images_inspect(s, host(), id()).await)),
        ("history", code(images_history(s, host(), id()).await)),
        (
            "search",
            code(images_search(s, host(), "nginx".into()).await),
        ),
        (
            "tag",
            code(images_tag(s, host(), id(), "nginx:2".into()).await),
        ),
        ("remove", code(images_remove(s, host(), id(), false).await)),
        ("prune", code(images_prune(s, host(), true).await)),
        (
            "pull",
            code(images_pull_subscribe(s, host(), id(), |_| true).await),
        ),
        (
            "push",
            code(images_push_subscribe(s, host(), id(), |_| true).await),
        ),
        (
            "build",
            code(images_build_subscribe(s, host(), build_spec("app:1"), |_| true).await),
        ),
        (
            "save",
            code(images_save_subscribe(s, host(), vec![id()], tar(), |_| true).await),
        ),
        (
            "load",
            code(images_load_subscribe(s, host(), tar(), |_| true).await),
        ),
        (
            "import",
            code(images_import_subscribe(s, host(), tar(), import, |_| true).await),
        ),
        (
            "export",
            code(containers_export_subscribe(s, host(), id(), tar(), |_| true).await),
        ),
        ("freshness", code(images_freshness(s, host(), true).await)),
    ];
    assert_all(ErrorCode::HostNotConnected, &results);
    assert!(!std::path::Path::new(&tar()).exists());
}

#[tokio::test]
async fn a_build_is_validated_before_its_host_is_asked() {
    let app = app();
    let spec = build_spec("not a tag");
    let built = images_build_subscribe(&app.state, host(), spec, |_| true).await;
    assert_eq!(code(built), Some(ErrorCode::Validation));
}

#[test]
fn only_a_failure_is_explained() {
    let explain = |e: AppError| e.with_detail("explained");
    assert!(matches!(
        explain_event(&explain, PullEvent::Done),
        PullEvent::Done
    ));
    let refused = PullEvent::Error {
        error: AppError::new(ErrorCode::RegistryAuthRequired, REFUSED),
    };
    let PullEvent::Error { error } = explain_event(&explain, refused) else {
        panic!("a failure stays one");
    };
    assert_eq!(error.code, ErrorCode::RegistryAuthRequired);
    assert_eq!(error.detail.as_deref(), Some("explained"));
}

use homeport_core::error::ErrorCode;
use homeport_stacks::{
    compose::ComposeAction,
    model::diff::Candidate,
    runtime::SourceKind,
    store::{history::NewDeploy, ComposeFile},
};

use super::super::fixtures;
use crate::app::{testing, AppState};

/// Records a finished deploy of the Managed Stack `id` as it is defined
/// in the fixtures.
fn deployed(state: &AppState, id: &str) -> String {
    let stack = state.stacks.managed.get(id).unwrap();
    let files = [("compose.yaml".to_owned(), fixtures::COMPOSE.to_owned())];
    let deploy = NewDeploy {
        owner: &stack.owner,
        stack_id: id,
        kind: SourceKind::Managed,
        name: &stack.name,
        action: ComposeAction::Up,
        files: &files,
        env: "KEY=value",
        profiles: &[],
        commit: None,
        automatic: false,
    };
    let deploy = state.stacks.history.start(&deploy).unwrap();
    state.stacks.history.finish(&deploy, true, Some(0)).unwrap();
    deploy
}

#[tokio::test]
async fn a_compose_command_needs_the_stack_and_its_engine() {
    let app = testing::app();
    let run = |id: &str| {
        let id = id.to_owned();
        super::stacks_compose_subscribe(
            &app.state,
            SourceKind::Managed,
            id,
            ComposeAction::Up,
            |_| true,
        )
    };
    assert_eq!(run("gone").await.unwrap_err().code, ErrorCode::NotFound);
    let stack = fixtures::managed(&app.state, &fixtures::host(&app.state));
    assert!(run(&stack.id).await.is_err());
    assert!(super::stacks_history(&app.state, stack.id)
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn a_redeploy_is_compared_to_the_last_deploy() {
    let app = testing::app();
    let stack = fixtures::managed(&app.state, &fixtures::host(&app.state));
    let diff = |candidate| {
        super::stacks_diff(&app.state, SourceKind::Managed, stack.id.clone(), candidate)
    };
    assert!(!diff(None).await.unwrap().known);

    deployed(&app.state, &stack.id);
    let same = diff(None).await.unwrap();
    assert!(same.known && same.unchanged);
    let candidate = Candidate {
        files: vec![ComposeFile::new("compose.yaml", "services: {}\n")],
        env: Some("KEY=other".into()),
    };
    let changed = diff(Some(candidate)).await.unwrap();
    assert!(!changed.unchanged);
    assert_eq!(changed.env.len(), 1);
}

#[tokio::test]
async fn the_history_lists_deploys_and_shows_one() {
    let app = testing::app();
    let stack = fixtures::managed(&app.state, &fixtures::host(&app.state));
    let deploy = deployed(&app.state, &stack.id);
    let history = super::stacks_history(&app.state, stack.id.clone()).unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(
        (history[0].id.as_str(), history[0].success),
        (&*deploy, Some(true))
    );

    let detail = super::stacks_deploy(&app.state, deploy).unwrap();
    assert_eq!(detail.deploy.files, fixtures::files());
    assert_eq!(detail.env_keys, Some(vec!["KEY".to_owned()]));
    let e = super::stacks_deploy(&app.state, "gone".into()).unwrap_err();
    assert_eq!(e.code, ErrorCode::NotFound);
}

#[tokio::test]
async fn a_rollback_restores_the_definition_before_it_deploys() {
    let app = testing::app();
    let e = super::stacks_rollback_subscribe(&app.state, "gone".into(), |_| true)
        .await
        .unwrap_err();
    assert_eq!(e.code, ErrorCode::NotFound);

    let stack = fixtures::managed(&app.state, &fixtures::host(&app.state));
    let deploy = deployed(&app.state, &stack.id);
    let files = [ComposeFile::new("compose.yaml", "services: {}\n")];
    app.state
        .stacks
        .managed
        .update(&stack.id, &files, "")
        .unwrap();
    // The Host has no Engine, so the redeploy can't start.
    assert!(
        super::stacks_rollback_subscribe(&app.state, deploy, |_| true)
            .await
            .is_err()
    );
    let restored = app.state.stacks.managed.get(&stack.id).unwrap();
    assert_eq!(restored.files, fixtures::files());
    assert_eq!(
        app.state.stacks.managed.env(&stack.id).unwrap(),
        "KEY=value"
    );
}

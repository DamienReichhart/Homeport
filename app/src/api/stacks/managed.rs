//! Managed Stacks: their definition and environment, kept by Homeport.

use homeport_core::error::AppResult;
use homeport_stacks::{
    store::{ComposeFile, ManagedStack},
    StackOwner,
};

use super::target;
use crate::app::AppState;

pub fn stacks_get(state: &AppState, id: String) -> AppResult<ManagedStack> {
    state.stacks.managed.get(&id)
}

/// The decrypted environment of a Managed Stack, for editing.
pub fn stacks_get_env(state: &AppState, id: String) -> AppResult<String> {
    state.stacks.managed.env(&id)
}

/// Creates a Managed Stack on a Host or a Swarm. With `adopt`, it takes
/// over the External Stack of that name; otherwise a name that runs there
/// outside Homeport is refused, since deploying would replace it.
pub async fn stacks_create(
    state: &AppState,
    owner: StackOwner,
    name: String,
    files: Vec<ComposeFile>,
    env: String,
    adopt: bool,
) -> AppResult<ManagedStack> {
    target::known(state, &owner)?;
    if !adopt {
        let engine = target::engine(state, &owner)?;
        state
            .stacks
            .refuse_taking_over(&engine, &owner, &name, None)
            .await?;
    }
    state.stacks.managed.create(&owner, &name, &files, &env)
}

pub fn stacks_update(
    state: &AppState,
    id: String,
    files: Vec<ComposeFile>,
    env: String,
) -> AppResult<ManagedStack> {
    state.stacks.managed.update(&id, &files, &env)
}

/// Forgets a Managed Stack's definition; its containers keep running and it
/// becomes an External Stack.
pub fn stacks_forget(state: &AppState, id: String) -> AppResult<()> {
    state.stacks.managed.delete(&id)
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;
    use homeport_stacks::{store::ComposeFile, StackOwner};

    use super::super::fixtures;
    use crate::app::testing;

    #[tokio::test]
    async fn a_managed_stack_is_created_read_updated_and_forgotten() {
        let app = testing::app();
        let owner = fixtures::host(&app.state);
        let stack = super::stacks_create(
            &app.state,
            owner.clone(),
            "web".into(),
            fixtures::files(),
            "KEY=value".into(),
            true,
        )
        .await
        .unwrap();
        assert_eq!((stack.owner.clone(), stack.name.as_str()), (owner, "web"));
        assert_eq!(
            super::stacks_get(&app.state, stack.id.clone()).unwrap(),
            stack
        );
        assert_eq!(
            super::stacks_get_env(&app.state, stack.id.clone()).unwrap(),
            "KEY=value"
        );

        let files = vec![ComposeFile::new("compose.yaml", "services: {}\n")];
        let updated =
            super::stacks_update(&app.state, stack.id.clone(), files.clone(), String::new())
                .unwrap();
        assert_eq!(updated.files, files);
        assert!(!updated.has_env);

        super::stacks_forget(&app.state, stack.id.clone()).unwrap();
        let e = super::stacks_get(&app.state, stack.id).unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn creating_checks_the_owner_and_what_runs_there() {
        let app = testing::app();
        let create = |owner, adopt| {
            super::stacks_create(
                &app.state,
                owner,
                "web".into(),
                fixtures::files(),
                String::new(),
                adopt,
            )
        };
        let e = create(StackOwner::Host("gone".into()), true)
            .await
            .unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
        assert!(create(StackOwner::Swarm("gone".into()), true)
            .await
            .is_err());
        // Without `adopt`, the Engine is asked whether the name runs there.
        let e = create(fixtures::host(&app.state), false).await.unwrap_err();
        assert_eq!(e.code, ErrorCode::HostNotConnected);
    }
}

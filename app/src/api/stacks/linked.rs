//! Linked Stacks: compose files read in place from a folder on this computer
//! (ADR 0008).

use homeport_core::error::AppResult;
use homeport_stacks::{
    linked::{
        files::LinkedFiles,
        folder::{self, FolderProject},
        LinkedStack,
    },
    StackOwner,
};

use super::target;
use crate::app::AppState;

/// What Compose would make of a folder picked for a new Linked Stack.
pub fn stacks_linked_inspect(path: String) -> AppResult<FolderProject> {
    folder::inspect(std::path::Path::new(&path))
}

/// Records a Linked Stack of a Host or a Swarm for the folder `working_dir`
/// and its compose files. It may take the name of an External Stack only
/// when compose started that one from this folder: it is the same project.
pub async fn stacks_linked_create(
    state: &AppState,
    owner: StackOwner,
    name: String,
    working_dir: String,
    config_files: Vec<String>,
) -> AppResult<LinkedStack> {
    target::known(state, &owner)?;
    let engine = target::engine(state, &owner)?;
    state
        .stacks
        .link(&engine, &owner, &name, &working_dir, &config_files)
        .await
}

/// A Linked Stack's compose files, read from its folder. `acknowledge` notes
/// them as seen, so they no longer count as changed outside Homeport.
pub fn stacks_linked_read(
    state: &AppState,
    id: String,
    acknowledge: bool,
) -> AppResult<LinkedFiles> {
    state.stacks.linked_files(&id, acknowledge)
}

/// Writes one compose file of a Linked Stack in place. `expected` is the
/// fingerprint of the file the edit started from; `None` overwrites it even
/// when it changed outside Homeport.
pub fn stacks_linked_write(
    state: &AppState,
    id: String,
    path: String,
    content: String,
    expected: Option<String>,
) -> AppResult<LinkedFiles> {
    state
        .stacks
        .write_linked_file(&id, &path, &content, expected.as_deref())
}

/// Sets which compose files of its folder a Linked Stack uses, in order.
pub fn stacks_linked_set_files(
    state: &AppState,
    id: String,
    config_files: Vec<String>,
) -> AppResult<LinkedFiles> {
    state.stacks.set_linked_files(&id, &config_files)
}

/// Forgets a Linked Stack: its record goes, its folder and containers stay.
pub fn stacks_linked_forget(state: &AppState, id: String) -> AppResult<()> {
    state.stacks.linked.delete(&id)
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;
    use homeport_stacks::StackOwner;

    use super::super::fixtures;
    use crate::app::testing;

    #[tokio::test]
    async fn a_folder_is_inspected_before_it_is_linked() {
        let app = testing::app();
        let owner = fixtures::host(&app.state);
        let folder = fixtures::linked(&app, &owner).working_dir;
        let project = super::stacks_linked_inspect(folder.clone()).unwrap();
        assert_eq!(project.name, "project");
        assert_eq!(project.files.len(), 1);
        assert_eq!(project.files[0].content, fixtures::COMPOSE);
        assert!(super::stacks_linked_inspect(format!("{folder}/missing")).is_err());
    }

    #[tokio::test]
    async fn linking_needs_a_known_owner_and_its_engine() {
        let app = testing::app();
        let create = |owner| {
            super::stacks_linked_create(
                &app.state,
                owner,
                "project".into(),
                "/tmp".into(),
                vec!["compose.yaml".into()],
            )
        };
        let e = create(StackOwner::Host("gone".into())).await.unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
        assert!(create(StackOwner::Swarm("gone".into())).await.is_err());
        let e = create(fixtures::host(&app.state)).await.unwrap_err();
        assert_eq!(e.code, ErrorCode::HostNotConnected);
    }

    #[tokio::test]
    async fn a_linked_stacks_files_are_read_written_and_forgotten() {
        let app = testing::app();
        let stack = fixtures::linked(&app, &fixtures::host(&app.state));
        let read = super::stacks_linked_read(&app.state, stack.id.clone(), true).unwrap();
        assert_eq!(read.files[0].content, fixtures::COMPOSE);
        assert!(!read.changed_outside);

        let file = read.files[0].clone();
        let content = "services:\n  web:\n    image: caddy\n";
        let written = super::stacks_linked_write(
            &app.state,
            stack.id.clone(),
            file.path.clone(),
            content.into(),
            Some(file.fingerprint.clone()),
        )
        .unwrap();
        assert_eq!(written.files[0].content, content);
        assert_eq!(std::fs::read_to_string(&file.path).unwrap(), content);
        // The file is no longer the one that edit started from.
        let stale = super::stacks_linked_write(
            &app.state,
            stack.id.clone(),
            file.path.clone(),
            String::new(),
            Some(file.fingerprint.clone()),
        );
        assert!(stale.is_err());

        let other = std::path::Path::new(&stack.working_dir).join("compose.prod.yaml");
        std::fs::write(&other, "services: {}\n").unwrap();
        let files = vec![file.path, other.to_str().unwrap().to_owned()];
        let set = super::stacks_linked_set_files(&app.state, stack.id.clone(), files).unwrap();
        assert_eq!(set.files.len(), 2);

        super::stacks_linked_forget(&app.state, stack.id.clone()).unwrap();
        let e = super::stacks_linked_read(&app.state, stack.id, false).unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
        assert!(other.exists(), "forgetting leaves the folder alone");
    }
}

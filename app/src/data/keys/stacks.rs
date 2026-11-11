//! Keys of the Stacks, of what Homeport keeps about each and of where they
//! come from (`stackKeys`, `syncedKeys`, `registryKeys` and
//! `repositoryKeys` in React).

use homeport_stacks::StackOwner;

use crate::data::{keys, Key};

/// The Stacks of a Host, or of a Swarm.
pub fn of(owner: &StackOwner) -> Key {
    match owner {
        StackOwner::Host(id) => keys::host::stacks(id),
        StackOwner::Swarm(id) => keys::swarm::stacks(id),
    }
}

/// Everything an owner has, refreshed after a deploy.
pub fn scope(owner: &StackOwner) -> Key {
    match owner {
        StackOwner::Host(id) => keys::host::scope(id),
        StackOwner::Swarm(id) => keys::swarm::scope(id),
    }
}

/// A Managed Stack's stored definition; its environment is under it.
pub fn managed(managed_id: &str) -> Key {
    Key::new(["managed-stack", managed_id])
}

/// A Linked Stack's compose files, as they are in its folder.
pub fn linked(linked_id: &str) -> Key {
    Key::new(["linked-stack", linked_id])
}

/// A Synced Stack's record; its files and environment are under it.
pub fn synced(synced_id: &str) -> Key {
    Key::new(["synced-stack", synced_id])
}

/// A Stack's deploys; under its Host or Swarm, so the end of a run
/// refreshes it.
pub fn history(owner: &StackOwner, stack_id: &str) -> Key {
    scope(owner).with("stack-history").with(stack_id)
}

/// One deploy of the history, with its output.
pub fn deploy(deploy_id: &str) -> Key {
    Key::new(["stack-deploy", deploy_id])
}

/// What deploying `candidate` would change in the Stack `stack_id` of
/// `kind` (`managed`, `linked`, `synced`).
pub fn diff(kind: &str, stack_id: &str, candidate: &str) -> Key {
    Key::new(["stack-diff", kind, stack_id, candidate])
}

/// Everything about the registry logins.
pub fn registries() -> Key {
    Key::new(["registries"])
}

/// Every stored login, without its secret.
pub fn registry_list() -> Key {
    registries().with("list")
}

/// The login an image reference would be pulled or pushed with.
pub fn registry_login_for(reference: &str) -> Key {
    registries().with("login-for").with(reference)
}

/// A Docker Hub repository's tags matching `filter`.
pub fn hub_tags(repository: &str, filter: &str) -> Key {
    registries().with("hub-tags").with(repository).with(filter)
}

/// Everything about Synced Stacks' git repositories.
pub fn repositories() -> Key {
    Key::new(["repositories"])
}

/// Every stored repository, without its key or token.
pub fn repository_list() -> Key {
    repositories().with("list")
}

/// A repository's branches and tags, read from its server.
pub fn repository_refs(id: &str) -> Key {
    repositories().with("refs").with(id)
}

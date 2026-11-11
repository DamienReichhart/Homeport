//! Keeping a new Stack from taking over one that runs outside Homeport.
//!
//! A Stack is its Compose project (or swarm stack) name, and deploying one
//! runs `up --remove-orphans` (`stack deploy --prune` on a Swarm): a new
//! Managed, Linked or Synced Stack named like an External Stack would remove
//! that Stack's other containers and replace the rest. Taking it over is
//! what Adopt is for, so a new Stack may only use such a name when it adopts.

use std::path::Path;

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_engine::{containers::Container, swarm::services::ServiceView};

use crate::{linked::detect, messages as msg};

/// A Compose project or swarm stack running on a Host or a Swarm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Running {
    pub name: String,
    /// The folder compose ran in, as its containers' labels say; `None` on
    /// a Swarm, whose services don't say.
    pub working_dir: Option<String>,
}

/// The Compose projects a Host's `containers` belong to.
pub fn on_host(containers: &[Container]) -> Vec<Running> {
    let mut projects: std::collections::BTreeMap<&str, Vec<&Container>> = Default::default();
    for c in containers {
        if let Some(project) = c.stack.as_deref() {
            projects.entry(project).or_default().push(c);
        }
    }
    projects
        .into_iter()
        .map(|(name, members)| Running {
            name: name.to_owned(),
            working_dir: detect::origin(&members).map(|o| o.working_dir),
        })
        .collect()
}

/// The swarm stacks a Swarm's `services` belong to.
pub fn on_swarm(services: &[ServiceView]) -> Vec<Running> {
    let mut names: Vec<&str> = services.iter().filter_map(|s| s.stack.as_deref()).collect();
    names.sort_unstable();
    names.dedup();
    names
        .into_iter()
        .map(|name| Running {
            name: name.to_owned(),
            working_dir: None,
        })
        .collect()
}

/// Refuses a new Stack named `name` when a Stack of that name runs outside
/// Homeport (`running`, less the names Homeport already has, which the
/// stores refuse with their own message). A Linked Stack for `folder` may
/// take the name of the project compose started from that same folder: it
/// is that project.
pub fn refuse_taking_over(
    name: &str,
    running: &[Running],
    known: &[String],
    folder: Option<&str>,
) -> AppResult<()> {
    let name = name.trim();
    let Some(project) = running
        .iter()
        .find(|r| r.name == name && !known.iter().any(|k| k == name))
    else {
        return Ok(());
    };
    if let (Some(folder), Some(started_in)) = (folder, project.working_dir.as_deref()) {
        if same_folder(folder, started_in) {
            return Ok(());
        }
    }
    Err(AppError::new(
        ErrorCode::NameInUse,
        msg::NAME_RUNS_EXTERNAL.with("name", name),
    )
    .with_hint(msg::NAME_RUNS_EXTERNAL_HINT.with("name", name)))
}

fn same_folder(a: &str, b: &str) -> bool {
    Path::new(a) == Path::new(b)
        || matches!(
            (std::fs::canonicalize(a), std::fs::canonicalize(b)),
            (Ok(a), Ok(b)) if a == b
        )
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn container(project: &str, working_dir: Option<&str>) -> Container {
        let mut labels = HashMap::new();
        if let Some(dir) = working_dir {
            labels.insert(detect::WORKING_DIR.to_owned(), dir.to_owned());
            labels.insert(
                detect::CONFIG_FILES.to_owned(),
                format!("{dir}/compose.yaml"),
            );
        }
        Container {
            id: format!("{project}-1"),
            name: format!("{project}-web-1"),
            image: "nginx".into(),
            image_id: String::new(),
            command: String::new(),
            created: 0,
            state: "running".into(),
            status: String::new(),
            health: None,
            ports: vec![],
            labels,
            stack: Some(project.into()),
            service: Some("web".into()),
            networks: vec![],
        }
    }

    #[test]
    fn a_new_stack_cannot_take_the_name_of_one_running_outside_homeport() {
        // `nextcloud` was started with `docker compose up` on the server.
        let running = on_host(&[
            container("nextcloud", Some("/srv/nextcloud")),
            container("shop", None),
        ]);
        let refused = refuse_taking_over("nextcloud", &running, &[], None).unwrap_err();
        assert_eq!(refused.code, ErrorCode::NameInUse);
        assert_eq!(refused.key, "errors.stacks.nameRunsExternal");
        assert!(refused.message.contains("nextcloud"));
        assert!(refused.hint.unwrap().text.contains("Adopt"));
        assert!(refuse_taking_over(" shop ", &running, &[], None).is_err());

        assert!(refuse_taking_over("blog", &running, &[], None).is_ok());
        // A name Homeport already has is the store's to refuse.
        assert!(refuse_taking_over("shop", &running, &["shop".into()], None).is_ok());
    }

    #[test]
    fn a_linked_stack_may_take_the_project_started_from_its_own_folder() {
        let running = on_host(&[container("blog", Some("/home/me/blog"))]);
        assert!(refuse_taking_over("blog", &running, &[], Some("/home/me/blog")).is_ok());
        assert!(refuse_taking_over("blog", &running, &[], Some("/home/me/other")).is_err());
    }

    #[test]
    fn swarm_stacks_are_found_by_their_services() {
        let service = |name: &str, stack: Option<&str>| ServiceView {
            id: format!("{name}-id"),
            version: 1,
            name: name.into(),
            image: "nginx".into(),
            image_digest: None,
            mode: homeport_engine::swarm::services::ServiceMode::Replicated,
            replicas: Some(1),
            running: 1,
            desired: 1,
            ports: vec![],
            update_state: None,
            update_message: None,
            stack: stack.map(str::to_owned),
            labels: HashMap::new(),
            created: None,
            updated: None,
        };
        let running = on_swarm(&[
            service("shop_web", Some("shop")),
            service("shop_db", Some("shop")),
            service("lone", None),
        ]);
        assert_eq!(
            running,
            [Running {
                name: "shop".into(),
                working_dir: None
            }]
        );
        // A swarm stack doesn't say where its files are: only Adopt or Link take it.
        assert!(refuse_taking_over("shop", &running, &[], Some("/srv/shop")).is_err());
    }
}

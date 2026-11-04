//! Committing a container's filesystem and configuration to a new image
//! (`docker commit`).

use bollard::{models::ContainerConfig, query_parameters::CommitContainerOptionsBuilder, Docker};
use serde::Deserialize;

use homeport_core::error::{AppError, AppResult};

use crate::images::split_reference;

/// The Dockerfile instructions `docker commit --change` accepts.
pub const COMMIT_INSTRUCTIONS: &[&str] = &[
    "CMD",
    "ENTRYPOINT",
    "ENV",
    "EXPOSE",
    "LABEL",
    "ONBUILD",
    "USER",
    "VOLUME",
    "WORKDIR",
];

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CommitSpec {
    /// The new image's name, `repo[:tag]`; an existing tag moves to it.
    pub reference: String,
    /// The commit message, shown in the image's history.
    pub comment: Option<String>,
    pub author: Option<String>,
    /// Pause the container while committing, so its files are consistent.
    pub pause: bool,
    /// Dockerfile instructions applied to the image, one per entry, like `ENV MODE=prod`.
    pub changes: Vec<String>,
}

/// Checks `spec`: a valid name and supported instructions. Returns the
/// repository, the tag and the change lines to send.
fn validate(spec: &CommitSpec) -> AppResult<(String, String, Vec<String>)> {
    let (repo, tag) = split_reference(&spec.reference);
    if repo.is_empty() || repo.contains('@') {
        return Err(AppError::validation(
            "Name the image like repo:tag, such as web-backup:2026-11-04.",
        ));
    }
    let changes = spec
        .changes
        .iter()
        .map(|c| c.trim())
        .filter(|c| !c.is_empty() && !c.starts_with('#'))
        .map(|c| {
            let (instruction, argument) = c.split_once(char::is_whitespace).unwrap_or((c, ""));
            let instruction = instruction.to_ascii_uppercase();
            if !COMMIT_INSTRUCTIONS.contains(&instruction.as_str()) {
                return Err(AppError::validation(format!(
                    "\"{c}\" isn't a change a commit can make. Start each line with one of {}.",
                    COMMIT_INSTRUCTIONS.join(", ")
                )));
            }
            if argument.trim().is_empty() {
                return Err(AppError::validation(format!(
                    "{instruction} needs a value, like {}.",
                    example(&instruction)
                )));
            }
            Ok(format!("{instruction} {}", argument.trim()))
        })
        .collect::<AppResult<Vec<_>>>()?;
    Ok((repo, tag.unwrap_or_else(|| "latest".into()), changes))
}

fn example(instruction: &str) -> &'static str {
    match instruction {
        "CMD" => "CMD [\"nginx\", \"-g\", \"daemon off;\"]",
        "ENTRYPOINT" => "ENTRYPOINT [\"/entrypoint.sh\"]",
        "ENV" => "ENV MODE=prod",
        "EXPOSE" => "EXPOSE 8080",
        "LABEL" => "LABEL team=web",
        "ONBUILD" => "ONBUILD RUN make",
        "USER" => "USER app",
        "VOLUME" => "VOLUME /data",
        _ => "WORKDIR /app",
    }
}

/// Commits container `id` to a new image; returns the image's ID.
pub async fn commit(docker: &Docker, id: &str, spec: CommitSpec) -> AppResult<String> {
    let (repo, tag, changes) = validate(&spec)?;
    let blank_to_none = |s: &Option<String>| {
        s.as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
    };
    let mut options = CommitContainerOptionsBuilder::default()
        .container(id)
        .repo(&repo)
        .tag(&tag)
        .pause(spec.pause);
    if let Some(comment) = blank_to_none(&spec.comment) {
        options = options.comment(&comment);
    }
    if let Some(author) = blank_to_none(&spec.author) {
        options = options.author(&author);
    }
    if !changes.is_empty() {
        options = options.changes(&changes.join("\n"));
    }
    let created = docker
        .commit_container(options.build(), ContainerConfig::default())
        .await?;
    Ok(created.id)
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::{containers, testing};

    fn spec(reference: &str, changes: &[&str]) -> CommitSpec {
        CommitSpec {
            reference: reference.into(),
            changes: changes.iter().map(|c| (*c).to_owned()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn names_and_changes_are_checked() {
        let (repo, tag, changes) = validate(&spec(
            "registry.home:5000/web-backup",
            &["env MODE=prod", "", "# a note", "  EXPOSE 8080  "],
        ))
        .unwrap();
        assert_eq!(
            (repo.as_str(), tag.as_str()),
            ("registry.home:5000/web-backup", "latest")
        );
        assert_eq!(changes, ["ENV MODE=prod", "EXPOSE 8080"]);

        for bad in [
            spec(" ", &[]),
            spec("web@sha256:abc", &[]),
            spec("web:1", &["RUN apt-get update"]),
            spec("web:1", &["ENV"]),
        ] {
            assert!(
                matches!(validate(&bad), Err(ref e) if e.is(ErrorCode::Validation)),
                "{bad:?}"
            );
        }
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn commits_a_containers_files_and_config_changes() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let guard = testing::container(
            &docker,
            &testing::unique("commit"),
            &["sh", "-c", "echo kept > /committed.txt && sleep 60"],
        )
        .await;
        containers::act(&docker, &guard.id, containers::ContainerAction::Start)
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        let image = testing::ImageName(
            docker.clone(),
            format!("{}:1", testing::unique("committed")),
        );
        commit(
            &docker,
            &guard.id,
            CommitSpec {
                reference: image.1.clone(),
                comment: Some("Before the upgrade".into()),
                pause: true,
                changes: vec!["ENV GREETING=hello".into()],
                ..Default::default()
            },
        )
        .await
        .unwrap();

        let inspected = docker.inspect_image(&image.1).await.unwrap();
        let env = inspected.config.unwrap().env.unwrap();
        assert!(env.contains(&"GREETING=hello".to_owned()), "{env:?}");
        let check = containers::run::run(
            &docker,
            containers::run::RunSpec {
                image: image.1.clone(),
                name: Some(testing::unique("commit-check")),
                command: Some(
                    "sh -c 'grep -q kept /committed.txt && test \"$GREETING\" = hello'".into(),
                ),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
        let exit = loop {
            let state = docker
                .inspect_container(&check, None)
                .await
                .unwrap()
                .state
                .unwrap();
            if state.running != Some(true) {
                break state.exit_code;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        };
        containers::remove(&docker, &check, true, true)
            .await
            .unwrap();
        assert_eq!(exit, Some(0));
        let running = docker.inspect_container(&guard.id, None).await.unwrap();
        assert_eq!(
            running.state.unwrap().running,
            Some(true),
            "the container keeps running"
        );
    }
}

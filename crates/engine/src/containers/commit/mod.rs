//! Committing a container's filesystem and configuration to a new image
//! (`docker commit`).

use bollard::{models::ContainerConfig, query_parameters::CommitContainerOptionsBuilder, Docker};
use serde::Deserialize;

use homeport_core::error::{AppError, AppResult};

use crate::messages as msg;

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
        return Err(AppError::validation(msg::COMMIT_NAME));
    }
    let changes = changes(&spec.changes)?;
    Ok((repo, tag.unwrap_or_else(|| "latest".into()), changes))
}

/// Dockerfile changes for a commit or an import: blank and `#` lines
/// dropped, each starting with an instruction they accept.
pub(crate) fn changes(lines: &[String]) -> AppResult<Vec<String>> {
    lines
        .iter()
        .map(|c| c.trim())
        .filter(|c| !c.is_empty() && !c.starts_with('#'))
        .map(|c| {
            let (instruction, argument) = c.split_once(char::is_whitespace).unwrap_or((c, ""));
            let instruction = instruction.to_ascii_uppercase();
            if !COMMIT_INSTRUCTIONS.contains(&instruction.as_str()) {
                return Err(AppError::validation(
                    msg::COMMIT_UNKNOWN_INSTRUCTION
                        .with("line", c)
                        .with("instructions", COMMIT_INSTRUCTIONS.join(", ")),
                ));
            }
            if argument.trim().is_empty() {
                return Err(AppError::validation(
                    msg::COMMIT_NEEDS_VALUE
                        .with("instruction", instruction.as_str())
                        .with("example", example(&instruction)),
                ));
            }
            Ok(format!("{instruction} {}", argument.trim()))
        })
        .collect()
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
mod tests;

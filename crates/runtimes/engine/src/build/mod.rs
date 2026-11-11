//! Building an image from a folder on this machine: the folder is packed
//! (minus what `.dockerignore` excludes) and sent to the Host's Engine, which
//! builds it with the classic builder, the one the plain Engine API offers.
//! So the same build works on a remote Host, with nothing installed there.

mod context;
mod ignore;

use std::{collections::VecDeque, path::Path};

use bollard::{
    query_parameters::{BuildImageOptionsBuilder, TagImageOptionsBuilder},
    Docker,
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};

use homeport_core::error::{engine, AppError, AppResult, ErrorCode, Text};

use crate::messages as msg;

use self::context::Context;
use crate::{
    auth::{self, RegistryAuth},
    images::split_reference,
};

/// Lines of output kept to explain a failed build.
const OUTPUT_TAIL: usize = 40;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BuildSpec {
    /// The build context: a folder on this machine.
    pub context: String,
    /// Relative to the context; `Dockerfile` when empty.
    pub dockerfile: String,
    pub build_args: Vec<BuildArg>,
    /// The stage to stop at, for multi-stage Dockerfiles.
    pub target: Option<String>,
    /// `repo:tag` names for the result; the first is applied by the build.
    pub tags: Vec<String>,
    pub no_cache: bool,
    /// Pull newer versions of the `FROM` images even if the Host has them.
    pub pull: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BuildArg {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum BuildEvent {
    /// A line of the build's output.
    Line {
        text: String,
    },
    /// The image was built and named.
    Done {
        image_id: Option<String>,
        tags: Vec<String>,
    },
    Error {
        error: AppError,
    },
}

/// Checks a spec before anything is packed or sent.
pub fn validate(spec: &BuildSpec) -> AppResult<()> {
    for tag in &spec.tags {
        let (repo, _) = split_reference(tag);
        if repo.is_empty() || repo.contains('@') || tag.chars().any(char::is_whitespace) {
            return Err(AppError::validation(msg::BUILD_TAG.with("tag", tag)));
        }
    }
    for arg in &spec.build_args {
        if arg.name.trim().is_empty() || arg.name.contains('=') {
            return Err(AppError::validation(msg::BUILD_ARG_NAME));
        }
    }
    Ok(())
}

/// Builds `spec` on the Host, with every login so private `FROM` images
/// pull, streaming the output until the image is built and named.
pub async fn build(
    docker: Docker,
    spec: BuildSpec,
    logins: Vec<RegistryAuth>,
    sink: impl Fn(BuildEvent) -> bool,
) {
    if let Err(error) = run(&docker, &spec, &logins, &sink).await {
        sink(BuildEvent::Error {
            error: error.logged("image build"),
        });
    }
}

async fn run(
    docker: &Docker,
    spec: &BuildSpec,
    logins: &[RegistryAuth],
    sink: &impl Fn(BuildEvent) -> bool,
) -> AppResult<()> {
    validate(spec)?;
    let dockerfile = match spec.dockerfile.trim() {
        "" => "Dockerfile",
        d => d,
    };
    let context = Context::open(Path::new(spec.context.trim()), dockerfile)?;
    let build_args = spec
        .build_args
        .iter()
        .map(|a| (a.name.trim().to_owned(), a.value.clone()))
        .collect();
    let mut options = BuildImageOptionsBuilder::default()
        .dockerfile(&context.dockerfile)
        .buildargs(&build_args)
        .nocache(spec.no_cache)
        .rm(true)
        .forcerm(true);
    if spec.pull {
        options = options.pull("1");
    }
    if let Some(target) = spec
        .target
        .as_deref()
        .map(str::trim)
        .filter(|t| !t.is_empty())
    {
        options = options.target(target);
    }
    if let Some(first) = spec.tags.first() {
        options = options.t(first);
    }
    let chunks = context.stream();
    let body = futures_util::stream::unfold(chunks, |mut rx| async move {
        rx.recv().await.map(|chunk| (chunk, rx))
    });
    let credentials = (!logins.is_empty()).then(|| auth::config(logins));
    let mut stream = docker.build_image(
        options.build(),
        credentials,
        Some(bollard::body_try_stream(body)),
    );

    let mut tail = VecDeque::with_capacity(OUTPUT_TAIL);
    let mut step = None::<String>;
    let mut image_id = None::<String>;
    while let Some(item) = stream.next().await {
        let info = match item {
            Ok(info) => info,
            // A failed step arrives as a stream error.
            Err(bollard::errors::Error::DockerStreamError { error }) => {
                return Err(failed(&error, step.as_deref(), &tail))
            }
            Err(e) => return Err(with_tail(AppError::from(e), &tail)),
        };
        if let Some(message) = info.error_detail.and_then(|e| e.message) {
            return Err(failed(&message, step.as_deref(), &tail));
        }
        if let Some(id) = info.aux.and_then(|a| a.id) {
            image_id = Some(id);
        }
        let text = info.stream.or(info.status);
        for line in text.iter().flat_map(|t| t.lines()).map(str::trim_end) {
            if line.is_empty() {
                continue;
            }
            if line.starts_with("Step ") {
                step = Some(line.to_owned());
            }
            if tail.len() == OUTPUT_TAIL {
                tail.pop_front();
            }
            tail.push_back(line.to_owned());
            if !sink(BuildEvent::Line {
                text: line.to_owned(),
            }) {
                return Ok(());
            }
        }
    }
    // The first name came with the build; the others are added now.
    if let Some(id) = &image_id {
        for tag in spec.tags.iter().skip(1) {
            let (repo, t) = split_reference(tag);
            let options = TagImageOptionsBuilder::default()
                .repo(&repo)
                .tag(t.as_deref().unwrap_or("latest"))
                .build();
            docker.tag_image(id, Some(options)).await?;
        }
    }
    sink(BuildEvent::Done {
        image_id,
        tags: spec.tags.clone(),
    });
    Ok(())
}

/// A build step failed: name the step, keep the output's end as detail.
/// Failures the Engine has a better name for (a private `FROM` image, a full
/// disk) keep it.
fn failed(message: &str, step: Option<&str>, tail: &VecDeque<String>) -> AppError {
    let classified = engine::from_message(None, message);
    let error = if matches!(
        classified.code,
        ErrorCode::EngineError | ErrorCode::BadRequest
    ) {
        AppError::new(
            ErrorCode::BuildFailed,
            match step {
                Some(step) => failed_at(step),
                None => msg::BUILD_FAILED.into(),
            },
        )
        .with_hint(msg::BUILD_FAILED_HINT)
    } else {
        classified
    };
    with_tail(error, tail).with_detail_prefix(message)
}

/// `Step 3/5 : RUN make` → "The build failed at step 3/5 (RUN make)."
fn failed_at(step: &str) -> Text {
    let rest = step.trim_start_matches("Step ");
    match rest.split_once(" : ") {
        Some((n, instruction)) => msg::BUILD_FAILED_AT_STEP
            .with("step", n)
            .with("instruction", instruction),
        None => msg::BUILD_FAILED_AT.with("step", rest),
    }
}

fn with_tail(error: AppError, tail: &VecDeque<String>) -> AppError {
    if tail.is_empty() {
        return error;
    }
    let output = tail.iter().cloned().collect::<Vec<_>>().join("\n");
    let detail = match &error.detail {
        Some(d) => format!("{d}\n\n{output}"),
        None => output,
    };
    error.with_detail(detail)
}

trait DetailPrefix {
    fn with_detail_prefix(self, prefix: &str) -> Self;
}

impl DetailPrefix for AppError {
    fn with_detail_prefix(self, prefix: &str) -> Self {
        let detail = match &self.detail {
            Some(d) => format!("{prefix}\n\n{d}"),
            None => prefix.to_owned(),
        };
        self.with_detail(detail)
    }
}

#[cfg(test)]
mod tests;

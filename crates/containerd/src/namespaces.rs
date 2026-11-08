//! containerd's namespaces: every object lives in one, and every call names
//! the namespace it's about.

use std::collections::HashMap;

use serde::Serialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    api::containerd::services::{
        containers::v1::ListContainersRequest,
        images::v1::ListImagesRequest,
        namespaces::v1::{
            CreateNamespaceRequest, DeleteNamespaceRequest, ListNamespacesRequest, Namespace,
        },
    },
    client::call,
    error, messages as msg, Containerd,
};

/// The namespace nerdctl and `ctr` use when none is named.
pub const DEFAULT: &str = "default";

/// The longest name containerd accepts.
const MAX_LEN: usize = 76;

/// One namespace and what it holds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContainerdNamespace {
    pub name: String,
    pub labels: HashMap<String, String>,
    pub containers: u32,
    pub images: u32,
}

/// Refuses a name containerd wouldn't accept: letters and digits, separated
/// by single `.`, `-` or `_`.
pub fn validate(name: &str) -> AppResult<()> {
    let valid = !name.is_empty()
        && name.len() <= MAX_LEN
        && name
            .split(['.', '-', '_'])
            .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_alphanumeric()));
    if valid {
        Ok(())
    } else {
        Err(AppError::validation(
            msg::NAMESPACE_INVALID.with("name", name),
        ))
    }
}

/// Every namespace, sorted by name, with how many containers and images it holds.
pub async fn list(client: &Containerd) -> AppResult<Vec<ContainerdNamespace>> {
    let namespaces = client
        .namespaces()
        .list(ListNamespacesRequest::default())
        .await
        .map_err(|s| error::status(&s))?
        .into_inner()
        .namespaces;
    let mut summaries = Vec::with_capacity(namespaces.len());
    for Namespace { name, labels } in namespaces {
        let containers = client
            .containers()
            .list(call(&name, ListContainersRequest::default())?)
            .await
            .map_err(|s| error::status(&s))?
            .into_inner()
            .containers
            .len();
        let images = client
            .images()
            .list(call(&name, ListImagesRequest::default())?)
            .await
            .map_err(|s| error::status(&s))?
            .into_inner()
            .images
            .len();
        summaries.push(ContainerdNamespace {
            name,
            labels,
            containers: u32::try_from(containers).unwrap_or(u32::MAX),
            images: u32::try_from(images).unwrap_or(u32::MAX),
        });
    }
    summaries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(summaries)
}

/// Creates an empty namespace.
pub async fn create(client: &Containerd, name: &str) -> AppResult<()> {
    validate(name)?;
    client
        .namespaces()
        .create(CreateNamespaceRequest {
            namespace: Some(Namespace {
                name: name.to_owned(),
                labels: HashMap::new(),
            }),
        })
        .await
        .map_err(|s| match s.code() {
            tonic::Code::AlreadyExists => AppError::new(
                ErrorCode::NameInUse,
                msg::NAMESPACE_EXISTS.with("name", name),
            )
            .with_detail(error::detail(&s)),
            _ => error::status(&s),
        })?;
    Ok(())
}

/// Removes an empty namespace; containerd refuses one that holds anything.
pub async fn remove(client: &Containerd, name: &str) -> AppResult<()> {
    validate(name)?;
    client
        .namespaces()
        .delete(DeleteNamespaceRequest {
            name: name.to_owned(),
        })
        .await
        .map_err(|s| match s.code() {
            tonic::Code::FailedPrecondition => not_empty(name, &s),
            _ => error::status(&s),
        })?;
    Ok(())
}

/// What containerd says a namespace still holds: "namespace x must be
/// empty, but it still has images, blobs, snapshots".
fn not_empty(name: &str, status: &tonic::Status) -> AppError {
    let holds = status
        .message()
        .split_once("still has ")
        .map(|(_, rest)| rest.trim().trim_end_matches('.').to_owned())
        .unwrap_or_default();
    let message = if holds.is_empty() {
        msg::NAMESPACE_NOT_EMPTY.with("name", name)
    } else {
        msg::NAMESPACE_STILL_HOLDS
            .with("name", name)
            .with("holds", holds)
    };
    AppError::new(ErrorCode::NamespaceNotEmpty, message)
        .with_hint(msg::NAMESPACE_NOT_EMPTY_HINT)
        .with_detail(error::detail(status))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_letters_and_digits_with_single_separators() {
        for good in ["default", "k8s.io", "my-ns_2", "a"] {
            assert!(validate(good).is_ok(), "{good}");
        }
        for bad in [
            "",
            "two words",
            "-lead",
            "trail.",
            "a..b",
            "é",
            &"a".repeat(77),
        ] {
            assert!(validate(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn a_refused_removal_says_what_the_namespace_still_holds() {
        let status = tonic::Status::failed_precondition(
            "namespace lab must be empty, but it still has images, snapshots",
        );
        let e = not_empty("lab", &status);
        assert_eq!(e.code, ErrorCode::NamespaceNotEmpty);
        assert_eq!(
            e.message,
            "The namespace lab still holds images, snapshots, so it can't be removed."
        );
        assert!(e.hint.is_some());
    }
}

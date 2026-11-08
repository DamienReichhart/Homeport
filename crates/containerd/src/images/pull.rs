//! Pulling through containerd's Transfer service: containerd resolves,
//! fetches and unpacks the image itself, asks for registry credentials on
//! one stream of the Streaming service and reports progress on another.

use std::collections::HashSet;

use futures_util::{Stream, StreamExt};
use prost::{Message, Name};
use tokio::sync::mpsc;

use homeport_core::{
    error::{engine::from_message, AppError, ErrorCode},
    pull::PullEvent,
};

use crate::{
    api::containerd::{
        services::{
            streaming::v1::StreamInit,
            transfer::v1::{TransferOptions, TransferRequest},
        },
        types::{
            transfer::{
                AuthRequest, AuthResponse, AuthType, ImageStore, OciRegistry, Progress,
                RegistryResolver, UnpackConfiguration,
            },
            Platform,
        },
    },
    client::stream,
    error, introspection, messages as msg, Containerd, PULL_VERSION,
};

use super::qualify;

/// A login to answer the registry's challenge with, in memory only.
#[derive(Clone)]
pub struct PullCredentials {
    pub username: String,
    pub secret: String,
}

impl std::fmt::Debug for PullCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PullCredentials")
            .field("username", &self.username)
            .finish_non_exhaustive()
    }
}

fn any<T: Message + Name>(message: &T) -> prost_types::Any {
    prost_types::Any {
        type_url: T::full_name(),
        value: message.encode_to_vec(),
    }
}

fn is<T: Name>(any: &prost_types::Any) -> bool {
    any.type_url.rsplit('/').next() == Some(T::full_name().as_str())
}

/// Opens a stream of the Streaming service named `id`, waiting for
/// containerd's acknowledgement; the sender keeps it open.
async fn open_stream(
    client: &Containerd,
    namespace: &str,
    id: &str,
) -> Result<
    (
        mpsc::Sender<prost_types::Any>,
        impl Stream<Item = Result<prost_types::Any, tonic::Status>>,
    ),
    AppError,
> {
    let (tx, rx) = mpsc::channel::<prost_types::Any>(16);
    tx.send(any(&StreamInit { id: id.to_owned() }))
        .await
        .map_err(|_| AppError::new(ErrorCode::EngineError, msg::FAILED))?;
    let outbound =
        futures_util::stream::unfold(rx, |mut rx| async move { rx.recv().await.map(|m| (m, rx)) });
    let mut inbound = client
        .streaming()
        .stream(stream(namespace, outbound)?)
        .await
        .map_err(|s| error::status(&s))?
        .into_inner();
    // containerd answers the StreamInit with an empty message once registered.
    inbound
        .next()
        .await
        .transpose()
        .map_err(|s| error::status(&s))?;
    Ok((tx, inbound))
}

/// A progress event as the pull dialog reads it: layers by their short
/// digest, everything else once as a line of its own.
fn event(progress: &Progress, seen: &mut HashSet<String>) -> Option<PullEvent> {
    let desc = progress.desc.as_ref();
    let layer = desc.is_some_and(|d| d.media_type.contains("layer"));
    if layer {
        let digest = desc.map(|d| d.digest.as_str()).unwrap_or_default();
        let hex = digest.split_once(':').map_or(digest, |(_, h)| h);
        return Some(PullEvent::Progress {
            id: Some(hex.chars().take(12).collect()),
            status: progress.event.clone(),
            current: (progress.total > 0).then_some(progress.progress),
            total: (progress.total > 0).then_some(progress.total),
        });
    }
    let line = if progress.name.is_empty() {
        progress.event.clone()
    } else {
        format!("{}: {}", progress.name, progress.event)
    };
    seen.insert(line.clone()).then_some(PullEvent::Progress {
        id: None,
        status: line,
        current: None,
        total: None,
    })
}

/// A failed transfer, in the registry terms the Docker pull uses.
fn failed(status: &tonic::Status) -> AppError {
    let classified = from_message(None, status.message());
    if classified.code != ErrorCode::EngineError {
        return classified.with_detail(error::detail(status));
    }
    let text = status.message().to_ascii_lowercase();
    let code = if text.contains("not found") {
        ErrorCode::ImageNotFound
    } else if text.contains("401") || text.contains("403") || text.contains("unauthorized") {
        ErrorCode::RegistryAuthRequired
    } else {
        return error::status(status);
    };
    let message = match code {
        ErrorCode::ImageNotFound => msg::PULL_NOT_FOUND,
        _ => msg::PULL_AUTH_REQUIRED,
    };
    AppError::new(code, message)
        .with_hint(msg::DETAILS_HINT)
        .with_detail(error::detail(status))
}

/// Pulls `reference` into `namespace` for the Host's platform and unpacks
/// it into its default snapshotter, so `nerdctl run` can use it. Sends
/// progress to `sink` until it's done, fails, or the sink is gone.
pub async fn pull(
    client: Containerd,
    namespace: String,
    reference: String,
    credentials: Option<PullCredentials>,
    sink: impl Fn(PullEvent) -> bool + Send + Sync + 'static,
) {
    let fail = |error: AppError| {
        sink(PullEvent::Error {
            error: error.logged("containerd pull"),
        });
    };
    if !client.at_least(PULL_VERSION) {
        return fail(
            AppError::new(
                ErrorCode::ContainerdTooOld,
                msg::TOO_OLD_TO_PULL
                    .with("name", client.name())
                    .with("version", client.version())
                    .with("minimum", PULL_VERSION),
            )
            .with_hint(msg::TOO_OLD_HINT.with("minimum", PULL_VERSION)),
        );
    }
    let reference = qualify(&reference);
    let info = match introspection::describe(&client).await {
        Ok(info) => info,
        Err(e) => return fail(e),
    };
    let platform = Platform {
        os: info.os.clone(),
        architecture: info.arch.clone(),
        variant: info.variant.clone(),
        os_version: String::new(),
    };
    let suffix = uuid::Uuid::new_v4().simple().to_string();

    let progress_id = format!("homeport-progress-{suffix}");
    let (progress_tx, mut progress) = match open_stream(&client, &namespace, &progress_id).await {
        Ok(opened) => opened,
        Err(e) => return fail(e),
    };
    let auth_id = format!("homeport-auth-{suffix}");
    let auth = match &credentials {
        Some(_) => match open_stream(&client, &namespace, &auth_id).await {
            Ok(opened) => Some(opened),
            Err(e) => return fail(e),
        },
        None => None,
    };

    let request = TransferRequest {
        source: Some(any(&OciRegistry {
            reference: reference.clone(),
            resolver: Some(RegistryResolver {
                auth_stream: if auth.is_some() {
                    auth_id
                } else {
                    String::new()
                },
                ..Default::default()
            }),
        })),
        destination: Some(any(&ImageStore {
            name: reference.clone(),
            platforms: vec![platform.clone()],
            unpacks: vec![UnpackConfiguration {
                platform: Some(platform),
                snapshotter: info.default_snapshotter().to_owned(),
            }],
            ..Default::default()
        })),
        options: Some(TransferOptions {
            progress_stream: progress_id,
        }),
    };
    let request = match stream(&namespace, request) {
        Ok(request) => request,
        Err(e) => return fail(e),
    };
    let mut transfer_client = client.transfer();
    let transfer = transfer_client.transfer(request);

    // Answers every challenge with the stored login; containerd asks once
    // per registry host it reaches.
    let answer = async {
        let (Some((tx, mut inbound)), Some(login)) = (auth, credentials) else {
            return std::future::pending::<()>().await;
        };
        while let Some(Ok(message)) = inbound.next().await {
            if !is::<AuthRequest>(&message) {
                continue;
            }
            let reply = AuthResponse {
                auth_type: AuthType::Credentials.into(),
                username: login.username.clone(),
                secret: login.secret.clone(),
                expire_at: None,
            };
            if tx.send(any(&reply)).await.is_err() {
                break;
            }
        }
        std::future::pending::<()>().await
    };
    let mut seen = HashSet::new();
    let report = async {
        while let Some(Ok(message)) = progress.next().await {
            if !is::<Progress>(&message) {
                continue;
            }
            let Ok(p) = Progress::decode(message.value.as_slice()) else {
                continue;
            };
            if let Some(e) = event(&p, &mut seen) {
                if !sink(e) {
                    return false;
                }
            }
        }
        std::future::pending::<bool>().await
    };
    let outcome = tokio::select! {
        outcome = transfer => Some(outcome),
        false = report => None,
        () = answer => None,
    };
    drop(progress_tx);
    match outcome {
        Some(Ok(_)) => {
            sink(PullEvent::Done);
        }
        Some(Err(status)) => fail(failed(&status)),
        // The sink is gone: the pull is dropped with the call.
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::containerd::types::Descriptor;

    fn progress(event: &str, name: &str, media_type: &str, done: i64, total: i64) -> Progress {
        Progress {
            event: event.into(),
            name: name.into(),
            parents: vec![],
            progress: done,
            total,
            desc: Some(Descriptor {
                media_type: media_type.into(),
                digest: "sha256:7761870a5aa9b608230b543714d3f77fec39b128e056a8d5a22d21d88a701adf"
                    .into(),
                size: total,
                annotations: Default::default(),
            }),
        }
    }

    #[test]
    fn layers_report_their_bytes_and_the_rest_reports_once() {
        let mut seen = HashSet::new();
        let layer = event(
            &progress(
                "downloading",
                "layer",
                "application/vnd.oci.image.layer.v1.tar+gzip",
                512,
                2048,
            ),
            &mut seen,
        );
        assert_eq!(
            layer,
            Some(PullEvent::Progress {
                id: Some("7761870a5aa9".into()),
                status: "downloading".into(),
                current: Some(512),
                total: Some(2048),
            })
        );
        let index = progress(
            "resolved",
            "docker.io/library/alpine:3.22",
            "application/vnd.oci.image.index.v1+json",
            0,
            0,
        );
        assert!(matches!(
            event(&index, &mut seen),
            Some(PullEvent::Progress { id: None, ref status, .. })
                if status == "docker.io/library/alpine:3.22: resolved"
        ));
        assert_eq!(event(&index, &mut seen), None);
    }

    #[test]
    fn refusals_read_as_the_docker_pull_reads_them() {
        let missing = failed(&tonic::Status::not_found(
            "failed to resolve reference \"docker.io/library/nope:latest\": docker.io/library/nope:latest: not found",
        ));
        assert_eq!(missing.code, ErrorCode::ImageNotFound);
        let limited = failed(&tonic::Status::unknown(
            "unexpected status from HEAD request to https://registry-1.docker.io/v2/library/alpine/manifests/3.22: 429 Too Many Requests",
        ));
        assert_eq!(limited.code, ErrorCode::RegistryRateLimited);
        let refused = failed(&tonic::Status::unknown(
            "failed to authorize: failed to fetch anonymous token: unexpected status: 401 Unauthorized",
        ));
        assert_eq!(refused.code, ErrorCode::RegistryAuthRequired);
    }
}

//! Telling an open page when what it shows changed, through Nomad's
//! blocking queries: each watched endpoint is asked again with the index
//! of its last answer, and Nomad answers once something changed or after
//! the wait. Nothing polls on a timer; the watch ends with its stream.

use std::{sync::Arc, time::Duration};

use futures_util::future::join_all;
use serde::Deserialize;

use crate::{api::client::segment, jobs::namespace_query, Nomad};

/// How long Nomad may hold a blocking query before answering unchanged.
const WAIT: Duration = Duration::from_secs(60);
/// How long to wait before asking again after a failure.
const RETRY: Duration = Duration::from_secs(5);

/// What a page watches.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(
    tag = "topic",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum NomadWatch {
    /// The jobs of a namespace (the token's default without one, all with `*`).
    Jobs { namespace: Option<String> },
    /// One job, its summary and its allocations.
    Job {
        namespace: Option<String>,
        job_id: String,
    },
    /// One allocation and its tasks' states.
    Allocation { alloc_id: String },
    /// The client nodes.
    Nodes,
    /// One node and its allocations, as a drain empties it.
    Node { node_id: String },
}

impl NomadWatch {
    /// The endpoints and their queries that answer when this changes.
    fn endpoints(&self) -> Vec<(String, Vec<(&'static str, String)>)> {
        match self {
            Self::Jobs { namespace } => {
                vec![("/v1/jobs".into(), namespace_query(namespace.as_deref()))]
            }
            Self::Job { namespace, job_id } => {
                let base = format!("/v1/job/{}", segment(job_id));
                let query = namespace_query(namespace.as_deref());
                vec![
                    (base.clone(), query.clone()),
                    (format!("{base}/summary"), query.clone()),
                    (format!("{base}/allocations"), query),
                ]
            }
            Self::Allocation { alloc_id } => {
                vec![(format!("/v1/allocation/{}", segment(alloc_id)), vec![])]
            }
            Self::Nodes => vec![("/v1/nodes".into(), vec![])],
            Self::Node { node_id } => {
                let base = format!("/v1/node/{}", segment(node_id));
                vec![
                    (base.clone(), vec![]),
                    (format!("{base}/allocations"), vec![]),
                ]
            }
        }
    }
}

/// Calls `changed` each time what `topic` covers changes, until it returns
/// `false` or the task is cancelled. Failures are retried after a pause:
/// the page shows its own query's error.
pub async fn watch(
    nomad: Nomad,
    topic: NomadWatch,
    changed: impl Fn() -> bool + Send + Sync + 'static,
) {
    let changed = Arc::new(changed);
    join_all(topic.endpoints().into_iter().map(|(path, query)| {
        let nomad = nomad.clone();
        let changed = changed.clone();
        async move { follow(nomad, path, query, changed).await }
    }))
    .await;
}

/// Asks `path` again and again with the index of its last answer, calling
/// `changed` when it moves, until that returns `false`.
async fn follow(
    nomad: Nomad,
    path: String,
    query: Vec<(&'static str, String)>,
    changed: Arc<impl Fn() -> bool + Send + Sync>,
) {
    let mut index = 0;
    loop {
        match nomad
            .get_indexed::<serde_json::Value>(&path, &query, index, WAIT)
            .await
        {
            Ok(answer) => {
                if index > 0 && answer.index != index && !changed() {
                    return;
                }
                // An index that went backwards (a restored server) starts over.
                index = if answer.index < index {
                    0
                } else {
                    answer.index
                };
            }
            Err(error) => {
                log::debug!("watching {path} on {}: {error}", nomad.name());
                tokio::time::sleep(RETRY).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    use super::*;
    use crate::api::client::testing::agent;

    #[test]
    fn a_job_is_watched_through_its_three_endpoints() {
        let topic = NomadWatch::Job {
            namespace: Some("shop".into()),
            job_id: "web".into(),
        };
        let paths: Vec<_> = topic.endpoints().into_iter().map(|(p, _)| p).collect();
        assert_eq!(
            paths,
            [
                "/v1/job/web",
                "/v1/job/web/summary",
                "/v1/job/web/allocations"
            ]
        );
    }

    #[tokio::test]
    async fn an_unchanged_index_reports_nothing_and_is_asked_again() {
        let fake = agent(vec![(
            "GET /v1/jobs",
            200,
            vec![("X-Nomad-Index", "11".into())],
            "[]".into(),
        )])
        .await;
        let calls = Arc::new(AtomicUsize::new(0));
        let seen = calls.clone();
        let handle = tokio::spawn(watch(
            fake.client(None),
            NomadWatch::Jobs { namespace: None },
            move || {
                seen.fetch_add(1, Ordering::SeqCst);
                true
            },
        ));
        tokio::time::sleep(Duration::from_millis(200)).await;
        handle.abort();
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        let sent = fake.received();
        assert!(sent.len() >= 2);
        assert!(!sent[0].target.contains("index="));
        assert!(sent[1].target.contains("index=11") && sent[1].target.contains("wait=60s"));
    }

    #[tokio::test]
    async fn a_change_is_reported_and_the_watch_ends_when_the_page_closes() {
        let fake = agent(vec![(
            "GET /v1/jobs",
            200,
            vec![("X-Nomad-Index", "$COUNT".into())],
            "[]".into(),
        )])
        .await;
        let calls = Arc::new(AtomicUsize::new(0));
        let seen = calls.clone();
        // The page closes on the first change it hears of: the watch ends by itself.
        tokio::time::timeout(
            Duration::from_secs(5),
            watch(
                fake.client(None),
                NomadWatch::Jobs { namespace: None },
                move || {
                    seen.fetch_add(1, Ordering::SeqCst);
                    false
                },
            ),
        )
        .await
        .expect("the watch ended");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}

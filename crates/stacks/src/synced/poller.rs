//! Polling Synced Stacks while Homeport runs (ADR 0014): one round of
//! fetches on an interval, at most one fetch per repository per round, and
//! an exponential backoff for a repository that keeps failing. A new commit
//! that changes a Stack's compose file's folder is "Update available"; with
//! auto-deploy on, it is deployed once, and a failed one isn't retried for
//! the same commit. Nothing polls while Homeport is closed, and reopening
//! it waits an interval before the first round, so there's no burst.

use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use homeport_core::error::{AppError, ErrorCode};
use homeport_git::{ops, GitCli, RefKind};

use super::{SyncedStack, SyncedStacks};

/// The shortest interval between rounds.
pub const MIN_INTERVAL: Duration = Duration::from_secs(60);
/// The longest a failing repository waits between tries.
pub const MAX_BACKOFF: Duration = Duration::from_secs(3600);

/// How an automatic deploy ended.
#[derive(Debug)]
pub enum AutoDeploy {
    Deployed,
    Failed(AppError),
    /// Not tried: the Host isn't connected. The next round tries again.
    Skipped,
}

/// What the poller tells the app, and asks it to do. Implemented by the
/// shell, which knows the Hosts and the webview (ADR 0005).
pub trait PollEvents: Send + Sync + 'static {
    /// A new commit changes `stack`'s compose file's folder.
    fn update_available(&self, stack: &SyncedStack);
    /// Deploys `commit` of `stack`, recorded as automatic.
    fn auto_deploy(
        &self,
        stack: SyncedStack,
        commit: String,
    ) -> Pin<Box<dyn Future<Output = AutoDeploy> + Send + '_>>;
    /// The automatic deploy of `stack` ended.
    fn auto_deployed(&self, stack: &SyncedStack, commit: &str, error: Option<&AppError>);
}

/// The interval with up to ±10% of jitter, so rounds of several Homeports
/// don't line up; `unit` is a random number in `[0, 1)`.
pub fn jittered(interval: Duration, unit: f64) -> Duration {
    let interval = interval.max(MIN_INTERVAL);
    let factor = 0.9 + 0.2 * unit.clamp(0.0, 1.0);
    interval.mul_f64(factor)
}

/// When each failing repository may be tried again.
#[derive(Debug, Default)]
pub struct Backoff {
    failing: HashMap<String, (u32, Instant)>,
}

impl Backoff {
    /// Whether `repository` may be fetched at `now`.
    pub fn ready(&self, repository: &str, now: Instant) -> bool {
        self.failing
            .get(repository)
            .is_none_or(|(_, next)| now >= *next)
    }

    /// Notes a failed fetch at `now`: it waits one interval, then two, four…
    /// up to [`MAX_BACKOFF`].
    pub fn failed(&mut self, repository: &str, now: Instant, interval: Duration) {
        let failures = self.failing.get(repository).map_or(0, |(n, _)| *n) + 1;
        let wait = interval
            .saturating_mul(2u32.saturating_pow(failures.saturating_sub(1)))
            .min(MAX_BACKOFF);
        self.failing
            .insert(repository.to_owned(), (failures, now + wait));
    }

    pub fn succeeded(&mut self, repository: &str) {
        self.failing.remove(repository);
    }
}

/// The polling Stacks made from one repository, and the refs they follow,
/// once each.
pub type Group = (Vec<SyncedStack>, Vec<(RefKind, String)>);

/// The repositories to fetch this round, each with its [`Group`].
pub fn plan(stacks: &[SyncedStack], backoff: &Backoff, now: Instant) -> BTreeMap<String, Group> {
    let mut rounds: BTreeMap<String, Group> = BTreeMap::new();
    for stack in stacks.iter().filter(|s| s.poll) {
        if !backoff.ready(&stack.repository_id, now) {
            continue;
        }
        let entry = rounds.entry(stack.repository_id.clone()).or_default();
        let wanted = (stack.ref_kind, stack.ref_name.clone());
        if !entry.1.contains(&wanted) {
            entry.1.push(wanted);
        }
        entry.0.push(stack.clone());
    }
    rounds
}

/// Whether an auto-deploy of `commit` should be started for `stack`.
pub fn should_auto_deploy(stack: &SyncedStack) -> bool {
    stack.auto_deploy
        && stack.update_available
        && stack.fetched_commit.is_some()
        && stack.auto_deploy_failed_commit != stack.fetched_commit
}

/// Runs the rounds while Homeport runs.
pub struct Poller {
    synced: Arc<SyncedStacks>,
    events: Arc<dyn PollEvents>,
    backoff: Mutex<Backoff>,
}

impl Poller {
    pub fn new(synced: Arc<SyncedStacks>, events: Arc<dyn PollEvents>) -> Self {
        Self {
            synced,
            events,
            backoff: Mutex::default(),
        }
    }

    /// Polls forever: waits a jittered `interval()`, then runs a round.
    /// `interval` is read before each wait, so a new setting applies from
    /// the next round. Dropping the future (or aborting its task) stops it.
    pub async fn run(self: Arc<Self>, interval: impl Fn() -> Duration) {
        loop {
            tokio::time::sleep(jittered(interval(), random_unit())).await;
            // Nothing to poll: don't even look for git.
            let polling = self
                .synced
                .store
                .list_all()
                .map(|all| all.iter().any(|s| s.poll));
            if !polling.unwrap_or(true) {
                continue;
            }
            match homeport_git::cli::detect().await {
                Ok(cli) => self.round(&cli, interval()).await,
                Err(e) => log::debug!("not polling Synced Stacks: {}", e.message),
            }
        }
    }

    /// One round: each repository fetched at most once.
    pub async fn round(&self, cli: &GitCli, interval: Duration) {
        let stacks = match self.synced.store.list_all() {
            Ok(s) => s,
            Err(e) => {
                e.logged("listing Synced Stacks to poll");
                return;
            }
        };
        let now = Instant::now();
        let plan = plan(
            &stacks,
            &self.backoff.lock().expect("backoff poisoned"),
            now,
        );
        for (repository, (group, refs)) in plan {
            let refs: Vec<(RefKind, &str)> = refs.iter().map(|(k, n)| (*k, n.as_str())).collect();
            let fetched = self
                .synced
                .repositories
                .fetch(cli, &repository, &refs)
                .await;
            let fetched = match fetched {
                // One Stack's branch or tag is gone: fetch the others alone.
                Err(e) if e.is(ErrorCode::SyncedComposeFileMissing) && group.len() > 1 => {
                    for stack in &group {
                        self.poll_alone(cli, stack).await;
                    }
                    self.backoff
                        .lock()
                        .expect("backoff poisoned")
                        .succeeded(&repository);
                    continue;
                }
                other => other,
            };
            match fetched {
                Ok(repo) => {
                    self.backoff
                        .lock()
                        .expect("backoff poisoned")
                        .succeeded(&repository);
                    for stack in group {
                        let commit = ops::resolve(cli, &repo, stack.ref_kind, &stack.ref_name)
                            .await
                            .and_then(|c| {
                                c.ok_or_else(|| {
                                    ops::ref_missing(&stack.ref_kind.full(&stack.ref_name))
                                })
                            });
                        self.handle(cli, stack, commit).await;
                    }
                }
                Err(e) => {
                    self.backoff.lock().expect("backoff poisoned").failed(
                        &repository,
                        Instant::now(),
                        interval,
                    );
                    for stack in group {
                        self.note_failure(&stack, &e);
                    }
                }
            }
        }
    }

    async fn poll_alone(&self, cli: &GitCli, stack: &SyncedStack) {
        let commit = self
            .synced
            .fetch_commit(cli, &stack.repository_id, stack.ref_kind, &stack.ref_name)
            .await;
        self.handle(cli, stack.clone(), commit).await;
    }

    fn note_failure(&self, stack: &SyncedStack, error: &AppError) {
        if let Err(e) = self.synced.store.set_polled(&stack.id, Some(error)) {
            e.logged("noting a failed poll");
        }
    }

    /// Notes what a poll found for `stack`, and deploys it when asked to.
    async fn handle(&self, cli: &GitCli, stack: SyncedStack, commit: Result<String, AppError>) {
        let commit = match commit {
            Ok(c) => c,
            Err(e) => return self.note_failure(&stack, &e),
        };
        if let Err(e) = self.synced.store.set_polled(&stack.id, None) {
            e.logged("noting a poll");
        }
        let new = stack.fetched_commit.as_deref() != Some(commit.as_str());
        let stack = if new {
            match self.synced.note_fetched(cli, &stack, &commit).await {
                Ok(s) => s,
                Err(e) => return self.note_failure(&stack, &e),
            }
        } else {
            stack
        };
        if new && stack.update_available {
            self.events.update_available(&stack);
        }
        if !should_auto_deploy(&stack) {
            return;
        }
        match self.events.auto_deploy(stack.clone(), commit.clone()).await {
            AutoDeploy::Deployed => self.events.auto_deployed(&stack, &commit, None),
            AutoDeploy::Skipped => {}
            AutoDeploy::Failed(error) => {
                if let Err(e) = self
                    .synced
                    .store
                    .set_auto_deploy_failed(&stack.id, &commit, &error)
                {
                    e.logged("noting a failed auto-deploy");
                }
                self.events.auto_deployed(&stack, &commit, Some(&error));
            }
        }
    }
}

/// A random number in `[0, 1)`, from the OS.
fn random_unit() -> f64 {
    let bytes = uuid::Uuid::new_v4().into_bytes();
    let n = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    f64::from(n) / (f64::from(u32::MAX) + 1.0)
}

#[cfg(test)]
mod tests {
    use homeport_git::cli;

    use super::*;
    use crate::synced::{testing::Lab, SyncedDraft};

    fn stack(id: &str, repository: &str, ref_name: &str, poll: bool) -> SyncedStack {
        SyncedStack {
            id: id.into(),
            host_id: "h".into(),
            name: id.into(),
            repository_id: repository.into(),
            ref_kind: RefKind::Branch,
            ref_name: ref_name.into(),
            compose_path: "compose.yaml".into(),
            has_env: false,
            profiles: vec![],
            deployed_commit: None,
            fetched_commit: None,
            poll,
            auto_deploy: false,
            last_polled_at: None,
            last_poll_error: None,
            update_available: false,
            auto_deploy_failed_commit: None,
            auto_deploy_error: None,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    #[test]
    fn rounds_are_jittered_within_ten_percent_and_never_under_a_minute() {
        let five = Duration::from_secs(300);
        assert_eq!(jittered(five, 0.0), Duration::from_secs(270));
        assert_eq!(jittered(five, 0.5), five);
        assert!(jittered(five, 0.999_999) < Duration::from_secs(330));
        assert_eq!(jittered(Duration::from_secs(5), 0.5), MIN_INTERVAL);
        for _ in 0..100 {
            let unit = random_unit();
            assert!((0.0..1.0).contains(&unit));
        }
    }

    #[test]
    fn a_repository_is_fetched_once_per_round_for_all_its_stacks() {
        let stacks = [
            stack("a", "r1", "main", true),
            stack("b", "r1", "main", true),
            stack("c", "r1", "dev", true),
            stack("d", "r2", "main", true),
            stack("e", "r3", "main", false),
        ];
        let plan = plan(&stacks, &Backoff::default(), Instant::now());
        assert_eq!(
            plan.keys().collect::<Vec<_>>(),
            ["r1", "r2"],
            "r3 doesn't poll"
        );
        let (group, refs) = &plan["r1"];
        assert_eq!(group.len(), 3);
        assert_eq!(
            refs,
            &[
                (RefKind::Branch, "main".to_owned()),
                (RefKind::Branch, "dev".to_owned())
            ]
        );
    }

    #[test]
    fn a_failing_repository_backs_off_exponentially_up_to_an_hour() {
        let interval = Duration::from_secs(300);
        let start = Instant::now();
        let mut backoff = Backoff::default();
        backoff.failed("r1", start, interval);
        assert!(!backoff.ready("r1", start + Duration::from_secs(299)));
        assert!(backoff.ready("r1", start + interval));
        assert!(backoff.ready("r2", start), "others aren't held back");
        backoff.failed("r1", start, interval);
        assert!(!backoff.ready("r1", start + Duration::from_secs(599)));
        assert!(backoff.ready("r1", start + Duration::from_secs(600)));
        for _ in 0..10 {
            backoff.failed("r1", start, interval);
        }
        assert!(backoff.ready("r1", start + MAX_BACKOFF));
        let stacks = [stack("a", "r1", "main", true)];
        assert!(plan(&stacks, &backoff, start).is_empty());
        backoff.succeeded("r1");
        assert!(backoff.ready("r1", start));
    }

    #[test]
    fn auto_deploy_needs_it_turned_on_a_waiting_update_and_no_failure_for_that_commit() {
        let mut s = stack("a", "r1", "main", true);
        s.fetched_commit = Some("c2".into());
        s.update_available = true;
        assert!(!should_auto_deploy(&s), "off by default");
        s.auto_deploy = true;
        assert!(should_auto_deploy(&s));
        s.auto_deploy_failed_commit = Some("c2".into());
        assert!(!should_auto_deploy(&s), "a failed commit isn't retried");
        s.auto_deploy_failed_commit = Some("c1".into());
        assert!(should_auto_deploy(&s), "a new commit is");
        s.update_available = false;
        assert!(!should_auto_deploy(&s));
    }

    #[derive(Default)]
    struct Recorded {
        updates: Mutex<Vec<String>>,
        deploys: Mutex<Vec<String>>,
        fail: bool,
    }

    impl PollEvents for Recorded {
        fn update_available(&self, stack: &SyncedStack) {
            self.updates.lock().unwrap().push(stack.name.clone());
        }
        fn auto_deploy(
            &self,
            _stack: SyncedStack,
            commit: String,
        ) -> Pin<Box<dyn Future<Output = AutoDeploy> + Send + '_>> {
            self.deploys.lock().unwrap().push(commit);
            let fail = self.fail;
            Box::pin(async move {
                if fail {
                    AutoDeploy::Failed(AppError::new(
                        ErrorCode::ComposeFailed,
                        crate::messages::GONE,
                    ))
                } else {
                    AutoDeploy::Deployed
                }
            })
        }
        fn auto_deployed(&self, _: &SyncedStack, _: &str, _: Option<&AppError>) {}
    }

    async fn polled_lab(fail: bool, auto: bool) -> (Lab, Arc<Recorded>, Poller, SyncedStack) {
        let git = cli::detect().await.unwrap();
        let lab = Lab::new();
        let first = lab.commit(
            "First",
            &[("compose.yaml", "services:\n  a:\n    image: x:1\n")],
        );
        let stack = lab
            .synced
            .create(
                &git,
                &lab.host,
                &SyncedDraft {
                    name: "shop".into(),
                    repository_id: "r1".into(),
                    ref_kind: RefKind::Branch,
                    ref_name: "main".into(),
                    compose_path: "compose.yaml".into(),
                },
                "",
            )
            .await
            .unwrap();
        lab.synced.store.set_deployed(&stack.id, &first).unwrap();
        let stack = lab.synced.store.set_sync(&stack.id, true, auto).unwrap();
        let events = Arc::new(Recorded {
            fail,
            ..Default::default()
        });
        // The poller needs its own handle on the same stores.
        let synced = Arc::new(super::super::SyncedStacks::new(
            super::super::SyncedStore::new(lab.db.clone(), lab.secrets.clone()),
            lab.synced.repositories.clone(),
            lab.dir.path().join("data/synced-stacks"),
        ));
        let poller = Poller::new(synced, events.clone());
        (lab, events, poller, stack)
    }

    #[tokio::test]
    async fn a_new_commit_is_picked_up_and_not_deployed_without_auto_deploy() {
        let git = cli::detect().await.unwrap();
        let (lab, events, poller, stack) = polled_lab(false, false).await;
        poller.round(&git, Duration::from_secs(300)).await;
        assert!(events.updates.lock().unwrap().is_empty(), "nothing new yet");

        let second = lab.commit(
            "Bump",
            &[("compose.yaml", "services:\n  a:\n    image: x:2\n")],
        );
        poller.round(&git, Duration::from_secs(300)).await;
        let after = lab.synced.store.get(&stack.id).unwrap();
        assert_eq!(after.fetched_commit, Some(second));
        assert!(after.update_available);
        assert!(after.last_polled_at.is_some() && after.last_poll_error.is_none());
        assert_eq!(*events.updates.lock().unwrap(), ["shop"]);
        assert!(events.deploys.lock().unwrap().is_empty());
        assert_ne!(after.deployed_commit, after.fetched_commit);

        // Seen again, it isn't announced twice.
        poller.round(&git, Duration::from_secs(300)).await;
        assert_eq!(events.updates.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn with_auto_deploy_a_commit_is_deployed_once_and_a_failure_is_not_looped() {
        let git = cli::detect().await.unwrap();
        let (lab, events, poller, stack) = polled_lab(true, true).await;
        let second = lab.commit(
            "Bump",
            &[("compose.yaml", "services:\n  a:\n    image: x:2\n")],
        );
        poller.round(&git, Duration::from_secs(300)).await;
        poller.round(&git, Duration::from_secs(300)).await;
        assert_eq!(
            *events.deploys.lock().unwrap(),
            std::slice::from_ref(&second),
            "tried once"
        );
        let after = lab.synced.store.get(&stack.id).unwrap();
        assert_eq!(after.auto_deploy_failed_commit, Some(second));
        assert!(after.auto_deploy_error.is_some());

        let third = lab.commit(
            "Fix",
            &[("compose.yaml", "services:\n  a:\n    image: x:3\n")],
        );
        poller.round(&git, Duration::from_secs(300)).await;
        assert_eq!(
            events.deploys.lock().unwrap().last(),
            Some(&third),
            "a new commit is tried"
        );
    }

    #[tokio::test]
    async fn a_failing_fetch_is_noted_on_its_stacks() {
        let git = cli::detect().await.unwrap();
        let (lab, _, poller, stack) = polled_lab(false, false).await;
        std::fs::remove_dir_all(lab.dir.path().join("origin.git")).unwrap();
        poller.round(&git, Duration::from_secs(300)).await;
        let after = lab.synced.store.get(&stack.id).unwrap();
        assert!(after.last_poll_error.is_some());
        // Backed off: the next round doesn't try it.
        let now = Instant::now();
        assert!(!poller.backoff.lock().unwrap().ready("r1", now));
    }
}

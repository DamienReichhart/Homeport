use homeport_git::cli;

use super::*;
use crate::synced::{testing::Lab, SyncedDraft};

fn stack(id: &str, repository: &str, ref_name: &str, poll: bool) -> SyncedStack {
    SyncedStack {
        id: id.into(),
        owner: crate::StackOwner::from("h"),
        registry_auth: false,
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
    let synced = Arc::new(crate::synced::SyncedStacks::new(
        crate::synced::SyncedStore::new(lab.db.clone(), lab.secrets.clone()),
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

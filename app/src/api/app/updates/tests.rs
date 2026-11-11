use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use super::*;

/// An updater that finds `found`, downloads it in two chunks of 40 and 60
/// bytes, and fails each step with `fails`, when set.
struct Fake {
    bundle: Bundle,
    found: Option<AvailableUpdate>,
    fails: Option<UpdateFailureKind>,
    waiting: Mutex<Option<String>>,
    restarted: AtomicBool,
}

impl Fake {
    fn new(bundle: Bundle, found: Option<&str>) -> Self {
        Self {
            bundle,
            found: found.map(|version| AvailableUpdate {
                version: version.to_owned(),
                notes: Some("Faster lists.".to_owned()),
                date: Some("2026-01-02T03:04:05Z".to_owned()),
            }),
            fails: None,
            waiting: Mutex::new(None),
            restarted: AtomicBool::new(false),
        }
    }

    fn failure(&self) -> Result<(), UpdateFailure> {
        match self.fails {
            Some(kind) => Err(UpdateFailure {
                kind,
                cause: "the updater's own words".into(),
            }),
            None => Ok(()),
        }
    }
}

impl Updater for Fake {
    fn bundle(&self) -> Bundle {
        self.bundle
    }

    async fn check(&self) -> Result<Option<AvailableUpdate>, UpdateFailure> {
        self.failure()?;
        *self.waiting.lock().unwrap() = self.found.as_ref().map(|u| u.version.clone());
        Ok(self.found.clone())
    }

    fn waiting(&self) -> Option<String> {
        self.waiting.lock().unwrap().clone()
    }

    async fn download_and_install(
        &self,
        mut on_chunk: impl FnMut(usize, Option<u64>) + Send,
    ) -> Result<(), UpdateFailure> {
        on_chunk(40, Some(100));
        self.failure()?;
        on_chunk(60, Some(100));
        Ok(())
    }

    fn restart(&self) {
        self.restarted.store(true, Ordering::SeqCst);
    }
}

/// What `on_progress` was given, and the callback to give.
fn progress() -> (
    Arc<Mutex<Vec<UpdateProgress>>>,
    impl Fn(UpdateProgress) -> bool + Send + Sync + 'static,
) {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let sink = seen.clone();
    (seen, move |p| {
        sink.lock().unwrap().push(p);
        // Nobody listens: the install must carry on.
        false
    })
}

#[tokio::test]
async fn a_check_says_what_is_available_and_how_this_copy_updates() {
    let updater = Fake::new(Bundle::AppImage, Some("9.9.9"));

    let check = updates_check(&updater).await.unwrap();

    assert_eq!(check.current_version, env!("CARGO_PKG_VERSION"));
    assert_eq!(check.available, updater.found);
    assert_eq!(check.method, UpdateMethod::InApp);
}

#[tokio::test]
async fn a_check_that_finds_nothing_still_says_how_this_copy_updates() {
    let check = updates_check(&Fake::new(Bundle::Unknown, None)).await;

    let check = check.unwrap();
    assert_eq!(check.available, None);
    assert_eq!(check.method, UpdateMethod::Manual);
}

#[tokio::test]
async fn an_install_reports_its_progress_then_restarts() {
    let updater = Fake::new(Bundle::AppImage, Some("9.9.9"));
    updates_check(&updater).await.unwrap();
    let (seen, on_progress) = progress();

    updates_install(&updater, on_progress).await.unwrap();

    let total = Some(100);
    let expected =
        [(40, total), (100, total)].map(|(downloaded, total)| UpdateProgress { downloaded, total });
    assert_eq!(*seen.lock().unwrap(), expected);
    assert!(updater.restarted.load(Ordering::SeqCst));
}

#[tokio::test]
async fn a_package_managed_install_is_refused() {
    let updater = Fake::new(Bundle::Deb, Some("9.9.9"));
    updates_check(&updater).await.unwrap();
    let (seen, on_progress) = progress();

    let refused = updates_install(&updater, on_progress).await.unwrap_err();

    assert_eq!(refused.code, ErrorCode::Validation);
    assert_eq!(refused.key, msg::UPDATE_BY_PACKAGE_MANAGER.key);
    assert!(seen.lock().unwrap().is_empty());
    assert!(!updater.restarted.load(Ordering::SeqCst));
}

#[tokio::test]
async fn nothing_is_installed_before_a_check_found_an_update() {
    let updater = Fake::new(Bundle::AppImage, Some("9.9.9"));
    let (_, on_progress) = progress();

    let refused = updates_install(&updater, on_progress).await.unwrap_err();

    assert_eq!(refused.code, ErrorCode::NotFound);
    assert_eq!(refused.key, msg::UPDATE_NOTHING_WAITING.key);
    assert!(!updater.restarted.load(Ordering::SeqCst));
}

#[tokio::test]
async fn a_failed_install_doesnt_restart() {
    let mut updater = Fake::new(Bundle::AppImage, Some("9.9.9"));
    updates_check(&updater).await.unwrap();
    updater.fails = Some(UpdateFailureKind::Signature);
    let (seen, on_progress) = progress();

    let failed = updates_install(&updater, on_progress).await.unwrap_err();

    assert_eq!(failed.code, ErrorCode::UpdateSignatureInvalid);
    assert_eq!(seen.lock().unwrap().len(), 1);
    assert!(!updater.restarted.load(Ordering::SeqCst));
}

#[tokio::test]
async fn each_failure_tells_the_user_what_to_do() {
    use UpdateFailureKind as K;
    let cases = [
        (K::Network, ErrorCode::UpdateNetwork, msg::UPDATE_DOWNLOAD),
        (
            K::NonePublished,
            ErrorCode::UpdateFailed,
            msg::UPDATE_NONE_PUBLISHED,
        ),
        (
            K::Signature,
            ErrorCode::UpdateSignatureInvalid,
            msg::UPDATE_SIGNATURE,
        ),
        (
            K::AdminRefused,
            ErrorCode::UpdatePermissionDenied,
            msg::UPDATE_ADMIN_REFUSED,
        ),
        (
            K::NotAllowed,
            ErrorCode::UpdatePermissionDenied,
            msg::UPDATE_NOT_ALLOWED,
        ),
        (
            K::WrongDisk,
            ErrorCode::UpdatePermissionDenied,
            msg::UPDATE_WRONG_DISK,
        ),
        (K::Other, ErrorCode::UpdateFailed, msg::UPDATE_FAILED),
    ];
    for (kind, code, message) in cases {
        let mut updater = Fake::new(Bundle::AppImage, None);
        updater.fails = Some(kind);

        let failed = updates_check(&updater).await.unwrap_err();

        assert_eq!(failed.code, code, "{kind:?}");
        assert_eq!(failed.key, message.key, "{kind:?}");
        // Only the catch-all has nothing to suggest.
        assert_eq!(failed.hint.is_some(), kind != K::Other, "{kind:?}");
        let detail = failed.detail.unwrap_or_default();
        assert!(detail.contains("the updater's own words"), "{detail}");
    }
}

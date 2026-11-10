use std::{fs, path::Path, sync::Arc};

use homeport_storage::Db;

use super::*;

fn service(dir: &Path) -> SettingsService {
    SettingsService::new(
        Arc::new(Db::open_in_memory().unwrap()),
        &dir.join("settings.json"),
    )
    .unwrap()
}

#[test]
fn empty_store_yields_defaults() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(service(dir.path()).get().unwrap(), Settings::default());
}

#[test]
fn the_language_follows_the_os_unless_chosen() {
    let dir = tempfile::tempdir().unwrap();
    let svc = service(dir.path());
    // Settings saved before 0.22 have no language and keep the rest.
    let saved: Settings =
        serde_json::from_str(r#"{ "theme": "dark", "checkForUpdates": false }"#).unwrap();
    assert_eq!(saved.language, SYSTEM_LANGUAGE);
    assert_eq!(saved.theme, Theme::Dark);
    assert!(!saved.check_for_updates);
    let set = svc
        .update(Settings {
            language: "fr".to_owned(),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(set.language, "fr");
    assert_eq!(svc.get().unwrap().language, "fr");
    let blank = svc
        .update(Settings {
            language: String::new(),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(blank.language, SYSTEM_LANGUAGE);
}

#[test]
fn update_checks_are_on_unless_turned_off() {
    let dir = tempfile::tempdir().unwrap();
    let svc = service(dir.path());
    assert!(svc.get().unwrap().check_for_updates);
    // Settings saved before the option existed keep update checks on.
    let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
    assert!(saved.check_for_updates);
    svc.update(Settings {
        check_for_updates: false,
        ..Default::default()
    })
    .unwrap();
    assert!(!svc.get().unwrap().check_for_updates);
}

#[test]
fn synced_stacks_poll_every_five_minutes_and_never_under_one() {
    let dir = tempfile::tempdir().unwrap();
    let svc = service(dir.path());
    assert_eq!(svc.get().unwrap().sync_interval().as_secs(), 300);
    let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
    assert_eq!(saved.sync_interval_minutes, 5, "settings from before 0.11");
    let set = svc
        .update(Settings {
            sync_interval_minutes: 0,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(set.sync_interval_minutes, 1);
    assert_eq!(svc.get().unwrap().sync_interval().as_secs(), 60);
}

#[test]
fn events_are_kept_thirty_days_from_one_to_a_year() {
    let dir = tempfile::tempdir().unwrap();
    let svc = service(dir.path());
    let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
    assert_eq!(saved.keep_events_days, 30, "settings from before 0.12");
    for (asked, kept) in [(0, 1), (90, 90), (1000, 365)] {
        let set = svc
            .update(Settings {
                keep_events_days: asked,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(set.keep_events_days, kept);
    }
    assert_eq!(svc.get().unwrap().keep_events().as_secs(), 365 * 86_400);
}

#[test]
fn closing_quits_unless_close_to_tray_is_on() {
    let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
    assert!(!saved.close_to_tray, "settings from before 0.13");
    assert!(!Settings::default().close_to_tray);
}

#[test]
fn starting_at_login_opens_the_window_unless_start_hidden_is_on() {
    let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
    assert!(!saved.start_hidden, "settings from before 0.13");
    assert!(!Settings::default().start_hidden);
}

#[test]
fn every_notification_is_on_unless_turned_off() {
    let saved: Settings = serde_json::from_str(r#"{ "theme": "dark" }"#).unwrap();
    assert_eq!(
        saved.notifications,
        NotificationSettings::default(),
        "settings from before 0.13"
    );
    let on = NotificationSettings::default();
    assert!(on.container_died && on.container_unhealthy && on.host_unreachable && on.host_back);
    // One kind turned off keeps the others on, as do kinds added later.
    let saved: Settings =
        serde_json::from_str(r#"{ "notifications": { "hostBack": false } }"#).unwrap();
    assert!(!saved.notifications.host_back);
    assert!(saved.notifications.container_died);
}

#[test]
fn settings_from_before_clusters_keep_their_host() {
    let saved: Settings = serde_json::from_str(r#"{ "lastHostId": "h1" }"#).unwrap();
    assert_eq!(saved.last_host_id.as_deref(), Some("h1"));
    assert_eq!(saved.last_selection, None);
}

#[test]
fn a_selected_cluster_is_read_back() {
    let saved: Settings =
        serde_json::from_str(r#"{ "lastSelection": { "kind": "cluster", "id": "c1" } }"#).unwrap();
    assert_eq!(
        saved.last_selection,
        Some(Selection::Cluster { id: "c1".into() })
    );
}

#[test]
fn update_is_read_back() {
    let dir = tempfile::tempdir().unwrap();
    let svc = service(dir.path());
    svc.update(Settings {
        theme: Theme::Dark,
        ..Default::default()
    })
    .unwrap();
    assert_eq!(svc.get().unwrap().theme, Theme::Dark);
}

#[test]
fn legacy_json_is_imported_once() {
    let dir = tempfile::tempdir().unwrap();
    let legacy = dir.path().join("settings.json");
    fs::write(&legacy, r#"{ "theme": "light" }"#).unwrap();
    let db = Arc::new(Db::open_in_memory().unwrap());

    let svc = SettingsService::new(db.clone(), &legacy).unwrap();
    assert_eq!(svc.get().unwrap().theme, Theme::Light);
    assert!(!legacy.exists());
    assert!(dir.path().join("settings.json.imported").exists());

    svc.update(Settings {
        theme: Theme::Dark,
        ..Default::default()
    })
    .unwrap();
    let again = SettingsService::new(db, &legacy).unwrap();
    assert_eq!(again.get().unwrap().theme, Theme::Dark);
}

#[test]
fn unreadable_legacy_json_is_left_alone() {
    let dir = tempfile::tempdir().unwrap();
    let legacy = dir.path().join("settings.json");
    fs::write(&legacy, "{ not json").unwrap();
    let svc = SettingsService::new(Arc::new(Db::open_in_memory().unwrap()), &legacy).unwrap();
    assert_eq!(svc.get().unwrap(), Settings::default());
    assert!(legacy.exists());
}

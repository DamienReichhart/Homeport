//! The selection's rules, as `selection.test.ts` and `ScopeGuard.test.tsx`
//! held them.

use homeport_settings::Settings;

use super::{
    remember::Remember, remembered, resolve, scopes, sends_to_overview, Candidate, Selection,
};
use crate::{app::testing, data::keys, ui::shell::registry::Scope};

fn host(id: &str) -> Selection {
    Selection::Host { id: id.to_owned() }
}

fn cluster(id: &str) -> Selection {
    Selection::Cluster { id: id.to_owned() }
}

fn usable(id: &str) -> Candidate {
    Candidate::new(id, true)
}

fn down(id: &str) -> Candidate {
    Candidate::new(id, false)
}

#[test]
fn the_selection_saved_by_this_version_is_read() {
    let settings = Settings {
        last_selection: Some(cluster("c1")),
        last_host_id: Some("h1".to_owned()),
        ..Settings::default()
    };
    assert_eq!(remembered(&settings), Some(cluster("c1")));
}

#[test]
fn the_host_remembered_by_an_older_version_is_kept() {
    let settings = Settings {
        last_host_id: Some("h1".to_owned()),
        ..Settings::default()
    };
    assert_eq!(remembered(&settings), Some(host("h1")));
    assert_eq!(remembered(&Settings::default()), None, "a first launch");
}

#[test]
fn the_chosen_one_is_kept_while_it_exists_or_its_list_loads() {
    let chosen = host("h2");
    let hosts = [usable("h1"), usable("h2")];
    assert_eq!(
        resolve(Some(&chosen), Some(&hosts), Some(&[])),
        Some(host("h2"))
    );
    let chosen = cluster("c1");
    assert_eq!(
        resolve(Some(&chosen), Some(&[usable("h1")]), None),
        Some(cluster("c1"))
    );
}

#[test]
fn it_falls_back_to_the_first_connected_host_then_the_first() {
    let gone = host("gone");
    assert_eq!(
        resolve(Some(&gone), Some(&[down("h1"), usable("h2")]), Some(&[])),
        Some(host("h2"))
    );
    assert_eq!(
        resolve(None, Some(&[down("h1"), down("h2")]), Some(&[])),
        Some(host("h1"))
    );
}

#[test]
fn it_falls_back_to_a_cluster_when_there_are_no_hosts() {
    let gone = cluster("gone");
    assert_eq!(
        resolve(Some(&gone), Some(&[]), Some(&[down("c1"), usable("c2")])),
        Some(cluster("c2"))
    );
}

#[test]
fn nothing_is_selected_before_the_hosts_load_or_when_there_is_nothing() {
    assert_eq!(resolve(None, None, None), None);
    assert_eq!(resolve(None, Some(&[]), Some(&[])), None);
}

#[test]
fn a_page_of_the_other_kind_gives_way_to_the_overview() {
    let (h, c) = (host("h1"), cluster("c1"));
    assert!(sends_to_overview(Scope::Host, Some(&c)));
    assert!(sends_to_overview(Scope::Cluster, Some(&h)));
    assert!(!sends_to_overview(Scope::Host, Some(&h)));
    assert!(!sends_to_overview(Scope::App, Some(&c)));
    // With nothing selected a page shows, and explains what to add.
    assert!(!sends_to_overview(Scope::Host, None));
    assert!(!sends_to_overview(Scope::Cluster, None));
}

#[test]
fn what_is_selected_owns_its_scope_of_the_data_layer() {
    assert_eq!(scopes(&host("h1")), [keys::host::scope("h1")]);
    assert_eq!(
        scopes(&cluster("c1")),
        [keys::cluster::scope("c1"), keys::nomad::scope("c1")]
    );
}

#[test]
fn a_host_is_remembered_for_older_versions_too_and_a_cluster_leaves_it() {
    let mut settings = Settings::default();
    Remember::Selection(host("h1")).apply(&mut settings);
    assert_eq!(settings.last_host_id.as_deref(), Some("h1"));
    Remember::Selection(cluster("c1")).apply(&mut settings);
    assert_eq!(settings.last_selection, Some(cluster("c1")));
    assert_eq!(settings.last_host_id.as_deref(), Some("h1"));
    Remember::ClusterNamespace {
        cluster_id: "c1".to_owned(),
        namespace: None,
    }
    .apply(&mut settings);
    assert_eq!(settings.cluster_namespaces["c1"], "");
}

#[tokio::test]
async fn the_selection_is_still_there_after_a_restart() {
    let app = testing::app();
    let state = std::sync::Arc::new(app.state);
    let saved = Remember::Selection(cluster("c1")).save(&state);
    let results = crate::test_support::tasks::messages(saved).await;
    assert!(results.len() == 1 && results[0].error.is_none());
    let picked = Remember::IncusProject {
        host_id: "h1".to_owned(),
        project: "lab".to_owned(),
    };
    crate::test_support::tasks::messages(picked.save(&state)).await;
    // What a new launch reads from the same database.
    let again = homeport_settings::SettingsService::new(
        state.db.clone(),
        &state.config_dir.join("settings.json"),
    )
    .unwrap()
    .get()
    .unwrap();
    assert_eq!(remembered(&again), Some(cluster("c1")));
    assert_eq!(again.incus_projects["h1"], "lab");
}

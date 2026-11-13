//! The Files tab without an Engine: the listing, the folders, and the
//! dialogs a transfer starts from.

use homeport_core::error::ErrorCode;

use super::*;
use crate::{
    platform::{dialogs::OpenFile, fake::Asked as AskedOfOs},
    test_support::{
        fixtures::file_entries,
        snapshot::{assert_snapshot, in_language},
    },
    ui::{
        features::docker::containers::detail::{
            queries,
            tabs::files::{self, Asking},
        },
        shell::page::testing::Asked,
        theme::Theme,
        widgets::{basic::technical, data::file_browser},
    },
};

fn key(path: &str) -> crate::data::Key {
    queries::files_key("h1", WEB, path)
}

/// The page on its Files tab, `/` read.
fn listed() -> Rig {
    let mut rig = Rig::at(WEB, "files");
    rig.bench.data.set(&key("/"), file_entries("/"));
    rig.refresh();
    rig
}

fn browse(message: file_browser::Message) -> Message {
    Message::Files(files::Message::Browser(message))
}

#[test]
fn the_root_is_read_when_the_tab_shows_and_a_folder_when_it_is_opened() {
    let _english = in_language("en");
    let mut bench = Bench::on_host(Route::Containers, connected_host());
    bench.data.set(&keys::host::containers("h1"), sample());
    bench.history.push(route(WEB, Some("logs")).into());
    let mut rig = Rig::open_on(bench);
    rig.send(Message::Tab(Tab::Files));
    assert_eq!(rig.bench.watched(), [key("/")]);
    rig.bench.data.set(&key("/"), file_entries("/"));
    rig.refresh();
    assert!(rig.shows(&technical("hello.txt")));
    rig.send(browse(file_browser::Message::Navigate("/etc".to_owned())));
    assert_eq!(rig.state.files.path, "/etc");
    assert_eq!(rig.bench.watched(), [key("/etc")]);
    // Refresh reads the folder that shows again.
    rig.send(browse(file_browser::Message::Refresh));
    assert_eq!(rig.bench.took(), [Asked::Invalidate(key("/etc"))]);
}

#[test]
fn a_stopped_container_s_files_are_never_asked_for() {
    let rig = Rig::at("shop-db-1-0123456789abcdef", "files");
    assert!(rig.bench.watched().is_empty());
    {
        let _english = in_language("en");
        assert!(rig.shows("Start the container to browse its files."));
    }
    assert_snapshot(
        "container-files-stopped-en",
        "en",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
}

#[test]
fn a_download_asks_where_under_a_safe_name_and_a_cancel_does_nothing() {
    let _english = in_language("en");
    let mut rig = listed();
    // A name Windows refuses, as a container may hold one.
    let mut odd = file_entries("/");
    odd[1].name = "report:2026?.txt".to_owned();
    odd[1].path = "/report:2026?.txt".to_owned();
    rig.bench.data.set(&key("/"), odd);
    rig.refresh();
    rig.send(browse(file_browser::Message::Download(
        "/report:2026?.txt".to_owned(),
    )));
    let asked = rig.bench.os.took();
    let [AskedOfOs::Save(request)] = asked.as_slice() else {
        panic!("{asked:?}");
    };
    assert_eq!(request.title, "Save report:2026?.txt");
    assert!(!request.name.contains([':', '?']), "{}", request.name);
    assert!(request.name.ends_with(".txt"));
    // Cancelled: nothing runs, nothing is said, the tab is free again.
    assert!(rig.state.files.transfer.is_none() && rig.state.files.asking.is_none());
    assert!(rig.bench.took().is_empty());
    // A folder is saved as an archive.
    rig.send(browse(file_browser::Message::Download("/etc".to_owned())));
    let asked = rig.bench.os.took();
    assert!(matches!(asked.as_slice(), [AskedOfOs::Save(request)] if request.name == "etc.tar"));
}

#[test]
fn an_upload_asks_for_files_for_the_folder_that_shows() {
    let _english = in_language("en");
    let mut rig = listed();
    rig.frame(|frame| frame.click("Upload here"));
    assert_eq!(
        rig.bench.os.took(),
        [AskedOfOs::Open(OpenFile::new("Upload to /"), true)]
    );
    assert!(rig.bench.took().is_empty(), "no file picked is a cancel");
    assert!(!rig.state.files.is_busy());
}

#[test]
fn a_dialog_that_is_open_holds_a_second_one_back() {
    let mut rig = listed();
    let ctx = rig.bench.ctx();
    // The dialog is asked, its answer not yet given.
    let _ = rig.state.files.browse(
        file_browser::Message::Download("/hello.txt".to_owned()),
        &ctx,
    );
    assert!(matches!(rig.state.files.asking, Some(Asking::Download(_))));
    let (task, _) = rig.state.files.ask_upload(&ctx);
    assert!(crate::test_support::tasks::is_none(task));
    let _ = rig
        .state
        .files
        .browse(file_browser::Message::Download("/etc".to_owned()), &ctx);
    assert_eq!(rig.bench.os.took().len(), 1, "one dialog");
}

#[test]
fn a_desktop_with_no_dialog_says_so_and_a_host_that_is_down_fails_the_transfer() {
    let _english = in_language("en");
    let mut rig = listed();
    rig.bench.os.answer(rig.bench.os.dir().join("hello.txt"));
    rig.send(browse(file_browser::Message::Download(
        "/hello.txt".to_owned(),
    )));
    // No Engine answers a test: the shell says why.
    let said = rig.bench.took();
    assert!(
        matches!(said.as_slice(), [Asked::Failed(None, error)] if error.code == ErrorCode::HostNotConnected),
        "{said:?}"
    );
    assert!(rig.state.files.transfer.is_none());
    assert!(!rig.bench.os.dir().join("hello.txt").exists());
    rig.bench.os.without_dialogs();
    rig.frame(|frame| frame.click("Upload here"));
    let said = rig.bench.took();
    assert!(
        matches!(said.as_slice(), [Asked::Failed(None, error)] if error.key == "errors.file.dialogUnavailable"),
        "{said:?}"
    );
    assert!(!rig.state.files.is_busy());
}

#[test]
fn the_files_tab_drawn() {
    let rig = listed();
    assert_snapshot("container-files-en", "en", &Theme::Light, SIZE, || {
        rig.view()
    });
    assert_snapshot("container-files-ar", "ar", &Theme::Light, SIZE, || {
        rig.view()
    });
    assert_snapshot("container-files-dark-en", "en", &Theme::Dark, SIZE, || {
        rig.view()
    });
    let rig = Rig::at(WEB, "files");
    assert_snapshot(
        "container-files-loading-en",
        "en",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
}

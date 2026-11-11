use std::{
    cell::{Cell, RefCell},
    io,
    path::{Path, PathBuf},
};

use homeport_core::error::ErrorCode;

use super::*;
use crate::app::testing;

/// A login entry kept in memory; `broken` makes the OS refuse everything.
#[derive(Default)]
struct Login {
    enabled: Cell<bool>,
    broken: bool,
}

impl Login {
    fn set(&self, enabled: bool) -> io::Result<()> {
        if self.broken {
            return Err(io::Error::other("login items are managed"));
        }
        self.enabled.set(enabled);
        Ok(())
    }
}

impl LoginEntry for Login {
    fn is_enabled(&self) -> io::Result<bool> {
        if self.broken {
            return Err(io::Error::other("login items are managed"));
        }
        Ok(self.enabled.get())
    }

    fn enable(&self) -> io::Result<()> {
        self.set(true)
    }

    fn disable(&self) -> io::Result<()> {
        self.set(false)
    }
}

#[test]
fn the_app_says_what_it_is_and_runs_on() {
    let info = get_app_info();
    assert_eq!(info.name, "Homeport");
    assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(info.os, std::env::consts::OS);
    assert_eq!(info.arch, std::env::consts::ARCH);
}

#[test]
fn a_normal_start_has_no_problem_to_show() {
    assert!(startup_problem(&Startup::default()).is_none());
}

#[test]
fn a_failed_start_keeps_the_error_and_the_database_path() {
    let error = AppError::new(ErrorCode::DbCorrupt, msg::WINDOW_FAILED);
    let startup = Startup::failed(error, PathBuf::from("/data/homeport.db"));

    let problem = startup_problem(&startup).unwrap();

    assert_eq!(problem.error.code, ErrorCode::DbCorrupt);
    assert_eq!(problem.database_path, "/data/homeport.db");
}

#[tokio::test]
async fn the_data_folder_is_the_one_holding_the_database() {
    let app = testing::app();
    let opened = RefCell::new(None);

    let done = open_data_folder(&app.state, |dir| {
        *opened.borrow_mut() = Some(dir.to_owned());
        Ok(())
    });

    done.unwrap();
    let opened = opened.into_inner().unwrap();
    assert_eq!(opened, app.state.data_dir);
    assert!(opened.join("homeport.db").exists());
}

#[tokio::test]
async fn a_file_manager_that_fails_is_reported_with_the_folder() {
    let app = testing::app();
    let refuse = |_: &Path| Err(io::Error::other("no file manager"));

    let failed = open_data_folder(&app.state, refuse).unwrap_err();
    assert_eq!(failed.code, ErrorCode::Io);
    assert!(failed
        .detail
        .unwrap_or_default()
        .contains("no file manager"));

    let logs = app.dir.path().join("logs");
    let failed = open_log_folder(&logs, refuse).unwrap_err();
    assert_eq!(failed.code, ErrorCode::Io);
}

#[test]
fn the_log_folder_is_made_before_it_is_opened() {
    let dir = tempfile::tempdir().unwrap();
    let logs = dir.path().join("never/written/logs");
    let opened = RefCell::new(None);

    let done = open_log_folder(&logs, |dir| {
        *opened.borrow_mut() = Some((dir.to_owned(), dir.is_dir()));
        Ok(())
    });

    done.unwrap();
    assert_eq!(opened.into_inner(), Some((logs, true)));
}

#[test]
fn start_at_login_follows_the_os_entry() {
    let login = Login::default();
    assert!(!app_start_at_login(&login).unwrap());

    assert!(app_set_start_at_login(&login, true).unwrap());
    assert!(app_start_at_login(&login).unwrap());

    assert!(!app_set_start_at_login(&login, false).unwrap());
    assert!(!app_start_at_login(&login).unwrap());
}

#[test]
fn a_refused_login_entry_says_so() {
    let login = Login {
        broken: true,
        ..Login::default()
    };

    let read = app_start_at_login(&login).unwrap_err();
    assert_eq!(read.code, ErrorCode::AutostartFailed);
    assert_eq!(read.key, msg::AUTOSTART_FAILED.key);
    assert!(read.hint.is_some());

    for enabled in [true, false] {
        let set = app_set_start_at_login(&login, enabled).unwrap_err();
        assert_eq!(set.code, ErrorCode::AutostartFailed);
    }
}

#[test]
fn licenses_do_without_the_third_party_file() {
    let dir = tempfile::tempdir().unwrap();

    let licenses = get_licenses(dir.path()).unwrap();

    assert!(!licenses.app.trim().is_empty());
    assert_eq!(licenses.third_party, None);
}

#[test]
fn licenses_hold_the_third_party_file_when_bundled() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("licenses")).unwrap();
    std::fs::write(
        dir.path().join("licenses/THIRD_PARTY_LICENSES.txt"),
        "serde: MIT",
    )
    .unwrap();

    let licenses = get_licenses(dir.path()).unwrap();

    assert_eq!(licenses.third_party.as_deref(), Some("serde: MIT"));
}

#[tokio::test]
async fn capabilities_say_what_this_machine_can_do() {
    let app = testing::app();

    let with_tray = get_capabilities(&app.state, true).await;
    assert!(with_tray.keychain);
    assert!(!with_tray.keychain_entry_missing);
    assert!(with_tray.tray);

    let without = get_capabilities(&app.state, false).await;
    assert!(!without.tray);
    // Whether the CLIs are installed here or not, each has an answer.
    let git = matches!(without.git, GitSupport::Available { .. });
    assert_eq!(git, homeport_git::cli::detect().await.is_ok());
}

#[tokio::test]
async fn starting_over_is_refused_while_the_key_works() {
    let app = testing::app();

    let refused = keychain_start_over(&app.state).await.unwrap_err();

    assert_eq!(refused.code, ErrorCode::Validation);
    assert!(app.state.secrets.is_available());
}

#[test]
fn text_files_are_read_back_as_written() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir
        .path()
        .join("compose.yaml")
        .to_string_lossy()
        .into_owned();

    write_text_file(path.clone(), "services: {}\n".to_owned()).unwrap();

    assert_eq!(read_text_file(path).unwrap(), "services: {}\n");
}

#[test]
fn a_missing_text_file_is_an_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir
        .path()
        .join("missing.pem")
        .to_string_lossy()
        .into_owned();

    assert!(read_text_file(path).is_err());
}

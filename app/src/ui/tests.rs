//! The window itself: what it shows when the app didn't start, and the
//! messages that reach it from outside a page.

use std::{io, path::PathBuf, sync::Arc};

use homeport_core::{
    error::{AppError, ErrorCode},
    messages as msg,
};
use homeport_settings::Theme as Setting;

use super::{theme::Theme, Failure, Homeport, Message, Phase};
use crate::{
    platform::fake::{Asked, Fake},
    t,
    test_support::{
        snapshot::{assert_snapshot, in_language},
        stage::Stage,
        tasks,
        window::drive,
    },
    ui::{
        shell,
        widgets::feedback::{
            error_details,
            error_message::Event,
            toast::{self, Kind},
            Surfaced,
        },
    },
};

fn failed() -> Homeport {
    let error = AppError::new(ErrorCode::Io, msg::IO_FAILED)
        .with_hint(msg::UNREADABLE_DATA)
        .with_detail("unable to open database file (code 14)");
    let mut window = Homeport::new(Fake::new().platform());
    let path = PathBuf::from("/home/ada/.local/share/homeport/homeport.db");
    window.phase = Phase::Failed(Failure::Database(Box::new(error), path));
    window
}

#[test]
fn the_theme_follows_the_settings_live_and_the_os_while_it_is_followed() {
    let mut window = Homeport::new(Fake::new().platform());
    assert_eq!(window.theme(), None, "iced follows the OS until told");
    let _ = window.update(Message::ThemeChanged(Setting::Dark));
    assert_eq!(window.theme(), Some(Theme::Dark));
    let _ = window.update(Message::ThemeChanged(Setting::Light));
    assert_eq!(window.theme(), Some(Theme::Light));
    // The OS's preference changes nothing for a user who chose…
    let _ = window.update(Message::SystemTheme(iced::theme::Mode::Dark));
    assert_eq!(window.shown(), Theme::Light);
    // … and is what shows for one who follows it.
    let _ = window.update(Message::ThemeChanged(Setting::System));
    assert_eq!(window.theme(), None);
    assert_eq!(window.shown(), Theme::Dark);
    let _ = window.update(Message::SystemTheme(iced::theme::Mode::Light));
    assert_eq!(window.shown(), Theme::Light);
}

#[test]
fn a_failed_start_is_shown_with_the_error_widgets() {
    let window = failed();
    assert_snapshot(
        "startup-failed-en",
        "en",
        &Theme::Light,
        (640.0, 300.0),
        || window.view(),
    );
}

#[test]
fn the_failed_start_leads_to_its_details_which_escape_closes() {
    let _english = in_language("en");
    let mut window = failed();
    let details = t!("common.details");
    let messages = drive(
        (640.0, 400.0),
        || window.view(),
        |page| {
            page.click(details.as_str()).expect("a Details button");
        },
    );
    assert!(matches!(
        messages.as_slice(),
        [Message::Failure(Event::Details)]
    ));
    for message in messages {
        let _ = window.update(message);
    }
    assert!(window.details.is_some());
    let mut stage = Stage::new((900.0, 700.0));
    let messages = stage.frame(window.view(), |frame| {
        assert!(frame.focused().is_some(), "the dialog has focus inside");
        frame.tap_key(iced::keyboard::key::Named::Escape);
    });
    for message in messages {
        let _ = window.update(message);
    }
    assert!(window.details.is_none());
}

#[test]
fn tab_moves_focus_and_a_failure_s_toast_opens_its_details() {
    let mut window = Homeport::new(Fake::new().platform());
    assert!(!tasks::is_none(window.update(Message::Tab(false))));
    assert!(!tasks::is_none(window.update(Message::Tab(true))));
    let error = AppError::new(ErrorCode::Io, msg::IO_FAILED);
    let id = window
        .toasts
        .failure(crate::ui::widgets::feedback::Surfaced::at(
            error,
            Some("hosts_list"),
            jiff::Timestamp::now(),
        ));
    window.toasts.say(Kind::Info, || "said".to_owned());
    let _ = window.update(Message::Toast(
        crate::ui::widgets::feedback::toast::Message::Details(id),
    ));
    assert!(window.details.is_some());
    assert_eq!(window.toasts.visible().len(), 1);
}

/// A window on a system that only notes what it is asked: no browser,
/// no file manager, and folders that go with the test.
fn on_a_fake(mut window: Homeport) -> (Homeport, Arc<Fake>) {
    let os = Fake::new();
    window.platform = os.platform();
    (window, os)
}

/// Runs what an `update` returned and hands its messages back to the window.
fn settle(window: &mut Homeport, task: iced::Task<Message>) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    for message in runtime.block_on(tasks::messages(task)) {
        let _ = window.update(message);
    }
}

#[test]
fn a_link_a_page_asked_for_goes_to_the_os_and_a_failure_is_said() {
    let _english = in_language("en");
    let (mut window, os) = on_a_fake(Homeport::new(Fake::new().platform()));
    let task = window.asked(shell::Event::OpenUrl("http://localhost:8080".to_owned()));
    assert!(
        os.took().is_empty(),
        "nothing is opened on the window's thread"
    );
    settle(&mut window, task);
    assert_eq!(os.took(), [Asked::Opened("http://localhost:8080".into())]);
    assert!(window.toasts.is_empty(), "a link that opened says nothing");

    // No browser answers: the toast names the link and what to do.
    os.refuse(io::ErrorKind::NotFound);
    let task = window.asked(shell::Event::OpenUrl("https://example.com".to_owned()));
    settle(&mut window, task);
    let [toast] = window.toasts.visible() else {
        panic!("one toast");
    };
    assert_eq!(toast.kind, Kind::Error);
    assert_eq!(
        toast.text(),
        (
            "https://example.com can't be opened.".to_owned(),
            Some("Copy the address and open it in your browser instead.".to_owned())
        )
    );
}

#[test]
fn the_failed_start_opens_the_folder_its_database_is_in() {
    let _english = in_language("en");
    let (mut window, os) = on_a_fake(failed());
    let database = os.dir().join("data/homeport.db");
    let Phase::Failed(Failure::Database(_, path)) = &mut window.phase else {
        panic!("a database that didn't open");
    };
    path.clone_from(&database);
    let open = t!("app.startup.openFolder");
    let messages = drive(
        (640.0, 400.0),
        || window.view(),
        |page| {
            page.click(open.as_str()).expect("an Open folder button");
        },
    );
    assert!(matches!(messages.as_slice(), [Message::OpenDataFolder]));
    for message in messages {
        let task = window.update(message);
        settle(&mut window, task);
    }
    let folder = database.parent().unwrap();
    assert_eq!(os.took(), [Asked::Opened(folder.into())]);
    assert!(folder.is_dir(), "a folder that isn't there yet is made");
    assert!(window.toasts.is_empty());

    os.refuse(io::ErrorKind::NotFound);
    let task = window.update(Message::OpenDataFolder);
    settle(&mut window, task);
    let [toast] = window.toasts.visible() else {
        panic!("one toast");
    };
    assert_eq!(
        toast.text().0,
        format!("{} can't be opened.", folder.display())
    );
}

#[test]
fn a_start_that_failed_otherwise_offers_no_folder() {
    let _english = in_language("en");
    let error = AppError::new(ErrorCode::Io, msg::IO_FAILED);
    let (mut window, os) = on_a_fake(Homeport::new(Fake::new().platform()));
    window.phase = Phase::Failed(Failure::Other(Box::new(error)));
    let open = t!("app.startup.openFolder");
    let mut found = true;
    let _ = drive(
        (640.0, 400.0),
        || window.view(),
        |page| found = page.click(open.as_str()).is_ok(),
    );
    assert!(!found, "there is no database to show the folder of");
    assert!(tasks::is_none(window.update(Message::OpenDataFolder)));
    assert!(os.took().is_empty());
}

#[test]
fn the_details_dialog_opens_the_log_folder_and_says_when_it_cannot() {
    let _english = in_language("en");
    let (mut window, os) = on_a_fake(Homeport::new(Fake::new().platform()));
    let error = AppError::new(ErrorCode::Io, msg::IO_FAILED);
    let surfaced = Surfaced::at(error, Some("hosts_list"), jiff::Timestamp::UNIX_EPOCH);
    window.details = Some(error_details::Details::new(surfaced));
    let open = t!("ui.errorDetails.openLogFolder");
    let press = |window: &Homeport| {
        drive(
            (900.0, 700.0),
            || window.view(),
            |page| {
                page.click(open.as_str())
                    .expect("an Open log folder button");
            },
        )
    };
    for message in press(&window) {
        let task = window.update(message);
        settle(&mut window, task);
    }
    let logs = os.dir().join("logs");
    assert_eq!(os.took(), [Asked::Opened(logs.clone().into())]);
    assert!(logs.is_dir());

    // The file manager doesn't answer: the dialog says so, in place.
    os.refuse(io::ErrorKind::NotFound);
    for message in press(&window) {
        let task = window.update(message);
        settle(&mut window, task);
    }
    let refused = format!("{} can't be opened.", logs.display());
    let mut shown = false;
    let _ = drive(
        (900.0, 700.0),
        || window.view(),
        |page| shown = page.find(refused.as_str()).is_ok(),
    );
    assert!(shown, "the failure shows under the button");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_pages_know_the_theme_on_screen_whoever_chose_it() {
    let crate::app::testing::TestApp {
        state,
        events: _events,
        dir: _dir,
    } = crate::app::testing::app();
    let mut window = Homeport::new(Fake::new().platform());
    let _ = window.update(Message::Started(Ok(Arc::new(state))));
    let shown = |window: &Homeport| window.shell.as_ref().map(|shell| shell.world().theme);
    // The Settings follow the OS, which is dark.
    let _ = window.update(Message::SystemTheme(iced::theme::Mode::Dark));
    assert_eq!(shown(&window), Some(Theme::Dark));
    let _ = window.update(Message::ThemeChanged(Setting::Light));
    assert_eq!(shown(&window), Some(Theme::Light));
    let _ = window.update(Message::ThemeChanged(Setting::System));
    assert_eq!(shown(&window), Some(Theme::Dark));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_toast_s_button_goes_where_its_action_said_and_the_toast_goes() {
    let _english = in_language("en");
    let crate::app::testing::TestApp {
        state,
        events: _events,
        dir: _dir,
    } = crate::app::testing::app();
    let mut window = Homeport::new(Fake::new().platform());
    let _ = window.update(Message::Started(Ok(Arc::new(state))));
    let images = shell::router::Request::to(shell::router::Route::Images);
    let label = toast::Said::new(|| "Open images".to_owned());
    let said = toast::Said::new(|| "Committed to web:backup".to_owned());
    let _ = window.asked(shell::Event::SaidGoing(Kind::Success, said, label, images));
    // Drawn with its button, which the pointer or the keyboard presses.
    let pressed = drive(
        (640.0, 400.0),
        || window.toasts.view().unwrap(),
        |corner| {
            corner.click("Open images").expect("the action's button");
        },
    );
    assert_eq!(pressed.len(), 1);
    for message in pressed {
        let _ = window.update(Message::Toast(message));
    }
    assert!(window.toasts.is_empty(), "the toast goes once pressed");
    let route = window
        .shell
        .as_ref()
        .map(|shell| shell.location().route.clone());
    assert_eq!(route, Some(shell::router::Route::Images));
}

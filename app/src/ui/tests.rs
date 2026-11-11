//! The window itself: what it shows when the app didn't start, and the
//! messages that reach it from outside a page.

use std::path::PathBuf;

use homeport_core::{
    error::{AppError, ErrorCode},
    messages as msg,
};
use homeport_settings::Theme as Setting;

use super::{theme::Theme, Failure, Homeport, Message, Phase};
use crate::{
    t,
    test_support::{
        snapshot::{assert_snapshot, in_language},
        stage::Stage,
        tasks,
        window::drive,
    },
    ui::widgets::feedback::{error_message::Event, toast::Kind},
};

fn failed() -> Homeport {
    let error = AppError::new(ErrorCode::Io, msg::IO_FAILED)
        .with_hint(msg::UNREADABLE_DATA)
        .with_detail("unable to open database file (code 14)");
    let mut window = Homeport::new();
    let path = PathBuf::from("/home/ada/.local/share/homeport/homeport.db");
    window.phase = Phase::Failed(Failure::Database(Box::new(error), path));
    window
}

#[test]
fn the_theme_follows_the_settings_live_and_the_os_while_it_is_followed() {
    let mut window = Homeport::new();
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
    let mut window = Homeport::new();
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

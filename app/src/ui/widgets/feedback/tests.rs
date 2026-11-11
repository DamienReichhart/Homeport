//! The feedback widgets, pressed and typed in as a user does.

use homeport_core::error::{AppError, ErrorCode};
use homeport_storage::messages as storage;
use iced::keyboard::key::Named;

use super::{
    confirm::{self, confirm_dialog, Ask, Outcome},
    error_details::{self, Details, Effect, Target},
    error_message::{error_message, Event},
    surfaced::{code_name, About, Surfaced},
    toast::{self, Kind, Toasts},
};
use crate::{
    i18n,
    test_support::{
        snapshot::in_language,
        window::{drive, window},
    },
    ui::theme::Element,
};

const SIZE: (f32, f32) = (800.0, 600.0);

/// Every code, read from where they are declared: a new one is tested the
/// day it is added.
fn codes() -> Vec<ErrorCode> {
    let source = include_str!("../../../../../crates/foundation/core/src/error/code.rs");
    let body = source.split_once("pub enum ErrorCode {").unwrap().1;
    let body = body.split_once("\n}").unwrap().0;
    body.lines()
        .map(str::trim)
        .filter_map(|line| line.strip_suffix(','))
        .filter(|name| name.chars().all(char::is_alphanumeric))
        .map(|name| {
            let mut snake = String::new();
            for letter in name.chars() {
                if letter.is_uppercase() && !snake.is_empty() {
                    snake.push('_');
                }
                snake.extend(letter.to_lowercase());
            }
            serde_json::from_value(serde_json::Value::String(snake))
                .unwrap_or_else(|e| panic!("{name}: {e}"))
        })
        .collect()
}

fn locked(code: ErrorCode) -> AppError {
    AppError::new(code, storage::DB_LOCKED)
        .with_hint(storage::DB_LOCKED_HINT)
        .with_command("ls -la")
        .with_detail("database is locked (code 5)")
}

fn surfaced() -> Surfaced {
    Surfaced::at(
        locked(ErrorCode::DbLocked),
        Some("hosts_list"),
        "2026-03-04T05:06:07Z".parse().unwrap(),
    )
}

fn about() -> About {
    About {
        name: "Homeport".to_owned(),
        version: "1.2.0".to_owned(),
        os: "linux".to_owned(),
        arch: "x86_64".to_owned(),
    }
}

#[test]
fn every_code_is_said_with_its_message_and_hint_in_the_apps_language() {
    let codes = codes();
    assert!(codes.len() > 60, "{} codes read", codes.len());
    let _french = in_language("fr");
    let language = i18n::current();
    let message = language.text("errors.storage.dbLocked", None);
    let hint = language.text("errors.storage.dbLocked.hint", None);
    assert_ne!(message, locked(ErrorCode::Unknown).message);
    for code in codes {
        let error = locked(code);
        let view: Element<'_, Event> = error_message(&error, true, |event| event);
        let mut place = window(SIZE, view);
        assert!(place.find(message.as_str()).is_ok(), "{code:?}: message");
        assert!(place.find(hint.as_str()).is_ok(), "{code:?}: hint");

        let shown = Surfaced::at(error, Some("hosts_list"), jiff::Timestamp::UNIX_EPOCH);
        let details = Details::new(shown);
        let mut dialog = window(SIZE, details.view(None));
        assert!(dialog.find(message.as_str()).is_ok(), "{code:?}: message");
        assert!(dialog.find(hint.as_str()).is_ok(), "{code:?}: hint");
        assert!(dialog.find(code_name(code).as_str()).is_ok(), "{code:?}");
    }
}

#[test]
fn an_error_in_place_offers_details_retry_and_its_commands() {
    let _english = in_language("en");
    let error = locked(ErrorCode::DbLocked);
    let press = |label: &'static str, retry: bool| {
        drive(
            SIZE,
            || error_message(&error, retry, |event| event),
            |place| {
                let _ = place.click(label);
            },
        )
    };
    assert_eq!(press("Details", true), [Event::Details]);
    assert_eq!(press("Retry", true), [Event::Retry]);
    assert_eq!(press("Copy", true), [Event::Copy("ls -la".to_owned())]);
    // Without a way to redo the work there is no Retry to press.
    assert_eq!(press("Retry", false), []);
}

#[test]
fn the_details_dialog_copies_the_report_and_the_commands() {
    let _english = in_language("en");
    let about = about();
    let mut details = Details::new(surfaced());
    let press = |details: &Details, label: &'static str| {
        drive(
            SIZE,
            || details.view(Some(&about)).into(),
            |dialog| {
                dialog.click(label).unwrap();
            },
        )
    };

    let pressed = press(&details, "Copy report");
    assert_eq!(pressed, [error_details::Message::Copy(Target::Report)]);
    let effect = details.update(pressed[0].clone(), Some(&about));
    let report = surfaced().report(Some(&about));
    assert!(report.contains("Command: hosts_list") && report.contains("database is locked"));
    assert_eq!(effect, Effect::Copy(report));
    // The button says so, until its time is up.
    assert!(window(SIZE, details.view(Some(&about)))
        .find("Copied")
        .is_ok());
    details.update(error_details::Message::CopyShown, Some(&about));

    let pressed = press(&details, "Copy");
    assert_eq!(pressed, [error_details::Message::Copy(Target::Command(0))]);
    let effect = details.update(pressed[0].clone(), Some(&about));
    assert_eq!(effect, Effect::Copy("ls -la".to_owned()));

    let pressed = press(&details, "Open log folder");
    assert_eq!(
        details.update(pressed[0].clone(), None),
        Effect::OpenLogFolder
    );
    // Why the folder didn't open is said in the dialog.
    let refused = AppError::new(ErrorCode::Io, storage::DB_LOCKED_HINT);
    details.update(
        error_details::Message::LogFolderOpened(Err(refused.clone())),
        None,
    );
    assert!(window(SIZE, details.view(None))
        .find(refused.message.as_str())
        .is_ok());
}

#[test]
fn escape_closes_the_details_dialog() {
    let details = Details::new(surfaced());
    let pressed = drive(
        SIZE,
        || details.view(None).into(),
        |dialog| {
            dialog.tap_key(Named::Escape);
        },
    );
    assert_eq!(pressed, [error_details::Message::Close]);
}

#[test]
fn a_destructive_confirmation_needs_the_name_typed() {
    let _english = in_language("en");
    let ask = Ask::new("Remove the volume?", "Its data is lost.", "Remove")
        .destructive()
        .typed("prod-db");
    let mut state = confirm::State::default();
    let remove = |state: &confirm::State| {
        drive(
            SIZE,
            || confirm_dialog(&ask, state).into(),
            |dialog| {
                dialog.click("Remove").unwrap();
            },
        )
    };
    // Nothing typed: the button is there and does nothing.
    assert_eq!(remove(&state), []);
    // The field has focus as the dialog opens: typing goes to it.
    for letter in ["p", "r", "o", "d"] {
        let typed = drive(
            SIZE,
            || confirm_dialog(&ask, &state).into(),
            |dialog| {
                dialog.typewrite(letter);
            },
        );
        for message in typed {
            state.update(message, &ask);
        }
    }
    assert_eq!(remove(&state), [], "half the name is not the name");
    state.update(confirm::Message::Input("prod-db".to_owned()), &ask);
    let pressed = remove(&state);
    assert_eq!(pressed, [confirm::Message::Confirm]);
    assert_eq!(state.update(pressed[0].clone(), &ask), Outcome::Confirmed);
    // Busy: Cancel and Escape wait for the action to end.
    let escaped = drive(
        SIZE,
        || confirm_dialog(&ask, &state).into(),
        |dialog| {
            dialog.tap_key(Named::Escape);
            let _ = dialog.click("Cancel");
        },
    );
    assert_eq!(escaped, []);
}

#[test]
fn a_confirmation_opens_on_cancel_and_escape_cancels() {
    let _english = in_language("en");
    let ask = Ask::new("Stop it?", "It can be started again.", "Stop");
    let state = confirm::State::default();
    let keys = |keys: &'static [Named]| {
        drive(
            SIZE,
            || confirm_dialog(&ask, &state).into(),
            |dialog| {
                for key in keys {
                    dialog.tap_key(*key);
                }
            },
        )
    };
    // Enter presses what has focus, which is Cancel: the safe one.
    assert_eq!(keys(&[Named::Enter]), [confirm::Message::Cancel]);
    // Tab goes to the action, and round to Cancel again.
    assert_eq!(
        keys(&[Named::Tab, Named::Enter]),
        [confirm::Message::Confirm]
    );
    assert_eq!(
        keys(&[Named::Tab, Named::Tab, Named::Enter]),
        [confirm::Message::Cancel]
    );
    assert_eq!(keys(&[Named::Escape]), [confirm::Message::Cancel]);
}

#[test]
fn a_failures_toast_opens_its_details() {
    let _english = in_language("en");
    let mut toasts = Toasts::default();
    toasts.say(Kind::Success, || "Started web".to_owned());
    let failed = toasts.failure(surfaced());
    let pressed = drive(
        SIZE,
        || toasts.view().unwrap(),
        |corner| {
            assert!(corner.find("Started web").is_ok());
            corner.click("Details").unwrap();
        },
    );
    assert!(pressed.contains(&toast::Message::Details(failed)));
    let opened = pressed
        .into_iter()
        .filter_map(|message| toasts.update(message))
        .next();
    assert_eq!(opened, Some(surfaced()));
    assert_eq!(toasts.visible().len(), 1);
}

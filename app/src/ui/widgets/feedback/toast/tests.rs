//! How toasts queue, wait, and go.

use homeport_core::{
    error::{AppError, ErrorCode},
    messages as msg,
};

use super::*;

fn failure() -> Surfaced {
    let error = AppError::new(ErrorCode::Io, msg::IO_FAILED).with_hint(msg::UNREADABLE_DATA);
    Surfaced::at(error, Some("containers_act"), jiff::Timestamp::UNIX_EPOCH)
}

fn ids(toasts: &Toasts) -> Vec<u64> {
    toasts.visible().iter().map(|toast| toast.id).collect()
}

#[test]
fn the_newest_three_show_and_the_others_wait_their_turn() {
    let mut toasts = Toasts::default();
    for n in 0..5 {
        toasts.say(Kind::Info, move || format!("toast {n}"));
    }
    assert_eq!(ids(&toasts), [2, 3, 4]);
    // Those on screen go together; the two that waited then show, with
    // their whole time ahead of them.
    toasts.advance(LIFETIME);
    assert_eq!(ids(&toasts), [0, 1]);
    toasts.advance(LIFETIME - TICK);
    assert_eq!(ids(&toasts), [0, 1]);
    toasts.advance(TICK);
    assert!(toasts.is_empty());
}

#[test]
fn a_failure_stays_longer_than_the_rest() {
    let mut toasts = Toasts::default();
    let failed = toasts.failure(failure());
    toasts.say(Kind::Success, || "done".to_owned());
    toasts.advance(LIFETIME);
    assert_eq!(ids(&toasts), [failed]);
    toasts.advance(ERROR_LIFETIME - LIFETIME - TICK);
    assert_eq!(ids(&toasts), [failed]);
    toasts.advance(TICK);
    assert!(toasts.is_empty());
}

#[test]
fn the_pointer_resting_on_them_stops_the_clocks() {
    let mut toasts = Toasts::default();
    let id = toasts.say(Kind::Success, || "done".to_owned());
    toasts.update(Message::Entered(id));
    toasts.advance(LIFETIME * 3);
    assert_eq!(ids(&toasts), [id]);
    toasts.update(Message::Left(id));
    toasts.advance(LIFETIME);
    assert!(toasts.is_empty());
    // Dismissing the last one under the pointer leaves no stopped clock.
    toasts.say(Kind::Info, || "one".to_owned());
    toasts.update(Message::Entered(1));
    toasts.update(Message::Dismiss(1));
    toasts.say(Kind::Info, || "two".to_owned());
    toasts.advance(LIFETIME);
    assert!(toasts.is_empty());
}

#[test]
fn a_toast_that_goes_from_under_the_pointer_does_not_stop_the_others() {
    let mut toasts = Toasts::default();
    let first = toasts.say(Kind::Info, || "one".to_owned());
    let second = toasts.failure(failure());
    // The pointer rests on the failure, whose Details are then asked
    // for: it goes, and the pointer never leaves it.
    toasts.update(Message::Entered(second));
    assert!(toasts.update(Message::Details(second)).is_some());
    toasts.advance(LIFETIME);
    assert!(toasts.is_empty(), "the one that stayed went in its time");
    // From one toast to its neighbour, in either order of the two
    // messages: the clocks stay stopped.
    let a = toasts.say(Kind::Info, || "a".to_owned());
    let b = toasts.say(Kind::Info, || "b".to_owned());
    toasts.update(Message::Entered(a));
    toasts.update(Message::Entered(b));
    toasts.update(Message::Left(a));
    toasts.advance(LIFETIME);
    assert_eq!(ids(&toasts), [a, b]);
    let _ = first;
}

#[test]
fn the_clocks_count_the_time_that_passed_not_the_ticks() {
    let mut toasts = Toasts::default();
    let start = Instant::now();
    let id = toasts.say(Kind::Info, || "one".to_owned());
    // The first reading starts the clock.
    toasts.update(Message::Tick(start));
    // One late tick, three seconds on: three seconds are gone, not a
    // tenth of one.
    toasts.update(Message::Tick(start + Duration::from_secs(3)));
    assert_eq!(ids(&toasts), [id]);
    toasts.update(Message::Tick(start + LIFETIME - TICK));
    assert_eq!(ids(&toasts), [id]);
    toasts.update(Message::Tick(start + LIFETIME));
    assert!(toasts.is_empty());
    // A new toast starts a new clock: the wait since the last one is
    // not taken from it.
    let id = toasts.say(Kind::Info, || "two".to_owned());
    toasts.update(Message::Tick(start + Duration::from_secs(60)));
    assert_eq!(ids(&toasts), [id]);
}

#[test]
fn details_hands_the_failure_over_and_the_toast_goes() {
    let mut toasts = Toasts::default();
    let said = toasts.say(Kind::Info, || "one".to_owned());
    let failed = toasts.failure_of(|| "Could not start web".to_owned(), failure());
    assert_eq!(toasts.update(Message::Details(said)), None);
    assert_eq!(toasts.update(Message::Details(failed)), Some(failure()));
    assert!(toasts.is_empty());
}

#[test]
fn a_failure_says_its_message_and_its_hint_or_what_failed_and_its_message() {
    let _english = crate::test_support::snapshot::in_language("en");
    let mut toasts = Toasts::default();
    toasts.failure(failure());
    toasts.failure_of(|| "Could not start web".to_owned(), failure());
    let error = failure().error;
    let hint = error.hint.as_ref().map(|hint| i18n::errors::hint(hint));
    let message = i18n::errors::message(&error);
    assert_eq!(toasts.visible()[0].text(), (message.clone(), hint));
    assert_eq!(
        toasts.visible()[1].text(),
        ("Could not start web".to_owned(), Some(message))
    );
}

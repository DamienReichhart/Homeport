//! The kit on a tree that lasts from one view to the next, as the window
//! keeps it: what `tests.rs`, which draws every step on a new tree, cannot
//! see. Scroll offsets and focus that survive a dialog, focus rings, the
//! tooltip's clock.

use std::{thread::sleep, time::Duration};

use iced::{keyboard::key::Named, Point};

use super::{controls_page, Gallery, Message};
use crate::{
    test_support::{snapshot::in_language, stage::Stage},
    ui::widgets::{
        feedback::{
            confirm::{self, Ask},
            confirm_dialog,
            error_details::Details,
            Surfaced,
        },
        overlay::{self, TOOLTIP_DELAY},
    },
};

const WINDOW: (f32, f32) = (1100.0, 1400.0);

fn feed(gallery: &mut Gallery, messages: Vec<Message>) {
    for message in messages {
        let _ = gallery.update(message);
    }
}

#[test]
fn a_dialog_and_a_toast_leave_the_page_scrolled_and_focus_where_it_was() {
    let _english = in_language("en");
    let mut gallery = Gallery::default();
    // Short enough to scroll, tall enough to keep the buttons in view.
    let mut stage = Stage::new((1100.0, 420.0));
    let mut offset = None;
    let messages = stage.frame(gallery.view(), |frame| {
        frame.point_at(Point::new(550.0, 200.0));
        frame.scroll(2.0);
        offset = frame.scrolled().first().copied();
        frame.click("Confirm");
        assert!(frame.is_focused("Confirm"));
    });
    let offset = offset.expect("the page scrolls");
    assert!(offset.y > 0.0, "the page is scrolled: {offset:?}");
    feed(&mut gallery, messages);
    assert!(gallery.confirm.is_some());

    // The dialog is up: the page under it hasn't moved, Cancel has focus.
    let messages = stage.frame(gallery.view(), |frame| {
        assert_eq!(frame.scrolled().first(), Some(&offset));
        assert!(frame.is_focused("Cancel"));
        frame.tap_key(Named::Escape);
    });
    feed(&mut gallery, messages);
    assert!(gallery.confirm.is_none());

    // It is gone: the same offset, and focus back on what opened it.
    let messages = stage.frame(gallery.view(), |frame| {
        assert_eq!(frame.scrolled().first(), Some(&offset));
        assert!(frame.is_focused("Confirm"));
        frame.click("Success toast");
    });
    feed(&mut gallery, messages);
    assert!(!gallery.toasts.is_empty());

    // The first toast shows: still the same offset and the same focus.
    stage.frame(gallery.view(), |frame| {
        assert_eq!(frame.scrolled().first(), Some(&offset));
        assert!(frame.is_focused("Success toast"));
    });
}

#[test]
fn a_dialog_that_takes_the_place_of_another_is_given_focus() {
    let _english = in_language("en");
    let ask = Ask::new("Remove db-data?", "It can't be brought back.", "Remove");
    let state = confirm::State::default();
    let details = Details::new(Surfaced::new(super::failure(), "volumes_remove"));
    let about = super::about();
    let mut stage = Stage::new((900.0, 700.0));
    let page = || iced::widget::Space::new();
    stage.frame(
        overlay::over(
            page(),
            Some(confirm_dialog(&ask, &state).map(Message::Confirm)),
        ),
        |frame| assert!(frame.is_focused("Cancel")),
    );
    stage.frame(
        overlay::over(
            page(),
            Some(details.view(Some(&about)).map(Message::Details)),
        ),
        |frame| {
            assert!(
                frame.focused().is_some(),
                "the dialog that replaced the first has focus inside"
            );
        },
    );
}

#[test]
fn a_click_gives_focus_without_a_ring_and_the_keyboard_shows_it() {
    let _english = in_language("en");
    let gallery = controls_page();
    let mut stage = Stage::new(WINDOW);
    stage.frame(gallery.view(), |frame| {
        frame.click("Secondary");
        assert!(frame.is_focused("Secondary"));
        assert!(!frame.shows_focus(), "a click shows no ring");
        frame.tab();
        assert!(frame.focused().is_some() && !frame.is_focused("Secondary"));
        assert!(frame.shows_focus(), "Tab shows the ring");
        frame.click("Ghost");
        assert!(frame.is_focused("Ghost") && !frame.shows_focus());
        // A key while it has focus: the ring is back.
        frame.tap_key(Named::ArrowDown);
        assert!(frame.shows_focus());
    });
}

#[test]
fn a_tooltip_waits_under_the_pointer_shows_at_once_for_the_keyboard_and_goes_on_a_press() {
    let _english = in_language("en");
    const SAYS: &str = "Shown on hover and on focus";
    let gallery = controls_page();
    let mut stage = Stage::new(WINDOW);
    stage.frame(gallery.view(), |frame| {
        // The button with the tooltip is the one after "Popover".
        let popover = frame.find("Popover").expect("the popover's button");
        let anchor = Point::new(popover.x + popover.width + 40.0, popover.center_y());
        assert!(frame.find(SAYS).is_none());
        frame.point_at(anchor);
        frame.redraw();
        assert!(frame.find(SAYS).is_none(), "not before the delay");
        sleep(TOOLTIP_DELAY + Duration::from_millis(40));
        frame.redraw();
        assert!(frame.find(SAYS).is_some(), "after the delay");
        // A press sends it away, and focus given by that press doesn't
        // bring it back, however long the pointer stays.
        frame.press();
        frame.redraw();
        assert!(frame.find(SAYS).is_none(), "a press closes it");
        sleep(TOOLTIP_DELAY + Duration::from_millis(40));
        frame.redraw();
        assert!(frame.find(SAYS).is_none(), "it stays closed after a click");
        // Away and back: it waits again.
        frame.point_at(Point::new(5.0, 5.0));
        frame.click("Popover");
        frame.tap_key(Named::Escape);
    });
    let mut stage = Stage::new(WINDOW);
    stage.frame(gallery.view(), |frame| {
        // Focus from the keyboard shows it at once, with no pointer near.
        frame.click("Popover");
        frame.tap_key(Named::Escape);
        assert!(frame.find(SAYS).is_none());
    });
}

#[test]
fn a_tooltip_shows_at_once_when_tab_reaches_its_button() {
    let _english = in_language("en");
    const SAYS: &str = "Shown on hover and on focus";
    let mut gallery = controls_page();
    let mut stage = Stage::new(WINDOW);
    let messages = stage.frame(gallery.view(), |frame| frame.click("Popover"));
    feed(&mut gallery, messages);
    let messages = stage.frame(gallery.view(), |frame| frame.tap_key(Named::Escape));
    feed(&mut gallery, messages);
    assert!(!gallery.popover);
    stage.frame(gallery.view(), |frame| {
        assert!(frame.is_focused("Popover"));
        assert!(frame.find(SAYS).is_none());
        frame.point_at(Point::new(5.0, 5.0));
        frame.tab();
        assert!(frame.find(SAYS).is_some(), "Tab reached it: shown at once");
        frame.tap_key(Named::Escape);
        frame.redraw();
        assert!(frame.find(SAYS).is_none(), "Escape closes it");
    });
}

#[test]
fn a_button_in_a_field_leaves_focus_in_the_field() {
    let _english = in_language("en");
    let mut gallery = controls_page();
    let mut stage = Stage::new(WINDOW);
    let mut field = None;
    let messages = stage.frame(gallery.view(), |frame| {
        // The password field is under its label; its eye is at its end.
        let label = frame.find("Password").expect("the password field");
        let inside = Point::new(label.x + 40.0, label.y + label.height + 22.0);
        frame.point_at(inside);
        frame.press();
        field = frame.focused();
        let field = field.expect("the field has focus");
        frame.point_at(Point::new(field.x + field.width - 14.0, field.center_y()));
        frame.press();
        assert_eq!(frame.focused(), Some(field), "focus stays in the field");
        assert!(!frame.shows_focus());
    });
    assert!(
        messages.iter().any(|m| matches!(m, Message::Reveal)),
        "the eye was pressed: {messages:?}"
    );
    feed(&mut gallery, messages);
    // What is typed next goes to the field, a space included.
    let messages = stage.frame(gallery.view(), |frame| {
        assert_eq!(frame.focused(), field);
        frame.typewrite("a b");
    });
    let typed: Vec<_> = messages
        .iter()
        .filter(|m| matches!(m, Message::Secret(_)))
        .collect();
    assert_eq!(typed.len(), 3, "{messages:?}");
    assert!(!messages.iter().any(|m| matches!(m, Message::Reveal)));
}

#[test]
fn a_press_outside_an_open_menu_or_select_only_closes_it() {
    let _english = in_language("en");
    let mut gallery = controls_page();
    let mut stage = Stage::new(WINDOW);
    let messages = stage.frame(gallery.view(), |frame| frame.click("Actions"));
    feed(&mut gallery, messages);
    assert!(gallery.menu.is_open());
    let messages = stage.frame(gallery.view(), |frame| frame.click("Logs"));
    feed(&mut gallery, messages);
    assert!(!gallery.menu.is_open());
    assert_eq!(gallery.tab, 0, "the tab under the pointer wasn't pressed");

    let messages = stage.frame(gallery.view(), |frame| frame.click("Docker Engine"));
    feed(&mut gallery, messages);
    assert!(gallery.select.is_open());
    let messages = stage.frame(gallery.view(), |frame| frame.click("Logs"));
    feed(&mut gallery, messages);
    assert!(!gallery.select.is_open());
    assert_eq!(gallery.tab, 0);
    // Closed, the same press does what it is for.
    let messages = stage.frame(gallery.view(), |frame| frame.click("Logs"));
    feed(&mut gallery, messages);
    assert_eq!(gallery.tab, 1);
}

#[test]
fn escape_in_a_submenu_closes_the_whole_menu() {
    let _english = in_language("en");
    let mut gallery = controls_page();
    let mut stage = Stage::new(WINDOW);
    let messages = stage.frame(gallery.view(), |frame| frame.click("Actions"));
    feed(&mut gallery, messages);
    let messages = stage.frame(gallery.view(), |frame| {
        // End is Remove, Up is Copy; the arrow towards the end opens it.
        frame.tap_key(Named::End);
        frame.tap_key(Named::ArrowUp);
        frame.tap_key(Named::ArrowRight);
    });
    feed(&mut gallery, messages);
    let messages = stage.frame(gallery.view(), |frame| {
        assert!(frame.find("Id").is_some(), "the submenu is open");
        frame.tap_key(Named::Escape);
    });
    feed(&mut gallery, messages);
    assert!(!gallery.menu.is_open());
    // One ring at a time while it was open: see the menu's snapshot.
}

/// A search field in a dialog or a panel: Escape empties a field that
/// holds something and leaves the dialog open; the next one closes it.
/// The dialog took the key before the field ever saw it.
#[test]
fn a_filled_search_field_takes_the_first_escape_in_a_dialog_and_in_a_panel() {
    use iced::widget::text;

    use crate::ui::{theme::Element, widgets::form::search};

    #[derive(Debug, Clone, PartialEq)]
    enum Sent {
        Typed(String),
        Closed,
    }
    let _english = in_language("en");
    let field = |value: &'static str| -> Element<'static, Sent> {
        search("search-in-a-dialog", "Search", value, Sent::Typed)
    };
    let dialog = |value: &'static str| -> Element<'static, Sent> {
        overlay::over(
            text("page"),
            Some(overlay::modal(field(value)).on_close(Sent::Closed)),
        )
    };
    let mut stage = Stage::new((600.0, 400.0));
    // The dialog gives its first control focus: the field.
    let sent = stage.frame(dialog("web"), |frame| {
        frame.tap_key(Named::Escape);
        assert!(frame.focused().is_some(), "focus stays in the field");
    });
    assert_eq!(sent, [Sent::Typed(String::new())]);
    let sent = stage.frame(dialog(""), |frame| frame.tap_key(Named::Escape));
    assert_eq!(sent, [Sent::Closed]);

    let panel = |value: &'static str| -> Element<'static, Sent> {
        overlay::popover(text("anchor"), true, move || field(value), Sent::Closed).into()
    };
    let mut stage = Stage::new((600.0, 400.0));
    let sent = stage.frame(panel("web"), |frame| {
        frame.tab();
        frame.tap_key(Named::Escape);
    });
    assert_eq!(sent, [Sent::Typed(String::new())]);
    let sent = stage.frame(panel(""), |frame| frame.tap_key(Named::Escape));
    assert_eq!(sent, [Sent::Closed]);
}

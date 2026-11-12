//! The Containers page under the keyboard alone: the search, the rows,
//! a selection's bar, a row's menu and the dialogs it opens.

use iced::keyboard::key::Named;

use super::*;
use crate::{
    test_support::snapshot::in_language,
    ui::shell::{page::testing::Asked, registry},
};

/// A focused widget this tall is the table's rows, which are one stop.
fn on_rows(frame: &mut Frame<'_, '_, Message>) -> bool {
    frame.focused().is_some_and(|bounds| bounds.height > 100.0)
}

impl Rig {
    fn key(&mut self, key: Named) {
        self.frame(|frame| frame.tap_key(key));
    }

    /// Tab, until the rows have focus.
    fn tab_to_rows(&mut self) {
        self.frame(|frame| {
            for _ in 0..40 {
                frame.tab();
                if on_rows(frame) {
                    return;
                }
            }
            panic!("Tab never reached the rows");
        });
    }

    /// The arrows, until the keyboard is on the container `name`.
    fn down_to(&mut self, name: &str) {
        let id = format!("{name}-0123456789abcdef");
        for _ in 0..20 {
            if self.state.table.focused().as_deref() == Some(id.as_str()) {
                return;
            }
            self.key(Named::ArrowDown);
        }
        panic!("the arrows never reached {name}");
    }

    /// Opens the menu of the row the keyboard is on and chooses `item`.
    fn choose(&mut self, item: &str) {
        self.key(Named::ContextMenu);
        self.frame(|frame| {
            for _ in 0..16 {
                if frame.is_focused(item) {
                    frame.tap_key(Named::Enter);
                    return;
                }
                frame.tap_key(Named::ArrowDown);
            }
            panic!("the menu never reached {item}");
        });
    }
}

#[test]
fn slash_gives_the_search_focus_and_typing_filters() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.frame(|frame| assert!(frame.focused().is_none()));
    let focus = super::super::request(&mut rig.state, registry::SEARCH, &rig.bench.ctx());
    rig.frame(|frame| {
        frame.perform(focus);
        assert!(frame.is_focused("Search containers"));
    });
    // A letter a frame: the field shows what the page was told.
    for letter in ["r", "e", "d"] {
        rig.frame(|frame| frame.typewrite(letter));
    }
    assert_eq!(rig.state.query, "red");
    assert!(rig.shows("redis") && !rig.shows("shop"));
    // Asked again, what is typed replaces what was there.
    let focus = super::super::request(&mut rig.state, registry::SEARCH, &rig.bench.ctx());
    rig.frame(|frame| {
        frame.perform(focus);
        frame.typewrite("b");
    });
    assert_eq!(rig.state.query, "b");
}

#[test]
fn the_keyboard_selects_rows_and_acts_on_them_from_the_bar() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.tab_to_rows();
    rig.down_to("redis");
    rig.key(Named::Space);
    // Up from the last line, to the one before.
    rig.key(Named::ArrowUp);
    rig.key(Named::Space);
    assert_eq!(rig.state.table.selection().len(), 2);
    assert!(rig.shows("2 selected"));
    // The bar is before the rows: Shift+Tab reaches its buttons.
    rig.frame(|frame| {
        for _ in 0..40 {
            frame.tab_back();
            if frame.is_focused("Stop") {
                frame.tap_key(Named::Enter);
                return;
            }
        }
        panic!("Shift+Tab never reached Stop");
    });
    let asked = rig.bench.took();
    assert!(
        matches!(asked.as_slice(), [Asked::Failed(Some(names), _), Asked::Invalidate(_)]
            if names == "redis, backup-job"),
        "{asked:?}"
    );
    // What failed stays selected, for another try; Escape on the rows
    // lets go of it.
    assert_eq!(rig.state.table.selection().len(), 2);
    rig.tab_to_rows();
    rig.key(Named::Escape);
    assert!(rig.state.table.selection().is_empty());
}

#[test]
fn a_row_s_menu_opens_rename_and_focus_comes_back_to_the_row() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.tab_to_rows();
    rig.down_to("redis");
    rig.choose("Rename");
    assert!(rig.shows("Rename redis"));
    rig.frame(|frame| {
        // The field has focus as the dialog opens: typing goes there.
        assert!(frame.focused().is_some() && !on_rows(frame));
        frame.typewrite("x");
    });
    assert!(rig.shows("redisx"));
    rig.key(Named::Escape);
    assert!(!rig.state.actions.has_dialog());
    rig.frame(|frame| assert!(on_rows(frame), "focus is back on the rows"));
    // And the keyboard goes on from the same row.
    rig.key(Named::ArrowUp);
    assert_eq!(
        rig.state.table.focused().as_deref(),
        Some("backup-job-0123456789abcdef")
    );
}

#[test]
fn removing_is_confirmed_and_cancel_gives_the_row_its_focus_back() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.tab_to_rows();
    rig.down_to("redis");
    rig.choose("Remove");
    assert!(rig.shows("Remove redis?"));
    rig.frame(|frame| {
        assert!(frame.is_focused("Cancel"), "the harmless one has focus");
        // Tab stays in the dialog.
        frame.tab();
        assert!(frame.is_focused("Remove"));
        frame.tab();
        assert!(frame.is_focused("Cancel"));
        frame.tap_key(Named::Enter);
    });
    assert!(!rig.state.actions.has_dialog());
    assert!(rig.bench.took().is_empty(), "nothing was removed");
    rig.frame(|frame| assert!(on_rows(frame), "focus is back on the rows"));
    // Confirmed with no Engine: it fails, the dialog stays for another
    // try, and Escape closes it.
    rig.choose("Remove");
    rig.frame(|frame| {
        frame.tab();
        frame.tap_key(Named::Enter);
    });
    assert!(rig.state.actions.has_dialog());
    assert!(matches!(
        rig.bench.took().as_slice(),
        [Asked::Failed(..), Asked::Invalidate(_)]
    ));
    rig.key(Named::Escape);
    assert!(!rig.state.actions.has_dialog());
}

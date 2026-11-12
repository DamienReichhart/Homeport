//! The Board under the keyboard alone: the search, a selection that
//! spans Hosts, and a row's menu with its confirmation.

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
}

#[test]
fn slash_gives_the_search_focus_and_typing_filters_every_host() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    let focus = super::super::request(&mut rig.state, registry::SEARCH, &rig.bench.ctx());
    rig.frame(|frame| {
        frame.perform(focus);
        assert!(frame.is_focused("Search every host"));
    });
    for letter in ["p", "r", "o"] {
        rig.frame(|frame| frame.typewrite(letter));
    }
    // The search looks in the Host's name too.
    assert_eq!(rig.state.filter.query, "pro");
    assert_eq!(rig.order(&["api", "cache", "db", "web"]), ["api", "cache"]);
}

#[test]
fn the_keyboard_selects_across_hosts_and_stops_the_selection() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.tab_to_rows();
    // db and web on this machine, api and cache on the server.
    rig.key(Named::ArrowDown);
    rig.key(Named::ArrowDown);
    rig.key(Named::Space);
    rig.key(Named::ArrowDown);
    rig.key(Named::Space);
    assert!(rig.shows("2 selected"));
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
    let failed: Vec<String> = rig
        .bench
        .took()
        .into_iter()
        .filter_map(|asked| match asked {
            Asked::Failed(title, _) => title,
            _ => None,
        })
        .collect();
    assert_eq!(failed, ["On local: web", "On prod-server: api"]);
}

#[test]
fn a_row_s_menu_asks_before_removing_and_cancel_gives_the_row_its_focus_back() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.tab_to_rows();
    rig.key(Named::ArrowDown);
    rig.key(Named::ContextMenu);
    rig.frame(|frame| {
        for _ in 0..16 {
            if frame.is_focused("Remove") {
                frame.tap_key(Named::Enter);
                return;
            }
            frame.tap_key(Named::ArrowDown);
        }
        panic!("the menu never reached Remove");
    });
    assert!(rig.shows("Remove db?"));
    rig.frame(|frame| {
        assert!(frame.is_focused("Cancel"));
        frame.tap_key(Named::Enter);
    });
    assert!(!rig.state.actions.has_dialog());
    rig.frame(|frame| assert!(on_rows(frame), "focus is back on the rows"));
    rig.key(Named::ArrowDown);
    assert_eq!(
        rig.state.table.focused().as_deref(),
        Some("h1/web-0123456789abcdef")
    );
}

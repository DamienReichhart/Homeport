//! The theme's gallery held to its pictures, in both themes and both
//! directions: a style that changes shows here, and nowhere by accident.

use super::{Gallery, Message};
use crate::{
    i18n::Direction,
    test_support::{
        snapshot::{assert_snapshot, in_language},
        window::drive,
    },
    ui::theme::Theme,
};

/// Tall enough for the whole page: a picture shows no scrolling.
const SIZE: (f32, f32) = (1100.0, 2500.0);

#[test]
fn the_gallery_in_both_themes_and_both_directions() {
    for theme in Theme::ALL {
        for direction in [Direction::Ltr, Direction::Rtl] {
            let mut gallery = Gallery::showing(theme, direction);
            // The scripts the OS's fonts draw differ from machine to machine.
            gallery.os_scripts = false;
            let name = format!("theme-{theme:?}-{direction:?}").to_lowercase();
            assert_snapshot(&name, "en", &theme, SIZE, || gallery.view());
        }
    }
}

/// Every section lays out with the scripts too, whatever draws them.
#[test]
fn the_whole_gallery_lays_out() {
    let _english = in_language("en");
    let gallery = Gallery::default();
    let messages = drive(
        SIZE,
        || gallery.view(),
        |window| {
            assert!(window.find("Other scripts (the OS's fonts)").is_ok());
            assert!(window.find("Icons").is_ok());
        },
    );
    assert!(messages.is_empty());
}

/// The switches at the top do what they say: a click sends the message.
#[test]
fn the_switches_change_the_theme_and_the_direction() {
    let _english = in_language("en");
    let mut gallery = Gallery::default();
    let messages: Vec<Message> = drive(
        SIZE,
        || gallery.view(),
        |window| {
            window.click("Dark").expect("a Dark switch");
            window.click("Right to left").expect("a direction switch");
        },
    );
    for message in messages {
        gallery.update(message);
    }
    assert_eq!(gallery.theme, Theme::Dark);
    assert_eq!(gallery.direction, Direction::Rtl);
}

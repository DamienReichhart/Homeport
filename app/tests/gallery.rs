//! Draws the theme's gallery (`examples/gallery.rs`) without a window, in
//! both themes and both directions, with iced's software renderer: every
//! style lays out and paints with the bundled fonts and icons.
//!
//! The pixels are not compared: text in other scripts is drawn with the
//! fonts of the machine, which differ. To look at them, name a folder:
//!
//! ```sh
//! HOMEPORT_GALLERY_SHOTS=/tmp/gallery cargo test -p homeport-app --test gallery
//! ```

use std::{env, path::PathBuf};

use homeport_app::{
    i18n::Direction,
    ui::theme::{font, Theme},
};
use iced_test::{core::Settings, Simulator};

// The example's `main` and what only a window sends are not the test's.
#[allow(dead_code)]
#[path = "../examples/gallery.rs"]
mod gallery;

use gallery::{Gallery, Message};

/// Tall enough for the whole page: a screenshot shows no scrolling.
const SIZE: (f32, f32) = (1100.0, 2500.0);

fn settings() -> Settings {
    Settings {
        fonts: font::files().collect(),
        default_font: font::DEFAULT,
        ..Settings::default()
    }
}

#[test]
fn the_gallery_draws_in_both_themes_and_directions() {
    let shots = env::var_os("HOMEPORT_GALLERY_SHOTS").map(PathBuf::from);
    for theme in Theme::ALL {
        for direction in [Direction::Ltr, Direction::Rtl] {
            let gallery = Gallery::showing(theme, direction);
            let mut window: Simulator<'_, Message, Theme, iced::Renderer> =
                Simulator::with_size(settings(), SIZE, gallery.view());
            let snapshot = window.snapshot(&theme).expect("the gallery draws");
            if let Some(folder) = &shots {
                let name = format!("gallery-{theme:?}-{direction:?}").to_lowercase();
                let path = folder.join(name);
                // Writes the PNG when there is none yet, which is all we want.
                assert!(snapshot.matches_image(&path).is_ok(), "{}", path.display());
            }
        }
    }
}

/// The switches at the top do what they say: a click sends the message.
#[test]
fn the_switches_change_the_theme_and_the_direction() {
    let mut gallery = Gallery::default();
    let mut window: Simulator<'_, Message, Theme, iced::Renderer> =
        Simulator::with_size(settings(), SIZE, gallery.view());
    window.click("Dark").expect("a Dark switch");
    window.click("Right to left").expect("a direction switch");
    let messages: Vec<Message> = window.into_messages().collect();
    for message in messages {
        gallery.update(message);
    }
    assert_eq!(gallery.theme, Theme::Dark);
    assert_eq!(gallery.direction, Direction::Rtl);
}

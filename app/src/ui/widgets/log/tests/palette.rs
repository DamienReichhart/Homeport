//! The colours a program asks for, as each theme draws them.

use iced::Color;

use crate::ui::{
    theme::{
        color::{contrast, over},
        Theme,
    },
    widgets::log::{
        ansi::{Attrs, Colour, Style},
        palette::{colours, fixed, LEGIBLE},
    },
};

fn fg(colour: Colour) -> Style {
    Style {
        fg: colour,
        ..Style::default()
    }
}

#[test]
fn the_sixteen_are_the_themes_and_bold_is_their_bright_form() {
    for theme in Theme::ALL {
        let terminal = &theme.tokens().terminal;
        let (red, behind) = colours(terminal, fg(Colour::Indexed(1)));
        assert_eq!((red, behind), (terminal.ansi[1], None));
        let bold = Style {
            attrs: Attrs::BOLD,
            ..fg(Colour::Indexed(1))
        };
        assert_eq!(colours(terminal, bold).0, terminal.ansi[9]);
        assert_eq!(
            colours(terminal, fg(Colour::Indexed(12))).0,
            terminal.ansi[12]
        );
        assert_eq!(
            colours(terminal, Style::default()),
            (terminal.foreground, None)
        );
    }
}

#[test]
fn every_colour_a_program_can_ask_for_reads_in_both_themes() {
    for theme in Theme::ALL {
        let terminal = &theme.tokens().terminal;
        let all = (0..=255_u8).map(Colour::Indexed).chain([
            Colour::Rgb(0, 0, 0),
            Colour::Rgb(255, 255, 255),
            Colour::Rgb(255, 255, 0),
            Colour::Rgb(0, 0, 128),
        ]);
        for colour in all {
            let (text, _) = colours(terminal, fg(colour));
            let ratio = contrast(text, terminal.background);
            assert!(ratio >= LEGIBLE, "{theme:?} {colour:?}: {ratio:.2}");
        }
        // A colour that already reads is the program's, untouched.
        let orange = colours(terminal, fg(Colour::Rgb(200, 90, 20))).0;
        assert_eq!(orange, Color::from_rgb8(200, 90, 20), "{theme:?}");
        // Dimmed text is still there.
        let dim = Style {
            attrs: Attrs::DIM,
            ..Style::default()
        };
        let dimmed = over(colours(terminal, dim).0, terminal.background);
        assert!(contrast(dimmed, terminal.background) >= LEGIBLE);
    }
}

#[test]
fn inverse_swaps_and_a_background_is_kept_as_asked() {
    let terminal = &Theme::Dark.tokens().terminal;
    let inverse = Style {
        attrs: Attrs::INVERSE,
        ..Style::default()
    };
    assert_eq!(
        colours(terminal, inverse),
        (terminal.background, Some(terminal.foreground))
    );
    let on_blue = Style {
        fg: Colour::Rgb(10, 10, 10),
        bg: Colour::Indexed(4),
        attrs: Attrs::default(),
    };
    // On a background of its own the program chose both: left alone.
    assert_eq!(
        colours(terminal, on_blue),
        (Color::from_rgb8(10, 10, 10), Some(terminal.ansi[4]))
    );
    // Text that would be lost on its own background takes whichever
    // of the viewer's two colours shows on it.
    for index in 0..16 {
        let behind = Style {
            bg: Colour::Indexed(index),
            ..Style::default()
        };
        for theme in Theme::ALL {
            let terminal = &theme.tokens().terminal;
            let (text, back) = colours(terminal, behind);
            let ratio = contrast(text, back.expect("a background"));
            assert!(ratio >= LEGIBLE, "{theme:?} on {index}: {ratio:.2}");
        }
    }
    assert_eq!(fixed(16), Color::from_rgb8(0, 0, 0));
    assert_eq!(fixed(231), Color::from_rgb8(255, 255, 255));
    assert_eq!(fixed(196), Color::from_rgb8(255, 0, 0));
    assert_eq!(fixed(244), Color::from_rgb8(128, 128, 128));
}

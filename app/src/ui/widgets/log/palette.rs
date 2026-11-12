//! The colours of a log, from the theme's terminal palette: the 16 a
//! program names are the theme's own in each theme, the others (the 256,
//! or one given as red, green and blue) are what they are, and any of
//! them that would not read on what is behind it is moved until it does
//! (black on the dark theme, pale yellow on the light one).

use iced::Color;

use super::ansi::{Attrs, Colour, Style};
use crate::ui::theme::{
    color::{alpha, contrast, mix},
    tokens::Terminal,
};

/// The least contrast of coloured text on the viewer's background.
pub const LEGIBLE: f32 = 3.0;
/// How much of its colour dimmed text keeps (`ESC [ 2 m`, a timestamp).
pub const DIM: f32 = 0.5;

/// The ANSI colours the sources of a multi-source log take in turn, as
/// `docker compose logs`: cyan, magenta, yellow, green, blue, red.
const SOURCES: [usize; 6] = [6, 5, 3, 2, 4, 1];

/// The colour of the label of the source `id`.
pub fn source(terminal: &Terminal, id: u16) -> Color {
    terminal.ansi[SOURCES[usize::from(id) % SOURCES.len()]]
}

/// One of the 256 colours past the theme's 16: the 6 × 6 × 6 cube, then
/// 24 greys.
pub(super) fn fixed(index: u8) -> Color {
    const STEPS: [u8; 6] = [0, 95, 135, 175, 215, 255];
    if index >= 232 {
        let grey = 8 + 10 * (index - 232);
        return Color::from_rgb8(grey, grey, grey);
    }
    let cube = usize::from(index.saturating_sub(16));
    Color::from_rgb8(STEPS[cube / 36], STEPS[cube / 6 % 6], STEPS[cube % 6])
}

/// `colour` as the theme draws it; `bright` takes the bright form of the
/// first eight, which is how bold shows in a font of one weight.
fn resolve(terminal: &Terminal, colour: Colour, bright: bool) -> Option<Color> {
    match colour {
        Colour::Default => None,
        Colour::Indexed(index @ 0..=7) if bright => Some(terminal.ansi[usize::from(index) + 8]),
        Colour::Indexed(index @ 0..=15) => Some(terminal.ansi[usize::from(index)]),
        Colour::Indexed(index) => Some(fixed(index)),
        Colour::Rgb(r, g, b) => Some(Color::from_rgb8(r, g, b)),
    }
}

/// `colour` moved towards the text's colour until it reads on `background`.
pub fn legible(colour: Color, background: Color, text: Color) -> Color {
    if contrast(colour, background) >= LEGIBLE {
        return colour;
    }
    [0.25, 0.5, 0.75]
        .into_iter()
        .map(|amount| mix(colour, text, amount))
        .find(|mixed| contrast(*mixed, background) >= LEGIBLE)
        .unwrap_or(text)
}

/// What a run is drawn in: its text, and what is behind it when that is
/// not the viewer's background.
pub fn colours(terminal: &Terminal, style: Style) -> (Color, Option<Color>) {
    let bold = style.attrs.has(Attrs::BOLD);
    let fg = resolve(terminal, style.fg, bold);
    let bg = resolve(terminal, style.bg, false);
    let (text, behind) = if style.attrs.has(Attrs::INVERSE) {
        (
            bg.unwrap_or(terminal.background),
            Some(fg.unwrap_or(terminal.foreground)),
        )
    } else {
        (fg.unwrap_or(terminal.foreground), bg)
    };
    // Whatever the program chose, the text reads on what is behind it:
    // on the viewer's background it is moved towards the text's colour,
    // on a background of its own it becomes the one of the viewer's two
    // colours that shows best.
    let text = match behind {
        None => legible(text, terminal.background, terminal.foreground),
        Some(behind) if contrast(text, behind) < LEGIBLE => {
            let (light, dark) = (terminal.foreground, terminal.background);
            if contrast(light, behind) >= contrast(dark, behind) {
                light
            } else {
                dark
            }
        }
        Some(_) => text,
    };
    let text = if style.attrs.has(Attrs::DIM) {
        alpha(text, DIM)
    } else {
        text
    };
    (text, behind)
}

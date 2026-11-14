//! The colours of a cell, as the log viewer draws them: the 16 a program
//! names are the theme's terminal palette, the 256 and 24-bit colours are
//! what they are, bold is the bright form of the first eight (the
//! monospaced face has one weight, as React's `fontWeightBold: 'normal'`),
//! and text that would not read on what is behind it is moved until it
//! does (`log::palette::colours`, which draws them). A colour a program set with OSC 4 is its own.

use alacritty_terminal::{
    term::{
        cell::{Cell, Flags},
        color::Colors,
    },
    vte::ansi::{Color as Wanted, NamedColor},
};

use crate::ui::widgets::log::ansi::{Attrs, Colour, Style};

/// A colour of a cell in the log viewer's terms, and whether it is a dim
/// one (`NamedColor::DimRed` and the like).
fn colour(wanted: Wanted, overrides: &Colors) -> (Colour, bool) {
    let rgb = |rgb: alacritty_terminal::vte::ansi::Rgb| Colour::Rgb(rgb.r, rgb.g, rgb.b);
    match wanted {
        Wanted::Spec(spec) => (rgb(spec), false),
        Wanted::Indexed(index) => match overrides[usize::from(index)] {
            Some(set) => (rgb(set), false),
            None => (Colour::Indexed(index), false),
        },
        Wanted::Named(named) => {
            let index = named as usize;
            if let Some(set) = overrides[index] {
                return (rgb(set), false);
            }
            match named {
                NamedColor::Foreground | NamedColor::Background | NamedColor::BrightForeground => {
                    (Colour::Default, false)
                }
                NamedColor::DimForeground | NamedColor::Cursor => (Colour::Default, true),
                _ if index < 16 => (Colour::Indexed(u8::try_from(index).unwrap_or(7)), false),
                // DimBlack to DimWhite follow the eight.
                _ => {
                    let dim = index.saturating_sub(NamedColor::DimBlack as usize) % 8;
                    (Colour::Indexed(u8::try_from(dim).unwrap_or(7)), true)
                }
            }
        }
    }
}

/// How a cell is drawn, as the log viewer's style.
pub fn style(cell: &Cell, overrides: &Colors) -> Style {
    let (fg, dim) = colour(cell.fg, overrides);
    let (bg, _) = colour(cell.bg, overrides);
    let flags = cell.flags;
    let mut attrs = Attrs::default();
    for (flag, attr) in [
        (Flags::BOLD, Attrs::BOLD),
        (Flags::DIM, Attrs::DIM),
        (Flags::ITALIC, Attrs::ITALIC),
        (Flags::ALL_UNDERLINES, Attrs::UNDERLINE),
        (Flags::INVERSE, Attrs::INVERSE),
        (Flags::STRIKEOUT, Attrs::STRIKE),
    ] {
        if flags.intersects(flag) {
            attrs = attrs.with(attr);
        }
    }
    if dim {
        attrs = attrs.with(Attrs::DIM);
    }
    Style { fg, bg, attrs }
}

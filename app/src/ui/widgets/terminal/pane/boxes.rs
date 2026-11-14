//! Box drawing and block elements drawn as shapes, not glyphs: the
//! monospaced face has none of them (its character map was read), and a
//! fallback font's would neither fill the cell nor meet the next one.
//! tmux's borders, htop's and `mc`'s frames, progress bars of blocks:
//! lines that join from cell to cell at any size, as alacritty and
//! xterm.js draw them.

use iced::{
    advanced::{renderer, Renderer as _},
    Color, Rectangle,
};

use crate::ui::theme::color::alpha;

/// How thick an arm of a box-drawing character is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Weight {
    None,
    Light,
    Heavy,
    Double,
}

/// The arms of U+2500 to U+257F, four characters each: left, right, up,
/// down; `-` none, `L` light, `H` heavy, `D` double. `?` is drawn by a
/// font (the diagonals). Dashed lines are drawn whole.
const ARMS: [&str; 128] = [
    "LL--", "HH--", "--LL", "--HH", "LL--", "HH--", "--LL", "--HH", // 2500
    "LL--", "HH--", "--LL", "--HH", "-L-L", "-H-L", "-L-H", "-H-H", // 2508
    "L--L", "H--L", "L--H", "H--H", "-LL-", "-HL-", "-LH-", "-HH-", // 2510
    "L-L-", "H-L-", "L-H-", "H-H-", "-LLL", "-HLL", "-LHL", "-LLH", // 2518
    "-LHH", "-HHL", "-HLH", "-HHH", "L-LL", "H-LL", "L-HL", "L-LH", // 2520
    "L-HH", "H-HL", "H-LH", "H-HH", "LL-L", "HL-L", "LH-L", "HH-L", // 2528
    "LL-H", "HL-H", "LH-H", "HH-H", "LLL-", "HLL-", "LHL-", "HHL-", // 2530
    "LLH-", "HLH-", "LHH-", "HHH-", "LLLL", "HLLL", "LHLL", "HHLL", // 2538
    "LLHL", "LLLH", "LLHH", "HLHL", "LHHL", "HLLH", "LHLH", "HHHL", // 2540
    "HHLH", "HLHH", "LHHH", "HHHH", "LL--", "HH--", "--LL", "--HH", // 2548
    "DD--", "--DD", "-D-L", "-L-D", "-D-D", "D--L", "L--D", "D--D", // 2550
    "-DL-", "-LD-", "-DD-", "D-L-", "L-D-", "D-D-", "-DLL", "-LDD", // 2558
    "-DDD", "D-LL", "L-DD", "D-DD", "DD-L", "LL-D", "DD-D", "DDL-", // 2560
    "LLD-", "DDD-", "DDLL", "LLDD", "DDDD", "-L-L", "L--L", "L-L-", // 2568
    "-LL-", "????", "????", "????", "L---", "--L-", "-L--", "---L", // 2570
    "H---", "--H-", "-H--", "---H", "LH--", "--LH", "HL--", "--HL", // 2578
];

fn weight(c: u8) -> Weight {
    match c {
        b'L' => Weight::Light,
        b'H' => Weight::Heavy,
        b'D' => Weight::Double,
        _ => Weight::None,
    }
}

/// A part of a cell: from its left, its top, to its right, its bottom,
/// as shares.
type Part = (f32, f32, f32, f32);

/// What a block element fills of its cell: from left, from top, to
/// right, to bottom, as shares; and how much of the colour (the shades).
fn block(c: char) -> Option<(Vec<Part>, f32)> {
    let eighth = |n: u32| f32::from(u8::try_from(n).unwrap_or(8)) / 8.0;
    let code = u32::from(c);
    let quadrant = |bits: u8| {
        let mut parts = vec![];
        for (bit, part) in [
            (1, (0.0, 0.0, 0.5, 0.5)),
            (2, (0.5, 0.0, 1.0, 0.5)),
            (4, (0.0, 0.5, 0.5, 1.0)),
            (8, (0.5, 0.5, 1.0, 1.0)),
        ] {
            if bits & bit != 0 {
                parts.push(part);
            }
        }
        parts
    };
    Some(match code {
        0x2580 => (vec![(0.0, 0.0, 1.0, 0.5)], 1.0),
        0x2581..=0x2588 => (vec![(0.0, 1.0 - eighth(code - 0x2580), 1.0, 1.0)], 1.0),
        0x2589..=0x258F => (vec![(0.0, 0.0, eighth(0x2590 - code), 1.0)], 1.0),
        0x2590 => (vec![(0.5, 0.0, 1.0, 1.0)], 1.0),
        0x2591 => (vec![(0.0, 0.0, 1.0, 1.0)], 0.25),
        0x2592 => (vec![(0.0, 0.0, 1.0, 1.0)], 0.5),
        0x2593 => (vec![(0.0, 0.0, 1.0, 1.0)], 0.75),
        0x2594 => (vec![(0.0, 0.0, 1.0, 0.125)], 1.0),
        0x2595 => (vec![(0.875, 0.0, 1.0, 1.0)], 1.0),
        0x2596 => (quadrant(4), 1.0),
        0x2597 => (quadrant(8), 1.0),
        0x2598 => (quadrant(1), 1.0),
        0x2599 => (quadrant(1 | 4 | 8), 1.0),
        0x259A => (quadrant(1 | 8), 1.0),
        0x259B => (quadrant(1 | 2 | 4), 1.0),
        0x259C => (quadrant(1 | 2 | 8), 1.0),
        0x259D => (quadrant(2), 1.0),
        0x259E => (quadrant(2 | 4), 1.0),
        0x259F => (quadrant(2 | 4 | 8), 1.0),
        _ => return None,
    })
}

/// Whether `c` is drawn here rather than by a font.
pub fn is_drawn(c: char) -> bool {
    let code = u32::from(c);
    match code {
        0x2500..=0x257F => ARMS[(code - 0x2500) as usize] != "????",
        0x2580..=0x259F => true,
        _ => false,
    }
}

fn fill(renderer: &mut iced::Renderer, bounds: Rectangle, colour: Color) {
    if bounds.width > 0.0 && bounds.height > 0.0 {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                ..renderer::Quad::default()
            },
            colour,
        );
    }
}

/// Draws `c` (one [`is_drawn`] says yes to) in the cell `cell`.
pub fn draw(renderer: &mut iced::Renderer, c: char, cell: Rectangle, colour: Color) {
    let code = u32::from(c);
    if let Some((parts, share)) = block(c) {
        let colour = alpha(colour, share);
        for (left, top, right, bottom) in parts {
            let area = Rectangle {
                x: cell.x + cell.width * left,
                y: cell.y + cell.height * top,
                width: cell.width * (right - left),
                height: cell.height * (bottom - top),
            };
            fill(renderer, area, colour);
        }
        return;
    }
    let Some(arms) = code
        .checked_sub(0x2500)
        .and_then(|at| ARMS.get(at as usize))
        .map(|arms| arms.as_bytes())
    else {
        return;
    };
    // A light line is a pixel for every eight of the cell's width, and
    // never less than one; the lines stand on whole pixels.
    let thin = (cell.width / 8.0).round().max(1.0);
    let cx = (cell.x + cell.width / 2.0 - thin / 2.0).floor();
    let cy = (cell.y + cell.height / 2.0 - thin / 2.0).floor();
    let (right, bottom) = (cell.x + cell.width, cell.y + cell.height);
    // Each arm as rectangles across its width, for its weight.
    let across = |weight: Weight, centre: f32| -> Vec<(f32, f32)> {
        match weight {
            Weight::None => vec![],
            Weight::Light => vec![(centre, thin)],
            Weight::Heavy => vec![(centre - thin / 2.0, thin * 2.0)],
            Weight::Double => vec![(centre - thin, thin), (centre + thin, thin)],
        }
    };
    let [left, right_arm, up, down] = [arms[0], arms[1], arms[2], arms[3]].map(weight);
    // How far a line reaches past its centre's thin line on each side, so
    // the arms across it start and end on its edges: a corner closes, and
    // nothing sticks out past it.
    let spread = |weight: Weight| match weight {
        Weight::None | Weight::Light => 0.0,
        Weight::Heavy => thin / 2.0,
        Weight::Double => thin,
    };
    let widest = |a: Weight, b: Weight| spread(a).max(spread(b));
    let upright = widest(up, down);
    let level = widest(left, right_arm);
    let joined = |a: Weight, b: Weight| a != Weight::None || b != Weight::None;
    // An arm alone (`╴`) stops at the centre; across a line it covers it.
    let (stop, start) = if joined(up, down) {
        (cx + thin + upright, cx - upright)
    } else {
        (cx + thin, cx)
    };
    for (y, height) in across(left, cy) {
        fill(
            renderer,
            Rectangle::new((cell.x, y).into(), (stop - cell.x, height).into()),
            colour,
        );
    }
    for (y, height) in across(right_arm, cy) {
        fill(
            renderer,
            Rectangle::new((start, y).into(), (right - start, height).into()),
            colour,
        );
    }
    let (stop, start) = if joined(left, right_arm) {
        (cy + thin + level, cy - level)
    } else {
        (cy + thin, cy)
    };
    for (x, width) in across(up, cx) {
        fill(
            renderer,
            Rectangle::new((x, cell.y).into(), (width, stop - cell.y).into()),
            colour,
        );
    }
    for (x, width) in across(down, cx) {
        fill(
            renderer,
            Rectangle::new((x, start).into(), (width, bottom - start).into()),
            colour,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_names_the_lines_unicode_names() {
        let arms = |c: char| ARMS[(u32::from(c) - 0x2500) as usize];
        assert_eq!(arms('─'), "LL--");
        assert_eq!(arms('│'), "--LL");
        assert_eq!(arms('┌'), "-L-L");
        assert_eq!(arms('┘'), "L-L-");
        assert_eq!(arms('┼'), "LLLL");
        assert_eq!(arms('╋'), "HHHH");
        assert_eq!(arms('═'), "DD--");
        assert_eq!(arms('╔'), "-D-D");
        assert_eq!(arms('╬'), "DDDD");
        assert_eq!(arms('╭'), "-L-L");
        assert_eq!(arms('╰'), "-LL-");
        assert_eq!(arms('╿'), "--HL");
        assert!(!is_drawn('╱') && !is_drawn('a') && !is_drawn('日'));
        assert!(is_drawn('█') && is_drawn('▒') && is_drawn('▟'));
    }
}

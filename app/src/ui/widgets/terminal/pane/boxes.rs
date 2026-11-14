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
    let arms = [arms[0], arms[1], arms[2], arms[3]].map(weight);
    if matches!(code, 0x256D..=0x2570) {
        arc(renderer, arms, cell, colour);
        return;
    }
    lines(renderer, arms, dashes(code), cell, colour);
}

/// How many dashes a dashed line is cut into: U+2504 to U+250B and
/// U+254C to U+254F.
fn dashes(code: u32) -> usize {
    match code {
        0x2504..=0x2507 => 3,
        0x2508..=0x250B => 4,
        0x254C..=0x254F => 2,
        _ => 1,
    }
}

/// How thick a light line is in a cell this wide: a pixel for every
/// eight, and never less than one.
fn thin_of(cell: Rectangle) -> f32 {
    (cell.width / 8.0).round().max(1.0)
}

/// How far a line of `weight` reaches from its centre.
fn reach(weight: Weight, thin: f32) -> f32 {
    match weight {
        Weight::None => 0.0,
        Weight::Light => thin / 2.0,
        Weight::Heavy => thin,
        Weight::Double => thin * 1.5,
    }
}

/// How far past the centre an arm's line (one of a double line's two,
/// or its only one) goes, towards the
/// other side: what makes corners close, crossings of double lines leave
/// their inner corners open (`╬`, `╔`), and a line meet one across it.
/// `same` is the arm across on the line's side, `other` the one opposite,
/// `ahead` the arm that goes on in the line's direction.
fn past(weight: Weight, same: Weight, other: Weight, ahead: Weight, thin: f32) -> f32 {
    if weight != Weight::Double {
        return reach(same, thin).max(reach(other, thin));
    }
    if same == Weight::Double {
        -thin / 2.0
    } else if same != Weight::None {
        reach(same, thin)
    } else if ahead != Weight::None {
        0.0
    } else {
        reach(other, thin)
    }
}

/// The straight lines of a box-drawing character, dashed or not.
fn lines(
    renderer: &mut iced::Renderer,
    arms: [Weight; 4],
    dashes: usize,
    cell: Rectangle,
    colour: Color,
) {
    let [left, right_arm, up, down] = arms;
    let thin = thin_of(cell);
    // The centre lines stand on whole pixels.
    let cx = (cell.x + cell.width / 2.0 - thin / 2.0).floor() + thin / 2.0;
    let cy = (cell.y + cell.height / 2.0 - thin / 2.0).floor() + thin / 2.0;
    let (right, bottom) = (cell.x + cell.width, cell.y + cell.height);
    // The lines of an arm across it: their offsets from its middle and
    // how thick each is.
    let strokes = |weight: Weight| -> Vec<(f32, f32)> {
        match weight {
            Weight::None => vec![],
            Weight::Light => vec![(0.0, thin)],
            Weight::Heavy => vec![(0.0, thin * 2.0)],
            Weight::Double => vec![(-thin, thin), (thin, thin)],
        }
    };
    // A line from `from` to `to` along an axis, cut into dashes.
    let mut line = |from: f32, to: f32, across: f32, thick: f32, level: bool| {
        let (start, end) = if from <= to { (from, to) } else { (to, from) };
        let length = end - start;
        #[allow(clippy::cast_precision_loss)]
        let each = length / dashes as f32;
        for dash in 0..dashes {
            #[allow(clippy::cast_precision_loss)]
            let (a, b) = if dashes == 1 {
                (start, end)
            } else {
                let at = start + each * dash as f32;
                (at + each * 0.2, at + each * 0.8)
            };
            let area = if level {
                Rectangle::new((a, across - thick / 2.0).into(), (b - a, thick).into())
            } else {
                Rectangle::new((across - thick / 2.0, a).into(), (thick, b - a).into())
            };
            fill(renderer, area, colour);
        }
    };
    let side =
        |offset: f32, before: Weight, after: Weight| if offset < 0.0 { before } else { after };
    for (offset, thick) in strokes(left) {
        let same = side(offset, up, down);
        let other = side(offset, down, up);
        let end = cx + past(left, same, other, right_arm, thin);
        line(cell.x, end, cy + offset, thick, true);
    }
    for (offset, thick) in strokes(right_arm) {
        let same = side(offset, up, down);
        let other = side(offset, down, up);
        let start = cx - past(right_arm, same, other, left, thin);
        line(start, right, cy + offset, thick, true);
    }
    for (offset, thick) in strokes(up) {
        let same = side(offset, left, right_arm);
        let other = side(offset, right_arm, left);
        let end = cy + past(up, same, other, down, thin);
        line(cell.y, end, cx + offset, thick, false);
    }
    for (offset, thick) in strokes(down) {
        let same = side(offset, left, right_arm);
        let other = side(offset, right_arm, left);
        let start = cy - past(down, same, other, up, thin);
        line(start, bottom, cx + offset, thick, false);
    }
}

/// A rounded corner (`╭ ╮ ╯ ╰`): a quarter of a circle between its two arms.
fn arc(renderer: &mut iced::Renderer, arms: [Weight; 4], cell: Rectangle, colour: Color) {
    use iced::{
        advanced::graphics::geometry::Renderer as _,
        widget::canvas::{path::Builder, Frame, Stroke},
        Point,
    };
    let [left, _, up, _] = arms;
    let thin = thin_of(cell);
    let cx = (cell.x + cell.width / 2.0 - thin / 2.0).floor() + thin / 2.0;
    let cy = (cell.y + cell.height / 2.0 - thin / 2.0).floor() + thin / 2.0;
    let across = if left == Weight::None {
        cell.x + cell.width
    } else {
        cell.x
    };
    let along = if up == Weight::None {
        cell.y + cell.height
    } else {
        cell.y
    };
    let radius = (cell.width / 2.0).min(cell.height / 2.0);
    let mut path = Builder::new();
    path.move_to(Point::new(across, cy));
    path.arc_to(Point::new(cx, cy), Point::new(cx, along), radius);
    path.line_to(Point::new(cx, along));
    let mut frame = Frame::with_bounds(renderer, cell);
    frame.stroke(
        &path.build(),
        Stroke::default().with_color(colour).with_width(thin),
    );
    renderer.draw_geometry(frame.into_geometry());
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
        assert_eq!(dashes(0x2504), 3);
        assert_eq!(dashes(0x250A), 4);
        assert_eq!(dashes(0x254E), 2);
        assert_eq!(dashes(0x2500), 1);
        assert_eq!(arms('╿'), "--HL");
        assert!(!is_drawn('╱') && !is_drawn('a') && !is_drawn('日'));
        assert!(is_drawn('█') && is_drawn('▒') && is_drawn('▟'));
    }
}

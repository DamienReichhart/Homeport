//! Whether the series can be told apart: each colour against both
//! backgrounds of both themes, each pair of colours as eyes that lack
//! one of the three cones see them, and each pair of series by more
//! than its colour.

use iced::Color;

use crate::ui::{
    theme::{
        color::{contrast, oklab},
        Theme,
    },
    widgets::chart::{Dash, Look, Series, Tone},
};

/// What isn't text but must be seen (WCAG 1.4.11).
const SEEN: f32 = 3.0;
/// How far apart two colours are in Oklab before a line of one is told
/// from a line of the other: several times what an eye just notices
/// (about 0.02), since a line is thin.
const APART: f32 = 0.115;

/// How a colour looks without one kind of cone, by Machado, Oliveira
/// and Fernandes (2009), severity 1: the matrices act on linear RGB.
const PROTANOPIA: [[f32; 3]; 3] = [
    [0.152_286, 1.052_583, -0.204_868],
    [0.114_503, 0.786_281, 0.099_216],
    [-0.003_882, -0.048_116, 1.051_998],
];
const DEUTERANOPIA: [[f32; 3]; 3] = [
    [0.367_322, 0.860_646, -0.227_968],
    [0.280_085, 0.672_501, 0.047_413],
    [-0.011_820, 0.042_940, 0.968_881],
];
const TRITANOPIA: [[f32; 3]; 3] = [
    [1.255_528, -0.076_749, -0.178_779],
    [-0.078_411, 0.930_809, 0.147_602],
    [0.004_733, 0.691_367, 0.303_900],
];
const IDENTITY: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

const EYES: [(&str, [[f32; 3]; 3]); 4] = [
    ("every cone", IDENTITY),
    ("no red cones", PROTANOPIA),
    ("no green cones", DEUTERANOPIA),
    ("no blue cones", TRITANOPIA),
];

fn linear(channel: f32) -> f32 {
    if channel <= 0.040_45 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

fn encoded(channel: f32) -> f32 {
    if channel <= 0.003_130_8 {
        12.92 * channel
    } else {
        1.055 * channel.powf(1.0 / 2.4) - 0.055
    }
}

fn seen(colour: Color, eye: [[f32; 3]; 3]) -> Color {
    let light = [linear(colour.r), linear(colour.g), linear(colour.b)];
    let through = |row: [f32; 3]| {
        encoded((row[0] * light[0] + row[1] * light[1] + row[2] * light[2]).clamp(0.0, 1.0))
    };
    Color::from_rgb(through(eye[0]), through(eye[1]), through(eye[2]))
}

/// How far apart `a` and `b` are for the eye that tells them apart
/// least, and which.
fn apart(a: Color, b: Color) -> (f32, &'static str) {
    EYES.iter()
        .map(|(name, eye)| {
            let (a, b) = (oklab(seen(a, *eye)), oklab(seen(b, *eye)));
            let far =
                ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt();
            (far, *name)
        })
        .fold(
            (f32::MAX, ""),
            |least, eye| if eye.0 < least.0 { eye } else { least },
        )
}

#[test]
fn every_series_colour_is_seen_on_the_page_and_on_a_card() {
    for theme in Theme::ALL {
        let tokens = theme.tokens();
        for tone in Tone::ALL {
            for (surface, back) in [("the page", tokens.background), ("a card", tokens.card)] {
                let ratio = contrast(tone.colour(theme), back);
                assert!(
                    ratio >= SEEN,
                    "{tone:?} on {surface} in {theme:?}: {ratio:.2}:1"
                );
            }
        }
    }
}

#[test]
fn every_two_series_colours_stay_apart_without_one_kind_of_cone() {
    for theme in Theme::ALL {
        for (index, a) in Tone::ALL.iter().enumerate() {
            for b in &Tone::ALL[index + 1..] {
                let (far, eye) = apart(a.colour(theme), b.colour(theme));
                assert!(
                    far >= APART,
                    "{a:?} and {b:?} in {theme:?} with {eye}: {far:.3} apart"
                );
            }
        }
    }
}

/// The measure is worth what it catches: the red and the green a
/// terminal prints are one colour without green cones, and the theme's
/// own blue and magenta without red ones.
#[test]
fn colours_that_are_confused_are_caught() {
    let ansi = Theme::Light.tokens().terminal.ansi;
    let (far, eye) = apart(ansi[1], ansi[2]);
    assert!(far < APART, "red and green: {far:.3} with {eye}");
    assert_ne!(eye, "every cone");
    let (far, eye) = apart(ansi[4], ansi[5]);
    assert!(far < APART, "blue and magenta: {far:.3} with {eye}");
    assert_ne!(eye, "every cone");
    // And apart for an eye with every cone, so it is the cones that count.
    let whole = |a: Color, b: Color| {
        let (a, b) = (oklab(a), oklab(b));
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
    };
    assert!(whole(ansi[1], ansi[2]) >= APART && whole(ansi[4], ansi[5]) >= APART);
}

/// docs/BRAND.md: a series is never a status colour.
#[test]
fn no_series_colour_is_a_status_colour() {
    for theme in Theme::ALL {
        let tokens = theme.tokens();
        for tone in Tone::ALL {
            for (name, status) in [("signal", tokens.signal), ("warning", tokens.warning)] {
                let (a, b) = (oklab(tone.colour(theme)), oklab(status));
                let far =
                    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt();
                assert!(far >= 0.08, "{tone:?} is {far:.3} from {name} in {theme:?}");
            }
        }
    }
}

/// Colour is never the only cue: of the looks the kit gives, no two of
/// the first twenty share both colour and dashes, two that share a
/// colour never share their dashes, and neighbours differ in both.
#[test]
fn no_two_series_look_alike() {
    let looks: Vec<Look> = (0..20).map(Look::of).collect();
    for (index, a) in looks.iter().enumerate() {
        for b in &looks[index + 1..] {
            assert!(a.tone != b.tone || a.dash != b.dash, "{a:?} twice");
        }
    }
    for pair in looks.windows(2) {
        assert!(pair[0].tone != pair[1].tone && pair[0].dash != pair[1].dash);
    }
    // Every dash pattern is its own.
    for (index, a) in Dash::ALL.iter().enumerate() {
        for b in &Dash::ALL[index + 1..] {
            assert_ne!(a.segments(), b.segments());
        }
    }
}

#[test]
fn a_page_that_names_its_colours_names_its_dashes() {
    // The brand's pair: ink, and grey dashed.
    let received = Series::new("Received").tone(Tone::Ink).look(0);
    let sent = Series::new("Sent").tone(Tone::Grey).dashed().look(1);
    assert_eq!((received.tone, received.dash), (Tone::Ink, Dash::Solid));
    assert_eq!((sent.tone, sent.dash), (Tone::Grey, Dash::Dashed));
    // A second ink series is solid until told: its page chose.
    assert_eq!(
        Series::new("Limit").tone(Tone::Ink).look(1).dash,
        Dash::Solid
    );
    // And one that names nothing gets the kit's look for its place.
    assert_eq!(Series::new("worker-2").look(1), Look::of(1));
    assert!(Series::new("Used").filled().look(0).area);
}

/// The legend's sample of each dash pattern, drawn: which pixels of its
/// middle row are inked.
fn inked(dash: Dash) -> Vec<bool> {
    use iced::widget::canvas::Canvas;

    use crate::{
        test_support::snapshot::pixels,
        ui::{theme::Element, widgets::chart::Swatch},
    };
    let look = Look {
        tone: Tone::Ink,
        dash,
        area: false,
    };
    let sample: Element<'_, ()> = Canvas::new(Swatch {
        look,
        hidden: false,
    })
    .width(crate::ui::widgets::chart::SWATCH)
    .height(12.0)
    .into();
    let (width, height, rgba) = pixels(
        &Theme::Light,
        (crate::ui::widgets::chart::SWATCH, 12.0),
        sample,
    );
    let row = (height / 2) as usize * width as usize * 4;
    rgba[row..row + width as usize * 4]
        .chunks_exact(4)
        .map(|pixel| pixel[0] < 128)
        .collect()
}

/// Colour is never the only cue only if the other cue can be seen: the
/// five patterns as the legend draws them differ, each from each, in
/// at least a seventh of the sample's pixels, and none is blank.
#[test]
fn the_dash_patterns_differ_where_the_legend_draws_them() {
    let drawn: Vec<(Dash, Vec<bool>)> =
        Dash::ALL.iter().map(|dash| (*dash, inked(*dash))).collect();
    for (dash, row) in &drawn {
        let ink = row.iter().filter(|on| **on).count();
        assert!(
            ink * 5 >= row.len(),
            "{dash:?}: {ink} of {} pixels",
            row.len()
        );
    }
    for (index, (a, row_a)) in drawn.iter().enumerate() {
        for (b, row_b) in &drawn[index + 1..] {
            let differ = row_a.iter().zip(row_b).filter(|(a, b)| a != b).count();
            assert!(
                differ * 7 >= row_a.len(),
                "{a:?} and {b:?} differ in {differ} of {} pixels",
                row_a.len()
            );
        }
    }
}

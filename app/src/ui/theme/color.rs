//! What CSS did to the tokens and iced has no word for: reading `#rrggbb`,
//! `bg-primary/80`, `color-mix(in oklch, …)`, and the WCAG contrast the
//! palette is held to.

use iced::Color;

/// `#rrggbb` as a colour, at compile time: `hex(0x14_14_14)` is Ink.
pub const fn hex(rgb: u32) -> Color {
    Color::from_rgb8((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
}

/// `rgb(r g b / a%)`: the dark theme's borders are Paper seen through.
pub const fn hex_alpha(rgb: u32, alpha: f32) -> Color {
    Color::from_rgba8((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8, alpha)
}

/// Tailwind's `/80`: the colour with its alpha multiplied, drawn over
/// whatever is behind.
pub fn alpha(colour: Color, opacity: f32) -> Color {
    colour.scale_alpha(opacity)
}

/// `front` composed over the opaque `back`, as the screen shows it.
pub fn over(front: Color, back: Color) -> Color {
    let a = front.a;
    Color::from_rgb(
        front.r * a + back.r * (1.0 - a),
        front.g * a + back.g * (1.0 - a),
        front.b * a + back.b * (1.0 - a),
    )
}

/// CSS `color-mix(in oklch, a, b amount)` for two opaque colours. The hue is
/// mixed as Oklab's a and b, which is the same thing for the palette's
/// near-neutral greys (their chroma is too low for the hue's arc to matter).
pub fn mix(a: Color, b: Color, amount: f32) -> Color {
    let (a, b) = (oklab(a), oklab(b));
    let lerp = |x: f32, y: f32| x + (y - x) * amount;
    from_oklab([lerp(a[0], b[0]), lerp(a[1], b[1]), lerp(a[2], b[2])])
}

fn linear(channel: f32) -> f32 {
    if channel <= 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

fn gamma(channel: f32) -> f32 {
    let channel = channel.clamp(0.0, 1.0);
    if channel <= 0.003_130_8 {
        channel * 12.92
    } else {
        1.055 * channel.powf(1.0 / 2.4) - 0.055
    }
}

/// sRGB to Oklab (Björn Ottosson's matrices, as CSS Color 4 uses them).
pub fn oklab(colour: Color) -> [f32; 3] {
    let (r, g, b) = (linear(colour.r), linear(colour.g), linear(colour.b));
    let l = (0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b).cbrt();
    let m = (0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b).cbrt();
    let s = (0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b).cbrt();
    [
        0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s,
        1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s,
        0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s,
    ]
}

/// Oklab back to sRGB, clamped to what a screen shows.
pub fn from_oklab([l, a, b]: [f32; 3]) -> Color {
    let l_ = (l + 0.396_337_78 * a + 0.215_803_76 * b).powi(3);
    let m_ = (l - 0.105_561_346 * a - 0.063_854_17 * b).powi(3);
    let s_ = (l - 0.089_484_18 * a - 1.291_485_5 * b).powi(3);
    Color::from_rgb(
        gamma(4.076_741_7 * l_ - 3.307_711_6 * m_ + 0.230_969_94 * s_),
        gamma(-1.268_438 * l_ + 2.609_757_4 * m_ - 0.341_319_38 * s_),
        gamma(-0.004_196_086_3 * l_ - 0.703_418_6 * m_ + 1.707_614_7 * s_),
    )
}

/// WCAG 2.2 relative luminance, with the constants `contrast.test.ts` uses.
fn luminance(colour: Color) -> f32 {
    let channel = |c: f32| {
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(colour.r) + 0.7152 * channel(colour.g) + 0.0722 * channel(colour.b)
}

/// The WCAG contrast ratio of two opaque colours, from 1 to 21.
pub fn contrast(a: Color, b: Color) -> f32 {
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb8(colour: Color) -> [u8; 3] {
        let [r, g, b, _] = colour.into_rgba8();
        [r, g, b]
    }

    #[test]
    fn hex_reads_the_brand_colours() {
        assert_eq!(rgb8(hex(0x14_14_14)), [20, 20, 20]);
        assert_eq!(rgb8(hex(0xf7_f7_f4)), [247, 247, 244]);
        assert_eq!(rgb8(hex(0xc8_10_2e)), [200, 16, 46]);
        assert_eq!(rgb8(hex(0xff_5c_5c)), [255, 92, 92]);
        assert_eq!(hex_alpha(0xf7_f7_f4, 0.1).a, 0.1);
    }

    #[test]
    fn oklab_matches_the_reference_values() {
        // The CSS Color 4 values of white and of sRGB red.
        let white = oklab(Color::WHITE);
        assert!((white[0] - 1.0).abs() < 1e-3 && white[1].abs() < 1e-3 && white[2].abs() < 1e-3);
        let red = oklab(Color::from_rgb(1.0, 0.0, 0.0));
        assert!((red[0] - 0.627_96).abs() < 1e-3);
        assert!((red[1] - 0.224_86).abs() < 1e-3);
        assert!((red[2] - 0.125_85).abs() < 1e-3);
        // And back, to the byte.
        for colour in [0x14_14_14, 0xc8_10_2e, 0x9a_5b_00, 0xeb_eb_e6] {
            assert_eq!(rgb8(from_oklab(oklab(hex(colour)))), rgb8(hex(colour)));
        }
    }

    #[test]
    fn mixing_moves_towards_the_second_colour() {
        let (paper, ink) = (hex(0xf7_f7_f4), hex(0x14_14_14));
        assert_eq!(rgb8(mix(paper, ink, 0.0)), rgb8(paper));
        assert_eq!(rgb8(mix(paper, ink, 1.0)), rgb8(ink));
        // secondary with 5 % of the text colour: a grey a step darker.
        let hover = rgb8(mix(hex(0xeb_eb_e6), ink, 0.05));
        assert!(hover[0] < 0xeb && hover[0] > 0xd0, "{hover:?}");
    }

    #[test]
    fn contrast_is_the_ratio_of_docs_brand() {
        let (paper, ink) = (hex(0xf7_f7_f4), hex(0x14_14_14));
        assert!((contrast(ink, paper) - 17.2).abs() < 0.05);
        assert!((contrast(hex(0x5c_5c_57), paper) - 6.3).abs() < 0.05);
        assert!((contrast(Color::WHITE, Color::BLACK) - 21.0).abs() < 1e-3);
        // Half an ink ring over the page, as the focus ring is drawn.
        let ring = over(alpha(ink, 0.5), paper);
        assert!((contrast(ring, paper) - 3.4).abs() < 0.05);
    }
}

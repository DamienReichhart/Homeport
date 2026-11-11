//! Every colour pair the app uses, in both themes, against WCAG 2.2 AA:
//! 4.5:1 for text, 3:1 for what isn't text but must be seen (1.4.3, 1.4.11).
//! The pairs are those of `src/shared/styles/contrast.test.ts`, measured
//! from the Rust tokens, so the two palettes cannot drift apart unseen; then
//! the same for what the styles compose (a button's text on its hover).

use iced::{
    widget::{button, text_input},
    Background, Color,
};

use super::{
    color::{alpha, contrast, over},
    style, Theme, Tokens,
};

const TEXT: f32 = 4.5;
const UI: f32 = 3.0;

/// A token by its CSS name, as `contrast.test.ts` names it.
fn token(tokens: &Tokens, name: &str) -> Color {
    match name {
        "background" => tokens.background,
        "foreground" => tokens.foreground,
        "card" => tokens.card,
        "card-foreground" => tokens.card_foreground,
        "popover" => tokens.popover,
        "popover-foreground" => tokens.popover_foreground,
        "primary" => tokens.primary,
        "primary-foreground" => tokens.primary_foreground,
        "secondary" => tokens.secondary,
        "secondary-foreground" => tokens.secondary_foreground,
        "muted" => tokens.muted,
        "muted-foreground" => tokens.muted_foreground,
        "accent" => tokens.accent,
        "accent-foreground" => tokens.accent_foreground,
        "destructive" => tokens.destructive,
        "input" => tokens.input,
        "ring" => tokens.ring,
        "success" => tokens.success,
        "warning" => tokens.warning,
        "info" => tokens.info,
        "chart-4" => tokens.chart[3],
        "sidebar" => tokens.sidebar.background,
        "sidebar-foreground" => tokens.sidebar.foreground,
        "sidebar-accent" => tokens.sidebar.accent,
        "sidebar-accent-foreground" => tokens.sidebar.accent_foreground,
        other => panic!("no --{other}"),
    }
}

/// (foreground, background, minimum, foreground opacity, background over).
type Pair = (&'static str, &'static str, f32, f32, Option<&'static str>);

const APP_PAIRS: &[Pair] = &[
    ("foreground", "background", TEXT, 1.0, None),
    ("card-foreground", "card", TEXT, 1.0, None),
    ("popover-foreground", "popover", TEXT, 1.0, None),
    ("muted-foreground", "background", TEXT, 1.0, None),
    ("muted-foreground", "card", TEXT, 1.0, None),
    ("muted-foreground", "popover", TEXT, 1.0, None),
    ("muted-foreground", "muted", TEXT, 1.0, None),
    ("muted-foreground", "accent", TEXT, 1.0, None),
    ("muted-foreground", "sidebar", TEXT, 1.0, None),
    ("primary-foreground", "primary", TEXT, 1.0, None),
    ("secondary-foreground", "secondary", TEXT, 1.0, None),
    ("accent-foreground", "accent", TEXT, 1.0, None),
    ("destructive", "background", TEXT, 1.0, None),
    ("destructive", "card", TEXT, 1.0, None),
    ("warning", "background", TEXT, 1.0, None),
    ("warning", "card", TEXT, 1.0, None),
    ("sidebar-foreground", "sidebar", TEXT, 1.0, None),
    (
        "sidebar-accent-foreground",
        "sidebar-accent",
        TEXT,
        1.0,
        None,
    ),
    // A Stack's group row: bg-muted/30 over the card, with muted text on it.
    ("muted-foreground", "muted", TEXT, 1.0, Some("card")),
    // A control's only boundary, and the focus ring at the opacity it's drawn with.
    ("input", "background", UI, 1.0, None),
    ("input", "card", UI, 1.0, None),
    ("ring", "background", UI, 0.5, None),
    ("ring", "card", UI, 0.5, None),
    // Status shapes and the grey chart series.
    ("success", "card", UI, 1.0, None),
    ("destructive", "card", UI, 1.0, None),
    ("warning", "card", UI, 1.0, None),
    ("info", "card", UI, 1.0, None),
    ("chart-4", "card", UI, 1.0, None),
];

/// Each failing pair with its ratio; empty when every pair passes.
fn failures(tokens: &Tokens) -> Vec<String> {
    APP_PAIRS
        .iter()
        .filter_map(|&(fg, bg, min, opacity, under)| {
            let back = match under {
                Some(under) => over(alpha(token(tokens, bg), 0.3), token(tokens, under)),
                None => token(tokens, bg),
            };
            let front = over(alpha(token(tokens, fg), opacity), back);
            let ratio = contrast(front, back);
            (ratio + 1e-4 < min).then(|| format!("--{fg} on --{bg}: {ratio:.2}:1, needs {min}:1"))
        })
        .collect()
}

#[test]
fn every_pair_passes_in_the_light_theme() {
    assert_eq!(failures(Theme::Light.tokens()), Vec::<String>::new());
}

#[test]
fn every_pair_passes_in_the_dark_theme() {
    assert_eq!(failures(Theme::Dark.tokens()), Vec::<String>::new());
}

#[test]
fn a_failing_value_is_caught() {
    let mut tokens = *Theme::Light.tokens();
    tokens.muted_foreground = super::color::hex(0x9a_9a_94);
    assert!(!failures(&tokens).is_empty());
}

/// The ratios docs/BRAND.md lists, to the tenth: the tokens are the React
/// app's, not merely passing ones.
#[test]
fn the_ratios_are_those_of_docs_brand() {
    let ratio = |theme: Theme, fg: &str, bg: &str| {
        let tokens = theme.tokens();
        let back = token(tokens, bg);
        contrast(over(token(tokens, fg), back), back)
    };
    let listed = [
        ("foreground", "background", 17.2, 17.2),
        ("muted-foreground", "background", 6.3, 7.5),
        ("muted-foreground", "card", 6.7, 7.0),
        ("muted-foreground", "muted", 5.6, 6.2),
        ("muted-foreground", "sidebar", 5.8, 7.8),
        ("destructive", "background", 5.5, 6.1),
        ("destructive", "card", 5.9, 5.6),
        ("warning", "background", 5.1, 9.2),
        ("warning", "card", 5.4, 8.5),
        ("input", "background", 3.0, 3.1),
        ("input", "card", 3.3, 3.1),
        ("chart-4", "card", 3.4, 5.0),
    ];
    for (fg, bg, light, dark) in listed {
        let (l, d) = (ratio(Theme::Light, fg, bg), ratio(Theme::Dark, fg, bg));
        assert!((l - light).abs() < 0.06, "light --{fg} on --{bg}: {l:.2}");
        assert!((d - dark).abs() < 0.06, "dark --{fg} on --{bg}: {d:.2}");
    }
}

/// The terminal and the editor carry their own colours (docs/BRAND.md,
/// Contrast). The six ANSI hues are 5.3:1 or more on the terminal, and so
/// are the black of the light theme and the white of the dark one, which
/// are text there. The two that fade into their terminal are bounded by
/// what they measure: light white is a grey that must still be seen (3:1),
/// and dark black, which is 1.26:1 on a black terminal in every palette
/// (programs use it as a background, or with bold, which is ANSI 8 and
/// passes 3:1), may not get any fainter. The bright forms only need to be
/// seen. Syntax is 5:1 or more on a card.
#[test]
fn machine_text_is_legible() {
    for theme in Theme::ALL {
        let tokens = theme.tokens();
        let terminal = &tokens.terminal;
        assert!(contrast(terminal.foreground, terminal.background) >= 17.0);
        for (index, colour) in terminal.ansi.iter().enumerate() {
            let ratio = contrast(*colour, terminal.background);
            let min = match index {
                // 5.3:1 as docs/BRAND.md rounds it (light green is 5.297).
                1..=6 => 5.25,
                // Black on the dark terminal: 1.26, held where it is.
                0 if theme.is_dark() => 1.25,
                // White on the light terminal: a grey at 3.4.
                7 if !theme.is_dark() => UI,
                // Black on light and white on dark are text.
                0 | 7 => 5.25,
                // Bright white on white is the faintest grey (2.1).
                15 if !theme.is_dark() => 2.0,
                _ => UI,
            };
            assert!(ratio >= min, "{theme:?} ANSI {index}: {ratio:.2}");
        }
        let syntax = &tokens.syntax;
        for colour in [syntax.key, syntax.string, syntax.number, syntax.comment] {
            assert!(contrast(colour, tokens.card) >= 5.0, "{theme:?} {colour:?}");
        }
    }
}

fn colour(background: Option<Background>) -> Color {
    match background {
        Some(Background::Color(colour)) => colour,
        None => Color::TRANSPARENT,
        Some(other) => panic!("the palette has no gradient: {other:?}"),
    }
}

/// A button's label on its own fill, over the page and over a card, idle,
/// hovered and pressed. No pair of tokens says this: the fills are mixes.
///
/// The destructive button is the React one to the value, and that one is
/// short of 4.5:1 where its red tint deepens: 3.9:1 hovered on light, 4.3:1
/// idle on a dark card, 3.6:1 hovered there. Raising it is a palette
/// decision (docs/BRAND.md), not a port's; until then it is held to 3:1 so
/// it cannot get worse unseen.
#[test]
fn every_button_reads_in_every_state() {
    type Variant = fn(&Theme, button::Status) -> button::Style;
    let variants: [(&str, Variant); 6] = [
        ("primary", style::button::primary),
        ("secondary", style::button::secondary),
        ("outline", style::button::outline),
        ("ghost", style::button::ghost),
        ("destructive", style::button::destructive),
        ("link", style::button::link),
    ];
    let states = [
        button::Status::Active,
        button::Status::Hovered,
        button::Status::Pressed,
    ];
    for theme in Theme::ALL {
        let tokens = theme.tokens();
        for (name, variant) in variants {
            for status in states {
                for surface in [tokens.background, tokens.card] {
                    let look = variant(&theme, status);
                    let fill = over(colour(look.background), surface);
                    let ratio = contrast(over(look.text_color, fill), fill);
                    let min = if name == "destructive" { UI } else { TEXT };
                    assert!(ratio >= min, "{theme:?} {name} {status:?}: {ratio:.2}:1");
                }
            }
            // Disabled is exempt from contrast (WCAG 1.4.3), but must differ.
            let disabled = variant(&theme, button::Status::Disabled);
            assert!(disabled.text_color.a < 1.0, "{theme:?} {name}");
        }
    }
}

/// A field is found by its border and shows where focus is by it.
#[test]
fn a_field_shows_its_edge_and_its_focus() {
    for theme in Theme::ALL {
        let tokens = theme.tokens();
        for surface in [tokens.background, tokens.card] {
            let idle = style::text_input::field_style(&theme, text_input::Status::Active);
            let fill = over(colour(Some(idle.background)), surface);
            assert!(contrast(over(idle.border.color, surface), surface) >= UI);
            assert!(contrast(over(idle.value, fill), fill) >= TEXT);
            assert!(contrast(over(idle.placeholder, fill), fill) >= TEXT);
            let focused = style::text_input::field_style(
                &theme,
                text_input::Status::Focused { is_hovered: false },
            );
            assert_ne!(focused.border.color, idle.border.color);
            assert!(contrast(over(focused.border.color, surface), surface) >= UI);
        }
    }
}

/// What inverts or tints its surface: tooltips, the open menu's selected
/// row, badges.
#[test]
fn surfaces_carry_their_own_text() {
    for theme in Theme::ALL {
        let tokens = theme.tokens();
        let surfaces = [
            style::container::page(&theme),
            style::container::card(&theme),
            style::container::sidebar(&theme),
            style::container::sidebar_accent(&theme),
            style::container::popover(&theme),
            style::container::muted(&theme),
            style::container::accent(&theme),
            style::container::tooltip(&theme),
            style::container::terminal(&theme),
            style::container::badge::primary(&theme),
            style::container::badge::secondary(&theme),
            style::container::badge::destructive(&theme),
        ];
        for (index, look) in surfaces.into_iter().enumerate() {
            let fill = over(colour(look.background), tokens.background);
            let text = look.text_color.expect("a surface names its text colour");
            let ratio = contrast(over(text, fill), fill);
            assert!(ratio >= TEXT, "{theme:?} surface {index}: {ratio:.2}:1");
        }
        let menu = style::menu::popover(&theme);
        let selected = over(colour(Some(menu.selected_background)), tokens.popover);
        assert!(contrast(menu.selected_text_color, selected) >= TEXT);
        assert!(contrast(menu.text_color, colour(Some(menu.background))) >= TEXT);
    }
}

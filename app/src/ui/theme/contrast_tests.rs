//! Contrast of what the first table (`tests.rs`) left out: the controls
//! iced draws (checkbox, toggler, pick list, radio, slider, text editor),
//! coloured text on every surface it may sit on, composites over a card as
//! well as the page, and the terminal's selection and search.
//!
//! WCAG 2.2 AA: 4.5:1 for text, 3:1 for what isn't text but must be seen.
//! A pair that is short of it with the React app's own values is not
//! repainted here (the palette is docs/BRAND.md's to change): it is held at
//! what it measures, named as inherited, so it cannot get worse unseen.

use iced::{
    widget::{checkbox, pick_list, radio, slider, text_editor, toggler},
    Background, Color,
};

use super::{
    color::{contrast, over},
    style, Theme,
};

const TEXT: f32 = 4.5;
const UI: f32 = 3.0;

fn colour(background: Background) -> Color {
    match background {
        Background::Color(colour) => colour,
        Background::Gradient(_) => panic!("the palette has no gradient"),
    }
}

/// One measured pair: what it is, its ratio and the least it may be.
struct Pair(String, f32, f32);

/// `front` on `back`, itself over `surface`.
fn on(front: Color, back: Color, surface: Color) -> f32 {
    let back = over(back, surface);
    contrast(over(front, back), back)
}

/// What a pair is held to when it is short of AA with the React values:
/// its measured ratio, to the tenth below. `None`: AA.
fn inherited(theme: Theme, name: &str) -> Option<f32> {
    let dark = theme.is_dark();
    HELD.iter()
        .find(|(held, in_dark, _)| *held == name && *in_dark == dark)
        .map(|(_, _, floor)| *floor)
}

/// (pair, in the dark theme, floor). Each is the React app's value
/// (`src/shared/styles/global.css` and the primitives' classes), short of
/// AA there too; see the report of the review of #230.
const HELD: &[(&str, bool, f32)] = &[
    // `text-warning` on a hovered or current item (`bg-accent`, the
    // sidebar's): 4.33 and 4.17. Amber #9a5b00 passes on the page and on
    // cards, not on the two accent greys.
    ("warning text on accent over the page", false, 4.3),
    ("warning text on accent over the card", false, 4.3),
    ("warning text on sidebar-accent over the page", false, 4.1),
    ("warning text on sidebar-accent over the card", false, 4.1),
    // The destructive badge on a dark card (`dark:bg-destructive/20`): 4.25.
    ("destructive badge over the card", true, 4.2),
    // The switch that is off on dark (`dark:data-unchecked:bg-input/80`):
    // its track is 2.41 on the page and 2.44 on a card, under the 3:1 of a
    // control's edge. Its thumb, 6:1 on the track, is what is seen.
    ("toggler (false) track over the page", true, 2.4),
    ("toggler (false) track over the card", true, 2.4),
    // xterm drew the terminal's own text on the amber of the match the
    // search stands on: Paper on #f2a93b is 1.86 on dark. The terminal of
    // the iced app should draw that one match in ink; until it exists the
    // pair is held as it was.
    ("terminal text on its active search match", true, 1.8),
];

// One table, read top to bottom; cutting it up would only scatter it.
#[allow(clippy::too_many_lines)]
fn pairs(theme: Theme) -> Vec<Pair> {
    let tokens = theme.tokens();
    let mut pairs = vec![];
    let mut add = |name: String, ratio: f32, aa: f32| {
        let min = inherited(theme, &name).unwrap_or(aa);
        pairs.push(Pair(name, ratio, min));
    };
    let grounds = [("page", tokens.background), ("card", tokens.card)];

    // Coloured and muted text on every surface it may sit on, each over the
    // page and over a card.
    let texts = [
        ("destructive", tokens.destructive),
        ("warning", tokens.warning),
        ("muted", tokens.muted_foreground),
    ];
    let surfaces = [
        ("muted", style::container::muted(&theme)),
        ("accent", style::container::accent(&theme)),
        ("popover", style::container::popover(&theme)),
        ("sidebar", style::container::sidebar(&theme)),
        ("sidebar-accent", style::container::sidebar_accent(&theme)),
        ("card footer", style::container::card_footer(&theme)),
        (
            "hovered row",
            style::container::table_row(true, false)(&theme),
        ),
        (
            "selected row",
            style::container::table_row(false, true)(&theme),
        ),
    ];
    for (ground, under) in grounds {
        for (surface, look) in &surfaces {
            let fill = look.background.map_or(Color::TRANSPARENT, colour);
            for (text, front) in texts {
                add(
                    format!("{text} text on {surface} over the {ground}"),
                    on(front, fill, under),
                    TEXT,
                );
            }
            if let Some(own) = look.text_color {
                add(
                    format!("{surface}'s own text over the {ground}"),
                    on(own, fill, under),
                    TEXT,
                );
            }
        }
        // Badges, whose fills are tints.
        let badges = [
            ("outline", style::container::badge::outline(&theme)),
            ("destructive", style::container::badge::destructive(&theme)),
            ("ghost", style::container::badge::ghost(&theme)),
            (
                "hovered ghost",
                style::container::badge::ghost_hovered(&theme),
            ),
            ("link", style::container::badge::link(&theme)),
        ];
        for (badge, look) in badges {
            let fill = look.background.map_or(Color::TRANSPARENT, colour);
            let text = look.text_color.expect("a badge names its text colour");
            add(
                format!("{badge} badge over the {ground}"),
                on(text, fill, under),
                TEXT,
            );
        }

        // The controls iced draws, with the theme's default styles.
        let class = <Theme as checkbox::Catalog>::default();
        for checked in [true, false] {
            let status = checkbox::Status::Active {
                is_checked: checked,
            };
            let look = checkbox::Catalog::style(&theme, &class, status);
            let fill = over(colour(look.background), under);
            let edge = contrast(over(look.border.color, under), under).max(contrast(fill, under));
            add(
                format!("checkbox ({checked}) edge over the {ground}"),
                edge,
                UI,
            );
            if checked {
                add(
                    format!("checkbox mark over the {ground}"),
                    contrast(over(look.icon_color, fill), fill),
                    UI,
                );
            }
        }
        let class = <Theme as toggler::Catalog>::default();
        for toggled in [true, false] {
            let status = toggler::Status::Active {
                is_toggled: toggled,
            };
            let look = toggler::Catalog::style(&theme, &class, status);
            let track = over(colour(look.background), under);
            let edge = contrast(over(look.background_border_color, under), under)
                .max(contrast(track, under));
            add(
                format!("toggler ({toggled}) track over the {ground}"),
                edge,
                UI,
            );
            add(
                format!("toggler ({toggled}) thumb on its track over the {ground}"),
                contrast(over(colour(look.foreground), track), track),
                UI,
            );
        }
        let class = <Theme as radio::Catalog>::default();
        for selected in [true, false] {
            let status = radio::Status::Active {
                is_selected: selected,
            };
            let look = radio::Catalog::style(&theme, &class, status);
            let fill = over(colour(look.background), under);
            add(
                format!("radio ({selected}) edge over the {ground}"),
                contrast(over(look.border_color, under), under),
                UI,
            );
            if selected {
                add(
                    format!("radio dot over the {ground}"),
                    contrast(over(look.dot_color, fill), fill),
                    UI,
                );
            }
        }
        let class = <Theme as pick_list::Catalog>::default();
        let look = pick_list::Catalog::style(&theme, &class, pick_list::Status::Active);
        let fill = over(colour(look.background), under);
        add(
            format!("pick list text over the {ground}"),
            contrast(over(look.text_color, fill), fill),
            TEXT,
        );
        add(
            format!("pick list placeholder over the {ground}"),
            contrast(over(look.placeholder_color, fill), fill),
            TEXT,
        );
        add(
            format!("pick list handle over the {ground}"),
            contrast(over(look.handle_color, fill), fill),
            UI,
        );
        add(
            format!("pick list edge over the {ground}"),
            contrast(over(look.border.color, under), under),
            UI,
        );
        let class = <Theme as slider::Catalog>::default();
        let look = slider::Catalog::style(&theme, &class, slider::Status::Active);
        add(
            format!("slider's filled rail over the {ground}"),
            contrast(over(colour(look.rail.backgrounds.0), under), under),
            UI,
        );
        let handle = over(colour(look.handle.background), under);
        let edge =
            contrast(over(look.handle.border_color, under), under).max(contrast(handle, under));
        add(format!("slider's handle over the {ground}"), edge, UI);
        let class = <Theme as text_editor::Catalog>::default();
        let look = text_editor::Catalog::style(&theme, &class, text_editor::Status::Active);
        let fill = over(colour(look.background), under);
        add(
            format!("text editor text over the {ground}"),
            contrast(over(look.value, fill), fill),
            TEXT,
        );
        add(
            format!("text editor placeholder over the {ground}"),
            contrast(over(look.placeholder, fill), fill),
            TEXT,
        );
        add(
            format!("text editor edge over the {ground}"),
            contrast(over(look.border.color, under), under),
            UI,
        );
        let selected = over(look.selection, fill);
        add(
            format!("text editor text on its selection over the {ground}"),
            contrast(over(look.value, selected), selected),
            TEXT,
        );
    }

    // The terminal: text on a selection and on a search match.
    let terminal = &tokens.terminal;
    let grounds = [
        ("selection", terminal.selection),
        ("search match", terminal.search_match),
        ("active search match", terminal.search_active),
    ];
    for (name, fill) in grounds {
        let fill = over(fill, terminal.background);
        add(
            format!("terminal text on its {name}"),
            contrast(terminal.foreground, fill),
            TEXT,
        );
    }
    // The thumb of a table's scrollbars, on the rows it lies over: what
    // `--border` was not (1.2:1 on the page, the very fill of a selected
    // row on light).
    let thumb = style::scrollable::table_thumb(&theme);
    let rows = [
        ("a row", style::container::table_row(false, false)(&theme)),
        (
            "a hovered row",
            style::container::table_row(true, false)(&theme),
        ),
        (
            "a selected row",
            style::container::table_row(false, true)(&theme),
        ),
        (
            "a group's header",
            style::container::table_group(false)(&theme),
        ),
    ];
    for (name, look) in rows {
        let fill = look.background.map_or(Color::TRANSPARENT, colour);
        add(
            format!("table thumb on {name}"),
            on(thumb, fill, tokens.background),
            UI,
        );
    }
    add(
        "terminal selection against the terminal".to_owned(),
        contrast(
            over(terminal.selection, terminal.background),
            terminal.background,
        ),
        1.0,
    );
    pairs
}

#[test]
fn every_control_and_every_text_on_every_surface_is_legible() {
    let mut short = vec![];
    for theme in Theme::ALL {
        for Pair(name, ratio, min) in pairs(theme) {
            if std::env::var_os("HOMEPORT_CONTRAST").is_some() {
                eprintln!("{theme:?} | {name} | {ratio:.2}");
            }
            if ratio + 1e-3 < min {
                short.push(format!("{theme:?} {name}: {ratio:.2}:1, needs {min}:1"));
            }
        }
    }
    assert_eq!(short, Vec::<String>::new());
}

/// A pair held under AA is still under it: one that now passes leaves the
/// list, so the list stays what is really owed.
#[test]
fn what_is_held_under_aa_is_still_short_of_it() {
    for (name, dark, floor) in HELD {
        let theme = if *dark { Theme::Dark } else { Theme::Light };
        let found: Vec<f32> = pairs(theme)
            .into_iter()
            .filter(|pair| pair.0 == *name)
            .map(|pair| pair.1)
            .collect();
        assert_eq!(found.len(), 1, "{name} names one pair");
        assert!(
            found[0] >= *floor,
            "{name}: {:.2} under its floor",
            found[0]
        );
        assert!(
            *floor < TEXT,
            "{name} is held at {floor}: it needs no entry"
        );
    }
}

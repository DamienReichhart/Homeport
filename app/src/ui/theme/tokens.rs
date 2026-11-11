//! Homeport's palette (docs/BRAND.md): Ink and Paper, and one Signal red that
//! means "this needs you". Healthy things get no colour; warning is amber;
//! nothing else is coloured.
//!
//! Every value is the one of the React app, copied from
//! `src/shared/styles/global.css` (`:root` and `.dark`), from the terminal's
//! palette (`shared/ui/terminal/XTerm.tsx`) and from the editor's
//! (`shared/ui/editor/editorTheme.ts`). They are all sRGB hex there, so they
//! are hex here; the dark theme's three translucent borders keep their alpha.

use iced::Color;

use super::color::{hex, hex_alpha};

/// Every colour of one theme, under its CSS name (`--muted-foreground` is
/// `muted_foreground`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tokens {
    pub background: Color,
    pub foreground: Color,
    pub card: Color,
    pub card_foreground: Color,
    pub popover: Color,
    pub popover_foreground: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub destructive: Color,
    pub border: Color,
    /// A field's only boundary: 3:1 against cards and the page (WCAG 1.4.11).
    pub input: Color,
    pub ring: Color,
    pub signal: Color,
    pub success: Color,
    pub warning: Color,
    pub info: Color,
    /// Charts stay neutral: a series is ink or grey (dashed), never a status
    /// colour. `chart[0]` is `--chart-1`.
    pub chart: [Color; 5],
    pub sidebar: Sidebar,
    pub terminal: Terminal,
    pub syntax: Syntax,
}

/// The `--sidebar-*` tokens.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sidebar {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub border: Color,
    pub ring: Color,
}

/// The terminal's palette. The 16 ANSI colours belong to the programs that
/// print them, so they stay distinct; red is Signal red.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Terminal {
    pub background: Color,
    pub foreground: Color,
    pub cursor: Color,
    pub cursor_accent: Color,
    pub selection: Color,
    /// Black, red, green, yellow, blue, magenta, cyan, white, then their
    /// bright forms: the index is the ANSI colour number.
    pub ansi: [Color; 16],
    /// A search match, and the one the search stands on (amber).
    pub search_match: Color,
    pub search_active: Color,
}

/// The editor's syntax colours: keys in the text colour, values in two quiet
/// hues, comments grey (and italic). Red, amber and green never mean syntax.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Syntax {
    pub key: Color,
    pub string: Color,
    pub number: Color,
    pub atom: Color,
    pub comment: Color,
    pub punctuation: Color,
}

/// The mark's ring, fixed on both backgrounds (docs/BRAND.md, Colour).
pub const MARK: Color = hex(0xe1_00_1a);

const INK: Color = hex(0x14_14_14);
const PAPER: Color = hex(0xf7_f7_f4);
const WHITE: Color = hex(0xff_ff_ff);
/// The grey chart series and the terminal's search matches, in both themes.
const GREY: Color = hex(0x8c_8c_86);

pub const LIGHT: Tokens = Tokens {
    background: PAPER,
    foreground: INK,
    card: WHITE,
    card_foreground: INK,
    popover: WHITE,
    popover_foreground: INK,
    primary: INK,
    primary_foreground: PAPER,
    secondary: hex(0xeb_eb_e6),
    secondary_foreground: INK,
    muted: hex(0xeb_eb_e6),
    muted_foreground: hex(0x5c_5c_57),
    accent: hex(0xe6_e6_e0),
    accent_foreground: INK,
    destructive: hex(0xc8_10_2e),
    border: hex(0xda_da_d3),
    input: hex(0x8f_8f_87),
    ring: INK,
    signal: hex(0xc8_10_2e),
    success: INK,
    warning: hex(0x9a_5b_00),
    info: hex(0x5c_5c_57),
    chart: [INK, INK, INK, GREY, GREY],
    sidebar: Sidebar {
        background: hex(0xef_ef_ea),
        foreground: INK,
        primary: INK,
        primary_foreground: PAPER,
        accent: hex(0xe2_e2_db),
        accent_foreground: INK,
        border: hex(0xda_da_d3),
        ring: INK,
    },
    terminal: Terminal {
        background: WHITE,
        foreground: INK,
        cursor: INK,
        cursor_accent: WHITE,
        selection: hex(0xd9_d9_d2),
        ansi: [
            INK,
            hex(0xc8_10_2e),
            hex(0x2f_7a_3a),
            hex(0x8f_5a_00),
            hex(0x1f_5a_96),
            hex(0x8a_3a_8f),
            hex(0x1f_73_80),
            GREY,
            hex(0x5c_5c_57),
            hex(0xe1_00_1a),
            hex(0x3a_8f_47),
            hex(0xa8_6b_00),
            hex(0x2b_6c_b0),
            hex(0xa3_4b_a8),
            hex(0x27_8a_99),
            hex(0xb5_b5_ae),
        ],
        search_match: hex_alpha(0x8c_8c_86, 0.4),
        search_active: hex(0xf2_a9_3b),
    },
    syntax: Syntax {
        key: INK,
        string: hex(0x1f_5a_96),
        number: hex(0x8a_3a_8f),
        atom: hex(0x8a_3a_8f),
        comment: hex(0x6e_6e_68),
        punctuation: hex(0x6e_6e_68),
    },
};

pub const DARK: Tokens = Tokens {
    background: INK,
    foreground: PAPER,
    card: hex(0x1c_1c_1b),
    card_foreground: PAPER,
    popover: hex(0x1f_1f_1e),
    popover_foreground: PAPER,
    primary: PAPER,
    primary_foreground: INK,
    secondary: hex(0x26_26_24),
    secondary_foreground: PAPER,
    muted: hex(0x26_26_24),
    muted_foreground: hex(0xa6_a6_a0),
    accent: hex(0x2a_2a_28),
    accent_foreground: PAPER,
    destructive: hex(0xff_5c_5c),
    border: hex_alpha(0xf7_f7_f4, 0.10),
    input: hex_alpha(0xf7_f7_f4, 0.35),
    ring: PAPER,
    signal: hex(0xff_5c_5c),
    success: PAPER,
    warning: hex(0xf2_a9_3b),
    info: hex(0xa6_a6_a0),
    chart: [PAPER, PAPER, PAPER, GREY, GREY],
    sidebar: Sidebar {
        background: hex(0x0f_0f_0f),
        foreground: PAPER,
        primary: PAPER,
        primary_foreground: INK,
        accent: hex(0x26_26_24),
        accent_foreground: PAPER,
        border: hex_alpha(0xf7_f7_f4, 0.08),
        ring: PAPER,
    },
    terminal: Terminal {
        background: hex(0x0f_0f_0f),
        foreground: PAPER,
        cursor: PAPER,
        cursor_accent: hex(0x0f_0f_0f),
        selection: hex(0x3a_3a_37),
        ansi: [
            hex(0x26_26_24),
            hex(0xff_5c_5c),
            hex(0x6c_c7_7d),
            hex(0xf2_a9_3b),
            hex(0x7a_a7_e0),
            hex(0xcf_8f_d6),
            hex(0x6c_c4_cf),
            hex(0xd6_d6_d0),
            hex(0x6e_6e_69),
            hex(0xff_80_80),
            hex(0x8f_dc_9d),
            hex(0xff_c4_6b),
            hex(0x9d_c0_ee),
            hex(0xe0_ae_e6),
            hex(0x93_d9_e2),
            WHITE,
        ],
        search_match: hex_alpha(0x8c_8c_86, 0.4),
        search_active: hex(0xf2_a9_3b),
    },
    syntax: Syntax {
        key: PAPER,
        string: hex(0x9d_c0_ee),
        number: hex(0xe0_ae_e6),
        atom: hex(0xe0_ae_e6),
        comment: GREY,
        punctuation: GREY,
    },
};

/// What a status says (docs/BRAND.md, Status). A status is a shape as well
/// as a colour; this is the colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tone {
    /// Running, connected, healthy: ink, because healthy is the default.
    Success,
    /// Stopped, exited, disabled.
    Muted,
    /// Connecting, not deployed.
    Info,
    /// Starting, paused, restarting, a key to check.
    Warning,
    /// Unhealthy, unreachable, refused, failed: the only red.
    Danger,
}

impl Tone {
    /// The colour `StatusDot` draws this tone in.
    pub fn colour(self, tokens: &Tokens) -> Color {
        match self {
            Self::Success => tokens.success,
            Self::Muted => tokens.muted_foreground,
            Self::Info => tokens.foreground,
            Self::Warning => tokens.warning,
            Self::Danger => tokens.destructive,
        }
    }
}

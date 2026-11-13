//! The plain parts: buttons, badges, cards, separators, progress bars,
//! scroll areas, rows that wrap and text cut with an ellipsis, as `src/shared/ui/primitives` drew them.

pub mod button;
pub mod fit;
pub mod flow;
pub mod shortcut;
pub mod surface;
pub mod truncate;

pub use self::{
    fit::{fit, Fit},
    flow::{flow, Flow},
    shortcut::shortcut,
    surface::{
        badge, card, collapsible, keys, progress, scroll_area, separator, separator_vertical,
        BadgeVariant,
    },
    truncate::{truncated, Truncated},
};

//! The plain parts: buttons, badges, cards, separators, progress bars,
//! scroll areas and text cut with an ellipsis, as `src/shared/ui/primitives` drew them.

pub mod button;
pub mod fit;
pub mod surface;
pub mod truncate;

pub use self::{
    fit::{fit, Fit},
    surface::{
        badge, card, collapsible, keys, progress, scroll_area, separator, separator_vertical,
        BadgeVariant,
    },
    truncate::{truncated, Truncated},
};

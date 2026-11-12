//! The plain parts: buttons, badges, cards, separators, progress bars and
//! scroll areas, as `src/shared/ui/primitives` drew them.

pub mod button;
pub mod fit;
pub mod surface;

pub use self::{
    fit::{fit, Fit},
    surface::{
        badge, card, collapsible, keys, progress, scroll_area, separator, separator_vertical,
        BadgeVariant,
    },
};

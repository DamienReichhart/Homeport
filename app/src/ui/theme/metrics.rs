//! Sizes, in logical pixels: the spacing, radius and type scales and the
//! heights of controls, as `src/shared/ui/primitives` draws them (Tailwind's
//! 4 px unit at a 16 px root). A layout names one of these, never a number.

/// Tailwind's spacing steps the primitives use: `gap-1.5` is `space::S1_5`.
pub mod space {
    pub const S0_5: f32 = 2.0;
    pub const S1: f32 = 4.0;
    pub const S1_5: f32 = 6.0;
    pub const S2: f32 = 8.0;
    pub const S2_5: f32 = 10.0;
    pub const S3: f32 = 12.0;
    pub const S4: f32 = 16.0;
    pub const S5: f32 = 20.0;
    pub const S6: f32 = 24.0;
    pub const S8: f32 = 32.0;
}

/// Corner radii, from `--radius: 0.25rem` (docs/BRAND.md, Shape).
pub mod radius {
    /// `rounded-sm`: a key cap in a tooltip.
    pub const SM: f32 = 2.4;
    /// `rounded-md`: small buttons, menu items, tooltips.
    pub const MD: f32 = 3.2;
    /// `rounded-lg`, the brand's 4 px: buttons, fields, popovers.
    pub const LG: f32 = 4.0;
    /// `rounded-xl`: cards.
    pub const XL: f32 = 5.6;
    /// `rounded-full`: toggles, progress bars, scrollbar thumbs.
    pub const FULL: f32 = 9999.0;
}

/// Text sizes, with the line height Tailwind pairs each with.
pub mod text {
    /// `text-xs`: tooltips, badges, the editor and the terminal.
    pub const XS: f32 = 12.0;
    /// `text-[0.8rem]`: a small button's label.
    pub const SM_BUTTON: f32 = 12.8;
    /// `text-sm`: the interface's body size.
    pub const SM: f32 = 14.0;
    /// `text-base`: card and dialog titles.
    pub const BASE: f32 = 16.0;
    pub const LG: f32 = 18.0;
    pub const XL: f32 = 20.0;
    pub const XXL: f32 = 24.0;

    /// Line height over font size, for `iced::widget::text::LineHeight::Relative`.
    pub const LEADING_XS: f32 = 16.0 / 12.0;
    pub const LEADING_SM: f32 = 20.0 / 14.0;
    pub const LEADING_BASE: f32 = 24.0 / 16.0;
    /// The editor's and the terminal's lines (`lineHeight: 1.6`).
    pub const LEADING_MACHINE: f32 = 1.6;
}

/// Heights and sizes of the controls.
pub mod control {
    /// `h-8`: a button, a field, a select.
    pub const HEIGHT: f32 = 32.0;
    pub const HEIGHT_XS: f32 = 24.0;
    pub const HEIGHT_SM: f32 = 28.0;
    pub const HEIGHT_LG: f32 = 36.0;
    /// A field's and a default button's horizontal padding (`px-2.5`).
    pub const PADDING_X: f32 = 10.0;
    /// `size-4`: a checkbox, and an icon in a default control.
    pub const CHECKBOX: f32 = 16.0;
    /// The switch is 32 by 18.4 with a 16 px thumb.
    pub const SWITCH_HEIGHT: f32 = 18.4;
    pub const SWITCH_HEIGHT_SM: f32 = 14.0;
    /// `h-1`: the progress bar.
    pub const PROGRESS: f32 = 4.0;
    /// `w-2.5`: a scrollbar, its thumb inset by one pixel.
    pub const SCROLLBAR: f32 = 10.0;
    pub const SCROLLER: f32 = 8.0;
    /// `min-h-16`: a multi-line field.
    pub const TEXTAREA_MIN: f32 = 64.0;
    /// `h-5`: a badge.
    pub const BADGE: f32 = 20.0;
}

/// Icon sizes: Lucide's `size-4`, and the smaller ones of small buttons.
pub mod icon {
    pub const XS: f32 = 12.0;
    pub const SM: f32 = 14.0;
    pub const MD: f32 = 16.0;
    pub const LG: f32 = 20.0;
}

/// The React `Button`'s `size`: what a button of that size measures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Xs,
    Sm,
    Lg,
}

impl ButtonSize {
    /// Also the width of the icon-only form (`size-8`, `size-6`…).
    pub fn height(self) -> f32 {
        match self {
            Self::Default => control::HEIGHT,
            Self::Xs => control::HEIGHT_XS,
            Self::Sm => control::HEIGHT_SM,
            Self::Lg => control::HEIGHT_LG,
        }
    }

    pub fn padding_x(self) -> f32 {
        match self {
            Self::Xs => space::S2,
            Self::Default | Self::Sm | Self::Lg => space::S2_5,
        }
    }

    pub fn text(self) -> f32 {
        match self {
            Self::Default | Self::Lg => text::SM,
            Self::Xs => text::XS,
            Self::Sm => text::SM_BUTTON,
        }
    }

    /// Between an icon and the label.
    pub fn gap(self) -> f32 {
        match self {
            Self::Default | Self::Lg => space::S1_5,
            Self::Xs | Self::Sm => space::S1,
        }
    }

    pub fn icon(self) -> f32 {
        match self {
            Self::Default | Self::Lg => icon::MD,
            Self::Xs => icon::XS,
            Self::Sm => icon::SM,
        }
    }

    pub fn radius(self) -> f32 {
        match self {
            Self::Default | Self::Lg => radius::LG,
            Self::Xs | Self::Sm => radius::MD,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_sizes_are_the_react_ones() {
        let sizes = [
            (ButtonSize::Default, 32.0, 10.0, 14.0, 16.0),
            (ButtonSize::Xs, 24.0, 8.0, 12.0, 12.0),
            (ButtonSize::Sm, 28.0, 10.0, 12.8, 14.0),
            (ButtonSize::Lg, 36.0, 10.0, 14.0, 16.0),
        ];
        for (size, height, padding, text, icon) in sizes {
            assert_eq!(size.height(), height);
            assert_eq!(size.padding_x(), padding);
            assert_eq!(size.text(), text);
            assert_eq!(size.icon(), icon);
        }
        assert_eq!(radius::LG, 4.0);
    }
}

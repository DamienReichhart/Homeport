//! How large the window draws. The React app has no text-size or density
//! setting (`Settings` holds none, and neither does `features/settings`), so
//! none is invented here: HiDPI is winit's monitor factor, which iced applies
//! by itself, and this is the one factor a later setting would set.

/// A factor over the monitor's own: 1 draws at the size the OS asks for.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scale(f32);

impl Scale {
    /// Below this the 12 px machine text stops being legible.
    pub const MIN: f32 = 0.75;
    /// Above this the default window no longer fits the sidebar and a table.
    pub const MAX: f32 = 2.0;

    /// `factor` kept within what the layout holds; 1 for what is no number.
    pub fn new(factor: f32) -> Self {
        if factor.is_finite() {
            Self(factor.clamp(Self::MIN, Self::MAX))
        } else {
            Self::default()
        }
    }

    /// What `iced::application(..).scale_factor(..)` returns: iced multiplies
    /// the monitor's factor by it, so text, spacing and icons scale together.
    pub fn factor(self) -> f32 {
        self.0
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Scale;

    #[test]
    fn a_factor_stays_within_what_the_layout_holds() {
        assert_eq!(Scale::default().factor(), 1.0);
        assert_eq!(Scale::new(1.25).factor(), 1.25);
        assert_eq!(Scale::new(0.1).factor(), Scale::MIN);
        assert_eq!(Scale::new(9.0).factor(), Scale::MAX);
        assert_eq!(Scale::new(f32::NAN).factor(), 1.0);
    }
}

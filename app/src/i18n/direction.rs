//! The writing direction of the app's language. Layouts ask it where
//! "start" and "end" are, so nothing branches on the language itself.

/// Left to right, or right to left (Arabic), which mirrors the layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl Direction {
    pub fn is_rtl(self) -> bool {
        self == Self::Rtl
    }

    /// One of two values by direction: `dir.pick(Left, Right)` is the side
    /// text starts on.
    pub fn pick<T>(self, ltr: T, rtl: T) -> T {
        match self {
            Self::Ltr => ltr,
            Self::Rtl => rtl,
        }
    }

    /// `items` written start to end, in the order a row draws them left to
    /// right.
    pub fn ordered<T>(self, mut items: Vec<T>) -> Vec<T> {
        if self.is_rtl() {
            items.reverse();
        }
        items
    }

    /// A (start, end) pair as (left, right): paddings, margins, corners.
    pub fn sides<T>(self, start: T, end: T) -> (T, T) {
        match self {
            Self::Ltr => (start, end),
            Self::Rtl => (end, start),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Direction::{Ltr, Rtl};

    #[test]
    fn right_to_left_mirrors_rows_and_sides() {
        assert_eq!(Ltr.ordered(vec![1, 2, 3]), [1, 2, 3]);
        assert_eq!(Rtl.ordered(vec![1, 2, 3]), [3, 2, 1]);
        assert_eq!(Ltr.sides(8, 2), (8, 2));
        assert_eq!(Rtl.sides(8, 2), (2, 8));
        assert_eq!(Rtl.pick("left", "right"), "right");
        assert!(!Ltr.is_rtl());
    }
}

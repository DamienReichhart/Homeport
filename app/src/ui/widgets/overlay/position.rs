//! Where a floating panel goes: beside what it belongs to, on the side
//! asked for when it fits there, on the other side when it only fits there,
//! and never past an edge of the window. Every popover, menu, select list
//! and tooltip is placed here, so the rule is written once.

use iced::{Point, Rectangle, Size};

use crate::i18n::Direction;

/// The side of the anchor the panel sits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    /// The side text ends on: where a submenu opens.
    pub fn end(direction: Direction) -> Self {
        direction.pick(Self::Right, Self::Left)
    }

    fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn is_vertical(self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }
}

/// How the panel lines up with the anchor along the side it sits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    /// Left edges together (top edges, beside the anchor).
    Near,
    Center,
    /// Right edges together (bottom edges, beside the anchor).
    Far,
}

impl Align {
    /// Lined up where text starts.
    pub fn start(direction: Direction) -> Self {
        direction.pick(Self::Near, Self::Far)
    }

    /// Lined up where text ends.
    pub fn end(direction: Direction) -> Self {
        direction.pick(Self::Far, Self::Near)
    }
}

/// Where a panel is asked to go.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub side: Side,
    pub align: Align,
    /// Between the anchor and the panel.
    pub gap: f32,
    /// Kept free between the panel and the window's edges.
    pub margin: f32,
}

impl Placement {
    pub const fn new(side: Side, align: Align) -> Self {
        Self {
            side,
            align,
            gap: 4.0,
            margin: 8.0,
        }
    }

    #[must_use]
    pub const fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// The same, on `flow`'s side when both are beside their anchors: a
    /// submenu goes the way the menu it comes from went, so one that had
    /// to flip doesn't send the next back over the first.
    #[must_use]
    pub fn following(mut self, flow: Option<Side>) -> Self {
        if let (false, Some(side)) = (self.side.is_vertical(), flow) {
            self.side = side;
        }
        self
    }

    /// The side of `anchor` a panel at `panel` ended up on, when this
    /// placement is beside the anchor; `None` above and below it.
    pub fn side_of(self, anchor: Rectangle, panel: Rectangle) -> Option<Side> {
        if self.side.is_vertical() {
            None
        } else if panel.center_x() < anchor.center_x() {
            Some(Side::Left)
        } else {
            Some(Side::Right)
        }
    }
}

/// The room on `side` of `anchor` in a window of `window`.
fn room(side: Side, anchor: Rectangle, window: Size, margin: f32) -> f32 {
    match side {
        Side::Top => anchor.y - margin,
        Side::Bottom => window.height - (anchor.y + anchor.height) - margin,
        Side::Left => anchor.x - margin,
        Side::Right => window.width - (anchor.x + anchor.width) - margin,
    }
}

/// `value` kept between `low` and `high`; `low` wins when they cross (a
/// panel larger than the window starts at its margin).
fn within(value: f32, low: f32, high: f32) -> f32 {
    value.min(high).max(low)
}

/// The top-left corner of a panel of `size` anchored to `anchor` in a window
/// of `window`, and the side it ended up on.
pub fn place(anchor: Rectangle, size: Size, window: Size, placement: Placement) -> (Point, Side) {
    let Placement {
        side,
        align,
        gap,
        margin,
    } = placement;
    let needed = gap
        + if side.is_vertical() {
            size.height
        } else {
            size.width
        };
    // Flip only when it doesn't fit here and the other side has more room.
    let here = room(side, anchor, window, margin);
    let there = room(side.opposite(), anchor, window, margin);
    let side = if here < needed && there > here {
        side.opposite()
    } else {
        side
    };
    let along = |start: f32, length: f32, panel: f32| match align {
        Align::Near => start,
        Align::Center => start + (length - panel) / 2.0,
        Align::Far => start + length - panel,
    };
    let (x, y) = match side {
        Side::Top => (
            along(anchor.x, anchor.width, size.width),
            anchor.y - gap - size.height,
        ),
        Side::Bottom => (
            along(anchor.x, anchor.width, size.width),
            anchor.y + anchor.height + gap,
        ),
        Side::Left => (
            anchor.x - gap - size.width,
            along(anchor.y, anchor.height, size.height),
        ),
        Side::Right => (
            anchor.x + anchor.width + gap,
            along(anchor.y, anchor.height, size.height),
        ),
    };
    // Then slide it back inside the window, whichever way it sticks out.
    let point = Point::new(
        within(x, margin, window.width - margin - size.width),
        within(y, margin, window.height - margin - size.height),
    );
    (point, side)
}

#[cfg(test)]
mod tests {
    use super::*;

    const WINDOW: Size = Size::new(800.0, 600.0);
    const PANEL: Size = Size::new(200.0, 100.0);

    fn anchor(x: f32, y: f32) -> Rectangle {
        Rectangle::new(Point::new(x, y), Size::new(100.0, 32.0))
    }

    fn below() -> Placement {
        Placement::new(Side::Bottom, Align::Near)
    }

    #[test]
    fn a_panel_sits_where_asked_when_it_fits() {
        let (at, side) = place(anchor(100.0, 100.0), PANEL, WINDOW, below());
        assert_eq!((at, side), (Point::new(100.0, 136.0), Side::Bottom));
        let centred = Placement::new(Side::Top, Align::Center);
        let (at, side) = place(anchor(300.0, 300.0), PANEL, WINDOW, centred);
        assert_eq!((at, side), (Point::new(250.0, 196.0), Side::Top));
        let beside = Placement::new(Side::Right, Align::Near).gap(0.0);
        let (at, side) = place(anchor(100.0, 100.0), PANEL, WINDOW, beside);
        assert_eq!((at, side), (Point::new(200.0, 100.0), Side::Right));
        let far = Placement::new(Side::Bottom, Align::Far);
        let (at, _) = place(anchor(400.0, 100.0), PANEL, WINDOW, far);
        assert_eq!(at, Point::new(300.0, 136.0));
    }

    #[test]
    fn it_flips_to_the_side_with_room() {
        // Near the bottom edge: above instead.
        let (at, side) = place(anchor(100.0, 540.0), PANEL, WINDOW, below());
        assert_eq!((at, side), (Point::new(100.0, 436.0), Side::Top));
        // A submenu near the right edge opens to the left.
        let beside = Placement::new(Side::Right, Align::Near).gap(0.0);
        let (at, side) = place(anchor(650.0, 100.0), PANEL, WINDOW, beside);
        assert_eq!((at, side), (Point::new(450.0, 100.0), Side::Left));
    }

    #[test]
    fn it_stays_where_asked_when_neither_side_is_better() {
        let window = Size::new(800.0, 150.0);
        let (at, side) = place(anchor(100.0, 30.0), PANEL, window, below());
        assert_eq!(side, Side::Bottom);
        // And is slid back inside, to the margin.
        assert_eq!(at, Point::new(100.0, 42.0));
    }

    #[test]
    fn it_never_passes_an_edge_of_the_window() {
        let (at, _) = place(anchor(750.0, 100.0), PANEL, WINDOW, below());
        assert_eq!(at.x, 592.0);
        let far = Placement::new(Side::Bottom, Align::Far);
        let (at, _) = place(anchor(0.0, 100.0), PANEL, WINDOW, far);
        assert_eq!(at.x, 8.0);
        // Larger than the window: it starts at the margin.
        let huge = Size::new(1000.0, 100.0);
        let (at, _) = place(anchor(300.0, 100.0), huge, WINDOW, below());
        assert_eq!(at.x, 8.0);
    }

    #[test]
    fn a_submenu_goes_the_way_the_menu_it_comes_from_went() {
        let beside = Placement::new(Side::Left, Align::Near).gap(0.0);
        // The first submenu had no room on the left and went right…
        let first = anchor(20.0, 100.0);
        let (at, side) = place(first, PANEL, WINDOW, beside);
        assert_eq!(side, Side::Right);
        let panel = Rectangle::new(at, PANEL);
        assert_eq!(beside.side_of(first, panel), Some(Side::Right));
        // … so the next, which would fit on the left over the first menu,
        // goes right too.
        let next = Rectangle::new(Point::new(at.x, 150.0), Size::new(200.0, 32.0));
        let asked = beside.following(Some(Side::Right));
        let (at, side) = place(next, PANEL, WINDOW, asked);
        assert_eq!((at.x, side), (panel.x + panel.width, Side::Right));
        // A menu under its trigger takes no side from anyone.
        assert_eq!(below().following(Some(Side::Left)), below());
        assert_eq!(below().side_of(first, panel), None);
    }

    #[test]
    fn start_and_end_follow_the_direction() {
        assert_eq!(Align::start(Direction::Ltr), Align::Near);
        assert_eq!(Align::start(Direction::Rtl), Align::Far);
        assert_eq!(Align::end(Direction::Rtl), Align::Near);
        assert_eq!(Side::end(Direction::Ltr), Side::Right);
        assert_eq!(Side::end(Direction::Rtl), Side::Left);
    }
}

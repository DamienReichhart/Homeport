//! Homeport's mark (`Logo.tsx`): a square (this machine), a line (the
//! connection) and a hollow ring (a Host, with nothing installed on it).

use iced::widget::{container, svg, text, Row, Space};

use crate::ui::{
    theme::{self, font, metrics::space, Element},
    widgets::line,
};

/// The mark on its ink tile, in its own colours, for a welcome.
pub fn mark<'a, Message: 'a>(size: f32) -> Element<'a, Message> {
    let handle = svg::Handle::from_memory(include_bytes!("../../../../assets/mark.svg").as_slice());
    svg(handle).width(size).height(size).into()
}

/// The tile-less glyph beside the name, as at the top of the sidebar. The
/// glyph is drawn the same way round in every language, as a logo is.
pub fn logo<'a, Message: 'a>() -> Element<'a, Message> {
    let part = |width: f32, height: f32| container(Space::new()).width(width).height(height);
    let glyph = Row::with_children([
        part(11.8, 11.8).style(theme::container::mark_ink).into(),
        part(13.8, 4.2).style(theme::container::mark_ink).into(),
        part(16.0, 16.0).style(theme::container::mark_ring).into(),
    ])
    .align_y(iced::alignment::Vertical::Center);
    line(vec![
        glyph.into(),
        // The name is Latin in every language.
        text("Homeport").size(17).font(font::SANS_SEMIBOLD).into(),
    ])
    .spacing(space::S2_5)
    .into()
}

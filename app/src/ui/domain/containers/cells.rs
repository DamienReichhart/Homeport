//! What a container's row shows the same way in every list.

use homeport_engine::containers::Container;

use super::model::status_without_health;
use crate::ui::{
    theme::Element,
    widgets::{
        basic::{badge, truncated, BadgeVariant},
        stack,
    },
};

/// A container's status, whole: `Up 3 hours`, `Exited (0) 2 days ago`.
/// Its health is a badge on the line under it, not said twice and not
/// beside it, where the two would share a column neither fits in; with no
/// health the status has both lines to wrap onto. What still doesn't fit
/// ends with an ellipsis and is whole under the pointer.
pub fn status<'a, Message: Clone + 'a>(container: &Container) -> Element<'a, Message> {
    let status = truncated(status_without_health(container));
    let Some(health) = &container.health else {
        return status.lines(2).with_tip().into();
    };
    let variant = if health == "unhealthy" {
        BadgeVariant::Destructive
    } else {
        BadgeVariant::Outline
    };
    stack(vec![
        status.with_tip().into(),
        badge(health.clone(), variant),
    ])
    .spacing(2.0)
    .into()
}

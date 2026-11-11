//! Each feature's manifest, by what its pages are about.

pub mod app;
pub mod cluster;
pub mod docker;
pub mod engines;

use super::{Label, Nav, Scope};
use crate::ui::{shell::router::Route, theme::Icon};

/// A sidebar entry.
const fn nav(route: Route, label: Label, icon: Icon, scope: Scope) -> Option<Nav> {
    Some(Nav {
        route,
        label,
        icon,
        scope,
    })
}

//! Forms: a [`field`] is a control with its label, its help and why its
//! value is refused; the controls are iced's own where it has one (the
//! checkbox, the radio, the switch and the slider, which the theme styles)
//! and the kit's for the rest: [`input`] and its group, [`password`],
//! [`number_input`], [`textarea`], [`select`], [`tabs`], [`toggle_group`]
//! and [`segmented`].
//!
//! [`Errors`] sorts the failures a form gets back from a use case under
//! the fields they are about.

pub mod field;
pub mod input;
pub mod number;
pub mod select;
pub mod tabs;

pub use self::{
    field::{field, Errors, Field},
    input::{input, input_group, password, textarea, Addon},
    number::{number_input, Number},
    select::select,
    tabs::{segmented, tabs, toggle_group, TabsVariant, Toggle},
};

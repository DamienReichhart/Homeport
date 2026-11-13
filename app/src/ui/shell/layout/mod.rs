//! The window's layout: what a page puts at its top ([`page_header`]), the
//! bars over every page ([`banners`]) and the mark ([`logo`]). The header
//! and the mark are the kit's (`widgets::layout`), named here for the
//! pages that have always found them here.

pub mod banners;

pub use crate::ui::widgets::layout::{logo, page_header, page_header_of};

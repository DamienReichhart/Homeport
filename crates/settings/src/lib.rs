//! The user's preferences (theme, language, last Host, update checks, the desktop), kept as
//! one JSON document in the database.

mod model;
mod service;

pub use self::{
    model::{NotificationSettings, Selection, Settings, Theme, SYSTEM_LANGUAGE},
    service::SettingsService,
};

#[cfg(test)]
mod tests;

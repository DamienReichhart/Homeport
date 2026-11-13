//! What shows data that is neither a table nor a log: a folder's
//! listing ([`file_browser`]), a JSON document ([`json`]) and a list of
//! labelled values ([`key_value`]); `src/shared/ui/data` in React.

pub mod file_browser;
pub mod json;
pub mod key_value;

pub use self::key_value::{key_value, Item};

#[cfg(test)]
mod tests;

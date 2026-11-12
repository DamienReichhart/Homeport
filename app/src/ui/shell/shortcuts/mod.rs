//! Keyboard shortcuts: how keys are written and matched ([`keys`]), the
//! table of those that work now ([`keymap`]) and the sheet that lists them
//! ([`help`]), opened with `?`.

pub mod help;
pub mod keymap;
pub mod keys;
#[cfg(test)]
mod tests;

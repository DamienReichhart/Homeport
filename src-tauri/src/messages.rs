//! The shell's own error messages (ADR 0011), and the English error
//! catalogue gathered from every crate.

#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
use homeport_core::error::{arguments, Message};

homeport_core::messages! {}

/// Where `bun run bindings` writes the English error catalogue.
#[cfg(test)]
const CATALOGUE_PATH: &str = "../src/shared/i18n/en/errors.json";

/// Every crate's messages, by key; fails on a key declared twice with
/// different text, a key outside `errors.`, or an argument that isn't a
/// plain name (Rust renders `{name}` arguments only).
#[cfg(test)]
fn catalogue() -> Result<BTreeMap<&'static str, &'static str>, String> {
    let all: [&[Message]; 8] = [
        homeport_core::messages::ALL,
        homeport_storage::messages::ALL,
        homeport_transport::messages::ALL,
        homeport_engine::messages::ALL,
        homeport_hosts::messages::ALL,
        homeport_stacks::messages::ALL,
        homeport_registries::messages::ALL,
        ALL,
    ];
    let mut catalogue = BTreeMap::new();
    for m in all.into_iter().flatten() {
        if !m.key.starts_with("errors.") {
            return Err(format!("{} doesn't start with errors.", m.key));
        }
        for arg in arguments(m.en) {
            if arg.is_empty() || !arg.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return Err(format!("{}: {{{arg}}} isn't a plain argument", m.key));
            }
        }
        if let Some(previous) = catalogue.insert(m.key, m.en) {
            if previous != m.en {
                return Err(format!("{} is declared twice with different text", m.key));
            }
        }
    }
    Ok(catalogue)
}

/// The catalogue as the JSON file the frontend imports.
#[cfg(test)]
fn catalogue_json() -> String {
    let catalogue = catalogue().unwrap_or_else(|e| panic!("{e}"));
    format!(
        "{}\n",
        serde_json::to_string_pretty(&catalogue).expect("strings serialise")
    )
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn bindings_are_up_to_date_for_errors() {
        let committed = Path::new(env!("CARGO_MANIFEST_DIR")).join(super::CATALOGUE_PATH);
        let fresh = super::catalogue_json();
        if std::env::var_os("UPDATE_BINDINGS").is_some() {
            std::fs::write(&committed, fresh).unwrap();
            return;
        }
        assert_eq!(
            std::fs::read_to_string(&committed).unwrap_or_default(),
            fresh,
            "the error catalogue is stale: run `bun run bindings`"
        );
    }
}

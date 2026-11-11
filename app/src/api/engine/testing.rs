//! What the Engine use cases' tests share. Without an Engine to talk to,
//! most of them check that a use case refuses a Host or a Swarm Homeport
//! can't reach before it does anything.

use homeport_core::error::{AppResult, ErrorCode};

/// The id of a Host no test app has.
pub(crate) fn host() -> String {
    "no-such-host".to_owned()
}

/// The id of a Swarm no test app's Hosts belong to.
pub(crate) fn swarm() -> String {
    "no-such-swarm".to_owned()
}

/// The code a use case failed with; `None` when it succeeded.
pub(crate) fn code<T>(result: AppResult<T>) -> Option<ErrorCode> {
    result.err().map(|e| e.code)
}

/// Fails, naming them, if any use case of `results` didn't fail with `expected`.
pub(crate) fn assert_all(expected: ErrorCode, results: &[(&str, Option<ErrorCode>)]) {
    let wrong: Vec<_> = results
        .iter()
        .filter(|(_, code)| *code != Some(expected))
        .collect();
    assert!(wrong.is_empty(), "expected {expected:?} from {wrong:?}");
}

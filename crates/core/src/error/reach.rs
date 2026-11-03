//! What every layer needs to classify a failure to reach an Engine: the
//! `io::Error` behind it, an error a dialer already classified, and a
//! connection cut midway. The transports classify the rest themselves.

use std::{error::Error as StdError, io};

use super::{AppError, ErrorCode};

/// `error` as an `io::Error`, looking through the transparent wrappers of
/// bollard and russh (whose `source()` skips the io::Error itself).
pub fn as_io<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a io::Error> {
    if let Some(io) = error.downcast_ref::<io::Error>() {
        return Some(io);
    }
    #[cfg(feature = "bollard")]
    if let Some(bollard::errors::Error::IOError { err }) =
        error.downcast_ref::<bollard::errors::Error>()
    {
        return Some(err);
    }
    #[cfg(feature = "russh")]
    if let Some(russh::Error::IO(err)) = error.downcast_ref::<russh::Error>() {
        return Some(err);
    }
    None
}

/// Every `io::Error` in `error`'s chain, outermost first.
fn io_errors<'a>(error: &'a (dyn StdError + 'static)) -> impl Iterator<Item = &'a io::Error> {
    std::iter::successors(Some(error), |&e| e.source()).filter_map(as_io)
}

/// The first `io::Error` in `error`'s chain.
pub fn io_cause<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a io::Error> {
    io_errors(error).next()
}

/// An error a dialer already classified, carried inside an `io::Error`
/// through hyper and bollard.
pub fn classified_cause<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a AppError> {
    io_errors(error).find_map(|io| io.get_ref()?.downcast_ref::<AppError>())
}

/// The connection to `address` (a Host's label, or "the Docker Engine") broke.
pub fn connection_lost(address: &str) -> AppError {
    AppError::new(
        ErrorCode::ConnectionLost,
        format!("The connection to {address} was cut."),
    )
    .with_hint("The machine or the network may have restarted. Homeport keeps retrying.")
}

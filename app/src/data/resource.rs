//! What a page knows of something it reads: nothing yet, that it is on its
//! way, what it is, or why it couldn't be read, with what was read before.

use homeport_core::error::AppError;

/// A cached read, as a view draws it.
#[derive(Debug, Clone, PartialEq)]
pub enum Resource<T> {
    /// Nobody asked for it yet.
    Idle,
    /// Being read for the first time.
    Loading,
    Loaded(T),
    /// Shown as it was read last, while it is read again.
    Refreshing(T),
    /// The last read failed; what an earlier one gave is kept.
    Failed(AppError, Option<T>),
}

impl<T> Resource<T> {
    /// What was read last, however old.
    pub fn data(&self) -> Option<&T> {
        match self {
            Self::Loaded(data) | Self::Refreshing(data) => Some(data),
            Self::Failed(_, data) => data.as_ref(),
            Self::Idle | Self::Loading => None,
        }
    }

    /// Why the last read failed.
    pub fn error(&self) -> Option<&AppError> {
        match self {
            Self::Failed(error, _) => Some(error),
            _ => None,
        }
    }

    /// Whether it is being read right now, for the first time or again.
    pub fn is_fetching(&self) -> bool {
        matches!(self, Self::Loading | Self::Refreshing(_))
    }

    /// Whether there is nothing to show yet and nothing went wrong: the
    /// page draws its loading state.
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Idle | Self::Loading)
    }

    /// The same state about something made from the data.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Resource<U> {
        match self {
            Self::Idle => Resource::Idle,
            Self::Loading => Resource::Loading,
            Self::Loaded(data) => Resource::Loaded(f(data)),
            Self::Refreshing(data) => Resource::Refreshing(f(data)),
            Self::Failed(error, data) => Resource::Failed(error, data.map(f)),
        }
    }
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;

    #[test]
    fn a_failure_keeps_what_was_read_before() {
        let error = AppError::new(
            ErrorCode::HostNotConnected,
            homeport_hosts::messages::NOT_CONNECTED_UNKNOWN,
        );
        let failed = Resource::Failed(error.clone(), Some(3));
        assert_eq!(failed.data(), Some(&3));
        assert_eq!(failed.error(), Some(&error));
        assert!(!failed.is_fetching());
        assert!(!failed.is_pending());
        assert_eq!(Resource::<u8>::Failed(error, None).data(), None);
    }

    #[test]
    fn only_a_first_read_is_pending() {
        assert!(Resource::<u8>::Idle.is_pending());
        assert!(Resource::<u8>::Loading.is_pending());
        assert!(Resource::<u8>::Loading.is_fetching());
        assert!(!Resource::Refreshing(1).is_pending());
        assert!(Resource::Refreshing(1).is_fetching());
        assert_eq!(Resource::Refreshing(1).data(), Some(&1));
        assert_eq!(Resource::Loaded(2).map(|n| n * 2), Resource::Loaded(4));
    }
}

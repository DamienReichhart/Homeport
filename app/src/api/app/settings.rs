//! The user's preferences.

use std::collections::BTreeMap;

use homeport_core::error::AppResult;
use homeport_settings::Settings;

use crate::app::{events::Event, AppState};

pub fn get_settings(state: &AppState) -> AppResult<Settings> {
    state.settings.get()
}

pub fn update_settings(state: &AppState, settings: Settings) -> AppResult<Settings> {
    let previous = state.settings.get()?;
    let updated = state.settings.update(settings)?;
    if updated.keep_events_days < previous.keep_events_days {
        super::history::prune(state);
    }
    Ok(updated)
}

/// Hands over the active language's templates (`native.*` and `errors.*`,
/// by key) for the text the OS draws: the tray menu and notifications.
/// An empty map means English (ADR 0011).
pub fn set_native_text(state: &AppState, templates: BTreeMap<String, String>) {
    homeport_core::error::set_translations(templates);
    state.events.send(Event::NativeTextChanged);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use homeport_core::error::localize;
    use homeport_settings::Settings;

    use crate::{
        api::app::history::testing::{event_of_days_ago, host},
        app::{events::Event, testing},
    };

    #[tokio::test]
    async fn settings_are_read_back_as_they_were_saved() {
        let app = testing::app();
        let defaults = super::get_settings(&app.state).unwrap();
        let wanted = Settings {
            check_for_updates: !defaults.check_for_updates,
            // Out of range: saved as the longest retention allowed.
            keep_events_days: 4000,
            ..defaults
        };

        let updated = super::update_settings(&app.state, wanted.clone()).unwrap();

        assert_eq!(updated.check_for_updates, wanted.check_for_updates);
        assert_eq!(updated.keep_events_days, 365);
        let read = super::get_settings(&app.state).unwrap();
        assert_eq!(read.check_for_updates, wanted.check_for_updates);
        assert_eq!(read.keep_events_days, 365);
    }

    #[tokio::test]
    async fn a_shorter_retention_prunes_the_history_at_once() {
        let app = testing::app();
        let events = &app.state.hosts.events;
        let host = host(&app.state);
        events.record(&host, &event_of_days_ago(5));
        events.flush().await;
        let settings = |days| Settings {
            keep_events_days: days,
            ..super::get_settings(&app.state).unwrap()
        };

        // A longer one leaves it alone.
        super::update_settings(&app.state, settings(60)).unwrap();
        assert_eq!(events.page(&host, 10, None).unwrap().len(), 1);

        super::update_settings(&app.state, settings(2)).unwrap();
        assert!(events.page(&host, 10, None).unwrap().is_empty());
    }

    #[tokio::test]
    async fn native_text_is_replaced_and_the_ui_told() {
        let mut app = testing::app();
        let key = "native.test.onlyInThisTest";
        let templates = BTreeMap::from([(key.to_owned(), "Ouvrir {name}".to_owned())]);

        super::set_native_text(&app.state, templates);

        assert!(localize(key, &Default::default()).is_some());
        assert!(matches!(
            app.events.try_recv(),
            Ok(Event::NativeTextChanged)
        ));

        // An empty map is English again.
        super::set_native_text(&app.state, BTreeMap::new());
        assert!(localize(key, &Default::default()).is_none());
        assert!(matches!(
            app.events.try_recv(),
            Ok(Event::NativeTextChanged)
        ));
    }
}

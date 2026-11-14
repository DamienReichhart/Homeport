//! The fields with suggestions (React's `<datalist>`): what each offers
//! and what its list does.

use super::{Caps, Form, Hint, Line, Lists, Message};
use crate::ui::{shell::Context, widgets::form::suggest};

/// The log drivers most Hosts have (`LoggingField`'s `DRIVERS`); any
/// other name can be typed.
pub const DRIVERS: [&str; 5] = ["json-file", "local", "journald", "syslog", "none"];

/// The lists of the fields with suggestions.
#[derive(Debug, Default)]
pub(super) struct Hints {
    pub(super) image: suggest::State,
    pub(super) cap_add: suggest::State,
    pub(super) cap_drop: suggest::State,
    pub(super) driver: suggest::State,
}

impl Form {
    /// What a field with suggestions offers (React's `<datalist>`): the
    /// Host's image tags, the capabilities not chosen, the usual drivers.
    pub fn suggestions(&self, hint: Hint, lists: &Lists) -> Vec<String> {
        match hint {
            Hint::Image => lists
                .images
                .data()
                .map(|images| images.iter().flat_map(|i| i.tags.iter().cloned()).collect())
                .unwrap_or_default(),
            Hint::Capability(caps) => {
                let security = &self.form.advanced.security;
                let chosen = match caps {
                    Caps::Add => &security.cap_add,
                    Caps::Drop => &security.cap_drop,
                };
                homeport_engine::containers::run::CAPABILITIES
                    .iter()
                    .filter(|name| !chosen.iter().any(|c| c == *name))
                    .map(|name| (*name).to_owned())
                    .collect()
            }
            Hint::LogDriver => DRIVERS.map(str::to_owned).to_vec(),
        }
    }

    /// The list of a field with suggestions.
    pub fn hint(&self, hint: Hint) -> &suggest::State {
        let h = &self.hints;
        match hint {
            Hint::Image => &h.image,
            Hint::Capability(Caps::Add) => &h.cap_add,
            Hint::Capability(Caps::Drop) => &h.cap_drop,
            Hint::LogDriver => &h.driver,
        }
    }

    /// The field a hint is of, and what it holds.
    fn hinted(&self, hint: Hint) -> (Line, String) {
        match hint {
            Hint::Image => (Line::Image, self.form.image.clone()),
            Hint::Capability(caps) => (Line::Capability(caps), self.drafts[caps as usize].clone()),
            Hint::LogDriver => (Line::LogDriver, self.form.advanced.logging.driver.clone()),
        }
    }

    pub(super) fn suggested(
        &mut self,
        hint: Hint,
        message: suggest::Message,
        ctx: &Context<'_>,
    ) -> (iced::Task<Message>, bool) {
        let options = self.suggestions(hint, &self.lists(ctx));
        let (line, value) = self.hinted(hint);
        let h = &mut self.hints;
        let state = match hint {
            Hint::Image => &mut h.image,
            Hint::Capability(Caps::Add) => &mut h.cap_add,
            Hint::Capability(Caps::Drop) => &mut h.cap_drop,
            Hint::LogDriver => &mut h.driver,
        };
        let (task, event) = state.update(message, &value, &options);
        match event {
            Some(suggest::Event::Input(text)) => self.typed(line, text),
            // Enter adds the capability typed (`TokenInput`); elsewhere it
            // sends the form.
            Some(suggest::Event::Submit) => match hint {
                Hint::Capability(caps) => self.add_capability(caps),
                Hint::Image | Hint::LogDriver => return (task, true),
            },
            None => {}
        }
        (task, false)
    }
}

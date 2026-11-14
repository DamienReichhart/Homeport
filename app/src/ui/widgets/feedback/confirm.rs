//! `ConfirmDialog` and `useConfirmedAction`: a step before an action that
//! can't be undone. The page says what to [`Ask`], keeps a [`State`] while
//! the dialog is open, and acts when [`State::update`] says
//! [`Outcome::Confirmed`]:
//!
//! - while the action runs the dialog is busy: its buttons and Escape do
//!   nothing;
//! - when the action fails, the page calls [`State::failed`]: the dialog
//!   stays open for another try, and reporting the failure is the action's
//!   job (a toast);
//! - when what is destroyed can't come back, the page asks for its name to
//!   be typed ([`Ask::typed`]): the action stays disabled until it is.

use iced::widget::{text, text_input};

use crate::{
    t,
    ui::{
        theme::{
            self, font,
            metrics::{control, space, text as size},
            Element,
        },
        widgets::{
            basic::button::{self, Variant},
            focus::pressable,
            overlay::{dialog, modal, Modal},
            paragraph, stack,
        },
    },
};

/// What to ask before an action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ask {
    pub title: String,
    /// What will happen, naming exactly what is lost for a destructive
    /// action.
    pub description: String,
    pub confirm_label: String,
    /// Red emphasis, for an action that destroys data.
    pub destructive: bool,
    /// The name to type before the action is offered.
    pub typed: Option<String>,
}

impl Ask {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        confirm_label: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            confirm_label: confirm_label.into(),
            destructive: false,
            typed: None,
        }
    }

    #[must_use]
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    /// The user types `name` first: for what can't be brought back.
    #[must_use]
    pub fn typed(mut self, name: impl Into<String>) -> Self {
        self.typed = Some(name.into());
        self
    }
}

/// The open dialog: what was typed, and whether the action is running.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    input: String,
    busy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Input(String),
    Cancel,
    Confirm,
}

/// What the page does after a message of the dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Nothing yet.
    Pending,
    /// Drop the dialog.
    Cancelled,
    /// Run the action; the dialog is busy until it ends.
    Confirmed,
}

impl State {
    pub fn is_busy(&self) -> bool {
        self.busy
    }

    /// Whether the action may run: nothing to type, or the name is typed.
    pub fn allowed(&self, ask: &Ask) -> bool {
        !self.busy
            && ask
                .typed
                .as_deref()
                .is_none_or(|name| self.input.trim() == name)
    }

    pub fn update(&mut self, message: Message, ask: &Ask) -> Outcome {
        match message {
            Message::Input(input) if !self.busy => self.input = input,
            Message::Cancel if !self.busy => return Outcome::Cancelled,
            Message::Confirm if self.allowed(ask) => {
                self.busy = true;
                return Outcome::Confirmed;
            }
            _ => {}
        }
        Outcome::Pending
    }

    /// The action failed: the dialog offers it again.
    pub fn failed(&mut self) {
        self.busy = false;
    }
}

/// What names a confirmation's Cancel.
fn cancel_id() -> iced::advanced::widget::Id {
    iced::advanced::widget::Id::new("confirm-cancel")
}

/// The dialog for `ask`.
pub fn confirm_dialog<'a>(ask: &Ask, state: &State) -> Modal<'a, Message> {
    confirm_dialog_with(ask, state, None, |message| message)
}

/// The dialog for `ask` with `more` under what it says: what the action
/// will cost, read as the dialog opened, with its own buttons, whose
/// messages are the page's (`ConfirmDialog`'s `description` as an
/// element). The dialog's own messages become the page's by `wrap`. An
/// empty description is left out.
pub fn confirm_dialog_with<'a, Page: Clone + 'a>(
    ask: &Ask,
    state: &State,
    more: Option<Element<'a, Page>>,
    wrap: impl Fn(Message) -> Page + 'a,
) -> Modal<'a, Page> {
    let wrap = std::rc::Rc::new(wrap);
    let mut content = dialog(ask.title.clone()).destructive(ask.destructive);
    if !ask.description.is_empty() {
        content = content.description(ask.description.clone());
    }
    if let Some(more) = more {
        content = content.push(more);
    }
    if let Some(name) = &ask.typed {
        let field = text_input(name, &state.input)
            .size(size::SM)
            .padding([space::S1, control::PADDING_X])
            .on_input_maybe((!state.busy).then(|| {
                let wrap = wrap.clone();
                move |input| wrap(Message::Input(input))
            }))
            .on_submit(wrap(Message::Confirm));
        content = content.push(
            stack(vec![
                paragraph(t!("ui.confirm.typeName", name = name.as_str()))
                    .font(font::medium())
                    .into(),
                field.into(),
            ])
            .spacing(space::S2),
        );
    }
    let cancel = button::text(t!("common.cancel"))
        .variant(Variant::Outline)
        .id(cancel_id())
        .on_press_maybe((!state.busy).then(|| wrap(Message::Cancel)));
    let confirm = (state.allowed(ask)).then(|| wrap(Message::Confirm));
    let action: Element<'a, Page> = if ask.destructive {
        // The one red slab of the app: the button that destroys.
        pressable(
            text(ask.confirm_label.clone())
                .size(size::SM)
                .font(font::medium())
                .height(iced::Length::Fill)
                .align_y(iced::alignment::Vertical::Center),
        )
        .height(control::HEIGHT)
        .padding([0.0, control::PADDING_X])
        .on_press_maybe(confirm)
        .style(theme::button::destructive_solid)
        .into()
    } else {
        button::text(ask.confirm_label.clone())
            .on_press_maybe(confirm)
            .into()
    };
    // Cancel takes focus as it opens, whatever stands before it (Radix's
    // `AlertDialog`): Enter must never confirm what can't be undone.
    modal(content.footer(vec![cancel.into(), action]))
        .focus_first(cancel_id())
        .key("confirm")
        .on_close_maybe((!state.busy).then(|| wrap(Message::Cancel)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn remove() -> Ask {
        Ask::new("Remove it?", "It is gone.", "Remove").destructive()
    }

    #[test]
    fn confirming_runs_the_action_once_and_a_failure_offers_it_again() {
        let (ask, mut state) = (remove(), State::default());
        assert_eq!(state.update(Message::Confirm, &ask), Outcome::Confirmed);
        // Busy: a second press, Cancel and Escape do nothing.
        assert!(state.is_busy());
        assert_eq!(state.update(Message::Confirm, &ask), Outcome::Pending);
        assert_eq!(state.update(Message::Cancel, &ask), Outcome::Pending);
        state.failed();
        assert_eq!(state.update(Message::Confirm, &ask), Outcome::Confirmed);
    }

    #[test]
    fn cancel_closes_without_acting() {
        let mut state = State::default();
        assert_eq!(state.update(Message::Cancel, &remove()), Outcome::Cancelled);
        assert!(!state.is_busy());
    }

    #[test]
    fn a_typed_confirmation_needs_the_exact_name() {
        let ask = remove().typed("prod-db");
        let mut state = State::default();
        assert_eq!(state.update(Message::Confirm, &ask), Outcome::Pending);
        for wrong in ["prod", "Prod-DB", "prod-db-2"] {
            state.update(Message::Input(wrong.to_owned()), &ask);
            assert!(!state.allowed(&ask), "{wrong}");
            assert_eq!(state.update(Message::Confirm, &ask), Outcome::Pending);
        }
        // A space pasted around the name is not a different name.
        state.update(Message::Input(" prod-db ".to_owned()), &ask);
        assert_eq!(state.update(Message::Confirm, &ask), Outcome::Confirmed);
    }
}

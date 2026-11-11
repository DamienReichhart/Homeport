//! `Field`: a control with its label, the help under it and, when its
//! value is refused, why. And [`Errors`], which sorts the failures a form
//! gets back under the fields they are about.

use homeport_core::error::AppError;
use iced::widget::text;

use crate::{
    i18n, t,
    ui::{
        theme::{
            self, font,
            metrics::{space, text as size},
            Element,
        },
        widgets::{line, paragraph, stack},
    },
};

/// A labelled control, being described.
pub struct Field<'a, Message> {
    label: String,
    optional: bool,
    control: Element<'a, Message>,
    help: Option<String>,
    error: Option<String>,
}

/// `control` under `label`.
pub fn field<'a, Message>(
    label: impl Into<String>,
    control: impl Into<Element<'a, Message>>,
) -> Field<'a, Message> {
    Field {
        label: label.into(),
        optional: false,
        control: control.into(),
        help: None,
        error: None,
    }
}

impl<'a, Message: 'a> Field<'a, Message> {
    /// "Optional" beside the label.
    #[must_use]
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// `FieldDescription`: what the value is for, under the control.
    #[must_use]
    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// `FieldError`: why the value is refused. It takes the help's place,
    /// in red; give the control its invalid style too.
    #[must_use]
    pub fn error(mut self, error: Option<String>) -> Self {
        self.error = error;
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let mut label: Vec<Element<'a, Message>> =
            vec![text(self.label).size(size::SM).font(font::medium()).into()];
        if self.optional {
            label.push(
                text(t!("common.optional"))
                    .size(size::XS)
                    .style(theme::text::muted)
                    .into(),
            );
        }
        let mut parts = vec![line(label).spacing(space::S2).into(), self.control];
        match (self.error, self.help) {
            (Some(error), _) => parts.push(paragraph(error).style(theme::text::destructive).into()),
            (None, Some(help)) => parts.push(paragraph(help).style(theme::text::muted).into()),
            (None, None) => {}
        }
        stack(parts).spacing(space::S1_5).into()
    }
}

impl<'a, Message: 'a> From<Field<'a, Message>> for Element<'a, Message> {
    fn from(field: Field<'a, Message>) -> Self {
        field.view()
    }
}

/// The failures of a form, by field. The backend refuses a value with a
/// keyed `AppError` (`errors.build.tag`); the form says which key is about
/// which field, and anything else stays the form's own failure, shown with
/// `error_message`.
///
/// It keeps the errors, not their text: [`Errors::message`] says them in
/// the language of the moment.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Errors {
    by_field: Vec<(&'static str, AppError)>,
}

impl Errors {
    pub fn is_empty(&self) -> bool {
        self.by_field.is_empty()
    }

    /// Puts `error` under `field`, in place of what was there.
    pub fn set(&mut self, field: &'static str, error: AppError) {
        self.clear(field);
        self.by_field.push((field, error));
    }

    /// The field was edited: what was wrong with it may not be any more.
    pub fn clear(&mut self, field: &str) {
        self.by_field.retain(|(name, _)| *name != field);
    }

    pub fn get(&self, field: &str) -> Option<&AppError> {
        self.by_field
            .iter()
            .find(|(name, _)| *name == field)
            .map(|(_, error)| error)
    }

    /// Why `field` is refused, in the app's language: for [`Field::error`].
    pub fn message(&self, field: &str) -> Option<String> {
        self.get(field).map(i18n::errors::message)
    }

    pub fn has(&self, field: &str) -> bool {
        self.get(field).is_some()
    }

    /// Sorts a failure the backend returned: under the field its key is
    /// about, per `fields` (key, field), or back to the caller when it is
    /// about none, for the form to show whole.
    pub fn route(
        &mut self,
        error: AppError,
        fields: &[(&str, &'static str)],
    ) -> Result<(), AppError> {
        match fields.iter().find(|(key, _)| *key == error.key) {
            Some((_, field)) => {
                self.set(field, error);
                Ok(())
            }
            None => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use homeport_core::{error::ErrorCode, messages as msg};

    use super::*;

    #[test]
    fn a_keyed_failure_goes_under_its_field_and_the_rest_comes_back() {
        let fields = [(msg::IO_FAILED.key, "path")];
        let mut errors = Errors::default();
        let refused = AppError::validation(msg::IO_FAILED);
        assert_eq!(errors.route(refused.clone(), &fields), Ok(()));
        assert_eq!(errors.get("path"), Some(&refused));
        assert!(errors.has("path") && !errors.has("name"));

        let other = AppError::new(ErrorCode::Json, msg::UNREADABLE_DATA);
        assert_eq!(errors.route(other.clone(), &fields), Err(other));

        // Editing the field takes its error away.
        errors.clear("path");
        assert!(errors.is_empty());
    }

    #[test]
    fn the_message_is_said_in_the_language_of_the_moment() {
        let mut errors = Errors::default();
        errors.set("path", AppError::validation(msg::IO_FAILED));
        let english = {
            let _english = crate::test_support::snapshot::in_language("en");
            errors.message("path").unwrap()
        };
        let _french = crate::test_support::snapshot::in_language("fr");
        let french = errors.message("path").unwrap();
        assert_ne!(english, french);
        assert_eq!(errors.message("name"), None);
    }
}

//! A field for a number: it takes digits only, the Up and Down arrows (and
//! the two small buttons at its end) step it, and it stays between its
//! bounds. The page keeps the text as typed, so "1." and "-" can be on the
//! way to a number, and reads it with [`Number::value`].

use iced::{
    keyboard::{key::Named, Key},
    widget::{container, Column},
    Length,
};

use super::input::{input, input_group, Addon};
use crate::ui::{
    theme::{self, icon, metrics::control, Element, Icon},
    widgets::focus::{key_area, pressable},
};

/// What a number field accepts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Number {
    pub min: f64,
    pub max: f64,
    pub step: f64,
    /// Whole numbers only: a port, a count of replicas.
    pub integer: bool,
}

impl Number {
    /// A whole number from `min` to `max`, stepped by one.
    pub const fn integer(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
            step: 1.0,
            integer: true,
        }
    }

    /// A number with decimals from `min` to `max`, stepped by `step`.
    pub const fn decimal(min: f64, max: f64, step: f64) -> Self {
        Self {
            min,
            max,
            step,
            integer: false,
        }
    }

    /// Whether `text` may be typed: a number, or the start of one.
    pub fn accepts(&self, text: &str) -> bool {
        let digits = text.strip_prefix('-').unwrap_or(text);
        if text.starts_with('-') && self.min >= 0.0 {
            return false;
        }
        let mut points = 0;
        digits.chars().all(|c| match c {
            '0'..='9' => true,
            '.' if !self.integer => {
                points += 1;
                points == 1
            }
            _ => false,
        })
    }

    /// The number `text` stands for, if it is one and within the bounds.
    pub fn value(&self, text: &str) -> Option<f64> {
        let value: f64 = text.parse().ok()?;
        (value.is_finite() && value >= self.min && value <= self.max).then_some(value)
    }

    /// `text` moved by `steps` steps, kept within the bounds; what isn't a
    /// number yet starts from the least it may be (or zero).
    pub fn stepped(&self, text: &str, steps: f64) -> String {
        let from = text
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
            .unwrap_or_else(|| self.min.max(0.0).min(self.max) - self.step * steps.signum());
        let value = (from + self.step * steps).clamp(self.min, self.max);
        if self.integer {
            format!("{}", value.round())
        } else {
            // As many decimals as the step has, so 0.1 + 0.2 reads 0.3.
            let decimals = format!("{}", self.step)
                .split_once('.')
                .map_or(0, |(_, fraction)| fraction.len());
            format!("{value:.decimals$}")
        }
    }
}

/// The field, for `text` as typed so far. `on_input` gets what it becomes:
/// what was typed when it may be, or the stepped number.
pub fn number_input<'a, Message: Clone + 'a>(
    placeholder: &str,
    text: &str,
    number: Number,
    on_input: impl Fn(String) -> Message + Clone + 'a,
) -> Element<'a, Message> {
    let typed = on_input.clone();
    let kept = text.to_owned();
    let field = input(placeholder, text)
        .on_input(move |next| {
            // What may not be typed leaves the field as it was.
            typed(if number.accepts(&next) {
                next
            } else {
                kept.clone()
            })
        })
        .style(if text.is_empty() || number.value(text).is_some() {
            theme::text_input::field_style
        } else {
            theme::text_input::invalid
        });
    let step = |glyph: Icon, steps: f64| {
        pressable(
            container(icon(glyph).width(12).height(12).style(theme::svg::muted))
                .center(Length::Fill),
        )
        .width(16)
        .height(control::HEIGHT / 2.0 - 3.0)
        .padding(0)
        .on_press(on_input(number.stepped(text, steps)))
        .style(theme::button::small(theme::button::ghost))
    };
    let steppers = Column::new()
        .push(step(Icon::ChevronUp, 1.0))
        .push(step(Icon::ChevronDown, -1.0));
    let field = input_group(field, None, Some(Addon::new(steppers, 8.0)));
    let (up, down) = (
        on_input(number.stepped(text, 1.0)),
        on_input(number.stepped(text, -1.0)),
    );
    key_area(field, move |key, _| match key {
        Key::Named(Named::ArrowUp) => Some(up.clone()),
        Key::Named(Named::ArrowDown) => Some(down.clone()),
        _ => None,
    })
    .into()
}

#[cfg(test)]
mod tests {
    use super::Number;

    const PORT: Number = Number::integer(1.0, 65535.0);
    const CPUS: Number = Number::decimal(0.0, 8.0, 0.1);
    const OFFSET: Number = Number::integer(-10.0, 10.0);

    #[test]
    fn it_takes_digits_and_what_is_on_the_way_to_a_number() {
        for typed in ["", "8", "8080", "65535999"] {
            assert!(PORT.accepts(typed), "{typed}");
        }
        for typed in ["8a", "8.0", "-1", " 8", "1e3"] {
            assert!(!PORT.accepts(typed), "{typed}");
        }
        for typed in ["", "1", "1.", "1.5", ".5"] {
            assert!(CPUS.accepts(typed), "{typed}");
        }
        assert!(!CPUS.accepts("1.5.2"));
        assert!(OFFSET.accepts("-") && OFFSET.accepts("-3"));
        assert!(!OFFSET.accepts("3-"));
    }

    #[test]
    fn its_value_is_a_number_within_the_bounds() {
        assert_eq!(PORT.value("8080"), Some(8080.0));
        assert_eq!(PORT.value("0"), None);
        assert_eq!(PORT.value("70000"), None);
        assert_eq!(PORT.value(""), None);
        assert_eq!(CPUS.value("1."), Some(1.0));
        assert_eq!(OFFSET.value("-"), None);
    }

    #[test]
    fn stepping_moves_by_the_step_and_stops_at_the_bounds() {
        assert_eq!(PORT.stepped("8080", 1.0), "8081");
        assert_eq!(PORT.stepped("8080", -1.0), "8079");
        assert_eq!(PORT.stepped("65535", 1.0), "65535");
        assert_eq!(PORT.stepped("1", -1.0), "1");
        // Nothing typed: the first step lands on the least it may be.
        assert_eq!(PORT.stepped("", 1.0), "1");
        assert_eq!(OFFSET.stepped("", 1.0), "0");
        assert_eq!(CPUS.stepped("0.1", 2.0), "0.3");
        assert_eq!(CPUS.stepped("7.95", 1.0), "8.0");
        assert_eq!(OFFSET.stepped("-10", -1.0), "-10");
    }
}

//! The Health tab: the healthcheck beside its last results.

use std::sync::Arc;

use iced::{widget::container, Length};

use homeport_engine::containers::health::{Health, HealthResult};

use crate::{
    data::Resource,
    i18n::format,
    t,
    ui::{
        features::docker::containers::detail::{
            model::{check_duration, health_timing, inspect::unix, zone, OUTPUT_LINES},
            tabs::health::{Message, Tab},
        },
        theme::{
            self, font,
            metrics::{space, text as size, ButtonSize},
            Element,
        },
        widgets::{
            basic::{
                badge,
                button::{self, Variant},
                card, flow, in_reading_order, separator, technical, BadgeVariant,
            },
            data::{key_value, Item},
            feedback::{error_message, spinner},
            line, paragraph, stack,
        },
    },
};

/// `text` as a program wrote it, a line a line, each laid out left to
/// right: an isolate ends where its line does.
fn written(text: &str) -> String {
    text.lines().map(technical).collect::<Vec<_>>().join("\n")
}

/// A run's output, folded past [`OUTPUT_LINES`] lines (`Output`).
fn output<'a>(tab: &Tab, result: &HealthResult) -> Element<'a, Message> {
    let trimmed = result.output.trim_end();
    if trimmed.trim().is_empty() {
        return paragraph(t!("containers.health.noOutput"))
            .size(size::XS)
            .style(theme::text::muted)
            .into();
    }
    let lines: Vec<&str> = trimmed.split('\n').collect();
    let long = lines.len() > OUTPUT_LINES;
    let open = tab.open.contains(&result.start);
    let shown = if long && !open {
        lines[..OUTPUT_LINES].join("\n")
    } else {
        trimmed.to_owned()
    };
    let pre = container(
        iced::widget::text(written(&shown))
            .font(font::MONO)
            .size(size::XS)
            .wrapping(iced::widget::text::Wrapping::WordOrGlyph),
    )
    .padding(space::S2)
    .width(Length::Fill)
    .style(theme::container::muted);
    let mut parts: Vec<Element<'a, Message>> = vec![pre.into()];
    if long {
        let label = if open {
            t!("containers.health.showLess")
        } else {
            t!(
                "containers.health.showMore",
                count = lines.len() - OUTPUT_LINES
            )
        };
        parts.push(
            button::text(label)
                .variant(Variant::Ghost)
                .size(ButtonSize::Sm)
                .on_press(Message::Fold(result.start.clone()))
                .into(),
        );
    }
    stack(parts).spacing(space::S1).into()
}

/// One run: how it ended, when, how long it took, what it said.
fn result<'a>(tab: &Tab, result: &HealthResult) -> Element<'a, Message> {
    let variant = if result.exit_code == 0 {
        BadgeVariant::Passed
    } else {
        BadgeVariant::Failed
    };
    let mut facts: Vec<Element<'a, Message>> = vec![badge(
        t!("containers.health.exit", code = result.exit_code),
        variant,
    )];
    if let Some(at) = unix(&result.start) {
        facts.push(
            iced::widget::text(format::date_time_in(at, &zone()))
                .size(size::SM)
                .into(),
        );
    }
    if let Some(took) = check_duration(&result.start, &result.end) {
        facts.push(
            iced::widget::text(in_reading_order(&t!(
                "containers.health.took",
                duration = took
            )))
            .size(size::SM)
            .style(theme::text::muted)
            .into(),
        );
    }
    stack(vec![
        flow(facts).spacing(space::S2).into(),
        output(tab, result),
    ])
    .spacing(space::S1_5)
    .into()
}

fn status<'a>(health: &Health) -> Element<'a, Message> {
    let variant = match health.status.as_str() {
        "unhealthy" => BadgeVariant::Destructive,
        "healthy" => BadgeVariant::Passed,
        _ => BadgeVariant::Outline,
    };
    badge(technical(&health.status), variant)
}

fn check<'a>(health: &Health) -> Element<'a, Message> {
    let items = vec![
        Item::mono(
            t!("containers.health.command"),
            health.config.command.clone(),
        ),
        Item::text(
            t!("containers.health.timingLabel"),
            health_timing(&health.config),
        ),
        Item::text(
            t!("containers.health.failingStreak"),
            health.failing_streak.to_string(),
        ),
    ];
    let title = line(vec![
        iced::widget::text(t!("containers.health.title"))
            .size(size::BASE)
            .font(font::medium())
            .into(),
        status(health),
    ])
    .spacing(space::S2);
    card(stack(vec![title.into(), key_value(items)]).spacing(space::S4)).into()
}

fn results<'a>(tab: &Tab, health: &Health) -> Element<'a, Message> {
    let body: Element<'a, Message> = if health.log.is_empty() {
        paragraph(t!("containers.health.noRuns"))
            .style(theme::text::muted)
            .into()
    } else {
        let mut runs = vec![];
        for (at, run) in health.log.iter().enumerate() {
            if at > 0 {
                runs.push(separator());
            }
            runs.push(result(tab, run));
        }
        stack(runs).spacing(space::S3).into()
    };
    card(body).title(t!("containers.health.lastResults")).into()
}

pub fn view<'a>(tab: &'a Tab, read: &Resource<Arc<Option<Health>>>) -> Element<'a, Message> {
    // `isPending ? <Spinner />`, then the failure, then what was read.
    if read.is_pending() {
        return spinner();
    }
    if let Some(error) = read.error() {
        return error_message(error, true, Message::Failure);
    }
    let Some(health) = read.data().and_then(|health| health.as_ref().as_ref()) else {
        return paragraph(t!("containers.health.none"))
            .style(theme::text::muted)
            .into();
    };
    // The healthcheck over its results, each as wide as the page: a
    // command reads on a line or two, where a third of the width (React's
    // `lg` grid) cut it over four.
    stack(vec![check(health), results(tab, health)])
        .spacing(space::S4)
        .into()
}

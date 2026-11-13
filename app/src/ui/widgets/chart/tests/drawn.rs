//! The chart drawn: in both themes and under Arabic chrome, with the
//! pointer on it, and on what a stream that misbehaves leaves.

use iced::{widget::container, Length, Point};

use crate::{
    test_support::snapshot::{assert_snapshot, assert_snapshot_after},
    ui::{
        theme::{self, metrics::space, Element, Theme},
        widgets::{
            basic::card,
            chart::{Message, State},
            gallery::{
                self,
                chart::{self as sample, HOUR, WINDOW},
                Gallery, Page,
            },
            stack,
        },
    },
};

type Sent = gallery::Message;

fn on_page(part: Element<'_, Sent>) -> Element<'_, Sent> {
    container(part)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::page)
        .into()
}

fn in_card(part: Element<'_, Sent>) -> Element<'_, Sent> {
    on_page(card(part).view().into())
}

fn charts() -> Gallery {
    Gallery {
        page: Page::Chart,
        ..Gallery::default()
    }
}

/// The gallery's page: one series, an area, two counters with a
/// restart, gaps, and ten series of an hour.
#[test]
fn the_charts_in_both_themes_and_mirrored() {
    let gallery = charts();
    for (name, language, theme) in [
        ("chart-gallery-en", "en", Theme::Light),
        ("chart-gallery-dark-en", "en", Theme::Dark),
        ("chart-gallery-ar", "ar", Theme::Light),
    ] {
        assert_snapshot(name, language, &theme, (1100.0, 960.0), || gallery.view());
    }
}

/// The pointer on the ten series: the crosshair on the nearest sample,
/// a mark on each line, and the tooltip beside it, in two columns since
/// eleven rows are taller than the chart.
#[test]
fn the_pointer_shows_a_crosshair_and_what_each_series_was() {
    let state = sample::many(HOUR);
    for (name, language, theme, at) in [
        ("chart-hover-en", "en", Theme::Light, 300.0),
        ("chart-hover-dark-en", "en", Theme::Dark, 760.0),
        ("chart-hover-ar", "ar", Theme::Light, 300.0),
    ] {
        assert_snapshot_after(
            name,
            language,
            &theme,
            (900.0, 360.0),
            || in_card(sample::many_view(&state)),
            |ui| {
                ui.point_at(Point::new(at, 150.0));
            },
        );
    }
}

/// Nothing pushed yet, one sample alone, and a series hidden from the
/// legend: the other's range is its own again.
#[test]
fn an_empty_chart_one_sample_and_a_hidden_series() {
    let empty = State::new(1, WINDOW);
    let one = sample::memory(1);
    let mut hidden = sample::network(WINDOW);
    hidden.update(Message::Toggle(0));
    assert_snapshot(
        "chart-edges-en",
        "en",
        &Theme::Light,
        (700.0, 900.0),
        || {
            on_page(
                stack(vec![
                    card(sample::empty_view(&empty)).title("Empty").into(),
                    card(sample::memory_view(&one)).title("One sample").into(),
                    card(sample::network_view(&hidden))
                        .title("Received is hidden")
                        .into(),
                ])
                .spacing(space::S4)
                .into(),
            )
        },
    );
}

/// A chart too narrow for its labels leaves them out whole.
#[test]
fn a_narrow_chart_keeps_its_lines() {
    let state = sample::network(WINDOW);
    assert_snapshot(
        "chart-narrow-en",
        "en",
        &Theme::Light,
        (220.0, 300.0),
        || in_card(sample::network_view(&state)),
    );
}

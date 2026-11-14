//! The gallery's second page: buttons, form controls, and what floats.

use iced::{
    widget::{checkbox, text, toggler},
    Length,
};

use super::{labels, Gallery, Message};
use crate::ui::{
    theme::{
        self,
        metrics::{control, space, text as size, ButtonSize},
        Element, Icon, Tone,
    },
    widgets::{
        basic::{
            badge,
            button::{self, Variant},
            card, progress, scroll_area, separator, BadgeVariant,
        },
        form::{
            field, input, input_group, number_input, password, segmented, select, suggest, tabs,
            textarea, toggle_group, Addon, Number, TabsVariant, Toggle,
        },
        line,
        overlay::{menu, popover, tooltip, Side},
        paragraph, stack,
    },
};

const VARIANTS: [(&str, Variant); 6] = [
    ("Primary", Variant::Primary),
    ("Secondary", Variant::Secondary),
    ("Outline", Variant::Outline),
    ("Ghost", Variant::Ghost),
    ("Destructive", Variant::Destructive),
    ("Link", Variant::Link),
];

/// One column of the fields' grid.
fn column(fields: Vec<Element<'_, Message>>) -> iced::widget::Column<'_, Message, theme::Theme> {
    stack(fields).spacing(space::S4).width(260)
}

impl Gallery {
    pub(super) fn controls(&self) -> Element<'_, Message> {
        stack(vec![
            Self::section("Buttons", Self::buttons()),
            Self::section("Fields", self.fields()),
            Self::section("Select and combobox", self.selects()),
            Self::section("Tabs and toggles", self.groups()),
            Self::section("Menu, popover, tooltip", self.floating()),
            Self::section("Badge, card, progress, scroll area", Self::surfaces()),
        ])
        .spacing(space::S6)
        .into()
    }

    pub(super) fn buttons() -> Element<'static, Message> {
        let rows = VARIANTS.into_iter().map(|(name, variant)| {
            let of = |label: &'static str, size, on: bool| -> Element<'static, Message> {
                button::text(label)
                    .variant(variant)
                    .size(size)
                    .on_press_maybe(on.then_some(Message::Nothing))
                    .into()
            };
            line(vec![
                of(name, ButtonSize::Default, true),
                of("Large", ButtonSize::Lg, true),
                of("Small", ButtonSize::Sm, true),
                of("Tiny", ButtonSize::Xs, true),
                of("Disabled", ButtonSize::Default, false),
                button::labelled(Icon::RefreshCw, "Refresh")
                    .variant(variant)
                    .on_press(Message::Nothing)
                    .into(),
                button::icon(Icon::Settings)
                    .variant(variant)
                    .on_press(Message::Nothing)
                    .into(),
            ])
            .spacing(space::S3)
            .into()
        });
        stack(rows.collect()).spacing(space::S3).into()
    }

    pub(super) fn fields(&self) -> Element<'_, Message> {
        let wrong = self.errors.has("argument");
        let name = input("NAME=value", &self.name)
            .on_input(Message::Name)
            .style(if wrong {
                theme::text_input::invalid
            } else {
                theme::text_input::field_style
            });
        let search = input_group(
            input("Search containers", "").on_input(|_| Message::Nothing),
            Some(Addon::icon(Icon::Search)),
            Some(Addon::new(
                text("⌘K").size(size::XS).style(theme::text::muted),
                20.0,
            )),
        );
        let disabled = input("Disabled", "");
        let secret = password(
            "Password",
            &self.secret,
            self.revealed,
            Message::Secret,
            Message::Reveal,
        );
        let port = number_input(
            "8080",
            &self.port,
            Number::integer(1.0, 65535.0),
            Message::Port,
        );
        let notes = textarea(&self.notes, Message::Notes);
        let image = self.suggestion();
        line(vec![
            column(vec![
                field("Build argument", name)
                    .help("Passed to the build as it is.")
                    .error(self.errors.message("argument"))
                    .into(),
                field("Search", search).into(),
                field("Image", image)
                    .help("Free text, or one of the suggestions.")
                    .into(),
                field("Disabled", disabled).optional().into(),
            ])
            .into(),
            column(vec![
                field("Password", secret).into(),
                field("Port", port)
                    .help("From 1 to 65535; the arrows step it.")
                    .into(),
                field("Notes", notes).optional().into(),
            ])
            .into(),
            column(vec![
                checkbox(self.checked)
                    .label("Start on login")
                    .size(control::CHECKBOX)
                    .text_size(size::SM)
                    .on_toggle(Message::Checked)
                    .into(),
                toggler(self.checked)
                    .label("Check for updates")
                    .size(control::SWITCH_HEIGHT)
                    .text_size(size::SM)
                    .on_toggle(Message::Checked)
                    .into(),
            ])
            .into(),
        ])
        .spacing(space::S6)
        .align_y(iced::alignment::Vertical::Top)
        .into()
    }

    /// The images a field with suggestions offers, kept for as long as the
    /// program runs.
    pub(super) fn image_tags() -> &'static [String] {
        static TAGS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
            [
                "nginx:1.27",
                "nginx:latest",
                "postgres:16",
                "redis:7-alpine",
            ]
            .map(str::to_owned)
            .to_vec()
        });
        &TAGS
    }

    /// A field of free text with suggestions (`<datalist>`).
    pub(super) fn suggestion(&self) -> Element<'_, Message> {
        suggest(&self.images, &self.image)
            .placeholder("nginx:latest")
            .view()
            .map(Message::Image)
    }

    pub(super) fn selects(&self) -> Element<'_, Message> {
        let plain = select(&self.select, Self::engines(), self.engine)
            .placeholder("Engine")
            .width(220)
            .view()
            .map(Message::Select);
        let searchable = select(&self.combo, Self::engines(), self.runtime)
            .placeholder("Search a runtime")
            .searchable(true)
            .width(220)
            .view()
            .map(Message::Combo);
        let refused = select(&self.select, Self::engines(), None)
            .placeholder("Refused")
            .invalid(true)
            .disabled(true)
            .width(220)
            .view()
            .map(|_| Message::Nothing);
        line(vec![plain, searchable, refused])
            .spacing(space::S3)
            .into()
    }

    /// The selects' options, kept for as long as the program runs.
    pub(super) fn engines() -> &'static [String] {
        static LABELS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(labels);
        &LABELS
    }

    pub(super) fn groups(&self) -> Element<'_, Message> {
        let names = || {
            ["Overview", "Logs", "Inspect", "Stats"]
                .map(str::to_owned)
                .to_vec()
        };
        let styles = vec![
            Toggle::text("Bold"),
            Toggle::text("Italic"),
            Toggle::icon(Icon::Settings),
        ];
        stack(vec![
            line(vec![
                tabs(names(), self.tab, TabsVariant::Default, Message::Tabs),
                tabs(names(), self.tab, TabsVariant::Line, Message::Tabs),
            ])
            .spacing(space::S6)
            .into(),
            line(vec![
                segmented(
                    vec!["List".to_owned(), "Board".to_owned()],
                    self.layout,
                    Message::Layout,
                ),
                toggle_group(
                    styles.clone(),
                    |index| self.bold[index],
                    true,
                    Message::Bold,
                ),
                toggle_group(styles, |index| self.bold[index], false, Message::Bold),
            ])
            .spacing(space::S6)
            .into(),
        ])
        .spacing(space::S3)
        .into()
    }

    pub(super) fn floating(&self) -> Element<'_, Message> {
        let trigger = button::labelled(Icon::Ellipsis, "Actions")
            .variant(Variant::Outline)
            .on_press(Message::Menu(menu::Event::Toggle));
        let items = vec![
            menu::Item::Label("web".to_owned()),
            menu::Item::action("Start", Message::Chosen("Start"))
                .glyph(Icon::Play)
                .shortcut("S"),
            menu::Item::action("Restart", Message::Chosen("Restart")).glyph(Icon::RotateCw),
            menu::Item::action("Stop", Message::Chosen("Stop")).disabled(true),
            menu::Item::action("Follow logs", Message::Chosen("Follow")).checked(true),
            menu::Item::sub(
                "Copy",
                vec![
                    menu::Item::action("Name", Message::Chosen("Name")),
                    menu::Item::action("Id", Message::Chosen("Id")),
                    menu::Item::sub(
                        "Address",
                        vec![
                            menu::Item::action("IPv4", Message::Chosen("IPv4")),
                            menu::Item::action("IPv6", Message::Chosen("IPv6")),
                        ],
                    ),
                ],
            ),
            menu::Item::Separator,
            menu::Item::action("Remove", Message::Chosen("Remove"))
                .glyph(Icon::Trash)
                .destructive(),
        ];
        let actions = menu::dropdown(&self.menu, trigger, items, Message::Menu);
        let details = popover(
            button::text("Popover")
                .variant(Variant::Outline)
                .on_press(Message::Popover(!self.popover)),
            self.popover,
            || {
                stack(vec![
                    paragraph("Ports").font(theme::font::SANS_MEDIUM).into(),
                    paragraph("8080 → 80/tcp, published on every address.")
                        .style(theme::text::muted)
                        .into(),
                    button::text("Close")
                        .variant(Variant::Outline)
                        .size(ButtonSize::Sm)
                        .on_press(Message::Popover(false))
                        .into(),
                ])
                .spacing(space::S2)
                .into()
            },
            Message::Popover(false),
        );
        let hint = tooltip(
            button::icon(Icon::Info)
                .variant(Variant::Ghost)
                .on_press(Message::Nothing),
            "Shown on hover and on focus",
            Side::Top,
        );
        line(vec![actions, details.into(), hint.into()])
            .spacing(space::S3)
            .into()
    }

    pub(super) fn surfaces() -> Element<'static, Message> {
        let badges = line(vec![
            badge("Default", BadgeVariant::Primary),
            badge("Secondary", BadgeVariant::Secondary),
            badge("Destructive", BadgeVariant::Destructive),
            badge("Outline", BadgeVariant::Outline),
        ])
        .spacing(space::S2);
        // A check's outcome, on a line of its own: the card is narrow.
        let outcomes = line(vec![
            badge("exit 0", BadgeVariant::Passed),
            badge("exit 1", BadgeVariant::Failed),
        ])
        .spacing(space::S2);
        let badges = stack(vec![badges.into(), outcomes.into()]).spacing(space::S2);
        let bars = stack(vec![
            progress(35.0, None).length(200).into(),
            progress(82.0, Some(Tone::Warning)).length(200).into(),
            progress(100.0, Some(Tone::Danger)).length(200).into(),
        ])
        .spacing(space::S2)
        .width(200);
        let lines = (1..=12).map(|n| paragraph(format!("Layer {n} of nginx:1.27")).into());
        // As tall as it was before the badges took a second line.
        let layers = scroll_area(stack(lines.collect()).spacing(space::S1)).height(68);
        let panel = card(stack(vec![badges.into(), separator(), layers.into()]).spacing(space::S3))
            .title("nginx:1.27")
            .description("Pulled 3 days ago")
            .footer(bars)
            .view()
            .width(Length::Fixed(320.0));
        panel.into()
    }
}

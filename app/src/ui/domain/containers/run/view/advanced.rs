//! The Advanced area of the Run dialog (`AdvancedOptions` and its
//! sections): a tab per section, each marked while one of its fields
//! can't be read.

use super::{choice, half, lines, pair, rows, switch, typed};
use crate::{
    t,
    ui::{
        domain::containers::run::{
            form::{
                Caps, Choice, Form, Hint, Line, Lists, Message, Section, Switch, Text, DRIVERS,
            },
            model::{
                advanced::{Field, GpuMode, HealthMode},
                Errors,
            },
        },
        theme::{metrics::space, Element},
        widgets::{
            feedback::alert,
            form::{field, suggest::suggest_owned, tabs, TabsVariant},
            stack,
        },
    },
};

/// Samples of what a field takes: words of no language.
const LABELS_SAMPLE: &str =
    "traefik.enable=true\ntraefik.http.routers.web.rule=Host(`web.home.arpa`)";
const HEALTH_SAMPLE: &str = "wget -qO- http://localhost/ || exit 1";
const EXTRA_HOSTS_SAMPLE: &str = "nas.home.arpa:192.168.1.10\nhost.docker.internal:host-gateway";
const LOG_OPTIONS_SAMPLE: &str = "max-size=10m\nmax-file=3";

/// A labelled field of one line, with the help under it, or why its
/// value can't be read.
fn entry<'a>(
    errors: &Errors,
    which: Line,
    label: String,
    value: &str,
    placeholder: &str,
    checked: Option<Field>,
    help: Option<String>,
) -> Element<'a, Message> {
    let error = checked.and_then(|f| errors.message(f));
    let wrong = error.is_some();
    let mut entry = field(label, typed(which, value, placeholder, false, wrong)).error(error);
    if let Some(help) = help {
        entry = entry.help(help);
    }
    entry.into()
}

fn resources<'a>(form: &'a Form, errors: &Errors) -> Element<'a, Message> {
    let r = &form.form.advanced.resources;
    let one = |which, label, value: &str, placeholder: &str, checked, help| {
        entry(
            errors,
            which,
            label,
            value,
            placeholder,
            Some(checked),
            Some(help),
        )
    };
    stack(vec![
        pair(
            one(
                Line::Cpus,
                t!("containers.advanced.cpus"),
                &r.cpus,
                &t!("containers.advanced.cpusPlaceholder"),
                Field::Cpus,
                t!("containers.advanced.cpusHint"),
            ),
            one(
                Line::CpuShares,
                t!("containers.advanced.cpuShares"),
                &r.cpu_shares,
                "1024",
                Field::CpuShares,
                t!("containers.advanced.cpuSharesHint"),
            ),
        ),
        pair(
            one(
                Line::Memory,
                t!("containers.advanced.memory"),
                &r.memory,
                &t!("containers.advanced.memoryPlaceholder"),
                Field::Memory,
                t!("containers.advanced.memoryHint"),
            ),
            one(
                Line::MemoryReservation,
                t!("containers.advanced.memoryReservation"),
                &r.memory_reservation,
                &t!("containers.advanced.none"),
                Field::MemoryReservation,
                t!("containers.advanced.memoryReservationHint"),
            ),
        ),
        pair(
            one(
                Line::MemorySwap,
                t!("containers.advanced.memorySwap"),
                &r.memory_swap,
                &t!("containers.advanced.memorySwapPlaceholder"),
                Field::MemorySwap,
                t!("containers.advanced.memorySwapHint"),
            ),
            iced::widget::Space::new(),
        ),
    ])
    .spacing(space::S3)
    .into()
}

fn security<'a>(form: &'a Form, lists: &Lists) -> Element<'a, Message> {
    let security = &form.form.advanced.security;
    let mut parts = vec![
        pair(
            field(
                t!("containers.advanced.capAdd"),
                rows::capabilities(form, lists, Caps::Add, "NET_ADMIN"),
            )
            .help(t!("containers.advanced.capAddHint")),
            field(
                t!("containers.advanced.capDrop"),
                rows::capabilities(form, lists, Caps::Drop, "ALL"),
            )
            .help(t!("containers.advanced.capDropHint")),
        ),
        switch(
            Switch::Privileged,
            security.privileged,
            t!("containers.advanced.privileged"),
        ),
    ];
    if security.privileged {
        parts.push(alert(
            Some(t!("containers.advanced.privilegedTitle")),
            t!("containers.advanced.privilegedWarning"),
            true,
        ));
    }
    stack(parts).spacing(space::S3).into()
}

fn devices<'a>(form: &'a Form, lists: &Lists, errors: &Errors) -> Element<'a, Message> {
    let gpus = &form.form.advanced.security.gpus;
    let mode = field(
        t!("containers.advanced.gpus"),
        choice(form, Choice::Gpus, lists, false),
    )
    .help(t!("containers.advanced.gpusHint"));
    let second: Element<'a, Message> = match gpus.mode {
        GpuMode::Count => entry(
            errors,
            Line::GpuCount,
            t!("containers.advanced.gpuCount"),
            &gpus.count,
            "",
            Some(Field::GpuCount),
            None,
        ),
        GpuMode::Ids => field(
            t!("containers.advanced.gpuIds"),
            typed(Line::GpuIds, &gpus.ids, "0, 1", true, false),
        )
        .help(t!("containers.advanced.gpuIdsHint"))
        .into(),
        GpuMode::None | GpuMode::All => iced::widget::Space::new().into(),
    };
    stack(vec![rows::devices(form), pair(mode, second)])
        .spacing(space::S3)
        .into()
}

fn health<'a>(form: &'a Form, lists: &Lists, errors: &Errors) -> Element<'a, Message> {
    let h = &form.form.advanced.healthcheck;
    let mode = field(
        t!("containers.advanced.healthcheck"),
        choice(form, Choice::Health, lists, false),
    );
    let mut parts = vec![mode.into()];
    if h.mode == HealthMode::Custom {
        parts.push(
            field(
                t!("containers.advanced.healthCommand"),
                typed(Line::HealthCommand, &h.command, HEALTH_SAMPLE, true, false),
            )
            .help(t!("containers.advanced.healthCommandHint"))
            .into(),
        );
        let one = |which, label, value: &str, placeholder: &str, checked| {
            half(entry(
                errors,
                which,
                label,
                value,
                placeholder,
                Some(checked),
                None,
            ))
        };
        parts.push(
            crate::ui::widgets::line(vec![
                one(
                    Line::Interval,
                    t!("containers.advanced.interval"),
                    &h.interval,
                    "30s",
                    Field::Interval,
                ),
                one(
                    Line::Timeout,
                    t!("containers.advanced.timeout"),
                    &h.timeout,
                    "30s",
                    Field::Timeout,
                ),
                one(
                    Line::Retries,
                    t!("containers.advanced.retries"),
                    &h.retries,
                    "3",
                    Field::Retries,
                ),
                one(
                    Line::StartPeriod,
                    t!("containers.advanced.startPeriod"),
                    &h.start_period,
                    "0s",
                    Field::StartPeriod,
                ),
            ])
            .spacing(space::S3)
            .align_y(iced::alignment::Vertical::Top)
            .into(),
        );
    }
    stack(parts).spacing(space::S3).into()
}

fn identity(form: &Form) -> Element<'_, Message> {
    let identity = &form.form.advanced.identity;
    stack(vec![
        pair(
            field(
                t!("containers.advanced.hostname"),
                typed(
                    Line::Hostname,
                    &identity.hostname,
                    &t!("containers.advanced.hostnamePlaceholder"),
                    false,
                    false,
                ),
            ),
            field(
                t!("containers.advanced.domainname"),
                typed(
                    Line::Domainname,
                    &identity.domainname,
                    &t!("containers.advanced.none"),
                    false,
                    false,
                ),
            ),
        ),
        pair(
            field(
                t!("containers.advanced.dns"),
                lines(form, Text::Dns, "192.168.1.1"),
            )
            .help(t!("containers.advanced.dnsHint")),
            field(
                t!("containers.advanced.dnsSearch"),
                lines(form, Text::DnsSearch, "home.arpa"),
            )
            .help(t!("containers.advanced.dnsSearchHint")),
        ),
        field(
            t!("containers.advanced.extraHosts"),
            lines(form, Text::ExtraHosts, EXTRA_HOSTS_SAMPLE),
        )
        .help(t!("containers.advanced.extraHostsHint"))
        .into(),
    ])
    .spacing(space::S3)
    .into()
}

fn logging(form: &Form) -> Element<'_, Message> {
    let logging = &form.form.advanced.logging;
    pair(
        field(
            t!("containers.advanced.logDriver"),
            suggest_owned(
                form.hint(Hint::LogDriver),
                DRIVERS.map(str::to_owned).to_vec(),
                &logging.driver,
            )
            .placeholder(t!("containers.advanced.logDriverPlaceholder"))
            .font(crate::ui::theme::font::MONO)
            .view()
            .map(|message| Message::Suggest(Hint::LogDriver, message)),
        )
        .help(t!("containers.advanced.logDriverHint")),
        field(
            t!("containers.advanced.logOptions"),
            lines(form, Text::LogOptions, LOG_OPTIONS_SAMPLE),
        )
        .help(t!("containers.advanced.logOptionsHint")),
    )
}

/// A section's name on its tab, with a dot while one of its fields
/// can't be read (React's red dot, named `hasErrors`): the tabs of the
/// kit are text.
fn tab_label(section: Section, errors: &Errors) -> String {
    let label = section.label();
    if section.fails(errors) {
        format!("{label} \u{2022}")
    } else {
        label
    }
}

/// The tabs of the sections, and the section that shows.
pub(super) fn view<'a>(form: &'a Form, lists: &Lists, errors: &Errors) -> Element<'a, Message> {
    let labels = Section::ALL.iter().map(|s| tab_label(*s, errors)).collect();
    let chosen = Section::ALL
        .iter()
        .position(|s| *s == form.section)
        .unwrap_or(0);
    let shown = match form.section {
        Section::Resources => resources(form, errors),
        Section::Labels => field(
            t!("containers.advanced.labels"),
            lines(form, Text::Labels, LABELS_SAMPLE),
        )
        .help(t!("containers.advanced.labelsHint"))
        .into(),
        Section::Security => security(form, lists),
        Section::Devices => devices(form, lists, errors),
        Section::Health => health(form, lists, errors),
        Section::Network => identity(form),
        Section::Logging => logging(form),
    };
    let tabs = tabs(labels, chosen, TabsVariant::Default, |index| {
        Message::Section(Section::ALL[index])
    });
    stack(vec![tabs, shown]).spacing(space::S3).into()
}

//! The rows of the Run dialog that are added and removed (`PortsField`,
//! `MountsField`, `DevicesField`) and the lists of capabilities typed a
//! name at a time (`TokenInput`).

use iced::{
    keyboard::{key::Named, Key},
    widget::{checkbox, container, text},
    Length,
};

use super::{choice, typed};
use crate::{
    t,
    ui::{
        domain::containers::run::{
            form::{Caps, Choice, Form, Hint, Line, Lists, Message, Rows, Switch},
            model::Errors,
        },
        theme::{
            self,
            metrics::{control, space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::{
                badge,
                button::{self, Variant},
                flow, technical, BadgeVariant,
            },
            focus::key_area,
            form::{field, suggest::suggest},
            line,
            overlay::{tooltip, Side},
            paragraph, stack,
        },
    },
};

/// The arrow between a row's two sides, pointing the way the language
/// reads: a sign of no language.
fn arrow<'a>() -> Element<'a, Message> {
    let arrow = if crate::i18n::direction().is_rtl() {
        "\u{2190}"
    } else {
        "\u{2192}"
    };
    text(arrow).style(theme::text::muted).into()
}

/// The × that removes a row, named under the pointer.
fn remove<'a>(rows: Rows, index: usize, label: String) -> Element<'a, Message> {
    let button = button::icon(Icon::X)
        .variant(Variant::Ghost)
        .size(ButtonSize::Sm)
        .on_press(Message::Remove(rows, index));
    tooltip(button, label, Side::Top).into()
}

/// The button that adds a row.
fn add<'a>(rows: Rows, label: String) -> Element<'a, Message> {
    button::labelled(Icon::Plus, label)
        .variant(Variant::Outline)
        .size(ButtonSize::Sm)
        .on_press(Message::Add(rows))
        .into()
}

/// What stands between a row's second field and its ×, as wide in the
/// rows of ports as in those of volumes, so their fields line up: the
/// protocol, or "read-only" (as long as German makes it).
const TRAILING: f32 = 144.0;

/// A field of a row, as tall as the select beside it (`h-9` both).
fn in_row(
    field: iced::widget::TextInput<'_, Message, crate::ui::theme::Theme>,
) -> Element<'_, Message> {
    field
        .padding([6.0, control::PADDING_X])
        .line_height(iced::widget::text::LineHeight::Relative(size::LEADING_SM))
        .into()
}

/// A field that takes its share of the row.
fn grow_field(
    field: iced::widget::TextInput<'_, Message, crate::ui::theme::Theme>,
) -> Element<'_, Message> {
    grow(in_row(field))
}

/// A part of a row that takes its share of it.
fn grow<'a>(part: Element<'a, Message>) -> Element<'a, Message> {
    container(part).width(Length::Fill).into()
}

fn group<'a>(
    label: String,
    mut rows: Vec<Element<'a, Message>>,
    add: Element<'a, Message>,
) -> Element<'a, Message> {
    rows.push(add);
    field(label, stack(rows).spacing(space::S1_5)).into()
}

/// The published ports, one row each.
pub(super) fn ports<'a>(form: &'a Form, lists: &Lists, errors: &Errors) -> Element<'a, Message> {
    let mut rows = vec![];
    for (i, port) in form.form.ports.iter().enumerate() {
        let wrong = errors.ports.contains(&i);
        // `w-24`, at the start of the room the row's end keeps.
        let protocol =
            container(container(choice(form, Choice::Protocol(i), lists, false)).width(96))
                .width(TRAILING);
        rows.push(
            line(vec![
                grow_field(typed(
                    Line::HostPort(i),
                    &port.host_port,
                    &t!("containers.run.hostPortPlaceholder"),
                    false,
                    wrong,
                )),
                arrow(),
                grow_field(typed(
                    Line::ContainerPort(i),
                    &port.container_port,
                    &t!("containers.run.containerPort"),
                    false,
                    wrong,
                )),
                protocol.into(),
                remove(Rows::Ports, i, t!("containers.run.removePort")),
            ])
            .spacing(space::S2)
            .width(Length::Fill)
            .align_y(iced::alignment::Vertical::Center)
            .into(),
        );
        if wrong {
            rows.push(
                paragraph(t!("containers.run.error.port"))
                    .style(theme::text::destructive)
                    .into(),
            );
        }
    }
    group(
        t!("containers.run.ports"),
        rows,
        add(Rows::Ports, t!("containers.run.addPort")),
    )
}

/// The volumes and bind mounts, one row each.
pub(super) fn mounts(form: &Form) -> Element<'_, Message> {
    let rows = form
        .form
        .mounts
        .iter()
        .enumerate()
        .map(|(i, mount)| {
            let read_only = checkbox(mount.read_only)
                .label(t!("containers.run.readOnly"))
                .size(control::CHECKBOX)
                .text_size(size::SM)
                .on_toggle(move |on| Message::Switch(Switch::ReadOnly(i), on));
            line(vec![
                grow_field(typed(
                    Line::Source(i),
                    &mount.source,
                    &t!("containers.run.sourcePlaceholder"),
                    false,
                    false,
                )),
                arrow(),
                // A path's sample, of no language.
                grow_field(typed(
                    Line::Target(i),
                    &mount.target,
                    "/container/path",
                    false,
                    false,
                )),
                container(read_only).width(TRAILING).into(),
                remove(Rows::Mounts, i, t!("containers.run.removeVolume")),
            ])
            .spacing(space::S2)
            .width(Length::Fill)
            .align_y(iced::alignment::Vertical::Center)
            .into()
        })
        .collect();
    group(
        t!("containers.run.volumes"),
        rows,
        add(Rows::Mounts, t!("containers.run.addVolume")),
    )
}

/// The devices of the Host, one row each.
pub(super) fn devices(form: &Form) -> Element<'_, Message> {
    let rows = form
        .form
        .advanced
        .security
        .devices
        .iter()
        .enumerate()
        .map(|(i, device)| {
            let permissions = typed(
                Line::Permissions(i),
                &device.permissions,
                "rwm",
                true,
                false,
            );
            line(vec![
                grow_field(typed(
                    Line::HostDevice(i),
                    &device.host_path,
                    "/dev/dri",
                    true,
                    false,
                )),
                arrow(),
                grow_field(typed(
                    Line::ContainerDevice(i),
                    &device.container_path,
                    &t!("containers.advanced.samePath"),
                    true,
                    false,
                )),
                container(in_row(permissions)).width(80).into(),
                remove(Rows::Devices, i, t!("containers.advanced.removeDevice")),
            ])
            .spacing(space::S2)
            .width(Length::Fill)
            .align_y(iced::alignment::Vertical::Center)
            .into()
        })
        .collect();
    group(
        t!("containers.advanced.devices"),
        rows,
        add(Rows::Devices, t!("containers.advanced.addDevice")),
    )
}

/// A list of capabilities (`TokenInput`): those chosen as badges with a
/// × each, and a field where Enter or a comma adds what was typed and
/// Backspace in an empty field takes the last away.
pub(super) fn capabilities<'a>(
    form: &'a Form,
    caps: Caps,
    sample: &'a str,
) -> Element<'a, Message> {
    let security = &form.form.advanced.security;
    let values = match caps {
        Caps::Add => &security.cap_add,
        Caps::Drop => &security.cap_drop,
    };
    let mut parts: Vec<Element<'a, Message>> = vec![];
    if !values.is_empty() {
        let badges = values
            .iter()
            .enumerate()
            .map(|(i, value)| {
                let remove = button::icon(Icon::X)
                    .variant(Variant::Ghost)
                    .size(ButtonSize::Xs)
                    .on_press(Message::RemoveCapability(caps, i));
                let remove = tooltip(
                    remove,
                    t!("containers.run.removeValue", value = technical(value)),
                    Side::Top,
                );
                line(vec![
                    badge(technical(value), BadgeVariant::Secondary),
                    remove.into(),
                ])
                .spacing(space::S0_5)
                .align_y(iced::alignment::Vertical::Center)
                .into()
            })
            .collect();
        parts.push(flow(badges).spacing(space::S1).into());
    }
    let draft = &form.drafts[caps as usize];
    let field = suggest(form.hint(Hint::Capability(caps)), draft)
        .placeholder(sample)
        .font(super::machine_face(draft))
        .view()
        .map(move |message| Message::Suggest(Hint::Capability(caps), message));
    let empty = draft.is_empty();
    // Backspace in an empty field: the field has nothing to take away.
    let field = key_area(field, |_, _| None).before(move |key, _| {
        (empty && *key == Key::Named(Named::Backspace))
            .then_some(Message::RemoveLastCapability(caps))
    });
    parts.push(field.into());
    stack(parts).spacing(space::S1_5).into()
}

//! What the Run dialog's fields draw (`RunFormFields`): the image and the
//! name, the command, the pod, the ports and volumes, the environment,
//! the network and the restart policy, and the Advanced area.

mod advanced;
mod rows;

use iced::{
    widget::{container, text, toggler},
    Length,
};

use homeport_registries::Registry;

use super::form::{Choice, Form, Line, Lists, Message, Switch, Text};
use crate::{
    data::Resource,
    t,
    ui::{
        theme::{
            self, font, icon,
            metrics::{control, space, text as size},
            Element, Icon,
        },
        widgets::{
            basic::collapsible,
            form::{field, input, select::select_owned, textarea},
            line, stack,
        },
    },
};

/// The image's sample, a name of no language.
const IMAGE_SAMPLE: &str = "nginx:latest";
/// The environment's sample.
const ENV_SAMPLE: &str = "KEY=value\nOTHER=value";

/// Two fields side by side, each half the row (`grid-cols-2`).
pub(super) fn pair<'a>(
    first: impl Into<Element<'a, Message>>,
    second: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    line(vec![half(first), half(second)])
        .spacing(space::S3)
        .width(Length::Fill)
        .align_y(iced::alignment::Vertical::Top)
        .into()
}

pub(super) fn half<'a>(part: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(part).width(Length::Fill).into()
}

/// A one-line field: its text, what it says when empty, in the app's
/// face or the monospaced one, marked when it can't be read.
pub(super) fn typed<'a>(
    which: Line,
    value: &str,
    placeholder: &str,
    mono: bool,
    wrong: bool,
) -> Element<'a, Message> {
    let field = input(placeholder, value)
        .on_input(move |typed| Message::Typed(which, typed))
        .on_submit(Message::Submit);
    let field = if mono { field.font(font::MONO) } else { field };
    if wrong {
        field.style(theme::text_input::invalid).into()
    } else {
        field.into()
    }
}

/// A field of several lines, in the monospaced face.
pub(super) fn lines<'a>(form: &'a Form, which: Text, placeholder: &'a str) -> Element<'a, Message> {
    textarea(form.editor(which), move |action| {
        Message::Text(which, action)
    })
    .placeholder(placeholder)
    .font(font::MONO)
    .size(size::XS)
    .into()
}

/// A select over the options the form says.
pub(super) fn choice<'a>(
    form: &'a Form,
    which: Choice,
    lists: &Lists,
    disabled: bool,
) -> Element<'a, Message> {
    let (labels, chosen) = form.options(which, lists);
    select_owned(form.select(which), labels, chosen)
        .disabled(disabled)
        .width(Length::Fill)
        .view()
        .map(move |message| Message::Choose(which, message))
}

/// A switch with its label.
pub(super) fn switch<'a>(which: Switch, on: bool, label: String) -> Element<'a, Message> {
    toggler(on)
        .label(label)
        .size(control::SWITCH_HEIGHT)
        .text_size(size::SM)
        .on_toggle(move |on| Message::Switch(which, on))
        .into()
}

/// Which stored login a pull of the image will use (`RegistryLogin`):
/// "Pulled as alice on ghcr.io", or "Pulled anonymously".
fn login<'a>(read: &Resource<std::sync::Arc<Option<Registry>>>) -> Option<Element<'a, Message>> {
    // Neither while it is read nor when it could not be.
    let login = read.data()?;
    let words = match login.as_ref() {
        Some(login) => t!(
            "registries.login.pulledAs",
            username = login.username.as_str(),
            registry = login.address.as_str()
        ),
        None => t!("registries.login.pulledAnonymously"),
    };
    Some(
        line(vec![
            icon(Icon::KeyRound).style(theme::svg::muted).into(),
            text(words).size(size::XS).style(theme::text::muted).into(),
        ])
        .spacing(space::S1_5)
        .into(),
    )
}

/// Every field of the dialog. `registry` is the login the image would be
/// pulled with, read for an image the Host doesn't have.
pub fn fields<'a>(
    form: &'a Form,
    lists: &Lists,
    registry: Option<&Resource<std::sync::Arc<Option<Registry>>>>,
) -> Element<'a, Message> {
    let run = &form.form;
    let errors = form.errors();
    let mut image = field(
        t!("containers.run.image"),
        typed(Line::Image, &run.image, IMAGE_SAMPLE, false, false),
    )
    .view();
    if let Some(shown) = registry.and_then(login) {
        image = stack(vec![image, shown]).spacing(space::S1_5).into();
    }
    let name_field = input(&t!("containers.run.namePlaceholder"), &run.name)
        .on_submit(Message::Submit)
        .on_input_maybe((!form.lock_name).then_some(|typed| Message::Typed(Line::Name, typed)));
    let mut name = field(t!("containers.run.name"), name_field);
    if form.lock_name {
        name = name.help(t!("containers.run.nameKept"));
    }
    let mut parts: Vec<Element<'a, Message>> = vec![
        pair(image, name),
        field(
            t!("containers.run.command"),
            typed(
                Line::Command,
                &run.command,
                &t!("containers.run.commandPlaceholder"),
                true,
                false,
            ),
        )
        .into(),
    ];
    if form.offers_pods {
        let mut pod = field(
            t!("containers.run.pod"),
            choice(form, Choice::Pod, lists, false),
        );
        if form.in_pod() {
            pod = pod.help(t!("containers.run.podOwnsNetwork", pod = run.pod.as_str()));
        }
        parts.push(pod.into());
    }
    if !form.in_pod() {
        parts.push(rows::ports(form, lists, &errors));
    }
    parts.push(rows::mounts(form));
    parts.push(
        field(
            t!("containers.run.environment"),
            lines(form, Text::Env, ENV_SAMPLE),
        )
        .into(),
    );
    let restart = field(
        t!("containers.run.restart"),
        choice(form, Choice::Restart, lists, run.auto_remove),
    );
    parts.push(if form.in_pod() {
        pair(restart, iced::widget::Space::new())
    } else {
        let network = field(
            t!("containers.run.network"),
            choice(form, Choice::Network, lists, false),
        );
        pair(network, restart)
    });
    parts.push(switch(
        Switch::AutoRemove,
        run.auto_remove,
        t!("containers.run.autoRemove"),
    ));
    let open = form.shows_advanced();
    parts.push(collapsible(
        t!("containers.advanced.title"),
        open,
        |open| Message::Switch(Switch::Advanced, open),
        || advanced::view(form, lists, &errors),
    ));
    stack(parts).spacing(space::S4).width(Length::Fill).into()
}

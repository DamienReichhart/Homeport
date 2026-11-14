//! What the Exec tab draws: a tab per terminal with its ×, "New
//! terminal" and its popover (command, user, folder), and the terminal
//! that shows.

use iced::{widget::container, Length};

use super::super::tabs::exec::{Draft, Exec, Message};
use crate::{
    t,
    ui::{
        theme::{
            self,
            metrics::{space, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::button::{self, Variant},
            feedback::error_message,
            form::{field, input},
            line,
            overlay::{popover, tooltip, Side},
            paragraph, stack, terminal,
        },
    },
};

/// What the tab row and the gap under it take of the height.
const TABS: f32 = 44.0;

/// The popover of "New terminal" (`w-80`). Enter in its fields does
/// nothing, as in React, whose fields were in no form: Open opens.
fn asking(exec: &Exec) -> Element<'_, Message> {
    let draft = &exec.draft;
    let typed = |which: Draft| move |text| Message::Typed(which, text);
    let command = input(&t!("containers.exec.commandPlaceholder"), &draft.command)
        .font(theme::font::MONO)
        .on_input(typed(Draft::Command));
    let user = input(&t!("containers.exec.default"), &draft.user).on_input(typed(Draft::User));
    let directory =
        input(&t!("containers.exec.default"), &draft.directory).on_input(typed(Draft::Directory));
    let half = |part: Element<'static, Message>| -> Element<'static, Message> {
        container(part).width(Length::Fill).into()
    };
    stack(vec![
        field(t!("containers.exec.command"), command).into(),
        line(vec![
            half(field(t!("containers.exec.user"), user).into()),
            half(field(t!("containers.exec.directory"), directory).into()),
        ])
        .spacing(space::S2)
        .width(Length::Fill)
        .into(),
        button::text(t!("containers.exec.open"))
            .on_press(Message::Open)
            .into(),
    ])
    .spacing(space::S3)
    .width(Length::Fill)
    .into()
}

/// A terminal's tab: its title, which shows it, and its ×.
fn tab(key: u32, title: String, active: bool) -> Element<'static, Message> {
    let show = button::text(title.clone())
        .variant(if active {
            Variant::Secondary
        } else {
            Variant::Ghost
        })
        .size(ButtonSize::Sm)
        .on_press(Message::Show(key));
    let close = button::icon(Icon::X)
        .variant(Variant::Ghost)
        .size(ButtonSize::Xs)
        .on_press(Message::Close(key));
    let close = tooltip(
        close,
        t!("containers.exec.closeOne", title = title),
        Side::Top,
    );
    line(vec![show.into(), close.into()])
        .spacing(space::S0_5)
        .align_y(iced::alignment::Vertical::Center)
        .into()
}

/// The tab, `height` tall in all.
pub fn view(exec: &Exec, height: f32) -> Element<'_, Message> {
    let mut tabs: Vec<Element<'_, Message>> = exec
        .shells
        .iter()
        .map(|shell| tab(shell.key, shell.title.text(), shell.key == exec.active))
        .collect();
    let new = button::labelled(Icon::Plus, t!("containers.exec.new"))
        .variant(Variant::Ghost)
        .size(ButtonSize::Sm)
        .on_press(Message::New);
    tabs.push(popover(new, exec.asking, || asking(exec), Message::New).into());
    let row = line(tabs)
        .spacing(space::S1)
        .align_y(iced::alignment::Vertical::Center);
    let shown = exec.shells.iter().find(|shell| shell.key == exec.active);
    let body: Element<'_, Message> = match shown {
        None => paragraph(t!("containers.exec.none"))
            .style(theme::text::muted)
            .into(),
        Some(shell) => {
            let key = shell.key;
            let failed = shell.failure.as_ref().map(|failure| {
                error_message(&failure.error, false, move |event| {
                    Message::Failure(key, event)
                })
            });
            terminal::Terminal::new(&shell.terminal, move |message| {
                Message::Terminal(key, message)
            })
            .failed(failed)
            .height((height - TABS).max(super::logs::MIN_HEIGHT))
            .view()
        }
    };
    stack(vec![row.into(), body])
        .spacing(space::S2)
        .width(Length::Fill)
        .into()
}

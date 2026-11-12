//! What the container actions draw: the start and stop button of a row,
//! the items of its menu, the bar of a selection and the dialogs.

use std::collections::BTreeMap;

use iced::widget::text;

use homeport_engine::containers::ContainerAction;

use super::{Actions, Bulk, Dialog, Message, Target};
use crate::{
    i18n::format,
    t,
    ui::{
        theme::{
            self,
            metrics::{space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::button::{self, Variant},
            feedback::{confirm_dialog, error_message},
            form::input,
            line,
            overlay::{dialog, menu::Item, modal, tooltip, Modal, Side},
        },
    },
};

/// The toast after an action, which takes the number of containers.
pub(super) fn done(action: ContainerAction, count: usize) -> String {
    match action {
        ContainerAction::Start => t!("containers.done.start", count = count),
        ContainerAction::Stop => t!("containers.done.stop", count = count),
        ContainerAction::Restart => t!("containers.done.restart", count = count),
        ContainerAction::Pause => t!("containers.done.pause", count = count),
        ContainerAction::Unpause => t!("containers.done.unpause", count = count),
        ContainerAction::Kill => t!("containers.done.kill", count = count),
    }
}

/// The title and the words of the confirmation for removing containers,
/// naming what goes with them (`removeConfirmation`).
pub(super) fn removal(targets: &[Target]) -> (String, String) {
    let count = targets.len();
    let running = targets.iter().filter(|target| target.is_running()).count();
    let title = match targets {
        [only] => t!("containers.remove.title", name = only.name.as_str()),
        _ => t!("containers.remove.titleMany", count = count),
    };
    let mut says = vec![t!("containers.remove.deleted", count = count)];
    if running > 0 {
        says.push(t!("containers.remove.running", count = running));
    }
    says.push(t!("containers.remove.namedKept"));
    (title, says.join(" "))
}

/// How many names of a Host's containers a confirmation spells out.
const NAMED: usize = 8;

/// What ends a list that is cut.
const ELLIPSIS: char = '…';

/// The same for a selection that may span Hosts: what is removed, Host by
/// Host (`BoardSelection`'s `RemoveSummary`).
pub(super) fn removal_by_host(targets: &[Target]) -> (String, String) {
    let mut hosts: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for target in targets {
        hosts
            .entry(&target.host_name)
            .or_default()
            .push(&target.name);
    }
    let mut lines = vec![t!("board.remove.description")];
    for (host, names) in hosts {
        let shown: Vec<&str> = names.iter().take(NAMED).copied().collect();
        let mut names_shown = format::list(&shown);
        if names.len() > NAMED {
            // A Host with a thousand selected is not read name by name:
            // the list goes on.
            names_shown.push(ELLIPSIS);
        }
        let names = names_shown;
        lines.push(t!("board.remove.onHost", host = host, names = names));
    }
    let title = t!("board.remove.title", count = targets.len());
    (title, lines.join("\n"))
}

/// The bar of a selection: how many are selected, and Start, Stop,
/// Restart and Remove for them (`SelectionActions`, `BoardSelection`).
/// The page collects the selected containers when a button is pressed.
pub fn selection_bar<'a, Message: Clone + 'a>(
    count: usize,
    on_press: impl Fn(Bulk) -> Message,
) -> Element<'a, Message> {
    let small = |glyph, label: String, variant, bulk| -> Element<'a, Message> {
        button::labelled(glyph, label)
            .variant(variant)
            .size(ButtonSize::Sm)
            .on_press(on_press(bulk))
            .into()
    };
    let outline = Variant::Outline;
    line(vec![
        text(t!("containers.selected", count = count))
            .size(size::SM)
            .style(theme::text::muted)
            .into(),
        small(
            Icon::Play,
            t!("containers.action.start"),
            outline,
            Bulk::Start,
        ),
        small(
            Icon::Square,
            t!("containers.action.stop"),
            outline,
            Bulk::Stop,
        ),
        small(
            Icon::RotateCw,
            t!("containers.action.restart"),
            outline,
            Bulk::Restart,
        ),
        small(
            Icon::Trash,
            t!("common.remove"),
            Variant::Destructive,
            Bulk::Remove,
        ),
    ])
    .spacing(space::S1_5)
    .into()
}

impl Actions {
    /// The button at the end of a row: Stop for a container that runs or
    /// is paused, Start for the others. It waits while an action is on its
    /// way for the container.
    pub fn toggle<'a>(&self, target: Target) -> Element<'a, Message> {
        let up = target.is_running() || target.is_paused();
        let (glyph, label, action) = if up {
            (
                Icon::Square,
                t!("containers.action.stop"),
                ContainerAction::Stop,
            )
        } else {
            (
                Icon::Play,
                t!("containers.action.start"),
                ContainerAction::Start,
            )
        };
        let waits = self.is_pending(&target.host_id, &target.id);
        let press = (!waits).then(|| Message::Act(action, vec![target]));
        let button = button::icon(glyph)
            .variant(Variant::Ghost)
            .size(ButtonSize::Sm)
            .on_press_maybe(press);
        tooltip(button, label, Side::Top).into()
    }

    /// The items of a container's menu (`ContainerMenu`).
    pub fn menu(target: &Target) -> Vec<Item<Message>> {
        let (running, paused) = (target.is_running(), target.is_paused());
        let act = |action| Message::Act(action, vec![target.clone()]);
        let item =
            |label: String, glyph, message: Message| Item::action(label, message).glyph(glyph);
        // Their dialogs come with the container's page and the Run dialog.
        let later = |label: String, glyph| item(label, glyph, Message::CloseDialog).disabled(true);
        let pause = if paused {
            item(
                t!("containers.action.resume"),
                Icon::Play,
                act(ContainerAction::Unpause),
            )
        } else {
            item(
                t!("containers.action.pause"),
                Icon::Pause,
                act(ContainerAction::Pause),
            )
            .disabled(!running)
        };
        vec![
            item(
                t!("containers.action.logs"),
                Icon::ScrollText,
                Message::Open(target.clone(), "logs"),
            ),
            item(
                t!("containers.action.terminal"),
                Icon::SquareTerminal,
                Message::Open(target.clone(), "exec"),
            )
            .disabled(!running),
            Item::Separator,
            item(
                t!("containers.action.restart"),
                Icon::RotateCw,
                act(ContainerAction::Restart),
            )
            .disabled(!running && !paused),
            pause,
            item(
                t!("containers.action.kill"),
                Icon::Skull,
                Message::AskKill(target.clone()),
            )
            .disabled(!running),
            item(
                t!("containers.action.rename"),
                Icon::Pencil,
                Message::AskRename(target.clone()),
            ),
            later(t!("containers.action.edit"), Icon::SlidersHorizontal),
            later(t!("containers.action.duplicate"), Icon::Copy),
            later(t!("containers.action.commit"), Icon::ImageDown),
            later(t!("containers.action.export"), Icon::FileArchive),
            Item::Separator,
            item(
                t!("common.remove"),
                Icon::Trash,
                Message::AskRemove {
                    targets: vec![target.clone()],
                    by_host: false,
                },
            )
            .destructive(),
        ]
    }

    /// The confirmation or the Rename dialog that is open, to lay over
    /// the page with `overlay::over`.
    pub fn dialog(&self) -> Option<Modal<'_, Message>> {
        match self.dialog.as_ref()? {
            Dialog::Confirm { what, state } => {
                Some(confirm_dialog(&Self::ask(what), state).map(Message::Confirm))
            }
            Dialog::Rename {
                target,
                name,
                pending,
                failure,
            } => {
                let waits = *pending;
                let field = input(&t!("containers.rename.newName"), name)
                    .on_input_maybe((!waits).then_some(Message::RenameInput))
                    .on_submit(Message::RenameSubmit);
                let mut content =
                    dialog(t!("containers.rename.title", name = target.name.as_str())).push(field);
                if let Some(failure) = failure {
                    content =
                        content.push(error_message(&failure.error, false, Message::RenameFailure));
                }
                let cancel = button::text(t!("common.cancel"))
                    .variant(Variant::Ghost)
                    .on_press(Message::CloseDialog);
                let rename = button::text(t!("containers.action.rename"))
                    .on_press_maybe((!waits).then_some(Message::RenameSubmit));
                Some(
                    modal(content.footer(vec![cancel.into(), rename.into()]))
                        .key("rename")
                        .on_close(Message::CloseDialog)
                        .close_on_scrim(),
                )
            }
        }
    }
}

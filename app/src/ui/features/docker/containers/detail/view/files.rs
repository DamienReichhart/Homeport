//! The Files tab: the kit's file browser, with "Upload here" and the
//! transfer that runs.

use iced::widget::text;

use crate::{
    t,
    ui::{
        features::docker::containers::detail::tabs::files::{Doing, Files, Message},
        theme::{
            self,
            metrics::{space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::{
                button::{self, Variant},
                in_reading_order, technical,
            },
            data::file_browser::FileBrowser,
            feedback::spinner::turning,
            line, paragraph,
        },
    },
};

/// What runs, said, with what stops it.
fn transfer<'a>(doing: &Doing) -> Element<'a, Message> {
    let said = match doing {
        Doing::Download(name) => t!("containers.files.downloading", name = technical(name)),
        Doing::Upload {
            folder,
            done,
            count,
        } => t!(
            "containers.files.uploading",
            done = *done,
            count = *count,
            path = technical(folder)
        ),
    };
    let cancel = button::text(t!("common.cancel"))
        .variant(Variant::Ghost)
        .size(ButtonSize::Sm)
        .on_press(Message::Cancel);
    line(vec![
        turning().size(size::SM).into(),
        text(in_reading_order(&said))
            .size(size::SM)
            .style(theme::text::muted)
            .into(),
        cancel.into(),
    ])
    .spacing(space::S1_5)
    .into()
}

pub fn view(files: &Files, running: bool) -> Element<'_, Message> {
    if !running {
        return paragraph(t!("containers.files.startFirst"))
            .style(theme::text::muted)
            .into();
    }
    let entries = files.read.then_some(files.entries.as_slice());
    let upload = button::labelled(Icon::Upload, t!("containers.files.uploadHere"))
        .variant(Variant::Outline)
        .size(ButtonSize::Sm)
        .on_press_maybe((!files.is_busy()).then_some(Message::Upload));
    let mut actions: Vec<Element<'_, Message>> = vec![upload.into()];
    if let Some(running) = &files.transfer {
        actions.push(transfer(&running.doing));
    }
    FileBrowser::new(&files.browser, &files.path, entries, Message::Browser)
        .failed(files.failure.as_ref())
        .actions(actions)
        .view()
}

//! The Files tab: the kit's file browser, with "Upload here" and the
//! transfer that runs.

use iced::widget::text;

use crate::{
    i18n::format,
    t,
    ui::{
        features::docker::containers::detail::tabs::files::{Doing, Files, Message, Transfer},
        theme::{
            self,
            metrics::{space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::{
                button::{self, Variant},
                in_reading_order, progress, technical,
            },
            data::file_browser::FileBrowser,
            feedback::spinner::turning,
            line, paragraph,
        },
    },
};

/// What runs, said, how far it is, and what stops it.
fn transfer<'a>(running: &Transfer) -> Element<'a, Message> {
    #[allow(clippy::cast_precision_loss)]
    let size = |bytes: u64| format::bytes(bytes as f64);
    let mut parts: Vec<Element<'a, Message>> = vec![turning().size(size::SM).into()];
    let said = match &running.doing {
        Doing::Download { name, bytes, total } => {
            let came = match total {
                Some(total) => format!("{} / {}", size(*bytes), size(*total)),
                None => size(*bytes),
            };
            let said = t!("containers.files.downloading", name = technical(name));
            if let Some(total) = total.filter(|total| *total > 0) {
                #[allow(clippy::cast_precision_loss)]
                let percent = *bytes as f32 / total as f32 * 100.0;
                parts.push(progress(percent, None).length(96).into());
            }
            format!("{} {MIDDLE_DOT} {came}", said)
        }
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
    parts.push(
        text(in_reading_order(&said))
            .size(size::SM)
            .style(theme::text::muted)
            .into(),
    );
    // Pressed, it waits for the file being put to go in whole.
    let cancel = button::text(t!("common.cancel"))
        .variant(Variant::Ghost)
        .size(ButtonSize::Sm)
        .on_press_maybe((!running.stopping).then_some(Message::Cancel));
    parts.push(cancel.into());
    line(parts).spacing(space::S1_5).into()
}

/// What stands between a name and a figure (`·`): a sign, of no language.
const MIDDLE_DOT: char = '\u{b7}';

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
        actions.push(transfer(running));
    }
    FileBrowser::new(&files.browser, &files.path, entries, Message::Browser)
        .failed(files.failure.as_ref())
        .actions(actions)
        .view()
}

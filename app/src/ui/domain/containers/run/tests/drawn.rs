//! What the Run dialog looks like: filled in, in English and in Arabic,
//! and dark with its Advanced area open on a value it can't read.

use super::{bench, networks, send, shown, type_in};
use crate::{
    test_support::snapshot::assert_snapshot,
    ui::{
        domain::containers::run::{
            dialog::{Dialog, Message},
            form::{Line, Message as Field, Rows, Switch},
        },
        shell::page::testing::Bench,
        theme::Theme,
    },
};

/// The window the dialog is drawn over.
const SIZE: (f32, f32) = (828.0, 980.0);

/// A dialog with an image, a name, a port and a volume.
fn filled(bench: &Bench, host: &str) -> Dialog {
    let mut dialog = Dialog::run(host, "", "", false, &bench.ctx());
    type_in(&mut dialog, bench, Line::Image, "nginx:1.27");
    type_in(&mut dialog, bench, Line::Name, "web");
    send(&mut dialog, bench, Message::Form(Field::Add(Rows::Ports)));
    type_in(&mut dialog, bench, Line::HostPort(0), "8080");
    type_in(&mut dialog, bench, Line::ContainerPort(0), "80");
    send(&mut dialog, bench, Message::Form(Field::Add(Rows::Mounts)));
    type_in(&mut dialog, bench, Line::Source(0), "site");
    type_in(&mut dialog, bench, Line::Target(0), "/usr/share/nginx/html");
    dialog
}

#[test]
fn the_run_dialog_filled_in_reads_in_english_and_mirrors_in_arabic() {
    let (mut bench, _engine, host) = bench();
    networks(&mut bench, &host);
    for language in ["en", "ar"] {
        let dialog = filled(&bench, &host);
        let ctx = bench.ctx();
        let name = format!("container-run-{language}");
        assert_snapshot(&name, language, &Theme::Light, SIZE, || {
            shown(&dialog, &ctx)
        });
    }
}

#[test]
fn the_run_dialog_in_the_dark_says_what_it_cannot_read() {
    let (mut bench, _engine, host) = bench();
    networks(&mut bench, &host);
    let mut dialog = filled(&bench, &host);
    let open = Message::Form(Field::Switch(Switch::Advanced, true));
    send(&mut dialog, &bench, open);
    type_in(&mut dialog, &bench, Line::Cpus, "half");
    type_in(&mut dialog, &bench, Line::Memory, "512m");
    let ctx = bench.ctx();
    assert_snapshot(
        "container-run-dark",
        "en",
        &Theme::Dark,
        (828.0, 1300.0),
        || shown(&dialog, &ctx),
    );
}

/// Where the dialog's content lies in a window 828 wide: the dialog is
/// 672 wide, centred, and pads its content by 16 on each side.
const CONTENT: (f32, f32) = (78.0 + 16.0, 750.0 - 16.0);

#[test]
fn the_advanced_area_s_tabs_stay_inside_the_dialog_in_every_language() {
    use crate::ui::domain::containers::run::form::Section;
    let (mut bench, _engine, host) = bench();
    networks(&mut bench, &host);
    let mut dialog = filled(&bench, &host);
    let open = Message::Form(Field::Switch(Switch::Advanced, true));
    send(&mut dialog, &bench, open);
    for language in crate::i18n::LOCALES.iter().map(|locale| locale.tag) {
        let _language = crate::test_support::snapshot::in_language(language);
        let ctx = bench.ctx();
        let mut drawn = crate::test_support::window::window((828.0, 1400.0), shown(&dialog, &ctx));
        for section in Section::ALL {
            let label = section.label();
            let found = drawn
                .find(label.as_str())
                .unwrap_or_else(|_| panic!("{language}: {label} is drawn"));
            let bounds = found.bounds();
            assert!(
                bounds.x >= CONTENT.0 && bounds.x + bounds.width <= CONTENT.1,
                "{language}: {label} at {bounds:?} is outside the dialog"
            );
        }
    }
}

#[test]
fn the_advanced_area_mirrors_in_arabic_and_its_tabs_fit_in_german() {
    let (mut bench, _engine, host) = bench();
    networks(&mut bench, &host);
    let mut dialog = filled(&bench, &host);
    let open = Message::Form(Field::Switch(Switch::Advanced, true));
    send(&mut dialog, &bench, open);
    type_in(&mut dialog, &bench, Line::Cpus, "half");
    let ctx = bench.ctx();
    for language in ["ar", "de"] {
        let name = format!("container-run-advanced-{language}");
        assert_snapshot(&name, language, &Theme::Light, (828.0, 1300.0), || {
            shown(&dialog, &ctx)
        });
    }
}

#[test]
fn the_dialog_opens_on_its_first_field_in_the_order_the_language_reads() {
    use crate::test_support::{snapshot::in_language, stage::Stage};
    let (bench, _engine, host) = bench();
    for language in ["en", "ar"] {
        let _language = in_language(language);
        let dialog = Dialog::run(&host, "nginx", "", false, &bench.ctx());
        let ctx = bench.ctx();
        let mut stage = Stage::new(SIZE);
        stage.frame(shown(&dialog, &ctx), |frame| {
            assert!(frame.is_focused("nginx"), "{language}: the image first");
        });
    }
}

#[test]
fn the_fields_of_the_ports_and_the_volumes_line_up() {
    use crate::test_support::{snapshot::in_language, stage::Stage};
    let (bench, _engine, host) = bench();
    for language in ["en", "de", "ar"] {
        let _language = in_language(language);
        let dialog = filled(&bench, &host);
        let ctx = bench.ctx();
        let mut stage = Stage::new(SIZE);
        stage.frame(shown(&dialog, &ctx), |frame| {
            let (host_port, source) = (frame.find("8080"), frame.find("site"));
            let (port, target) = (frame.find("80"), frame.find("/usr/share/nginx/html"));
            let (Some(host_port), Some(source), Some(port), Some(target)) =
                (host_port, source, port, target)
            else {
                panic!("{language}: the rows are drawn");
            };
            // The fields' texts start at the same place and run as far.
            let ltr = language != "ar";
            let start = |b: iced::Rectangle| if ltr { b.x } else { b.x + b.width };
            assert!(
                (start(host_port) - start(source)).abs() < 1.0,
                "{language}: {host_port:?} {source:?}"
            );
            assert!(
                (start(port) - start(target)).abs() < 1.0,
                "{language}: {port:?} {target:?}"
            );
        });
    }
}

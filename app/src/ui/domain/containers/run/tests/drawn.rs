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

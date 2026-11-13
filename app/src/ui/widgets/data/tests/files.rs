//! The path over a folder (`fileBrowser.model.test.ts`, the same cases)
//! and what the browser asks of its page.

use iced::keyboard::key::Named;

use crate::{
    test_support::{snapshot::in_language, stage::Stage},
    ui::widgets::{
        basic::technical,
        data::file_browser::{crumbs, Event, Kind, Message, State},
        feedback::error_message::Event as Failure,
        gallery::{
            self,
            data::{self as sample, Data},
        },
        table,
    },
};

fn pairs(path: &str) -> Vec<(String, String)> {
    crumbs(path)
}

fn owned(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|(label, to)| ((*label).to_owned(), (*to).to_owned()))
        .collect()
}

/// `crumbs`: "splits a path into clickable parents".
#[test]
fn a_path_is_split_into_clickable_parents() {
    assert_eq!(pairs("/"), owned(&[("/", "/")]));
    assert_eq!(
        pairs("/usr/local/bin"),
        owned(&[
            ("/", "/"),
            ("usr", "/usr"),
            ("local", "/usr/local"),
            ("bin", "/usr/local/bin"),
        ])
    );
}

/// What the React model did with paths its test did not name: empty
/// parts are left out (`filter(Boolean)`), and a name is kept whole
/// whatever its script.
#[test]
fn a_path_written_loosely_gives_the_same_parents() {
    assert_eq!(pairs(""), owned(&[("/", "/")]));
    assert_eq!(
        pairs("//var///log/"),
        owned(&[("/", "/"), ("var", "/var"), ("log", "/var/log")])
    );
    assert_eq!(pairs("etc"), owned(&[("/", "/"), ("etc", "/etc")]));
    assert_eq!(
        pairs("/مجلد/ملف é.txt"),
        owned(&[
            ("/", "/"),
            ("مجلد", "/مجلد"),
            ("ملف é.txt", "/مجلد/ملف é.txt")
        ])
    );
}

/// A press on the row of `path`, drawn as the `index`-th.
fn pressed(path: &str, index: usize) -> Message {
    Message::Table(table::Message::Pressed {
        key: path.to_owned(),
        index,
        modifiers: iced::keyboard::Modifiers::default(),
    })
}

#[test]
fn a_folder_opens_and_a_file_does_not() {
    let entries = sample::listing("/").unwrap();
    let mut state = State::default();
    assert_eq!(
        state.update(pressed("/etc", 0), Some(&entries)),
        Some(Event::Navigate("/etc".to_owned()))
    );
    // `onRowClick`: only a directory; a link is not followed.
    assert_eq!(state.update(pressed("/hello.txt", 3), Some(&entries)), None);
    assert_eq!(state.update(pressed("/lib", 4), Some(&entries)), None);
    // A row that is no longer listed opens nothing.
    assert_eq!(state.update(pressed("/gone", 9), Some(&entries)), None);
    assert_eq!(state.update(pressed("/etc", 0), None), None);
}

#[test]
fn the_browser_asks_its_page_and_does_nothing_itself() {
    let entries = sample::listing("/").unwrap();
    let mut state = State::default();
    let entries = Some(entries.as_slice());
    assert_eq!(
        state.update(Message::Navigate("/usr".to_owned()), entries),
        Some(Event::Navigate("/usr".to_owned()))
    );
    assert_eq!(
        state.update(Message::Refresh, entries),
        Some(Event::Refresh)
    );
    assert_eq!(state.update(Message::Upload, entries), Some(Event::Upload));
    // A download names the entry whole: its kind and size are the page's to read.
    let Some(Event::Download(entry)) =
        state.update(Message::Download("/hello.txt".to_owned()), entries)
    else {
        panic!("no download asked for");
    };
    assert_eq!(
        (entry.name.as_str(), entry.kind, entry.size),
        ("hello.txt", Kind::File, 2048)
    );
    assert_eq!(
        state.update(Message::Download("/gone".to_owned()), entries),
        None
    );
    // Retry on a failure is a refresh; the rest is handed on.
    assert_eq!(
        state.update(Message::Failure(Failure::Retry), None),
        Some(Event::Refresh)
    );
    assert_eq!(
        state.update(Message::Failure(Failure::Details), None),
        Some(Event::Failure(Failure::Details))
    );
}

type Sent = gallery::Message;

/// Hands the gallery's page what the widgets sent. The dialogs a
/// message opens are `transfers.rs`'s to answer: here none is run.
fn feed(data: &mut Data, messages: Vec<Sent>) -> Vec<String> {
    let platform = crate::platform::Platform::default();
    let mut said = vec![];
    for message in messages {
        if let Sent::Data(event) = message {
            said.extend(data.update(event, &platform).1);
        }
    }
    said
}

/// On a window that lasts: a press on a folder opens it, a press on a
/// part of the path goes back, and the buttons ask for what they say.
#[test]
fn the_pointer_goes_into_a_folder_and_back_by_the_path() {
    let _english = in_language("en");
    let mut data = Data::default();
    let mut stage = Stage::new((900.0, 500.0));
    let messages = stage.frame(sample::files_view(&data), |frame| {
        frame.click(&technical("etc"))
    });
    feed(&mut data, messages);
    assert_eq!(data.path, "/etc");
    stage.frame(sample::files_view(&data), |frame| {
        assert!(
            frame.find(&technical("hosts")).is_some()
                && frame.find(&technical("hello.txt")).is_none()
        );
    });
    // The first part of the path is the root.
    let messages = stage.frame(sample::files_view(&data), |frame| {
        frame.click(&technical("/"))
    });
    feed(&mut data, messages);
    assert_eq!(data.path, "/");
    let messages = stage.frame(sample::files_view(&data), |frame| frame.click("Upload"));
    let asked =
        |message: &Sent| matches!(message, Sent::Data(sample::Event::Files(Message::Upload)));
    assert!(messages.iter().any(asked), "{messages:?}");
    // A folder that can't be read says so in place of its entries,
    // and Retry reads it again.
    let messages = stage.frame(sample::files_view(&data), |frame| {
        frame.click(&technical("root"))
    });
    feed(&mut data, messages);
    assert!(data.listing.is_err());
    let messages = stage.frame(sample::files_view(&data), |frame| {
        assert!(frame.find(&technical("hello.txt")).is_none());
        frame.click("Retry");
    });
    assert!(matches!(
        messages.as_slice(),
        [Sent::Data(sample::Event::Files(Message::Failure(
            Failure::Retry
        )))]
    ));
}

/// With the keyboard alone: Tab through the path and the buttons to
/// the rows, the arrows to an entry, Enter to open a folder.
#[test]
fn the_keyboard_reaches_the_path_the_buttons_and_the_rows() {
    let _english = in_language("en");
    let mut data = Data::default();
    let mut stage = Stage::new((900.0, 500.0));
    let messages = stage.frame(sample::files_view(&data), |frame| {
        frame.tab();
        assert!(frame.is_focused(&technical("/")));
        frame.tab();
        assert!(frame.is_focused("Refresh"));
        frame.tab();
        assert!(frame.is_focused("Upload"));
        // Past the four headers that sort, to the rows.
        for _ in 0..5 {
            frame.tab();
        }
        frame.tap_key(Named::ArrowDown);
        frame.tap_key(Named::Enter);
    });
    feed(&mut data, messages);
    assert_eq!(data.path, "/etc");
}

/// What shows down the first column, top to bottom.
fn names(
    frame: &mut crate::test_support::stage::Frame<'_, '_, Sent>,
    entries: &[&str],
) -> Vec<String> {
    let mut placed: Vec<(f32, String)> = entries
        .iter()
        .filter_map(|name| Some((frame.find(&technical(name))?.y, (*name).to_owned())))
        .collect();
    placed.sort_by(|a, b| a.0.total_cmp(&b.0));
    placed.into_iter().map(|(_, name)| name).collect()
}

/// Each header sorts by what its column holds: names as a person reads
/// them, sizes and dates by their value, modes as text.
#[test]
fn each_column_sorts_the_listing() {
    let _english = in_language("en");
    let mut data = Data::default();
    // Sizes and dates that tell the orders apart.
    if let Ok(entries) = &mut data.listing {
        entries.truncate(5);
        for (entry, modified) in entries.iter_mut().zip([50, 10, 40, 20, 30]) {
            entry.modified = modified;
        }
    }
    let all = ["etc", "root", "var", "hello.txt", "lib"];
    let mut stage = Stage::new((900.0, 500.0));
    let mut sorted = |data: &mut Data, header: &str| {
        let messages = stage.frame(sample::files_view(data), |frame| frame.click(header));
        feed(data, messages);
        let mut order = vec![];
        stage.frame(sample::files_view(data), |frame| order = names(frame, &all));
        order
    };
    // As the page gave them, until a header is pressed.
    let mut order = vec![];
    Stage::new((900.0, 500.0)).frame(sample::files_view(&data), |frame| {
        order = names(frame, &all)
    });
    assert_eq!(order, all);
    assert_eq!(
        sorted(&mut data, "Name"),
        ["etc", "hello.txt", "lib", "root", "var"]
    );
    // The largest first: sizes are numbers.
    assert_eq!(sorted(&mut data, "Size")[..2], ["etc", "root"]);
    assert_eq!(sorted(&mut data, "Size")[..2], ["lib", "hello.txt"]);
    // The newest first.
    assert_eq!(
        sorted(&mut data, "Modified"),
        ["etc", "var", "lib", "hello.txt", "root"]
    );
    assert_eq!(
        sorted(&mut data, "Mode"),
        ["hello.txt", "root", "etc", "var", "lib"]
    );
}

/// A folder's size is its entry's, which says nothing: only a file's
/// is shown.
#[test]
fn only_a_file_has_a_size() {
    let _english = in_language("en");
    let data = Data::default();
    let mut stage = Stage::new((900.0, 500.0));
    stage.frame(sample::files_view(&data), |frame| {
        let texts = frame.texts();
        assert!(texts.contains(&"2.0 KB".to_owned()), "{texts:?}");
        // etc is 4,096 bytes, the link 7, the socket 0.
        for hidden in ["4.0 KB", "7 B", "0 B"] {
            assert!(!texts.contains(&hidden.to_owned()), "{hidden} is shown");
        }
    });
}

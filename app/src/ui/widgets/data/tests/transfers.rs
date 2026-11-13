//! What the page of a file browser does of its Upload and Download: the
//! OS's dialog, asked of the platform, and what it answered. The gallery
//! stands for the page, on a system that only notes what it is asked.

use std::path::PathBuf;

use crate::{
    platform::{
        dialogs::{OpenFile, SaveFile},
        fake::{Asked, Fake},
    },
    test_support::{snapshot::in_language, tasks::messages},
    ui::widgets::{
        data::file_browser::Message,
        gallery::data::{save_request, Data, Event},
    },
};

/// Hands `event` to the page, then every message its tasks produce,
/// and gives what the page said on the way.
fn play(data: &mut Data, os: &std::sync::Arc<Fake>, event: Event) -> Vec<String> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let platform = os.platform();
    let (mut pending, mut said) = (vec![event], vec![]);
    while let Some(event) = pending.pop() {
        let (task, told) = data.update(event, &platform);
        said.extend(told);
        pending.extend(runtime.block_on(messages(task)));
    }
    said
}

#[test]
fn an_upload_asks_for_files_and_a_cancel_does_nothing() {
    let _english = in_language("en");
    let (mut data, os) = (Data::default(), Fake::new());
    // The dialog is closed without a file.
    let said = play(&mut data, &os, Event::Files(Message::Upload));
    assert!(said.is_empty(), "{said:?}");
    assert_eq!(os.took(), [Asked::Open(OpenFile::new("Upload to /"), true)]);

    // Two files are picked, in the folder the browser shows.
    play(
        &mut data,
        &os,
        Event::Files(Message::Navigate("/etc".to_owned())),
    );
    os.answers(vec!["/home/ada/a.conf".into(), "/home/ada/b.conf".into()]);
    let said = play(&mut data, &os, Event::Files(Message::Upload));
    assert_eq!(said, ["Would upload 2 files to /etc"]);
    assert_eq!(
        os.took(),
        [Asked::Open(OpenFile::new("Upload to /etc"), true)]
    );
}

#[test]
fn a_download_asks_where_to_save_under_the_entry_s_name() {
    let _english = in_language("en");
    let (mut data, os) = (Data::default(), Fake::new());
    let download = |path: &str| Event::Files(Message::Download(path.to_owned()));

    // A file keeps its name; cancelled, nothing is said or kept.
    let said = play(&mut data, &os, download("/hello.txt"));
    assert!(said.is_empty(), "{said:?}");
    assert!(data.downloading.is_none());
    assert_eq!(
        os.took(),
        [Asked::Save(SaveFile::new("Save hello.txt", "hello.txt"))]
    );

    // A folder comes as an archive.
    os.answer("/home/ada/etc.tar");
    let said = play(&mut data, &os, download("/etc"));
    assert_eq!(said, ["Would save /etc as /home/ada/etc.tar"]);
    assert_eq!(
        os.took(),
        [Asked::Save(SaveFile::new("Save etc", "etc.tar"))]
    );
    assert!(data.downloading.is_none());

    // An answer that comes when nothing was asked changes nothing.
    let said = play(&mut data, &os, Event::SaveTo(Some(PathBuf::from("/tmp/x"))));
    assert!(said.is_empty(), "{said:?}");
}

#[test]
fn a_name_from_a_container_is_made_safe_before_it_is_suggested() {
    let _english = in_language("en");
    let mut entries = Data::default().listing.unwrap();
    let mut entry = entries.remove(0);
    for (name, suggested) in [
        ("con", "_con.tar"),
        ("a:b*c", "a_b_c.tar"),
        ("logs. ", "logs.tar"),
    ] {
        entry.name = name.to_owned();
        let request = save_request(&entry);
        assert_eq!(request.name, suggested);
        // The title says the name as the container has it.
        assert_eq!(request.title, format!("Save {name}"));
    }
    let mut file = entries.remove(2);
    assert_eq!(file.name, "hello.txt");
    file.name = "NUL.txt".to_owned();
    assert_eq!(save_request(&file).name, "_NUL.txt");
}

//! The data widgets: a folder's listing, a JSON document, labelled
//! values, a copy button, shortcuts as each system names them, and a
//! search that waits for the typing to pause.

use std::path::PathBuf;

use homeport_core::error::AppResult;
use iced::{widget::text, Task};

use super::{failure, Gallery, Message};
use crate::{
    platform::{
        dialogs::{OpenFile, SaveFile},
        names, Platform,
    },
    t,
    ui::{
        theme::{
            metrics::{space, text as size},
            Element,
        },
        widgets::{
            basic::{self, shortcut},
            data::{
                file_browser::{self, Entry, FileBrowser, Kind},
                json, key_value, Item,
            },
            feedback::{copy_button, Copied, Surfaced},
            form::{search, Debounce},
            line, stack,
        },
    },
};

/// The shortcuts the gallery names, as the keymap writes them.
pub const SHORTCUTS: [&str; 5] = ["Mod+K", "Mod+Shift+P", "Alt+ArrowUp", "Enter", "/"];

/// A container's inspect, cut short: nested objects, an array, every
/// kind of value, a name that needs quotes, text in another script and
/// a string too long for its row.
pub const INSPECT: &str = r#"{
  "Id": "8f3a1c2b9d7e4f60a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718",
  "Name": "/shop-web-1",
  "State": { "Status": "running", "Running": true, "ExitCode": 0, "Error": null, "Pid": 4172 },
  "Config": {
    "Image": "ghcr.io/acme/shop-web:1.42.0",
    "Env": ["NODE_ENV=production", "PORT=8080", "GREETING=مرحبا بالعالم"],
    "Labels": {
      "com.docker.compose.project": "shop",
      "com.docker.compose.service": "web",
      "note": "a label long enough to be cut where its column ends: the quick brown fox jumps over the lazy dog, twice over, and then once more for good measure"
    },
    "Healthcheck": { "Test": ["CMD", "curl", "-f", "http://localhost:8080/health"], "Interval": 30000000000, "Retries": 3 }
  },
  "NetworkSettings": { "Ports": { "8080/tcp": [{ "HostIp": "0.0.0.0", "HostPort": "8080" }] }, "Networks": {} },
  "Mounts": [],
  "SizeRw": 12.5
}"#;

/// A document of `items` objects of five values each, for the tests
/// that hold a large one to staying quick.
pub fn large(items: usize) -> String {
    let rows: Vec<String> = (0..items)
        .map(|n| {
            format!(
                r#"{{"id":{n},"name":"container-{n}","running":{},"ports":[80,{}],"note":null}}"#,
                n % 2 == 0,
                8000 + n % 1000
            )
        })
        .collect();
    format!("[{}]", rows.join(","))
}

/// What the gallery's folders hold. `/root` can't be read.
pub fn listing(path: &str) -> Result<Vec<Entry>, homeport_core::error::AppError> {
    let entry = |name: &str, kind, size, mode: &str| Entry {
        name: name.to_owned(),
        path: format!("{}/{name}", path.trim_end_matches('/')),
        kind,
        size,
        // 2026-05-01 10:00:00 UTC, as the chart's samples.
        modified: 1_777_629_600,
        mode: mode.to_owned(),
    };
    match path {
        "/" => Ok(vec![
            entry("etc", Kind::Directory, 4096, "drwxr-xr-x"),
            entry("root", Kind::Directory, 4096, "drwx------"),
            entry("var", Kind::Directory, 4096, "drwxr-xr-x"),
            entry("hello.txt", Kind::File, 2048, "-rw-r--r--"),
            entry("lib", Kind::Symlink, 7, "lrwxrwxrwx"),
            entry(
                "a-backup-with-a-name-far-longer-than-its-column-2026-05-01T10-00-00Z.tar.gz",
                Kind::File,
                734_003_200,
                "-rw-------",
            ),
            entry("تقرير.txt", Kind::File, 512, "-rw-r--r--"),
            entry("docker.sock", Kind::Other, 0, "srw-rw----"),
        ]),
        "/etc" => Ok(vec![
            entry("nginx", Kind::Directory, 4096, "drwxr-xr-x"),
            entry("hosts", Kind::File, 174, "-rw-r--r--"),
            entry("resolv.conf", Kind::File, 38, "-rw-r--r--"),
        ]),
        "/root" => Err(failure()),
        _ => Ok(vec![]),
    }
}

/// What the gallery says of an event: in passing, or as a failure.
pub type Said = AppResult<String>;

pub struct Data {
    pub files: file_browser::State,
    pub path: String,
    pub listing: Result<Vec<Entry>, homeport_core::error::AppError>,
    /// The entry the save dialog is open for.
    pub downloading: Option<Entry>,
    pub json: json::State,
    pub search: Debounce,
    /// What the search last ran for, once the typing paused.
    pub searched: String,
    pub copied: Copied,
}

#[derive(Debug, Clone)]
pub enum Event {
    Files(file_browser::Message),
    /// The save dialog of a download answered; `None` when cancelled.
    SaveTo(AppResult<Option<PathBuf>>),
    /// The open dialog of an upload answered; no file when cancelled.
    UploadFrom(AppResult<Vec<PathBuf>>),
    Json(json::Message),
    Typed(String),
    Paused(u64),
    Copy,
    CopyShown(u64),
}

impl Default for Data {
    fn default() -> Self {
        Self {
            files: file_browser::State::default(),
            path: "/".to_owned(),
            listing: listing("/"),
            downloading: None,
            json: json::State::new(json::Document::parse(INSPECT).expect("the sample is JSON")),
            search: Debounce::default(),
            searched: String::new(),
            copied: Copied::default(),
        }
    }
}

impl Data {
    /// What the gallery's page does with a data widget's message, and
    /// what it says of it in passing, or the failure it says. The dialogs
    /// are asked of `platform`, as a page asks them of `ctx.platform`.
    pub fn update(&mut self, event: Event, platform: &Platform) -> (Task<Event>, Option<Said>) {
        match event {
            Event::Files(message) => {
                let entries = self.listing.as_deref().ok();
                match self.files.update(message, entries) {
                    Some(file_browser::Event::Navigate(path)) => {
                        self.listing = listing(&path);
                        self.path = path;
                    }
                    Some(file_browser::Event::Refresh) => {
                        self.listing = listing(&self.path);
                    }
                    // A dialog is open for an entry already: it is the
                    // one the answer will be about.
                    Some(file_browser::Event::Download(_)) if self.downloading.is_some() => {}
                    Some(file_browser::Event::Download(entry)) => {
                        let asked = platform.save_file(save_request(&entry));
                        self.downloading = Some(entry);
                        return (asked.map(Event::SaveTo), None);
                    }
                    Some(file_browser::Event::Upload) => {
                        let title = t!("containers.files.uploadTo", path = self.path.as_str());
                        let asked = platform.pick_files(OpenFile::new(title));
                        return (asked.map(Event::UploadFrom), None);
                    }
                    Some(file_browser::Event::Failure(_)) | None => {}
                }
                (Task::none(), None)
            }
            // Where the file goes is known: moving it out of the
            // container is the Engine's transfer (#258), which a page
            // starts here with `ctx.act`. The gallery has no Engine and
            // says what it would ask.
            Event::SaveTo(target) => {
                let entry = self.downloading.take();
                let said = match target {
                    Ok(target) => entry.zip(target).map(|(entry, target)| {
                        Ok(format!("Would save {} as {}", entry.path, target.display()))
                    }),
                    // No dialog could be shown: a failure, not a cancel.
                    Err(error) => Some(Err(error)),
                };
                (Task::none(), said)
            }
            // No file picked is a cancel: nothing is said.
            Event::UploadFrom(Ok(sources)) => {
                let said = (!sources.is_empty()).then(|| {
                    Ok(format!(
                        "Would upload {} files to {}",
                        sources.len(),
                        self.path
                    ))
                });
                (Task::none(), said)
            }
            Event::UploadFrom(Err(error)) => (Task::none(), Some(Err(error))),
            Event::Json(message) => {
                let (task, event) = self.json.update(message);
                let copy = match event {
                    Some(json::Event::Copy(text)) => iced::clipboard::write(text),
                    None => Task::none(),
                };
                (Task::batch([task.map(Event::Json), copy]), None)
            }
            Event::Typed(text) => (self.search.input(text).map(Event::Paused), None),
            Event::Paused(ticket) => {
                if let Some(query) = self.search.settled(ticket) {
                    self.searched = query.to_owned();
                }
                (Task::none(), None)
            }
            Event::Copy => {
                let shown = self.copied.start(()).map(Event::CopyShown);
                let write = iced::clipboard::write("docker pull nginx".to_owned());
                (Task::batch([shown, write]), None)
            }
            Event::CopyShown(ticket) => {
                self.copied.finish(ticket);
                (Task::none(), None)
            }
        }
    }
}

/// The save dialog of an entry's download, as the container's Files tab
/// asked it: the entry's name, or `name.tar` for a folder, which comes
/// as an archive. The name is a container's: it is made safe to suggest.
pub fn save_request(entry: &Entry) -> SaveFile {
    let name = match entry.kind {
        Kind::Directory => names::with_extension(&entry.name, "tar", ""),
        _ => names::suggested(&entry.name, ""),
    };
    let title = t!("containers.files.saveOne", name = entry.name.as_str());
    SaveFile::new(title, &name)
}

fn send(event: Event) -> Message {
    Message::Data(event)
}

/// The gallery's file browser, as a page would build it.
pub fn files_view(data: &Data) -> Element<'_, Message> {
    FileBrowser::new(
        &data.files,
        &data.path,
        data.listing.as_deref().ok(),
        |message| send(Event::Files(message)),
    )
    .failed(data.listing.as_ref().err())
    .uploads(true)
    .view()
}

pub fn json_view(data: &Data) -> Element<'_, Message> {
    json::View::new(&data.json, |message| send(Event::Json(message)))
        .max_height(320.0)
        .view()
}

pub fn facts<'a>() -> Element<'a, Message> {
    key_value(vec![
        Item::text("Image", "ghcr.io/acme/shop-web:1.42.0"),
        Item::mono(
            "Digest",
            "sha256:8f3a1c2b9d7e4f60a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90",
        ),
        Item::new("Restart policy", basic::badge("unless-stopped", basic::BadgeVariant::Secondary)),
        Item::maybe("Health", None),
        Item::text("الوصف", "حاوية تجريبية تعمل على المنفذ 8080"),
    ])
}

/// Each shortcut as this computer names it, then as a Mac and as any
/// other would.
pub fn shortcuts<'a>() -> Element<'a, Message> {
    let named = |mac: bool| -> Element<'a, Message> {
        line(
            SHORTCUTS
                .iter()
                .map(|spec| basic::keys(&shortcut::format(spec, mac)))
                .collect(),
        )
        .spacing(space::S4)
        .into()
    };
    stack(vec![
        line(SHORTCUTS.iter().map(|spec| basic::shortcut(spec)).collect())
            .spacing(space::S4)
            .into(),
        named(true),
        named(false),
    ])
    .spacing(space::S2)
    .into()
}

pub fn small<'a>(data: &'a Data) -> Element<'a, Message> {
    let field = search("gallery-debounced", "Search", data.search.text(), |text| {
        send(Event::Typed(text))
    });
    let searched = text(format!("Searched for: {:?}", data.searched)).size(size::SM);
    line(vec![
        copy_button(None, data.copied.is(&()), send(Event::Copy)),
        field,
        searched.into(),
    ])
    .spacing(space::S4)
    .into()
}

/// A page's header with a description and two actions, the mark beside
/// the name, the mark on its tile, and GitHub's.
pub fn layout<'a>() -> Element<'a, Message> {
    use crate::ui::{
        theme::{self, Icon},
        widgets::{
            basic::button::{self, Variant},
            layout::{logo, page_header},
        },
    };
    let header = page_header(
        "Containers",
        Some("3 running of 5".to_owned()),
        vec![
            button::labelled(Icon::RefreshCw, "Check for newer images")
                .variant(Variant::Outline)
                .on_press(Message::Nothing)
                .into(),
            button::labelled(Icon::Plus, "Run container")
                .on_press(Message::Nothing)
                .into(),
        ],
    );
    let marks = line(vec![
        logo::logo(),
        logo::mark(32.0),
        theme::icon(Icon::Github).into(),
    ])
    .spacing(space::S6);
    stack(vec![header, marks.into()]).spacing(space::S4).into()
}

impl Gallery {
    pub(super) fn data_event(&mut self, event: Event) -> Task<Message> {
        let (task, said) = self.data.update(event, &self.platform);
        match said {
            Some(Ok(said)) => {
                self.toasts
                    .say(super::toast::Kind::Info, move || said.clone());
            }
            Some(Err(error)) => {
                self.toasts.failure(Surfaced::new(error, "save_file"));
            }
            None => {}
        }
        task.map(Message::Data)
    }

    pub(super) fn data(&self) -> Element<'_, Message> {
        stack(vec![
            Self::section("File browser", files_view(&self.data)),
            Self::section("JSON view", json_view(&self.data)),
            Self::section("Key and value", facts()),
            Self::section("Copy button and a search that waits", small(&self.data)),
            Self::section("Shortcuts: here, on a Mac, elsewhere", shortcuts()),
            Self::section("Page header and logos", layout()),
        ])
        .spacing(space::S6)
        .into()
    }
}

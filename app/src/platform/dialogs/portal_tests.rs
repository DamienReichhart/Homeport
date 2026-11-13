//! The portal's dialogs against a portal that is one: a bus of the
//! test's own (`dbus-daemon`, as every Linux desktop and CI image has),
//! and on it a `org.freedesktop.portal.Desktop` that answers as the
//! desktop's would. No dialog shows, and the session's bus is not used.

use std::{
    collections::HashMap,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use zbus::{
    fdo, interface,
    message::Header,
    zvariant::{OwnedObjectPath, OwnedValue, Value},
    Connection,
};

use super::{
    portal::{ask, folder_path, over, Ask},
    Filter, OpenFile, OpenFolder, SaveFile,
};

/// A session bus that lasts as long as a test.
pub(super) struct Bus {
    daemon: Child,
    address: String,
    _dir: tempfile::TempDir,
}

impl Bus {
    pub(super) fn start() -> Self {
        let dir = tempfile::tempdir().expect("a temporary folder");
        let socket = dir.path().join("bus");
        let address = format!("unix:path={}", socket.display());
        // A bus with nothing to start on demand: whatever portal this
        // machine has installed stays out of the tests.
        let config = dir.path().join("bus.conf");
        let policy = r#"<allow send_destination="*" eavesdrop="true"/><allow eavesdrop="true"/><allow own="*"/>"#;
        std::fs::write(
            &config,
            format!(
                "<busconfig><type>session</type><listen>{address}</listen><auth>EXTERNAL</auth>\
                 <policy context=\"default\">{policy}</policy></busconfig>"
            ),
        )
        .expect("the bus's configuration");
        let daemon = Command::new("dbus-daemon")
            .args(["--nofork", "--nopidfile"])
            .arg(format!("--config-file={}", config.display()))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("`dbus-daemon`, which the tests of the Linux dialogs run a bus with");
        let started = Instant::now();
        while !socket.exists() {
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "the bus never started"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
        Self {
            daemon,
            address,
            _dir: dir,
        }
    }

    pub(super) async fn connect(&self) -> Connection {
        zbus::connection::Builder::address(self.address.as_str())
            .expect("an address")
            // As the app's, but for a test: see `portal::CALL_TIMEOUT`.
            .method_timeout(Duration::from_secs(2))
            .build()
            .await
            .expect("a connection to the test's bus")
    }
}

impl Drop for Bus {
    fn drop(&mut self) {
        let _ = self.daemon.kill();
        let _ = self.daemon.wait();
    }
}

/// What the portal does of a request.
#[derive(Debug, Clone)]
pub(super) enum Answer {
    Picked(Vec<&'static str>),
    Cancelled,
    /// The portal's "something else went wrong" (response 2).
    Failed,
    /// It takes the request and never answers.
    Silent,
}

/// What the portal was asked.
#[derive(Debug)]
pub(super) struct Asked {
    pub(super) method: &'static str,
    pub(super) parent: String,
    pub(super) title: String,
    pub(super) options: HashMap<String, OwnedValue>,
}

impl Asked {
    fn flag(&self, name: &str) -> Option<bool> {
        self.options
            .get(name)
            .and_then(|value| bool::try_from(value).ok())
    }

    fn text(&self, name: &str) -> Option<String> {
        self.options
            .get(name)
            .and_then(|value| <&str>::try_from(value).ok().map(str::to_owned))
    }

    /// The filters as the portal reads them: a name, and its globs.
    fn filters(&self) -> Vec<(String, Vec<String>)> {
        type Wire = Vec<(String, Vec<(u32, String)>)>;
        let Some(value) = self.options.get("filters") else {
            return vec![];
        };
        let filters = Wire::try_from(value.try_clone().expect("a copy")).expect("a(sa(us))");
        filters
            .into_iter()
            .map(|(name, globs)| {
                // 0 is a glob, 1 a MIME type: the app only sends globs.
                assert!(globs.iter().all(|(kind, _)| *kind == 0));
                (name, globs.into_iter().map(|(_, glob)| glob).collect())
            })
            .collect()
    }

    fn folder(&self) -> Option<Vec<u8>> {
        let value = self.options.get("current_folder")?;
        Vec::<u8>::try_from(value.try_clone().expect("a copy")).ok()
    }
}

/// `org.freedesktop.portal.FileChooser`, answering what it is told to.
pub(super) struct Chooser {
    pub(super) answer: Answer,
    pub(super) asked: Arc<Mutex<Vec<Asked>>>,
}

impl Chooser {
    async fn take(
        &self,
        method: &'static str,
        (parent, title, options): (&str, &str, HashMap<String, OwnedValue>),
        header: &Header<'_>,
        connection: &Connection,
    ) -> fdo::Result<OwnedObjectPath> {
        let sender = header.sender().expect("a sender").to_owned();
        let token = options
            .get("handle_token")
            .and_then(|value| <&str>::try_from(value).ok())
            .expect("a handle token")
            .to_owned();
        let handle = format!(
            "/org/freedesktop/portal/desktop/request/{}/{token}",
            sender.trim_start_matches(':').replace('.', "_")
        );
        self.asked.lock().unwrap().push(Asked {
            method,
            parent: parent.to_owned(),
            title: title.to_owned(),
            options,
        });
        let mut results: HashMap<&str, Value<'_>> = HashMap::new();
        let code: u32 = match &self.answer {
            Answer::Picked(uris) => {
                let uris: Vec<String> = uris.iter().map(|uri| (*uri).to_owned()).collect();
                results.insert("uris", Value::from(uris));
                0
            }
            Answer::Cancelled => 1,
            Answer::Failed => 2,
            Answer::Silent => return Ok(handle.as_str().try_into().expect("a path")),
        };
        connection
            .emit_signal(
                Some(sender),
                handle.as_str(),
                "org.freedesktop.portal.Request",
                "Response",
                &(code, results),
            )
            .await?;
        Ok(handle.as_str().try_into().expect("a path"))
    }
}

#[interface(name = "org.freedesktop.portal.FileChooser")]
impl Chooser {
    async fn open_file(
        &self,
        parent: &str,
        title: &str,
        options: HashMap<String, OwnedValue>,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] connection: &Connection,
    ) -> fdo::Result<OwnedObjectPath> {
        self.take("OpenFile", (parent, title, options), &header, connection)
            .await
    }

    async fn save_file(
        &self,
        parent: &str,
        title: &str,
        options: HashMap<String, OwnedValue>,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] connection: &Connection,
    ) -> fdo::Result<OwnedObjectPath> {
        self.take("SaveFile", (parent, title, options), &header, connection)
            .await
    }

    #[zbus(property, name = "version")]
    fn version(&self) -> u32 {
        4
    }
}

/// A desktop whose portal answers `answer`: its bus, the portal's
/// connection (dropping it is the portal dying), what it was asked, and
/// the app's own connection.
pub(super) struct Desktop {
    pub(super) portal: Option<Connection>,
    pub(super) asked: Arc<Mutex<Vec<Asked>>>,
    pub(super) app: Connection,
    pub(super) bus: Bus,
}

pub(super) async fn desktop(answer: Answer) -> Desktop {
    let bus = Bus::start();
    let asked = Arc::new(Mutex::new(vec![]));
    let chooser = Chooser {
        answer,
        asked: asked.clone(),
    };
    let portal = zbus::connection::Builder::address(bus.address.as_str())
        .expect("an address")
        .name("org.freedesktop.portal.Desktop")
        .expect("a name")
        .serve_at("/org/freedesktop/portal/desktop", chooser)
        .expect("an object")
        .build()
        .await
        .expect("the portal on the test's bus");
    let app = bus.connect().await;
    Desktop {
        portal: Some(portal),
        asked,
        app,
        bus,
    }
}

fn paths(paths: &[&str]) -> Vec<PathBuf> {
    paths.iter().map(PathBuf::from).collect()
}

#[tokio::test]
async fn the_portal_is_asked_as_requested_and_its_files_are_paths() {
    let desktop = desktop(Answer::Picked(vec![
        "file:///srv/shop/compose.yml",
        "file:///srv/shop/My%20extra.yml",
        "https://example.com/not-a-file",
    ]))
    .await;
    let request = OpenFile::new("Choose compose files")
        .filter(Filter::new("Compose files", &["yml", "yaml"]))
        .filter(Filter::new("All files", &["*"]))
        .starting_in("/srv/shop");
    let picked = ask(desktop.app.clone(), None, Ask::Open(request, true))
        .await
        .unwrap();
    // What names no file on this computer is left out.
    assert_eq!(
        picked,
        paths(&["/srv/shop/compose.yml", "/srv/shop/My extra.yml"])
    );

    let asked = desktop.asked.lock().unwrap();
    let [asked] = asked.as_slice() else {
        panic!("one request: {asked:?}");
    };
    assert_eq!(
        (asked.method, asked.title.as_str()),
        ("OpenFile", "Choose compose files")
    );
    assert_eq!(asked.parent, "", "no window was named");
    assert_eq!(asked.flag("multiple"), Some(true));
    assert_eq!(asked.flag("modal"), Some(true));
    assert_eq!(asked.flag("directory"), None);
    assert_eq!(
        asked.filters(),
        [
            (
                "Compose files".to_owned(),
                vec!["*.yml".to_owned(), "*.yaml".to_owned()]
            ),
            ("All files".to_owned(), vec!["*".to_owned()]),
        ]
    );
    assert_eq!(asked.folder().as_deref(), Some(b"/srv/shop\0".as_slice()));
}

#[tokio::test]
async fn a_folder_and_a_place_to_save_are_asked_their_own_way() {
    let desktop = desktop(Answer::Picked(vec!["file:///home/ada/out"])).await;
    let picked = ask(
        desktop.app.clone(),
        None,
        Ask::Folder(OpenFolder::new("Pick a folder")),
    )
    .await;
    assert_eq!(picked.unwrap(), paths(&["/home/ada/out"]));
    let save = SaveFile::new("Save logs", "logs.txt").filter(Filter::new("Text", &["txt"]));
    let picked = ask(desktop.app.clone(), None, Ask::Save(save)).await;
    assert_eq!(picked.unwrap(), paths(&["/home/ada/out"]));

    let asked = desktop.asked.lock().unwrap();
    let [folder, save] = asked.as_slice() else {
        panic!("two requests: {asked:?}");
    };
    assert_eq!(
        (folder.method, folder.flag("directory")),
        ("OpenFile", Some(true))
    );
    assert_eq!(folder.flag("multiple"), None);
    assert_eq!(folder.folder(), None);
    assert_eq!(
        (save.method, save.title.as_str()),
        ("SaveFile", "Save logs")
    );
    assert_eq!(save.text("current_name").as_deref(), Some("logs.txt"));
    assert_eq!(
        save.filters(),
        [("Text".to_owned(), vec!["*.txt".to_owned()])]
    );
}

#[tokio::test]
async fn a_dialog_the_user_closes_answers_no_file_and_is_no_failure() {
    let desktop = desktop(Answer::Cancelled).await;
    let picked = ask(
        desktop.app.clone(),
        None,
        Ask::Open(OpenFile::new("Open"), false),
    )
    .await;
    assert_eq!(picked.unwrap(), Vec::<PathBuf>::new());
    let saved = ask(
        desktop.app.clone(),
        None,
        Ask::Save(SaveFile::new("Save", "a.txt")),
    )
    .await;
    assert_eq!(saved.unwrap(), Vec::<PathBuf>::new());
    assert_eq!(
        desktop.asked.lock().unwrap().len(),
        2,
        "the portal was asked both times"
    );
}

#[test]
fn a_starting_folder_is_handed_over_as_its_bytes() {
    use std::path::Path;
    let folder = folder_path(Path::new("/srv/My stacks/日本")).expect("a path");
    assert_eq!(folder.as_ref(), Path::new("/srv/My stacks/日本"));
    // A path the portal's wire can't carry is left out, not cut.
    assert!(folder_path(Path::new("/srv/a\0b")).is_none());
}

/// A dialog that must end by itself: a test that waits is a test that failed.
async fn soon<T>(asked: impl std::future::Future<Output = T>) -> T {
    tokio::time::timeout(Duration::from_secs(20), asked)
        .await
        .expect("the dialog never ended")
}

fn open() -> Ask {
    Ask::Open(OpenFile::new("Open"), false)
}

fn unavailable(failed: &homeport_core::error::AppError) {
    assert_eq!(failed.key, "errors.file.dialogUnavailable");
    assert_eq!(failed.message, "Homeport couldn't open a file dialog.");
    let hint = failed.hint.as_ref().expect("what to install");
    assert!(hint.text.contains("xdg-desktop-portal"), "{}", hint.text);
    assert!(
        failed.detail.is_some(),
        "what the bus said is kept for the Details"
    );
}

#[tokio::test]
async fn a_desktop_with_no_portal_is_a_failure_and_not_a_cancel() {
    // A bus, and nothing on it that answers for the portal.
    let bus = Bus::start();
    let failed = soon(ask(bus.connect().await, None, open()))
        .await
        .unwrap_err();
    unavailable(&failed);
    let detail = failed.detail.unwrap();
    assert!(detail.contains("ServiceUnknown"), "{detail}");
}

/// Something on the portal's object that is no file chooser.
struct Settings;

#[interface(name = "org.freedesktop.portal.Settings")]
impl Settings {
    #[zbus(property, name = "version")]
    fn version(&self) -> u32 {
        2
    }
}

#[tokio::test]
async fn a_portal_without_a_file_chooser_is_a_failure() {
    // The portal is installed, and no backend gives it a file chooser.
    let bus = Bus::start();
    let _portal = zbus::connection::Builder::address(bus.address.as_str())
        .unwrap()
        .name("org.freedesktop.portal.Desktop")
        .unwrap()
        .serve_at("/org/freedesktop/portal/desktop", Settings)
        .unwrap()
        .build()
        .await
        .unwrap();
    let failed = soon(ask(bus.connect().await, None, open()))
        .await
        .unwrap_err();
    unavailable(&failed);
}

#[tokio::test]
async fn a_portal_that_never_takes_the_request_is_given_up_on() {
    // Something owns the portal's name and answers nothing at all.
    let bus = Bus::start();
    let _mute = zbus::connection::Builder::address(bus.address.as_str())
        .unwrap()
        .name("org.freedesktop.portal.Desktop")
        .unwrap()
        .build()
        .await
        .unwrap();
    let failed = soon(ask(bus.connect().await, None, open()))
        .await
        .unwrap_err();
    unavailable(&failed);
}

#[tokio::test]
async fn a_session_with_no_bus_is_a_failure() {
    let nowhere = tempfile::tempdir().unwrap();
    let address = format!("unix:path={}", nowhere.path().join("no-bus").display());
    let connect = async {
        zbus::connection::Builder::address(address.as_str())?
            .build()
            .await
    };
    let failed = soon(over(connect, None, open())).await.unwrap_err();
    unavailable(&failed);
}

#[tokio::test]
async fn a_portal_that_says_it_failed_is_a_failure() {
    let desktop = desktop(Answer::Failed).await;
    let failed = soon(ask(desktop.app.clone(), None, open()))
        .await
        .unwrap_err();
    unavailable(&failed);
}

#[tokio::test]
async fn a_portal_that_dies_with_its_dialog_open_ends_the_dialog_with_a_failure() {
    let mut desktop = desktop(Answer::Silent).await;
    let asked = tokio::spawn(ask(desktop.app.clone(), None, open()));
    // The dialog is open: the portal took the request and says nothing.
    let started = Instant::now();
    while desktop.asked.lock().unwrap().is_empty() {
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "the portal was never asked"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(
        !asked.is_finished(),
        "a portal that is thinking is waited for"
    );
    // It crashes: its name leaves the bus.
    drop(desktop.portal.take());
    let failed = soon(asked).await.unwrap().unwrap_err();
    unavailable(&failed);
}

#[tokio::test]
async fn a_bus_that_goes_away_with_a_dialog_open_ends_the_dialog_with_a_failure() {
    let desktop = desktop(Answer::Silent).await;
    let asked = tokio::spawn(ask(desktop.app.clone(), None, open()));
    let started = Instant::now();
    while desktop.asked.lock().unwrap().is_empty() {
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "the portal was never asked"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    // The session ends under the app.
    let Desktop { bus, portal, .. } = desktop;
    drop(bus);
    let failed = soon(asked).await.unwrap().unwrap_err();
    unavailable(&failed);
    drop(portal);
}

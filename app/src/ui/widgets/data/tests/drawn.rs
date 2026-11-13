//! The data widgets drawn: in both themes and under Arabic chrome.

use iced::{widget::container, Length};

use crate::{
    test_support::{snapshot::assert_snapshot, stage::Stage, tasks::messages},
    ui::{
        theme::{self, Element, Theme},
        widgets::{
            data::json,
            gallery::{
                self,
                data::{self as sample, Data, Event},
                Gallery, Page,
            },
        },
    },
};

type Sent = gallery::Message;

const LOOKS: [(&str, &str, Theme); 3] = [
    ("en", "en", Theme::Light),
    ("dark-en", "en", Theme::Dark),
    ("ar", "ar", Theme::Light),
];

fn on_page(part: Element<'_, Sent>) -> Element<'_, Sent> {
    container(part)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::page)
        .into()
}

#[test]
fn the_gallery_s_page_of_data_widgets() {
    // The labels are measured when the view is built: with the app's
    // fonts, which a window has loaded before its first view.
    drop(Stage::new((10.0, 10.0)));
    let gallery = Gallery {
        page: Page::Data,
        ..Gallery::default()
    };
    for (look, language, theme) in LOOKS {
        let name = format!("widgets-data-{look}");
        assert_snapshot(&name, language, &theme, (1100.0, 1500.0), || gallery.view());
    }
}

#[test]
fn a_folder_listed_read_and_refused() {
    let listed = Data::default();
    for (look, language, theme) in LOOKS {
        let name = format!("files-listed-{look}");
        assert_snapshot(&name, language, &theme, (900.0, 420.0), || {
            on_page(sample::files_view(&listed))
        });
    }
    // While the folder is read; a folder with nothing in it; one that
    // can't be read.
    let empty = Data {
        path: "/var/lib/docker".to_owned(),
        listing: Ok(vec![]),
        ..Data::default()
    };
    assert_snapshot(
        "files-empty-en",
        "en",
        &Theme::Light,
        (900.0, 240.0),
        || on_page(sample::files_view(&empty)),
    );
    let refused = Data {
        path: "/root".to_owned(),
        listing: sample::listing("/root"),
        ..Data::default()
    };
    assert_snapshot(
        "files-failed-en",
        "en",
        &Theme::Light,
        (900.0, 300.0),
        || on_page(sample::files_view(&refused)),
    );
}

/// A search that found something: the match shows, marked, and the
/// count says where the view is among them.
#[tokio::test(start_paused = true)]
async fn a_document_with_a_search_on_it() {
    let mut data = Data::default();
    let mut pending = vec![Event::Json(json::Message::Typed("RUNNING".to_owned()))];
    while let Some(event) = pending.pop() {
        let (task, _) = data.update(event, &crate::platform::Platform::default());
        pending.extend(messages(task).await);
    }
    // The state's status, whatever the case, and the member named Running.
    assert_eq!(data.json.matches(), (Some(0), 2));
    for (look, language, theme) in LOOKS {
        let name = format!("json-search-{look}");
        assert_snapshot(&name, language, &theme, (900.0, 460.0), || {
            on_page(sample::json_view(&data))
        });
    }
}

#[test]
fn labelled_values_and_shortcuts() {
    // The labels are measured when the view is built: with the app's
    // fonts, which a window has loaded before its first view.
    drop(Stage::new((10.0, 10.0)));
    for (look, language, theme) in LOOKS {
        let name = format!("key-value-{look}");
        assert_snapshot(&name, language, &theme, (560.0, 260.0), || {
            on_page(sample::facts())
        });
    }
    assert_snapshot("keys-en", "en", &Theme::Light, (560.0, 150.0), || {
        on_page(sample::shortcuts())
    });
}

/// `PageHeader`, `Logo`, `LogoMark` and `GitHubIcon`, which the shell
/// uses from the kit.
#[test]
fn a_page_s_header_and_the_logos() {
    for (look, language, theme) in LOOKS {
        let name = format!("layout-{look}");
        assert_snapshot(&name, language, &theme, (700.0, 170.0), || {
            on_page(sample::layout())
        });
    }
}

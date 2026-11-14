//! What the page reads and what each message changes.

use homeport_core::error::{AppError, ErrorCode};

use super::*;
use crate::{
    data::resources,
    t,
    test_support::{fixtures::podman_host, snapshot::in_language, tasks},
    ui::{
        domain::{
            containers::{actions, Bulk},
            freshness,
        },
        shell::{page::testing::Asked, registry, router::Request},
        widgets::{feedback::error_message::Event, table},
    },
};

#[test]
fn opening_reads_the_host_s_containers_and_what_marks_them() {
    let mut bench = Bench::on_host(Route::Containers, connected_host());
    let _ = open(&bench.ctx());
    assert_eq!(
        bench.watched(),
        [keys::host::containers("h1"), keys::host::freshness("h1")]
    );
    // Podman says which pod a container is in.
    bench = Bench::on_host(Route::Containers, podman_host());
    let _ = open(&bench.ctx());
    assert_eq!(bench.watched().last(), Some(&keys::host::pods("h1")));
    // Global search opens the list searched.
    bench
        .history
        .push(Request::to(Route::Containers).searching("redis"));
    let (state, _) = open(&bench.ctx());
    assert_eq!(state.query, "redis");
    let _ = resources::pods("h1");
}

#[test]
fn stack_containers_are_nested_under_their_stack_by_name() {
    let _english = in_language("en");
    let rig = Rig::populated();
    // The Stacks by name, then the containers of none; under a Stack,
    // its containers by name, called by their service.
    let order = ["blog", "app", "shop", "db", "web", "backup-job", "redis"];
    let tops: Vec<f32> = order.iter().map(|label| rig.top(label)).collect();
    assert!(tops.windows(2).all(|pair| pair[0] < pair[1]), "{tops:?}");
    assert!(rig.shows("1/2 running") && rig.shows("1/1 running"));
    assert!(rig.shows(&t!("containers.summary", running = 3, count = 5)));
    assert!(!rig.shows("shop-web-1"), "a nested row shows its service");
}

#[test]
fn a_stack_closes_and_the_search_and_the_switch_filter() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.frame(|frame| frame.click("shop"));
    assert!(!rig.shows("web") && !rig.shows("db"), "the Stack is closed");
    assert!(rig.shows("app"));
    rig.send(Message::Search("redis".to_owned()));
    assert!(!rig.shows("shop") && !rig.shows("blog"));
    assert!(rig.shows("redis"));
    rig.send(Message::Search("nothing-like-it".to_owned()));
    assert!(rig.shows("No containers match."));
    rig.send(Message::Search(String::new()));
    rig.send(Message::OnlyRunning(true));
    assert!(rig.shows("redis") && !rig.shows("backup-job"));
}

#[test]
fn a_press_on_a_container_opens_its_logs() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    rig.frame(|frame| frame.click("redis"));
    let logs = Route::Container {
        id: "redis-0123456789abcdef".to_owned(),
        tab: Some("logs".to_owned()),
    };
    assert_eq!(rig.bench.took(), [Asked::Open(logs.into())]);
}

#[test]
fn a_selection_s_action_asks_for_each_selected_container_and_names_each_failure() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    let ids = ["redis-0123456789abcdef", "backup-job-0123456789abcdef"];
    for (index, id) in ids.iter().enumerate() {
        rig.send(Message::Table(table::Message::Toggle {
            key: (*id).to_owned(),
            index,
        }));
    }
    assert!(rig.shows("2 selected"));
    // No Engine answers in a test: both are refused, for the same reason.
    rig.send(Message::Bulk(Bulk::Stop));
    let asked = rig.bench.took();
    let [Asked::Failed(None, error), Asked::Invalidate(key)] = asked.as_slice() else {
        panic!("{asked:?}");
    };
    assert_eq!(error.code, ErrorCode::HostNotConnected);
    let named = error.detail.as_deref().unwrap_or_default();
    assert!(named.contains("local/redis: ") && named.contains("local/backup-job: "));
    assert_eq!(*key, keys::host::containers("h1"));
    // Remove asks first, naming how many go and how many run.
    rig.send(Message::Bulk(Bulk::Remove));
    assert!(rig.shows("Remove 2 containers?"));
    assert!(rig.state.actions.has_dialog());
}

#[test]
fn a_row_that_changes_or_comes_keeps_the_selection_and_the_scroll() {
    let _english = in_language("en");
    let mut list: Vec<Container> = (0..60)
        .map(|n| container(&format!("job-{n:02}"), None, "running", now()))
        .collect();
    let mut rig = Rig::with(list.clone());
    // Drawn once, the table knows how much of its rows shows.
    rig.frame(|_| {});
    rig.send(Message::Table(table::Message::Scrolled(600.0)));
    let key = list[20].id.clone();
    rig.send(Message::Table(table::Message::Toggle { key, index: 20 }));
    let scrolled = rig.state.table.offset();
    assert!(scrolled > 0.0);
    // An Engine event: one container exited, one was created, one is gone.
    "exited".clone_into(&mut list[20].state);
    list.remove(3);
    list.insert(0, container("job-new", None, "running", now()));
    rig.bench
        .data
        .set(&keys::host::containers("h1"), list.clone());
    rig.refresh();
    assert_eq!(rig.state.counts, (59, 60));
    rig.frame(|_| {});
    assert!((rig.state.table.offset() - scrolled).abs() < f32::EPSILON);
    let selection = rig.state.table.selection();
    assert!(selection.contains(&list[20].id) && selection.len() == 1);
}

#[test]
fn a_usage_report_is_shown_for_its_host_only() {
    let _english = in_language("en");
    let mut rig = Rig::with(sample());
    let list = sample();
    assert!(!rig.shows("1.5%"));
    rig.send(Message::Usage(("another".to_owned(), usage(&list))));
    assert!(!rig.shows("1.5%"));
    rig.send(Message::Usage(("h1".to_owned(), usage(&list))));
    // The containers' own, and their Stack's total.
    assert!(rig.shows("1.5%") && rig.shows("64.0 MB"));
}

#[test]
fn checking_for_newer_images_files_the_answer_and_marks_the_containers() {
    let _english = in_language("en");
    let mut rig = Rig::with(sample());
    assert!(rig.state.newer.is_empty());
    let _ = update(
        &mut rig.state,
        Message::Check(freshness::Message::Pressed),
        &rig.bench.ctx(),
    );
    assert!(rig.state.check.is_pending() && rig.shows("Checking…"));
    let newer = |reference: &str| FreshnessResult {
        reference: reference.to_owned(),
        image_id: "sha256:abc".to_owned(),
        freshness: Freshness::Newer,
    };
    // The registries answered: the shell files it and says it, and the
    // page's button stops waiting.
    let found = Ok(vec![newer("web:latest"), newer("db:latest")]);
    rig.bench.told(freshness::checked("h1", &found));
    rig.send(Message::Check(freshness::Message::Checked));
    assert_eq!(
        rig.bench.took(),
        [
            Asked::Said("2 images have a newer version".to_owned()),
            Asked::Put(keys::host::freshness("h1")),
        ]
    );
    rig.refresh();
    assert_eq!(rig.state.newer.len(), 2);
    assert!(!rig.state.check.is_pending() && rig.shows("Check for newer images"));
    // A check that fails says so, and marks nothing.
    rig.send(Message::Check(freshness::Message::Pressed));
    let asked = rig.bench.took();
    assert!(
        matches!(asked.as_slice(), [Asked::Failed(None, _)]),
        "{asked:?}"
    );
}

#[test]
fn links_go_where_they_say() {
    let mut rig = Rig::populated();
    rig.send(Message::OpenPort(8080));
    let stack = Route::Stack {
        name: "shop".to_owned(),
    };
    rig.send(Message::Go(stack.clone()));
    assert_eq!(
        rig.bench.took(),
        [
            Asked::OpenUrl("http://localhost:8080".to_owned()),
            Asked::Open(stack.into()),
        ]
    );
}

#[test]
fn the_page_s_actions_start_from_the_keyboard_and_the_palette() {
    let mut rig = Rig::populated();
    let task = super::super::request(&mut rig.state, registry::SEARCH, &rig.bench.ctx());
    assert!(!tasks::is_none(task), "/ gives the search field focus");
    assert!(rig.state.run.is_none());
    let _ = super::super::request(&mut rig.state, "run", &rig.bench.ctx());
    assert!(rig.state.run.is_some());
    // The Run dialog reads the Host's images and networks for its fields.
    let host = rig.state.host_id.clone();
    let watched = rig.bench.watched();
    assert!(watched.contains(&keys::host::images(&host)), "{watched:?}");
    assert!(
        watched.contains(&keys::host::networks(&host)),
        "{watched:?}"
    );
    rig.send(Message::Actions(actions::Message::CloseDialog));
    assert!(rig.bench.took().is_empty());
}

#[test]
fn a_list_that_could_not_be_read_shows_why_and_reads_again() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    let down = AppError::new(
        ErrorCode::HostNotConnected,
        homeport_hosts::messages::NOT_CONNECTED_UNKNOWN,
    );
    rig.bench.data.fail(&keys::host::containers("h1"), &down);
    assert!(!rig.shows("redis"), "the failure and nothing else");
    rig.send(Message::Failure(Event::Details));
    rig.send(Message::Failure(Event::Retry));
    assert_eq!(
        rig.bench.took(),
        [
            Asked::Details(down),
            Asked::Invalidate(keys::host::containers("h1")),
        ]
    );
}

#[test]
fn a_selected_container_the_search_hides_is_still_acted_on() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    let redis = "redis-0123456789abcdef".to_owned();
    rig.send(Message::Table(table::Message::Toggle {
        key: redis,
        index: 0,
    }));
    // React sent every selected id; here a row that left the list is let
    // go of by the table, and one the search only hides stays selected.
    rig.send(Message::Search("shop".to_owned()));
    assert!(!rig.shows("redis") && rig.shows("1 selected"));
    rig.send(Message::Bulk(Bulk::Restart));
    let asked = rig.bench.took();
    let Some(Asked::Failed(None, error)) = asked.first() else {
        panic!("{asked:?}");
    };
    assert!(error
        .detail
        .as_deref()
        .is_some_and(|d| d.starts_with("local/redis: ")));
    // The one that is gone from the list is no longer selected.
    rig.bench
        .data
        .set(&keys::host::containers("h1"), sample()[..3].to_vec());
    rig.refresh();
    rig.send(Message::Search(String::new()));
    rig.frame(|_| {});
    assert!(rig.state.table.selection().is_empty());
}

#[test]
fn no_band_stands_over_the_table_and_selecting_moves_no_row() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    // The header of the table comes right under the line of the filters.
    // Under the summary: the line of the filters, then the table at once.
    let gap = rig.top("Name") - rig.top("3 running of 5");
    assert!(gap < 100.0, "{gap} px between the header and the table");
    let before = rig.top("redis");
    let redis = "redis-0123456789abcdef".to_owned();
    rig.send(Message::Table(table::Message::Toggle {
        key: redis,
        index: 0,
    }));
    // The selection's buttons end the line of the filters, as in React.
    let bar = rig.top("1 selected");
    assert!(bar > rig.top("3 running of 5") && bar < rig.top("Name") - 30.0);
    assert!(
        (rig.top("redis") - before).abs() < f32::EPSILON,
        "{} {before}",
        rig.top("redis")
    );
}

#[test]
fn every_column_shows_without_scrolling_at_the_window_s_first_size() {
    for language in ["en", "ar"] {
        let _language = in_language(language);
        let mut rig = Rig::populated();
        // The last column ends before the start and stop button and the
        // menu of a row, which are in the window: nothing to scroll to.
        let (created, starts) = match language {
            "en" => ("Created", false),
            _ => ("تاريخ الإنشاء", true),
        };
        rig.frame(|frame| {
            let header = frame.find(created).expect("the last header");
            let buttons = 36.0 + 44.0;
            if starts {
                assert!(header.x >= buttons, "{language}: {header:?}");
            } else {
                assert!(
                    header.x + header.width <= SIZE.0 - buttons,
                    "{language}: {header:?}"
                );
            }
            assert!(frame.scrolled().is_empty(), "nothing scrolls on the page");
        });
        // And the wheel, turned sideways over the rows, moves nothing.
        rig.frame(|frame| {
            let header = frame.find(created).expect("the last header");
            frame.point_at(iced::Point::new(header.center_x(), header.y + 80.0));
            frame.send([iced::Event::Mouse(iced::mouse::Event::WheelScrolled {
                delta: iced::mouse::ScrollDelta::Lines { x: -4.0, y: 0.0 },
            })]);
            frame.send([iced::Event::Mouse(iced::mouse::Event::WheelScrolled {
                delta: iced::mouse::ScrollDelta::Lines { x: 4.0, y: 0.0 },
            })]);
        });
        assert!(
            rig.state.table.across().abs() < f32::EPSILON,
            "{language}: the columns scroll sideways by {}",
            rig.state.table.across()
        );
    }
}

#[test]
fn a_stack_s_fraction_reads_the_same_way_in_arabic() {
    let _arabic = in_language("ar");
    let rig = Rig::populated();
    // One number, left to right: not two, each on its own.
    assert!(rig.shows("1/2 قيد التشغيل"));
}

#[test]
fn run_container_opens_the_run_dialog_which_cancel_and_escape_close() {
    let _english = in_language("en");
    let mut rig = Rig::populated();
    let _ = super::super::request(&mut rig.state, "run", &rig.bench.ctx());
    assert!(rig.shows("Run a container") && rig.shows("Cancel"));
    // Run waits for an image.
    assert!(rig.shows("Run"));
    rig.frame(|frame| frame.click("Cancel"));
    assert!(rig.state.run.is_none() && !rig.shows("Run a container"));
    rig.send(Message::Run);
    rig.frame(|frame| frame.tap_key(iced::keyboard::key::Named::Escape));
    assert!(rig.state.run.is_none());
}

/// The statuses Docker writes every day, on the sample's containers.
fn ordinary() -> Vec<Container> {
    let mut list = sample();
    "Up 3 hours (healthy)".clone_into(&mut list[0].status);
    "Exited (0) 2 days ago".clone_into(&mut list[1].status);
    "Up 12 minutes".clone_into(&mut list[2].status);
    list
}

#[test]
fn every_status_is_whole_at_the_window_s_first_size() {
    for language in ["en", "ar"] {
        let _language = in_language(language);
        let list = ordinary();
        let mut rig = Rig::with(list.clone());
        rig.send(Message::Usage(("h1".to_owned(), usage(&list))));
        rig.frame(|frame| {
            let fitted = frame.fitted();
            // The health is the badge under it, not said twice.
            let statuses = [
                "Up 3 hours",
                "Exited (0) 2 days ago",
                "Up 12 minutes",
                "Up 1 minute",
                "Exited (0)",
            ];
            for status in statuses {
                let shown: Vec<_> = fitted.iter().filter(|text| text.whole == status).collect();
                assert!(!shown.is_empty(), "{language}: no row says {status}");
                for text in shown {
                    assert_eq!(text.shown, text.whole, "{language}: a status is cut");
                }
            }
            assert!(frame.find("healthy").is_some());
        });
    }
}

#[test]
fn what_lacks_room_wraps_onto_the_row_s_second_line_before_it_is_cut() {
    let _arabic = in_language("ar");
    let list = ordinary();
    let mut rig = Rig::with(list.clone());
    rig.send(Message::Usage(("h1".to_owned(), usage(&list))));
    rig.frame(|frame| {
        // "A minute ago" is longer in Arabic than its column: two lines.
        let created = crate::i18n::format::relative(list[3].created);
        let fitted = frame.fitted();
        let shown: Vec<_> = fitted.iter().filter(|text| text.whole == created).collect();
        assert_eq!(shown.len(), list.len());
        assert!(shown.iter().all(|text| !text.is_cut()), "{shown:?}");
        let cell = frame.find(&created).expect("a created cell");
        assert!(cell.height > 30.0, "on one line: {cell:?}");
        // And the status of a container with no health check has both.
        let exited = frame.find("Exited (0) 2 days ago").expect("its status");
        assert!(exited.height > 30.0, "on one line: {exited:?}");
    });
}

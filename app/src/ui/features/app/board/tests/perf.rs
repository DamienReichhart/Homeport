//! Twenty thousand containers on the Board: what a draw and a change cost.

use std::time::Instant;

use super::*;
use crate::ui::widgets::table;

const HOSTS: usize = 20;
const EACH: usize = 1000;

fn fleet() -> Rig {
    let hosts: Vec<HostView> = (0..HOSTS)
        .map(|n| host_view(&format!("host-{n:02}"), connected()))
        .collect();
    let mut rig = Rig::over(hosts);
    for host in 0..HOSTS {
        let list: Vec<Container> = (0..EACH)
            .map(|n| {
                let stack = format!("stack-{}", n % 40);
                container(
                    &format!("svc-{host:02}-{n:04}"),
                    Some(&stack),
                    "running",
                    now(),
                )
            })
            .collect();
        let usage = usage(&list);
        let id = format!("host-{host:02}-id");
        rig.bench.data.set(&keys::host::containers(&id), list);
        rig.send(Message::Usage((id, usage)));
    }
    rig.refresh();
    rig
}

#[test]
fn twenty_thousand_containers_are_rows_made_once() {
    let mut rig = fleet();
    assert_eq!(rig.state.rows.len(), HOSTS * EACH);
    assert_eq!(rig.state.counts, (HOSTS * EACH, HOSTS * EACH));
    assert_eq!(rig.state.stacks.len(), 40);
    // A draw, a scroll, a selection: the rows are the ones it made.
    let revision = rig.state.revision;
    rig.frame(|_| {});
    rig.send(Message::Table(table::Message::Scrolled(400_000.0)));
    rig.send(Message::Table(table::Message::SelectAll));
    rig.refresh();
    rig.frame(|_| {});
    assert_eq!(rig.state.revision, revision);
    assert_eq!(rig.state.table.selection().len(), HOSTS * EACH);
    assert!(rig.state.table.offset() > 100_000.0);
}

/// `cargo test -p homeport-app --lib board::tests::perf -- --ignored --nocapture`
///
/// It holds a scroll step and a usage report to the table's budget for a
/// frame: 2 ms, eight times that in a build that is not optimised. For
/// what a release build does, optimise the crate and the crates that
/// shape and draw text as `widgets/table/tests/perf.rs` says, and set
/// `TABLE_PERF_OPTIMISED=1`: 1.8 ms a scroll step then, of which about
/// half is fitting the texts of the row that came into view.
#[test]
#[ignore = "a measurement: run it alone, as its timings need the machine to itself"]
fn what_twenty_thousand_containers_cost() {
    let mut rig = fleet();
    rig.frame(|_| {});
    let time = |label: &str, rig: &mut Rig, act: &mut dyn FnMut(&mut Rig)| -> f64 {
        const RUNS: u32 = 30;
        let started = Instant::now();
        for _ in 0..RUNS {
            act(rig);
        }
        let each = started.elapsed().as_secs_f64() * 1000.0 / f64::from(RUNS);
        println!("{label}: {each:.2} ms");
        each
    };
    time("refresh, nothing changed", &mut rig, &mut |rig| {
        rig.refresh()
    });
    time("view", &mut rig, &mut |rig| drop(rig.view()));
    let fits =
        || crate::ui::widgets::basic::truncate::FITS.load(std::sync::atomic::Ordering::Relaxed);
    let before = fits();
    let mut offset = 0.0;
    let scroll = time(
        "a scroll step: update, view, layout, draw",
        &mut rig,
        &mut |rig| {
            offset += 53.0;
            rig.send(Message::Table(table::Message::Scrolled(offset)));
            rig.frame(|_| {});
        },
    );
    println!("texts fitted in 30 scroll steps: {}", fits() - before);
    let report = time(
        "a usage report: update, view, layout, draw",
        &mut rig,
        &mut |rig| {
            let usage = rig.state.usage["host-03-id"].clone();
            rig.send(Message::Usage(("host-03-id".to_owned(), usage)));
            rig.frame(|_| {});
        },
    );
    time("a list read again: refresh, view", &mut rig, &mut |rig| {
        let list: Vec<Container> = (0..EACH)
            .map(|n| container(&format!("svc-03-{n:04}"), None, "exited", now()))
            .collect();
        rig.read("host-03-id", list);
        drop(rig.view());
    });
    time(
        "a letter in the search: update, view",
        &mut rig,
        &mut |rig| {
            rig.send(Message::Search("svc-07".to_owned()));
            drop(rig.view());
        },
    );
    // The table's budget for a frame, which the Board's own work rides
    // on: 2 ms, eight times that in a build that is not optimised
    // (`TABLE_PERF_OPTIMISED=1` with the crate optimised holds it to 2).
    let budget = if std::env::var_os("TABLE_PERF_OPTIMISED").is_some() {
        2.0
    } else {
        16.0
    };
    assert!(scroll < budget, "a scroll step took {scroll:.2} ms");
    assert!(report < budget, "a usage report took {report:.2} ms");
}

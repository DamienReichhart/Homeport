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
#[test]
#[ignore = "a measure, not a check: it prints what the Board costs"]
fn what_twenty_thousand_containers_cost() {
    let mut rig = fleet();
    rig.frame(|_| {});
    let time = |label: &str, rig: &mut Rig, act: &mut dyn FnMut(&mut Rig)| {
        const RUNS: u32 = 30;
        let started = Instant::now();
        for _ in 0..RUNS {
            act(rig);
        }
        println!(
            "{label}: {:.2} ms",
            started.elapsed().as_secs_f64() * 1000.0 / f64::from(RUNS)
        );
    };
    time("refresh, nothing changed", &mut rig, &mut |rig| {
        rig.refresh()
    });
    time("view", &mut rig, &mut |rig| drop(rig.view()));
    let mut offset = 0.0;
    time(
        "a scroll step: update, view, layout, draw",
        &mut rig,
        &mut |rig| {
            offset += 53.0;
            rig.send(Message::Table(table::Message::Scrolled(offset)));
            rig.frame(|_| {});
        },
    );
    time(
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
}

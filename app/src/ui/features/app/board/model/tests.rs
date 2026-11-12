//! The Board's model, held to `model/board.test.ts`.

use super::*;
use crate::{
    test_support::fixtures::container,
    ui::shell::router::{History, Request, Route},
};

fn source(id: &str, name: &str, containers: Vec<Container>) -> Source {
    Source {
        host_id: id.into(),
        host_name: name.into(),
        containers: Some(Arc::new(containers)),
    }
}

fn sample() -> Vec<Row> {
    let mut api = container("api", None, "running", 0);
    api.health = Some("unhealthy".to_owned());
    rows(&[
        source(
            "a",
            "laptop",
            vec![
                container("web", Some("shop"), "running", 0),
                container("db", Some("shop"), "exited", 0),
            ],
        ),
        source(
            "b",
            "prod-server",
            vec![api, container("cache", Some("shop"), "running", 0)],
        ),
    ])
}

fn names<'a>(rows: impl IntoIterator<Item = &'a Row>) -> Vec<String> {
    rows.into_iter()
        .map(|row| format!("{}:{}", row.host_name, row.container().name))
        .collect()
}

fn kept(rows: &[Row], filter: &Filter) -> Vec<String> {
    names(rows.iter().filter(|row| filter.keeps(row)))
}

/// The rows as the table shows them sorted by one column: a stable sort,
/// turned round for a descending one.
fn sorted(
    rows: &[Row],
    key: SortKey,
    descending: bool,
    cpu: fn(&Row) -> Option<f64>,
) -> Vec<String> {
    let usages: Vec<Option<ContainerUsage>> = rows
        .iter()
        .map(|row| {
            cpu(row).map(|cpu_percent| ContainerUsage {
                id: row.container().id.clone(),
                cpu_percent,
                memory_usage: 0,
                memory_limit: 0,
            })
        })
        .collect();
    let usage_of = |row: &Row| {
        let at = rows.iter().position(|other| other.key == row.key)?;
        usages[at].as_ref()
    };
    let mut order: Vec<&Row> = rows.iter().collect();
    order.sort_by(|a, b| {
        let ordering = compare(key, a, b, usage_of);
        if descending {
            ordering.reverse()
        } else {
            ordering
        }
    });
    names(order)
}

#[test]
fn it_lists_every_host_s_containers_each_with_its_host() {
    let rows = sample();
    // By Host, then by name: the order of a Board that isn't sorted.
    assert_eq!(
        names(&rows),
        [
            "laptop:db",
            "laptop:web",
            "prod-server:api",
            "prod-server:cache"
        ]
    );
    let mut keys: Vec<&str> = rows.iter().map(|row| row.key.as_str()).collect();
    keys.dedup();
    assert_eq!(keys.len(), 4);
    assert_eq!(rows[0].key, "a/db-0123456789abcdef");
    // A Host whose list was not read yet has no row.
    let waiting = Source {
        host_id: "c".into(),
        host_name: "nas".into(),
        containers: None,
    };
    assert!(super::rows(&[waiting]).is_empty());
}

#[test]
fn it_filters_by_host_state_stack_and_search() {
    let rows = sample();
    let all = Filter::default();
    let hosts = Filter {
        hosts: vec!["b".to_owned()],
        ..all.clone()
    };
    assert_eq!(
        kept(&rows, &hosts),
        ["prod-server:api", "prod-server:cache"]
    );
    let state = |state| Filter {
        state,
        ..all.clone()
    };
    assert_eq!(kept(&rows, &state(StateFilter::Stopped)), ["laptop:db"]);
    assert_eq!(
        kept(&rows, &state(StateFilter::Unhealthy)),
        ["prod-server:api"]
    );
    let shop = Filter {
        stack: Some("shop".to_owned()),
        state: StateFilter::Running,
        ..all.clone()
    };
    assert_eq!(kept(&rows, &shop), ["laptop:web", "prod-server:cache"]);
    // The search matches the Host's name too.
    let query = |query: &str| Filter {
        query: query.to_owned(),
        ..all.clone()
    };
    assert_eq!(kept(&rows, &query("PROD")).len(), 2);
    assert_eq!(kept(&rows, &query("cache")), ["prod-server:cache"]);
    assert_eq!(kept(&rows, &all).len(), 4);
    assert!(!all.is_set() && shop.is_set());
}

#[test]
fn it_keeps_the_filter_in_the_location() {
    let filter = Filter {
        hosts: vec!["a".to_owned(), "b".to_owned()],
        state: StateFilter::Running,
        stack: Some("shop".to_owned()),
        query: "w".to_owned(),
    };
    let (query, params) = filter.write();
    let written: Vec<String> = params.iter().map(|(k, v)| format!("{k}={v}")).collect();
    assert_eq!(written, ["host=a", "host=b", "state=running", "stack=shop"]);
    assert_eq!(query.as_deref(), Some("w"));
    let mut history = History::starting_at(Request::to(Route::Board));
    history.amend(query, params);
    assert_eq!(Filter::read(history.current()), filter);
    // A state the Board doesn't know is no filter.
    let bogus = Request::to(Route::Board).with_param("state", "bogus");
    let history = History::starting_at(bogus);
    assert_eq!(Filter::read(history.current()), Filter::default());
    assert_eq!(Filter::default().write(), (None, vec![]));
}

#[test]
fn it_sorts_by_any_column_ties_by_host_then_name() {
    let rows = sample();
    let none: fn(&Row) -> Option<f64> = |_| None;
    assert_eq!(
        sorted(&rows, SortKey::Name, false, none),
        [
            "prod-server:api",
            "prod-server:cache",
            "laptop:db",
            "laptop:web"
        ]
    );
    #[allow(clippy::cast_precision_loss)]
    let by_cpu = sorted(&rows, SortKey::Cpu, true, |row| {
        let name = &row.container().name;
        (name != "db").then_some(name.len() as f64)
    });
    assert_eq!(by_cpu[0], "prod-server:cache");
    assert_eq!(by_cpu[3], "laptop:db");
    // Descending by Host: the ties still read by name.
    assert_eq!(
        sorted(&rows, SortKey::Host, true, none),
        [
            "prod-server:api",
            "prod-server:cache",
            "laptop:db",
            "laptop:web"
        ]
    );
    assert_eq!(
        sorted(&rows, SortKey::Stack, false, none)[0],
        "prod-server:api"
    );
    assert_eq!(sorted(&rows, SortKey::Status, false, none)[0], "laptop:db");
}

#[test]
fn it_lists_the_stacks_and_names_a_row_s_target() {
    let rows = sample();
    assert_eq!(stacks_of(&rows), ["shop"]);
    let api = rows[2].target();
    assert_eq!(
        (
            api.host_id.as_str(),
            api.host_name.as_str(),
            api.name.as_str()
        ),
        ("b", "prod-server", "api")
    );
}
